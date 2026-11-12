---
paths:
  - "src/**/*.rs"
---

# Arquitetura

## Camadas por módulo

| Tipo de arquivo | Pode importar Bevy? | Exemplos |
|---|---|---|
| Regra pura | **Não** (só std, serde, ron, thiserror, tracing) | `player/attributes.rs`, `player/progression.rs`, `player/state.rs`, `combat/damage.rs`, `combat/command_buffer.rs`, `combat/command_parser.rs`, `skills/{definitions,unlock,cooldowns,execution}.rs`, `inventory/{items,equipment,inventory}.rs`, `magic/elements.rs`, `enemies/{ai,definitions}.rs`, `world/{collision,progress,rooms}.rs`, `save/{serialization,save_manager}.rs`, `core/rng.rs` |
| Sistema/componente Bevy | Sim | `*/components.rs`, `*/systems.rs`, `player/movement.rs`, `combat/{hitboxes,attacks,command_input}.rs`, `magic/{casting,projectiles}.rs`, `enemies/{behavior,spawning}.rs`, `world/{physics,maps,camera}.rs`, `ui/*` |

Antes de adicionar `use bevy` num arquivo puro, crie um arquivo Bevy separado.

## Comunicação entre módulos

- Mensagens (`#[derive(Message)]`) em `src/core/events.rs`: `CommandResolved`,
  `SkillActivated`, `BasicAttackRequested`, `DamageDealt`, `EnemyDefeated`, `Notification`.
- Um módulo **não** altera componentes de outro módulo para aplicar efeitos:
  ele emite uma mensagem e o dono reage (ex.: combate emite `DamageDealt`;
  player e enemies aplicam o dano cada um no seu `receive_damage`).
- O parser de comandos nunca aplica efeitos; ele só classifica a entrada.

## Ordem do frame (`GameSet`, `src/app/states.rs`)

`Input → Commands → Actions → Physics → Combat → Resolution → Progression`
(só em `GameState::Playing`), depois `Presentation` (sempre).
Todo sistema novo deve entrar em um `GameSet`.

## Dados

Balanceamento e conteúdo ficam em `assets/data/*.ron` e `assets/maps/*.ron`,
carregados e validados por `GameData::load` (`src/core/resources.rs`).
Novo conteúdo não deve exigir mudança de código.
