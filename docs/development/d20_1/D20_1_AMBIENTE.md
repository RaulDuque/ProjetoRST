# D20.1 — Ambiente

**Data:** 2026-11-17  
**Branch:** `art/v1.2-protagonist`  
**HEAD:** `a9b8f96f2fec20526059b4f16537a5eead3377ab`  
**Working tree:** dirty (D19 + D20 uncommitted + D20.1 probe/docs)

## Sistema

| Item | Valor |
|---|---|
| OS | Linux (Ubuntu 26.04), kernel 7.0.0-43-generic |
| CPU | 13th Gen Intel Core i5-13420H (8 cores) |
| RAM | 14.6 GiB (≈10 GiB disponíveis no início) |
| GPU | NVIDIA GeForce RTX 4050 Laptop (Vulkan, driver 580.178.04) |
| Display | `DISPLAY=:0`, `WAYLAND_DISPLAY=wayland-0` |
| Rust / Cargo | 1.99.1 |
| Bevy | `=0.20.1` |

## Janela gráfica

Confirmada. `cargo run` abriu `Dark Castle RPG` (~20 s com `timeout`), carregou `vigil_gates_approach`, adapter NVIDIA Vulkan.

## Ferramentas reutilizadas

| Ferramenta | Uso nesta etapa |
|---|---|
| `examples/autoplay.rs` | Combate/HUD/save isolado (fixture `entry_hall`) |
| `tests/support/route_driver.rs` | Input-only + Expect (estendido) |
| `examples/d20_1_probe.rs` (**novo**) | Mundo real + D17–D20 + screenshots |
| `tests/support/d20_1_route.rs` (**novo**) | Rota scriptada D20.1 |

## Limitações

- Blender do usuário estava aberto — não foi encerrado.
- Saves reais em `saves/` (mtime 2026-11-11) não foram tocados; probes usam `target/*/saves/`.
- Arte Hunyuan/Mixamo/Blender MCP: **não executados** (FUTURO).
- `/tmp` é tmpfs — builds só em `target/` da raiz, ≤4 jobs.
