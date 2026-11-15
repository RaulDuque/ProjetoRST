# 09 — Criar salas secretas

> Situação: **funciona**.

## 1. O que vamos fazer?

Criar uma sala que só aparece depois que o jogador descobre um segredo: a
**Secret Niche** (`ac_secret_niche`), atrás de uma parede rachada.

## 2. O que você precisa saber antes?

- Sala secreta = `role: Secret`. O validador exige que ela **só** seja
  alcançável achando um segredo.
- A entrada é uma porta com `hidden_by: Some("id_do_segredo")`. Até o segredo
  ser achado, a porta não existe (nem no mapa).
- Três jeitos de esconder uma entrada:

| Segredo | Como se acha | Onde se escreve |
|---|---|---|
| Parede falsa (`IllusoryWall`) | Andando através dela | `secrets: [ ... ]` |
| Parede quebrável (`BreakableWall`) | Atacando (tutorial 10) | `objects: [ ... ]` |
| Interruptor oculto (`HiddenSwitch`) | Apertando D nele | `objects: [ ... ]` |

- O `id` do segredo (ou do objeto) é o que vai em `hidden_by`.
- Para não revelar antes da hora: a porta escondida só pode ser usada depois
  do segredo; e um `requires` no objeto (ex.: uma relíquia) atrasa ainda mais.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/world/regions/ashen_crypt.ron` | Ossuário (entrada escondida) e Nicho (sala secreta) |

## 4. Passo a passo

1. Crie a sala com `role: Secret` (o Nicho do tutorial 03).
2. Na sala vizinha, crie o objeto que esconde a entrada (a parede rachada).
3. Na porta dessa sala para o Nicho, ponha `hidden_by: Some("ac_cracked_wall")`.
4. A porta de volta, dentro do Nicho, é normal.
5. Teste.

## 5. Exemplo completo

A porta escondida (no Ossuário):

```text
(id: "to_niche", side: Right, to: (room: "ac_secret_niche", door: "to_ossuary"),
    hidden_by: Some("ac_cracked_wall")),
```

Alternativa com parede falsa (sem colisão), cobrindo a porta da direita:

```ron
// exemplo-verificado: Segredo
(id: "ac_false_wall", kind: IllusoryWall, area: (min: (1160.0, 40.0), size: (120.0, 240.0)))
```

Alternativa com interruptor oculto (um tijolo solto na parede):

```ron
// exemplo-verificado: Objeto
(id: "ac_loose_brick", kind: HiddenSwitch,
    area: (min: (1150.0, 50.0), size: (24.0, 24.0)))
```

## 6. Como testar?

1. `cargo run -j 2 --example world_report`: em `Access:`, o Nicho aparece como
   `secret`; `Errors (0):`.
2. `cargo run -j 2`: antes do segredo, o mapa não mostra a porta. Ache o
   segredo; a porta aparece; a contagem `secrets` do mapa sobe.
3. `cargo test -j 2 --test tutorial_examples_tests`.

## 7. Como editar depois?

Troque o tipo de segredo mudando o objeto/segredo e o `hidden_by`.

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `SecretRoomNotHidden` | Dá para chegar sem segredo | Esconda **todas** as entradas |
| `RequiredBehindSecret` | Sala `Required`/`Optional` atrás de segredo | Use `role: Secret` |
| `UnknownSecret` | `hidden_by` com ID que não existe | Use o `id` do segredo/objeto |
| `DuplicateId` | Segredo com ID repetido | Use outro |

## 9. Como desfazer com segurança?

Apague a sala secreta, o segredo/objeto e a porta escondida (as duas metades).
