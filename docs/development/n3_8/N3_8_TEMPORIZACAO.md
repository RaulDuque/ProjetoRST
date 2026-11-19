# N3.8 — Temporização de 15 fps

## Contrato

| Valor | Manifesto (`stage/animations/player.ron`) |
|---|---|
| Quadros | 6 (`count: 6`, `first: 0`) |
| Duração por quadro | `0.06666667` s (o `f32` mais próximo de 1/15) |
| Ciclo | 0,4 s (soma, testada com erro < 1e-6) |
| Modo | `Loop(fps: 15.0)` |
| Cadência | 5 passos por segundo (2 passos por ciclo) |
| `speed_reference` | `None` |

### Por que `speed_reference: None`

Com uma referência, a taxa de reprodução seguiria a velocidade (`playback::locomotion_rate`): passaria de 15 fps com bônus de movimento e cairia andando a 190 u/s. O proprietário pediu 15 fps e "não aumentar automaticamente o FPS". Sem referência, a taxa é fixa: 15 fps correndo (310 u/s), 15 fps andando (190 u/s) e 15 fps contra uma parede. Andando a 190 u/s o deslizamento estimado cai para |190 − 1,25 × 157| ≈ 6 u/s, porque o pé recua 196 u/s (`N3_7_TESTE_15FPS.md`). A física não mudou.

## Relógio

Não há relógio novo. É o mesmo de toda animação de loop:

- `VisualPlayback.elapsed += dt × taxa` (taxa = 1), uma vez por frame, em `animate_player_visual`;
- `loop_frame(durations, elapsed)` faz `elapsed.rem_euclid(total)` e procura o intervalo. Não há contador de quadros que acumule arredondamento, e o quadro sai sempre do tempo total;
- `dt` vem de `core::time::frame_delta`, limitado a 1/30 s. Como 1/30 < 1/15, um update nunca avança mais de um quadro, mesmo com engasgos;
- o ciclo só reinicia quando o clipe ou o estado muda (`playback.clip != key || playback.state != state`). Mudanças de velocidade não reiniciam o ciclo, e virar de lado também não (o teste confirma que o clipe continua `run` depois de virar).

## Medição no jogo real

`cargo run --example n3_8_production_run` (janela, updates fixos de 20 ms), sequência `run_right` (41 updates, 0,8 s):

| | N3 experimental | Produção V2.2 |
|---|---|---|
| Trocas de quadro | 12 | 32 |
| fps medido | **15,0** | 40,0 (24 fps × 310/190) |
| Permanência por quadro | 3 ou 4 updates de 20 ms | 1 ou 2 |
| Média | 65,5 ms (ideal 66,7; amostragem em múltiplos de 20 ms) | 25,2 ms |
| Velocidade | 310 u/s em todos os updates | 310 u/s |

Fonte: `evidence/measurements.json` e `target/n3_8_production_run/n3-405/frames.log`.

## Testes

`tests/n3_8_production_run_tests.rs`:

- `the_run_clip_is_six_frames_at_exactly_15_fps`: quantidade, duração individual, total, modo, sem referência, fallback;
- `the_loop_is_stable_at_30_60_and_144_fps`: 20 s a cada taxa; cada update é comparado com o quadro exato `floor(t × 15) mod 6` (fora de ±1 ms da borda), nunca pula quadro e faz 300 ± 1 trocas;
- `the_loop_is_stable_with_a_variable_frame_rate`: 3000 updates com jitter de 4 a 33 ms e engasgos de 250 ms (limitados a 1/30 s);
- `idle_and_run_show_the_new_art_at_15_fps_and_310_units_per_second`: no jogo headless, 3 s correndo: o quadro só avança, de 1 em 1, com 15 trocas por segundo (±1), e os 6 quadros aparecem.
