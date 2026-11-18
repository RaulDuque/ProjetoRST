# D20 — Implementação

## Arquivos

| Arquivo | Mudança |
|---|---|
| `src/enemies/ai.rs` | `AiStyle`, `decide` por estilo |
| `src/enemies/behavior.rs` | Projéteis inimigos + windup de dados |
| `assets/data/enemies/castle_undead.ron` | `bone_bolt`, `ash_warden` |
| `assets/world/regions/vigil_gates.ron` | Bolt no shaft (ledge); Warden na gallery |
| `tests/enemy_archetype_tests.rs` | Integração |
| `tests/world_*` | Expectativas de contagem no shaft |

## Spawns

- **Bone Bolt** — Watch Shaft, plataforma ~y 409 (rotas de chão livres).
- **Ash Warden** — Upper Gallery ~x 1600 (leste da relíquia).
