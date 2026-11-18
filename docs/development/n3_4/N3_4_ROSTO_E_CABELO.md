# N3.4 — Rosto e cabelo

Evidências: `evidence/face_study.png` e `evidence/hair_study.png`.

## Medidas (canvas 80×128)

- Cabeça: linhas 13 a 27, ou seja 15 px numa figura de 106 px (cerca de 7 cabeças).
- Pele visível do rosto: linhas 17 a 28, colunas 42 a 48. O perfil chega à coluna 48, na ponta do nariz (linha 22).
- Cabelo: linhas 13 a 48 e colunas 25 a 43. Cobre um terço da altura, da coroa ao meio das costas.

## Perfil, linha por linha (de cima para baixo)

| Linhas | O que está desenhado |
|---|---|
| 17–18 | testa, com a franja por cima |
| 19 | sobrancelha (2 px `e`) e arco superciliar |
| 20 | olho (1 px `e`) recuado do perfil |
| 21–22 | ponte do nariz e ponta, 1 px à frente da testa |
| 23–24 | sob o nariz e lábio superior |
| 25 | linha da boca (`z`) recuada |
| 26–27 | queixo firme e base do queixo |
| 28 | sombra sob o maxilar, passando ao pescoço |

A linha do maxilar é feita com `z` nas colunas 42 e 43 (linhas 25 a 27). A mecha que cai à frente da orelha usa `w` na coluna 40 e `h` na coluna 41. A sombra dessa mecha encosta na pele e separa cabelo de rosto.

## Iterações (todas vistas em close antes da seguinte)

1. **v1:** o cabelo virou um capuz vertical e o rosto ficou numa lasca de 3 px na borda. Rejeitada.
2. **v2:** a massa de cabelo passou a cair em diagonal, da nuca até a coluna 25 na linha 39. O rosto foi empurrado 1 px para a frente. As listras alternadas pareciam "bengala doce".
3. **v3:** as mechas foram agrupadas em massas de `w` e `W`, com poucas linhas `h`. A pele do rosto foi alargada para começar na coluna 42.
4. **v4:** a mecha frontal foi invertida (claro por fora, sombra encostando na pele). Os dois `z` soltos na bochecha, que pareciam barba, foram removidos.

## Resultado honesto

- Em close (4× e 8×), o perfil lê como um rosto masculino sério, com nariz, boca e queixo. É uma adaptação, não um retrato. As maçãs do rosto e a expressão da ficha não cabem em 7 px de largura.
- Em 1× e na escala do jogo, o rosto vira 3 ou 4 manchas de pele com um ponto escuro. A identidade, nessa escala, vem do cabelo, não do rosto.
- O cabelo funciona em fundo claro, neutro e escuro. Em fundo claro, o contorno `H` azulado impede que ele se dissolva.
- Ainda parece um pouco "capacete" no alto da cabeça: a coroa (linhas 13–16) é uma cúpula lisa, sem fios soltos. É a correção mais óbvia para uma próxima passada.
