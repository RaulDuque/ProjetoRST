# Auditoria do renderer / visual atual (D11)

## Caminho publicado (default)

- Plugin: `PlayerVisualPlugin` em `src/player/visual/mod.rs`
- Manifesto: `assets/animations/player.ron` via `MANIFEST_PATH`
- Playback puro: `playback` / `selection` / `sockets`
- Sistemas Bevy: `systems` + `vfx` (skills/guard/revive)
- Escala: `custom_size` a partir do manifesto; câmera `view_height` 405
- Filtro: sampler por imagem (docs recomendam nearest; evitar `ImagePlugin::default_nearest` global)

## Slots

| Funcionais (inventário) | Visuais held (V2.2) |
|---|---|
| Weapon, Offhand, Head, Body, Gloves, Boots, Accessory1–2 | Camadas held + sockets por frame |

Não há slots inventário novos no D11.

## Impacto da integração experimental

- **Não** altera `PlayerVisualPlugin` nem o manifesto publicado.
- Código puro novo: `modular_lab` (só regras + testes).
- Consumidor: `examples/modular_visual_lab.rs` + `assets/experimental/d11/`.
