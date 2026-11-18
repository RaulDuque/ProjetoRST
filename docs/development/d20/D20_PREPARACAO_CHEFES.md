# D20 — Preparação para chefes (D21)

## Já existe

- Definição orientada a dados + spawn por sala.
- IA por perfil (`AiProfile` / estilos).
- Dano, knockback, morte → `EnemyDefeated` (EXP/drops).
- Respawn temporizado; limpeza em mudança de sala.
- Portas com requisitos; flags de progresso.
- Hitboxes/hurtboxes genéricos.

## Falta para um chefe mínimo

| Necessidade | Sugestão mínima |
|---|---|
| Id/flag de chefe | Campo `boss: true` ou `kind: Boss` na definição |
| Morte persistente | Não enfileirar respawn; gravar id em `WorldProgress` |
| Fases por HP | Tabela `(threshold, AiProfile \| attack_set)` em dados |
| Arena / portas | `TransitionLock` + requisitos de porta enquanto vivo |
| HP UI | Barra Presentation lendo `EnemyHealth` do boss marcado |
| Recompensa única | Pickup/chest já cobertos; amarrar ao flag de derrota |
| Seleção de ataques | Extender `Attack` → padrão nomeado em RON (sem FSM nova) |

## Não antecipar na D20

Sem sistema de fases, sem barra de chefe, sem arena trancada — só este diagnóstico.
