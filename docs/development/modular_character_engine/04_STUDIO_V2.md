# Dark Castle Equipment Studio V2 — Modular Character Engine 04

Briefing: `Md-usario/engine/Engine-04.md` (do proprietário). Relatório:
[`docs/relatorios/modular-character-engine/04_STUDIO_V2.md`](../../relatorios/modular-character-engine/04_STUDIO_V2.md).
Etapa anterior: [`03_EQUIPMENT_STUDIO.md`](03_EQUIPMENT_STUDIO.md).

```bash
cargo run --example equipment_studio                    # o editor (pasta do proprietário)
cargo run --example equipment_studio -- --items DIR     # outra pasta de itens
cargo run --example equipment_studio -- --tour          # roteiro numa cópia isolada, depois sai
cargo run --example equipment_studio -- --tour-reload   # reabre a cópia num segundo processo
~/.local/share/dc-art-tools/asf-venv/bin/python -B art-src/equipment-studio/scripts/make_evidence_04.py
```

## 1. Hierarquia de partes (`rig::design`, puro)

Campos novos de `Part`, todos com `serde(default)` (os RON da 03 carregam sem mudança):

| Campo | Efeito |
|---|---|
| `parent: Option<String>` | pai pelo `id`; a transformação local fica no espaço do pai |
| `bind: Option<DesignSpace>` | só em raiz: a parte (e os filhos) vai para outro espaço (por exemplo, o braçal de trás no antebraço de trás) |
| `depth: Option<DepthSlot>` | profundidade própria; os filhos herdam |
| `hides: Vec<BodyRegion>` | regiões do corpo que a parte **substitui** (ver seção 3) |

Fontes novas: `PartSource::Group` (só agrupa) e `PartSource::Pixels` (grade de índices numa
paleta, pintada no editor).

- Transformações: `PartTransform::affine()` e `Affine` (`apply`, `then`, `inverse`).
  `world_affine(i) = pai ∘ … ∘ local`. O pintor usa `mount ∘ world_affine`.
- Visibilidade herdada: `effective_visible` (a parte e todos os ancestrais visíveis) e
  `Visibility::allows` de cada ancestral.
- `reparent(i, parent, keep_global)`: recusa pai inexistente, a própria parte, um
  descendente (ciclo) e `bind` em filho. Com `keep_global`, a transformação local é
  recalculada para a parte ficar no mesmo lugar (decomposição afim, o cisalhamento é
  descartado e documentado no teste).
- `duplicate_subtree`, `group(membros, nome)`, `remove_subtree`, `children`, `subtree`,
  `ancestors` (devolve `None` se houver laço num arquivo editado à mão), `validate` recusa
  pais ausentes e ciclos.

## 2. Regiões semânticas do corpo (`rig::layers`)

`BodyLayer` virou `BodyRegion`, com 8 regiões. `refine_regions` divide o corte da 02:
- braço de perto em **mão** (até 3,5 texels do punho), **antebraço** e **braço** (segmento
  mais próximo);
- faixa de 4 texels acima da linha da cintura do tronco vira **cintura**.

O corte continua exato: recomposição com 0 texels diferentes nos 7 quadros (teste e log do
roteiro).

Ordem de profundidade (`DepthSlot`, de trás para a frente):

```text
BackGear, MainHandBehind, FarArm, FarHandGear, Legs, LegGear, BootGear, Waist, Torso,
ChestGear, WaistGear, BeltGear, NearUpperArm, ShoulderGear, Hair, HelmetGear,
NearForearm, BracerGear, MainHandGear, NearHand
```

Consequências pedidas no briefing:
- o peitoral (`ChestGear`) fica **atrás** dos braços de perto;
- a ombreira (`ShoulderGear`) fica sobre o braço e **sob o cabelo**;
- o braçal (`BracerGear`) fica sobre o antebraço e sob a mão;
- a mão fica na frente de tudo (o punho nunca é coberto, testado nas 8 poses).

## 3. Composição final única (`rig::paint`)

- `paint_on_frame` pinta um design num quadro; cada parte usa o espaço do próprio `bind`
  (`space_map`).
- `split_by_depth` separa o raster por profundidade efetiva de cada parte.
- `hide_mask` marca os texels do corpo que uma parte substitui (a pegada dilatada de 1 texel,
  para não sobrar contorno do corpo na borda).
- `compose_frame(body, drop, width, items)` monta o quadro final: regiões do corpo (com a
  respiração do idle), menos o que foi substituído, mais cada parte na sua profundidade.

O editor mostra **essa mesma imagem** na vista principal (zoom 2× a 8×) e nas prévias 2× e
1×. As evidências 4× e 1× vêm dos mesmos bytes, gravados pelo roteiro. Só o zoom e as guias
mudam entre as vistas.

## 4. Desfazer (`rig::history`, puro)

`History<T>` guarda instantâneos (todos os designs, design e parte selecionados), limite de
200. Cada ação dos painéis grava um passo antes de mudar. Um arraste do mouse grava **um**
passo ao soltar, e só se algo mudou. Ctrl+Z, Ctrl+Y e Ctrl+Shift+Z.

Proteção contra perda:
- fechar a janela com alterações não salvas avisa e pede um segundo pedido
  (`close_when_requested: false` e `WindowCloseRequested`);
- Ctrl+R (voltar ao disco) também pede confirmação;
- designs alterados aparecem com `*` na biblioteca e o botão Save fica aceso.

## 5. Editor (`examples/equipment_studio.rs`)

Interface em Bevy UI. Os botões têm teste de clique próprio (`ComputedNode::contains_point`)
e chamam a mesma função `apply(Action)` do teclado.

| Área | Controles |
|---|---|
| Barra superior | New, Open (reler a pasta), Save, Save as, Import PNG, Undo (com o nome do passo), Redo, status |
| Esquerda | biblioteca por categoria (equipar com `on`), árvore de partes com recuo e botão `p` (tornar pai da parte selecionada), +part, +pixels, dup, group, remove, hide |
| Centro | personagem com os itens; grade de pixels; seleção de parte pelo mouse; esqueleto e gaiola; guias desligáveis de uma vez |
| Direita | espaço e profundidade do design; nome e tipo da parte; pai (próximo / nenhum / manter global); x, y, rotação, camada, escala, pivô; vínculo a osso; profundidade; material (próximo, **novo**, acabamento, rampa de 6 tons com R/G/B/brilho); contorno, bisel, curva; regiões (+, −, editar vértices da região); ornamentos (rebite, costura); visibilidade por clipe e por vista; regiões substituídas; modo; ferramentas de pixel e paleta |
| Inferior | idle / corrida, quadros 0–5, passo, play/pause, velocidade (0,25× a 2×), espelhar, zoom, grade, guias, esqueleto, isolado, sem equipamento |

Mouse no canvas:
- modo parte: clique seleciona a parte de cima naquele texel; arrastar move; botão direito gira; Ctrl+roda escala;
- modo vértice: arrasta o vértice mais próximo (do polígono ou da região escolhida);
- modo pivô: posiciona o pivô sem mexer na geometria;
- modo pintura: pincel, borracha, balde e conta-gotas na parte de pixels (`pixel_cell_at` usa o mesmo mapeamento do pintor).

As teclas da 03 continuam (lista no topo do arquivo).

## 6. Conjunto modular de prova (Modelo A refinado)

Quatro itens independentes, cada um com fonte, soquete, geometria, material e regras próprios:

| Item | Espaço (soquete) | Profundidade | Estrutura |
|---|---|---|---|
| `gothic_set_cuirass.ron` | gaiola do tronco | `ChestGear` | grupo → placa de trás, lateral, peito (→ quilha, emblema só na vista 3/4), gola; sai do tronco até u 1,32 (volume externo) |
| `gothic_set_pauldron.ron` | osso do braço de perto (início, segue 0,5) | `ShoulderGear` | grupo de 3 lâminas com barra dourada; substitui `NearUpperArm` sob a peça |
| `gothic_set_bracers.ron` | antebraço de perto | `BracerGear` | braçal de perto (placa + punho); a placa de trás tem `bind` no antebraço de trás e `FarHandGear` |
| `gothic_set_faulds.ron` | gaiola do tronco | `WaistGear` (slot `Belt`) | grupo de 3 lâminas com barras douradas |

Cada um equipa e desequipa sozinho, em idle e nas 6 poses da corrida, e combina com o Modelo B
(`leather_jerkin`). A troca não mexe no relógio da animação.

## 7. Roteiros isolados

`--tour` copia `art-src/equipment-studio/items/*.ron` para `target/equipment_studio/workspace/`
(apaga antes só os `.ron` dessa pasta) e edita a cópia. `--tour-reload` reabre a cópia num
segundo processo. A pasta do proprietário só é lida, e os roteiros repetem sem depender de
itens criados antes (a falha da 03 com `studio_item_N`).

O roteiro usa:
- cliques nos botões reais (posição tirada do nó de UI);
- teclas pelo `ButtonInput<KeyCode>`;
- um **ponteiro sintético**: cada quadro o roteiro define posição e botões, e o mesmo
  sistema `mouse` do editor trata o evento (o cursor do sistema operacional não é movido).

São 72 passos no `--tour` e 2 no `--tour-reload`, todos com verificação de estado, `RESULT: PASS` nos dois processos, e uma captura de
janela mais os bytes da composição por passo (`target/equipment_studio/v2/`).

## 8. Ferramentas externas

| Ferramenta | Uso nesta etapa |
|---|---|
| ASF `pixel_qa.py` | composições 1× das 6 poses da corrida: `partial_alpha` 0, mesmo tamanho, `loop_seam` 1,03 (limite 2,0; o loop não salta). Folha `tools/asf_pixel_qa_run_review.png` |
| ASF `render_pixelspec.py`, `svg_render.py --crisp` | as fivelas e o emblema da 03 continuam como importações (`import/`) |
| pixelfx `rig.js` (modelo, não executado nesta etapa) | hierarquia de transformações com pivô, normal + bisel e luz quantizada, tinta × emenda, profundidade por parte: reescritos em Rust no pintor |

Nenhuma chamada paga. ASF e pixelfx são MIT. Nenhuma linha deles foi copiada para o jogo.

## 9. Limitações conhecidas

- Não há campo de texto: números mudam por botões −/+ e teclas; nomes e ids são gerados
  (`item_N`, `material_N`, `<id>_copy`).
- A importação percorre `import/` (sem seletor de arquivo).
- Região nova nasce com tom −1 e sem material próprio; trocar material/tom de uma região
  ainda é edição no RON.
- A ordem cabelo × ombreira é fixa por profundidade, sem exceção por pose.
- Parte de pixels no tronco é um decalque sem rotação; na mão ou no osso ela gira.
- A árvore mostra até 20 linhas.
- Capa: fora do escopo desta etapa (restrição do briefing).

## Atualização (05)

A etapa 05 acrescentou equipamentos flexíveis (`rig::flex`, cadeias em dados, espaço da cadeia no pintor), o conjunto gótico refinado, capas, edição do pano e de material/tom de regiões no editor e a ferramenta `equipment_preview`. Ver [`05_ART_AND_FLEXIBLE_EQUIPMENT.md`](05_ART_AND_FLEXIBLE_EQUIPMENT.md).
