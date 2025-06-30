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

    // ---------------------------------------//
    // Funções Aqui

    // O que faz: Esta função
    // é responsável por criar um novo objeto Zombie e armazená-lo na blockchain.
    //
    // POR QUE É ESCRITA DESTA FORMA:
    // 1. Assinatura da Função:
    //    - `&self`: Permite que a função interaja com o armazenamento do contrato.
    //    - `name: ManagedBuffer`: Recebe o nome do novo zumbi. 
    //      `ManagedBuffer` é o tipo usado para strings/arrays de bytes no ambiente MultiversX SC.
    //    - `dna: u64`: Recebe o valor de DNA do novo zumbi, um inteiro de 64 bits.
    //
    // 2. `self.zombie_last_index().update(|id| { ... });`
    //    - `self.zombie_last_index()`: Acede o mapeador de armazenamento 
    //       que guarda o último ID de zumbi utilizado (ou o próximo disponível).
    //    - `.update(|id| { ... })`: Este é um método padrão para modificar valores no armazenamento persistente.
    //      Ele garante que a leitura do valor atual (`*id`) e a escrita do novo valor (`*id += 1`)
    //      ocorram de forma atômica e segura, prevenindo inconsistências em um ambiente de concorrência.
    //      A closure `|id|` recebe uma referência mutável ao valor armazenado (`&mut usize`), permitindo que ele seja modificado.
    //
    // 3. `self.zombies(id).set(Zombie { name, dna });`
    //    - `self.zombies(id)`: Acede o mapeador de armazenamento específico para o ID atual do zumbi.
    //      Sua declaração `fn zombies(&self, id: usize) -> SingleValueMapper<Zombie<Self::Api>>;`
    //      significa que cada ID de zumbi terá seu próprio "slot" de armazenamento de valor único.
    //    - `.set(Zombie { name, dna })`: Define o valor nesse slot de armazenamento como uma nova instância da struct `Zombie`.
    //      A sintaxe `{ name, dna }` é um atalho de Rust quando os nomes dos campos da struct
    //      são os mesmos dos nomes das variáveis que você está usando para preenchê-los.
    //      Isso salva o novo zumbi na blockchain, associado ao seu ID único.
    //
    // 4. `*id += 1;`
    //    - Incrementa o valor do `id` (o contador `zombie_last_index`) em 1.
    //    - Isso prepara o `zombie_last_index` para o próximo zumbi que será criado,
    //      garantindo que ele receba um ID subsequente. A modificação é feita dentro
    //      da closure `update`, assegurando que o novo valor seja persistido.
    fn create_zombie(&self, name: ManagedBuffer, dna: u64) {
        self.zombie_last_index().update(|id| //capturando o id disponível na lista de zombies
            {
            self.zombies(id).set(Zombie { name, dna }); // Atualizando lista com novo Zombie no ID 
        *id +=1; // Atualizando id para próxima criação pegar o ID correto
        });
    }
    // iniciando desenvolvimento de uma função que irá retornar um DNA aleatório
    // ela deve retornar um dado u64
    fn generate_random_dna(&self) -> u64 {
    }

    //----------------------------------------//


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

    // marcando como view, transformando o storage em um endpoint,
    // funções com essa marcação tem sua chamada gratuita, ou seja, não é cobrado GAS
    // porque não modificam o estado da blockchain
    // Isso permitirá ao nosso dApp consultar informações em tempo real sem custos adicionais.
    #[view]
    #[storage_mapper("zombies")]
    fn zombies(&self, id: usize) -> SingleValueMapper<Zombie<Self::Api>>;
}
