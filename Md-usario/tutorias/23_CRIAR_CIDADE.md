# 23 — Criar cidade

> Situação: **funciona**. A Vila das Cinzas (V2.1) é o modelo real.

## 1. O que vamos fazer?

Entender como a **Vila das Cinzas** foi montada (região + NPCs + falas +
loja + ligação ao castelo), para criar outra cidade do mesmo jeito.

## 2. O que você precisa saber antes?

- Uma cidade é uma **região** comum (tutorial 03) com NPCs (18), diálogos (19)
  e lojas (20). Não há código especial de "cidade".
- Ligação ao castelo: uma porta legítima (04), alcançável andando e pulando.
  A vila sai da porta direita do **Watch Shaft**, na segunda célula (altura
  ~760), com um degrau novo para chegar lá.
- Deixe as salas `role: Optional` para a cidade não ser obrigatória.
- NPCs só existem na sala em que estão; ao trocar de sala, somem e voltam.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/world/regions/ash_village.ron` | As 6 salas da vila |
| `assets/world/regions/vigil_gates.ron` | Porta `to_village` e degrau do Watch Shaft |
| `assets/data/npcs/ash_village.ron` | Bram, Hooded Pilgrim, Hessa, Garrick, Old Wenna |
| `assets/data/dialogues/ash_village.ron` | Falas, com condições de progresso |
| `assets/data/shops/ash_village.ron` | Hessa's Outfitters |

## 4. Passo a passo

1. **Lugar no mapa:** células livres vizinhas de uma sala do castelo (15).
2. **Região:** um arquivo novo em `assets/world/regions/` (03).
3. **Ligação:** porta nas duas salas (04); se a porta ficar alta, um degrau
   (`one_way`) para chegar até ela (01).
4. **Moradores:** um arquivo em `assets/data/npcs/` (18) e as falas (19).
5. **Comércio:** loja (20), mercadorias (21) e preços (22).
6. Teste a rota real: `cargo test -j 2 --test world_town_route_tests`.

## 5. Exemplo completo

A ligação real no Watch Shaft (`vigil_gates.ron`):

```text
(id: "to_village", side: Right, cell: 1, to: (room: "av_gate_road", door: "to_shaft")),
(min: (750.0, 760.0), size: (260.0, 16.0), one_way: true),
```

O primeiro morador da vila, com fala que muda depois da Relíquia do Véu:

```ron
// exemplo-verificado: Npc
(
    id: "bram_road_warden",
    name: "Bram, the Road Warden",
    kind: Informant,
    room: "av_gate_road",
    position: (900.0, 63.0),
    size: (28.0, 46.0),
    color: (0.45, 0.5, 0.62),
    greetings: [
        (when: [Ability(VeilStep)], node: "bram_after_relic"),
        (node: "bram_intro"),
    ],
)
```

A loja real (trecho):

```ron
// exemplo-verificado: Loja
(
    id: "hessa_outfitters",
    name: "Hessa's Outfitters",
    keeper: "hessa_outfitter",
    buys: true,
    entries: [
        (item: "minor_tonic", price: 25, stock: Unlimited),
        (item: "gutter_knife", price: 80, stock: Unlimited),
    ],
)
```

## 6. Como testar?

1. `cargo run -j 2 --example world_report` → `Errors (0):`.
2. `cargo test -j 2 --test world_town_route_tests` (do castelo até a vila,
   conversa, loja, compra, venda e volta, só com teclas).
3. Com janela: `cargo run -j 2 --example world_town_route` (capturas em
   `target/world_town_route/`).
4. `cargo run -j 2`: suba o Watch Shaft até o degrau da direita, na altura da
   segunda célula, e siga para a direita.

## 7. Como editar depois?

Cada parte tem seu tutorial: salas (02), NPCs (18), falas (19), loja (20–22).

## 8. Problemas comuns

| Problema | Solução |
|---|---|
| Não consigo chegar à porta da cidade | O degrau precisa ficar a até ~110 de altura do anterior |
| A rota principal falhou depois da cidade | Você mexeu no caminho dela (16) |
| NPC não aparece | `room` errado, ou você está em outra sala |

## 9. Como desfazer com segurança?

Para uma cidade **sua**: apague o arquivo da região, os de NPCs, falas e loja,
e a porta/degrau acrescentados no castelo. **Não apague a Vila das Cinzas**:
ela faz parte do jogo (e da rota de testes).
