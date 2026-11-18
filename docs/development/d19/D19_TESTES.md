# D19 — Testes

## Unitários (`fast_travel.rs`)

- Sites ordenados; destinos excluem origem.
- Recusa: não descoberto, mesmo altar, morto, busy, lock.
- Posição de chegada centrada no altar.

## Integração (`fast_travel_app_tests.rs`)

- Descobre A → sem menu; descobre B → menu; Enter → sala A; inventário
  intacto; `last_save_point` permanece o do descanso em B; F5/F9.
- Esc cancela; destino inexistente não muda a sala.

## Validação

`cargo fmt`, `clippy -D warnings`, `cargo test`, `cargo check` — OK.
