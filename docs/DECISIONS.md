# Registro de decisões

## D-001 — Bevy 0.20.1 com features mínimas
Versão estável mais recente no crates.io em 2026-11-11 (MSRV 1.97.1).
Usamos `default-features = false` com `2d`, `ui` e `audio`: o jogo não tem 3D,
então evitamos compilar PBR, glTF etc. Versões fixadas com `=` no `Cargo.toml`
e `Cargo.lock` versionado.

## D-002 — Dependências diretas
`serde 1.0.229`, `ron 0.12.3`, `thiserror 2.0.21`, `tracing 0.1.44` — as
mesmas versões que o próprio Bevy 0.20.1 usa (sem duplicação no grafo).
`tracing` é usado diretamente nos módulos puros para não depender de Bevy.
**Não adicionados:** `bevy-inspector-egui` (0.38 não foi validado com Bevy 0.20 e
não é necessário ainda), crates de tilemap (salas simples em RON bastam),
`rand` (um SplitMix64 de 20 linhas em `core/rng.rs` é suficiente e determinístico).

## D-003 — RON para dados e saves
Formato nativo do ecossistema Rust/Bevy, legível, com enums e structs.
Comandos de habilidades usam uma string de notação (`"D DF F + ATTACK"`, ou
setas `"↓ ↘ → + ATTACK"`) convertida por `TryFrom<String>` — erros de digitação
são detectados no carregamento e no `cargo test` (`tests/data_tests.rs`).

## D-004 — Regras puras separadas dos sistemas Bevy
Fórmulas, parser, progressão, inventário e save não importam Bevy. Ganhos:
testes rápidos e determinísticos, regras reutilizáveis em ferramentas, e
nenhuma lógica de negócio presa a sistemas de renderização.

## D-005 — Física própria (AABB) em vez de engine de física
Plataforma 2D precisa de colisão de caixas, plataformas atravessáveis e
resposta previsível. `world/collision.rs` resolve por eixo; uma engine de
física (avian/rapier) traria peso e comportamento menos "arcade".

## D-006 — Comandos relativos à orientação
O personagem vira para a direção pressionada **antes** da gravação do comando
(`update_facing` roda antes de `record_and_resolve_commands`). Assim "→" é
sempre "para frente" do ponto de vista do jogador. Consequência: "trás" só é
gravado quando o personagem não pode virar (atacando, guardando...).

## D-007 — Ajustes na estrutura de pastas do briefing
- `core/time.rs` contém só o *clamp* do delta (evita atravessar plataformas em travadas).
- Não criados (sem finalidade ainda): `combat/combos.rs`, `combat/combat_state.rs`
  (o estado de combate é a `PlayerStateMachine` em `player/state.rs`),
  `skills/components.rs` (o componente `SkillBook` está em `player/components.rs`),
  `magic/spells.rs` (magias usam `SkillDefinition`), `inventory/equipment_slots.rs`
  (slots ficam em `equipment.rs`), `world/transitions.rs`, `ui/menus.rs`,
  `ui/inventory_screen.rs`, `ui/skill_screen.rs` (a lista de habilidades está na tela de status).
- Adicionados: `input/` (mapeamento de teclas), `player/state.rs`, `world/{physics,camera,progress}.rs`,
  `enemies/definitions.rs`, `ui/{theme,damage_numbers}.rs`, `save/systems.rs`,
  `examples/autoplay.rs`, `tests/{data,inventory,save,app_smoke}_tests.rs`.
- Pastas de assets sem arquivos não foram criadas (git não versiona pastas vazias); ver `assets/README.md`.

## D-008 — Lints do clippy
`unwrap_used`/`expect_used` = warn. `type_complexity` e `too_many_arguments`
liberados: parâmetros de sistemas Bevy (queries com filtros) disparam esses
lints por natureza, e o próprio projeto Bevy os desativa.

## D-009 — Hitboxes perfurantes no corpo a corpo
Golpes corpo a corpo atingem cada alvo uma vez (`already_hit`) e podem atingir
vários alvos; projéteis (`pierce: false`) somem no primeiro acerto.

## D-010 — Teste de fumaça visual
`examples/autoplay.rs` injeta teclas no app real após `InputSystems`, registra
resultados e salva screenshots com a API de screenshot do Bevy. Complementa
`tests/app_smoke_tests.rs` (headless, roda no `cargo test`).
