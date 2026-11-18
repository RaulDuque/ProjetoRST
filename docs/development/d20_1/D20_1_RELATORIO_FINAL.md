# D20.1 — Relatório final

Validação real com janela: o jogo sobe, D17–D20 funcionam no mundo Vigil Gates, autoplay e `d20_1_probe` PASS. Sem P0/P1. Sem commit/push.

## A. Estado inicial

- Branch: `art/v1.2-protagonist`  
- HEAD: `a9b8f96`  
- Dirty: D19/D20 (código+docs) + briefing `DarkCastleR-d-20-1.md` + tooling D20.1  
- D19/D20 ainda **não** commitados nesta sessão  

## B. Ambiente

Ubuntu 26.04, RTX 4050 Vulkan, `DISPLAY=:0` / Wayland, Rust 1.99.1, Bevy 0.20.1, ~10 GiB RAM livre. Janela Bevy confirmada.

## C. Execução real

| Comando | Resultado |
|---|---|
| `timeout 20 cargo run` | Janela + `vigil_gates_approach` |
| `cargo run --example autoplay` | PASS 23/23, saves isolados |
| `cargo run --example d20_1_probe` | PASS, 15 marks, evidence/ |

Logs: `target/d20_1/*.log`, `evidence/route.log`.

## D. Testes visuais

Capturas reais em `docs/development/d20_1/evidence/` (start, inventário, altar, sentry, tonic, FT, mapa, shaft, bolt, warden). Ver `D20_1_TESTES_VISUAIS.md`.

## E. D17

Altar Approach (`vg_approach_save`): D → discover + rest + save isolado; sem menu FT com um só altar. Mapa marca altares.

## F. D18

Inventário I → Minor Tonic → Enter com HP baixo → bag 3→2. Persistência coberta por testes headless + save isolado do altar.

## G. D19

Segundo altar Causeway abre FT; destino Approach; após viagem `last_save_point` permanece `vg_causeway_save` (não cura/salva de novo).

## H. D20

Bone Sentry (causeway), Bone Bolt (shaft ledge y≈409), Ash Warden (gallery) — presentes, derrota do sentry/warden na rota; bolt observado sem Fight (risco de queda).

## I. Defeitos

Só P3 (cursor inventário, combate em ledge, arte bloco). Tabela: `D20_1_DEFEITOS.md`.

## J. Correções

Nenhuma correção de gameplay P0/P1. Tooling: Expects no `route_driver`, `d20_1_route` + `d20_1_probe`. Regressão fmt/check/clippy/test OK.

## K. Segurança

Sem commit/push/reset/clean. `saves/slot1.ron` mtime 2026-11-11. `Md-usario/` e `white_mesh.glb` intocados. Sem Hunyuan/Blender MCP.

## L. D21

Chefe em dados (`boss`), morte persistente, fases por HP ratio, arena plana trancada, barra UI. Detalhe: `D20_1_PREPARACAO_D21.md`.

## M. Arte futura

Mixamo + Hunyuan + Blender = pipeline **FUTURO não executado** nesta etapa.
