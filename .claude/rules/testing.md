---
paths:
  - "src/**/*.rs"
  - "tests/**/*.rs"
---

# Testes

- **Regras puras**: testes unitários (`#[cfg(test)] mod tests`) ou de integração em `tests/`. Nunca inicialize a engine para testar fórmula.
- **Integração** (`tests/`):
  - `attributes_tests.rs`, `progression_tests.rs`, `combat_tests.rs`, `command_parser_tests.rs`, `inventory_tests.rs`, `save_tests.rs`: domínio.
  - `data_tests.rs`: valida os arquivos RON reais de `assets/`.
  - `app_smoke_tests.rs`: app Bevy real **headless** (`MinimalPlugins`), detecta conflitos de query e verifica fluxos (ex.: comando de magia gasta MP e cria projétil). Teclas são injetadas após `InputSystems` via `PendingKeys`.
- **Fumaça visual**: `cargo run --example autoplay` (precisa de sessão gráfica) imprime os resultados de comandos/dano/derrotas e salva screenshots em `target/autoplay/`. Abra os PNGs para conferir.
- Aleatoriedade sempre injetada (`DamageRolls`, `GameRng::seeded`) para testes determinísticos.
- Toda correção de bug ganha um teste que falhava antes.
- Nunca enfraqueça uma asserção para "fazer passar"; investigue.
