# Etapa 03D — take02: leitura de `inbox/armour_master.png`

> **Aviso principal:** a imagem de entrada deste take é **a mesma do take01**, byte a byte.
> - sha256 `d5aed573…a7d9c` em `inbox/armour_master.png`, `runs/take01/source.png` e `runs/take02/source.png`;
> - o arquivo é de 20/11 às 00:15, anterior ao take01;
> - o texto `request_pose/take02_prompt.txt` ainda **não** foi usado em nenhuma imagem do ChatGPT.
>
> O proprietário pediu para processar esta imagem mesmo assim. O fluxo oficial foi rodado
> por completo, e todos os números abaixo são reais. Como a entrada é a mesma, o resultado
> final é idêntico ao do take01.

- **Ferramenta:** Agent Sprite Forge 0.6.0, `pixel_look.py pose`, offline, sem API paga.
- **Pedido usado:** `request_pose_take02/` (mesmo `master.png` e mesma `reference.png` do take01).
- **Reprodução** (da raiz do projeto):

      python3 -I -B evidence/erro_corretion/etapa03d/scripts/import_external.py --image evidence/erro_corretion/etapa03d/inbox/armour_master.png --run take02 --request evidence/erro_corretion/etapa03d/request_pose_take02
      python3 -I -B evidence/erro_corretion/etapa03d/scripts/take_report.py --run take02
      python3 -I -B evidence/erro_corretion/etapa03d/scripts/restore_hair.py --run take02
      python3 -I -B evidence/erro_corretion/etapa03d/scripts/compare_takes.py --takes take01 take02 --out take02/compare_check
      python3 -I -B evidence/erro_corretion/etapa03d/scripts/compare_take02_sheet.py

## 1. Leitura da grade

**Funcionou.** O `pose` achou sozinho uma célula de **11,021 px** com deslocamento (0, 0).
- 1254 / 1024 × 9 = 11,02, ou seja, é a mesma grade da referência ampliada.
- A imagem confere com o master em 60,5 % das células (`pictureEqualShare`).
- 551 células foram aproveitadas; o resto continua com o pixel original (`masterKeptShare` 64,8 %).

## 2. Posição no canvas do jogo

**Preservada.** O sprite ocupa o canvas de 80×128 no deslocamento (26, 13), com pivô 40,118.
É o mesmo encaixe do sprite N3.5 aprovado, com os pés na mesma linha
(`armour_on_canvas_80x128.png`).

## 3. Métricas por região (contra o N3.5 aprovado)

| Região | Texels | Idênticos | Alterados |
|---|---|---|---|
| Cabeça (linhas 0–26, inclui o rosto) | 175 | **175 (100 %)** | 0 |
| Rosto (pele acima da linha 36) | 33 | 31 | 2 (pescoço coberto pela gola, linhas 27–35) |
| Cabelo (camadas N4, antes da restauração) | 235 | 211 | **24** |
| Pernas (linhas 72–127: calça e botas) | 465 | **465 (100 %)** | 0 |
| Faixa do tronco (linhas 27–71) | 680 | 215 | **465** |

Outros números:
- **Total alterado:** 562 texels antes da restauração e 538 depois.
- **Silhueta:** +97 texels ganhos e 2 perdidos. Tudo nas linhas 27–71.
- **Transparência:** alfa binário (0 e 255).
- **Cores:** 17 → 29, ou seja, **12 cores novas** (rampas de roxo e dourado).
- **Contorno:** fechado. As 62 células de borda que mudaram usam todas a cor de contorno `K`.

Sobre calça e botas, as contagens por material em `metrics.json` mostram 84 e 69 texels.
Esses texels ficam na faixa do tronco, cobertos pelas escarcelas e pelo cinto. Abaixo da
linha 72 nada mudou.

## 4. Restauração do cabelo (camadas N4)

`restore_hair.py` compôs `hair_back` e `hair_front` da N4 por cima da armadura, na ordem
de camadas aprovada. Nenhum pixel foi redesenhado.
- **24** texels de armadura voltaram a ser cabelo.
- Depois da restauração, o cabelo ficou com **0 texels diferentes** (235/235).
- Arquivos: `hair_restore/`.

## 5. Take01 x Take02 (`takes_01_vs_02_comparison.png`, `takes_dark_preview.png`)

| Par | Pixels diferentes |
|---|---|
| take01 final (cabelo N4) x take02 + cabelo N4 | **0** |
| take01 final x take02 bruto | 24 (só o cabelo restaurado) |
| aprovado x take02 + cabelo N4 | 538 |

**O take02 não melhorou nem piorou em relação ao take01: é o mesmo sprite.**

## 6. Avaliação visual (pelas imagens geradas)

- **Armadura mais legível que no take01?** Não. É idêntica.
- **Quilha central mais visível?** Não. Em 1× ela continua quase sumida, como no take01.
- **Faixas douradas com largura útil?** Não. Na imagem elas têm cerca de 1 célula e bordas
  suavizadas; em 1× viram segmentos e pontos, sobretudo no peitoral e no braçal.
- **Ombreiras como placas planas?** Sim, em parte. Leem como 3 lâminas sobrepostas (não
  esferas), mas em 1× parecem faixas horizontais listradas.
- **Regressão em cabelo, rosto, calça ou botas?** Não.
  - Cabelo: 0 após a restauração N4.
  - Cabeça e pernas: 100 % idênticas.
  - Pescoço: 2 texels cobertos pela gola, coerente com a armadura.
- **Pronto para aprovação?** Não. Os mesmos defeitos do take01 continuam: dourado fino e
  fragmentado, quilha invisível e excesso de listras.

## 7. Conclusão

**Rejeitado como take02.** Ele não traz imagem nova: repete o take01, com os mesmos
defeitos e as mesmas métricas. Serve só como prova de que o fluxo é reproduzível, porque a
mesma entrada deu a mesma saída, pixel a pixel.

Próximo passo: gerar uma imagem nova no ChatGPT com `request_pose/reference.png` e
`request_pose/take02_prompt.txt`, salvar como `inbox/armour_master_02.png` e rodar o take
de novo.

## Arquivos

| Arquivo | Conteúdo |
|---|---|
| `source.png` | Cópia da entrada (idêntica à do take01) |
| `pose/` | Saída oficial do `pixel_look pose` |
| `armour_on_canvas_80x128.png` | Sprite final 80×128, pivô 40,118 (não integrado) |
| `take02_comparison_sheet.png` | Original, ChatGPT, leitura e diferenças; tronco a 8× |
| `take02_torso_source_vs_1x.png` | Tronco da imagem contra a leitura em 1× |
| `comparison.png`, `preview_1x_2x.png` | Comparação e prévias do importador |
| `hair_restore/` | Sprite com cabelo N4 restaurado, folha e JSON |
| `takes_01_vs_02_comparison.png` | Aprovado, take01 final, take02 bruto e take02 + N4; 1×, 2× e tronco 8× |
| `takes_dark_preview.png` | As mesmas 4 versões sobre fundo escuro, 1× e 3× |
| `takes_01_vs_02_pairs.json` | Pixels diferentes entre cada par |
| `compare_check/` | Saída de `compare_takes.py` (take01 x take02, com cabelo N4) |
| `metrics.json`, `zone_metrics.json` | Métricas |
