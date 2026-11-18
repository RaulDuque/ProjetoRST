# N3.7 — Contraste em fundo escuro

Script: `art-src/n3-7-run-sync/scripts/contrast_study.py`. Dados: `analysis/contrast_metrics.json`. Evidências:
- `dark_original.png`: versão A em quatro fundos;
- `dark_rim_light.png`: versão B em quatro fundos;
- `dark_comparison.png`: as quatro versões em quatro fundos;
- `dark_comparison.gif`: as quatro versões em movimento, no quase preto, a 12 fps.

## Versões (todas identificadas, nenhuma sobrescreve o `run6`)

| Versão | O que muda | Arquivo |
|---|---|---|
| A original | nada | `n3-6-animation-proof/generated/run6.png` |
| B borda de luz | do quadril (linha 62) para baixo, o texel escuro (luminância ≤ 60) cujo vizinho de trás está transparente vira `q`, o cinza frio da própria paleta. São 512 texels nos 6 quadros, 1 texel de largura, só nas costas | `generated/run6_rim.png` |
| C botas +1 tom | da linha 92 para baixo, K→d, d→l e o→p (um degrau da paleta). São 1897 texels nos 6 quadros | `generated/run6_boots.png` |
| D contraluz no cenário | sprite intacto; o fundo quase preto ganha uma luz fria suave atrás do personagem | `generated/backlight.png` |

Nenhuma cor nova: B e C usam só chaves da paleta N3.4. Não há contorno branco nem calça clareada.

## Fundos

Claro (226), cinza (150), quase preto (18, 14, 22) e um recorte da cena real do castelo (`target/world_visual_showcase/after_square.png`, uma captura do jogo).

## Métrica de borda

Diferença média de luminância entre os texels da borda da silhueta, do quadril para baixo, e o fundo logo atrás deles, na média dos 6 quadros:

| Versão | claro | cinza | quase preto | castelo |
|---|---|---|---|---|
| A original | 196,7 | 122,3 | **12,3** | **10,1** |
| B borda de luz | 179,8 | 105,4 | **29,2** | **21,1** |
| C botas +1 tom | 190,0 | 115,6 | 18,9 | 9,7 |
| D contraluz | 196,7 | 122,3 | 27,9 | 10,1 (sem luz na cena) |

## Leitura

- A calça e as botas da arte aprovada quase somem no escuro: diferença de 10–12 de luminância, contra mais de 120 no cinza. É o defeito que a N3.6 apontou.
- **B** é a mais eficaz e a mais barata: mais que dobra a borda no escuro e no castelo, mexe só em 1 texel de largura nas costas e quase não muda nada nos fundos claros.
- **D** resolve tanto quanto B sem tocar no sprite, mas só onde a cena tiver luz atrás do personagem. Na captura do castelo não há essa luz.
- **C** ajuda pouco (18,9) e muda o volume das botas em todos os fundos.
- No GIF em movimento, B lê as pernas alternando, e A só mostra a camisa e o cabelo.

## Recomendação

Sem aplicar nada: **B** como candidata para a corrida, e **D** como diretriz de iluminação para salas muito escuras. A decisão é do proprietário. C não é recomendada. O `run6` aprovado continua como está.
