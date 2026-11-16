# Contrato de exportação experimental (D5)

## Célula lab

| Campo | Valor |
|---|---|
| Render Blender | 256×256, EEVEE, `film_transparent`, câmera `side` |
| Célula composição | 128×160 RGBA |
| Clip | `run`, 10 frames, 24 fps (contrato V1.3/jogo) |
| Nomenclatura | `camadas/<layer>/run_fXX.png` |

## Camadas

| Pasta | Objetos Blender |
|---|---|
| `body/` | todas as meshes exceto EQUIP |
| `light/` | `coat` |
| `cape/` | `half_cape`, `cape_clasp`, `cape_cord`, `cape_stud` |
| `heavy/` | `pauldron`, `pauldron_lame` |
| `weapon/` | `blade`, `blade_readable`, `sword_*` |
| `head/`, `shield/` | props 2D estáticos (não malha) |

EQUIP = sword ∪ cape ∪ coat ∪ heavy.

## Reprodutibilidade

- Poses: `game_sprites_v1_3.clips.build_action(arm,"run")` — determinístico.
- Compose: `compose_d5.py` — determinístico (testado em `testes/modular_tests.json`).
- EEVEE pode variar levemente entre GPUs/drivers; nesta máquina os PNGs
  regenerados com os mesmos inputs devem bater no compose.

## Coordenadas

- Blender: +X frente do modelo, +Z up; câmera side com `turn_rig(-90)`.
- Bevy sockets: u a partir da sola, x frente, y up (`player_sockets.ron`).
- Lab props: bbox do body na célula 128×160 (aproximação).
