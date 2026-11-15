# 03 — Criar regiões e fases

> Situação: **funciona**. Exemplo verificado por `tests/tutorial_examples_tests.rs`.

## 1. O que vamos fazer?

Criar uma **região** (uma "fase" do castelo, com nome e cor próprios no
mapa) chamada **Ashen Crypt**, com 3 salas, e ligá-la ao jogo pela casa da
Old Wenna, na Vila das Cinzas.

## 2. O que você precisa saber antes?

- O mundo é uma **grade**. Cada célula mede **1280 × 720** unidades.
- Cada sala ocupa um retângulo de células (`map: (x, y, w, h)`). `y` cresce
  para **cima**.
- Uma região é **um arquivo** em `assets/world/regions/`. Todo arquivo
  `.ron` dessa pasta é carregado automaticamente.
- Salas de regiões diferentes se ligam por **portas** (tutorial 04).
- Células usadas hoje (y=1): de x=0 a x=9 (a vila vai até x=9). As células
  (10,1), (11,1) e (12,1) estão livres.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/world/regions/ashen_crypt.ron` | **Novo.** A região e suas 3 salas |
| `assets/world/regions/ash_village.ron` | Existe. Ganha uma porta na casa da Wenna |
| `assets/data/enemies/crypt_dwellers.ron` | **Novo.** O inimigo da região (tutorial 14) |
| `assets/data/items/crypt_gear.ron` | **Novo.** Os itens dos baús (tutorial 05) |
| `assets/data/npcs/ashen_crypt.ron`, `assets/data/dialogues/ashen_crypt.ron`, `assets/data/shops/ashen_crypt.ron` | **Novos.** O NPC da cripta (tutoriais 18 a 20) |

## 4. Passo a passo

1. Faça uma cópia de segurança: `cp assets/world/regions/ash_village.ron ~/copia_ash_village.ron`
2. Crie o arquivo `assets/world/regions/ashen_crypt.ron` com o conteúdo da seção 5.
3. Repare: em **todas** as salas, `region: "ashen_crypt"` é igual ao `id` da região.
4. Abra `assets/world/regions/ash_village.ron`, procure a sala `av_cottage` e,
   dentro de `doors: [ ... ]`, acrescente a porta para a cripta:

   ```ron
   // exemplo-verificado: Porta@av_cottage
   (id: "to_crypt", side: Right, to: (room: "ac_crypt_stair", door: "to_cottage")),
   ```
5. Crie também os arquivos dos tutoriais 05, 14, 18, 19 e 20 (itens, inimigo,
   NPC, diálogo e loja). Sem eles, o jogo recusa o mundo.
6. Teste (seção 6).

## 5. Exemplo completo

`assets/world/regions/ashen_crypt.ron`:

```ron
// exemplo-verificado: Regiao
// Região de exemplo dos tutoriais: Ashen Crypt (Cripta Cinzenta).
//
//   y=1  [av_cottage] -> [ac_crypt_stair] -> [ac_bone_ossuary] ~parede~> [ac_secret_niche]
(
    region: (id: "ashen_crypt", name: "Ashen Crypt", color: (0.50, 0.46, 0.40)),
    rooms: [
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
                // Alavanca no chão: abre a grade da porta da direita.
                (id: "ac_stair_lever", kind: Lever(sets: "ac_ossuary_gate_open"),
                    area: (min: (300.0, 40.0), size: (16.0, 40.0))),
            ],
            doors: [
                (id: "to_cottage", side: Left, to: (room: "av_cottage", door: "to_crypt")),
                (id: "to_ossuary", side: Right, to: (room: "ac_bone_ossuary", door: "to_crypt_stair"),
                    requires: Some(Flag("ac_ossuary_gate_open"))),
            ],
        ),
        (
            id: "ac_bone_ossuary",
            name: "Bone Ossuary",
            region: "ashen_crypt",
            role: Optional,
            map: (x: 11, y: 1, w: 1, h: 1),
            background: (0.06, 0.05, 0.05),
            pickups: [
                // Relíquia: dá a habilidade que a parede rachada exige.
                (id: "ac_grave_relic", name: "Grave Relic", position: (400.0, 70.0), grants: Ability(WallBreaker)),
            ],
            objects: [
                (id: "ac_ossuary_chest", kind: Chest(tier: Common, reward: Item("crypt_cleaver")),
                    area: (min: (640.0, 40.0), size: (48.0, 36.0))),
                // Parede rachada sobre a porta da direita (precisa da relíquia).
                (id: "ac_cracked_wall", kind: BreakableWall(breaks_on: AnyAttack),
                    area: (min: (1200.0, 40.0), size: (80.0, 160.0)),
                    requires: Some(Ability(WallBreaker))),
            ],
            doors: [
                (id: "to_crypt_stair", side: Left, to: (room: "ac_crypt_stair", door: "to_ossuary"),
                    requires: Some(Flag("ac_ossuary_gate_open"))),
                (id: "to_niche", side: Right, to: (room: "ac_secret_niche", door: "to_ossuary"),
                    hidden_by: Some("ac_cracked_wall")),
            ],
        ),
        (
            id: "ac_secret_niche",
            name: "Secret Niche",
            region: "ashen_crypt",
            role: Secret,
            map: (x: 12, y: 1, w: 1, h: 1),
            background: (0.09, 0.07, 0.05),
            objects: [
                (id: "ac_niche_chest", kind: Chest(tier: Secret, reward: Item("gravecloth_mantle")),
                    area: (min: (600.0, 40.0), size: (48.0, 36.0))),
                (id: "ac_niche_charm_chest", kind: Chest(tier: Secret, reward: Item("bone_charm")),
                    area: (min: (900.0, 40.0), size: (48.0, 36.0))),
            ],
            doors: [
                (id: "to_ossuary", side: Left, to: (room: "ac_bone_ossuary", door: "to_niche")),
            ],
        ),
    ],
)
```

| Campo da região | Significado |
|---|---|
| `region.id` | Nome interno, único |
| `region.name` | Nome mostrado no mapa |
| `region.color` | Cor no mapa: vermelho, verde, azul, cada um de `0.0` a `1.0` |
| `rooms` | As salas (tutorial 01) |

## 6. Como testar?

1. `cargo run -j 2 --example world_report` → deve mostrar `Errors (0):` e as
   salas `ac_...` na lista `Access:`.
2. `cargo run -j 2` → vá até a casa da Wenna (Vila das Cinzas, tutorial 23)
   e siga para a direita.
3. Teste automático do exemplo: `cargo test -j 2 --test tutorial_examples_tests`.

## 7. Como editar depois?

- Nome ou cor: mude `name` / `color`.
- Mais uma sala: acrescente outro bloco `( id: ..., ... )` em `rooms` e uma
  porta que leve até ela (tutoriais 01 e 04).
- Mudar a região de lugar no mapa: tutorial 15.

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `UnknownRegion` | `region` da sala diferente do `id` da região | Copie o `id` exato |
| `DuplicateRegion` / `DuplicateRoom` | ID repetido | Use um prefixo próprio (`ac_`) |
| `Unreachable` | Nenhuma porta chega até a sala | Faça o passo 4 |
| `spawns unknown enemy` / `grants unknown item` | Inimigo ou item não existe | Crie-o (tutoriais 14 e 05) |
| `NPC ... is in unknown room` | Um NPC aponta para uma sala que não existe | Corrija o `room` do NPC |

## 9. Como desfazer com segurança?

1. Apague os arquivos novos: `rm assets/world/regions/ashen_crypt.ron` (e os
   dos tutoriais 05, 14, 18, 19 e 20, se criou).
2. Volte a vila: `cp ~/copia_ash_village.ron assets/world/regions/ash_village.ron`
   (ou apague só a linha da porta `to_crypt`).
3. Rode `cargo run -j 2 --example world_report` para conferir `Errors (0):`.
