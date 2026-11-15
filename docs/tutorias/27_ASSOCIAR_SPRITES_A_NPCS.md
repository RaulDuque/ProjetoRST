# 27 — Associar a figura ao papel do NPC

> Situação: **funciona**. **Pendente:** um NPC ainda não troca de figura
> conforme o progresso (a figura é fixa por NPC).

## 1. O que vamos fazer?

Escolher a figura certa para cada **papel** da cidade, para o jogador
reconhecer de longe quem vende, quem guarda e quem só conversa, e conferir
que o mapa marca a loja.

## 2. O que você precisa saber antes?

- O papel do NPC é o `kind` (`Villager`, `Merchant`, `Smith`, `Informant`,
  `QuestGiver`). Ele **não** desenha nada sozinho: quem desenha é o `look`
  (tutorial 26). Combine os dois.
- Quem tem `shop` aparece no mapa com o marcador de **loja** (violeta); os
  outros NPCs, com o marcador rosa (tutorial 28).
- Combinações usadas na Vila das Cinzas:

| NPC | `kind` | `look` |
|---|---|---|
| Bram (guarda da estrada) | `Informant` | `Tabard` + `Helm` + `Spear` |
| Peregrino encapuzado | `QuestGiver` | `Robe` + `Hood` + `LanternStaff` |
| Hessa (loja) | `Merchant` | `Dress` + `Kerchief` + `Satchel` |
| Garrick (ferreiro) | `Smith` | `Apron` + `Bare` + `Hammer` |
| Old Wenna | `Villager` | `Shawl` + `GreyBun` + `Cane` |

- Garrick é ferreiro só de figura: **não há serviço de ferreiro** no jogo
  ainda, então o mapa o marca como NPC comum.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/data/npcs/ash_village.ron` | Os NPCs da vila |
| `assets/data/shops/*.ron` | As lojas (quem tem `shop`) |

## 4. Passo a passo

1. Decida o papel (`kind`) e se vende (`shop`).
2. Escolha um objeto que diga o papel (lança para guarda, martelo para
   ferreiro, bolsa para comerciante, cajado para peregrino, bengala para idoso).
3. Use uma cor de corpo (`color`) e uma de destaque (`accent`) diferentes dos
   NPCs da mesma sala.
4. Teste.

## 5. Exemplo completo

Um guarda para a entrada da cripta:

```ron
// exemplo-verificado: AparenciaNpc
(outfit: Tabard, headwear: Helm, prop: Some(Spear), accent: (0.25, 0.35, 0.6))
```

Uma vendedora de ervas (com `shop` no NPC, ganha marcador de loja):

```ron
// exemplo-verificado: AparenciaNpc
(outfit: Dress, headwear: Kerchief, prop: Some(Satchel), accent: (0.4, 0.6, 0.3))
```

## 6. Como testar?

1. `cargo test -j 2 --test npc_commerce_tests --test town_app_tests`.
2. `cargo run -j 2 --example world_town_route`: confira `t03`, `t07`, `t10`
   em `target/world_town_route/` (NPCs) e `t08` (mapa).
3. `cargo run -j 2 --example equipment_visual_showcase` (fila de NPCs).

## 7. Como editar depois?

Mude o `look` sem mexer em `kind`, falas ou loja: nada de progresso muda.

## 8. Problemas comuns

| Sintoma | Causa | Solução |
|---|---|---|
| Dois NPCs parecidos na mesma sala | Mesma roupa e cores | Troque `accent`/`prop` |
| Loja sem marcador violeta | NPC sem `shop` | Ligue NPC e loja (tutorial 20) |
| Marcador não aparece | Sala ainda não visitada | Normal: o mapa só mostra o que já foi visitado |

## 9. Como desfazer com segurança?

Volte o `look` anterior (ou apague a linha). Falas, loja e saves não mudam.
