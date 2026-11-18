# N3.6 — Resolução e canvas

| Medida | Valor | Origem |
|---|---|---|
| Altura do corpo de referência | 106 texels | quadro 0 do vídeo (figura de 450 px) → escala 0,23661, a mesma da N3.5 |
| Escala dos quadros de corrida | a mesma 0,23661 em todos | nenhum quadro foi ampliado ou reduzido para caber |
| Altura dos quadros de corrida | 101–105 texels | o corpo se inclina e encolhe na corrida |
| Largura dos quadros | 22 (de pé) a 85 texels (passada) | `metrics.json` |
| Canvas comum | **112×128** | 85 texels de passada não cabem em 80 |
| Pivô | (56, 118) | ver `N3_6_ALINHAMENTO.md` |
| Densidade | 2 texels/unidade | figura de 53 unidades, como a N3.5 |
| Paleta | N3.4 sem `M` e `e` | igual à N3.5 |

O canvas da N3.5 (80×128) foi mantido para o personagem parado. A corrida usa outro canvas, mais largo, com a **mesma** escala, o mesmo sistema de coordenadas (sola na linha 118) e a mesma altura de corpo. Só a largura muda: 112 dá folga de cerca de 9 texels de cada lado na passada mais aberta.

## Apresentação

Os números da N3.5 continuam válidos:

- **Câmera 360:** 1 px de tela por texel. As capturas dos 6 quadros do `run6` são **idênticas** aos sprites, com 0 diferenças em 2 219 a 2 528 texels opacos por quadro.
- **Câmera 405:** 0,889 px de tela por texel. O quadro 2 (passada) cai de 85×101 para 76×89 px de tela (`run_405.png`).
