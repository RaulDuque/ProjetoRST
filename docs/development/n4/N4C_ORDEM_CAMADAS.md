# N4-1 (C1) — Ordem das camadas

A ordem completa (14 camadas, de trás para a frente) está em `N4C_ESTRATEGIA_SEPARACAO.md` e em `stage/protagonist.ron`.

## Verificação por quadro

| Quadro | Cruzamento | Como a ordem resolve |
|---|---|---|
| #0 | braço da câmera à frente do tronco; braço de trás escondido atrás do tronco | `arm_front` (9) > `shirt_torso` (8); braço de trás em `body_back` (0) e `sleeve_back` (3), sob o tronco (5, 8) |
| #0 | perna de trás dobrada cruza a perna de apoio | a perna perto (dobrada) fica em `body` / `trousers` / `boots` (5–7); a de apoio (longe) em `*_back` (0–2) |
| #2 | braço da câmera sobe à frente do peito; braço de trás aparece atrás do tronco | igual a #0; o antebraço e a mão de trás ficam visíveis porque saem da silhueta do tronco |
| #2 | pernas abertas: só se encostam no quadril | perna perto (atrás) na frente da perna longe (à frente), contralateral ao braço |
| #4 | braço da câmera balança para trás **cruzando o tronco** | continua `arm_front` sobre `shirt_torso`: o braço perto está sempre na frente, para onde quer que balance |
| #4 | braço de trás à frente, punho na frente do peito | `body_back` / `sleeve_back`: o punho aparece onde sai da silhueta do tronco |

O que se vê vestido é a partição do original, então a ordem não altera a imagem vestida (0 diferenças). A ordem decide o que aparece **quando uma peça sai**, e isso é conferido nas remoções (`n4c_removal.png`): sem camisa, o braço da câmera continua sobre o tronco nu; sem calça, a perna perto fica sobre a perna longe.

## Nenhum membro foi escondido para resolver um cruzamento

Os membros de trás têm anatomia própria (`body_back`, 487 a 501 texels pintados por quadro). Eles só ficam encobertos onde o original os mostra encobertos.

## Ordem por quadro

O contrato aceita exceções por clipe e quadro (`order_overrides`, N4-A, testadas). Na C1 não foi preciso usar, porque perto e longe são sempre os mesmos membros físicos. A C2 deve conferir os quadros #1, #3 e #5, em especial a troca do braço da câmera de frente para trás.
