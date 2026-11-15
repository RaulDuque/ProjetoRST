# 26 — Criar a figura (sprite) de um NPC

> Situação: **funciona**. As figuras são **provisórias, geradas em código**
> a partir de dados (roupa, chapéu, objeto, cores), não desenhos à mão.
> **Pendente:** animação dos NPCs e arte final; imagens PNG próprias para NPCs
> ainda não são lidas pelo jogo.

## 1. O que vamos fazer?

Dar ao NPC **Morrow, o Guardião da Cripta** (tutorial 18) uma figura
reconhecível: robe, capuz e cajado com lanterna.

## 2. O que você precisa saber antes?

- A figura nasce do campo opcional `look` do NPC, com o tamanho (`size`) e a
  cor do corpo (`color`) que o NPC já tinha.
- Sem `look`, o NPC vira uma figura simples da cor `color`.
- Opções:

| Campo | Valores |
|---|---|
| `outfit` | `Tunic`, `Tabard` (guarda), `Robe` (peregrino), `Dress`, `Apron` (ferreiro), `Shawl` (xale) |
| `headwear` | `Bare` (cabelo), `Helm`, `Hood`, `Kerchief` (lenço), `GreyBun` (coque grisalho). Padrão: `Bare` |
| `prop` | `Some(Spear)`, `Some(LanternStaff)`, `Some(Satchel)`, `Some(Hammer)`, `Some(Cane)` ou `None` (padrão) |
| `accent` | Segunda cor (tabardo, avental, xale, capuz, lenço), de 0.0 a 1.0 |

- A posição, o alcance do **D** e o aviso "D Talk" **não mudam**: a figura é
  centrada no mesmo ponto do retângulo antigo.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/data/npcs/ashen_crypt.ron` | O NPC do tutorial 18 (ou qualquer arquivo de NPCs) |
| `assets/data/npcs/ash_village.ron` | Os 5 NPCs da vila, já com `look`, como modelo |
| `src/npc/art.rs` | Só leitura: como cada opção é desenhada |

## 4. Passo a passo

1. Abra o arquivo do NPC.
2. Antes de `greetings`, acrescente `look: Some((...))` (seção 5).
3. Escolha roupa, chapéu e objeto que contem o papel dele.
4. Teste (seção 6).

## 5. Exemplo completo

Só a aparência:

```ron
// exemplo-verificado: AparenciaNpc
(outfit: Robe, headwear: Hood, prop: Some(LanternStaff), accent: (0.3, 0.28, 0.36))
```

O NPC inteiro com a aparência:

```ron
// exemplo-verificado: Npc
(
    id: "morrow_crypt_keeper",
    name: "Morrow, the Crypt Keeper",
    kind: Merchant,
    room: "ac_crypt_stair",
    position: (560.0, 63.0),
    size: (26.0, 46.0),
    color: (0.45, 0.42, 0.5),
    look: Some((outfit: Robe, headwear: Hood, prop: Some(LanternStaff), accent: (0.3, 0.28, 0.36))),
    greetings: [
        (when: [Ability(WallBreaker)], node: "morrow_relic"),
        (node: "morrow_greet"),
    ],
    shop: Some("morrow_wares"),
),
```

## 6. Como testar?

1. `cargo test -j 2 --test data_tests` e `cargo test -j 2 --test npc_commerce_tests`.
2. `cargo run -j 2 --example equipment_visual_showcase`: a captura
   `s25_npc_lineup_presentation_only.png` mostra **todos** os NPCs dos dados
   lado a lado (só apresentação).
3. `cargo run -j 2` e vá até a sala do NPC.

## 7. Como editar depois?

Troque `outfit`, `headwear`, `prop` ou as cores. Para voltar ao visual
simples, apague a linha `look`.

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `look accent outside 0..1` | Cor com valor fora de 0–1 | Corrija `accent` |
| `unknown variant` | Nome de roupa/chapéu/objeto errado | Use exatamente os nomes da tabela |
| `unknown field` | Campo escrito errado dentro de `look` | Só `outfit`, `headwear`, `prop`, `accent` |

## 9. Como desfazer com segurança?

Apague a linha `look: Some((...)),`. O NPC continua funcionando igual.
