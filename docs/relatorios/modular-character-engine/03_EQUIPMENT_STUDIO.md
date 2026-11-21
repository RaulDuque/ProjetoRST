# Modular Character Engine 03 — Equipment Studio

**Status:** AGUARDANDO APROVAÇÃO VISUAL
**Aprovação artística:** pendente

A ferramenta, o modelo genérico e as provas foram executados e validados. Ficam com o
proprietário o julgamento da arte dos Modelos A e B, da espada e do cinto, e a decisão sobre os
próximos passos. Nada foi integrado ao jogo.

## 1. Identificação
- Etapa: Modular Character Engine 03, Equipment Studio
- Frente: modular-character-engine
- Branch: `art/v1.2-protagonist`

## 2. Data e hora da execução
2026-11-21, das 17:20 às 18:40 (-03). O início é aproximado: o briefing foi lido depois do
commit `63ffb91`, às 17:15.

## 3. Objetivo original
- A 02 foi aprovada tecnicamente (abordagem híbrida, empunhadura, projeção), mas a arte dos
  peitorais e das ombreiras foi reprovada.
- O pedido foi criar uma ferramenta própria em Rust/Bevy (`cargo run --example
  equipment_studio`). Ela deveria ter um modelo genérico de partes (geometria, transformação,
  pivô, material, profundidade, contorno, regiões, ornamentos, vínculos, visibilidade,
  perspectiva) e um editor visual com as funções listadas no briefing.
- Provas pedidas:
  - dois peitorais com estruturas diferentes;
  - uma espada a partir de PNG;
  - um terceiro item feito só por dados ou pelo editor, sem mudar Rust.
- A capa era opcional e podia ficar para a Engine 04.

## 4. Prompt executado
`Md-usario/engine/Engine-03.md`, arquivo do proprietário. Não está versionado
(`?? Md-usario/engine/`), e eu não o editei nem commitei.

## 5. Contexto técnico inicial
- A 02 estava publicada em `63ffb91`: rig, empunhadura, gaiola do tronco, `ChestModel` com
  silhueta, gola, lamelas e quilha, laboratório e camadas da N4-C.
- Não havia modelo genérico, editor nem salvamento de equipamento.

## 6. Ferramentas, plugins, skills e versões
- Rust 2024 e Bevy 0.20.1, sem dependência nova (o `ron` e o `serde` já existiam).
- Agent Sprite Forge 0.6.0, executado: `render_pixelspec.py`, `svg_render.py --crisp` (com o
  Chrome do sistema) e `pixel_qa.py`.
- pixelfx 0.3.1: o pintor do `rig.js` foi estudado e o modelo de sombreamento portado.
- Python 3.11 + Pillow (venv `~/.local/share/dc-art-tools/asf-venv`).
- Nenhuma chamada paga.

## 7. Arquivos consultados
- `docs/relatorios/ULTIMO_RELATORIO.md`, `modular-character-engine/01_PROTOTIPO.md` e
  `02_EQUIPAMENTOS.md`.
- `src/player/visual/rig/*` e `src/core/pixel_image.rs`, `src/core/pixel_art.rs`.
- `examples/modular_character_engine.rs`.
- ASF: `codeart2d/examples/slime.pixelspec.json` (esquema da PixelSpec),
  `render_pixelspec.py --help`, `svg_render.py --help`, `pixel_qa.py --help`.
- pixelfx: `src/core/rig.js` (pintor: normal × luz, tinta × emenda), já estudado na 01.
- APIs do Bevy 0.20 no registro do cargo: `Assets::insert`, `Window::cursor_position`,
  `AccumulatedMouseScroll` e `Image::from_buffer`.

## 8. Arquivos criados
- `src/player/visual/rig/design.rs`: modelo genérico, validação, RON, Chaikin, adaptador.
- `src/player/visual/rig/paint.rs`: pintor genérico, `pose_map`.
- `examples/equipment_studio.rs`: o editor e os roteiros `--tour` e `--tour-reload`.
- `tests/equipment_studio_tests.rs`.
- `art-src/equipment-studio/`:
  - `README.md`;
  - `items/{gothic_cuirass, leather_jerkin, arming_sword, studio_item_1}.ron`;
  - `sources/{buckle.pixelspec.json, emblem.svg}`;
  - `import/{buckle_iron, buckle_brass, emblem_gothic, sword_arming_cross}.png`;
  - `scripts/{build_imports.py, make_evidence_03.py}`.
- `evidence/modular_character_engine/03_equipment_studio/`: 15 imagens e GIFs, `editor/` com 5
  capturas, `tools/` com 3 arquivos, `09_studio_item_1.ron` e `metrics.json`.
- `docs/development/modular_character_engine/03_EQUIPMENT_STUDIO.md`.
- Este relatório.

## 9. Arquivos modificados
- `src/player/visual/rig/{mod,data,equipment,grip,layers,skeleton}.rs`: `Serialize` nos enums
  usados pelos designs e `RigClip::cut_layers` compartilhado.
- `examples/modular_character_engine.rs`: passou a usar `cut_layers`, e a cópia do mesmo laço
  saiu.
- `docs/ARCHITECTURE.md`, `docs/DECISIONS.md` (D-048) e
  `docs/development/modular_character_engine/02_EQUIPAMENTOS.md` (seção "Atualização (03)").
- `.claude/rules/testing.md` e `.claude/rules/architecture.md`.
- `docs/relatorios/INDICE.md` e `ULTIMO_RELATORIO.md`.

## 10. Arquivos removidos
Nenhum versionado. O `studio_item_1.ron` gerado nas primeiras execuções do roteiro (com erros
de navegação) foi apagado e refeito antes do commit.

## 11. Descrição detalhada das alterações

### Modelo genérico (`rig::design`)
- **`EquipmentDesign`:** slot, espaço (`Torso`, `Hand` ou `Bone`), profundidade, `mount`
  (transformação do encaixe), luz, faixa de perspectiva, materiais e partes.
- **`Part`:**
  - fonte `Polygon` (com `smooth` = passes de Chaikin, que fazem curvas) ou `Image` (PNG);
  - transformação com pivô, material, camada, contorno e bisel;
  - regiões (outro material ou tom) e ornamentos (rebite, linha, linha tracejada);
  - visibilidade por clipe e por faixa de giro, que serve como variante de perspectiva.
- Validação com 9 erros, RON de ida e volta, `free_id` e o adaptador `from_chest_model` da 02.

### Pintor (`rig::paint`)
1. Mapeia cada texel da célula para o espaço do design e depois para cada parte.
2. A parte de cima vence.
3. A normal vem do cilindro do tronco somado ao bisel, e a luz é quantizada numa rampa de 5
   tons:
   - couro pontilha só perto das trocas de tom;
   - metal tem especular;
   - regiões e ornamentos entram por cima.
4. Contorno: tinta na silhueta e emenda sobre as partes de baixo. O PNG vira decalque
   (pixel-alinhado) no tronco e é girado por mapeamento inverso no espaço da mão.
5. `pose_map` dá espaço, profundidade e giro de um design num quadro.

### Editor (`examples/equipment_studio.rs`)
- **Tela:**
  - painel com designs, partes, propriedades, material e tom, encaixe, clipe e status;
  - vista principal com zoom inteiro de 2× a 8×, prévias em 2× e 1×;
  - gizmos da parte selecionada (contorno, vértices, pivô) e do encaixe.
- **Modos:** parte, vértice, pivô, encaixe e cor.
- **Operações:**
  - adicionar, remover, duplicar, esconder e reordenar partes;
  - importar PNG de `import/`;
  - novo design, trocar de slot, salvar, recarregar;
  - equipar e desequipar, item isolado, personagem sem equipamento;
  - idle e corrida, pausa e quadro a quadro, espelho, esqueleto e gaiola;
  - arrastar com o mouse.
- **Roteiros:** `--tour` faz 61 passos com teclas reais pelo `ButtonInput`, cada um com
  verificação de estado. `--tour-reload` é o segundo processo.

### Designs
- **Modelo A (9 partes):** costeira escura, três lamelas segmentadas com barra dourada, placa
  lateral angular com rebites, peitoral angular com barra dourada, quilha dourada, gola
  dourada (curva) e emblema (SVG pelo ASF) visível só de 3/4.
- **Modelo B (8 partes):** jaqueta de couro longa e arredondada com barra assimétrica e
  costuras tracejadas, gola enrolada (curva), placa de ferro alta na frente e placa pequena
  atrás (assimetria), tira diagonal com costura, tira da cintura e duas fivelas (PixelSpec pelo
  ASF), uma delas só de 3/4.
- **Espada:** PNG da N41-B girado −90° no espaço da mão, mais um rubi poligonal no pomo.
- **Cinto `studio_item_1`:** feito só pelo editor no roteiro: Ctrl+N, slot `Belt`, vértices
  formando uma faixa, três tons recoloridos de aço para couro, fivela de latão importada e
  posicionada, Ctrl+S.

### Compartilhamento
`RigClip::cut_layers` (puro) corta as camadas do corpo para o editor e para o laboratório.

## 12. Decisões técnicas e artísticas
- **Pintar por pose, direto na grade de texels**, como na projeção aprovada na 02. A mesma
  fonte acompanha as 8 poses sem reamostrar e fica nítida em 1×.
- **Modelo de sombreamento do pixelfx**, em vez de cores chapadas: os biséis e o cilindro dão
  volume com poucas regras.
- **SVG pelo ASF, offline:** o editor não lê SVG para não acrescentar dependência. O caminho
  SVG → PNG crisp → parte importada está provado com o emblema.
- **Importação sem seletor de arquivo:** percorre a pasta `import/`. Um seletor nativo
  exigiria um crate novo.
- **Terceiro item pelo próprio editor:** foi feito com teclas reais e salvo pelo Ctrl+S, e o
  commit `350e615` contém só esse RON.
- **Capa adiada para a Engine 04:** arquitetura descrita em `03_EQUIPMENT_STUDIO.md` §6. O
  briefing permitia, e a prioridade era o editor e os peitorais.
- **Cópia da espada da N41-B em `import/`:** a pasta de importação é a fonte do editor. O PNG é
  idêntico ao original.

## 13. Comandos relevantes executados
```bash
PATH=~/.local/share/dc-art-tools/asf-venv/bin:$PATH python -B art-src/equipment-studio/scripts/build_imports.py
cargo test --lib -j 2 rig::
STUDIO_DUMP=1 cargo test -j 2 --test equipment_studio_tests -- --nocapture
cargo run --example equipment_studio -j 2 -- --tour > target/equipment_studio/tour.log
cargo run --example equipment_studio -j 2 -- --tour-reload > target/equipment_studio/tour_reload.log
cargo run --example modular_character_engine -j 2 > target/modular_character_engine/run.log
~/.local/share/dc-art-tools/asf-venv/bin/python -B art-src/equipment-studio/scripts/make_evidence_03.py
cargo fmt && cargo fmt --check
cargo clippy --all-targets -j 2 -- -D warnings
cargo test -j 2
cargo check -j 2
git diff --check
```

## 14. Testes realizados
- **Unitários novos:**
  - `design` (6): ida e volta em RON, 7 tipos de design inválido recusados, transformações
    inversíveis com pivô fixo, Chaikin, visibilidade e giro, adaptador;
  - `paint` (7): alfa binário e paleta na gaiola, tinta nas bordas, gaiola mais larga, parte de
    cima e partes escondidas ou só de idle, vértice mudando a silhueta, PNG mantendo os pixels,
    encaixe movendo o item, linhas;
  - `data` (1): `cut_layers` sem perda.
- **Integração (`tests/equipment_studio_tests.rs`, 7):**
  - designs carregam, validam e fazem ida e volta, sem referência à arte do personagem;
  - a pasta contém A, B, a espada e um item salvo pelo editor;
  - cada design pinta as 8 poses com alfa binário e só cores próprias;
  - A × B diferem em estrutura;
  - a espada mantém os pixels do PNG no punho;
  - o adaptador da 02 funciona;
  - mudar um vértice em dados muda a silhueta.
- **Roteiros do editor:** 61 passos no `--tour` e 3 no `--tour-reload`, todos com verificação.
- **Laboratório da 02** depois do refactor: `RESULT: PASS`, partição com 0 texels diferentes.
- **ASF `pixel_qa`** nos rasters pintados (6 poses de corrida por design).

## 15. Resultado dos testes
- `cargo fmt --check`: ok.
- `cargo clippy --all-targets -j 2 -- -D warnings`: limpo, depois de corrigir
  `manual_is_multiple_of`, `manual_contains`, `chunks_exact_to_as_chunks` e
  `iter_cloned_collect`.
- `cargo test -j 2`: **58 suítes, 762 passaram, 0 falharam**.
- `cargo check -j 2`: ok.
- `--tour`: `RESULT: PASS` (59 capturas no total). `--tour-reload`: `RESULT: PASS`.
- Corpo no editor: 0 texels diferentes nos 7 quadros. A composição sem equipamento continua
  idêntica à aprovada.
- `pixel_qa` (ASF) nos rasters pintados de A, B, espada e cinto: `partial_alpha` 0 e
  `same_size` aprovado. Paleta e contorno não foram conferidos pelo ASF, porque os designs
  misturam rampas e PNGs; o teste Rust confere a paleta por design.
- `git diff --check`: limpo.

## 16. Métricas e comparações
- **A × B:** sobreposição de silhueta 0,79 em todas as poses. Sobreposição das emendas internas
  (onde duas partes se encontram): 0,47, 0,44, 0,39, 0,40, 0,40, 0,36, 0,38 e 0,38.
- **Partes:** A tem 9 e B tem 8. Ao menos 5 partes visíveis por pose em ambos.
- **Cinto no idle de perfil:** cerca de 42 texels (faixa de 4 texels de altura).
- **ASF:** `render_pixelspec` qa `pass` (variantes ferro e latão), `svg_render --crisp` qa
  `pass` (0 alfa parcial).
- **Código novo:** cerca de 3650 linhas (modelo, pintor, editor e teste).

## 17. Problemas encontrados
- **Bisel invisível:** o anel do contorno comia o bisel.
- **Couro ruidoso:** o pontilhado formava um xadrez em toda a superfície.
- **Modelo A borrado:** uma região escura grande no peitoral.
- **Silhuetas de A e B parecidas (0,89):** as duas cobriam o tronco inteiro.
- **Teste de estrutura:** a sobreposição de silhueta sozinha não mede estrutura interna.
- **Roteiro:**
  - Enter alternava um item já equipado;
  - a lista de designs dava a volta nas pontas;
  - errei a contagem de M entre modos;
  - o ciclo de slots passou de `Belt` e trocou a espada.
- **Revisão marcada todo quadro:** os modos parte e encaixe chamavam `edit()` mesmo sem tecla,
  o que deixava tudo como "*unsaved*" e repintava sempre.
- **Conflito de nome:** `design::Visibility` sombreava `Visibility` do Bevy.
- **Limiar de texels:** era alto demais para o cinto no idle.
- **PixelSpec:** o ASF exige `layers` e `frames`.
- **Cobertura do peitoral:** na corrida de 3/4, a manga do braço de perto (mapa N4-C) cobre boa
  parte de um peitoral que fica dentro do tronco.

## 18. Problemas corrigidos
- Bisel medido de dentro do contorno, e pontilhado do couro só numa faixa estreita perto das
  trocas de tom. Especular no metal.
- Modelo A sem a região escura, com bisel 2,5 e menos rebites. Modelo B com barra assimétrica
  em ponta: a silhueta caiu de 0,89 para 0,79.
- O teste mede silhueta e emendas internas.
- Navegação do roteiro corrigida e `edit()` só com edição real.
- `PartVisibility` como alias.
- Limiar de 30 texels, com comentário.
- PixelSpec completada.

## 19. Problemas pendentes
- **Arte não aprovada:**
  - o Modelo A ainda tem faixas claras nas lamelas que parecem ruído;
  - o Modelo B tem placas de ferro claras e pouco integradas;
  - os dois cobrem pouco a frente do peito na corrida, porque a manga fica na frente (camada
    `NearArm`). Para aparecer mais, as peças precisariam avançar além do braço.
- **Partes sem hierarquia:** girar a lâmina não leva o rubi junto. Para mover tudo, usa-se o
  encaixe.
- **Sem desfazer** (Ctrl+R descarta tudo o que não foi salvo).
- **Sem seletor de arquivo e sem campo de texto:** ids e nomes novos são automáticos
  (`studio_item_N`).
- **No editor não é possível:** editar regiões e ornamentos (só no RON), criar materiais novos
  (só trocar e editar os existentes), escolher o espaço de um design novo (sempre tronco) ou
  editar a visibilidade.
- **Capa articulada não implementada** (Engine 04).

## 20. Limitações técnicas
- **Espaço `Torso`:** as coordenadas são u e v. Rotação e escala de uma parte acontecem nesse
  espaço normalizado, então não preservam ângulos em texels. Imagens no tronco são decalques sem
  rotação.
- **Variante de perspectiva:** é só por visibilidade (uma parte aparece numa faixa de giro).
  Ainda não existe interpolação de vértices entre vistas.
- **Pintura:** repinta o item inteiro quando muda design, clipe ou quadro. É rápido nesta
  escala, mas não há cache por parte.
- **Roteiro:** depende do estado da pasta. Com `studio_item_1.ron` já existente, o Ctrl+N cria
  `_2` e as verificações falham (documentado no README).
- **Mouse:** está implementado, mas não foi exercitado pelo roteiro automático (só o teclado
  foi).

## 21. Evidências visuais
- **01_editor_capture.png** e `editor/*.png`: capturas reais da janela do editor (painel,
  vista principal, prévias 2× e 1×, gizmos).
- **02_edit_part_visual.gif / _frames.png:** um vértice do peitoral do Modelo A movido passo a
  passo, depois tons do material editados. Tudo descartado no fim com Ctrl+R.
- **03_pivot_and_socket.gif / _frames.png:** o pivô da lâmina movido sem deslocar a lâmina,
  rotação em volta do pivô novo (o rubi fica, partes sem hierarquia), depois o encaixe inteiro
  movido e girado.
- **04_create_and_swap.gif:** cinto criado do zero pelo editor e salvo, depois trocas de
  equipamento durante a corrida.
- **04b_swap_during_run.gif:** só as trocas.
- **05_six_poses_*:** A e B nas 6 poses da corrida (capturas reais).
- **06_breastplates_compared_isolated.png:** A × B isolados no idle e na corrida em 6×.
- **07_equipment_isolated.png:** os 5 designs isolados (A, B, espada, cinto, peitoral 02 pelo
  adaptador).
- **07b_equipped_unequipped_isolated.png:** equipado, sem equipamento, isolado.
- **08_resolution_1x_2x_enlarged.png:** prévia 1×, prévia 2× e vista 4× do editor, mais
  ampliação de 8× por vizinho mais próximo.
- **09_save_and_reload.png** e `09_studio_item_1.ron`: salvo no processo 1 e reaberto no
  processo 2.
- **tools/:** fivela da PixelSpec e emblema do SVG renderizados pelo ASF (8×), com o relatório
  do ASF.

## 22. Caminhos das imagens, GIFs e vídeos
`evidence/modular_character_engine/03_equipment_studio/`:
- `01_editor_capture.png`
- `02_edit_part_visual.gif`, `02_edit_part_visual_frames.png`
- `03_pivot_and_socket.gif`, `03_pivot_and_socket_frames.png`
- `04_create_and_swap.gif`, `04b_swap_during_run.gif`
- `05_six_poses_a_b.png`, `05_six_poses_model_a.gif`, `05_six_poses_model_b.gif`
- `06_breastplates_compared_isolated.png`
- `07_equipment_isolated.png`, `07b_equipped_unequipped_isolated.png`
- `08_resolution_1x_2x_enlarged.png`
- `09_save_and_reload.png`, `09_studio_item_1.ron`
- `editor/{t00_start, edit_select_part, new_belt_saved, reload_selected, final_all}.png`
- `tools/{asf_render_pixelspec_buckle_x8.png, asf_svg_render_emblem_x8.png, asf_build_imports.json}`
- `metrics.json`

As capturas brutas (59 PNG e os logs dos roteiros), os rasters 1× (`preview/`) e o QA ficam em
`target/equipment_studio/`, que não é versionado.

## 23. Links do GitHub
Já publicados no push `63ffb91..4dcc5bf`:
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/docs/development/modular_character_engine/03_EQUIPMENT_STUDIO.md
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/modular_character_engine/03_equipment_studio/01_editor_capture.png
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/modular_character_engine/03_equipment_studio/04_create_and_swap.gif
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/modular_character_engine/03_equipment_studio/06_breastplates_compared_isolated.png
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/modular_character_engine/03_equipment_studio/05_six_poses_a_b.png
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/modular_character_engine/03_equipment_studio/03_pivot_and_socket.gif
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/modular_character_engine/03_equipment_studio/08_resolution_1x_2x_enlarged.png
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/art-src/equipment-studio/items/studio_item_1.ron

## 24. Resultado final
- **Ferramenta:** o Equipment Studio existe e funciona separado do jogo, e o motor interpreta
  qualquer equipamento descrito por partes.
- **Provas:**
  - dois peitorais estruturalmente diferentes (9 × 8 partes, silhuetas e emendas distintas);
  - uma espada importada de PNG;
  - um cinto criado e salvo só pelo editor, sem mudar Rust (commit `350e615`, só RON), e
    reaberto num segundo processo.
- **Comportamento:** os itens acompanham as 8 poses, trocam durante a corrida sem reiniciar a
  animação e continuam nítidos em 1×. O personagem sem equipamento continua idêntico ao
  aprovado.
- **Capa articulada:** ficou para a Engine 04.

## 25. Recomendações para a próxima etapa
1. **Avaliar:** abrir `cargo run --example equipment_studio` e olhar `05_six_poses_a_b.png`,
   `06_breastplates_compared_isolated.png` e `04_create_and_swap.gif`.
2. **Arte:** refinar A e B no próprio editor:
   - avançar a frente do peito além do braço;
   - reduzir as faixas claras das lamelas do A;
   - integrar melhor o ferro do B.
3. **Editor, em ordem de impacto:** desfazer e refazer, grupos e hierarquia de partes, edição
   de regiões e ornamentos, materiais novos e campo de texto para id e nome.
4. **Engine 04:** a capa (cadeia presa às costas) sobre o mesmo modelo.

## 26. Decisões que dependem do proprietário
1. Aprovar ou não a direção artística de A e B e o nível do sombreamento pintado.
2. Decidir se o peitoral deve avançar além do braço de perto na corrida, ou se o braço na
   frente está correto.
3. Escolher a prioridade da Engine 04: capa ou melhorias do editor (desfazer, hierarquia,
   edição de regiões).
4. Autorizar, mais adiante, a integração ao jogo. Por enquanto continua separado.

## 27. Estado do Git e publicação
- Branch: `art/v1.2-protagonist`
- Commit(s) da tarefa:
  - `9c9ece9` feat(player): add the generic equipment design model and painter
  - `e6c67b8` refactor(player): share the body layer cut with the rig lab
  - `6acaa61` feat(data): add the equipment studio designs and imports
  - `6377c2b` feat(player): add the equipment studio editor
  - `350e615` feat(data): add a belt made only with the equipment studio (**só
    `studio_item_1.ron`, nenhum arquivo Rust**)
  - `88b197f` test(player): validate the equipment studio designs
  - `509cc0c` docs(art): add the equipment studio evidence
  - `350887a` docs(architecture): document the equipment studio
  - `4dcc5bf` docs(claude): describe the equipment studio tests and modules
  - O commit deste relatório, com `INDICE.md` e `ULTIMO_RELATORIO.md`, vem depois. O hash dele
    é o HEAD publicado.
- Push: feito, sem force (`63ffb91..4dcc5bf`), seguido do push do commit do relatório.
- Local x remoto: sincronizado. `Md-usario/` não foi alterado nem commitado.
