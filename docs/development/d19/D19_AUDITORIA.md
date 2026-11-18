# D19 — Auditoria

## Estado Git (início)

- Branch: `art/v1.2-protagonist`
- HEAD: `a9b8f96` (D17/D18 + arte já no remoto)
- Working tree: só edição local do briefing em `Md-usario/` (não tocada)

## Reutilizado

| Peça | Papel |
|---|---|
| `ObjectKind::SavePoint` / `SavePointActivated` | Descoberta + descanso D17 |
| `discovered_save_points` | Lista de destinos |
| `last_save_point` | Só no descanso (não na viagem) |
| `RoomRequest` + `Arrival::Position` | Mesmo caminho do F9 |
| `TransitionLock` / `RoomStep` | Ciclo de salas V1.7 |
| UI Bevy pausada (`GameState`) | Inventário/loja como modelo |

## Lacuna

Um altar (`vg_approach_save`) e descanso sem menu de deslocamento.
