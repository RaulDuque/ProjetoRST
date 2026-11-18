# N3.6 — Pivô e alinhamento

Evidências: `run_alignment.png` (os 10 quadros sobrepostos, com a caixa de cada um e o pivô), `run_anatomy_comparison.png` e `bevy_n36.png` (sobreposição no Bevy).

## Medidas no vídeo (pixels do vídeo)

| Quadro | Topo | Sola | Borda traseira do quadril |
|---|---|---|---|
| 0 (de pé) | 221 | 670 | 224 |
| 46–61 (corrida) | 228–236 | 650–674 | **208–216** |

O quadril não anda mais que ±4 px de vídeo (±1 texel). A câmera é fixa e a corrida é no lugar.

## Decisão

- **Coordenadas:** cada quadro mantém as coordenadas do vídeo. Há um único deslocamento global, `(-3, -5)` texels depois da redução. Nenhum quadro foi movido sozinho, então não existe tremor criado por alinhamento.
- **Pivô lógico:** `(56, 118)` no canvas de 112×128, igual em todos os clipes.
  - **x** = a coluna do quadril no quadro 0, no centro da figura a 45 % da altura.
  - **y** = a sola mais baixa entre os quadros de contato.
- **Linha do chão:** linha 118. Os quadros de contato (48, 53, 54, 56, 58 e 59) tocam essa linha. O 50, na impulsão, fica na ponta do pé, na linha 117. Os de voo sobem: 51 e 57 até a linha 113, 52 até a 114. O balanço vertical de 5 texels é o do vídeo.
- **Quadro de pé:** a sola fica na linha 117, 1 texel acima das passadas, por arredondamento da redução. Foi mantido assim e está documentado.

## Caixas por quadro (texels do canvas)

`generated/boxes.ron` e `generated/metrics.json`. A largura vai de 54 a 85 texels: de 30 a 83 no quadro 54 (passagem) até de 10 a 94 nos quadros 52 e 53 (passada aberta). O topo da cabeça fica entre as linhas 14 e 16 em todos os quadros de corrida.

## Parado e com deslocamento

- **Parado:** é o modo padrão do exemplo (`in place`).
- **Com deslocamento** (tecla T): o corpo cruza a vista a 78 unidades/s e volta a entrar pelo outro lado. Esse valor é uma estimativa do deslize do pé de apoio no ciclo (`planted_sole_speed_units_per_s`).

## Achado: velocidade

A corrida do jogo é de **310 unidades/s** (`PLAYER_RUN_SPEED`). Pela passada do vídeo, os pés acompanham o chão entre cerca de 78 e 170 unidades/s: a primeira é a estimativa pelo pé de apoio, a segunda pela abertura máxima da passada. Na velocidade do jogo, os pés **deslizariam**. Para casar, seria preciso tocar o clipe cerca de 2 a 4 vezes mais rápido (`speed_reference`), dar ao personagem uma passada maior ou reduzir a velocidade do jogo. Não decidi nada disso. O manifesto deixa `speed_reference: None`.
