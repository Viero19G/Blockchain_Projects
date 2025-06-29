Um contrato vazio chamado HelloWorldpode ser criado com sc-meta executando 

sc-meta new --name hello-world --template empty

a partir do terminal e a estrutura do diretório ficaria assim:

├── meta
├── scenarios
├── src
  ├── hello_world.rs
├── tests
├── wasm
. Cargo.toml

Em nosso código vamos alteralo para lib.rs