# N4-1 (C1) — Estratégia de separação

## Esqueleto por pose

`art-src/n4-sprite-machine/scripts/run_poses.py` marca à mão, em texels do canvas (112×128), as articulações de cada quadro: cabeça, pescoço, ombros, quadril, polígono do tronco, e para cada braço ombro → cotovelo → pulso → punho e para cada perna quadril → joelho → tornozelo → ponta do pé, mais o calcanhar e onde o cano da bota começa ao longo da canela (`boot_t`). Foram medidas nas grades ampliadas e conferidas sobre os quadros (sobreposição em `N4C_ANATOMIA_POSES.md`).

**Perto / longe da câmera.** "Perto" é sempre o mesmo membro físico, o do lado da câmera:

| Quadro | Braço perto | Perna perto |
|---|---|---|
| #0 contato | à frente (lido no vídeo) | atrás, dobrada |
| #2 passada | à frente e para cima (vídeo) | atrás, estendida |
| #4 recuperação | para trás, cruzando o tronco (vídeo) | à frente, joelho alto |

Os braços foram lidos nos quadros em alta resolução. As pernas seguem a marcha contralateral (braço perto à frente ⇔ perna perto atrás), porque a oclusão no quadril é ambígua na redução. **Isto é interpretação.**

## Partição (nenhum texel muda)

`build_run_layers.py`, `classify`: cada texel opaco vai para **uma** camada:

1. dentro do círculo da cabeça: cabelo (chaves `WwhH`, ou não-pele atrás do rosto; `hair_front` se estiver sobre o rosto) ou rosto (`body`);
2. acima do ombro e fora do tronco: cabelo;
3. perto do braço da câmera: o braço é desenhado **sobre** o tronco;
4. dentro do polígono do tronco: camisa (`shirt_torso`), ou cinto e quadril (`trousers`) abaixo de `belt_y`, ou pele do pescoço (`body`);
5. o resto: o membro mais próximo (distância ao osso menos o raio).

O material sai da chave e da posição: pele → braço do corpo; não-pele a até 2,6 texels do pulso → bracelete; não-pele no punho → braço do corpo; resto do braço → manga. Na perna: canela além de `boot_t` e pé → bota; resto → calça. "Perto" vai para as camadas da frente, "longe" para as `*_back`.

Recompor todas as camadas devolve o quadro aprovado texel a texel, seja qual for a qualidade das bordas: é uma partição.

## Camadas da corrida

A composição ganhou 6 camadas que só existem na corrida, e a ordem relativa das 8 do idle não mudou (teste):

| z | Camada | Peça | Conteúdo |
|---|---|---|---|
| 0 | `body_back` | corpo | braço e perna de trás (pele) |
| 1 | `trousers_back` | calça | perna de trás |
| 2 | `boots_back` | botas | bota de trás |
| 3 | `sleeve_back` | camisa | manga de trás |
| 4 | `bracers_back` | braceletes | bracelete de trás |
| 5 | `body` | corpo | cabeça, tronco, quadril, perna da frente |
| 6 | `trousers` | calça | quadril e perna da frente |
| 7 | `boots` | botas | bota da frente |
| 8 | `shirt_torso` | camisa | tronco da camisa (atrás do braço da frente) |
| 9 | `arm_front` | corpo | braço da frente |
| 10 | `shirt` | camisa | manga da frente (no idle: a camisa toda) |
| 11 | `bracers` | braceletes | bracelete da frente |
| 12 | `hair_back` | cabelo | cabelo |
| 13 | `hair_front` | cabelo | mechas sobre o rosto |

Cada peça lógica continua **uma** só (por exemplo, a camisa = `sleeve_back` + `shirt_torso` + `shirt`), como o briefing pede.

## Por que camadas de frente e trás em vez de ordem por quadro

"Perto" e "longe" são sempre os mesmos membros físicos, então a profundidade não troca entre os quadros da C1. O braço da frente fica sempre sobre o tronco (camada `arm_front` acima de `shirt_torso`), mesmo quando balança para trás (#4). Por isso as exceções de ordem por quadro (`order_overrides`), que o contrato já tem, **não foram necessárias** na C1. Elas ficam para casos em que um membro passe atrás do tronco só em alguns quadros.
