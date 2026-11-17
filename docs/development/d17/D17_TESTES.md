# D17 — Testes

## Comandos

```bash
cargo fmt --check
CARGO_BUILD_JOBS=2 cargo check
CARGO_BUILD_JOBS=2 cargo clippy --all-targets -- -D warnings
CARGO_BUILD_JOBS=2 cargo test
```

## Resultados

| Comando | Resultado |
|---|---|
| `cargo fmt` | ok |
| `cargo check` | ok |
| `cargo clippy --all-targets -D warnings` | ok |
| `cargo test` | ok (incluindo `save_point_rests_discovers_and_persists`) |
| Unit `WorldProgress::discover_save_point` | ok |

## Novo teste

`tests/world_objects_tests.rs::save_point_rests_discovers_and_persists`

- Fere HP/MP, usa `D` no altar, verifica cura + descoberta + arquivo de save.
- Segunda interação: cura de novo, uma só entrada em `discovered_save_points`.
- F9 restaura a descoberta.

## Regressão notável

Posição inicial do altar em x≈120 colidia com fixture NPC dos testes de
cidade (`a_load_during_a_conversation_closes_it`). Altar movido para
x≈340–380.

## Não executado

- `cargo run` com janela (opcional; headless cobre o fluxo).
- Autoplay (sem mudança de rota obrigatória).
- Hunyuan / Blender (proibidos na D17).
