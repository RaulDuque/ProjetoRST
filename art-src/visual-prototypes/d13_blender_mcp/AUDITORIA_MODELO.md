# Auditoria do modelo — `swordsman_rig_v007` (cópia lab)

**Arquivo lab:** `blender/d13_lab_swordsman.blend`  
**Original (intact):** `art-src/player/blender/swordsman_rig_v007_game_v1_5.blend` (mtime 2026-11-13)

## Inventário

| Tipo | Qtd |
|---|---|
| Mesh | 132 |
| Armature | 1 (`rig_v007`, 43 bones) |
| Camera | 14 (+ `D13_cam_ortho_side`) |
| Light | 4 stock (+ luzes D13 de estudo) |
| Empty | 1 (`light_rig`) |
| Actions | 18 (POSE_* + GAME_V15_*) |

## Reutilização (decisão)

**Não recriar o personagem.** O modelo já é modular em meshes nomeadas:

- Corpo/cabelo/rosto: `head`, `hair_*`, olhos, etc.
- Casaco: `coat`, `collar`, `sleeve_*`, `cuff_*`, vest/trousers/belt
- Capa: `half_cape`, `cape_clasp`, `cape_cord`, `cape_stud`
- Espada: `blade`, `blade_readable`, `sword_guard`, `sword_grip`, `sword_pommel`, `sword_collar`
- Armadura: `pauldron`, `pauldron_lame`
- Botas/luvas: `boot_*`, `glove_*`

Todos com modificador `ARMATURE` → `rig_v007`. Ossos de capa/casaco/espada presentes.

## Lacunas

- Sem `helmet` / `shield` / cuirass completa.
- Materiais `v4_*` EEVEE — bons para referência 3D; **não** substituem pixel D7-B/D12 na escala do jogo.
- Pose idle detalhada no jogo ainda depende do pipeline 2D.

## Coleções D13

`D13_body`, `D13_coat`, `D13_cape`, `D13_sword`, `D13_armor`, `D13_boots`, `D13_gloves` para hide/show modular.
