
# gRPC Say Hello

<h1 align="center">
  <img src="./assets/banner.webp" alt="Box" width="600px" />
</h1>

Este projeto demonstra uma aplicação simples usando gRPC com comunicação entre múltiplos servidores e clientes em Rust. Ele permite que você inicie múltiplos servidores gRPC em diferentes portas e que envie mensagens entre eles via terminal.

## Como Rodar

### Servidores

1. Abra um terminal e inicie o primeiro servidor:

   ```bash
   cargo run server 50051
   ```

   Isso iniciará um servidor gRPC escutando na porta `50051`.

2. Em outro terminal, inicie um segundo servidor: `50052`:

   ```bash
   cargo run server 50052
   ```

### Enviar Mensagens entre Servidores

Para enviar uma mensagem de um servidor (simulado pelo cliente) para o outro:

1. Abra um terceiro terminal.
2. Envie uma mensagem do servidor na porta `50051` para o servidor na porta `50052`:

   ```bash
   cargo run client 50051 50052 "Hello 52, I am 51"
   ```

## Comandos

- Iniciar um servidor:

  ```bash
  cargo run server <porta>
  ```

- Enviar uma mensagem entre servidores:

  ```bash
  cargo run client <porta_origem> <porta_destino> <mensagem>
  ```