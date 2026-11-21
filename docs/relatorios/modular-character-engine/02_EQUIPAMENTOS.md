# Modular Character Engine 02 — Equipamentos independentes

**Status:** AGUARDANDO APROVAÇÃO VISUAL
**Aprovação artística:** pendente

A execução técnica está concluída e validada. Ficam com o proprietário o julgamento visual
(empunhadura, peitoral, peças) e a decisão de integrar ao jogo.

## 1. Identificação
- Etapa: Modular Character Engine 02, equipamentos independentes
- Frente: modular-character-engine
- Branch: `art/v1.2-protagonist`

## 2. Data e hora da execução
2026-11-21, das 16:05 às 17:15 (-03). O início é aproximado: o briefing foi lido logo depois
do commit `dcaa881`, às 16:02.

## 3. Objetivo original
A abordagem híbrida foi aprovada como base experimental, e a articulada fica preservada como
experimento. O pedido tinha sete partes:
1. **Esqueleto:** refinar os marcos (idle + 6 de corrida), tirar as oscilações e criar uma
   transformação própria de mão e pulso.
2. **Empunhadura:** perfis por estado, com idle diagonal para baixo sem atravessar o chão e
   corrida levada para trás com oscilação controlada. Espelhamento, profundidade por pose e
   troca sem reiniciar.
3. **Camadas:** sem pixels de camisa no braço ou no cabelo, reaproveitando a N4 e a 02B.
4. **Contrato:** cobrir os 10 tipos de peça, com fontes próprias, pivôs, encaixes, ossos,
   profundidade e tipo de deformação (rígido, articulado, flexível) e variantes de
   perspectiva.
5. **Peitoral:** um único peitoral acompanhando as 7 poses, comparando três abordagens.
6. **Ferramentas:** uso prático do pixelfx e do ASF.
7. **Laboratório:** interativo, com os controles pedidos.

## 4. Prompt executado
`Md-usario/engine/Engine-02.md`, arquivo do proprietário. Ainda não está versionado
(`?? Md-usario/engine/`), e eu não o editei nem commitei.

## 5. Contexto técnico inicial
- A 01 estava publicada em `dcaa881` (AGUARDANDO APROVAÇÃO VISUAL).
- Rig puro com 15 marcos e 12 ossos, FK, IK, 5 camadas pela partição por cápsulas.
- Espada perpendicular ao antebraço e ombreira com `follow` 0,5.
- Contrato com 2 slots.
- Marcos estimados com cerca de ±2 texels.

## 6. Ferramentas, plugins, skills e versões
- Rust 2024 e Bevy 0.20.1, sem dependência nova.
- Agent Sprite Forge 0.6.0, executado de fato: `rig_animate.py`, funções do
  `pixel_motion.py`, `build_animation_clips.py` e `pixel_qa.py`.
- pixelfx 0.3.1, executado: `rig.js` `skeleton()` e `tools/act.mjs`, em Node v24.18.0.
- Python 3.11, Pillow e numpy no venv `~/.local/share/dc-art-tools/asf-venv`.
- Nenhuma API paga e nenhuma geração de imagem.

## 7. Arquivos consultados
- `docs/relatorios/ULTIMO_RELATORIO.md` e `modular-character-engine/01_PROTOTIPO.md`.
- `art-src/n4-sprite-machine/scripts/run_poses.py` (marcos N4-C),
  `stage/parts/run_c1/*.txt` (camadas N4-C), `stage/protagonist.ron`.
- `docs/development/n4/N4C_ESTRATEGIA_SEPARACAO.md` e `N4C_ORDEM_CAMADAS.md`.
- `art-src/n3-6-animation-proof/frames/video_048…058.png` (vídeo em alta resolução),
  `source/video_0*.txt` e `generated/metrics.json` (escala e deslocamento da redução).
- `art-src/n3-4-original-protagonist/scripts/n34_palette.py`.
- ASF: `codeart2d/examples/hero.rig.svg`, `hero.anim.json`, `rig_animate.py --help`,
  `pixel_motion.py` (`measure`, `load_frames`), `build_animation_clips.py --help`,
  `pixel_qa.py --help`.
- pixelfx: `src/core/rig.js` (`skeleton`), `poses.js`, `tools/render.mjs`, `tools/act.mjs`.
- `evidence/erro_corretion/etapa02b/`: consultada. É a 02B do Blender, com outras poses, e
  não serve para os quadros da corrida (ver seção 12).

## 8. Arquivos criados
- `src/player/visual/rig/{grip,chest,measure}.rs`
- `art-src/modular-character-engine/stage/history/rig_lab_01.ron` (cópia dos dados da 01)
- `art-src/modular-character-engine/stage/layers/run_{0,2,4}.txt`
- Scripts em `art-src/modular-character-engine/scripts/`:
  - `n4c_label_maps.py`
  - `run_external_tools.py`
  - `make_evidence_02.py`
  - `tools/asf_rig_crosscheck.py`
  - `tools/asf_motion_measure.py`
  - `tools/pixelfx_probe.mjs`
- `evidence/modular_character_engine/02_equipamentos/` (21 imagens/GIFs, `lab/` com 13
  capturas, `tools/` com 15 arquivos, `metrics.json`)
- `docs/development/modular_character_engine/02_EQUIPAMENTOS.md`
- este relatório

## 9. Arquivos modificados
- `src/player/visual/rig/{mod,skeleton,layers,equipment,data}.rs`
- `examples/modular_character_engine.rs`
- `tests/modular_character_engine_tests.rs`
- `art-src/modular-character-engine/stage/rig_lab.ron` e `README.md`
- `docs/ARCHITECTURE.md`, `docs/DECISIONS.md` (D-047),
  `docs/development/modular_character_engine/01_ARQUITETURA.md` (seção "Atualização (02)")
- `.claude/rules/testing.md`
- `docs/relatorios/INDICE.md` e `ULTIMO_RELATORIO.md`

## 10. Arquivos removidos
Nenhum. O teste temporário de varredura de raios (`tests/zz_scratch_sweep.rs`) foi criado e
apagado durante o ajuste, sem chegar a ser commitado.

## 11. Descrição detalhada das alterações

### Esqueleto
- **Joints:** 19. As 4 novas formam a caixa do tronco (`chest_back`, `chest_front`,
  `waist_front`, `waist_back`).
- **Corrida #0, #2 e #4:** marcos da N4-C, medidos no vídeo em alta resolução. O punho do #4
  foi corrigido para (51, 56), porque o da N4-C cai na calça.
- **Corrida #1, #3 e #5:** medi nos quadros do vídeo pela mesma redução.
- **Ombro:** acompanha o braço numa curva suave, e o pescoço fica estável.
- A 01 está preservada em `history/`.

### Mão e pulso
`rig::grip`:
- `GripProfile`: referência `Body`, `Forearm` ou `World`, ângulo da lâmina,
  `follow_forearm` e profundidade por quadro;
- `hand_frame`: referencial no punho com +x na lâmina, independente do antebraço.

Perfis:
- `sword_idle`: −55° no referencial do corpo, estável;
- `sword_run`: 200°, follow 0,15, atrás do corpo (`MainHandBehind`) nos quadros 3–5, quando o
  braço vai para trás.

### Camadas
- **#0, #2, #4:** mapas gerados das camadas da N4-C, com a camada visível de cima.
- **Idle, #1, #3, #5:** regras portadas do `classify` da N4-C:
  - raio menor do braço dentro do tronco;
  - os cinzas e marrons da sombra da manga contam como manga;
  - cabelo pelo círculo da cabeça, incluindo a tinta atrás do rosto;
  - a linha da cintura separa tronco e pernas;
  - braço de trás só fora do tronco.
- `ArmRadii` virou parâmetro, e ajustei os raios contra a N4-C.

### Contrato
- 10 slots e `DeformKind` (`Rigid`, `Articulated`, `Flexible`).
- Arte: `ItemArt` (`Sprite` ou `Model`).
- Montagem: `Mount` (`Bone`, `Hand`, `Torso`), mais `Segment` para os articulados.
- Validação `RigItem::check` com 6 tipos de problema.
- Profundidade com 16 slots.

### Peitoral
`rig::chest`:
- `TorsoCage` com bilinear e bilinear inversa;
- `ChestModel`: silhueta, rampa, gola, lamelas, crista com variante de perspectiva pela
  proporção da gaiola;
- `rasterize` (projetada) e `warp` (malha);
- a rígida é o raster do idle posicionado como sprite.

### Métricas
`rig::measure`: `loop_jerk`, `loop_angle_jerk`, `spread`, `spine_frame` e `centroid_near`.

### Laboratório
- Andar com setas ou A/D, liga e desliga de cada peça, abordagem do peitoral.
- Camadas uma a uma, painéis de comparação ocultáveis.
- Roteiro de 131 capturas, sendo 13 feitas apertando teclas reais, cada uma com verificação
  do estado.
- Exporta `rig_export.json` (poses para a ASF) e os rasters do peitoral (`raw/`).

## 12. Decisões técnicas e artísticas
- **Peitoral: abordagem projetada** (modelo vetorial desenhado na grade de cada pose).
  - A rígida fica estreita na corrida de 3/4.
  - A malha estica os texels.
  - O Blender exigiria exportar por pose, e a projeção 2D na gaiola cumpre o mesmo papel
    (como o `rig.js` do pixelfx desenha geometria em pixels).
- **02B não reaproveitada:** são camadas do manequim 3D em outras poses (Erro-Corretion), sem
  correspondência com os quadros da corrida aprovada. A separação reaproveitada é a da N4-C,
  que é dos mesmos quadros.
- **Espada atrás do corpo nos quadros 3–5:** a mão de perto vai para trás, e a lâmina por cima
  cruzava a perna e o quadril. Atrás do corpo ela lê como levada ao lado.
- **Ombro:** o deslocamento subiu (0,86 → 1,14 texels) porque agora acompanha o braço, como na
  arte. O tremor caiu (1,81 → 1,00).
- **Ferramentas incompatíveis não foram forçadas:**
  - a CLI `pixel_motion measure` exige o `frames.json` do pipeline de vídeo dela, então chamei
    as funções `measure_walk` e `legs_picture`;
  - o QA de chão do `rig_animate` (`ground_row`) não se aplica a um esqueleto sem pés, então o
    resultado ficou registrado como está.

## 13. Comandos relevantes executados
```bash
python3 -B art-src/modular-character-engine/scripts/n4c_label_maps.py
cargo test --lib -j 2 rig::
cargo test -j 2 --test modular_character_engine_tests -- --nocapture
cargo run --example modular_character_engine -j 2 > target/modular_character_engine/run.log
PATH=~/.local/share/dc-art-tools/asf-venv/bin:$PATH python -B art-src/modular-character-engine/scripts/run_external_tools.py
~/.local/share/dc-art-tools/asf-venv/bin/python -B art-src/modular-character-engine/scripts/make_evidence_02.py
cargo fmt && cargo fmt --check
cargo clippy --all-targets -j 2 -- -D warnings
cargo test -j 2
cargo check -j 2
git diff --check
```

## 14. Testes realizados
- **Unitários de `rig::*`:** 50 testes, entre eles:
  - grip: no punho, `follow`, referência `Body`, `Forearm` igual à 01, profundidade por quadro;
  - chest: bilinear inversa de ida e volta, gaiola mais larga gira o modelo e alarga o raster,
    contorno, warp identidade, modos;
  - contrato: todos os slots aceitos com montagem coerente e 6 casos recusados;
  - camadas: cabelo dentro e fora do tronco, raio da manga, mapas de rótulos, IoU, polígono;
  - measure: tremor, ângulo, deslocamento, centroide;
  - data: perfis de empunhadura inválidos.
- **Integração (`tests/modular_character_engine_tests.rs`, 11 testes):**
  - arte aprovada e peças existentes;
  - corrida 6 × 1/15 s;
  - marcos na arte (punho centrado na pele a menos de 2 texels);
  - mapas da N4-C contra a arte e concordância das regras;
  - todo quadro rotulado uma vez, sem camisa distante no braço;
  - espadas no punho, acima do chão, espelhadas, idle estável, oscilação de no máximo ±20°;
  - um peitoral cobrindo mais de 90 % da caixa do tronco nas 8 poses, além da camisa, só
    paleta, warp também;
  - marcos 02 mais estáveis que a 01;
  - ombreira no ombro;
  - troca sem mexer no relógio;
  - IK medida.
- **Laboratório:**
  - partição por quadro;
  - continuidade do quadro nas trocas e na virada;
  - 13 verificações dos controles reais;
  - `RESULT`.

## 15. Resultado dos testes
- `cargo fmt --check`: ok.
- `cargo clippy --all-targets -j 2 -- -D warnings`: limpo, depois de corrigir `manual_contains`
  e `manual_is_multiple_of`.
- `cargo test -j 2`: **57 suítes, 741 passaram, 0 falharam**.
- `cargo check -j 2`: ok.
- Laboratório: **`RESULT: PASS`**, 131 capturas. Partição com 0 texels diferentes nos 7
  quadros, seja pelo mapa da N4-C, seja pelas regras.
- Original × híbrido sem equipamento: 0 px nos 6 quadros da corrida e no idle #0. O idle #1 (a
  respiração) difere de propósito.
- `git diff --check`: limpo.

## 16. Métricas e comparações
- **Estabilidade (corrida, 01 → 02):**
  - tremor do ombro: 1,81 → **1,00** texel;
  - tremor do ângulo da espada: 47,8° → **5,8°**;
  - deslocamento do ombro: 0,86 → 1,14 (acompanha o braço de propósito).
- **Concordância regras × N4-C (IoU):** braço de perto 0,62 / 0,64 / 0,65; cabelo
  0,84 / 0,78 / 0,76.
- **ASF `rig_animate.py`:** 72/72 marcadores, erro médio de **0,16 texel** e máximo de 1,55
  (antebraço de trás nos quadros 2 e 4) em relação ao nosso FK Rust. QA da ASF: `seam` 0,996
  (aprovado), `ik_clamp` 0 (aprovado), `ground_row` reprovado (6 quadros sem pé no chão da ASF,
  esperado).
- **ASF `pixel_motion` (funções):** passo a cada 6 quadros, pisadas nos quadros 0, 5 e 11 (3
  ciclos), iguais na corrida aprovada e na híbrida equipada.
- **ASF `build_animation_clips`:** 4 ticks por quadro a 60 Hz, deriva máxima de 0,333 ms, 0
  avisos, `needs-visual-review`.
- **ASF `pixel_qa`:**
  - peitoral (6 poses de corrida e 2 de idle): `partial_alpha` 0, `off_palette` 0,
    `outline_gaps` 0, `l_corners` 4 (limite 10);
  - ombreiras: `partial_alpha` 0.
- **pixelfx:** mudar só o ângulo da arma em 190° move o antebraço 3,5°. Mover só a mão não
  muda a lâmina. `act.mjs`: 21 passos do sword-slash.
- **IK do articulado:** 2 tornozelos fora do alcance da perna do idle (quadro 1 A +2,94 e
  quadro 4 B +1,25 texels).
- **Código:** cerca de 5600 linhas no módulo rig, no exemplo e no teste (eram cerca de 3200 na
  01).

## 17. Problemas encontrados
- Dois marcos da N4-C fora da arte: o punho do #4 na calça e o cotovelo do #2 na borda da
  manga.
- A primeira versão das regras deixava o braço salpicado. A sombra da manga usa cores das
  paletas de cabelo e de calça.
- As camadas da N4-C tinham rosto nas camadas de braço.
- A medida "deslocamento do ombro" piorou por causa do movimento real. A métrica de tremor é
  a correta.
- Roteiro de teclas: a mesma tecla apertada várias vezes no mesmo quadro só contava uma vez, e
  a última captura não chegava ao disco antes de o app fechar.
- ASF: `--ss 1` é inválido; a CLI `measure` não aceita quadros prontos; o `pixel_qa` recusou
  canvases de tamanhos diferentes num mesmo `--review`; o `rig_animate` deu QA `ground_row`.
- O guard do projeto bloqueou `rm -rf` na pasta de evidências (regra conhecida).

## 18. Problemas corrigidos
- Marcos corrigidos e testados (punho centrado na pele).
- Sombras da manga tratadas como manga, cintura como divisão tronco/pernas, tinta do cabelo
  e raios ajustados contra a N4-C.
- Rosto devolvido ao tronco nos mapas da N4-C (círculo de raio 9).
- Métrica de tremor (`loop_jerk`) adotada, com o deslocamento ainda informado.
- Uma tecla por quadro no roteiro, com 30 quadros finais para os screenshots.
- `--ss` padrão; wrapper das funções do `pixel_motion` com 3 ciclos; `pixel_qa` por tamanho;
  QA da ASF registrado como está.
- Evidências reescritas sem apagar a pasta.

## 19. Problemas pendentes
- **Idle de perfil:** o braço de perto cobre quase todo o peitoral, que só aparece na borda
  da frente e de trás. A espada do idle aponta para a frente e para baixo e encosta na perna.
- **Peitoral:** a gola dourada sai em traços de 1–2 texels, o modelo é de diagnóstico (não
  aprovado) e nos quadros 3–5 a ombreira fica no meio do peitoral, porque o ombro de perto
  está ali na vista de 3/4.
- **Camadas:** os quadros #1, #3 e #5 usam regras, enquanto os vizinhos usam a N4-C. Pode haver
  bordas que mudam entre quadros (concordância por volta de 0,63 no braço).
- **Arte das peças:** espadas da N41-B não aprovadas e ombreiras derivadas e provisórias.
- **Sem arte:** escudo, braçais, capa, capacete, cinto, calças e botas só têm contrato e
  testes (nenhum item no RON).

## 20. Limitações técnicas
- **Flexíveis:** só a gaiola do tronco existe. Capa e calças flexíveis precisam de gaiolas
  próprias (costas e pernas).
- **Variantes de perspectiva:** só no modelo do peitoral (posição da crista). Sprites ainda não
  têm variantes.
- **Corpo híbrido:** continua preso aos quadros aprovados, então animação nova exige arte e
  marcos.
- **Walker do laboratório:** anda a 120 texels/s, e os pés deslizam (a física do jogo não
  entra no laboratório).
- **Modo articulado:** continua experimental e visualmente reprovado: peças do idle,
  serrilhado de rotação, cabelo caído. O peitoral entra nele só no modo rígido.

## 21. Evidências visuais
- **01_run_sword_grip:** corrida com o perfil `sword_run` (GIF e quadros).
- **02_idle_sword:** idle com espada, diagonal para baixo, em 2 quadros (respiração).
- **03_breastplate_all_poses.gif:** o peitoral projetado nas 8 poses.
- **03_breastplate_approaches.png:** rígida × malha × projetada em 7 poses.
- **04_swap_during_run:** 24 passos trocando espada, ombreira e peitoral sem reiniciar
  (legenda com quadro e itens).
- **05_combo_sword_pauldron_breastplate:** as três peças juntas.
- **05b_turn_around.gif:** virada no meio da corrida.
- **05c_mirror.png:** espelhado com esqueleto e soquetes.
- **05d_depth_layers.png:** camadas coloridas.
- **05e_layers_one_by_one.png:** cada camada sozinha e só equipamento.
- **06_equipment_isolated.png:** as 4 peças com pivô e o peitoral isolado nas 8 poses.
- **06b_exploded.png:** vista explodida.
- **07_compare_01_vs_02:** espada e ombreira 01 × 02 (GIF e PNG).
- **07b_skeleton_01_vs_02.png:** marcos 01 (vermelho) × 02 (ciano) sobre a arte.
- **07c_skeleton_motion.gif:** esqueleto, caixa do tronco e soquetes.
- **08_lab_interactive.png e `lab/ui_*.png`:** capturas reais do laboratório com teclas.
- **tools/:** os resultados das ferramentas externas (lista na seção 22).

## 22. Caminhos das imagens, GIFs e vídeos
`evidence/modular_character_engine/02_equipamentos/`:
- `01_run_sword_grip.gif`, `01_run_sword_grip_frames.png`, `02_idle_sword.gif`,
  `02_idle_sword_frames.png`
- `03_breastplate_all_poses.gif`, `03_breastplate_approaches.png`
- `04_swap_during_run.gif`, `04_swap_during_run_frames.png`
- `05_combo_sword_pauldron_breastplate.gif`, `05_combo_frames.png`, `05b_turn_around.gif`,
  `05c_mirror.png`, `05d_depth_layers.png`, `05e_layers_one_by_one.png`
- `06_equipment_isolated.png`, `06b_exploded.png`
- `07_compare_01_vs_02.gif`, `07_compare_01_vs_02.png`, `07b_skeleton_01_vs_02.png`,
  `07c_skeleton_motion.gif`
- `08_lab_interactive.png`, `lab/ui_00_start.png` … `lab/ui_12_comparison_hidden.png`
- `tools/`:
  - ASF `rig_animate`: `asf_rig_animate_vs_rust_fk.gif` / `.png`,
    `asf_rig_animate_review.png`, `asf_rig_animate_onion.png`, `asf_skeleton.rig.svg`,
    `asf_skeleton.anim.json`, `asf_rig_crosscheck.json`;
  - ASF `pixel_motion`: `asf_pixel_motion_legs_approved.png`,
    `asf_pixel_motion_legs_hybrid_equipped.png`;
  - ASF `build_animation_clips`: `asf_build_animation_clips_contact.png`,
    `asf_build_animation_clips_turn_test.png`;
  - ASF `pixel_qa`: `asf_pixel_qa_breastplate_run.png`, `asf_pixel_qa_breastplate_idle.png`;
  - pixelfx: `pixelfx_act_sword_slash.png`;
  - resumo: `external_tools.json`.
- `metrics.json`

As capturas brutas (131 PNG, `run.log`, `rig_export.json`, `raw/`, `tools/`) ficam em
`target/modular_character_engine/`, que não é versionado.

## 23. Links do GitHub
Já publicados no push `dcaa881..745e705`:
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/docs/development/modular_character_engine/02_EQUIPAMENTOS.md
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/modular_character_engine/02_equipamentos/05_combo_sword_pauldron_breastplate.gif
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/modular_character_engine/02_equipamentos/03_breastplate_approaches.png
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/modular_character_engine/02_equipamentos/04_swap_during_run.gif
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/modular_character_engine/02_equipamentos/01_run_sword_grip.gif
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/modular_character_engine/02_equipamentos/02_idle_sword.gif
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/modular_character_engine/02_equipamentos/07_compare_01_vs_02.gif
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/modular_character_engine/02_equipamentos/08_lab_interactive.png
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/modular_character_engine/02_equipamentos/tools/asf_rig_animate_vs_rust_fk.gif

## 24. Resultado final
- **Empunhadura:** a espada é segurada por perfis reutilizáveis, independentes do antebraço:
  - idle diagonal para baixo, acima do chão;
  - corrida levada para trás, com ±menos de 20° de oscilação, passando atrás do corpo quando
    o braço recua;
  - espelhamento e troca sem reiniciar.
- **Marcos:** refinados, com o tremor da espada 8 vezes menor e o do ombro quase pela metade.
- **Camadas:** reaproveitam a N4-C e continuam idênticas à arte texel a texel.
- **Contrato:** cobre os 10 tipos de peça e os 3 tipos de deformação.
- **Peitoral:** um único modelo acompanha as 7 poses. A abordagem projetada foi a melhor das
  três comparadas.
- **Ferramentas:** ASF e pixelfx foram executados de fato, e o `rig_animate.py` confirma o
  nosso FK.
- **Laboratório:** atende aos controles pedidos, verificados com teclas reais.

## 25. Recomendações para a próxima etapa
1. Avaliar `05_combo_*.gif`, `03_breastplate_approaches.png`, `01_run_sword_grip.gif`,
   `02_idle_sword.gif` e `08_lab_interactive.png`, ou jogar com
   `MCE_INTERACTIVE=1 cargo run --example modular_character_engine`.
2. Se o peitoral for aprovado como abordagem:
   - desenhar um modelo definitivo (silhueta, gola com faixas de 2 texels ou mais, ornamento)
     no mesmo formato;
   - criar a gaiola das costas para a capa.
3. Medir os quadros #1, #3 e #5 por camadas à mão, como a N4-C, para tirar a diferença entre
   regras e mapa.
4. Só depois disso integrar ao jogador atrás de uma flag experimental.

## 26. Decisões que dependem do proprietário
1. Aprovar ou não a empunhadura do idle e da corrida, incluindo a lâmina atrás do corpo nos
   quadros 3–5.
2. Aprovar a abordagem projetada para peitorais e o nível visual do modelo de diagnóstico.
3. Aceitar a ombreira no meio do peitoral nos quadros 3–5 (onde o ombro realmente está na vista
   de 3/4) ou pedir outra regra.
4. Decidir se a arte definitiva das peças vem de desenho à mão, da fábrica ou de outro
   caminho.
5. Autorizar a integração experimental ao jogo. Até lá o laboratório continua separado.

## 27. Estado do Git e publicação
- Branch: `art/v1.2-protagonist`
- Commit(s) da tarefa:
  - `8ef6e8d` feat(player): extend the modular character engine to independent equipment
  - `0179fb8` build(art): add practical pixelfx and agent sprite forge runs for the rig lab
  - `729ca9c` docs(art): add the modular character engine 02 equipment evidence
  - `6954734` docs(architecture): document the modular character engine 02 equipment
  - `745e705` docs(claude): describe the modular character engine 02 tests
  - O commit deste relatório, com `INDICE.md` e `ULTIMO_RELATORIO.md`, vem depois. O hash
    dele é o HEAD publicado.
- Atomicidade: código, dados e testes do primeiro commit dependem um do outro. As juntas novas
  e o contrato mudam o RON que os testes leem, então ficaram juntos para que cada commit
  compile e passe.
- Push: feito, sem force (`dcaa881..745e705`), seguido do push do commit do relatório.
- Local x remoto: sincronizado. `Md-usario/` não foi alterado nem commitado.
