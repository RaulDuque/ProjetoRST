# N3.4 — Identidade visual

Guerreiro gótico aristocrático, de cabelo longo branco-prateado e expressão séria. A base fica sem equipamentos removíveis.

## O que o sprite precisa manter, em ordem

1. A massa de cabelo branco-prateado, longa, caindo em diagonal pelas costas. É o traço que identifica o personagem de longe.
2. Um perfil masculino legível: testa, sobrancelha, olho, nariz, boca e queixo.
3. A camisa cinza de manga dobrada, em contraste com a calça marrom-escura.
4. O cinto, os braceletes e as botas de cano alto com aba dobrada.
5. Uma silhueta esguia, sem trapézio de casaco e sem bloco de capa.

## O que foi evitado

Capuz, cabelo curto, cabelo escuro (o caçador da N3.2), formas de bloco, casaco, capa e espada no corpo-base.

## Paleta (19 cores, em `art-src/n3-4-original-protagonist/scripts/n34_palette.py`)

| Grupo | Chaves | Uso |
|---|---|---|
| Contorno | `K` | contorno escuro de corpo, calça e bota |
| Cabelo | `W w h H` | luz, meio-tom, sombra fria e contorno azulado |
| Pele | `S s z` | luz, meio-tom, sombra e contorno do perfil |
| Olho e sobrancelha | `e` | colocado só à mão |
| Camisa | `G g q` | cinza claro, médio e sombra |
| Calça | `P p o` | marrom-escuro: luz, médio e sombra |
| Couro | `L l d` | cinto, bracelete e botas |
| Metal | `M` | fivela, rebite e fivela da bota; colocado só à mão |

O cabelo não usa branco puro. A luz é (238, 238, 244) e as sombras puxam para o azul, (156, 160, 180) e (104, 108, 130). Assim ele se separa da pele quente e funciona em fundo claro e escuro (`evidence/hair_study.png`).
