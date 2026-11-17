# D17 — Implementação: pontos de salvamento

## Escopo

- `ObjectKind::SavePoint` reutilizável (não consome pickup).
- Interagir (`D`): cura HP/MP, marca descoberta, define `last_save_point`, grava slot.
- Marcador ciano no mapa (M) após descoberta.
- F5 continua salvando em qualquer lugar (dev / acessibilidade).
- Um altar em `vigil_gates_approach` (`vg_approach_save`).

## Critérios de aceite

- [x] Altar interagível no mundo.
- [x] HP/MP reenchidos.
- [x] `discovered_save_points` / `last_save_point` gravados e restaurados.
- [x] Reutilizável.
- [x] Mapa mostra altares descobertos.
- [x] Sem bump de save version.
- [x] Sem arte definitiva / Hunyuan.

## Arquivos

**Criados:** `docs/development/d17/*`

**Modificados:**

- `src/world/graph.rs`, `exploration.rs`, `progress.rs`, `interactions.rs`
- `src/core/events.rs`, `core/mod.rs` — `SavePointActivated`
- `src/save/systems.rs`, `save/mod.rs` — `rest_at_save_point`
- `src/ui/world_map.rs`
- `assets/world/regions/vigil_gates.ron`
- `tests/world_objects_tests.rs`
- `docs/SAVE_SYSTEM.md`, `ROADMAP.md`, `CHANGELOG.md`, `DECISIONS.md`

## Fluxo

```text
D no altar → ExplorationState::interact → Interaction::SavePoint
  → SavePointActivated
  → rest_at_save_point (heal + WorldProgress + write slot)
```

## Limitações

- Sem animação/menu de save room.
- Hearts não são reenchidos (só HP/MP).
- Sem viagem rápida entre altares (próxima candidata).
- F5 não exige altar.
