# N3.5 — Câmera e apresentação

Evidências: `resolution_matrix.png` (simulação), `camera_405.png` e `camera_360.png` (recortes das capturas reais do Bevy), `bevy_n35_405.png`, `bevy_n35_360.png`, `bevy_n34_405.png` e `bevy_n34_360.png`.

## Regra mantida em todos os testes

O personagem tem sempre **53 unidades de mundo** de altura (106 texels a densidade 2, como a N3.4; a colisão tem 46). Muda apenas quantos texels a arte tem e quantas unidades a câmera mostra. A janela é de 720 px de altura. `assets/data/camera.ron` não foi tocado.

## Matriz

| Arte | Câmera | Texels/unidade | Px de tela por texel | Altura na tela | Amostragem |
|---|---|---|---|---|---|
| 80×128 (106) | 405 | 2,00 | 0,889 | 94,2 px | reduz: cerca de 1 em 9 linhas e colunas some |
| 80×128 (106) | **360** | 2,00 | **1,000** | 106 px | **inteira, sem perda** |
| 96×160 (132) | 405 | 2,49 | 0,714 | 94,2 px | reduz: 2 em 7 somem |
| 96×160 (132) | 360 | 2,49 | 0,803 | 106 px | reduz: 1 em 5 some |
| 72×112 (94) | **405** | 1,77 | **1,002** | 94,2 px | **inteira** (com densidade não inteira) |
| 72×112 (94) | 360 | 1,77 | 1,128 | 106 px | amplia de forma desigual: 1 em 8 duplica |

A combinação "resolução derivada × 360" é igual a "80×128 × 360", porque a resolução medida da `reducao` é 106.

## Capturas reais do Bevy (`n3_visual_proof`, amostragem nearest)

- **Câmera 360, densidade 2:** a figura ocupa 23×106 px na janela. Comparada pixel a pixel com `candidate_106.png`, há **0 diferenças em 1 320 texels opacos**. É exatamente o desenho.
- **Câmera 405, densidade 2:** a figura vira 20×94 px. Somem 12 linhas e 3 colunas. Na captura, o olho e a boca afinam e as tiras da bota ficam irregulares.

## Cenário 3: densidade alternativa

A arte de 94 px na câmera 405 dá 1:1 sem mexer na câmera, mas exige densidade 1,777 texel por unidade. No manifesto, `density` é inteiro (`u32`, em `src/player/visual/manifest.rs`), então esse cenário não é válido no formato atual sem uma mudança de código. Ele aparece só na simulação e não foi capturado no Bevy. Além disso, nessa resolução o olho e a boca quase se fundem.

## Conclusão

Mais resolução de arte só aparece na tela se o personagem também ficar maior na tela. Com o tamanho de mundo atual, a apresentação fiel é **80×128 com a câmera de 360**. É a única combinação inteira com o formato de densidade existente. Mudar a câmera do jogo é decisão do proprietário. Nesta etapa, isso existe só no exemplo isolado.
