# N3.4 — Resolução

## Contrato mantido

Canvas 80×128, altura opaca 106 px (linhas 13 a 118), sola na linha 118 e pivô `(40, 118)`, como na N3.3. Nada foi alterado.

A sola real vai da coluna 31 à 47 (salto 31–35, arco 36–37, pé 38–47). O centro do apoio é a coluna 39, 1 px atrás do pivô. Isso é aceitável para uma pose estática e fica registrado.

## A identidade cabe em 80×128?

Cabe para a silhueta, o cabelo, a roupa e um perfil esquemático. O rosto tem 7 px de largura e 12 de altura: há nariz, boca e queixo, mas não maçã do rosto, olhar nem expressão. Uma resolução intermediária, por exemplo 96×160 com figura de cerca de 132 px, daria cerca de 9 px de rosto. Ela foi considerada e **não foi feita**, pelo motivo da seção seguinte.

## O problema real está na escala do jogo

| Grandeza | Valor | Fonte |
|---|---|---|
| Altura visível da câmera do jogo | 405 unidades | `assets/data/camera.ron` |
| Janela | 720 px | 1,78 px de tela por unidade |
| Altura de colisão do jogador | 46 unidades | `PLAYER_SIZE` em `src/core/constants.rs` |
| Densidade dos sprites publicados | 2 texels por unidade | manifestos `density: 2` |

Com densidade 2, os 106 px viram 53 unidades, perto dos 46 da colisão. Na tela, isso dá 53 × 1,78 ≈ **94 px de tela para 106 texels**, ou seja 0,89 px por texel. O sprite é **reduzido** na tela: com amostragem nearest, cerca de 1 linha e 1 coluna em cada 9 some. O olho, a boca e a fivela são justamente pixels isolados que podem desaparecer. A captura `evidence/bevy_n34_game.png` mostra esse enquadramento (o exemplo isolado desenha em 0,5×).

Desenhar maior (96×160) pioraria essa redução. Desenhar menor (cerca de 53 px, densidade 1) daria 1,78 px de tela por texel, uma escala não inteira que deixa os pixels desiguais, e o rosto não passaria de 4 px.

## Proposta (não aprovada, não aplicada)

Com `view_height` de **360** em vez de 405, a janela de 720 fica com 2 px por unidade. Com densidade 2, cada texel vira **exatamente 1 px de tela**, e os 106 px aparecem inteiros, sem perda. É a única combinação que preserva o rosto desenhado sem redesenhar. A alternativa é manter 405 e aceitar a perda, ou ajustar a densidade do jogador para 1,78.

Mudar `camera.ron` é proibido nesta etapa e afeta o enquadramento do jogo todo. Fica como decisão do proprietário.
