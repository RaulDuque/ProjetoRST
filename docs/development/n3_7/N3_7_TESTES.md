# N3.7 — Testes

Executados na árvore de trabalho, um comando por vez, com `-j 2`. Memória disponível antes: 10 GiB.

| Comando | Resultado |
|---|---|
| `cargo fmt --check` | sem diferenças |
| `cargo check -j 2` | sem erros |
| `cargo clippy --all-targets -j 2 -- -D warnings` | sem avisos |
| `cargo test -j 2` | 51 suítes, **623 passaram, 0 falharam** |
| `cargo test -j 2 --test n3_7_sync_tests` | 7 passaram |
| `cargo run --example n3_visual_proof` | 47 capturas, saída 0; as 41 antigas saíram com bytes idênticos |
| `animation_pipeline.py configs/run_n36.json configs/run_start.json` | 189 quadros decodificados; 13 e 14 quadros reduzidos; 13/13 grades N3.6 idênticas |
| `foot_analysis.py` | referência 157 u/s (passos de 150 e 164) |
| `sync_study.py` | 4 cenários, `sync.ron`, 2 GIFs e 2 folhas |
| `contrast_study.py` | 512 texels de borda e 1897 de bota alterados (em arquivos novos), métricas, 3 folhas e 1 GIF |
| `transition_study.py` | quadro de saída `run6` #3, `transition.ron`, GIF e folha |
| `pipeline_evidence.py` | 13/13 idênticas |
| comparação captura × sprite (câmera 360) | 0 diferenças em 2458 texels |

Antes desta etapa: 50 suítes e 616 testes.

## Teste novo: `tests/n3_7_sync_tests.rs`

- `the_approved_n35_and_n36_art_is_byte_identical`: FNV-1a 64 de 21 arquivos aprovados. Preserva os seis quadros, o manifesto, os pivôs, as dimensões, as durações e o loop da N3.6 e o candidato da N3.5.
- `the_pipeline_reproduces_the_n36_run_texel_for_texel`: as 13 grades refeitas são iguais às aprovadas, e os clipes `run6` e `run10` têm a mesma contagem, o mesmo pivô e as mesmas durações.
- `the_real_start_is_a_valid_once_clip_on_the_run_canvas`: `run_start.ron` é um `AnimationManifest` válido, com 6 quadros, `Once`, fallback idle, 0,5 s, pivô (56, 118), célula 112×128, densidade 2 e PNG do tamanho declarado. Toda grade só usa a paleta e não passa do chão.
- `every_pipeline_frame_records_its_provenance`: para cada quadro extraído das duas configurações, índice, tempo, `manual_edit: false`, `agent_generated: false` e `source_ai_generated: true`, além da versão do pipeline, do hash da fonte e de `NOT APPROVED`.
- `the_speed_scenarios_follow_the_measured_stride`:
  - a velocidade do jogo de `sync.ron` é `PLAYER_RUN_SPEED`;
  - a referência fica entre 100 e 200 u/s e é a mesma de `foot_metrics.json`;
  - A desliza mais de 100 u/s, e B, C e D deslizam menos de 1;
  - 6 fases, com uma de voo.
- `the_transition_and_contrast_data_fit_the_approved_run`: plano de transição coerente, variantes do tamanho do `run6`, contraluz de 112×128.
- `the_game_camera_and_run_speed_are_untouched`: `camera.ron` continua com 405 e `PLAYER_RUN_SPEED` com 310.

Nenhum teste antigo foi alterado. Os testes da N3.6 (`n3_6_run_proof_tests.rs`) continuam passando, então a compatibilidade com a N3.5 e a N3.6 se mantém.

## Não executado

`cargo run --example autoplay` (o jogo não mudou) e `cargo build --profile dist` (não foi pedido).
