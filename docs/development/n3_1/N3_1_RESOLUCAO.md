# N3.1 — Resolução

A câmera do jogo não muda: `view_height` 405, janela 1280×720. `camera.ron` não foi editado.

Nesta prova isolada a vista próxima usa altura 360. Na janela de 720 px isso é escala inteira 2×, e a largura visível é 640.

| Leitura | Célula | Altura opaca da figura |
|---|---|---|
| Contrato, equivalente ao recorte do jogo | 128×96 | 49 texels |
| Apresentação vizinha 2× | 256×192 | 102 texels |

102 texels cabem na faixa de 80–110 pedida para a vista próxima. 49 texels é o que a célula de contrato do combate realmente guarda. Não são a mesma medida.

No exemplo, a vista próxima desenha a célula de apresentação em escala 1. A vista de jogo usa escala 0,5 na altura 405. Na captura de 1280×720, a caixa do corpo (sem a lâmina branca e sem o texto) mede 223 px na vista próxima e 99 px na vista 405. São `bevy_attack_pixel.png` e `bevy_attack_game_frame.png`. Essa caixa inclui o braço erguido e a capa; não é a altura de 102 texels do atlas.

O filtro da prova é o nearest já usado pelo app. O atlas automático mede 1792×192.
