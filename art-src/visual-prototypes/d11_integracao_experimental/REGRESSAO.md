# Regressão D11

## Reversibilidade

| Checagem | Resultado |
|---|---|
| `assets/animations/player.ron` intocado | SIM |
| `PlayerVisualPlugin` não registra modular lab | SIM |
| `cargo run` usa visual publicado | SIM (plugin path inalterado) |
| Assets experimentais sob `assets/experimental/d11/` | SIM |
| Exemplo isolado `modular_visual_lab` | SIM |

## Testes

| Comando | Resultado |
|---|---|
| `cargo test --lib modular_lab` | 5/5 OK |
| `cargo run --example modular_visual_lab` | 6 screenshots OK |
| `cargo fmt --check` | OK |
| `cargo clippy --all-targets -D warnings` | OK |
| `cargo check` | OK |

## Funcional (não alterado)

Gameplay, câmera, parallax, Save V3, slots de inventário: sem mudanças de código nestes módulos nesta entrega.
