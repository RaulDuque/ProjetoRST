# Sistema de encaixes D15 (3D) ↔ sockets 2D existentes

## Fontes

- Rig 3D: `art-src/player/RIG_SPECIFICATION.md` (`rig_v007`)
- Sockets 2D por quadro: `assets/animations/player_sockets.ron` (V2.2)
- Medição piloto: `sockets/v007_bone_sockets.json`

## Mapa socket → osso

| Socket lógico | Osso v007 | Tipo típico de peça | Deformação |
|---|---|---|---|
| `weapon` | `sword` (filho de `hand.R`) | Armas rígidas | **Rígido** (parent bone) |
| `hand_r` | `hand.R` | Fallback arma / item mão | Rígido |
| `hand_l` | `hand.L` | Escudo, offhand | Rígido |
| `head` | `head` | Elmo, máscara, chapéu | Rígido ou skinned leve |
| `chest` / tronco | `chest` | Peitoral rígido | Rígido ou skinned |
| `hips` | `pelvis` | Cintos, saias curtas | Skinned / cloth |
| `legs` | `thigh.*` / `shin.*` | Grevas | Skinned |
| `foot_r` / `foot_l` | `foot.R` / `foot.L` | Calçados | Skinned |
| `back` / capa | `cape_back_01` (+ `cape_arm.L_*`) | Capas, mantos | **Deformável** (cloth/bones) |

## Rígido vs deformável

**Rígido:** a peça acompanha um único osso/socket (espada, elmo simples).
Parent em `sword` ou `hand.R`; sem weights.

**Deformável:** armadura de tecido, capa, botas — precisa de skinning (ou
ossos auxiliares COAT/CAPE já existentes no rig). Importar um GLB solto
**não** torna a peça modular animável.

## Eixos do modelo

- Personagem olha **+X**, **+Z** para cima, sola em z = 0.
- Direita do personagem = lado da câmera lateral (−Y) = sufixo `.R`.

## Estado D15

- Empties `socket_*` criados na cena piloto (seguem ossos).
- Validação animada (run/attack) **não** executada nesta fase.
- Sockets 2D do jogo **não** foram alterados.
