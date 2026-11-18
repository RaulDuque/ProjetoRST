# N3 — Animações

Quatro clipes, no manifesto `art-src/n3-proof/generated/proof.ron`. O parser do jogo aceita o arquivo. `weapon_in_body` é `false`.

| Clipe | Quadros | Modo | O que muda |
|---|---|---|---|
| idle | 4 | loop, 6 fps | o peito sobe e a capa balança; o pivô fica no chão |
| walk | 6 | loop, 10 fps | as pernas trocam de lado e o braço da arma acompanha |
| jump | 6 | uma vez, 12 fps | agacha, sobe com o braço para cima, fica no ar e cai |
| attack | 6 | uma vez, 14 fps | preparo com a arma atrás, golpe à frente, volta |

Não existe clipe de corrida separado. O walk não é um recorte do `run` publicado. O pulo não é pulo duplo e não entra na física do jogo.

A sequência de walk capturada pela janela está em `bevy_walk_00.png` … `bevy_walk_05.png`. Um quadro parado não é a prova de fluidez. A fluidez ainda é curta: são seis poses, não um ciclo polido.
