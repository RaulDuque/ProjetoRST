# 18 — Criar NPCs

> Situação: **funciona**. **Pendente:** NPCs ainda são retângulos coloridos
> (sem arte nem animação) e não andam.

## 1. O que vamos fazer?

Criar um NPC (personagem que não é o jogador) numa sala: **Morrow, o
Guardião da Cripta**, que conversa e vende coisas.

## 2. O que você precisa saber antes?

- NPCs ficam em `assets/data/npcs/*.ron` (uma lista `[ ... ]`).
- Cada NPC está **numa sala** (`room`) e numa **posição** (`position`, o
  centro do corpo). No chão: `y = 40 + metade da altura` (altura 46 → `y = 63`).
- O jogador conversa chegando perto (até 56 de distância) e apertando **D**.
  Aparece "Nome / D Talk" sobre o NPC. Se um baú ou alavanca também estiver
  ao alcance, o **objeto** tem prioridade.
- Durante a conversa e a loja o jogo fica pausado (sem combate).
- `kind`: `Villager`, `Merchant`, `Smith`, `Informant` ou `QuestGiver`.
- `greetings`: qual fala começa a conversa (tutorial 19). A **última** não pode
  ter condições (é a fala padrão).
- `shop: Some("id_da_loja")`: só para quem vende (tutorial 20).

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/data/npcs/ashen_crypt.ron` | **Novo.** O NPC |
| `assets/data/dialogues/ashen_crypt.ron` | **Novo.** As falas dele (tutorial 19) |
| `assets/data/shops/ashen_crypt.ron` | **Novo.** A loja dele (tutorial 20) |
| `assets/data/npcs/ash_village.ron` | Existe. Os 5 NPCs da vila, como modelo |

## 4. Passo a passo

1. Crie `assets/data/npcs/ashen_crypt.ron` (seção 5).
2. Escolha um `id` único e uma sala que existe (`ac_crypt_stair`).
3. Posição: x livre (longe de baús e alavancas), y = 63.
4. Escreva as falas (19) e, se vender, a loja (20).
5. Teste.

## 5. Exemplo completo

```ron
// exemplo-verificado: Npcs
// NPC da região de exemplo Ashen Crypt.
[
    (
        id: "morrow_crypt_keeper",
        name: "Morrow, the Crypt Keeper",
        kind: Merchant,
        room: "ac_crypt_stair",
        position: (560.0, 63.0),
        size: (26.0, 46.0),
        color: (0.45, 0.42, 0.5),
        greetings: [
            (when: [Ability(WallBreaker)], node: "morrow_relic"),
            (node: "morrow_greet"),
        ],
        shop: Some("morrow_wares"),
    ),
]
```

| Campo | Significado |
|---|---|
| `id` | Único no jogo |
| `name` | Nome na conversa e sobre a cabeça |
| `room`, `position` | Onde fica |
| `size`, `color` | Tamanho e cor do retângulo |
| `greetings` | Fala inicial conforme o progresso (a primeira que valer) |
| `shop` | Loja que ele administra |

## 6. Como testar?

1. `cargo test -j 2 --test data_tests` e `cargo run -j 2 --example world_report`.
2. `cargo run -j 2`: chegue perto → "D Talk" → aperte D.
3. `cargo test -j 2 --test tutorial_examples_tests` (o teste conversa com ele).

## 7. Como editar depois?

Mude posição, nome ou cor. Para trocar a sala, mude `room` e a posição.

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `NPC ... is in unknown room` | Sala não existe | Corrija `room` |
| `NPC ... stands outside room` | Posição fora das paredes | Ajuste `position` |
| `no greeting without conditions` | Falta a fala padrão | Última `greetings` sem `when` |
| `unknown dialogue node` | `node` não existe | Crie a fala (19) |
| `NPC ... and shop ... do not match` | Loja e NPC não se apontam | `shop` do NPC = `id` da loja; `keeper` da loja = `id` do NPC |
| D abre o baú em vez de conversar | Baú ao alcance | Normal; abra o baú, depois converse, ou afaste o NPC |

## 9. Como desfazer com segurança?

Apague o arquivo do NPC **e** as falas e a loja dele (senão o jogo acusa
`unknown speaker` ou `do not match`).
