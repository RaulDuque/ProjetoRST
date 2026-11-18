# N3.7 — Velocidade da corrida

## O jogo

`PLAYER_RUN_SPEED = 310` unidades/s (`src/core/constants.rs`), sem alteração. O personagem tem 53 unidades de altura (106 texels a 2 texels por unidade), então o jogo corre a **5,8 alturas de corpo por segundo**. É uma corrida de ação rápida: um velocista humano real fica perto de 4,4 alturas/s.

## O vídeo

- Corrida no lugar, com câmera fixa, 24 fps, ciclo de 12 quadros (0,5 s) e dois passos.
- **O quadril não anda:** a coluna do quadril varia 2 unidades no ciclo. Portanto, a velocidade "real" da corrida é a velocidade com que o pé apoiado recua na tela.
- Medida: **157 u/s** (média dos dois passos, de 150 e 164 u/s), com faixa de 112 a 169 u/s entre quadros apoiados (`N3_7_DESLIZAMENTO.md`).
- Cadência: 4 passos/s. Cada passo cobre cerca de 39 u (78 texels).

## A comparação

| | Valor |
|---|---|
| velocidade do jogo | 310 u/s |
| velocidade da passada a 12 fps | 157 u/s |
| razão | **1,97** |
| estimativa antiga (N3.6) | 78 u/s, corrigida aqui |

Se o `run6` tocar a 12 fps enquanto o corpo anda a 310 u/s, o pé apoiado escorrega 153 u/s para a frente, ou seja 49 % do deslocamento. O personagem parece "patinar".

## Por que não mudar `PLAYER_RUN_SPEED`

O briefing proíbe, e com razão: a velocidade define o alcance dos pulos, o tempo de reação aos inimigos e o desenho das salas. Reduzir para 157 u/s deixaria a corrida mais lenta que a caminhada atual (190 u/s). O problema é estudado só na apresentação (`N3_7_SINCRONIZACAO.md`).
