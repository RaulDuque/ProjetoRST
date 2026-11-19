# N4-1 (C1) — Testes (resultados reais)

| Comando | Resultado |
|---|---|
| `cargo fmt --check` | limpo |
| `cargo check -j 2` | limpo |
| `cargo clippy -j 2 --all-targets -- -D warnings` | limpo (forçado a reanalisar tocando um arquivo da biblioteca, porque a primeira chamada terminou em 0,6 s sem reanalisar) |
| `cargo test -j 2` | **54 suítes, 660 passaram, 0 falharam** (N4-A: 53 suítes, 655) |
| `cargo test -j 2 --test n4c_run_c1_tests --test n4_sprite_machine_tests` (depois da correção dos braceletes, com os dados finais) | 5 + 6 passaram |
| `cargo run --example n4_sprite_machine` (idle, sem regressão) | `RESULT: PASS` |
| `cargo run --example n4_sprite_machine -- --run-c1` | `RESULT: PASS`, 23 capturas (dados finais) |
| `python3 -B .../build_run_layers.py` | 0 diferenças, 0 cavidades novas |
| `python3 -B .../check_bevy.py` | 0 pixels diferentes em 7 capturas comparadas (3 idle + 4 corrida) |

A suíte completa rodou antes da correção dos braceletes, que só mudou dados de arte da C1 (partição dos pulsos). Os dois testes que leem esses dados foram rodados de novo depois, junto com o exemplo e as evidências.

## Testes novos: `tests/n4c_run_c1_tests.rs` (5)

| Teste | Cobre |
|---|---|
| `the_c1_clip_uses_the_approved_run_canvas_and_pivot` | canvas 112×128, pivô (56,118), igual ao `run` da N3.8; 14 folhas de 3 colunas (336×128); só chaves da paleta |
| `each_c1_frame_recomposes_the_approved_frame_texel_for_texel` | recomposição exata dos 3 quadros; mesma linha de contato com o chão |
| `each_piece_is_independent_and_opens_no_cavity_in_any_frame` | cada remoção muda só os texels da peça; nenhuma cavidade nova; o corpo-base tem pele em > 90 linhas |
| `the_layers_change_frame_together_at_any_render_rate` | 30, 60 e 144 fps e taxa variável: as 14 camadas no mesmo quadro, espelho e pivô em todas as atualizações; nunca pula quadro; ciclo de 0,4 s |
| `removing_a_piece_during_the_run_hides_only_its_layers` | remoção durante a animação: só as camadas da peça somem; as outras ficam no quadro atual |

## Teste atualizado: `tests/n4_sprite_machine_tests.rs`

`the_proof_composition_loads_with_the_n35_canvas_and_pivot` conferia a lista exata das 8 camadas. Agora confere a lista exata das 14 **e** que o idle resolve as mesmas 8 camadas na mesma ordem de antes. As verificações de folha e de independência pulam as camadas que só existem na corrida. Nenhuma asserção foi enfraquecida: a lista continua exata, e a recomposição do idle continua texel a texel.

## Regressões verificadas

- Arte original: hashes da auditoria iguais; `n3_7_sync_tests` (hash da baseline) e `n4_sprite_machine_tests` (hash do N3.5) passam.
- Manifestos V2.2 e N3.8: carregam (teste).
- Idle N4-A: recomposição exata e captura do Bevy sem diferença.
- N3.8 (15 fps, 310 u/s, gameplay idêntico) e V2.2: suítes passando.
- Save v3, física e combate: sem mudança de código; suítes passando.
- Nenhum código Rust de produção mudou na N4-1.
