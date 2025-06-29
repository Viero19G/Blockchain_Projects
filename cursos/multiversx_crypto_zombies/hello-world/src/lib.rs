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
// porém a notação acima resolve apenas se for um único dado.
// O que fazemos com struct é dizer "O que queremos armazenar tem a seguinte estrutura.... "
// Adicionamos as notações necessárias 
pub struct Zombie<M: ManagedTypeApi> {
  name: ManagedBuffer<M>,
  dna: u64,

}

#[allow(unused_imports)]
use multiversx_sc::imports::*;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait ZombiesContract {
    #[init]
    fn init(&self) {
    self.dna_digits().set(16u8);
    // nicializando um contador para o próximo ID de zumbi em 1
    self.zombie_last_index().set(1usize);
    }

    #[upgrade]
    fn upgrade(&self) {}

    // Mapeando variável para ser salva onchain com SingleValueMapper
    // deve ser utilizado chamando SingleValueMapper<aqui_o_tipo_da_variável_rust >
    #[storage_mapper("dnaDigits")]
    fn dna_digits(&self) -> SingleValueMapper<u8>;

    // no armazenamento global do contrato será "zombieLastIndex".
    // Por que existe: Mapeadores de armazenamento são a forma como os contratos MultiversX
    // guardam dados de forma persistente, ou seja, dados que sobrevivem após a execução
    // de uma transação e estão disponíveis para futuras interações com o contrato.
    #[storage_mapper("zombieLastIndex")]
    fn zombie_last_index(&self) -> SingleValueMapper<usize>;

    // O que faz: Este é um mapeador de armazenamento configurado para guardar os objetos 'Zombie'.
    // Diferente de um 'MapMapper' (que armazena múltiplos itens sob uma única chave raiz),
    // um 'SingleValueMapper' aqui, com um parâmetro 'id', indica que cada Zumbi será
    // armazenado como um valor único, acessível por uma chave composta que inclui "zombies" e o 'id' fornecido.
    #[storage_mapper("zombies")]
    fn zombies(&self, id: usize) -> SingleValueMapper<Zombie<Self::Api>>;
}
