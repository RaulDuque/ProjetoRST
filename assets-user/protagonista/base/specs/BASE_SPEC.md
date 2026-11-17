# Especificação — protagonista-base (sem equipamentos removíveis)

Status: **especificação + piloto parcial**. Nenhuma nova identidade oficial foi
produzida nesta fase.

## Objetivo D15

Corpo humanoide reutilizável, neutro para rigging, sem peças removíveis
(espada, escudo, capa destacável, armadura, botas de combate ornamentadas,
acessórios de cabeça).

## Identidade canônica (não alterar sem autorização)

Fonte: `docs/ART_DIRECTION.md` + modelo `swordsman_rig_v007`.

- Cabelo curto cinza-ardósia, franja à frente.
- Rosto estreito, acento âmber no olho.
- Silhueta esguia (~7 cabeças, ~1,82 m no Blender).

## Conflito registrado (decisão pendente)

| Fonte | O que mostra |
|---|---|
| Conceito B / v007 / D7-B | Casaca longa, meia-capa vinho, botas altas e sabre fazem parte da **marca visual** |
| Pedido D15 “base limpa” | Roupas básicas simples; capa/botas de combate/armadura como peças modulares |

**Não foi gerada** uma versão oficial “só roupa íntima/túnica neutra”, para não
alterar cabelo/rosto/identidade nem criar um segundo protagonista não aprovado.

### Decisão solicitada ao proprietário

1. **A — Base vestida atual (interina):** corpo v007 **sem espada** (já existe
   pipeline V2.2 weaponless). Casaca/capa/botas continuam no corpo até haver
   arte modular de roupa.
2. **B — Base underwear/túnica neutra:** nova produção 3D preservando só
   cabeça/cabelo/proporções; casaca, capa e botas viram `equip/*`.

Até a decisão, o piloto D15 usa a opção **A**.

## Vistas necessárias (contrato)

Todas devem ser o **mesmo** personagem-base escolhido.

| Vista | Pasta | Estado atual |
|---|---|---|
| Frente | `protagonista/frente/` | Cópias v007 + v002 (personagem **vestido com casaca/capa**, com ou sem sabre nos masters) |
| Costas | `protagonista/costas/` | Idem |
| Lateral (esq./dir.) | `protagonista/lateral/` | Master lateral v007/v002; **não** há par L/R separado verificado |
| Pose neutra | `protagonista/poses/` | **Pendente** — extrair de v007 rest pose |
| Pose de rigging (A-pose) | `protagonista/poses/` | **Pendente** — braços mais abertos que o rest v007 |

Não inventar vistas “aprovadas” sem arquivo. O que falta está marcado pendente.

## Requisitos de pose (quando produzidas)

- Braços ligeiramente afastados do tronco.
- Mãos visíveis.
- Pés separados, sola em z = 0.
- Sem anatomia explícita; roupa neutra cobrindo o torso.

## Fonte 3D interina

- Blend: `art-src/player/blender/swordsman_rig_v007.blend` (somente leitura)
- Ponteiro: `protagonista/base/modelo/SOURCE.txt`
- Piloto: `art-src/visual-prototypes/d15_equip_modular/pilot/`
