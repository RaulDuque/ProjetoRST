# N4-1-A — Arquitetura: equipamentos por dados sobre o personagem vestido

## Auditoria (estado inicial)

- Branch `art/v1.2-protagonist`, local = remoto = `bf121193fa03828dcc9e7de88d5251023c2a9e28` (registro da C1). Árvore limpa, exceto arquivos pessoais não rastreados em `Md-usario/`.
- Inspecionados `CharacterComposition`, `Outfit`, `resolve`, `CompositeCharacter`, `sync_composite_layers` e `VisualPlayback`.
- Os três quadros C1 (#0, #2, #4) estão em `stage/parts/run_c1/` (14 camadas, recomposição exata).
- O contrato de equipamento funcional (`inventory::equipment`, `items/*.ron`, `ItemVisual` com `Cape`/`Trail`/`Tint`) e o save v3 **não foram tocados**. O equipamento visual da N4-1-A é só apresentação.
- Hashes protegidos iguais aos das auditorias N4-A e C1 (teste `the_save_format_and_the_approved_art_do_not_move`).

## Princípio

A base é o **protagonista vestido aprovado**. A partição da N4-A/C1 já separa as peças visíveis da roupa (camisa do tronco, mangas, cabelo...) **sem** precisar de corpo nu, e é ela que torna possível ocultar só a camisa do tronco. A anatomia reconstruída da C1 continua nos dados, mas **nenhum equipamento depende dela**: o verificador recusa um item que, ao ocultar uma camada, deixe ver o que estava por baixo.

## Proposta → implementado

| Proposta do briefing | Implementado | Onde |
|---|---|---|
| ler a definição do personagem | `CharacterComposition::from_ron` (N4-A) | `composition.rs` |
| ler a definição do equipamento | `VisualFamily::from_ron`, `EquipmentVisual::from_ron` | `equipment_visual.rs` (novo, puro) |
| regiões cobertas | `covers: [Region]` na família (documentação + verificador) | idem |
| ocultar partes incompatíveis | **HIDE** por camada e por clipe: `hide_layers` | `PartDef` + `resolve_facing` |
| substituir | **REPLACE**: `replaces: [part]` | idem |
| desenhar por cima | **OVERLAY**: a peça no slot da família | `layer` da família |
| sincronizar com a animação | o item vira uma peça comum e usa o mesmo `VisualPlayback` | `equip` → `resolve` → `sync_composite_layers` |
| reutilizar entre itens da família | slot, perfil de animação, HIDE/REPLACE padrão, máscaras, exceções de ordem, regra de clipe ausente | `VisualFamily` |
| exportar prévias | PNG, spritesheet composta e GIF, **offline** | `scripts/build_equipment.py` |
| espelhamento | o item espelha com o corpo; folha espelhada própria opcional (`mirrored`) | `PieceDef.mirrored`, `DrawLayer.mirrored` |
| exceções por quadro | `frame_order` na família (compartilhado) e no item | `equipment_visual.rs` |
| fallback de recurso ausente | `missing_clip: Hide` (não desenha nesse clipe) ou `Refuse` (erro ao carregar) | idem |

## Mudanças no código Rust (todas genéricas, nenhuma por item)

- `composition.rs`: `PartDef.hide_layers`, `PartDef.replaces`, `PieceDef.mirrored` (todos opcionais; os RON antigos continuam válidos), validação desses campos, `resolve_facing` (aplica HIDE, REPLACE e a folha espelhada), `DrawLayer.mirrored` e um desempate de ordem pelo índice declarado da camada (antes era pelo nome).
- `equipment_visual.rs` (novo, puro): família, item, `equip` e erros descritivos (`EquipError`).
- `composite.rs`: usa `resolve_facing` com a direção do `VisualPlayback`; não espelha o sprite quando a folha já é espelhada.
- `protagonist.ron`: um slot de camada vazio, `armor_torso`, entre a camisa do tronco e o braço da frente.

Nenhum código de produção do jogo (V2.2, N3.8, inventário, save, física, combate) mudou. O `cargo run` padrão não carrega nada disso.

## Fluxo

```
protagonist.ron ─┐
families/*.ron ──┼─ equip() ── composição com itens como peças ── Outfit.wear(item) ── resolve_facing ── sync_composite_layers
items/*.ron ─────┘                       (validada)                                       (HIDE/REPLACE/ordem)   (um VisualPlayback)
```

Autoria (offline, fora do jogo):

```
families/<f>.json (regras das máscaras) ─┐
recipes/<item>.json (paleta e padrão) ───┼─ build_equipment.py ── máscaras da família + folhas do item (PNG + TXT)
camadas vestidas (stage/parts/) ─────────┘                       + verificação de cobertura + evidências + GIF/folha
```
