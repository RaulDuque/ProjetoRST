# N3.6 — Seleção e extração dos quadros

Evidências: `run_selected_frames.png`, `run_cycle.png` e `run_bevy_frames.png`.

## Procedência

Todos os quadros vêm de `trimmed_fd4c76cf-0416-4387-b451-da4cc90a2061.mp4`, decodificado pelo GStreamer (`scripts/extract_video.py`). O tempo de cada quadro é número/24 s. Todos usam a mesma janela de recorte, `(0, 150, 496, 702)` em pixels do vídeo, portanto a "posição na folha" é a mesma para todos. Os intermediários com chroma key, em resolução cheia, ficam em `art-src/n3-6-animation-proof/frames/video_NNN.png`.

## Tabela

| Id | Tempo | Fase | Apoio | Braços | Torso | Cabelo | Qualidade | Problema | Decisão |
|---|---|---|---|---|---|---|---|---|---|
| 0 | 0,000 s | de pé | os dois pés | soltos | ereto | caído | nítido | — | `idle` (quadro de referência) |
| 48 | 2,000 s | passagem | perna sob o corpo | próximo atrás | leve inclinação à frente | para trás | nítido | — | run6, run10 |
| 49 | 2,042 s | transição | — | — | — | — | **pernas duplicadas** | fantasma de interpolação | descartado |
| 50 | 2,083 s | impulsão | ponta do pé de trás | trocando | inclinado | para trás, ondulado | nítido | — | run6, run10 |
| 51 | 2,125 s | voo | nenhum | abertos | inclinado | para trás | nítido | — | run10 |
| 52 | 2,167 s | voo, alcance | nenhum | abertos | inclinado | para trás | nítido | — | run6, run10 |
| 53 | 2,208 s | contato | pé da frente | abertos | inclinado | para trás | nítido | — | run10 |
| 54 | 2,250 s | passagem (outro lado) | perna sob o corpo | próximo à frente | leve inclinação | para trás | nítido | — | run6, run10 |
| 55 | 2,292 s | transição | — | — | — | — | **pernas duplicadas** | fantasma | descartado |
| 56 | 2,333 s | impulsão | ponta do pé de trás | trocando | inclinado | para trás | nítido | — | run6, run10 |
| 57 | 2,375 s | voo | nenhum | abertos | inclinado | para trás | nítido | — | run10 |
| 58 | 2,417 s | contato | pé da frente | abertos | inclinado | para trás | nítido | — | run6, run10 |
| 59 | 2,458 s | contato e passagem | pé da frente | fechando | inclinado | para trás | nítido | — | run10 |

## Clipes

- **run6:** os quadros pares 48, 50, 52, 54, 56 e 58 (um a cada dois do vídeo), a 12 fps. É exatamente a velocidade do vídeo, e nenhum quadro com fantasma entra. É o clipe principal.
- **run10:** os 10 quadros nítidos, cada um durando até o próximo quadro mantido no vídeo (48 e 54 duram 2/24 s, os demais 1/24 s). Dá a temporização real, sem os fantasmas.

Os dois somam 0,5 s por ciclo, e um teste verifica isso.

## Extração

1. **Chroma key:** um pixel é fundo quando o verde supera o vermelho e o azul em mais de 30.
2. **Máscara** erodida em 1 px, como na N3.5, para tirar a borda misturada ao verde.
3. **Excesso de verde** limitado a max(vermelho, azul).

O cabelo e as botas não foram cortados: o recorte vai do alto do cabelo (linha 221 do vídeo) até abaixo das solas (674), em toda a largura.

## Limitações

- O vídeo foi gerado e não é pixel art. A qualidade de cada quadro é a da redução, igual à da N3.5.
- Restos de borrão de movimento podem sobrar em volta das pernas nos quadros de voo. Eles foram mantidos, porque removê-los exigiria redesenhar.
- Não houve edição à mão nos quadros (`hand_edits: 0` em `generated/metrics.json`).
