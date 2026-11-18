# N3.6 — Testes

Executados na árvore de trabalho, um comando por vez, com `-j 2`. Memória disponível antes: 10 GiB.

| Comando | Resultado |
|---|---|
| `cargo fmt --check` | sem diferenças |
| `cargo check -j 2` | sem erros |
| `cargo clippy --all-targets -j 2 -- -D warnings` | sem avisos |
| `cargo test -j 2` | 50 suítes, **616 passaram, 0 falharam** |
| `cargo test -j 2 --test n3_6_run_proof_tests` | 5 passaram |
| `cargo run --example n3_visual_proof` | 41 capturas, saída 0; as antigas saíram idênticas |
| `python3 -I art-src/n3-6-animation-proof/scripts/segment_sheets.py` | 8, 19, 12, 19 e 31 poses |
| `python3 -I art-src/n3-6-animation-proof/scripts/extract_video.py` | 189 quadros decodificados, 13 recortados |
| `python3 -I art-src/n3-6-animation-proof/scripts/build_run.py` | atlas, manifesto, caixas, métricas, 5 GIFs e as folhas da corrida |
| `python3 -I art-src/n3-6-animation-proof/scripts/analyze_references.py` | 6 folhas de análise |
| comparação pixel a pixel captura × sprite (câmera 360) | 0 diferenças nos 6 quadros |

Antes desta etapa: 49 suítes e 611 testes.

## Teste novo: `tests/n3_6_run_proof_tests.rs`

- `the_manifest_has_a_stand_frame_and_two_run_cycles`: manifesto válido, clipes `idle`/`run6`/`run10` com 1, 6 e 10 quadros, pivô (56, 118), modo `Loop`, densidade 2, célula 112×128 e PNG do tamanho que o atlas declara (dimensões e quantidade de quadros).
- `both_run_cycles_last_one_video_cycle`: os dois ciclos somam 0,5 s, e o `run6` tem todas as durações iguais (sequência de reprodução).
- `every_kept_frame_shares_canvas_ground_and_head_height`: as grades 112×128 só usam chaves da paleta, nenhum quadro passa da linha do chão, pelo menos 4 tocam o chão, o topo da cabeça varia no máximo 4 linhas (uma escala só) e todo quadro do `run6` é um dos quadros mantidos.
- `the_overlay_boxes_match_the_clips`: `boxes.ron` tem uma caixa por quadro, todas dentro do canvas.
- `the_n3_5_candidate_still_loads`: compatibilidade com a N3.5.

**Transparência:** a grade é a fonte, cada célula é transparente ou opaca, então o alfa do PNG é binário por construção.

**Integridade dos originais:** o SHA-256 é conferido no fim (`N3_6_REFERENCIAS.md`). Não há teste em Rust para isso, porque exigiria uma dependência nova.

Nenhum teste antigo foi alterado.

## Não executado

`cargo run --example autoplay` (o jogo não mudou) e `cargo build --profile dist` (não foi pedido).
