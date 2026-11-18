# N3.5 — Testes

Executados na árvore de trabalho, um comando por vez, com `-j 2`. Memória disponível antes: 9,9 GiB.

| Comando | Resultado |
|---|---|
| `cargo fmt --check` | sem diferenças |
| `cargo check -j 2` | sem erros |
| `cargo clippy --all-targets -j 2 -- -D warnings` | sem avisos |
| `cargo test -j 2` | 49 suítes, **611 passaram, 0 falharam** |
| `cargo test -j 2 --test n3_5_fidelity_tests` | 4 passaram |
| `cargo run --example n3_visual_proof` | 32 capturas, saída 0, capturas antigas idênticas |
| `python3 -I art-src/n3-5-fidelity/scripts/reduce_fidelity.py` | 3 reduções e 2 variantes de paleta |
| `python3 -I art-src/n3-5-fidelity/scripts/build_n3_5.py` | candidato, manifesto, métricas e 16 folhas |

Antes desta etapa: 48 suítes e 607 testes.

## Teste novo: `tests/n3_5_fidelity_tests.rs`

- `the_candidate_is_one_idle_frame_at_density_two_without_equipment`: um clipe `idle` de um quadro, um único atlas, `density: 2`, pivô `(40, 118)` e PNG de 80×128 (carregamento e dimensões).
- `the_candidate_keeps_the_canvas_pivot_and_stays_the_reduction`: grade 80×128 só com chaves da paleta, figura nas linhas 13 a 118, pivô dentro do apoio dos pés, e entre 1 e 16 pixels diferentes da redução.
- `the_resolution_hypotheses_have_their_own_canvases`: 80×128, 96×160 e 72×112.
- `the_n3_4_base_is_still_loadable_and_unchanged_in_shape`: compatibilidade com a prova anterior.

A transparência foi verificada no build: os valores de alfa são só 0 e 255 (`metrics.json`). A escala foi verificada no Bevy, pela comparação pixel a pixel da captura 360. Nenhum teste antigo foi alterado.

## Não executado

- `cargo run --example autoplay`: o jogo principal não mudou.
- `cargo build --profile dist`: não foi pedido.
