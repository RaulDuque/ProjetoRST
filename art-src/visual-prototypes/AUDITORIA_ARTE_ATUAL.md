# Auditoria da arte atual (laboratório)

Fonte: docs + `assets/` + `art-src/player/` + código de visual. Sem alterações no jogo.

## Produção do protagonista

- **Blender 5.2.4 LTS** (modelo/rig **v007**), EEVEE.
- Módulos de jogo: V1.3 locomoción → V1.4 clips extras → V1.5 polish/VFX → **V2.2** corpo sem sabre + sockets.
- Publish: `assets/animations/player.publish.json` (pipeline 2.2, state `1cd66a2977d0`).
- Densidade 2: locomoción 128×128 tex (=64×64 u), combate 256×192 (=128×96 u).
- Corpo de colisão `PLAYER_SIZE` (26, 46); ~52 u de silhueta; câmera `view_height` 405 → ~92 px na janela 720.

## Animações publicadas (contagem de quadros)

Locomoção 64 | Combate 58 | Efeitos 44 — ver `assets/animations/player.ron`.

## Armas / armaduras / capa

- Âncoras por quadro em `player_sockets.ron`; runtime `held_visual` + `held_art` (silhuetas).
- Categorias: Sword, Dagger, Axe, Spear, HeavyWeapon (+ escudo offhand).
- Capa de item: textura 24×64 gerada, socket `back`.
- Head/Gloves/Boots: sem arte dedicada.

## Efeitos

- PIL `vfx_v1_5.py` → atlas effects. Ember Bolt pede `fire_bolt` **sem clip**.

## Forma espectral

- Tint/alpha/outline/motes em runtime; sem sprites novos.

## HUD

- Lenda de teclas **permanente** no painel esquerdo (`hud.rs`).

## Inimigos

- Um arquivo de definição (`bone_sentry`); visual = retângulo colorido.
