# D19 — Implementação

## Novos

| Arquivo | Papel |
|---|---|
| `src/world/fast_travel.rs` | Regras puras + testes |
| `src/ui/fast_travel_screen.rs` | UI Bevy |
| `tests/fast_travel_app_tests.rs` | Integração headless |
| `docs/development/d19/*` | Documentação |

## Alterados

- `GameState::FastTravel`; `FastTravelRequest`
- `save/systems.rs`: `offer_fast_travel`, `apply_fast_travel_requests`
- `assets/world/regions/vigil_gates.ron`: `vg_causeway_save`
- HUD / status / UI plugin

## Dados

Segundo altar em Broken Causeway (`vg_causeway_save`, ~x 1920), longe da
patrulha do sentry. Sem bump de Save V3.
