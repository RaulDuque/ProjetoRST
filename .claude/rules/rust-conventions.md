---
paths:
  - "src/**/*.rs"
  - "tests/**/*.rs"
  - "examples/**/*.rs"
---

# Convenções Rust

- Edição 2024, toolchain stable (MSRV no `Cargo.toml`: `rust-version`).
- Nomes em inglês; comentários de documentação (`//!`, `///`) em inglês no código.
- `unsafe` é proibido (`unsafe_code = "forbid"`).
- `unwrap`/`expect` geram aviso do clippy: use `?`, `let ... else`, `match`, `Option` combinators.
- Erros de domínio com `thiserror` (`#[derive(Error)]`). Erros grandes (ex.: `ron::SpannedError`) vão em `Box`.
- Logs: `tracing` em módulos puros; `bevy::log` (`info!`, `warn!`...) em sistemas Bevy.
- Prefira enums a strings para estados e categorias; ids de conteúdo de dados são `String`.
- Campos privados + métodos quando há invariantes (ex.: `ResourcePool`, `CharacterSheet`, `Progression`).
- Sistemas Bevy: parâmetros mínimos; `Single<...>` para o jogador; `Query::get`/`get_mut` com `let Ok(..) else`.
- `type_complexity` e `too_many_arguments` estão liberados no `Cargo.toml` por causa dos parâmetros de sistemas Bevy. Não use isso como desculpa para funções puras gigantes.
- Remover entidades: `try_despawn()` quando outra parte do frame também puder removê-la.
- Formatação: `cargo fmt` (padrão, sem rustfmt.toml).
