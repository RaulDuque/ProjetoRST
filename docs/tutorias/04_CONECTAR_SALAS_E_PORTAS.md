# 04 — Conectar salas e portas

> Situação: **funciona**. Exemplo verificado por `tests/tutorial_examples_tests.rs`.

## 1. O que vamos fazer?

Ligar duas salas com uma porta, nos dois sentidos, e conhecer as portas
especiais: trancadas, escondidas, só de chegada e atalhos.

## 2. O que você precisa saber antes?

- **Uma porta são duas metades**, uma em cada sala, apontando uma para a outra.
- `side`: `Left`, `Right` (parede), `Top` (teto) ou `Bottom` (chão). As duas
  metades ficam em lados opostos (`Right` ↔ `Left`, `Bottom` ↔ `Top`).
- `cell`: em salas maiores que 1 célula, diz em qual célula do lado a porta
  fica (0 = a de baixo para Left/Right, a da esquerda para Top/Bottom).
- **As salas precisam se tocar no mapa** naquela célula.
- Abertura da porta: nas paredes, 160 de altura a partir do chão da célula;
  no chão/teto, 200 de largura no meio da célula. Portas em células acima da
  primeira ganham uma **saliência** (220 de largura) para o jogador ficar em pé.
- Portas no **teto** são usadas, hoje, só como chegada de uma queda
  (`entry_only: true`): o jogador não sobe pelo teto.
- Campos opcionais da porta:
  - `requires: Some(...)`: trancada (grade) até o jogador ter algo:
    `Ability(VeilStep)`, `Item("watch_pike")`, `Flag("nome")`,
    `All([...])` (tudo) ou `Any([...])` (um deles);
  - `hidden_by: Some("id_do_segredo")`: escondida até o segredo ser achado (09, 10);
  - `entry_only: true`: só chegada (não dá para sair por ela);
  - `shortcut: true`: atalho (normalmente aberto por alavanca do outro lado, 11).

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/world/regions/ash_village.ron` | Ganha a porta `to_crypt` na casa da Wenna |
| `assets/world/regions/ashen_crypt.ron` | A outra metade, `to_cottage`, na Crypt Stair |

## 4. Passo a passo

1. Confira no mapa que as salas se tocam: `av_cottage` está em x=9, y=1 e
   `ac_crypt_stair` em x=10, y=1 (vizinhas lado a lado).
2. Na sala da esquerda (`av_cottage`), acrescente em `doors` a metade `Right`.
3. Na sala da direita (`ac_crypt_stair`), a metade `Left`.
4. Em cada metade, `to:` aponta para a **sala** e a **porta** do outro lado.
5. Teste.

## 5. Exemplo completo

Metade na casa da Wenna (`ash_village.ron`, sala `av_cottage`):

```ron
// exemplo-verificado: Porta@av_cottage
(id: "to_crypt", side: Right, to: (room: "ac_crypt_stair", door: "to_cottage")),
```

Metade na cripta (já está no arquivo do tutorial 03):

```text
(id: "to_cottage", side: Left, to: (room: "av_cottage", door: "to_crypt")),
```

Uma porta trancada por alavanca (as duas metades pedem a mesma flag):

```text
(id: "to_ossuary", side: Right, to: (room: "ac_bone_ossuary", door: "to_crypt_stair"),
    requires: Some(Flag("ac_ossuary_gate_open"))),
```

## 6. Como testar?

1. `cargo run -j 2 --example world_report` → `Errors (0):`.
2. `cargo run -j 2`: atravesse a porta nos dois sentidos.
3. `cargo test -j 2 --test tutorial_examples_tests`.

## 7. Como editar depois?

- Trancar: acrescente `requires: Some(...)` nas duas metades.
- Mudar o destino: troque `to:` **nas duas metades**.
- Remover a porta: apague as duas metades.

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `MissingTargetRoom` / `MissingTargetDoor` | `to:` aponta para algo que não existe | Corrija o nome |
| `NotReciprocal` | A outra metade não aponta de volta | As duas precisam se apontar |
| `Misaligned` | As salas não se tocam naquela célula | Ajuste `map` ou `cell` |
| `DoorCellOutOfRange` | `cell` maior que o tamanho da sala | `cell` vai de 0 a `h-1` (laterais) ou `w-1` (teto/chão) |
| `BothEntryOnly` | As duas metades com `entry_only` | Só a de chegada |
| `ImpossibleRequirement` | Nada no jogo dá o que a porta pede | Crie a alavanca/relíquia/item |
| `DuplicateDoor` | Dois `id` iguais na mesma sala | Renomeie |

## 9. Como desfazer com segurança?

Apague as duas metades da porta. Se copiou os arquivos antes, copie-os de volta.
