# Arquitetura de equipamentos visuais (proposta)

## Distinções

| Conceito | Significado |
|---|---|
| Slot funcional | `EquipmentSlot` atual (8): Weapon, Offhand, Head, Body, Gloves, Boots, Acc1/2 |
| Slot visual | Categoria de camada renderizada (Head, Face, Torso, …) |
| Item equipado | ID no inventário |
| Camada | PNG/mesh desenhada |
| Socket | Âncora por quadro (`player_sockets.ron`) |
| Ocultação | Regra visual (elmo fecha → some cabelo) |
| Z-order | Ordem de desenho / front flags |

## Mapeamento sugerido (sem mudar Save V3)

| Visual | Funcional |
|---|---|
| MainHand | Weapon |
| OffHand | Offhand |
| Head / Face* | Head / Accessory |
| Torso + Back (capa) | Body |
| Hands | Gloves |
| Legs + Feet | Boots (hoje “Legs” na UI) |
| FX | Accessory / trail |

\*Face cosmético pode usar Accessory sem novo enum.

## Itens multi-camada

Armadura pesada = várias meshes (`d6_cuirass`, pauldrons, …) ativadas juntas,
ainda mapeáveis a Body+Head+Gloves+Boots conforme o item.
