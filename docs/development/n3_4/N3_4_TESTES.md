# N3.4 — Testes

Executados na árvore de trabalho, um comando de cada vez, com `-j 2` (limite de memória desta máquina). Memória disponível antes de compilar: 8,6 GiB.

| Comando | Resultado |
|---|---|
| `cargo fmt --check` | sem diferenças |
| `cargo check -j 2` | sem erros |
| `cargo clippy --all-targets -j 2 -- -D warnings` | sem avisos |
| `cargo test -j 2` | 48 suítes, **607 passaram, 0 falharam** |
| `cargo test -j 2 --test n3_4_original_protagonist_tests` | 2 passaram |
| `cargo run --example n3_visual_proof` | todas as capturas gravadas, saída 0 |
| `python3 -I art-src/n3-4-original-protagonist/scripts/build_n3_4.py` | sprite, manifesto, métricas e 11 evidências gerados |

Antes do teste novo, a suíte tinha 47 suítes e 605 testes, todos passando.

## Teste novo: `tests/n3_4_original_protagonist_tests.rs`

- `the_base_is_one_idle_frame_without_equipment`: o manifesto `base.ron` tem um único clipe `idle` de um quadro, pivô `(40, 118)`, um único atlas (nenhum atlas de capa ou arma), `weapon_in_body: false`, e o PNG tem 80×128.
- `the_source_grid_keeps_the_n3_3_canvas_and_pivot`: a grade-fonte e o rascunho têm 80×128 e só usam chaves da paleta. O primeiro pixel opaco está na linha 13 e o último na 118 (106 px). O pivô x = 40 fica dentro do apoio dos pés. O rascunho não tem nenhum `M` nem `e`, a grade final tem, e as duas grades são diferentes.

Nenhum teste antigo foi alterado.

## Não executado

- `cargo run --example autoplay`: o jogo principal não mudou.
- `cargo build --profile dist`: não foi pedido.
- Avaliação humana do sprite: é o Gate A, do proprietário.
