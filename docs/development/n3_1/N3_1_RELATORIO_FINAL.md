# N3.1 — Relatório

A N3.1 responde com imagens se o manequim 3D que já existe vira pixel gótico detalhado, com animação convincente e equipamento modular, por um pipeline reproduzível. A resposta desta execução: o render ortográfico é legível como manequim; a redução automática perde o rosto e não separa os materiais; não houve pintura manual. O estilo não está aprovado. A N4 não começa.

## A. Estado inicial

Branch `art/v1.2-protagonist`. HEAD de referência `963cbb9`. A N3 já estava publicada. A N4 e a N5 não foram autorizadas.

## B. Ferramentas

Blender 5.2.4 LTS. Pillow 12.1.1, com o helper de vizinho do pipeline porque `Image.resize` NEAREST falha nesta versão. Pixelorama 1.2.4 baixado; o `--help` é o do Godot 4.7.1. Nenhum crate novo.

## C. Blender

O blend V2.2 foi aberto sem gravação. Ataque, subida do pulo, queda e o ciclo de caminhada de `animations.py` foram renderizados com a espada oculta e o acabamento `W_flat`. Detalhe em `N3_1_BLENDER.md`.

## D. Pixelorama

O binário abre o suficiente para imprimir a ajuda do Godot. Não houve importar, pintar e exportar. Ver `N3_1_PIXELORAMA.md`.

## E. Comparação

O gesto comum é o ataque do mesmo manequim. Caminho A é o render. Caminho B é a redução automática. Caminho C não foi executado. `same_gesture_attack.png`.

## F. Direção artística

A silhueta do casaco e da capa sobrevive a 102 px. O rosto não. Tecido e metal não se distinguem na paleta de 16 cores. Ver `N3_1_AVALIACAO.md`.

## G. Animações

O ataque publicado não tem antecipação. O pulo da malha mantém a capa atrás do corpo. O walk é o ciclo de 16 quadros, não o `run` do jogo. Não foi feito um idle de respiração novo.

## H. Capa

Na malha, a capa não atravessa o casaco nos quadros renderizados. No sprite da N3, o balanço do pulo foi invertido para ficar atrás do corpo virado à direita; a imagem do exemplo é a conferência.

## I. Armas

A espada não está no atlas. O soquete medido move a mão ao longo do golpe. A espada desenhada no exemplo, quando a fonte é o render, é o sprite geométrico da N3.

## J. Armaduras

Uma malha de casaco. A segunda armadura da linha de base N3 não tem equivalente neste rig.

## K. Resolução

Vista próxima isolada: altura 360, escala inteira 2× na janela 720. Célula de apresentação 256×192, figura de 102 texels. Célula de contrato 128×96, figura de 49 texels. Na captura, a caixa do corpo mede 223 px na vista próxima e 99 px na vista 405. A câmera do jogo permanece em 405.

## L. Protótipo

```text
python3 art-src/n3-proof/scripts/build_n3_proof.py
python3 art-src/n3-proof/scripts/build_n3_1_compare.py
cargo run --example n3_visual_proof
```

Tecla B alterna a linha de base, o pixel automático e o render bruto do ataque. O restante das teclas é o da N3. `N3_INTERACTIVE=1` não grava a sequência.

## M. Evidências

`docs/development/n3_1/evidence/`: renders brutos do ataque, redução automática, a folha do mesmo gesto, fundos, recorte do rosto, silhueta, sequências de ataque e de pulo, capa no pulo e no ataque, e o antes/depois contra a figura da N3. As capturas do exemplo entram na mesma pasta com prefixo `bevy_`.

## N. Métricas

Conversão do ataque: 0,66 s de máquina. Horas de artista: nenhuma. Ver `N3_1_METRICAS.md`.

## O. Testes

Registrados em `N3_1_TESTES.md` com a saída real da execução.

## P. Critérios

| Critério | Estado | Evidência |
|---|---|---|
| O render do Blender foi usado | Concluído | masters e `raw/attack_00.png` |
| O mesmo gesto foi comparado | Concluído | `same_gesture_attack.png`, só o ataque |
| Pixelorama instalado e validado | Parcial | binário e `--help`; sem sessão de pintura |
| Acabamento manual real | Não concluído | nenhuma pintura |
| Arquivo-fonte editável | Concluído | blend original intocado, scripts, atlas, manifesto, soquetes |
| A silhueta melhorou | Parcial | o casaco é mais específico que a cápsula; continua manequim |
| O rosto está mais legível | Não concluído | some na redução; `raw_face_crop.png` |
| O tecido se distingue do metal | Não concluído | paleta automática |
| O walk tem melhor transferência de peso | Parcial | o ciclo foi renderizado; não foi repintado |
| O jump está mais legível | Parcial | oito quadros da malha; arco curto |
| A capa não atravessa o corpo | Parcial | atrás do casaco na malha; o sprite da N3 depende da captura |
| A arma acompanha a mão | Parcial | soquete medido; a malha da espada não foi desenhada |
| As armaduras continuam alinhadas | Não concluído | não há segunda malha |
| Os equipamentos continuam modulares | Parcial | arma fora do corpo; uma armadura só |
| A resolução mantém pixels consistentes | Concluído | 2× vizinho na vista 360 |
| O protótipo abre | Concluído | capturas `bevy_` nesta pasta e na N3 |
| Os testes passam | Concluído | `N3_1_TESTES.md`, saída 0 |
| As evidências foram publicadas | ver o push | este diretório |
| Os arquivos originais foram preservados | Concluído | blend sem gravação; `white_mesh.glb` intocado |

## Q. Limitações

Sem pintura manual, sem desenho independente, sem segunda armadura, sem antecipação no ataque, sem idle de respiração, sem trocar a câmera do jogo, sem promover o estilo.

## R. Arquivos

Scripts em `art-src/n3-proof/scripts/`, atlas e manifesto em `art-src/n3-proof/n3_1/`, exemplo `examples/n3_visual_proof.rs`, testes `tests/n3_1_visual_proof_tests.rs`, estas notas. O binário do Pixelorama não entra no git.

## S. Segurança

`swordsman_rig_v007.blend` não foi salvo. `white_mesh.glb` não foi aberto. `camera.ron`, o save v3, os controles da N2 e o protagonista publicado não foram substituídos.

## T. GitHub

- `fa7fcd4` feat(visual): compare the V2.2 attack inside the isolated proof
- `354ad9e` docs(n3): record the mannequin comparison without approving it
- O commit que grava este parágrafo é o registro da publicação. O hash remoto é o que o push deixar em `origin/art/v1.2-protagonist`.

## U. Recomendação

Não promover esta figura para o jogo. O passo que ainda falta, se o proprietário quiser continuar a experiência, é pintura manual de verdade sobre estes renders, no Pixelorama ou por um artista. Repetir a redução automática não responde a pergunta.

## V. Aprovação pendente

O proprietário olha `same_gesture_attack.png`, `raw_face_crop.png`, `before_after_n3.png` e `jump_sequence.png` e decide se a direção segue. Esta nota não aprova o visual.
