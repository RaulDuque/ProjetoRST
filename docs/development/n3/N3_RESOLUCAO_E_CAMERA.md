# N3 — Resolução e câmera

A câmera do jogo não mudou. `camera.ron` não foi editado. O `cargo run` continua com a janela 1280×720 e a vista de altura 405.

O exemplo `n3_visual_proof` abre a própria janela 1280×720, com filtro nearest (`ImagePlugin::default_nearest`). Duas vistas, as duas com pixel quadrado porque a janela é 16:9:

| Vista | Altura da câmera | Escala do sprite | O que a captura mediu |
|---|---|---|---|
| close | 360 | 1 texel = 1 unidade | `bevy_attack.png`: caixa do personagem com 193 px de altura. 720/360 = 2 exato. A largura visível é 640. |
| jogo | 405 | 1 texel = 0,5 unidade | `bevy_framing_game.png`: caixa com 86 px. 720/405 não é inteiro. |

Os 97 texels do corpo, na vista close, aparecem com cerca de 194 px na janela por causa do 2×. Na vista que imita o enquadramento do jogo, a mesma arte fica perto de 86 px. A vista close é a maior. A do jogo mostra mais chão vazio em volta.

Não foi aberta uma janela 1920×1080. 960×540 não tem escala inteira dentro de 1280×720, e esta etapa não afirma o contrário.

A tecla `V` no modo interativo troca as duas vistas. O script padrão grava as duas.
