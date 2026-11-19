# N41-B — Fábrica de equipamentos orientada por dados: arquitetura e contratos

## Auditoria (estado inicial)

- Branch `art/v1.2-protagonist`, local = remoto = `4cb0ac644bf29593b19a2c89e22b871b8c0d0cbf` (registro da N4-1-A). Árvore limpa, exceto arquivos pessoais não rastreados em `Md-usario/` (preservados, não tocados).
- Confirmado **no código**: `VisualFamily`/`EquipmentVisual`/`equip` (`src/player/visual/equipment_visual.rs`), HIDE/REPLACE/folha espelhada e `resolve_facing` (`composition.rs`), um `VisualPlayback` por personagem e `sync_composite_layers` (`composite.rs`), o modo `--equipment` de `examples/n4_sprite_machine.rs` e o pintor procedural `build_equipment.py` (padrões novos = código Python, a limitação registrada na N4-1-A).
- Regiões com máscara e perfil válidos: **tronco** (família `torso_plate`, idle e C1 #0/#2/#4). Sem camada nem soquete: **ombro**, **costas/capa**. Soquetes da mão: **candidatos sem confirmação** (`N4C_SOQUETES.md`), sem ângulo nem frente medidos.
- `tests/n41a_equipment_tests.rs` exige exatamente os itens A e B em `stage/equipment/items/`. Por isso a fábrica escreve em `stage/factory/` e **reutiliza** a família `torso_plate` (não cria família nova).
- Hashes protegidos (teste `the_save_format_and_the_approved_art_do_not_move`): `run6.png` = `0x0798ef4f59448a15`, `candidate_106.png` = `0x63a38af5ff4fd252` (FNV-1a 64). Continuam iguais (o teste passa).

## Os cinco subsistemas

| Subsistema | Onde | O que é |
|---|---|---|
| A. Templates | `art-src/n4-sprite-machine/factory/templates/*.json` | forma + operações de relevo + slots de material (+ `known_issues` da revisão visual) |
| B. Materiais | `factory/materials/*.json` | rampa de tons (`outline`, `deep`, `shadow`, `base`, `light`, `highlight`, `specular` opcional) + `gloss`, `roughness`, `contrast`, `wear` |
| C. Detalhes | `factory/details/*.json` | ornamentos por **operações genéricas**: `stamp`, `edge_band`, `line`, `scatter` |
| D. Gerador | `scripts/factory.py` | valida, forma, relevo, luz, quantização, contorno seletivo, detalhes, PNG/TXT, proveniência, cache |
| E. Integrador | `stage/factory/items/*.ron` | manifestos `EquipmentVisual` da família `torso_plate`, lidos pelo loader existente |

Perfis de animação: `factory/profiles/*.json` (`humanoid_torso`, `weapon_main_hand`, `humanoid_shoulder`, `cape_back`). Receitas: `factory/recipes/*.json`.

**Não há segundo renderizador, inventário nem relógio.** O jogo não gera nada em tempo de execução: os itens de tronco são peças comuns da composição, desenhadas pelo mesmo `sync_composite_layers` com o mesmo `VisualPlayback`. Nenhum arquivo em `src/` mudou na N41-B.

## Fluxo

```
templates/ materials/ details/ profiles/ ─┐
recipes/<id>.json ─────────────────────────┼─ factory.py ─┬─ stage/factory/art/<id>/<clipe>.png|txt   (tronco: folhas no canvas/pivô do corpo)
máscaras da família torso_plate ───────────┤  (validação) ├─ stage/factory/items/<id>.ron             (tronco: manifesto do jogo)
camadas vestidas (stage/parts/) ───────────┘              ├─ stage/factory/preview/<id>.png|txt      (tronco: prévia na pose C1 #2)
                                                          ├─ stage/factory/isolated/<id>.png|txt     (espada, capa, ombreira)
                                                          ├─ stage/factory/relief/<template>_<clipe>.png (mapa de altura)
                                                          └─ stage/factory/provenance/<id>.json      (hashes, seed, versão, cache)
stage/factory/items/*.ron ── equip() ── Outfit::wear ── resolve_facing ── sync_composite_layers (um VisualPlayback)
```

## Formatos

RON para o que o jogo lê; JSON para receitas, bibliotecas e relatórios; PNG RGBA para sprites, máscaras e mapas de altura. SVG não foi usado: as formas são polígonos inteiros e grades de texel em JSON, rasterizadas pelo centro do texel, sem suavização. Base64 não foi usado (sem caso de transporte). HTML/JS não são dependências.

## Contrato de receita (`schema: 1`)

| Campo | Regra (validada; erro descritivo, nada aceito em silêncio) |
|---|---|
| `id` | `lower_snake_case`, igual ao nome do arquivo, único |
| `category` | `chest_armor`, `sword`, `cape`, `shoulder` (define o tipo de template exigido) |
| `family` | documentação do agrupamento (tronco: `torso_plate`, a família real do jogo) |
| `template` / `templates` | tronco/capa/ombro: um template do tipo certo; espada: `{blade, hilt}` |
| `materials` | preenche **exatamente** os `material_slots` do(s) template(s), com materiais existentes |
| `details[]` | `{detail, material, place}`; as chaves de `place` dependem da operação do detalhe |
| `generation.seed` | inteiro ≥ 0, explícito; `generation.animation_profile` = o perfil do template |
| `covers` | regiões (documentação) |
| `hides` | `auto` (a fábrica liga o HIDE da família só se o item cobrir 100% da camada em todos os quadros) ou `none` |
| `mirroring` | `flip` (o corpo espelha o item; folha espelhada própria não é gerada nesta etapa) |
| `fallback.missing_clip` | `hide` ou `refuse` |
| `variants` | `{}` (variantes não são geradas nesta etapa: recusa se declarar) |

Chaves desconhecidas são erro. Erros de uma receita não param o lote: vão para `failures` e o comando sai com código 1.

## Templates

| Template | Tipo | Forma | Relevo |
|---|---|---|---|
| `segmented_cuirass` | tronco | extensão total, gorjal | cilindro, cúpula do peito, lâminas (`bands`), aro superior |
| `ridged_breastplate` | tronco | curto (0,74), ponta em V, decote | cilindro, quilha central, aro em toda a borda |
| `brigandine_coat` | tronco | inteiro + saia segmentada sobre o quadril (só onde o topo é a calça) | cilindro, grade de placas, segmentos da saia |
| `scale_hauberk` | tronco | inteiro, gorjal, barra em escamas, saia curta | cilindro, escamas sobrepostas |
| `arming_blade`, `estoc_blade`, `broad_falchion`, `flame_blade` | lâmina | perfil de meia-largura, curvatura, ondulação | bisel/quilha, sulco (fuller), dorso de um gume |
| `cross_hilt`, `crescent_hilt`, `ring_hilt` | guarda/cabo/pomo | grades de texel com tons simbólicos | (pintadas pelos tons do material) |
| `long_cape` | capa (flexível) | polígono + barra rasgada + forro na borda da frente | dobras, caimento |
| `plate_pauldron` | ombreira (rígida) | polígono arredondado | cilindro, cúpula, lâminas curvas, aro |
| `fur_mantle` | acessório de ombro | polígono + gola de pelagem | cilindro, dobras, textura de pelo |

As silhuetas do tronco **vêm das máscaras da família** (camisa vestida, polígono do tronco de cada pose): os quatro modelos mudam extensão, barra, gola, saia e relevo, não desenham anatomia.

## Mapas de relevo

Cada template gera um **mapa de altura** (0..255, 0 = transparente) por clipe em `stage/factory/relief/`. **Não é um mapa de normais**: a normal é estimada pelo gradiente da altura entre vizinhos do mesmo material; a luz vem de cima e da frente (`LIGHT = (0.55, -0.65, 0.55)`, o personagem olha para +x). Intensidade = Lambert + oclusão pela altura + contraste do material + pontilhado Bayer 4×4 pela rugosidade; quantizada na rampa do material; especular só onde a normal quase coincide com a luz e o material tem `specular`. O relevo não inventa volume físico: só o que as operações descrevem.

## Contorno seletivo (sel-outline)

Borda externa = `outline` do material (azul-aço no ferro, marrom no couro, vinho no tecido vermelho, marrom-dourado no ouro); borda iluminada (frente ou topo, um lado só) = o tom mais escuro da rampa; borda cortada por um oclusor (braço, manga) = sombra projetada; divisa entre dois materiais = tom mais escuro do de baixo. Nenhum contorno é preto puro (o validador recusa `outline: [0,0,0]`). Os contornos pretos da arte aprovada não são tocados (a fábrica só desenha no slot do item).

## Detalhes

| Operação | Uso | Posicionamento |
|---|---|---|
| `stamp` | rebites, fivelas, emblemas, gemas, runas | `at: [u, v]` ou `along: seams | grid | column | row` (`u`, `v`, `step`), `part` |
| `edge_band` | frisos, bordados | `edge: top | bottom` |
| `line` | costuras, juntas | `axis: vertical | horizontal`, `u`/`v` |
| `scatter` | desgaste | `density` (semeado pela seed da receita) |

Tons dos carimbos: `o d s b l h p` (outline, deep, shadow, base, light, highlight, specular) do material do detalhe; em maiúscula, do material hospedeiro (sombra do rebite sobre a placa). Detalhes só tocam **texels internos** da sua parte (nunca o contorno, nunca fora da máscara). Um carimbo que não cabe é pulado e registrado (`on_overflow: skip`) ou vira erro (`error`).

## Perfis de animação e tabela por quadro

`profiles/humanoid_torso.json`: para cada clipe/quadro, offset (0,0), pivô, camada de desenho, z do idle (12), regiões ocultadas, oclusores, confiança da medição. `weapon_main_hand.json`: soquete da mão por quadro com o **candidato** da N4C e `status: pendente` (ângulo `null`): nenhum valor inventado. `humanoid_shoulder.json` e `cape_back.json`: idem, pendentes.

## Determinismo e cache

RNG local semeado por texto (`random.Random("<seed>:<template>:<clipe>:<quadro>:<detalhe>")`), nunca global; arquivos lidos em ordem ordenada; texels percorridos em ordem ordenada; formas, dobras, barra rasgada e pelagem semeadas pelo **template** (iguais em todos os itens do template); só o desgaste usa a seed da receita. `provenance/<id>.json`: versão do gerador (`n41b-1`), SHA-256 da receita canônica, SHA-256 de cada entrada (receita, templates, perfis, materiais, detalhes, família, camadas vestidas, `protagonist.ron`, scripts de poses e o próprio gerador) e de cada saída, e a chave de cache (hash de tudo isso). O lote pula itens com chave e saídas iguais (`cached`) e **recusa sobrescrever** saídas de outra chave sem `--rebuild`.

## Integração

- **Tronco:** manifesto RON da família `torso_plate` (idle para os 4 templates; `run_c1` só para `segmented_cuirass`, a família da corrida C1). HIDE automático: só a couraça segmentada cobre toda a `shirt_torso` da corrida; os demais não ocultam nada. No idle não há HIDE (camisa e manga são uma camada só).
- **Espadas, capas, ombreiras:** isoladas (`stage/factory/isolated/`), com pivô e canvas na proveniência. Bloqueios: soquete da mão não confirmado; sem camada `shoulder`/`back` em `protagonist.ron`.
- **Exemplo:** `cargo run --example n4_sprite_machine -- --factory [--run-c1]` carrega `equipment/` + `factory/items/` com o mesmo loader; o roteiro de capturas sai dos dados (todos os itens com folha do clipe + um espelhado) e a contagem de camadas visíveis é conferida com `resolve_facing`. Tecla E percorre os itens da fábrica no modo interativo.
