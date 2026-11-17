# Contrato de coordenadas — D12

| Campo | Valor |
|---|---|
| Célula | 128 × 160 px |
| Pivô sola | (64, 156) topo-esquerda |
| World size | **51.2 × 64.0** (altura alinhada ao locomotion publicado) |
| Anchor Bevy | `ax = px/cw − 0.5`, `ay = 0.5 − py/ch` → (0.0, −0.475) |
| Filtro | nearest **por imagem** |
| Câmera lab | `FixedVertical` viewport_height **405** (igual jogo; não alterada no projeto) |
| Janela lab | 1280 × 720 |

## Equipamentos

Slots funcionais inalterados. Peças visuais do bake D10/D12 compartilham a mesma célula e pivô; não há escala por peça no runtime experimental (composição pré-bakeada).

## D11 → D12

| | D11 | D12 |
|---|---|---|
| world_h | 52 | **64** |
| Anchor | centro (bug) | sola |
| assets | `experimental/d11` | `experimental/d12` |
