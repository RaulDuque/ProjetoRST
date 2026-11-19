# N4-1 (C1) — Diferenças visuais em relação à N3.6

## Vestido × aprovado

| Medida | #0 | #2 | #4 |
|---|---|---|---|
| Texels opacos do original | 2219 | 2458 | 2487 |
| Texels diferentes (grade de paleta, Rust, ordem do RON) | **0** | **0** | **0** |
| Texels RGBA diferentes (PNG, Python) | **0** | **0** | **0** |
| Silhueta (texels opacos só de um dos lados) | **0** | **0** | **0** |
| Linha mais baixa (contato com o chão), original / modular | 118 / 118 | 114 / 114 | 118 / 118 |
| Pivô | (56, 118) / (56, 118) | idem | idem |
| Bevy real, pixels diferentes (4 px/texel) | **0** de 229 376 | **0** | **0** |
| Bevy real espelhado (#2) | — | **0** | — |

Os pixels da figura no Bevy (35 504, 39 328 e 39 792) são exatamente os texels × 16, o que confirma escala e pivô. Fontes: `evidence/n4c/metrics.json`, testes `each_c1_frame_recomposes_the_approved_frame_texel_for_texel` e `the_c1_clip_uses_the_approved_run_canvas_and_pivot`, e `check_bevy.py`.

Mapas de diferença: `evidence/n4c/n4c_recomposition.png` (original, modular e mapa para cada quadro). Verde-escuro = igual; vermelho = diferente. **Não há vermelho.**

## O que muda quando uma peça sai (não é diferença do vestido)

Texels opacos no original que ficam transparentes depois de remover a peça (a roupa é maior que o corpo):

| Remoção | #0 | #2 | #4 |
|---|---|---|---|
| cabelo | 302 | 331 | 317 |
| camisa | 88 | 20 | 60 |
| calça | 259 | 202 | 202 |
| botas | 120 | 204 | 204 |
| braceletes | 0 | 0 | 0 |
| tudo (corpo-base) | 769 | 757 | 783 |

Cavidades fechadas novas (transparência cercada por corpo): **0** em todas as remoções e no corpo-base, nos 3 quadros. O original já tem 0, 44 e 53 vãos fechados (entre braço dobrado e tronco), que continuam iguais.

## Interpretação

A igualdade exata prova que a máquina recompõe a corrida aprovada sem perder um texel. Ela não diz nada sobre a qualidade da anatomia: essa é avaliação visual (`N4C_ANATOMIA_POSES.md`, "Defeitos").
