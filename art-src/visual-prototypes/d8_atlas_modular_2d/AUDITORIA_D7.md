# Auditoria D7 (entrada da D8)

## Decisão do proprietário

Direção visual **D7-B** aprovada (com D2.game / B.game). Não voltar a D5/D6 toon.
Loadouts D7 destacados para revisão, sem aprovação peça a peça.

## De onde vêm os sprites D7-B

- Máscaras semânticas de `d2_protagonista_B/.../idle_f00.png` (head/body/wine/boots/sword).
- Stack opcional de `d4_modular_d2/camadas/*.png`.
- Scripts: `d7_pixel_programatico/scripts/draw_B_sprites_masks.py`.
- O código **não inventa** o acabamento; corta e recompõe arte-fonte.

## O que é realmente independente

| Peça | Independente? | Fonte |
|------|---------------|-------|
| Corpo/idle/run frames | Quase (sprite completo) | D2 |
| Capa D4 | Sim (camada) | D4 `01_cape_back` |
| Casaco leve D4 | Sim | D4 `02_light_coat` |
| Armadura / elmo / escudo / espada D4 | Sim | D4 `03`–`08` |
| Máscaras D2 head/wine/boots/sword | Parcial (recorte sobre o mesmo frame) | D2 |
| Luvas / botas separadas | **Não** no D7 | — |

## Chroma / máscaras

D4 ainda deixa franjas magenta/hot-pink em bordas. D7 stripou parcialmente;
D8 deve limpar no atlas sem comer forro vinho.

## Loadouts e animação D7

- Loadouts = composição de camadas (com bug histórico de scale em peças
  pequenas, corrigido depois).
- Idle 6 / run 8 = frames D2 com extras **estáticos** empilhados (capa/armadura
  não deformam com o passo).

## Pipeline reproduzível?

Sim: `python3 …/d7_pixel_programatico/scripts/run_d7.py`.

## Limitações que bloqueiam produção

1. Separação D2 ainda é por bandas/cor, não atlas artístico limpo.
2. Sem sockets 2D por quadro para anexar peças.
3. Sem movimento próprio de capa.
4. Peças faltantes (luvas, bainha, ombreiras finas) como camadas puras.
5. Equipamento pesado D4 ≠ shading idêntico ao D7-B clássico.
