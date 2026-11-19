# N4-1 (C1) — Sincronização

## Uma fonte do quadro

Nada novo foi criado. O quadro e o espelho vêm de `VisualPlayback.frame` e `.flip` (N4-A), e `sync_composite_layers` aponta as 14 camadas para o mesmo índice `first + frame` da folha de cada uma (todas são tiras de 3 colunas), com a mesma âncora do pivô (56, 118) e o mesmo espelho.

## Tempo do clipe C1

C1 tem 3 dos 6 quadros. Para manter o ciclo da corrida aprovada (0,4 s), cada quadro C1 dura 2/15 s (7,5 trocas por segundo). O quadro k da C1 é o quadro 2k do `run6`. A corrida completa a 15 fps (6 × 1/15 s) é a C2, bloqueada até a aprovação da C1. Na prova, o quadro vem da **mesma** função do jogador (`playback::loop_frame`) com durações `[2/15; 3]`.

Os 15 fps do jogo não mudaram: o clipe `run` da N3.8 continua 6 × 1/15 s (testes `n3_8_production_run_tests`), e o teste da N4-A com 6 quadros (`every_layer_follows_the_one_visual_playback`) já mostra camadas seguindo 6 quadros.

## Testes (`tests/n4c_run_c1_tests.rs`)

`the_layers_change_frame_together_at_any_render_rate`, num app ECS sem janela, com a composição real e as 14 camadas:

| Taxa | Atualizações | Verificado em cada atualização |
|---|---|---|
| 30 fps | 120 | as 14 camadas visíveis, todas com índice = quadro, mesmo espelho, mesma âncora |
| 60 fps | 240 | idem |
| 144 fps | 576 | idem |
| variável (4–33 ms, engasgos de 250 ms limitados a 1/30 s) | 300 | idem |

Em todas: o quadro só avança de 1 em 1 (nunca pula), e o número de trocas = tempo / (2/15 s) ± 1. Nenhuma camada adianta ou atrasa: o teste falha se uma única camada mostrar outro índice em uma única atualização. O espelho alterna a cada 50 atualizações, e todas as camadas seguem.

`removing_a_piece_during_the_run_hides_only_its_layers`: com a animação em andamento, tirar cada peça esconde exatamente as camadas dela (cabelo 2, camisa 3, calça 2, botas 2, braceletes 2), e as outras continuam no quadro atual.

## No Bevy real

`cargo run --example n4_sprite_machine -- --run-c1`: o quadro aprovado (`run6` #2k) à esquerda e o modular à direita, trocando juntos. Com `N4_INTERACTIVE=1`, P toca o ciclo C1 e Esquerda/Direita passam os quadros.
