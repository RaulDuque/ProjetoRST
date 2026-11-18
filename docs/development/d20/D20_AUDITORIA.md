# D20 — Auditoria de inimigos

## Git

- Branch `art/v1.2-protagonist`, HEAD `a9b8f96` + trabalho D19 ainda local.
- D19 preservada (sem commit/push nesta etapa).

## O que já existia

| Peça | Estado |
|---|---|
| `EnemyDefinition` + RON | Só `bone_sentry` |
| `decide` Patrol/Chase/Attack/Wait/Flee | Melee funcional |
| Wind-up + hitbox corpo a corpo | Sim |
| Defend / Retreat / UseSkill | Enum; Defend/UseSkill = parado |
| Projéteis inimigos | Ausentes |
| Keep-distance | Ausente |

## Lacunas escolhidas

1. Arquétipo **à distância** (projétil + recuo se encurralado).
2. Arquétipo **manter distância** (skirmisher defensivo).
