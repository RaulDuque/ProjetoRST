# Contrato de atlas — D11 experimental

Espelho legível de `assets/experimental/d11/contract.json`.

| Campo | Valor |
|---|---|
| cell_px | 128 × 160 |
| pivot_sole_px | (64, 156) |
| game_height_px / world_height | 52 |
| filter | nearest (por `Image`) |
| loadouts | classico, pesado |
| idle | 6 frames @ 10 fps |
| run | 8 frames @ 12 fps |

## Slots visuais (arte) vs funcionais (jogo)

Visuais do lab: weapon, offhand, head, body, gloves, boots, cape, scabbard.  
Funcionais inalterados: Weapon, Offhand, Head, Body, Gloves, Boots, Accessory1, Accessory2.

Capa/scabbard no lab mapeiam para camadas de apresentação; **não** criam slots de inventário novos.
