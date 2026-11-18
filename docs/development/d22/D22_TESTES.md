# D22 — Testes

Cabeçalho desta rodada (o `cargo test` completo entra no relatório depois do clippy):

| Comando | Resultado |
|---|---|
| `cargo test --test boss_real_fight_tests` | 3 PASS (luta, guarda, esquiva) |
| `cargo test --test boss_encounter_tests` | 4 PASS (selo, fase por lógica, vitória/save, morte e retry) |
| `cargo test --test world_graph_tests` | 23 PASS |
| `cargo test --test world_integration_tests` | 17 PASS |
| `cargo test --test world_route_tests` | 2 PASS |
| `cargo run --example d22_boss_probe` | PASS |
| `cargo fmt` | PASS |
| `CARGO_BUILD_JOBS=2 cargo check` | PASS |
| `CARGO_BUILD_JOBS=2 cargo clippy --all-targets -- -D warnings` | PASS |
| `CARGO_BUILD_JOBS=2 cargo test` | PASS (nenhuma suíte falhou) |

Guarda (`A` segurado): o golpe frontal tira HP e o estado continua `Guarding`.

Esquiva (`S`, virado para longe, tell entre 0,18 s e 0,32 s): o HP não cai nesse golpe.

Morte e nova tentativa: `player_death_clears_lock_and_despawns_boss_for_retry` (chefe some, lock abre, reentrada com HP cheio, sem marca).

Persistência: `defeat_grants_reward_once_and_blocks_respawn` no save isolado do teste. A luta real também confere um `sentinel_brand` e o id em `defeated_bosses`. Save V3 não mudou de versão. `saves/` do projeto não foi tocado (mtime 2026-11-11).

Evidência gráfica: `docs/development/d22/evidence/01`–`06`. Saves do probe em `target/d22/saves/`.
