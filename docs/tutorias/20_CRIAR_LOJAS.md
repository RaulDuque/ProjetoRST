# 20 — Criar lojas

> Situação: **funciona**. Desde a V2.1.1 (save v3), o F5 grava o dinheiro e
> o estoque limitado das lojas junto com a bolsa, e o F9 restaura os três
> juntos: uma compra e o pagamento dela nunca se separam. Saves antigos (v1/v2)
> carregam com as moedas iniciais (`coins` de `character.ron`) e estoque cheio.

## 1. O que vamos fazer?

Criar a loja do Morrow: o que ele vende, por quanto, e se compra os itens
do jogador.

## 2. O que você precisa saber antes?

- Lojas ficam em `assets/data/shops/*.ron` (uma lista `[ ... ]`).
- A loja tem um dono (`keeper`, o `id` do NPC), e o NPC aponta de volta com
  `shop: Some("id_da_loja")`. Uma escolha `OpenShop` no diálogo abre a loja.
- **Moeda:** "Marks" (aparece no HUD, ao lado de Hearts). O personagem começa
  com `coins: 120` (`assets/data/character.ron`).
- Na tela da loja: `←`/`→` Comprar/Vender, `↑`/`↓` escolhem, **Enter**
  compra ou vende **uma** unidade, **Esc** sai. Equipamentos mostram a
  comparação com o que está equipado.
- `buys: true`: a loja compra itens da bolsa pelo `sell_value` do item.
  Nunca compra: itens equipados (tire antes), `QuestItem`, itens sem valor,
  e a última arma (se nenhuma estiver equipada).

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/data/shops/ashen_crypt.ron` | **Novo.** A loja |
| `assets/data/npcs/ashen_crypt.ron` | O NPC dono (tutorial 18) |
| `assets/data/shops/ash_village.ron` | Existe. A loja da Hessa, como modelo |

## 4. Passo a passo

1. Crie `assets/data/shops/ashen_crypt.ron` (seção 5).
2. `keeper` = `id` do NPC; no NPC, `shop: Some("morrow_wares")`.
3. No diálogo, uma escolha `OpenShop` (tutorial 19).
4. Cadastre as mercadorias (21) e os preços (22).
5. Teste comprar e vender.

## 5. Exemplo completo

```ron
// exemplo-verificado: Lojas
// Loja do Morrow (região de exemplo Ashen Crypt).
[
    (
        id: "morrow_wares",
        name: "Morrow's Grave Goods",
        keeper: "morrow_crypt_keeper",
        buys: true,
        entries: [
            (item: "crypt_draught", price: 30, stock: Unlimited),
            (item: "sentry_coif", price: 70, stock: Unlimited),
            (item: "hearthsplitter", price: 140, stock: Limited(1)),
        ],
    ),
]
```

## 6. Como testar?

1. `cargo test -j 2 --test data_tests`.
2. `cargo run -j 2`: D no Morrow → "Show me what you sell." → compre (Enter),
   veja Marks diminuir; `→` Vender, venda algo, veja Marks subir.
3. `cargo test -j 2 --test tutorial_examples_tests` (o teste abre esta loja) e
   `cargo test -j 2 --test town_app_tests`.

## 7. Como editar depois?

Acrescente ou tire linhas de `entries`. `buys: false` faz a loja só vender.

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `NPC ... and shop ... do not match` | `keeper` e `shop` não se apontam | Confira os dois IDs |
| `no entries` | Loja vazia | Pelo menos uma mercadoria |
| `unknown item` | Mercadoria inexistente | Tutorial 05 |
| "Cannot trade: not enough coins" | Pouco dinheiro | Venda algo |
| "Cannot trade: sold out" | Estoque limitado acabou | Normal |

## 9. Como desfazer com segurança?

Apague o arquivo da loja, o `shop` do NPC e a escolha `OpenShop` do diálogo.
