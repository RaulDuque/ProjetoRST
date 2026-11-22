# Modular Character Engine 05 — produção artística, equipamentos flexíveis e conjunto de referência

**Status:** AGUARDANDO APROVAÇÃO VISUAL
**Aprovação artística:** pendente

## 1. Identificação
- Etapa: Modular Character Engine 05
- Frente: modular-character-engine
- Branch: `art/v1.2-protagonist`

## 2. Data e hora da execução
2026-11-21 (-03), concluída por volta das 23h. Hora exata não registrada.

## 3. Objetivo original
Usar a infraestrutura das Engines 01–04 para produzir equipamentos de melhor qualidade:
- **Conjunto gótico refinado** (peitoral, ombreiras, braçais, proteção da cintura) como itens
  novos, preservando os da 04, com rampas de metal roxo, dourado e couro refinadas.
- **Primeira capa flexível**, reutilizável por dados, com comportamento em:
  - parado e respiração;
  - corrida, partida e parada;
  - virada e espelho.
- **Segunda capa** feita só com dados.
- **Provas no Equipment Studio** e melhorias pontuais nele.
- **15 evidências** obrigatórias.

Sem reescrever a interface e sem produzir equipamentos em massa. Ao final, parar para
aprovação visual.

## 4. Prompt executado
`Md-usario/engine/Engine-05.md` (do proprietário, não versionado por esta tarefa).

## 5. Contexto técnico inicial
- Engine 04 aprovada tecnicamente; a arte do conjunto gótico não foi aprovada. Relatório
  `04_STUDIO_V2.md`, commit `4abf570`.
- Não havia equipamento flexível: só a arquitetura de capa prevista na 03.

## 6. Ferramentas, plugins, skills e versões
- Rust 2024 e Bevy 0.20.1.
- Agent Sprite Forge 0.6.0, no venv `~/.local/share/dc-art-tools/asf-venv`: `pixel_qa.py` e
  `rig_animate.spring`.
- pixelfx 0.3.1: consultado; não tem módulo de pano.
- Python 3 com Pillow.
- Blender: não usado.
- Nenhuma chamada paga, nenhuma instalação.

## 7. Arquivos consultados
- `Md-usario/engine/Engine-05.md`
- `docs/relatorios/ULTIMO_RELATORIO.md`, `docs/relatorios/modular-character-engine/04_STUDIO_V2.md`
- `docs/development/modular_character_engine/0{3,4}_*.md`
- `src/player/visual/rig/*`, `examples/equipment_studio.rs`, `tests/equipment_studio_tests.rs`
- `art-src/equipment-studio/items/*`, `art-src/modular-character-engine/stage/rig_lab.ron`
- `src/core/constants.rs` (velocidade do jogador)
- código do pixelfx (`src/core/*.js`) e do ASF (`rig_animate.py`, `pixel_qa.py`)

## 8. Arquivos criados
- `src/player/visual/rig/flex.rs`
- `examples/equipment_preview.rs`
- `art-src/equipment-studio/items/gothic_v2_{cuirass,pauldron,bracers,faulds}.ron`
- `art-src/equipment-studio/items/gothic_cape.ron`, `gothic_cape_short.ron`, `leather_v2_jerkin.ron`
- `art-src/equipment-studio/scripts/{preview_sheet,run_tools_05,make_evidence_05}.py`
- `docs/development/modular_character_engine/05_ART_AND_FLEXIBLE_EQUIPMENT.md`
- `evidence/modular_character_engine/05_art_and_flexible_equipment/`:
  - as 15 evidências;
  - `12b_swaps_during_the_run.gif`, `15b_editor_cloth_parameter.png`;
  - `metrics.json` e `tools/`.
- este relatório

## 9. Arquivos modificados
- `src/player/visual/rig/{mod,design,paint,data}.rs`
- `art-src/modular-character-engine/stage/rig_lab.ron` (só o campo `ground_speed` da corrida)
- `examples/equipment_studio.rs`, `tests/equipment_studio_tests.rs`
- `docs/ARCHITECTURE.md`, `docs/DECISIONS.md` (D-050), `04_STUDIO_V2.md` (nota),
  `art-src/equipment-studio/README.md`
- `.claude/rules/{architecture,testing}.md`
- `docs/relatorios/INDICE.md`, `ULTIMO_RELATORIO.md`

## 10. Arquivos removidos
Nenhum. Os itens e as evidências das Engines 01–04 continuam no lugar.

## 11. Descrição detalhada das alterações

**Arte (itens novos, só dados):**
- Peitoral anatômico: volume no peito e na lateral, borda em V sobre a placa do abdome que
  afina na cintura, quilha, gola escura.
- Ombreira em cúpula mais larga que o braço, com duas lâminas em crescente.
- Braçais com cotoveleira, placa afunilada e punho dourado que para antes da mão.
- Proteção da cintura: faldas curtas de borda curva e duas escarcelas presas às coxas.
- Gibão de couro com rampas refinadas.
- Metal mais escuro e menos saturado, dourado só em frisos contínuos (linha de ornamento com
  `material`) e menos linhas horizontais.

**Pano (`rig::flex`):**
- `FlexChain` em dados.
- Simulação por ângulos (pêndulo amortecido por segmento, acoplado, com ar da corrida,
  abrigo, ondulação forçada, inércia da aceleração do corpo e dos saltos da fixação, limites,
  dobra máxima).
- `turn_around`, `from_points`, `settle` periódico.

**Pintor:**
- `SpaceMap::Chain` (projeção na curva suavizada);
- `paint_on_frame_with` (cadeias ao vivo), `settled_chains`, `flex_input`;
- `pixel_cell_at` passou a receber o mapa por referência.

**Dados do clipe:** `RigClip.ground_speed`.

**Editor:**
- pano ao vivo por item vestido; velocidade do corpo suavizada nas partidas e paradas;
  espelhar = virada;
- passo fixo nos roteiros;
- painel "CLOTH CHAIN" (13 parâmetros, com desfazer) e cadeia nas guias;
- material e tom da região em edição; vínculo de parte às cadeias;
- painéis mais compactos;
- operação de gravação quadro a quadro no roteiro e 26 passos novos.

**Ferramenta sem janela:** `equipment_preview`, para iterar a arte e montar as evidências, e
`--trace` para o movimento da capa.

## 12. Decisões técnicas e artísticas
D-050 em `docs/DECISIONS.md`. Pontos principais:
- **Ângulos em vez de partículas:** a versão Verlet com follow-the-leader balançava
  caoticamente a cada salto de pose (medido: a barra ia de 0° a 92° no mesmo loop).
- **Capa em `BackGear`:** atrás do corpo, do cabelo e da espada, sem interseção grosseira. O
  fecho na garganta fica em `ChestGear`.
- **Profundidade sem exceções:** a ordem cabelo × ombreira foi mantida.
- **Arte nova em arquivos novos:** a 04 fica preservada.
- **Linha de ornamento com material:** os frisos dourados finos feitos com regiões se partiam
  em traços em 1×.
- **Rigidez das capas:** subiu para a resposta ficar em ~0,5 s. Com rigidez 0, a capa levava
  mais de 4 s para assentar.
- **Pano novo:** começa na forma assentada do quadro, para não aparecer pendurado numa pose de
  corrida.

## 13. Comandos relevantes executados
```bash
cargo build --example equipment_preview -j 2
cargo run --example equipment_preview -- NOME ITEM...   # dezenas de iterações de arte
cargo run --example equipment_preview -- --trace gothic_cape
cargo run --example equipment_studio -j 2 -- --tour
cargo run --example equipment_studio -j 2 -- --tour-reload
cargo run --example modular_character_engine -j 2
cargo fmt
cargo clippy --all-targets -j 2 -- -D warnings
cargo test -j 2
cargo check -j 2
PATH=~/.local/share/dc-art-tools/asf-venv/bin:$PATH python -B art-src/equipment-studio/scripts/run_tools_05.py
python3 -B art-src/equipment-studio/scripts/make_evidence_05.py
git diff --check
```

## 14. Testes realizados
- **Unitários de `rig::flex`:**
  - comprimento e limites;
  - parado pendurado e a corrida levantando atrás;
  - parada com atraso e queda;
  - virada e retorno;
  - `settle` periódico e determinístico;
  - mapa da cadeia em ida e volta;
  - forma conhecida em ida e volta;
  - dados inválidos recusados.
- **Integração (`tests/equipment_studio_tests.rs`):**
  - `the_refined_set_composes_every_pose_and_keeps_the_hands`;
  - `both_capes_hang_behind_the_body_in_every_pose` (nenhum texel do corpo coberto);
  - `the_run_lifts_the_cape_and_the_idle_lets_it_hang`;
  - `the_second_cape_is_only_data` (inclui ida e volta em RON);
  - os testes anteriores, que agora carregam e pintam também os itens novos.
- **Roteiros:**
  - editor `--tour` (98 passos, 26 da Engine 05) e `--tour-reload`;
  - laboratório `modular_character_engine`.
- **ASF:** `pixel_qa` (paleta, alfa, emenda do loop) e `rig_animate.spring` (movimento da
  capa).

## 15. Resultado dos testes
- `cargo test -j 2`: **792 aprovados, 0 falhas**.
- `cargo clippy --all-targets -- -D warnings`: limpo.
- `cargo fmt --check`, `cargo check`, `git diff --check`: ok.
- `equipment_studio --tour`: **RESULT: PASS**. `--tour-reload`: **RESULT: PASS**. Os itens do
  proprietário não foram alterados (`git status`).
- `modular_character_engine`: **RESULT: PASS**.

## 16. Métricas e comparações
- **Corpo sem equipamento:** idêntico à arte aprovada (teste, e 0 texels diferentes no corte,
  conforme o log do roteiro).
- **ASF `pixel_qa`:**
  - nos 7 itens novos isolados (8 poses): `off_palette` 0 e `partial_alpha` 0;
  - na corrida com o conjunto, a capa e a espada: `loop_seam` 1,041; só a capa: 1,017
    (limite 2,0).
- **Movimento da capa** (`--trace`, 60 Hz):
  - partida: sobe de 4° a ~67° em ~0,5 s e fica entre 47° e 54° na corrida;
  - parada: desce em ~0,4 s, com 19% de ultrapassagem para a frente, e assenta em 0,65 s;
  - comparado com a mola amortecida do ASF, o melhor ajuste é k 106, d 16,5, RMS 0,086
    (subamortecida).
- **Pose assentada:** a barra fica em 39–46° na corrida e abaixo de 15° no idle (teste).

## 17. Problemas encontrados
- Capa rígida como uma tábua na primeira versão: arrasto uniforme dá uma linha reta.
- Partículas instáveis nos saltos de pose.
- Ondulação filtrada pelo pêndulo lento.
- Resposta lenta da capa (mais de 4 s).
- Frisos dourados partidos em traços, e frisos engolidos pelo contorno da borda.
- Excesso de faixas horizontais (ombreira e faldas).
- Painel esquerdo transbordando.
- `pixel_qa` não sobrescreve relatórios, então a segunda execução leu dados velhos.
- Capa equipada em pausa aparecia pendurada numa pose de corrida.
- Clippy `large_enum_variant` no `SpaceMap`.

## 18. Problemas corrigidos
- Abrigo (`shelter`) e ondulação de través na capa; troca para a simulação por ângulos;
  ondulação como força.
- Rigidez e amortecimento ajustados nos dados.
- Linha de ornamento com material, posta dentro da borda; frisos reduzidos aos essenciais.
- Espaçamento dos painéis mais compacto.
- O script apaga os próprios relatórios antes de rodar o `pixel_qa`.
- `FlexState::from_points`: o pano novo começa assentado.
- `Arc<ChainMap>` no `SpaceMap` e `pixel_cell_at(&SpaceMap)`.

## 19. Problemas pendentes
Nenhum problema técnico aberto. A arte espera aprovação.

## 20. Limitações técnicas
- O pintor de polígonos dá um acabamento mais liso que o pixel art feito à mão. A melhora em
  relação à 04 é perceptível, mas o julgamento é do proprietário.
- A capa fica sempre atrás do corpo. Uma capa envolvente pediria regra de profundidade por
  pose.
- A velocidade da corrida (110 texels/s) é uma estimativa.
- Partida e parada usam as poses idle e de corrida, sem clipe de transição.
- A ondulação é forçada, não aerodinâmica.
- As escarcelas das duas coxas usam a mesma profundidade.
- O pixelfx não tem módulo de pano, e o Blender não foi necessário.

## 21. Evidências visuais
| Arquivo | Mostra |
|---|---|
| `01_armor_before_after.png` | conjunto da 04 × refinado, idle e corrida 2 e 4, em 4×, sem espada, e as 8 poses em 1× |
| `02_equipment_components.png` | cada peça isolada e vestida sozinha (3×), e a capa |
| `03_armor_idle.png` | idle 0–1 (respiração), sem espada e com capa e espada, em 4×, 2× e 1× |
| `04_armor_run_frames.png` | as 6 poses da corrida sem espada e com capa e espada (3×) e em 1× |
| `05_armor_run.gif` | corrida do conjunto refinado, 4× e 1× |
| `06_cape_isolated.png` | capa sozinha e vestida nas 8 poses |
| `07_cape_idle.gif` | capa no idle com respiração (editor, ao vivo) |
| `08_cape_run.gif` | capa na corrida (editor, ao vivo) |
| `09_cape_start_stop.gif` | idle → corrida → idle sem cortes (editor) |
| `10_cape_turnaround.gif` | duas viradas durante a corrida (editor) |
| `11_complete_equipment_set.gif` | conjunto completo com capa e espada, idle e corrida |
| `12_equipment_combinations.png` | roupa padrão, cada peça sozinha, capa, conjunto com espada, couro + peças góticas + capa, capa curta |
| `12b_swaps_during_the_run.gif` | ombreira, braçais e capa trocados durante a corrida, sem reiniciar a animação |
| `13_cape_variants.png` | as duas capas (a segunda só com dados) |
| `14_native_resolution.png` | 1× real: corpo nu, conjunto, conjunto com capa e espada, couro misto; 2× |
| `15_editor_capture.png` | editor com a capa selecionada, cadeia nas guias e painel do pano |
| `15b_editor_cloth_parameter.png` | um parâmetro do pano editado no painel |

## 22. Caminhos das imagens, GIFs e vídeos
Todos em `evidence/modular_character_engine/05_art_and_flexible_equipment/`:
- `01_armor_before_after.png`
- `02_equipment_components.png`
- `03_armor_idle.png`
- `04_armor_run_frames.png`
- `05_armor_run.gif`
- `06_cape_isolated.png`
- `07_cape_idle.gif`
- `08_cape_run.gif`
- `09_cape_start_stop.gif`
- `10_cape_turnaround.gif`
- `11_complete_equipment_set.gif`
- `12_equipment_combinations.png`
- `12b_swaps_during_the_run.gif`
- `13_cape_variants.png`
- `14_native_resolution.png`
- `15_editor_capture.png`
- `15b_editor_cloth_parameter.png`
- `tools/qa_full_run_review.png`, `tools/tools05.json` (ASF)

## 23. Links do GitHub
- https://github.com/RaulDuque/ProjetoRST/tree/art/v1.2-protagonist/evidence/modular_character_engine/05_art_and_flexible_equipment
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/docs/development/modular_character_engine/05_ART_AND_FLEXIBLE_EQUIPMENT.md
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/src/player/visual/rig/flex.rs

## 24. Resultado final
**Técnico (validado):**
- equipamentos independentes;
- capa flexível funcionando em idle, respiração, corrida, partida, parada, virada e espelho;
- segunda capa sem mudar código;
- trocas sem reiniciar a animação;
- salvar e carregar;
- nenhuma regressão (792 testes, roteiros com PASS);
- compatível com o Equipment Studio;
- arte original intacta.

**Artístico:** pendente. Melhoria em relação à 04 nas silhuetas, materiais e frisos; capa com
atraso e ondulação. O julgamento é do proprietário.

Editor: `cargo run --example equipment_studio`.

## 25. Recomendações para a próxima etapa
- Se a arte for aprovada em parte: refinar as peças apontadas pelo proprietário no editor,
  usando `equipment_preview` para iterar.
- Para mais qualidade: partes de pixels pintadas à mão (já suportadas) para os detalhes
  principais (gola, crista, punho).
- Capa envolvente: regra de profundidade por pose orientada por dados.
- Clipe de transição para partida e parada.

## 26. Decisões que dependem do proprietário
- Aprovar ou reprovar a arte do conjunto refinado, do couro e das duas capas.
- Dizer se a capa sempre atrás do corpo atende ou se precisa envolver o corpo em algumas poses.
- Escolher a próxima etapa: refinamento artístico, integração ao jogo ou outra.

## 27. Estado do Git e publicação
- Branch: `art/v1.2-protagonist`
- Commits da tarefa:
  - `580af01`: cadeias flexíveis no rig e no editor;
  - `f61db86`: itens;
  - `51f5351`: testes;
  - `f38b403`: ferramenta de prévia;
  - `5b0de00`: evidências e ferramentas;
  - `83de99c`: arquitetura;
  - `dda0697`: regras;
  - o commit deste relatório.
- Push: feito, sem force.
- Local × remoto: sincronizado depois do push do relatório.
