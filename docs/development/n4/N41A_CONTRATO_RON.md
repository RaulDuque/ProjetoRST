# N4-1-A — Contrato RON de equipamento visual

Leitor: `src/player/visual/equipment_visual.rs` (`VisualFamily::from_ron`, `EquipmentVisual::from_ron`, `equip`). Os exemplos reais estão em `art-src/n4-sprite-machine/stage/equipment/`.

## Família (`families/<id>.ron`)

```ron
(
    version: 1,
    id: "torso_plate",
    region: Chest,                       // Region do contrato N4-A
    layer: "armor_torso",                // slot da composição base onde os itens são desenhados
    clips: ["idle", "run_c1"],           // perfil de animação
    hide_layers: { "run_c1": ["shirt_torso"] },   // HIDE padrão por clipe (opcional)
    replaces: [],                        // REPLACE padrão (opcional)
    covers: [Chest],                     // documentação e verificação (opcional)
    masks: { "idle": (texture: "..."), "run_c1": (texture: "...", columns: 3) },  // autoria (opcional)
    missing_clip: Hide,                  // Hide | Refuse
    frame_order: [(clip: "idle", z: 12)],         // exceções de ordem compartilhadas (opcional)
)
```

## Item (`items/<id>.ron`)

```ron
(
    version: 1,
    id: "iron_cuirass",                  // em geral o id do item do inventário que ele veste
    family: "torso_plate",
    sheets: { "idle": (texture: "..."), "run_c1": (texture: "...", columns: 3) },
    mirrored: { },                       // folhas desenhadas para o lado esquerdo (opcional)
    variants: { },                       // nome -> folhas por clipe (opcional)
    hide_layers: None,                   // Some({...}) substitui o HIDE da família (opcional)
    replaces: None,                      // Some([...]) substitui o REPLACE da família (opcional)
    palette: Some("iron"),               // nome da paleta da receita (documentação)
    frame_order: [],                     // exceções de ordem do item (opcional)
)
```

Todas as folhas usam a célula e o pivô **do clipe da composição base** (80×128 / 40,118 no idle; 112×128 / 56,118 na corrida). Assim o item fica alinhado ao personagem por construção, sem pontos de encaixe.

## Validação (antes de desenhar)

`equip` recusa, com mensagem que nomeia o item ou a família e o campo:

| Erro | Quando |
|---|---|
| `Parse`, `Version` | RON inválido ou versão ≠ 1 |
| `DuplicateFamily` | duas famílias com o mesmo id |
| `FamilyLayer` / `FamilyClip` | o slot ou um clipe do perfil não existe na composição base |
| `UnknownFamily` | o item aponta para uma família inexistente |
| `ExtraClip` | o item tem folha de um clipe fora do perfil |
| `MissingClip` | falta um clipe do perfil e a família usa `Refuse` |
| `DuplicateItem` | id repetido ou igual a uma peça da base |
| `Composition` | a composição resultante é inválida: folha fora da tira, HIDE de uma camada inexistente ou do próprio slot, REPLACE do corpo-base ou de si mesmo, exceção de ordem inválida |

## Diferenças em relação ao exemplo JSON do briefing

| Briefing (ilustrativo) | Contrato real | Por quê |
|---|---|---|
| `asset` | `sheets` por clipe (+ `mirrored`, `variants`) | um item precisa de uma folha por clipe, com o mesmo canvas da animação |
| `regions` | `covers` na família | as regiões são da família; o item herda |
| `composition.mode: "overlay"` | sempre OVERLAY no slot; HIDE e REPLACE são campos | os três modos combinam (sobrepor **e** ocultar), então são campos, não um modo exclusivo |
| `composition.hide` | `hide_layers` **por clipe** | o idle tem a camisa em uma camada só (com a manga): ocultá-la lá abriria buracos |
| `draw_order` | `layer` da família + `frame_order` | ordem é por slot da composição, com exceções por clipe e quadro |
| `animation.profile` / `frame_mapping: "shared"` | `clips` da família; o quadro sempre vem do `VisualPlayback` | não há outro mapeamento: um relógio só |
| `palette` | `palette` (nome) + receita JSON | a paleta é dado de autoria; o jogo desenha PNG |

## Compatibilidade

- Os campos novos de `PartDef` e `PieceDef` têm padrão: `protagonist.ron` da N4-A/C1 e os testes antigos continuam válidos.
- Os itens não entram na roupa padrão: sem `Outfit::wear(item)`, o personagem é o aprovado, texel a texel (teste).
- Não há inventário novo nem campo novo no save.
