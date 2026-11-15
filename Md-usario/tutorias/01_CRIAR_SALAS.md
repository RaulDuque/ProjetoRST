# 01 — Criar salas

> Situação: **funciona**. Exemplo verificado por `tests/tutorial_examples_tests.rs`.

## 1. O que vamos fazer?

Criar uma sala nova: escolher o ID, o tamanho, as plataformas, as portas, o
lugar no mapa, e testar. Vamos usar a sala **Crypt Stair** (`ac_crypt_stair`)
da região de exemplo (tutorial 03).

## 2. O que você precisa saber antes?

- Uma sala vive **dentro de um arquivo de região** (`assets/world/regions/*.ron`),
  na lista `rooms: [ ... ]`.
- **Tamanho:** a sala ocupa `w × h` células do mapa. Cada célula tem 1280 de
  largura e 720 de altura. `w: 2` = sala com 2560 de largura.
- **Coordenadas dentro da sala:** `(0, 0)` é o canto **inferior esquerdo**;
  `y` cresce para cima.
- **Paredes, chão e teto são criados sozinhos** (espessura 40). O topo do
  chão fica em `y = 40`. O jogador em pé fica com o centro em `y = 63`.
- **Spawn:** não existe um campo de spawn. O jogador aparece **ao lado da
  porta por onde entrou**. Ao morrer e levantar (R), volta para perto da
  **primeira porta normal** da sala (nem `entry_only` nem escondida). A sala
  inicial do jogo é escolhida em `assets/world/world.ron` (`start_room` e
  `start_door`).
- **ID:** curto, minúsculo, com `_`, com o prefixo da região (`ac_`). Nunca
  repita um ID de sala em todo o jogo.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/world/regions/ashen_crypt.ron` | A região de exemplo, onde a sala fica |

## 4. Passo a passo

1. Abra o arquivo da região (ou crie-o como no tutorial 03).
2. Dentro de `rooms: [ ... ]`, acrescente um bloco `( ... ),` para a sala.
3. Preencha os campos obrigatórios:
   - `id`, `name` (nome no mapa), `region` (o `id` da região);
   - `role`: `Required` (caminho principal), `Optional` (fora do caminho,
     mas sem segredo) ou `Secret` (só por um segredo, tutorial 09);
   - `map: (x, y, w, h)`: posição e tamanho na grade (tutorial 15);
   - `background: (r, g, b)`: cor de fundo, cada valor de 0.0 a 1.0.
4. Opcionais: `platforms`, `enemies` (14), `doors` (04), `pickups` (12),
   `secrets` (09), `objects` (08, 10, 11), `veils` (13).
5. **Plataformas:** `(min: (x, y), size: (largura, altura))`. `min` é o canto
   inferior esquerdo. `one_way: true` = dá para subir por baixo e ficar em cima.
   - O pulo alcança cerca de **110** de altura: degraus mais altos que isso
     ficam inalcançáveis.
6. **Portas:** pelo menos uma, ligada a outra sala (tutorial 04).
7. Teste.

## 5. Exemplo completo

Trecho da sala (o arquivo inteiro está no tutorial 03):

```ron
// exemplo-verificado: Sala
(
    id: "ac_crypt_stair",
    name: "Crypt Stair",
    region: "ashen_crypt",
    role: Optional,
    map: (x: 10, y: 1, w: 1, h: 1),
    background: (0.07, 0.06, 0.06),
    platforms: [
        (min: (700.0, 130.0), size: (200.0, 16.0), one_way: true),
    ],
    enemies: [
        (enemy: "crypt_ghoul", position: (1000.0, 70.0), patrol_half_width: 120.0),
    ],
    objects: [
        (id: "ac_stair_lever", kind: Lever(sets: "ac_ossuary_gate_open"),
            area: (min: (300.0, 40.0), size: (16.0, 40.0))),
    ],
    doors: [
        (id: "to_cottage", side: Left, to: (room: "av_cottage", door: "to_crypt")),
        (id: "to_ossuary", side: Right, to: (room: "ac_bone_ossuary", door: "to_crypt_stair"),
            requires: Some(Flag("ac_ossuary_gate_open"))),
    ],
)
```

Uma plataforma sozinha (para copiar):

```ron
// exemplo-verificado: Plataforma@ac_crypt_stair
(min: (300.0, 240.0), size: (160.0, 16.0), one_way: true),
```

## 6. Como testar?

1. `cargo run -j 2 --example world_report` → `Errors (0):`, e a sala aparece em
   `Access:` como `main` (Required/Optional) ou `secret` (Secret).
2. `cargo run -j 2` → entre na sala. O minimapa (canto superior direito)
   mostra a sala; `M` esconde e mostra o mapa.
3. `cargo test -j 2 --test tutorial_examples_tests`.

## 7. Como editar depois?

- Fundo: mude `background`. Altura/largura: mude `w`/`h` do `map` e ajuste
  as salas vizinhas (tutorial 15).
- Remover a sala: apague o bloco **e** todas as portas de outras salas que
  apontam para ela.

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `NoDoors` | Sala sem nenhuma porta | Acrescente uma porta |
| `Unreachable` | Ninguém chega na sala | Ligue-a com uma porta (04) |
| `RequiredBehindSecret` | Sala `Required`/`Optional` só alcançável por segredo | Use `role: Secret` |
| `SecretRoomNotHidden` | Sala `Secret` alcançável sem segredo | Esconda a porta (09) ou use `Optional` |
| O jogador cai e fica preso | Degrau alto demais | Degraus de até ~110 de altura |

## 9. Como desfazer com segurança?

Apague o bloco da sala e as portas que apontam para ela. Se você copiou o
arquivo antes (tutorial 00), copie a cópia de volta. Não use `git restore`.
