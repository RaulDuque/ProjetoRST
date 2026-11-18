# D20.1 — Regressão

Executado em sequência (um Cargo por vez), 2026-11-17:

| Comando | Resultado | Log |
|---|---|---|
| `cargo fmt` (após `--check` com diff) | OK | `target/d20_1/fmt.log` |
| `CARGO_BUILD_JOBS=2 cargo check` | OK | `target/d20_1/check.log` |
| `CARGO_BUILD_JOBS=2 cargo clippy --all-targets -- -D warnings` | OK (0) | `target/d20_1/clippy.log` |
| `CARGO_BUILD_JOBS=2 cargo test` | OK (0) | `target/d20_1/test.log` |

## Suites relevantes D17–D20

- Save / world integration / stress — PASS  
- `tests/fast_travel_app_tests.rs` — PASS  
- `tests/enemy_archetype_tests.rs` — PASS  
- Rotas world (main/town/spectral/secrets/equipment) — PASS  

## Alterações de tooling nesta etapa

- `tests/support/route_driver.rs`: Expects `FastTravelOpen`, `HpBelowMax`, `LastSavePoint`, `EnemyAlive`  
- `tests/support/d20_1_route.rs`, `examples/d20_1_probe.rs`: probe visual  

Nenhuma correção de lógica de jogo P0/P1 foi necessária após a validação visual.
