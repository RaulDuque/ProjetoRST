# N4-1-A — Métricas de reutilização

## O que custou a família + o equipamento A (uma vez)

| Item | Quantidade |
|---|---|
| Código Rust genérico | `equipment_visual.rs` (novo), extensões em `composition.rs` e `composite.rs` (HIDE por camada, REPLACE, folha espelhada, ordem estável) |
| Dados da composição | 1 slot vazio (`armor_torso`) em `protagonist.ron` |
| Família | 1 RON (`families/torso_plate.ron`) + 1 JSON de regras de máscara |
| Máscaras | 2 tiras (idle 1 quadro, corrida 3 quadros), **geradas** da arte vestida |
| Item A | 1 receita JSON + 1 RON + 4 arquivos gerados (2 PNG + 2 TXT) |
| Ferramenta | `build_equipment.py` (pintura procedural, verificação de cobertura, evidências, exportação) |

## O que custou o equipamento B, depois de A

| Pergunta do briefing | Resposta |
|---|---|
| Arquivos criados à mão | **2**: `recipes/blackened_breastplate.json` (12 campos) e `items/blackened_breastplate.ron` (manifesto). Mais 4 gerados pela ferramenta (2 PNG + 2 TXT) |
| Quadros com arte específica | **0 desenhados à mão**. Os 4 quadros (idle + 3 da corrida) foram pintados pela ferramenta dentro das máscaras da família |
| Regras reutilizadas da família | **8**: slot, perfil de clipes, região, `covers`, máscaras (4 quadros), decote, exceção de ordem do idle, regra de clipe ausente |
| Regras sobrescritas | **1**: HIDE (`hide_layers: Some({})`), porque B é mais curto |
| Código Rust modificado | **nenhum** (o teste `a_third_item_of_the_family_is_data_only` carrega um terceiro item só de RON) |
| Corpo-base alterado | **não** |
| Máscaras novas | **nenhuma** |
| Reaproveitado | geometria da silhueta por quadro, decote, sombreamento (luz e faixa especular), contorno e sombra do braço, ordem e ocultação da família |
| Desenhado de novo | nada à mão. Mudaram só os parâmetros: paleta, altura (72 %), aresta central e friso |

## Ressalvas (honestas)

- Os padrões que B usa (friso, aresta, altura reduzida, gorjal em A) foram **acrescentados ao pintor genérico durante esta fase**, junto com A e B. Um quarto item que use as opções que já existem não precisa de código nenhum. Um padrão novo (por exemplo, gravação ornamental) precisa de código na ferramenta de autoria, e nunca no jogo.
- A pintura é **procedural**. Ela dá volume, luz e contorno coerentes, mas não substitui um pixel artist. Para qualidade final, o fluxo previsto é o artista pintar à mão sobre as máscaras da família (que já resolvem silhueta, alinhamento, ordem e ocultação), gravar o PNG no mesmo caminho e rodar o verificador. O ganho continua: o artista desenha **só a peça**, quadro a quadro, sem refazer o personagem e sem uma folha por combinação.
- Uma peça com silhueta diferente (ombreiras largas, casaco longo) precisa de máscara própria (uma família nova) ou de arte específica.
