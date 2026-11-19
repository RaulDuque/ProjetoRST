# N4-1-A — Equipamentos A e B e regras de composição

## Família `torso_plate`

| Compartilhado | Valor |
|---|---|
| Slot | `armor_torso`, entre a camisa do tronco e o braço da frente |
| Perfil | `idle` e `run_c1` (3 quadros C1) |
| Silhueta (máscara) | **corrida:** o tronco inteiro de cada pose (polígono do tronco de `run_poses.py` recortado pela silhueta vestida, acima do cinto). **Idle:** a camisa vestida sem a manga da frente |
| Decote | recorte elíptico em volta do pescoço de cada pose; um item com gorjal o preenche |
| HIDE padrão | `run_c1: [shirt_torso]` |
| Ordem | no idle, acima da camisa (`frame_order idle z = 12`; o desempate pela ordem declarada mantém o peitoral sob os braceletes e o cabelo) |
| Clipe ausente | `Hide` |

As máscaras são geradas **uma vez por família** (`stage/equipment/families/torso_plate/{idle,run_c1}_mask.png`) a partir da arte aprovada vestida. Nenhum corpo nu é usado.

## Item A — `iron_cuirass` (couraça de ferro com gorjal)

- Receita `equipment/recipes/iron_cuirass.json`: paleta de ferro (5 tons + contorno do protagonista), altura total, gorjal, lâminas a partir de 62 % da altura a cada 3 linhas, um rebite na ponta de trás de cada lâmina.
- Manifesto `stage/equipment/items/iron_cuirass.ron`: herda o HIDE da família.
- **OVERLAY + HIDE:** na corrida, cobre 100 % da camisa do tronco (0 texel exposto), então a oculta. As mangas e o braço da frente continuam por cima. No idle não oculta nada: a camisa do idle inclui a manga, e ocultá-la exporia 143 texels.

## Item B — `blackened_breastplate` (peitoral escurecido com friso dourado)

- Receita `equipment/recipes/blackened_breastplate.json`: paleta escura + 3 tons de dourado, **72 % da altura** (mais curto), sem gorjal (usa o decote da família), aresta central, friso contínuo no topo e na base.
- Manifesto `stage/equipment/items/blackened_breastplate.ron`: `hide_layers: Some({})`, ou seja, **não oculta nada**.
- **Só OVERLAY:** como é mais curto e tem decote, a camisa aparece abaixo dele e na gola (é roupa por baixo). Se ocultasse a camisa do tronco, exporia 32, 87 e 68 texels na corrida (`coverage.json` e `n41a_run_c1_hide_check.png`). Por isso o manifesto declara a ocultação vazia, e o verificador recusa a combinação errada.

## Regras de composição

| Modo | Como funciona | Garantia |
|---|---|---|
| OVERLAY | o item é desenhado no slot da família, na mesma célula, pivô e quadro do corpo | o item só existe dentro da silhueta vestida (teste "floats") |
| HIDE | `hide_layers[clip]` tira camadas da base enquanto o item está vestido e visível | o verificador e o teste exigem que **todo texel oculto esteja sob o item**: nada do que fica por baixo da camisa aparece, e nenhum buraco se abre |
| REPLACE | `replaces: [part]` tira uma peça inteira da base (botas novas no lugar das botas) | implementado e testado com dados de teste; não foi preciso nos peitorais |

**Sem camisa atravessando a armadura:** o slot fica acima da camisa do tronco. As únicas camadas acima do peitoral são o braço da frente e a manga dele, que estão de fato na frente do peito no perfil, além de braceletes e cabelo. No idle, a exceção de ordem põe o peitoral acima da camisa inteira, e a máscara exclui a manga. Teste: em todo texel do item, o que aparece é o item ou uma dessas camadas.

## Como criar um equipamento novo da mesma família

1. Escrever a receita `art-src/n4-sprite-machine/equipment/recipes/<id>.json` (paleta, altura, gorjal, lâminas, aresta, friso), ou desenhar à mão as folhas sobre as máscaras da família.
2. Escrever `stage/equipment/items/<id>.ron` (id, família, folhas, HIDE se for o caso).
3. Rodar `python3 -B art-src/n4-sprite-machine/scripts/build_equipment.py`: gera as folhas, confere a cobertura (recusa HIDE inseguro) e gera evidências, folha composta e GIF.
4. Rodar `cargo test -j 2 --test n41a_equipment_tests` e olhar no `cargo run --example n4_sprite_machine -- --equipment [--run-c1]` (tecla E).

Nenhum código Rust muda. O teste `a_third_item_of_the_family_is_data_only` mostra um terceiro item carregado só de um RON.

Família nova (capacete, luvas, botas, capa) = um RON de família + as regras de máscara no JSON da família. Pode exigir uma regra de máscara nova no gerador (por exemplo, "região da cabeça"), mas não no jogo.
