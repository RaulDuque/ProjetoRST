# N4-1 (C1) — Evidências

Todas em `docs/development/n4/evidence/n4c/`, geradas a partir dos arquivos reais do projeto (grades aprovadas, `run6.png`, folhas das camadas e capturas do Bevy).

| Arquivo | Conteúdo | Gerado por |
|---|---|---|
| `n4c_skeletons.png` | esqueleto marcado à mão sobre #0, #2 e #4 | `draw_skeletons.py` |
| `n4c_recomposition.png` | original, modular e mapa de diferenças de cada quadro | `build_run_layers.py` |
| `n4c_layers.png` | as 14 camadas isoladas de cada quadro | idem |
| `n4c_removal.png` / `n4c_removal_light.png` | por quadro: original, modular, sem cabelo, sem camisa, sem calça, sem botas, sem braceletes, corpo-base (fundo escuro e claro) | idem |
| `n4c_new_holes.png` | as remoções com cavidades novas em vermelho (nenhuma depois do fechamento) | idem |
| `n4c_run_modular.gif` | corrida modular vestida, ciclo C1 de 0,4 s | `make_run_gifs.py` |
| `n4c_run_base_body.gif` | corpo-base em movimento | idem |
| `n4c_run_removal.gif` | as 7 versões animadas juntas | idem |
| `n4c_original_vs_modular.gif` | `run6` aprovado a 15 fps (6 quadros, 70/70/60 ms) ao lado da C1 vestida e do corpo-base (cada quadro C1 vale por dois) | idem |
| `bevy_n4c_f{0,2,4}_{dressed,no_hair,no_shirt,no_trousers,no_boots,no_bracers,base_body}.png` | 21 capturas do Bevy real: aprovado à esquerda, modular à direita | `cargo run --example n4_sprite_machine -- --run-c1` |
| `bevy_n4c_f2_mirrored.png` | os dois espelhados | idem |
| `bevy_n4c_f2_base_body_light_grid.png` | corpo-base, fundo claro, grade e pivô | idem |
| `metrics.json` | diferenças, silhueta, contato, pintura, cavidades, exposição, checagem do Bevy | scripts acima e `check_bevy.py` |

Atrasos de GIF são centésimos inteiros: 15 fps = 7/7/6 cs, e o ciclo C1 = 13/13/14 cs (0,40 s).

Não há GIF capturado do Bevy: o movimento foi demonstrado no Bevy com capturas por quadro e no modo interativo (P toca o ciclo). Os GIFs foram montados das folhas que o Bevy desenha, e as capturas confirmam que o Bevy desenha essas folhas pixel a pixel.
