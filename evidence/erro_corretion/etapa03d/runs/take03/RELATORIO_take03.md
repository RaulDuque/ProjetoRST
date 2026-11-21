# Etapa 03D — take03: imagem nova do ChatGPT lida pelo Agent Sprite Forge

- **Fonte:**
  - `inbox/armour_master_03.png`, gerada pelo proprietário no ChatGPT com
    `request_pose_take03/reference.png` e `prompt.txt`;
  - sha256 `e46890d4…4cb5b66`, diferente do take01 (`d5aed573…`);
  - o arquivo chegou em `request_pose_take03/` e foi movido, sem alteração, para `inbox/`;
  - cópia em `source.png`.
- **Ferramenta:** Agent Sprite Forge 0.6.0, `pixel_look.py pose`, offline, sem chamadas pagas.
- **Reprodução** (da raiz):

      python3 -I -B evidence/erro_corretion/etapa03d/scripts/import_external.py --image evidence/erro_corretion/etapa03d/inbox/armour_master_03.png --run take03 --request evidence/erro_corretion/etapa03d/request_pose_take03
      python3 -I -B evidence/erro_corretion/etapa03d/scripts/take_report.py --run take03
      python3 -I -B evidence/erro_corretion/etapa03d/scripts/restore_hair.py --run take03
      python3 -I -B evidence/erro_corretion/etapa03d/scripts/compare_takes.py --takes take01 take03 --out take03/compare_check
      python3 -I -B evidence/erro_corretion/etapa03d/scripts/compare_take02_sheet.py --take take03

## Grade e posição
- **Imagem:** 1254×1254 px, de novo, e não os 1024 pedidos. A grade foi detectada
  proporcionalmente.
- **Leitura do `pose`:** `pictureEqualShare` 63,6 %, 494 células aproveitadas,
  `masterKeptShare` 65,6 %.
- **Canvas:** 80×128 no deslocamento (26, 13), pivô 40,118, igual ao aprovado.

## Métricas (contra o N3.5 aprovado)

| Região | Resultado |
|---|---|
| Cabeça (linhas 0–26) | **175 de 175** idênticos |
| Rosto (pele acima da linha 36) | **33 de 33** idênticos. O pescoço não foi coberto, ao contrário do take01 |
| Cabelo nas camadas N4 | 3 texels trocados; **0** depois da restauração |
| Pernas (linhas 72–127) | **465 de 465** idênticos |
| Faixa do tronco (linhas 27–71) | 454 de 680 alterados |
| Total | 500 alterados (497 depois do cabelo) |
| Silhueta | +46 / −4 (take01: +97 / −2) |
| Alfa | binário (0 e 255) |
| Cores | 17 → 29 (+12) |
| Contorno | as 55 células de borda que mudaram usam todas a cor `K` |

**Dourado em 1×:** 53 texels dourados, contra 160 no take01. Medindo a espessura vertical
coluna a coluna, 15 trechos têm 2 px e 23 têm só 1 px. Abaixo de cada faixa há uma linha de
sombra vermelho-escura de 1 px.

**Rótulo enganoso no JSON:** em `hair_restore/hair_restore.json`, o campo
`take01_report_note` é um nome fixo do script. Os valores ali são do take03: 26 texels com
chaves de cabelo, nenhum deles cabelo N4.

## Take01 x take03
- **Diferença:** 563 px entre os finais com cabelo N4. É um desenho realmente diferente.
- **Folhas:** `takes_01_vs_03_comparison.png` (1×, 2× e tronco 8×) e `takes_dark_preview.png`.

## Avaliação visual

| Pergunta | Resposta |
|---|---|
| Mais legível que o take01? | **Sim.** Os blocos de cor são limpos e não há o ruído de listras douradas quebradas. Em 1× a leitura é clara. |
| A quilha aparece em 1×? | **Em parte.** Aparece uma faixa roxa clara contínua na frente do peito, mas no perfil ela lê como a borda iluminada do peitoral, não como uma aresta central. |
| Dourado com 2 px ou mais? | **Não sempre.** Cerca de 40 % das colunas das faixas têm 2 px; o resto tem 1 px, com sombra embaixo. Mesmo assim, as faixas são contínuas e não viram pontos. |
| A ombreira lê como 2 placas planas? | **Fraco.** Lê como a manga com duas faixas douradas. Não há degrau nem volume de placa, e a silhueta do ombro quase não cresce (+46 texels no total). |
| Regressão em cabelo, rosto, calça ou botas? | **Não.** Cabeça, rosto e pernas estão 100 % idênticos e o cabelo volta a 0 depois da N4. |
| Pronto para aprovação? | **Ainda não, na minha leitura.** O conjunto lê mais como **gibão ou túnica roxa com debrum dourado** do que como armadura de placas. A simplificação pedida resolveu o ruído, mas tirou volume e a leitura de metal. |

## Conclusão

**Melhor que o take01, mas ainda insuficiente.**
- **Ganhos:** legibilidade, limpeza e zero regressão.
- **Falta:**
  - volume de placa nas ombreiras, com silhueta mais larga e um degrau visível;
  - quilha central;
  - dourado sempre com 2 px.

A aprovação visual é do proprietário.

## Arquivos

| Arquivo | Conteúdo |
|---|---|
| `source.png` | Cópia da imagem do ChatGPT |
| `pose/` | Saída oficial do `pixel_look pose` |
| `armour_on_canvas_80x128.png` | Sprite 80×128, pivô 40,118 (não integrado) |
| `take03_comparison_sheet.png` | Original, ChatGPT, leitura e diferenças |
| `take03_torso_source_vs_1x.png` | Tronco da imagem contra a leitura em 1× |
| `hair_restore/` | Sprite com cabelo N4, folha e JSON |
| `takes_01_vs_03_comparison.png` | Aprovado, take01, take03 bruto e take03 + N4 |
| `takes_dark_preview.png` | As 4 versões sobre fundo escuro |
| `takes_01_vs_03_pairs.json` | Pixels diferentes entre cada par |
| `compare_check/` | Saída de `compare_takes.py` |
| `metrics.json`, `zone_metrics.json` | Métricas |
