# 21 — Cadastrar mercadorias

> Situação: **funciona** (estoque limitado volta ao carregar; veja o aviso do
> tutorial 20).

## 1. O que vamos fazer?

Acrescentar itens à venda numa loja, com estoque ilimitado ou limitado, e
itens que só aparecem depois de um certo progresso.

## 2. O que você precisa saber antes?

- Cada mercadoria é uma linha em `entries` da loja:
  `(item: "id", price: N, stock: Unlimited)` ou `stock: Limited(quantidade)`.
- O item precisa existir no catálogo (tutorial 05). Não use `QuestItem`.
- Cada item só uma vez por loja.
- `when: [...]`: a mercadoria só aparece (e só pode ser comprada) quando as
  condições valem (as mesmas dos diálogos, tutorial 19).
- Estoque limitado: a última unidade só pode ser comprada uma vez, mesmo
  apertando Enter várias vezes no mesmo instante.
- Não coloque o catálogo inteiro numa loja; itens fortes demais cedo deixam o
  castelo fácil.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/data/shops/ashen_crypt.ron` | A loja do exemplo |

## 4. Passo a passo

1. Abra a loja e encontre `entries: [ ... ]`.
2. Acrescente uma linha por mercadoria.
3. Defina o preço (tutorial 22).
4. Teste.

## 5. Exemplo completo

Ilimitada:

```ron
// exemplo-verificado: Mercadoria
(item: "crypt_draught", price: 30, stock: Unlimited)
```

Uma unidade só:

```ron
// exemplo-verificado: Mercadoria
(item: "hearthsplitter", price: 140, stock: Limited(1))
```

Só depois da Relíquia do Véu:

```ron
// exemplo-verificado: Mercadoria
(item: "still_water_amulet", price: 330, stock: Limited(1), when: [Ability(VeilStep)])
```

## 6. Como testar?

1. `cargo test -j 2 --test data_tests`.
2. `cargo run -j 2`: abra a loja; a coluna da direita mostra `x1` (limitado),
   `-` (ilimitado) ou `sold out`.
3. `cargo test -j 2 --test npc_commerce_tests`.

## 7. Como editar depois?

Mude `price`, `stock` ou `when`; apague a linha para tirar da venda.

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `... listed twice` | Mesmo item duas vezes | Uma linha só |
| `Limited(0)` | Estoque zero | Mínimo 1 |
| `is a key item` | `QuestItem` à venda | Não venda itens-chave |
| Item não aparece | `when` ainda não vale | Normal |

## 9. Como desfazer com segurança?

Apague a linha da mercadoria.
