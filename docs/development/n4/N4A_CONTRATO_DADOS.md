# N4-A — Contrato de dados (RON)

Formato lido por `src/player/visual/composition.rs` (`CharacterComposition::from_ron`). Exemplo real: `art-src/n4-sprite-machine/stage/protagonist.ron`.

```ron
(
    version: 1,                          // só 1 é aceito
    id: "protagonist_n4a",               // identificador do personagem
    clips: {                             // canvas e pivô por clipe (todas as camadas)
        "idle": (cell: (80, 128), pivot: (40.0, 118.0), density: 2, frames: 1),
        // "run": (cell: (112, 128), pivot: (56.0, 118.0), density: 2, frames: 6),
    },
    layers: ["body", "trousers", "boots", "arm_front", "shirt", "bracers", "hair_back", "hair_front"],
    parts: {
        "body": (region: Body, removable: false, pieces: [
            (layer: "body", sheets: { "idle": (texture: "parts/idle/body.png") }),
            (layer: "arm_front", sheets: { "idle": (texture: "parts/idle/arm_front.png") }),
        ]),
        "hair": (region: HairBack, pieces: [ ... ]),
        // ...
    },
    default_outfit: ["hair", "shirt", "trousers", "boots", "bracers"],
    order_overrides: [],                 // opcional
)
```

## Campos

| Campo | Tipo | Regra |
|---|---|---|
| `version` | `u32` | `1` |
| `id` | texto | identificador do personagem |
| `clips` | mapa clipe → `(cell, pivot, density, frames)` | célula > 0, densidade > 0, quadros > 0, pivô dentro da célula. **Todas** as camadas do clipe usam essa célula e esse pivô |
| `layers` | lista de nomes | sem repetição; a posição é o z (0 = mais atrás) |
| `parts` | mapa peça → `PartDef` | ver abaixo |
| `default_outfit` | lista de peças | todas existem; o corpo-base é sempre vestido |
| `order_overrides` | lista `(clip, frames: Some([..]) ou None, layer, z)` | clipe e camada existentes; muda o z só naquele clipe (e quadros) |

`PartDef`:

| Campo | Padrão | Regra |
|---|---|---|
| `region` | — | enum `Region` (Body, HairBack, HairFront, Shirt, Trousers, Boots, Bracers, Gloves, Coat, Chest, Shoulders, ArmArmor, LegArmor, Helmet, Hood, CapeBack, CapeFront, MainWeapon, OffHand, Accessory) |
| `removable` | `true` | exatamente uma peça `region: Body, removable: false` (o corpo-base) |
| `pieces` | — | ≥ 1; cada uma é `(layer, sheets: { clipe: (texture, first, columns, rows) })` |
| `hides` | `[]` | regiões escondidas enquanto a peça está vestida e visível; não pode esconder a própria região |
| `variants` | `{}` | nome → lista de peças alternativas, com as mesmas regras |
| `socket` | `None` | soquete seguido por peças fora do canvas do corpo (armas, escudos; ver `sockets.rs`). Reservado: nenhuma peça da N4-A usa |

`SheetRef`: `texture` (caminho de asset), `first` (índice do quadro 0 do clipe, padrão 0), `columns` e `rows` (padrão 1). `first + frames` precisa caber em `columns × rows`.

## Erros (nunca silenciosos)

`CompositionError`: `Parse`, `Version`, `DuplicateLayer`, `UnknownLayer`, `UnknownClip`, `FrameRange`, `Clip`, `Empty`, `HidesItself`, `NoBase`, `UnknownPart`, `UnknownVariant`, `Override`. `resolve` também devolve erro para clipe, peça ou variante desconhecidos, em vez de desenhar outra coisa.

## Compatibilidade

- O `AnimationManifest` (`player.ron` da V2.2 e o staging N3.8) não mudou e continua carregando (teste). Um RON de composição não é aceito como manifesto, e um manifesto não é aceito como composição.
- RON é o formato principal. JSON pode vir depois como formato de intercâmbio: os tipos são `serde::Deserialize`, então o contrato não muda, mas ler JSON exigiria a dependência `serde_json`, que hoje não existe e teria de ser justificada em `docs/DECISIONS.md`. A N4-A não adicionou nenhuma dependência, nem HTML ou JavaScript.
