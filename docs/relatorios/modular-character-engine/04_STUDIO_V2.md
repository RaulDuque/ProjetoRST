# Modular Character Engine 04 — Equipment Studio V2: hierarquia, editor visual e encaixe real

**Status:** AGUARDANDO APROVAÇÃO VISUAL
**Aprovação artística:** pendente

## 1. Identificação
- Etapa: Modular Character Engine 04 (Equipment Studio V2)
- Frente: modular-character-engine
- Branch: `art/v1.2-protagonist`

## 2. Data e hora da execução
2026-11-21 19:45 (-03)

## 3. Objetivo original
- Hierarquia de partes: pai e filhos, grupos, transformações local e global.
- Interface gráfica funcional com painéis e desfazer/refazer.
- Edição pelo mouse, de materiais e de pixels.
- Encaixe real das armaduras: regiões semânticas do corpo, profundidades por peça, cabelo à
  frente, volume externo.
- O Modelo A refeito como peças independentes (peitoral, ombreiras, braçais, proteção da
  cintura), combinável com o Modelo B.
- Prévias 1×/2×/4×/8× iguais.
- Roteiros isolados que não alteram os itens do proprietário.
- Evidências em `evidence/modular_character_engine/04_studio_v2/`.

Depois disso, parar e esperar a aprovação visual.

## 4. Prompt executado
`Md-usario/engine/Engine-04.md` (do proprietário, não versionado por esta tarefa).

## 5. Contexto técnico inicial
- Engines 01–03 entregues e aguardando aprovação visual (último commit `f44c086`).
- A 03 tinha:
  - partes sem hierarquia;
  - corpo em 5 camadas grossas;
  - cada item num sprite próprio sobre o corpo, e o peitoral podia ficar na frente do braço;
  - sem desfazer;
  - um editor de texto e teclado.
- Os roteiros da 03 falhavam se `studio_item_1.ron` já existisse.

## 6. Ferramentas, plugins, skills e versões
- Rust 2024 e Bevy 0.20.1.
- Agent Sprite Forge 0.6.0 (`pixel_qa.py`) no venv isolado `~/.local/share/dc-art-tools/asf-venv`.
- Python 3 com Pillow (montagem das evidências).
- pixelfx (`rig.js`) só como modelo do pintor; nada dele rodou nesta etapa.
- Nenhuma API paga.

## 7. Arquivos consultados
- `Md-usario/engine/Engine-04.md`
- `docs/ARCHITECTURE.md`, `docs/DECISIONS.md`
- `docs/development/modular_character_engine/0{1,2,3}_*.md`
- `src/player/visual/rig/*`
- `examples/equipment_studio.rs` (versão 03), `examples/modular_character_engine.rs`
- `tests/equipment_studio_tests.rs`
- `art-src/equipment-studio/*`
- `evidence/modular_character_engine/03_equipment_studio/*`
- APIs do Bevy 0.20.1 em `~/.cargo/registry` (`ComputedNode::contains_point`,
  `despawn_children`, `WindowCloseRequested`)

## 8. Arquivos criados
- `src/player/visual/rig/history.rs`
- `art-src/equipment-studio/items/gothic_set_{cuirass,pauldron,bracers,faulds}.ron`
- `art-src/equipment-studio/scripts/make_evidence_04.py`
- `docs/development/modular_character_engine/04_STUDIO_V2.md`
- `evidence/modular_character_engine/04_studio_v2/`: 11 evidências, a folha
  `06_full_armor_idle_run_frames.png`, `metrics.json` e `tools/`
- este relatório

## 9. Arquivos modificados
- `src/player/visual/rig/{mod,design,layers,data,paint}.rs`
- `examples/equipment_studio.rs` (reescrito como V2), `examples/modular_character_engine.rs`
  (migração para as regiões)
- `tests/equipment_studio_tests.rs`
- `docs/ARCHITECTURE.md`, `docs/DECISIONS.md` (D-049),
  `docs/development/modular_character_engine/03_EQUIPMENT_STUDIO.md` (nota de atualização)
- `art-src/equipment-studio/README.md`
- `.claude/rules/{architecture,testing}.md`
- `docs/relatorios/INDICE.md`, `docs/relatorios/ULTIMO_RELATORIO.md`

## 10. Arquivos removidos
Nenhum. Os itens e as evidências das etapas anteriores continuam no lugar.

## 11. Descrição detalhada das alterações

### Hierarquia (`rig::design`)
- `Part` ganhou `parent`, `bind`, `depth` e `hides`, todos opcionais; os RON da 03 carregam sem
  mudança.
- Fontes novas: `Group` e `Pixels` (grade de índices numa paleta).
- Transformação afim composta (`world_affine`) e visibilidade herdada.
- Operações:
  - `reparent`: recusa ciclos, pai inexistente e `bind` em filho; opção de manter a posição
    global;
  - `duplicate_subtree`, `group`, `remove_subtree`.

### Regiões do corpo (`rig::layers`)
- 8 regiões: braço de trás, pernas, cintura, tronco, cabelo, braço, antebraço e mão de perto.
- Uma ordem única de `DepthSlot` para corpo e peças:
  - peitoral atrás dos braços;
  - ombreira sobre o braço e sob o cabelo;
  - braçal sobre o antebraço e sob a mão.

### Composição (`rig::paint`)
- `space_map` por parte (cada raiz pode ter seu próprio espaço).
- `split_by_depth`, `hide_mask` (substituição de regiões) e `pixel_cell_at` (o pincel).
- `compose_frame` gera a imagem final única: o editor mostra ela na vista principal, em 2× e
  em 1×, e as evidências usam os mesmos bytes.

### Desfazer (`rig::history`)
- Instantâneos, com transação para arrastes.

### Editor V2 (`examples/equipment_studio.rs`)
- Painéis em Bevy UI: barra superior, biblioteca/árvore, propriedades, linha do tempo.
- Todos os botões chamam a mesma `apply(Action)` do teclado.
- Mouse: seleção pelo canvas, mover, girar, vértices, pivô, pincel/borracha/balde/conta-gotas.
- Edição: material novo, acabamento, rampa RGB, regiões, ornamentos, visibilidade por clipe e
  por vista, espaço e vínculo a osso, profundidade, regiões substituídas, pai (com
  "manter global"), grupo, duplicar, remover, esconder.
- Undo/redo com Ctrl+Z, Ctrl+Y e Ctrl+Shift+Z.
- Proteção ao fechar e no Ctrl+R com alterações não salvas.
- Velocidade de reprodução e espelho.
- `--items DIR` abre outra pasta de itens.

### Roteiros
- `--tour` copia os itens para `target/equipment_studio/workspace/` e edita a cópia.
- `--tour-reload` reabre a cópia num segundo processo.

### Conjunto gótico modular
- 4 RON hierárquicos, cada um com soquete, geometria, material e regras próprios
  (tabela em `04_STUDIO_V2.md`, seção 6).

## 12. Decisões técnicas e artísticas
D-049 em `docs/DECISIONS.md`. Pontos principais:
- **Hierarquia dentro do `Part`**, compatível com a 03.
- **Regiões por geometria** sobre o corte existente. O corte continua exato: 0 texels
  diferentes nos 7 quadros.
- **Substituir em vez de só sobrepor:** é o que dá volume à ombreira e ao braçal.
- **Uma composição na CPU para todas as vistas**, o que garante prévias consistentes.
- **Teste de clique próprio nos botões**, para o roteiro clicar pelo mesmo caminho do mouse
  real, através de um ponteiro sintético.
- **Pintura de pixels no próprio editor**, sem etapa separada.
- **Commit do núcleo único:** `1569d74` junta núcleo, exemplos, testes e itens, porque a
  mudança de API do pintor precisava compilar junto com os dois exemplos.

## 13. Comandos relevantes executados
```bash
cargo build --example equipment_studio -j 2
cargo run --example equipment_studio -j 2 -- --tour          # 3 execuções, a última com tour.log
cargo run --example equipment_studio -j 2 -- --tour-reload
cargo run --example modular_character_engine -j 2
cargo fmt
cargo clippy --all-targets -j 2 -- -D warnings
cargo test -j 2
cargo check -j 2
~/.local/share/dc-art-tools/asf-venv/bin/python -B art-src/equipment-studio/scripts/make_evidence_04.py
git diff --check
```

## 14. Testes realizados
Testes unitários de `rig::`:
- `history`: undo/redo, limpar o refazer, arraste = 1 passo, limite;
- `design`: ida e volta afim, hierarquia e visibilidade, ciclos recusados, manter global,
  grupo/duplicar/remover, linhas de pixels;
- `layers`: ordem de desenho, refino das regiões;
- `paint`: filho gira com o pai e profundidades separadas, parte de pixels e máscaras,
  ordem da composição, célula do pincel = texel pintado.

`tests/equipment_studio_tests.rs` (10 testes), entre eles:
- `the_bare_composite_is_the_approved_art`;
- `the_gothic_set_and_a_mix_compose_every_pose`;
- `rotating_a_parent_moves_its_children_in_every_pose`.

Roteiros com janela:
- editor `--tour`: 72 passos verificados;
- `--tour-reload`: 2 passos;
- laboratório `modular_character_engine` (regressão da migração).

## 15. Resultado dos testes
- `cargo test -j 2`: **780 aprovados, 0 falhas**, em 58 binários (lib: 259).
- `cargo clippy --all-targets -- -D warnings`: limpo.
- `cargo fmt`, `cargo check`, `git diff --check`: ok.
- `equipment_studio --tour`: **RESULT: PASS**. `--tour-reload`: **RESULT: PASS**.
- `modular_character_engine`: **RESULT: PASS**.
- A pasta `art-src/equipment-studio/items/` do proprietário não foi alterada pelos roteiros
  (`git status` depois das execuções).

## 16. Métricas e comparações
- **Corte do corpo nas 8 regiões:** 0 texels diferentes da arte aprovada em idle e nas 6
  corridas (log do roteiro, `metrics.json`).
- **ASF `pixel_qa` nas composições 1× da corrida:**
  - `partial_alpha` 0;
  - mesmo tamanho (112×128);
  - `loop_seam` 1,029 (limite 2,0).

  Relatório em `evidence/modular_character_engine/04_studio_v2/tools/asf_pixel_qa_run_composites.json`.
- **Desfazer:** os passos de undo voltam cada item editado ao estado do disco (verificado pela
  comparação com o salvo):
  - 3 no passo 02;
  - 9 no 03;
  - 8 no 08, que desfaz arraste, rotação, 2 vértices, parte de pixels, 2 pinceladas e balde.

## 17. Problemas encontrados
- Na primeira execução do roteiro, um clique em "guides" a mais desligou as guias antes da
  hora, e 2 verificações falharam.
- A parte de pixels pintada no peito ficou quase toda escondida pelo braço de perto na
  corrida #2. É a regra de profundidade funcionando, mas a evidência ficou ilegível.
- A grade de pixels em alfa 0,06 lavava a vista principal.
- Duas regras do clippy no `paint.rs` (`needless_range_loop`, `manual_memcpy`).

## 18. Problemas corrigidos
- O clique a mais foi removido do roteiro.
- A pintura de pixels do roteiro acontece com o item isolado e volta à vista equipada no fim.
- A grade passou para alfa 0,03.
- O clippy foi corrigido com iteradores e `copy_from_slice`.
- **Herdado da 03:** os roteiros dependiam de `studio_item_N`. Agora rodam numa cópia isolada
  e repetem.

## 19. Problemas pendentes
Nenhum problema técnico aberto. A arte das peças está pendente de aprovação.

## 20. Limitações técnicas
- Não há campo de texto: números mudam por −/+ e teclas, e nomes são gerados.
- Não há seletor de arquivo: a importação percorre `import/`.
- Material e tom de uma região nova só mudam no RON.
- A ordem cabelo × ombreira é fixa, sem exceção por pose.
- Parte de pixels no tronco é um decalque sem rotação.
- A árvore mostra até 20 linhas.
- O mouse do roteiro é sintético: ele passa pelo mesmo sistema, mas o cursor do sistema
  operacional não se move.
- A ombreira isolada é pequena (cerca de 12×10 texels): é o tamanho real no personagem.
- Capa: fora do escopo (restrição do briefing).

## 21. Evidências visuais
| Arquivo | Mostra |
|---|---|
| `01_editor_interface.png` | janela inteira: barra superior, biblioteca, árvore do peitoral, canvas com grade, propriedades, linha do tempo, prévias 2× e 1× |
| `02_hierarchy_demo.gif` | o peito girado e arrastado leva quilha e emblema; undo; o rubi vira filho da lâmina (manter global) e gira com ela |
| `03_undo_redo.gif` | undo em sequência até o estado salvo; Ctrl+Y e Ctrl+Shift+Z |
| `04_material_editor.gif` | material novo na ombreira, rampa editada, acabamento, região, rebite e costura, aviso de alterações não salvas no Ctrl+R |
| `05_armor_components.png` | corpo nu (arte aprovada), cada peça isolada e o conjunto vestido (corrida #2, 4×) |
| `06_full_armor_idle_run.gif` | idle 0–1 e corrida 0–5, composição 4× e 1× sem guias (também como folha `06_full_armor_idle_run_frames.png`) |
| `07_equipment_mix.gif` | animação tocando, gibão de couro + peças góticas, tira e põe ombreira, braçais e faldas sem reiniciar a animação |
| `08_mouse_editing.gif` | seleção pelo clique no canvas, arrastar, girar com o botão direito, vértices arrastados, parte de pixels com pincel e balde, undo de tudo |
| `09_save_reload.png` | salvar, salvar como, item novo feito só pelos painéis, reaberto por outro processo |
| `10_before_after_engine03_engine04.png` | editor 03 × 04 e corrida #2 da 03 × da 04 |
| `11_native_resolution.png` | 8 poses em 1× real e o corpo nu em 1× |

## 22. Caminhos das imagens, GIFs e vídeos
- `evidence/modular_character_engine/04_studio_v2/01_editor_interface.png`
- `evidence/modular_character_engine/04_studio_v2/02_hierarchy_demo.gif`
- `evidence/modular_character_engine/04_studio_v2/03_undo_redo.gif`
- `evidence/modular_character_engine/04_studio_v2/04_material_editor.gif`
- `evidence/modular_character_engine/04_studio_v2/05_armor_components.png`
- `evidence/modular_character_engine/04_studio_v2/06_full_armor_idle_run.gif`
- `evidence/modular_character_engine/04_studio_v2/06_full_armor_idle_run_frames.png`
- `evidence/modular_character_engine/04_studio_v2/07_equipment_mix.gif`
- `evidence/modular_character_engine/04_studio_v2/08_mouse_editing.gif`
- `evidence/modular_character_engine/04_studio_v2/09_save_reload.png`
- `evidence/modular_character_engine/04_studio_v2/10_before_after_engine03_engine04.png`
- `evidence/modular_character_engine/04_studio_v2/11_native_resolution.png`
- `evidence/modular_character_engine/04_studio_v2/tools/asf_pixel_qa_run_review.png`

## 23. Links do GitHub
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/docs/development/modular_character_engine/04_STUDIO_V2.md
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/examples/equipment_studio.rs
- https://github.com/RaulDuque/ProjetoRST/tree/art/v1.2-protagonist/evidence/modular_character_engine/04_studio_v2

## 24. Resultado final
A parte técnica está entregue e validada:
- hierarquia;
- editor gráfico funcional com desfazer, mouse, materiais e pixels;
- regiões e profundidades do encaixe;
- conjunto gótico em 4 peças independentes, combinável com o Modelo B;
- prévias da mesma composição;
- roteiros isolados repetíveis.

A arte das peças ainda não foi aprovada.

Editor: `cargo run --example equipment_studio`.

## 25. Recomendações para a próxima etapa
- Depois da aprovação: refinar a arte das peças no próprio editor.
- Possíveis acréscimos ao editor:
  - campo de texto para nomes e números;
  - edição do material e do tom de uma região;
  - exceções de profundidade por pose (cabelo × ombreira).
- Avaliar a capa com a cadeia prevista na 03.

## 26. Decisões que dependem do proprietário
- Aprovar ou reprovar a arte do conjunto gótico modular e o encaixe (peitoral atrás do braço,
  ombreira sob o cabelo, braçais nos antebraços).
- Dizer se a ordem fixa cabelo × ombreira atende ou se precisa de exceção por pose.
- Escolher a próxima etapa: refinamento artístico, capa ou integração ao jogo.

## 27. Estado do Git e publicação
- Branch: `art/v1.2-protagonist`
- Commits da tarefa:
  - `42b3af8` (histórico);
  - `1569d74` (hierarquia, regiões, composição, editor V2, testes, conjunto gótico);
  - `701bfcc` (evidências);
  - `ba5f049` (arquitetura);
  - `b030beb` (regras);
  - o commit deste relatório.
- Push: feito, sem force.
- Local × remoto: sincronizado depois do push do relatório.
