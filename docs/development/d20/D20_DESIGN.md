# D20 — Design

## Arquétipos entregues

| Id | Estilo | Comportamento |
|---|---|---|
| `bone_sentry` | `melee` (existente) | Patrulha → persegue → golpe corpo a corpo |
| `bone_bolt` | `ranged` | Mantém faixa; dispara projétil; recua se `min_range` |
| `ash_warden` | `keep_distance` | Recua se encurralado; segura faixa; golpe curto |

## Dados (`AiProfile`)

Novos campos com default (compatível com RON antigo):

- `style`, `preferred_range`, `min_range`, `projectile_speed`, `windup`

## Ataque à distância

Mesmo wind-up/telegraph laranja; ao completar, spawna `Projectile` + `Hitbox`
(`Team::Enemy`). Colisão com sólidos via `magic::projectiles` (já existente).
