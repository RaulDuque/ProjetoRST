# D18 — Testes

## Unitários (`src/inventory/consume.rs`)

- Restaura HP e consome 1.
- Clamp no máximo; recusa com recurso cheio sem consumir.
- Restaura MP.
- Recusa morto / não-consumível / não-possuído; `use_allowed`.

## Integração (`tests/consumable_app_tests.rs`)

- `inventory_enter_uses_tonic_and_persists` — ferir, `I`, selecionar tonic,
  Enter, quantidade−1, HP sobe, F5/F9 / slot.
- `full_hp_refuses_tonic_without_consuming` — “Already at full”.
- `ether_restores_mp` — MP baixo → ether → MP sobe, qty 0.

## Regressão

`cargo fmt`, `CARGO_BUILD_JOBS=2 cargo clippy --all-targets -- -D warnings`,
`CARGO_BUILD_JOBS=2 cargo test`, `cargo check` — aprovados após D18
(incluindo rotas de equipamento e vila ajustadas ao ether na loja).

## Manual

Ver `D18_RELATORIO_FINAL.md`.
