# N4-A — Arquitetura da máquina de sprites (implementada)

**Existe agora.** É usada só pela prova isolada (`cargo run --example n4_sprite_machine`). O `cargo run` padrão não muda; o modo experimental N3.8 continua igual.

## Peças

| Arquivo | Tipo | Papel |
|---|---|---|
| `src/player/visual/composition.rs` | regra pura (serde, sem Bevy) | contrato RON (`CharacterComposition`), validação, `Outfit` (o que está vestido e o que está desligado), `resolve` (camadas de um clipe e quadro, de trás para a frente) |
| `src/player/visual/composite.rs` | Bevy | `CompositeCharacter` (composição + roupa + handles), `CompositeLayer` (um filho por slot de camada), `sync_composite_layers` |
| `src/player/visual/mod.rs` | Bevy | `VisualPlayback` ganhou `frame` e `flip`; o sistema de composição foi registrado depois de `animate_player_visual` |
| `src/player/visual/systems.rs` | Bevy | `animate_player_visual` preenche `frame` e `flip` |
| `art-src/n4-sprite-machine/stage/protagonist.ron` | dados | o protagonista N3.5 como 6 peças e 8 camadas |
| `art-src/n4-sprite-machine/stage/parts/idle/*.png` / `*.txt` | arte | uma folha por camada (80×128, pivô 40,118) |
| `examples/n4_sprite_machine.rs` | prova | original × modular, alternância, grade, fundo, espelho, pausa |

## Fluxo (como o Rust lê e compõe)

1. `CharacterComposition::from_ron` lê o RON (serde + `ron`, como todo dado do jogo) e valida: versão, camadas únicas, peças apontando para camadas e clipes existentes, quadros cabendo na folha, exatamente um corpo-base não removível, roupa padrão conhecida e exceções de ordem válidas.
2. `CompositeCharacter::load` cria um handle de imagem e de grade para cada folha `(clipe, textura)`. A grade usa a célula **do clipe**, então toda camada tem o mesmo canvas por construção.
3. `CompositeCharacter::spawn_layers` cria **um filho por slot de camada** (8 no protagonista), não um por detalhe anatômico.
4. A cada frame, `sync_composite_layers` lê o `VisualPlayback` do pai (clipe, `frame`, `flip`), chama `resolve` e aponta cada filho para a folha da peça visível naquele slot: índice `first + frame`, `flip_x`, âncora do pivô do clipe (`anchor_for`), z do slot (`LAYER_Z_STEP`) e visibilidade.

## Um relógio só

Não existe temporizador nas camadas. O quadro vem de `VisualPlayback.frame`, que `animate_player_visual` calcula como sempre (mesmo `elapsed`, mesmo `loop_frame`) e agora também guarda, junto com `flip`. `resolve` usa o mesmo `frame` para todas as peças e o limita ao último quadro do clipe. Peças sem folha para o clipe são puladas: nunca rodam em outro relógio. Testes: `every_layer_uses_the_same_frame` (puro), `every_layer_follows_the_one_visual_playback` (ECS) e `the_player_animation_fills_the_shared_frame_and_facing` (jogo headless).

## Camadas do protagonista (de trás para a frente)

| z | Camada | Peça | Por quê nessa posição |
|---|---|---|---|
| 0 | `body` | corpo-base | cabeça, pescoço, torso, quadril, pernas e pés ficam atrás de tudo |
| 1 | `trousers` | calça | sobre as pernas |
| 2 | `boots` | botas | sobre a barra da calça (enfiada no cano) e os pés |
| 3 | `arm_front` | corpo-base | o braço da frente fica **na frente** da calça e **atrás** da manga e do bracelete |
| 4 | `shirt` | camisa | cobre torso e braço superior |
| 5 | `bracers` | braceletes | sobre o pulso |
| 6 | `hair_back` | cabelo | sobre o crânio e as costas da camisa |
| 7 | `hair_front` | cabelo | mechas sobre o rosto |

O corpo-base é **uma peça com duas camadas** (`body` e `arm_front`), e o cabelo também (`hair_back` e `hair_front`). É a subdivisão frente/trás que o briefing permite, usada só onde a imagem exige.

## Regras de sobreposição implementadas

- **Ordem explícita:** a lista `layers` do RON; o índice é o z.
- **Visibilidade por peça:** `Outfit::set_visible` desliga uma peça vestida sem tirá-la.
- **Ocultação simples:** `hides: [Region]` numa peça esconde as peças daquelas regiões enquanto ela estiver vestida e visível (ex.: elmo → cabelo). Testada com dados de teste; o protagonista ainda não tem peça que esconda outra.
- **Variantes:** `variants: { "nome": [peças] }`, escolhida por `Outfit::wear(part, Some("nome"))`. Testada com dados de teste.
- **Ordem por clipe ou quadro (futuro):** `order_overrides: [(clip, frames, layer, z)]`, já aplicada por `resolve` e testada; o idle não precisa.
- **Sem shader de máscara:** nenhuma necessidade apareceu no idle (`N4A_RISCOS.md`).

## O que não foi feito (de propósito)

- Nenhum segundo inventário: `Outfit` é só apresentação, não é salvo e não toca `Equipment`.
- Nenhuma persistência de aparência; o save continua v3.
- O jogador do jogo não usa a composição (o V2.2 e o N3.8 continuam como estavam).
- A corrida não foi modularizada: o contrato aceita o clipe 112×128 (pivô 56,118, 6 quadros; ver o teste de sincronização), mas não há folhas da corrida.
