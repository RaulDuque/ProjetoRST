# N3.6 — Análise da corrida

Evidências: `run_pose_analysis.png` (todas as poses de corrida de todas as fontes) e `run_selected_frames.png`.

## Poses disponíveis

| Fonte | Poses | Vista | Avaliação |
|---|---|---|---|
| `run_a` #1–8 | 8 | lateral, direita | poses bonitas e independentes. Não há ordem de ciclo: #3 tem o braço totalmente esticado, que não aparece em nenhuma outra, e #1, #5 e #8 são variações da mesma fase de voo. |
| `run_jump` #1–8 | 8 | lateral, direita | mesmo estilo de `run_a`, mas a escala e o caimento do cabelo variam entre as poses. #7 e #8 já são o início do salto. |
| `idle_run_jump` #7–14 | 8 | **3/4** | é outra vista, outra camisa, outra cor de bota e tem a fita solta. Incompatível com um clipe lateral. |
| vídeo, quadros 48–59 | 12 | lateral, direita | **continuidade real**: o mesmo personagem, quadro a quadro, com câmera fixa. |

Misturar poses de folhas diferentes num clipe mudaria escala, tom da camisa e forma do cabelo de um quadro para o seguinte. Montar um ciclo só com uma folha exigiria escolher uma ordem que a folha não define e, provavelmente, inventar as fases que faltam.

## O ciclo do vídeo

O período foi medido pela diferença média entre quadros do trecho de corrida (quadros 30 a 188), reduzidos a 80×100, com o fundo verde zerado:

| Intervalo (quadros) | Diferença média |
|---|---|
| 6 | 14,74 |
| 9 | 21,44 |
| **12** | **10,16** (mínimo) |
| 18 | 15,61 |
| 24 | 13,20 |

O ciclo completo tem **12 quadros (0,5 s)**, com dois passos: 48–53 e 54–59, em que pernas e braços trocam de lado. O quadro 60 repete o 48, o que fecha o laço.

## Fases observadas (quadros do vídeo)

| Quadro | Fase |
|---|---|
| 48 | passagem: perna de apoio sob o corpo, perna livre dobrada atrás |
| 49 | **fantasma**: interpolação com pernas duplicadas |
| 50 | impulsão: perna de trás estendida, joelho da frente subindo |
| 51 | voo (sola mais baixa na linha 113, fora do chão) |
| 52 | voo, perna da frente estendida para o contato |
| 53 | contato (sola na linha 118) |
| 54 | passagem do outro lado (braço próximo à frente, com bracelete) |
| 55 | **fantasma** |
| 56 | impulsão |
| 57 | voo (linha 113) |
| 58 | contato |
| 59 | contato e início da passagem |

## Decisão

A prova de corrida usa **somente o vídeo**. As folhas ficam como referência de pose e de acabamento. Nenhum quadro foi inventado, interpolado ou redesenhado.
