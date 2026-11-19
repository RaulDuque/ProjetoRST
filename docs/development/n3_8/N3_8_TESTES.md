# N3.8 — Testes e validação (resultados reais)

## Comandos

| Comando | Resultado |
|---|---|
| `cargo fmt --check` | limpo |
| `cargo check -j 2` | limpo |
| `cargo clippy -j 2 --all-targets -- -D warnings` | limpo, 0 avisos |
| `cargo test -j 2` | **52 suítes, 641 passaram, 0 falharam, 0 ignorados** (N3.7: 51 suítes, 623) |
| `python3 -m unittest discover -s art-src/player/scripts/post/tests` | 21 testes OK (assets publicados conferidos com a trava) |
| `cargo run` (padrão) | abre o jogo com o V2.2, sem avisos de arte |
| `DARK_CASTLE_PROTAGONIST=n3 cargo run` | abre o jogo com o novo protagonista; log `no player socket table; no held equipment layers` (esperado) |
| `DARK_CASTLE_PROTAGONIST=n4 cargo run` | avisa `unknown DARK_CASTLE_PROTAGONIST value "n4" ...; using the production protagonist` e abre a produção |
| `cargo run --example n3_8_production_run` | `RESULT: PASS`, 446 capturas, 0 falhas (n3 405) |
| `cargo run --example n3_8_production_run -- --production` | `RESULT: PASS`, 446 capturas |
| `cargo run --example n3_8_production_run -- --view 360` | `RESULT: PASS`, 446 capturas |
| `cargo run --example n3_visual_proof` | 41 capturas regravadas, **idênticas** às versionadas (`git status` sem mudanças nas evidências N3–N3.7) |

As três variantes do exemplo N3.8 foram rodadas duas vezes, a pedido do proprietário, com o mesmo resultado. Na primeira repetição, a variante 360 foi chamada com o argumento errado (`"--view 360"` como uma palavra só, porque o zsh não divide `$a`). Ela gravou a vista 405 de novo. Isso foi notado pelo log (sem `cam=`), e a variante foi rodada outra vez com `-- --view 360`.

`cargo run` com janela foi aberto por 25 s em cada modo. A conferência visual do comportamento vem das capturas do exemplo, que monta o app com a **mesma** função (`app::build_for_protagonist`).

## Testes novos

`tests/n3_8_production_run_tests.rs` (13):

| Teste | Cobre |
|---|---|
| `the_staged_art_is_a_byte_identical_copy_of_the_approved_art` | integridade da arte (bytes iguais aos originais N3.5/N3.6) |
| `the_run_clip_is_six_frames_at_exactly_15_fps` | quantidade de quadros, duração individual e total, modo, sem `speed_reference`, fallback |
| `cells_pivots_and_scale_follow_the_approved_sources` | manifesto, células, pivôs, densidade, escala no mundo, tamanho dos PNG, só os clipes aprovados |
| `mirroring_keeps_the_pivot_and_the_sole_line` | espelhamento sem deslocamento, mesma linha da sola nos dois clipes |
| `no_socket_table_so_the_held_layers_stay_off` | fallback seguro de equipamento |
| `the_production_art_is_still_the_default_and_unchanged` | produção padrão; `run` V2.2 intacto (10 quadros, `speed_reference` 190) |
| `every_state_resolves_to_an_approved_clip_with_a_visible_marker` | todos os 12 estados caem em `idle`/`run`, nunca V2.2; dano, guarda, magia e morte mantêm o tom |
| `the_loop_is_stable_at_30_60_and_144_fps` | estabilidade em várias taxas, sem pular quadro, sem desvio |
| `the_loop_is_stable_with_a_variable_frame_rate` | jitter e engasgos |
| `idle_and_run_show_the_new_art_at_15_fps_and_310_units_per_second` | jogo headless: idle → corrida → virar → idle, 15 fps, 310 u/s, flip, âncora |
| `states_without_approved_art_hold_the_n35_pose_and_name_the_missing_clip` | corrida → salto, ataque, esquiva e guarda: pose N3.5 e `placeholder_for` |
| `gameplay_is_identical_with_the_production_and_the_experimental_art` | física, estados e posições idênticos em 220 updates |
| `equipment_is_kept_while_its_layers_stay_off_without_sockets` | arma equipada mantida, nenhuma camada visível |

Unitários: `art_profile` (3: padrão, valores conhecidos, valor desconhecido) e `art_badge` (1: texto do placeholder).

Nenhum teste anterior foi alterado ou enfraquecido.

## O que os testes headless NÃO provam

Que a imagem fica boa na tela. Isso vem das capturas reais (`N3_8_AVALIACAO.md`) e, no fim, do olhar do proprietário. A verificação pixel a pixel na vista 360 (24/24 capturas idênticas ao `run6.png`) prova que o jogo **desenha** a arte aprovada sem alterá-la, não que a arte está aprovada para todos os estados.
