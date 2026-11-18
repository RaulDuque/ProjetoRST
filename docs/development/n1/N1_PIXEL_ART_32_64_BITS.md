# N1 — Pixel art e escala

Medidas de hoje, lidas do código:

- Janela: 1280×720 (`WINDOW_WIDTH`, `WINDOW_HEIGHT`).
- Câmera: altura visível 405, aspecto 16:9, então a largura visível é cerca de 720 (`camera_rules`, `view_height` 405, `REFERENCE_ASPECT`).
- O zoom cabe entre 240 e 720 de altura visível.
- Colisão do protagonista: 26×46.
- Atlas de locomoção: célula 128×128, `density` 2, então 64×64 unidades de mundo. Combate: 256×192 com density 2, 128×96 no mundo.
- Filtro nearest já vale para a arte gerada em `pixel_image`. A câmera segue com subpixel (`follow_rate`, look-ahead 70).

"32/64 bits" não fixa sprite, tela nem quantidade de quadros. O protagonista não fica preso a 64×64.

## Duas configurações

| | A — 640×360 | B — 960×540 |
|---|---|---|
| Escala inteira até a janela atual | 2× cabe em 1280×720 | 1280/960 = 1,333, não é inteiro |
| Escala inteira folgada | 2× | 2× só numa janela 1920×1080 |
| Mundo visível se 1 texel = 1 unidade | 640×360, mais fechado que os 720×405 atuais | 960×540, mais aberto que hoje |
| Altura do corpo de 46 unidades | 46 texels | 46 texels |
| Custo | menos texel por cenário | cerca de 2,25× a área da opção A |
| Leitura | o corpo fica baixo se a câmera não aproximar | mais cenário, personagem ainda baixo se a unidade não mudar |

Nenhuma das duas é a câmera atual. Hoje a janela é 1280×720 e o mundo desenhado não passa por um buffer interno.

## Decisão proposta, ainda sem código

Adotar a opção A como alvo da prova de conceito: buffer interno 640×360, escala 2 com nearest, na janela que o jogo já abre. Aproximar a câmera até o protagonista ocupar algo entre 80 e 110 texels de altura. Com o corpo a 46 unidades, isso pede uma altura visível perto de 170 a 200, abaixo do mínimo atual de 240. Por isso a prova mexe no enquadramento num app isolado, não em `camera.ron`.

A opção B fica reservada se a prova A parecer pobre. Ela pede janela 1920×1080 para continuar inteira. Misturar as duas no mesmo build borra o pixel.

## Pixel perfect

- Sprite de personagem, inimigo e tile: nearest, sem rotação do corpo.
- Câmera: o follow pode continuar em float e só o resultado final encosta no texel, quando o buffer interno existir. Antes disso, snap nenhum.
- Zoom do gameplay: só escalas inteiras do buffer. O zoom contínuo da câmera atual não entra na prova.
- Texto e HUD: ficam na resolução da janela, por cima do buffer. Não pixelizar a fonte à força.
- Luz e partícula: podem ser uma camada de resolução maior, com alpha, desde que não desenhem por cima do contorno do golpe.
- Partícula girando e sprite de corpo girando são coisas diferentes. A segunda deforma o pixel. Evitar.

## Alternativas

Continuar em 1280×720 sem buffer, desenhando o pixel já esticado. É o que o jogo faz hoje e serve ao protótipo. Não serve de alvo para a arte nova, porque o artista não sabe quantos texels o corpo ocupa na tela.

## Riscos

Baixar `view_height` no jogo real para caber na faixa de 80–110 texels quebra o mínimo de 240 e muda a leitura de todas as salas já feitas. Por isso a mudança de câmera não entra no `cargo run` nesta etapa.

## Recomendação

Aprovar a opção A só para a prova. O jogo jogável permanece em 1280×720 e altura 405 até o proprietário ver o quadro lado a lado.
