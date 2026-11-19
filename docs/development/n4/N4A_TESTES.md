# N4-A — Testes (resultados reais)

| Comando | Resultado |
|---|---|
| `cargo fmt --check` | limpo |
| `cargo check -j 2` | limpo |
| `cargo clippy -j 2 --all-targets -- -D warnings` | limpo (rodado de novo com os arquivos N4 tocados, para forçar a reanálise) |
| `cargo test -j 2` | **53 suítes, 655 passaram, 0 falharam** (N3.8: 52 suítes, 641) |
| `cargo test -j 2 --test n4_sprite_machine_tests` (versão final do arquivo) | 6 passaram |
| `cargo run --example n4_sprite_machine` | `RESULT: PASS`, 11 capturas (a primeira tentativa gravou uma tela vazia porque as texturas ainda não estavam na GPU; foi acrescentado um aquecimento de 30 quadros e o roteiro foi rodado de novo) |
| `python3 -B art-src/n4-sprite-machine/scripts/build_idle_layers.py` | 0 diferenças na recomposição, 0 buracos |
| `python3 -B art-src/n4-sprite-machine/scripts/check_bevy.py` | 0 pixels diferentes nas 3 capturas comparadas |

## Testes novos

Unitários em `src/player/visual/composition.rs` (8):

| Teste | Cobre |
|---|---|
| `the_default_outfit_draws_back_to_front` | ordem de desenho |
| `every_layer_uses_the_same_frame` | sincronização (mesmo quadro para todas as peças; peças sem folha do clipe são puladas; quadro além do fim é limitado) |
| `removing_and_hiding_parts` | remoção, visibilidade, corpo-base não removível |
| `a_worn_part_hides_the_regions_it_covers` | ocultação (elmo → cabelo; elmo desligado não esconde) |
| `variants_swap_the_pieces` | variantes e variante desconhecida |
| `order_overrides_apply_to_their_clip_and_frames` | ordem por clipe e quadro |
| `invalid_compositions_are_refused` | validação (versão, camada repetida ou desconhecida, sem corpo-base, peça que esconde a si mesma, quadros fora da folha, roupa desconhecida, RON quebrado) |
| `unknown_clips_and_parts_are_errors_not_silent` | erros explícitos |

Integração em `tests/n4_sprite_machine_tests.rs` (6):

| Teste | Cobre |
|---|---|
| `the_proof_composition_loads_with_the_n35_canvas_and_pivot` | leitura do RON, canvas 80×128, pivô (40,118), densidade, camadas, peças removíveis, tamanho dos PNG, só chaves da paleta |
| `the_layers_recompose_the_approved_idle_texel_for_texel` | recomposição exata, na ordem do RON |
| `each_piece_is_independent_and_leaves_no_hole` | cada remoção muda só os texels da própria peça; nenhum buraco dentro do corpo; o corpo-base tem pele na cabeça, no torso, na coxa e no pé |
| `approved_art_and_older_manifests_are_unchanged` | integridade da arte N3.5 (hash da N3.7), manifestos V2.2 e N3.8 carregam, formatos não se confundem, save versão 3 |
| `every_layer_follows_the_one_visual_playback` | ECS: as camadas seguem `frame` e `flip` do `VisualPlayback`, pivô e ordem; remover uma peça esconde só a camada dela |
| `the_player_animation_fills_the_shared_frame_and_facing` | jogo headless (arte N3.8): `animate_player_visual` preenche `frame` (= índice − primeiro) e `flip` (= espelho do sprite) |

Nenhum teste anterior foi alterado ou enfraquecido.

## Segurança verificada

`git status` desde `2a2cc85` não mostra `assets/`, `Md-usario/`, `art-src/n3-5-fidelity`, `n3-6`, `n3-7`, `n3-8`, as evidências N3.7 e N3.8, save, combate, magia, movimento, constantes nem entrada. Os SHA-256 de `candidate_106.png`, `candidate_106.txt`, `player.ron` e `camera.ron` são os da auditoria.

## O que os testes não provam

Que a anatomia reconstruída está boa. Isso é avaliação artística do proprietário.
