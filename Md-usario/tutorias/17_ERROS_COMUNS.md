# 17 — Erros comuns

> Situação: referência para todos os tutoriais.

## 1. O que vamos fazer?

Reunir as mensagens de erro mais comuns, o que significam e como resolver.

## 2. O que você precisa saber antes?

- Erros de **sintaxe RON** dizem o arquivo, a linha e a coluna:
  `invalid data in assets/data/items/crypt_gear.ron: 12:5: Expected ...`.
- Erros do **mundo** aparecem no `world_report` (`Errors (N):`) ou ao abrir o
  jogo (`invalid world: [...]`).
- Erros de **dados** (itens, inimigos, NPCs, lojas) aparecem ao abrir o jogo
  e no `cargo test -j 2 --test data_tests`.

## 3. Quais arquivos vamos utilizar?

O arquivo citado na mensagem de erro.

## 4. Passo a passo

1. Leia a mensagem inteira; procure o nome do arquivo e o `id` citado.
2. Ache a linha (`grep -n "id_citado" -r assets/`).
3. Compare com o exemplo do tutorial do assunto.
4. Corrija e rode o teste de novo.

## 5. Exemplo completo

| Mensagem | Significado | Solução |
|---|---|---|
| `Expected ... but found ...` | Vírgula, parêntese ou aspas faltando | Confira o fim da linha anterior |
| `unknown field` | Campo com nome errado | Copie o nome do exemplo |
| `unknown variant` | Opção fixa escrita errada (`sword` em vez de `Sword`) | Mesma grafia do exemplo |
| `missing field` | Campo obrigatório faltando | Acrescente-o |
| `duplicate id` / `DuplicateId` / `DuplicateRoom` | ID repetido | Use outro ID |
| `UnknownRegion` | Sala com `region` errado | Igual ao `id` da região |
| `MissingTargetRoom` / `MissingTargetDoor` | Porta aponta para o nada | Corrija `to:` |
| `NotReciprocal` | Porta só de um lado | Faça a outra metade |
| `Misaligned` | Salas não encostam | Ajuste `map`/`cell` |
| `Unreachable` | Sala sem caminho | Ligue com uma porta |
| `ImpossibleRequirement` | Exigência que nada concede | Crie alavanca/relíquia/item |
| `SecretRoomNotHidden` / `RequiredBehindSecret` | `role` errado para a sala | Tutorial 09 |
| `ObjectOutOfRoom` | Objeto fora da sala | Ajuste `area` |
| `UnusedLever` (aviso) | Alavanca inútil | Use a flag numa porta |
| `BadVeil` | Véu inválido | Tutorial 13 |
| `spawns unknown enemy` | Inimigo inexistente | Tutorial 14 |
| `grants unknown item` | Item inexistente | Tutorial 05 |
| `uses unknown rarity` | Raridade errada | `common`, `uncommon`, `rare`, `epic`, `legendary` |
| `NPC ... is in unknown room` / `stands outside room` | NPC mal posicionado | Tutorial 18 |
| `unknown dialogue node` | Fala com ID errado | Tutorial 19 |
| `OpenShop for an NPC without a shop` | Opção de loja num NPC sem loja | Tutorial 20 |
| `price ... must be above its sell value` | Preço baixo demais | Tutorial 22 |
| `NPC ... and shop ... do not match` | `shop` do NPC e `keeper` da loja não batem | Tutorial 20 |
| "Cannot trade: sold out" (no jogo) | Estoque limitado acabou (continua acabado depois de F5/F9) | Normal |

## 6. Como testar?

Depois de corrigir: `cargo run -j 2 --example world_report` e
`cargo test -j 2 --test data_tests`.

## 7. Como editar depois?

Esta lista cresce a cada versão.

## 8. Problemas comuns

Se a mensagem não está aqui, copie-a inteira e peça ajuda ao desenvolvedor.

## 9. Como desfazer com segurança?

Volte a cópia do arquivo que você fez antes de editar (tutorial 00). Não use
`git restore`, `git reset --hard` nem `git clean`.
