
# 🚀 Criando um dApp com MultiversX em 15 minutos

Este guia mostra como criar rapidamente um dApp na **MultiversX**, com backend rodando na **Devnet** e frontend usando template oficial.

---

## 📦 Dependências

Antes de começar, garanta que o ambiente tenha:

- **Rust** >= 1.78
- **Node.js** >= 20 ([instalar Node.js](https://nodejs.org/en/download))
- **Yarn**:
  ```bash
  npm install --global yarn
  ```

### 🛠️ Dependências Linux/WSL

```bash
sudo apt-get install build-essential pkg-config libssl-dev
```

### 🦀 Instale o Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

> Siga com a opção 1 (instalação padrão) e reinicie o terminal.

### 🧱 Instale o `sc-meta` e ferramentas relacionadas

```bash
cargo install multiversx-sc-meta --locked
sc-meta install all
cargo install twiggy
```

> Obs: se necessário, instale também:
```bash
rustup target add wasm32-unknown-unknown
```

---

## 📁 Estrutura do Projeto

Crie o repositório e entre na pasta:

```bash
mkdir multiversX
cd multiversX
```

---

## 🔐 Criando a Wallet

Você pode:

### ✅ Usar a Web Wallet

- Acesse: https://docs.multiversx.com/wallet/web-wallet/

### 🧪 Ou criar uma wallet PEM local:

```bash
mkdir -p wallet
sc-meta wallet new --format pem --outfile ./wallet/wallet-owner.pem
```

1. Acesse: https://devnet-wallet.multiversx.com/unlock/pem  
2. Faça login com o arquivo `.pem`  
3. Use o **Faucet** para obter **5 xEGLD gratuitos**

---

## ⚙️ Smart Contract Ping-Pong

### Clone o contrato:

```bash
mkdir -p ping-pong
cd ping-pong
git clone https://github.com/multiversx/mx-ping-pong-sc contract
```

### Compile o contrato:

```bash
cd contract/ping-pong
sc-meta all build
```

> O bytecode será gerado em: `output/ping-pong.wasm`

---

## 🔧 Usando a Wallet no Interactor

Altere o arquivo:

```txt
ping-pong/contract/ping-pong/interactor/src/interact.rs
```

### Substitua:

```rust
let alice_wallet_address = interactor.register_wallet(test_wallets::alice()).await;
```

### Por:

```rust
let alice_wallet_address = interactor
    .register_wallet(Wallet::from_pem_file("/CAMINHO-ABSOLUTO/ping-pong/wallet/wallet-owner.pem").unwrap())
    .await;
```

> Use o **caminho absoluto correto** para seu `.pem`.

---

## 🚀 Deploy do Contrato

Execute o comando dentro da pasta:

```bash
cd ../../interactor
cargo run deploy --ping-amount 1000000000000000000 --duration-in-seconds 180
```

Exemplo de retorno:

```
sender's recalled nonce: 12422
sc deploy tx hash: b6ca6c8e6ac54ed168bcd6929e762610e2360674f562115107cf3702b8a22467
deploy address: erd1qqqqqqqqqqqqqpgqymj43x6anzr38jfz7kw3td2ew33v9jtrd8sse5zzk6
```

### 🔍 Verifique no Explorer:

https://devnet-explorer.multiversx.com/

---

## 💻 Camada de Aplicação (Frontend)

### Volte para a raiz:

```bash
cd ../../..
```

### Clone o template dApp oficial:

```bash
git clone https://github.com/multiversx/mx-template-dapp dapp
cd dapp
```

### Configure o contrato

Edite o arquivo:

```ts
src/config/config.devnet.ts
```
![image](https://github.com/user-attachments/assets/203415d0-28cd-44a3-876b-72c976d95d9d)


Substitua o valor:

```ts
contractAddress: "erd1qqqqqqqqqqqqqpgqymj43x6anzr38jfz7kw3td2ew33v9jtrd8sse5zzk6",
```

---


Use o comando:  yarn start:devnet no raiz do dapp
```ts
 yarn start:devnet
```

![image](https://github.com/user-attachments/assets/63d31bb9-49e6-44c2-ab88-d221ef2a71bd)

## 🏁 Conclusão

Em 15 minutos você terá:

- ✅ Smart Contract implantado na Devnet
- ✅ Frontend funcional conectado ao contrato
- ✅ Ambiente pronto para evoluir seu dApp (como no projeto `MultiversX_leilao`)

---

## 📚 Fontes oficiais

- [Docs MultiversX](https://docs.multiversx.com/)
- [Explorer Devnet](https://devnet-explorer.multiversx.com/)
- [Template dApp](https://github.com/multiversx/mx-template-dapp)
