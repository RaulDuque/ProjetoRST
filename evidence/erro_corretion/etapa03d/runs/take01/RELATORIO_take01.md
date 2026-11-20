# Etapa 03D — take01: imagem do ChatGPT lida pelo Agent Sprite Forge

- **Fonte artística:** `inbox/armour_master.png`, gerada pelo proprietário no ChatGPT a
  partir de `request_pose/reference.png` e `prompt.txt`. Cópia guardada em `source.png`.
- **Ferramenta:** Agent Sprite Forge 0.6.0, `generate2dsprite/scripts/pixel_look.py pose`.
  Rodou offline e sem nenhuma chamada de API. Não houve algoritmo próprio no lugar da ferramenta.
- **Reprodução:** `scripts/import_external.py --run take01`, depois `scripts/take_report.py --run take01`.

## 1. Compatibilidade da imagem recebida

| | Referência enviada | Imagem recebida |
|---|---|---|
| Tamanho | 1024×1024 | **1254×1254** (fator 1,2246) |
| Fundo | verde #00FF00 | verde liso (cerca de #03F903), 89 % da imagem |
| Caixa da figura | x 405–612, y 45–999 | x 496–753, y 55–1224 (= referência × 1,2246) |
| Grade | célula de 9 px | **célula de 11,021 px**, deslocamento (0, 0), encontrada pelo `pose` |

A imagem é a referência redimensionada, com a mesma posição e a mesma escala. A grade é
compatível e o `pose` a encontrou sem ajuste manual.

## 2. Resultado do `pixel_look pose`

- **Leitura da imagem:**
  - A imagem confere com o master em **60,5 %** das células (`pictureEqualShare`).
  - O ChatGPT repintou também áreas que não foram pedidas, com pequenas variações de cor.
- **O que a ferramenta aproveitou:**
  - Foram aproveitadas **551 células**: a parte nova e o que está ligado a ela.
  - Dessas, 86 são células novas fora da silhueta, e 4 ficaram vazias.
  - **O resto é o pixel original (`masterKeptShare` 64,8 %).**
- **Contorno e fundo:** o contorno foi fechado por regra (cor `K` 26,20,26) e não sobrou resíduo do verde.
- **Opções documentadas da ferramenta testadas** (`variants/`):
  - `--smallest 30` e `--smallest 60` deram o mesmo resultado (0 e 1 px de diferença).
  - `--new-colours 8` deu 25 cores, com 196 px trocados para tons próximos.
  - A skill não tem máscara de proteção. A regra documentada para uma versão que erra é
    gerar outra imagem, sem conserto à mão.

## 3. Canvas do jogo (80×128, pivô 40,118)

- `armour_on_canvas_80x128.png`: o encaixe ficou no deslocamento (26, 13), o mesmo do
  sprite aprovado, com os pés na mesma linha.
- O sprite N3.5 não foi alterado; ele foi apenas lido.

| Zona | Texels | Idênticos | Mudaram |
|---|---|---|---|
| Cabeça (linhas 0–26: cabelo do alto e rosto) | 175 | **175 (100 %)** | 0 |
| Pernas (linhas 72–127: calça abaixo das escarcelas e botas) | 465 | **465 (100 %)** | 0 |
| Cabelo em qualquer lugar (chaves W, w, h, H) | 207 | 164 (79,2 %) | **43** |
| Faixa do tronco (linhas 27–71), onde a armadura foi pedida | 680 | 215 | 465 |

**Totais:**
- texels alterados: **562**;
- silhueta nova: **97**;
- removidos: 2;
- **todas** as mudanças ficam nas linhas 27–71.

**Transparência, cores e contorno:**
- **Alfa:** binário (0 e 255).
- **Cores:** 17 → **29** (+12, rampas de roxo e dourado).
- **Contorno:** as 62 células de borda que mudaram são todas da cor de contorno `K`.

## 4. Legibilidade em 1× (`take01_torso_source_vs_1x.png`)

| Peça | Leitura em 1× |
|---|---|
| Ombreiras | **Legíveis como placas sobrepostas** (3 lâminas com borda dourada), não como esferas. Viram faixas horizontais. |
| Peitoral | Lê como placa roxa com borda dourada. **A quilha central e a gola alta quase somem.** |
| Braçais | Antebraço roxo com borda dourada no punho; a mão de pele continua original. |
| Detalhes dourados | **Quebrados:** na imagem as faixas têm cerca de 1 célula de largura, com contorno escuro anti-serrilhado; na grade de 1× viram segmentos e pontos. |
| Cintura e escarcelas | Legíveis como saiote de placas com borda dourada. |

**Por que perde qualidade:** o ChatGPT desenhou a armadura com "pixels" menores que a
grade pedida (cerca de 1/2 célula, bordas suavizadas). A redução para a grade de 9 px
mantém a cor e a massa, mas não as linhas finas.

## 5. Defeitos registrados (não corrigidos à mão)

1. **43 texels de cabelo trocados:** mechas sobre o ombro e as costas foram substituídas
   pela ombreira e pelo contorno. Na imagem do ChatGPT a ombreira já fica por cima do
   cabelo. É uma escolha da imagem, não erro de conversão, mas muda a camada aprovada.
2. **Pescoço:** a gola cobre pele do pescoço (linhas 27–35), o que é coerente com a armadura.
3. **Linhas douradas fragmentadas** e quilha pouco visível em 1× (item 4).
4. **Fidelidade fora da área pedida:** a imagem mudou cerca de 40 % das células que não
   deveria; o `pose` descartou essas mudanças fora do tronco. **A fidelidade é perfeita
   só para cabeça e pernas; não é perfeita para o cabelo.**

## Arquivos

| Arquivo | Conteúdo |
|---|---|
| `take01_comparison_sheet.png` | Original, ChatGPT, Agent Sprite Forge e diferenças; 1×, 2× e 3×; torso a 8× |
| `take01_torso_source_vs_1x.png` | Tronco da imagem contra a leitura em 1× |
| `comparison.png`, `preview_1x_2x.png` | Comparação e prévias do importador |
| `armour_on_canvas_80x128.png` | Sprite final 80×128 (não integrado) |
| `pose/armour.png`, `armour.json`, `armour-review.png` | Saída oficial do `pixel_look pose` |
| `metrics.json`, `zone_metrics.json` | Métricas |
| `variants/` | As 3 variações de opções do `pose` |
