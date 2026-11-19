# N4-A — Composição e remoção das peças

## Peças realmente independentes

| Peça | Camadas | Removível | Folha |
|---|---|---|---|
| corpo-base | `body`, `arm_front` | não | `parts/idle/body.png`, `arm_front.png` |
| cabelo | `hair_back`, `hair_front` | sim | `hair_back.png`, `hair_front.png` |
| camisa | `shirt` | sim | `shirt.png` |
| calça | `trousers` | sim | `trousers.png` |
| botas | `boots` | sim | `boots.png` |
| braceletes | `bracers` | sim | `bracers.png` |

Cada peça é uma imagem própria no canvas do corpo. Remover uma peça é tirá-la do `Outfit`: nenhuma imagem completa de personagem é trocada. O teste `each_piece_is_independent_and_leaves_no_hole` remove cada peça sozinha e verifica que **só os texels das camadas dessa peça** mudam.

## Demonstração

- Folhas offline (texels exatos): `evidence/n4a_removal.png` (fundo escuro) e `n4a_removal_light.png` (fundo claro).
- No Bevy real (`cargo run --example n4_sprite_machine`, 4 px por texel, *nearest*):

| Captura | O que mostra |
|---|---|
| `bevy_n4a_original_vs_modular.png` | original à esquerda, modular completo à direita |
| `bevy_n4a_no_hair.png` | sem cabelo |
| `bevy_n4a_no_shirt.png` | sem camisa |
| `bevy_n4a_no_trousers.png` | sem calça |
| `bevy_n4a_no_boots.png` | sem botas |
| `bevy_n4a_no_bracers.png` | sem braceletes |
| `bevy_n4a_base_body.png` | corpo-base |
| `bevy_n4a_light_background.png` | fundo claro |
| `bevy_n4a_mirrored.png` | os dois espelhados (virados para a esquerda) |
| `bevy_n4a_grid_pivot.png` | grade a cada 8 texels, canvas e pivô |
| `bevy_n4a_base_body_light_grid.png` | corpo-base, fundo claro, grade |

O roteiro confere, em cada captura, quantas camadas estão visíveis (8 com a roupa completa, 6 sem cabelo, 7 sem outra peça, 2 no corpo-base) e termina com `RESULT: PASS`.

No modo interativo (`N4_INTERACTIVE=1`): teclas 1–5 tiram e vestem cada peça, 0 deixa o corpo-base, 9 volta à roupa, O mostra ou esconde o original, G mostra a grade e o pivô, K troca o fundo, M espelha e Espaço pausa.

## Sincronização

Todas as camadas leem o `VisualPlayback` da raiz. No espelho, a raiz só muda `flip`, e as 8 camadas se espelham em volta do mesmo pivô (a captura espelhada é idêntica ao original espelhado, `N4A_DIFERENCAS_VISUAIS.md`). Com um clipe de 6 quadros (dados de teste, canvas 112×128), as 3 camadas mostram o mesmo índice em todas as combinações de quadro e direção testadas.
