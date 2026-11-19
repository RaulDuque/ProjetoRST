# N41-B — Qualidade, diversidade, determinismo, desempenho, limitações e riscos

Números de `docs/development/n4/evidence/n41b/*.json`, gerados por `scripts/factory_report.py` sobre as saídas reais (42 receitas: as 40 da prova + 2 da prova de automação).

## Controle automático (`qa.json`)

**ERRO TÉCNICO** (invalida o item): PNG ausente ou fora de RGBA, dimensão errada, alfa parcial (anti-aliasing/mistura), cor fora das paletas declaradas pelo item (prova de grade 1:1 sem escala nem interpolação), preto puro, grade TXT diferente do alfa do PNG, item tocando a borda do canvas, quadro vazio, texel fora da silhueta vestida (flutuando), buraco causado pelo HIDE.

**AVISO ARTÍSTICO** (exige revisão): contorno fraco (< 60% da borda escura), menos de 4 tons do material principal, uma cor cobrindo > 40% do item (preenchimento plano), poucos texels visíveis (< 40), detalhe pulado por não caber, e os `known_issues` que o agente registrou no template após olhar as evidências.

| Resultado | Itens |
|---|---:|
| Receitas válidas | 42 / 42 |
| PNGs produzidos | 42 |
| Tecnicamente válidos (0 erro técnico) | 42 |
| Com aviso artístico | 29 |
| Aprovação artística | **nenhuma** (Gate G: proprietário) |

Avisos mais comuns: carimbos que não cabem no idle (a máscara do tronco no idle tem ~5 texels de largura de cada lado da manga), preenchimento plano no idle da couraça segmentada (41–46%, o contorno domina uma faixa tão estreita), e os problemas conhecidos dos templates (abaixo).

## Diversidade (`diversity.json`)

Comparação **alinhada** (mesmo canvas, mesmo pivô, mesma pose: tronco na pose C1 #2, espada no canvas 21×64, capa/ombro no canvas 40×52), dentro de cada categoria. Métricas separadas: % de pixels visíveis diferentes, diferença de silhueta, diferença média de cor, distância L1 de histograma de paleta (cores quantizadas em 4 bits) e distância Hamming de dHash 64 bits. **Hamming de dHash não é porcentagem de pixels**; as duas são reportadas separadamente.

Classificação por par: `DUPLICATA_EXATA` (bytes iguais), `VARIACAO_MENOR` (< 8% dos pixels visíveis), `REVISAR_VISUALMENTE` (≥ 8%, mas mesma silhueta < 3% e dHash ≤ 8: recoloração), `CANDIDATO_DISTINTO`. Cada item recebe a classe do seu vizinho mais próximo.

| Classe (por item) | Itens |
|---|---:|
| `DUPLICATA_EXATA` | 0 |
| `VARIACAO_MENOR` | 0 |
| `REVISAR_VISUALMENTE` | 30 |
| `CANDIDATO_DISTINTO` | 12 |

Leitura honesta: os 30 itens de tronco, capa, ombreira e mantelete têm, dentro do mesmo template, **a mesma silhueta** e diferem em material e detalhes (pixels diferentes entre 81% e 100%, silhueta 0%). São recolorações com ornamentos diferentes, não designs novos. Entre templates a silhueta muda de verdade: tronco 6–26% de silhueta diferente entre modelos, espadas 17–37%, capa × ombreira 60%, mantelete × ombreira 11%. As 12 espadas são candidatas a designs distintos (lâmina × guarda). Nenhuma duplicata exata e nenhuma variação menor.

## Determinismo (`determinism.json`)

Duas gerações completas do zero em `target/n41b_determinism/{a,b}` (`--rebuild`): **204 arquivos idênticos byte a byte entre si e iguais aos de `stage/factory/`** (o `batch_report.json`, que tem tempos, fica fora da comparação; a proveniência não tem tempos de propósito). Testes: `test_same_recipe_same_pixels`, `test_batch_is_deterministic_cached_and_does_not_overwrite`, `test_the_seed_only_moves_seeded_details`.

## Desempenho (`performance.json`)

| Medida | Valor |
|---|---|
| Lote completo do zero (42 itens) | 0,40 s (≈ 0,01 s por item) |
| Memória de pico do gerador | 36 MiB |
| Arquivos em `stage/factory/` | 205 (0,9 MB) |
| Texturas lidas pelo jogo | 22 PNG, 17 KB |
| Custo de composição | +1 sprite (slot `armor_torso`) por item vestido; trocar = `Outfit::wear` + troca de textura no `sync_composite_layers` |
| Não medido | tempo de carga e custo por quadro no Bevy (prova isolada, sem profiler; 36 capturas sem queda perceptível) |

## Bevy (`bevy_pixel_check.json`)

36 capturas reais (`--factory`: 18 itens no idle + nenhum item + espelhado; `--factory --run-c1`: 4 itens × 3 quadros + nenhum item + espelhado), cada uma comparada com a composição calculada offline a partir dos mesmos dados (camadas vestidas + PNG do item + HIDE do RON, espelhada quando for o caso): **36/36 com 0 pixels diferentes**. O exemplo também conferiu, em cada captura, o número de camadas visíveis contra `resolve_facing` (`RESULT: PASS` nos dois roteiros).

## Testes

- Python (`python3 -B -m unittest discover -s art-src/n4-sprite-machine/scripts/tests`): **13 testes** — catálogo e biblioteca válidos; 14 receitas inválidas recusadas; materiais e detalhes inválidos recusados; determinismo; seed; relevo não plano e ≥ 4 tons; sel-outline sem preto; detalhes só no interior; itens na silhueta vestida; lote determinístico, cache e alfa binário; recusa de sobrescrita sem `--rebuild`; material + detalhe + item novos só com dados.
- Rust (`tests/n41b_factory_tests.rs`): **5 testes** — itens gerados carregam pelo loader real no slot da família (um manifesto por receita de tronco); sem buraco, sem texel flutuando, HIDE só onde cobre, oclusão pela ordem das camadas na corrida; corrida C1 limitada a uma família; mesmo quadro e espelho do `VisualPlayback`; categorias isoladas sem manifesto e dados N4-1-A intactos.
- `cargo fmt --check`, `cargo check`, `cargo clippy --all-targets -j 2 -- -D warnings` limpos; `cargo test -j 2`: **56 suítes, 680 passaram, 0 falharam**.

## Evidências (`docs/development/n4/evidence/n41b/`)

| Arquivo | Conteúdo |
|---|---|
| `n41b_catalog.png` | os 42 itens com o veredito do QA |
| `n41b_torso_templates.png` | 4 templates de tronco em ferro: sozinho, vestido (pose C1 #2), idle |
| `n41b_materials_items.png`, `n41b_material_ramps.png` | mesmo template em 4 materiais; rampas dos 17 materiais |
| `n41b_detail_stamps.png` | carimbos da biblioteca |
| `n41b_swords.png`, `n41b_capes_shoulders.png` | recursos isolados |
| `n41b_relief_maps.png` | mapas de altura por template |
| `n41b_idle_all_torso.png` | os 18 itens de tronco no idle N3.5 |
| `n41b_occlusion.png` | corrida C1: texels do peitoral cobertos pelo braço/manga (vermelho) |
| `n41b_mirroring.png` | quadro e espelho |
| `n41b_run_c1_family.png`, `n41b_run_c1_family.gif` | família da corrida C1 (#0, #2, #4), 4 materiais |
| `n41b_diversity_nearest.png` | cada item ao lado do vizinho mais próximo |
| `n41b_spotlight.png` | prova de automação: os 2 itens feitos só com dados, inclusive a captura do Bevy |
| `bevy_n41b_*.png` | 36 capturas reais do Bevy |
| `qa.json`, `diversity.json`, `determinism.json`, `performance.json`, `bevy_pixel_check.json` | relatórios |

## Problemas artísticos (revisão do agente; a aprovação é do proprietário)

1. **Tronco estreito.** Na corrida, o braço da frente e a manga de trás ficam por cima; o peitoral aparece como uma faixa entre os braços. No idle, camisa e manga são uma camada só: o peitoral aparece em duas faixas estreitas em volta da manga. Sem ombros cobertos, a peça ainda pode ler como "adesivo".
2. **Desgaste no ferro** (`wear_scratches`) deixa as peças de ferro ruidosas.
3. **Ombreira:** melhorou com lâminas curvas e cúpula, mas isolada, sem ombro de referência nem correia.
4. **Mantelete:** a gola de pelagem lê como faixa de textura; a silhueta lembra uma saia.
5. **Falcão:** curvatura do gume discreta e ponta curta.
6. **Escala:** no idle, carimbos maiores que 2×2 não cabem; emblemas só aparecem na corrida/prévia.
7. Recolorações: dentro de um template, os itens mudam material e detalhes, não silhueta.

## Trabalho manual

Por item novo de um template existente: **1 arquivo JSON** (receita), 0 quadro desenhado, 0 código. Por material ou detalhe novo dentro do contrato: 1 JSON. Por template novo de tronco: 1 JSON (forma + relevo). O que ainda é manual: revisão artística, `known_issues`, marcação de soquetes e polígonos de pose (C2), e pintura final se a qualidade procedural não for aceita.

## Extensões que exigem código (separadas)

Uma **operação** gráfica nova (novo tipo de relevo, novo tipo de detalhe, nova regra de forma) é código em `factory.py` + teste. Nesta etapa foram criadas as operações `cylinder`, `dome`, `bands` (com `curve`), `ridge`, `rim`, `grid`, `skirt_segments`, `scales`, `folds`, `drape`, `blade_bevel`, `fuller` e os detalhes `stamp`, `edge_band`, `line`, `scatter`.

## Riscos

| Risco | Nível | Mitigação |
|---|---|---|
| Tomar a pintura procedural por arte final | alto | Gate G com o proprietário; QA nunca aprova arte |
| Volume artificial (recolorações contadas como designs) | médio | diversidade por silhueta e dHash; relatório separa recoloração de design |
| Espada/capa integradas sem soquete | médio | categorias isoladas; teste exige que não tenham manifesto |
| Crescimento das operações do gerador | médio | operações genéricas + testes; nenhuma condição por item |
| Itens de tronco pouco legíveis no idle | médio | separar a manga da camisa do idle (decisão do proprietário) |
| Cache enganado por arquivo fora das entradas | baixo | a chave inclui o gerador, os scripts de pose e todas as camadas vestidas |
