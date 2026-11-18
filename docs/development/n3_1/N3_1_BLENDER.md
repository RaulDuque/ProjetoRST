# N3.1 — Renders do Blender

Fonte: o blend V2.2, aberto com `--factory-startup` e sem gravação. Acabamento `W_flat`, o mesmo toon do pipeline publicado. Espada oculta. Fundo transparente.

| Clipe | Quadros | Saída | Hash dos keyframes |
|---|---|---|---|
| `attack_basic` | 7 | `art-src/player/renders/v2_2_weaponless/v1_3/masters/attack_basic/` | bateu com o módulo |
| `jump_rise` | 4 | `.../v1_4/masters/jump_rise/` | bateu |
| `fall` | 4 | `.../v1_4/masters/fall/` | bateu |
| `walk` | 16 | `art-src/n3-proof/blender/walk/` | pose aplicada por `animations.py`, não é o `run` publicado |

A primeira tentativa do walk falhou: o script não colocava `swordsman_v003` e `swordsman_v004` no `sys.path`, e `params` não era encontrado. O blend não foi salvo. A segunda execução inclui esses diretórios.

Cada master de ataque mede 1024×768. A figura opaca, depois do BOX para a célula de contrato 128×96, tem 49 texels de altura. Em 2× vizinho, 102 texels. Os números estão em `art-src/n3-proof/n3_1/metrics.json`.

Cópias dos masters de ataque: `art-src/n3-proof/n3_1/raw/attack_00.png` até `attack_06.png`.

Não foi criado um quadro de antecipação. Inventar essa pose mudaria o clipe publicado.
