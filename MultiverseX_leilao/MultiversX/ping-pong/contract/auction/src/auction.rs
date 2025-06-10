#![no_std]

use multiversx_sc::imports::*;

/// Smart Contract para leilão de bens
/// Permite que um usuário cadastre um bem para leilão com valor mínimo e prazo
/// Outros usuários podem dar lances, e o maior lance válido ganha o bem
#[multiversx_sc::contract]
pub trait Auction {
    /// Inicialização do contrato
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    // Estrutura para armazenar informações do leilão
    #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
    pub struct AuctionInfo<M: ManagedTypeApi> {
        pub owner: ManagedAddress<M>,
        pub item_name: ManagedBuffer<M>,
        pub item_description: ManagedBuffer<M>,
        pub minimum_bid: BigUint<M>,
        pub current_highest_bid: BigUint<M>,
        pub highest_bidder: ManagedAddress<M>,
        pub end_timestamp: u64,
        pub is_active: bool,
        pub token_id: EgldOrEsdtTokenIdentifier<M>,
    }

    /// Cadastra um novo item para leilão
    #[endpoint(createAuction)]
    fn create_auction(
        &self,
        item_name: ManagedBuffer,
        item_description: ManagedBuffer,
        minimum_bid: BigUint,
        duration_in_seconds: u64,
        opt_token_id: OptionalValue<EgldOrEsdtTokenIdentifier>,
    ) -> u64 {
        require!(!item_name.is_empty(), "Item name cannot be empty");
        require!(minimum_bid > 0, "Minimum bid must be greater than zero");
        require!(duration_in_seconds > 0, "Duration must be greater than zero");

        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();
        let end_timestamp = current_timestamp + duration_in_seconds;

        let token_id = match opt_token_id {
            OptionalValue::Some(t) => t,
            OptionalValue::None => EgldOrEsdtTokenIdentifier::egld(),
        };

        let auction_id = self.next_auction_id().get();
        
        let auction_info = AuctionInfo {
            owner: caller.clone(),
            item_name: item_name.clone(),
            item_description: item_description.clone(),
            minimum_bid: minimum_bid.clone(),
            current_highest_bid: BigUint::zero(),
            highest_bidder: ManagedAddress::zero(),
            end_timestamp,
            is_active: true,
            token_id: token_id.clone(),
        };

        self.auction_info(auction_id).set(&auction_info);
        self.next_auction_id().set(auction_id + 1);

        self.auction_created_event(
            &auction_id,
            &caller,
            &item_name,
            &minimum_bid,
            &end_timestamp,
            &token_id,
        );

        auction_id
    }

    /// Permite dar um lance em um leilão ativo
    #[payable]
    #[endpoint(placeBid)]
    fn place_bid(&self, auction_id: u64) {
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();
        
        require!(
            self.auction_info(auction_id).is_empty() == false,
            "Auction does not exist"
        );

        let mut auction_info = self.auction_info(auction_id).get();
        
        require!(auction_info.is_active, "Auction is not active");
        require!(
            current_timestamp < auction_info.end_timestamp,
            "Auction has ended"
        );
        require!(
            caller != auction_info.owner,
            "Owner cannot bid on their own auction"
        );

        let (payment_token, payment_amount) = self.call_value().egld_or_single_fungible_esdt();
        require!(
            payment_token == auction_info.token_id,
            "Invalid payment token"
        );
        require!(
            payment_amount >= auction_info.minimum_bid,
            "Bid must be at least the minimum bid amount"
        );
        require!(
            payment_amount > auction_info.current_highest_bid,
            "Bid must be higher than current highest bid"
        );

        // Reembolsar o lance anterior se existir
        if auction_info.current_highest_bid > 0 && !auction_info.highest_bidder.is_zero() {
            self.send().direct(
                &auction_info.highest_bidder,
                &auction_info.token_id,
                0,
                &auction_info.current_highest_bid,
            );
        }

        // Atualizar informações do leilão
        auction_info.current_highest_bid = payment_amount.clone();
        auction_info.highest_bidder = caller.clone();
        
        self.auction_info(auction_id).set(&auction_info);

        self.bid_placed_event(&auction_id, &caller, &payment_amount);
    }

    /// Finaliza um leilão após o prazo
    #[endpoint(finalizeAuction)]
    fn finalize_auction(&self, auction_id: u64) {
        require!(
            self.auction_info(auction_id).is_empty() == false,
            "Auction does not exist"
        );

        let mut auction_info = self.auction_info(auction_id).get();
        let current_timestamp = self.blockchain().get_block_timestamp();
        
        require!(auction_info.is_active, "Auction is already finalized");
        require!(
            current_timestamp >= auction_info.end_timestamp,
            "Auction has not ended yet"
        );

        auction_info.is_active = false;
        
        if auction_info.current_highest_bid >= auction_info.minimum_bid && !auction_info.highest_bidder.is_zero() {
            // Transferir o valor para o dono do leilão
            self.send().direct(
                &auction_info.owner,
                &auction_info.token_id,
                0,
                &auction_info.current_highest_bid,
            );
            
            self.auction_info(auction_id).set(&auction_info);
            
            self.auction_finalized_event(
                &auction_id,
                &auction_info.highest_bidder,
                &auction_info.current_highest_bid,
                true,
            );
        } else {
            // Nenhum lance válido, leilão cancelado
            if auction_info.current_highest_bid > 0 && !auction_info.highest_bidder.is_zero() {
                // Reembolsar o último lance se existir
                self.send().direct(
                    &auction_info.highest_bidder,
                    &auction_info.token_id,
                    0,
                    &auction_info.current_highest_bid,
                );
            }
            
            self.auction_info(auction_id).set(&auction_info);
            
            self.auction_finalized_event(
                &auction_id,
                &ManagedAddress::zero(),
                &BigUint::zero(),
                false,
            );
        }
    }

    /// Cancela um leilão (apenas o dono pode cancelar antes do fim)
    #[endpoint(cancelAuction)]
    fn cancel_auction(&self, auction_id: u64) {
        let caller = self.blockchain().get_caller();
        
        require!(
            self.auction_info(auction_id).is_empty() == false,
            "Auction does not exist"
        );

        let mut auction_info = self.auction_info(auction_id).get();
        
        require!(auction_info.is_active, "Auction is already finalized");
        require!(
            caller == auction_info.owner,
            "Only auction owner can cancel"
        );

        auction_info.is_active = false;
        
        // Reembolsar o lance atual se existir
        if auction_info.current_highest_bid > 0 && !auction_info.highest_bidder.is_zero() {
            self.send().direct(
                &auction_info.highest_bidder,
                &auction_info.token_id,
                0,
                &auction_info.current_highest_bid,
            );
        }
        
        self.auction_info(auction_id).set(&auction_info);
        
        self.auction_cancelled_event(&auction_id);
    }

    // Views

    #[view(getAuctionInfo)]
    fn get_auction_info(&self, auction_id: u64) -> OptionalValue<AuctionInfo<Self::Api>> {
        if self.auction_info(auction_id).is_empty() {
            OptionalValue::None
        } else {
            OptionalValue::Some(self.auction_info(auction_id).get())
        }
    }

    #[view(getActiveAuctions)]
    fn get_active_auctions(&self) -> MultiValueEncoded<u64> {
        let mut active_auctions = MultiValueEncoded::new();
        let total_auctions = self.next_auction_id().get();
        
        for auction_id in 0..total_auctions {
            if !self.auction_info(auction_id).is_empty() {
                let auction_info = self.auction_info(auction_id).get();
                if auction_info.is_active {
                    let current_timestamp = self.blockchain().get_block_timestamp();
                    if current_timestamp < auction_info.end_timestamp {
                        active_auctions.push(auction_id);
                    }
                }
            }
        }
        
        active_auctions
    }

    #[view(getTimeRemaining)]
    fn get_time_remaining(&self, auction_id: u64) -> OptionalValue<u64> {
        if self.auction_info(auction_id).is_empty() {
            return OptionalValue::None;
        }

        let auction_info = self.auction_info(auction_id).get();
        if !auction_info.is_active {
            return OptionalValue::Some(0);
        }

        let current_timestamp = self.blockchain().get_block_timestamp();
        if current_timestamp >= auction_info.end_timestamp {
            OptionalValue::Some(0)
        } else {
            let time_left = auction_info.end_timestamp - current_timestamp;
            OptionalValue::Some(time_left)
        }
    }

    #[view(canFinalize)]
    fn can_finalize(&self, auction_id: u64) -> bool {
        if self.auction_info(auction_id).is_empty() {
            return false;
        }

        let auction_info = self.auction_info(auction_id).get();
        if !auction_info.is_active {
            return false;
        }

        let current_timestamp = self.blockchain().get_block_timestamp();
        current_timestamp >= auction_info.end_timestamp
    }

    // Storage

    #[storage_mapper("nextAuctionId")]
    fn next_auction_id(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("auctionInfo")]
    fn auction_info(&self, auction_id: u64) -> SingleValueMapper<AuctionInfo<Self::Api>>;

    // Events

    #[event("auctionCreated")]
    fn auction_created_event(
        &self,
        #[indexed] auction_id: &u64,
        #[indexed] owner: &ManagedAddress,
        #[indexed] item_name: &ManagedBuffer,
        minimum_bid: &BigUint,
        end_timestamp: &u64,
        token_id: &EgldOrEsdtTokenIdentifier,
    );

    #[event("bidPlaced")]
    fn bid_placed_event(
        &self,
        #[indexed] auction_id: &u64,
        #[indexed] bidder: &ManagedAddress,
        bid_amount: &BigUint,
    );

    #[event("auctionFinalized")]
    fn auction_finalized_event(
        &self,
        #[indexed] auction_id: &u64,
        #[indexed] winner: &ManagedAddress,
        winning_bid: &BigUint,
        successful: bool,
    );

    #[event("auctionCancelled")]
    fn auction_cancelled_event(&self, #[indexed] auction_id: &u64);
}