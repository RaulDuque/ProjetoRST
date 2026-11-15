# 22 — Configurar preços

> Situação: **funciona**.

## 1. O que vamos fazer?

Definir quanto a loja cobra e quanto ela paga, sem criar um jeito de ganhar
dinheiro infinito.

## 2. O que você precisa saber antes?

- **Preço de compra** (o jogador paga): `price` na mercadoria da loja.
- **Preço de venda** (a loja paga): `sell_value` no item (catálogo). É o mesmo
  em todas as lojas que compram.
- **Regra de segurança:** `price` precisa ser **maior** que o `sell_value`
  do item. O jogo recusa a loja se não for. Assim comprar e revender sempre
  perde dinheiro.
- Limites: `price` de 1 a 99 999; a bolsa vai até 999 999 Marks; nada fica
  negativo. Sempre números inteiros.
- Sugestão usada no jogo: preço de compra entre 2 e 2,5 vezes o `sell_value`
  (mais caro para itens raros ou limitados).

| Item (exemplo) | `sell_value` | `price` na loja |
|---|---|---|
| Minor Tonic | 10 | 25 |
| Gutter Knife | 35 | 80 |
| Quiet Soles (limitado) | 100 | 260 |
| Crypt Draught | 15 | 30 |

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/data/shops/*.ron` | `price` de cada mercadoria |
| `assets/data/items/*.ron` | `sell_value` de cada item |
| `assets/data/character.ron` | `coins`: dinheiro inicial |

## 4. Passo a passo

1. Veja o `sell_value` do item no catálogo.
2. Escolha `price` maior que ele (ex.: 2×).
3. Para mudar o dinheiro inicial, edite `coins` em `character.ron`.
4. Teste.

## 5. Exemplo completo

Item com valor de venda 15:

```ron
// exemplo-verificado: Item
(
    id: "crypt_draught",
    name: "Crypt Draught",
    description: "Restores 20 MP.",
    category: Consumable,
    rarity: "common",
    consumable: Some(RestoreMp(20)),
    max_stack: 9,
    sell_value: 15,
)
```

Mercadoria com preço 30 (maior que 15):

```ron
// exemplo-verificado: Mercadoria
(item: "crypt_draught", price: 30, stock: Unlimited)
```

## 6. Como testar?

1. `cargo test -j 2 --test data_tests` (recusa preço ≤ valor de venda).
2. `cargo test -j 2 --test npc_commerce_tests` (compra e revende 20 vezes e
   confere que o dinheiro só diminui).

## 7. Como editar depois?

Mude `price` ou `sell_value` e rode os testes. Se subir o `sell_value` de um
item, confira as lojas que o vendem.

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `price N must be above its sell value M` | Preço baixo demais | `price` > `sell_value` |
| `price out of range` | 0 ou acima de 99 999 | Ajuste |
| "Cannot trade: purse full" | Bolsa no máximo | Gaste antes de vender |

## 9. Como desfazer com segurança?

Volte os números antigos (ou a cópia do arquivo).
