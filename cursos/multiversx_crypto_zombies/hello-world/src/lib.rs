#![no_std]
// importando bibliotecas Rust
multiversx_sc::imports!(); 
multiversx_sc::derive_imports!();
//--------------------------//

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
// Structs ajudam a criar um formato mais adequado para dados complexos como por exemplo nosso ZOmbie
// Gost de pensar que "Structs são uma forma de definir como 'um objeto' será armazenado"
// Pensar dessa forma me ajuda a esclarecer que Struct vai otimizar o uso da VM no momento do armazenamento
// Para String podemos usar ManageBuffer que pode ser chamado no código assim: 
// let greeting = ManagedBuffer::from(b"Hello world!");

pub struct Zombie<M: ManagedTypeApi> {
  dna: u64,
  name: ManagedBuffer<M>,
}

#[allow(unused_imports)]
use multiversx_sc::imports::*;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait ZombiesContract {
    #[init]
    fn init(&self) {
    self.dna_digits().set(16u8);
    }

    #[upgrade]
    fn upgrade(&self) {}

    // Mapeando variável para ser salva onchain com SingleValueMapper
    // deve ser utilizado chamando SingleValueMapper<aqui_o_tipo_da_variável_rust >
    #[storage_mapper("dnaDigits")]
    fn dna_digits(&self) -> SingleValueMapper<u8>;
}
