# 07 — Criar armaduras e acessórios

> Situação: **funciona**. **Pendente:** elmos, luvas e botas ainda não aparecem
> no desenho do personagem (só mudam atributos). Capas aparecem.

## 1. O que vamos fazer?

Criar uma capa (**Gravecloth Mantle**) e um amuleto (**Bone Charm**), e
entender cada parte do corpo.

## 2. O que você precisa saber antes?

| Parte | `category` | Aparência possível (`visual`) |
|---|---|---|
| Cabeça | `Helmet` | `Tint` (tom da cor do corpo) |
| Tronco / capa | `Armor` | `Cape(color, length)` ou `Tint` |
| Braços | `Gloves` | — |
| Pernas | `Boots` | — |
| Mão secundária | `Shield` | — (não combina com arma de duas mãos) |
| Acessórios | `Ring`, `Amulet` | — (2 slots; o mesmo item precisa de 2 cópias para ocupar os dois) |

- Não existe slot separado de capa: **capa é uma peça de `Armor`** com
  `visual: Some(Cape(...))`. Comprimento da capa: 12 a 44.
- `Tint`: cada canal de 0,45 a 1,0 (escurece o corpo de leve).
- Ao trocar uma peça que dá HP/MP máximo, o valor atual só é cortado se
  passar do novo máximo; nunca é restaurado de graça.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/data/items/crypt_gear.ron` | Os itens (tutorial 05) |

## 4. Passo a passo

1. No arquivo de itens, crie a peça com a `category` da parte do corpo.
2. Some defesa, resistências e outros bônus em `modifiers`.
3. Opcional: `visual` (capa ou tom).
4. Ponha no jogo (baú ou loja) e teste.

## 5. Exemplo completo

```ron
// exemplo-verificado: Item
(
    id: "gravecloth_mantle",
    name: "Gravecloth Mantle",
    description: "A grey burial cloth worn as a cloak. Shadows slide off it.",
    category: Armor,
    rarity: "rare",
    modifiers: (defense: 4, magic_defense: 3, resistances: (values: [(Dark, 0.2)])),
    visual: Some(Cape(color: (0.30, 0.28, 0.26), length: 36.0)),
    weight: 2.0,
    sell_value: 150,
)
```

```ron
// exemplo-verificado: Item
(
    id: "bone_charm",
    name: "Bone Charm",
    description: "A carved knucklebone on a cord.",
    category: Amulet,
    rarity: "common",
    modifiers: (max_mp: 8, luck: 1),
    sell_value: 60,
)
```

## 6. Como testar?

1. `cargo test -j 2 --test data_tests`.
2. `cargo run -j 2`: pegue a peça, `I`, equipe. Veja DEF/MDEF/MP na
   comparação e, para capas, a capa atrás do personagem.
3. `cargo test -j 2 --test tutorial_examples_tests` (o exemplo abre os baús
   que dão esses itens).

## 7. Como editar depois?

Mude `modifiers`, `color` ou `length` e teste de novo.

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `visual Cape does not fit category` | Capa fora de `Armor` | Use `category: Armor` |
| `invalid visual value (cape length out of range)` | `length` fora de 12..44 | Ajuste |
| `invalid visual value (tint channel ...)` | Tom muito escuro | Canais ≥ 0,45 |
| "The weapon needs both hands" | Escudo com lança/pesada | Use arma de uma mão |

## 9. Como desfazer com segurança?

Apague o bloco do item e tire-o de baús e lojas.
