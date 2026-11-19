# N4-A — Auditoria inicial

## Git

- Branch `art/v1.2-protagonist`, HEAD local = remoto = `2a2cc85a8b664f65dba671a7693181154c2b14d9` (registro de publicação da N3.8).
- Árvore limpa. Só os arquivos pessoais de `Md-usario/` estão não rastreados, e foram preservados (nada escrito ou commitado).

## Lido

`CLAUDE.md`, `.claude/rules/`, `N3_8_ARQUITETURA_TERRARIA.md`, `N3_8_PLANO_MODULAR_N4.md`, `N3_8_RISCOS_ARTISTICOS.md`, e no código `VisualPlayback` (`src/player/visual/mod.rs`), `FacingDirection` (`src/world/physics.rs`), `HeldLayer` (`src/inventory/held_visual.rs`), `CapeLayer` (`src/inventory/visual.rs`), `animate_player_visual` e `anchor_for`.

Referências do proprietário **lidas, não alteradas**: `Md-usario/assets-user/prota/base/Ficha_de_Modelagem_do_Protagonista_Gotico.png` e `Ficha_Modular_do_Protagonista_Gotico.png`. Trazem a vista lateral do corpo neutro em A-pose e as peças isoladas (calça, botas, casaca, capa). São pequenas e geradas por IA: servem para proporção, não para recuperar pixels.

## Fonte aprovada do idle N3.5

`art-src/n3-5-fidelity/source/candidate_106.txt`: grade 80×128, um caractere de paleta por texel (chaves de `art-src/n3-4-original-protagonist/scripts/n34_palette.py`), 1320 texels opacos. `generated/candidate_106.png` é a mesma imagem renderizada. Pivô (40, 118) e densidade 2 (`generated/candidate.ron`).

A grade é uma redução de imagem gerada por IA: as chaves se misturam entre as peças (`p`, `l`, `o` e `d` aparecem na calça, nas botas e no cinto; tons de cabelo aparecem em sombras da camisa). Não dá para separar as peças só pela cor. A separação usa regiões de linha e coluna (`N4A_RECONSTRUCAO_ARTISTICA.md`).

## Hashes dos recursos protegidos (SHA-256, antes de qualquer mudança)

| Arquivo | SHA-256 |
|---|---|
| `art-src/n3-5-fidelity/generated/candidate_106.png` | `e8e4bfd3837ea58dbc1437ee5ff9f829f80702c8732dfba4044611d6da971984` |
| `art-src/n3-5-fidelity/source/candidate_106.txt` | `31e171da134102511e6ebdd34e73740572250bc2742401c63033a6634fd43fa2` |
| `art-src/n3-5-fidelity/generated/candidate.ron` | `b99639e10850df0a16845d5e91ddb6fe555d8de4668fcc59b56c9812410b9ee1` |
| `art-src/n3-6-animation-proof/generated/run6.png` | `e15803703a4d1d3c8e6da5d815f3a6a2d759839bbb966b6c8762de8b36864957` |
| `art-src/n3-8-production-run/stage/sprites/player/n35_idle.png` | `e8e4bfd3...71984` (igual ao N3.5) |
| `art-src/n3-8-production-run/stage/sprites/player/n36_run6.png` | `e1580370...64957` (igual ao run6) |
| `assets/animations/player.ron` | `7491f714364997b6278b7821438f3a101f039fcc3423790523116cc0f40c0c3a` |
| `assets/sprites/player/player_locomotion.png` | `6373c896d2fcde5e192728574e7a787adb859da4d63edd76df8b0aca6e648580` |
| `assets/sprites/player/player_combat.png` | `286735f6a500a0f8b10870a1f24bdfd503ec0140a03c0a4a247ea2eb56139637` |
| `assets/sprites/player/player_effects.png` | `9e08d365a01c07da3e10aa3c72383851b6b456c07edb325dd13af7a68dc58996` |
| `assets/data/camera.ron` | `e822c722d64b76d2536aff98b82f978d05c68aa62bd1dd0982f357aef113637c` |

No fim da N4-A, `git diff` desde `2a2cc85` não mostra nenhum desses arquivos (`N4A_TESTES.md`).
