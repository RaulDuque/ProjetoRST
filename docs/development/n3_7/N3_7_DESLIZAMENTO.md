# N3.7 — Medição dos pés

Script: `art-src/n3-7-run-sync/scripts/foot_analysis.py`. Dados: `analysis/foot_metrics.json`. Evidências: `foot_contact_analysis.png` e `foot_sliding_metrics.png`.

## Método

- Fonte: os recortes em resolução cheia da N3.6 (`frames/video_048..059.png`), todos com a mesma janela, então as coordenadas são comparáveis. 1 px de vídeo = 0,237 texel = 0,118 unidade.
- **Pés:** regiões opacas conectadas nos 60 px mais baixos da figura.
- **Contato:** o pé cujo ponto mais baixo fica a até 4 px do chão do ciclo (linha 523). A área de contato são os pixels dele nas 7 linhas mais baixas.
- **Fases:**
  - **apoio:** a área de contato cobre ≥ 55 % do pé;
  - **contato (calcanhar):** a área está só atrás. O corpo olha para a direita, então o calcanhar é a ponta esquerda;
  - **impulsão (ponta):** a área está só na frente;
  - **voo:** nenhum pé no chão.
- **Corpo:** a coluna central do quadril, na linha do quadril do quadro 0.
- **Velocidade sem deslizamento:** a velocidade com que a área de contato do mesmo pé recua entre quadros consecutivos, convertida por × 0,118 u/px × 24 quadros/s.

## Resultado por quadro (px de vídeo)

| Quadro | Fase | Pé apoiado (x0–x1) | Área de contato |
|---|---|---|---|
| 48 (run6) | apoio | 237–319 | 237–319 |
| 49 (fantasma) | apoio | 178–258 | 179–258 |
| 50 (run6) | impulsão | 84–147 | 117–147 |
| 51 | voo | — | — |
| 52 (run6) | voo | — | — |
| 53 | contato | 327–416 | 337–350 |
| 54 (run6) | apoio | 211–297 | 213–296 |
| 55 (fantasma) | apoio | 155–238 | 192–238 |
| 56 (run6) | impulsão | 112–189 | 152–189 |
| 57 | voo | — | — |
| 58 (run6) | contato | 320–413 | 335–351 |
| 59 | apoio | 281–365 | 281–364 |

Fases no `run6`: apoio, impulsão, voo, apoio, impulsão, contato.

## Velocidades

| Medida | u/s |
|---|---|
| passo 58 → 50 (4 quadros, do contato à impulsão) | 150 |
| passo 53 → 56 (3 quadros) | 164 |
| **referência (média dos passos)** | **157** |
| entre quadros apoiados (59→48, 48→49, 54→55) | 112–169 |
| N3.6 (método antigo) | 78 |

O quadril varia 2 unidades no ciclo, então a corrida é mesmo no lugar.

## Por que a N3.6 deu 78

`planted_speed` comparava só a borda da sola na linha do pivô e aceitava deslocamentos de até 8 texels por quadro. Os deslocamentos reais são de 9 a 14 texels por quadro, e foram descartados. Sobraram os pares de rolagem do pé, que são menores.

## Limites (não é biomecânica exata)

- O vídeo foi gerado por IA e tem quadros interpolados: os quadros 49 e 55 mostram pernas duplicadas.
- Não há root motion: a corrida foi feita no lugar.
- O pé rola do calcanhar para a ponta. O centro da área de contato se move mesmo sem deslizar, o que espalha os valores entre quadros (112–169). Por isso a referência é o passo inteiro.
- O passo 53→56 tem um quadro a menos que o passo 58→50: o vídeo não é perfeitamente simétrico.
- A redução arredonda para texels inteiros. No `run6` a 12 fps só três quadros de cada passo são vistos.
