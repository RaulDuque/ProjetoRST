# N4-A: máquina de sprites (prova isolada no idle N3.5)

- `stage/protagonist.ron`: composição de dados do protagonista (6 peças, 14 camadas: 8 no idle e 14 na corrida C1), lida por `src/player/visual/composition.rs`.
- `stage/parts/idle/<camada>.png` e `.txt`: uma folha por camada, no canvas 80×128 com pivô (40, 118); o `.txt` é a grade de paleta usada pelos testes.
- `scripts/partition.py`: separa cada texel do idle aprovado em uma camada (sem mudar nenhum).
- `scripts/reconstruct.py`: anatomia escondida, **interpretação do agente** (não aprovada).
- `scripts/build_idle_layers.py`: gera as folhas, confere a recomposição (0 diferenças exigidas) e faz as folhas de evidência em `docs/development/n4/evidence/`.
- `scripts/check_bevy.py`: compara original × modular nas capturas reais de `cargo run --example n4_sprite_machine`.

N4-1 (C1, corrida #0, #2 e #4):

- `stage/parts/run_c1/<camada>.png` e `.txt`: tiras de 3 quadros (336×128), canvas 112×128, pivô (56, 118); 14 camadas, incluindo os membros de trás.
- `scripts/run_poses.py`: esqueletos marcados à mão por pose (interpretação).
- `scripts/build_run_layers.py`: partição, anatomia por pose (**interpretação do agente**), fechamento de cavidades, verificação (0 diferenças exigidas) e folhas de evidência em `docs/development/n4/evidence/n4c/`.
- `scripts/make_run_gifs.py`: GIFs a partir das tiras reais; `scripts/draw_skeletons.py`: esqueletos sobre os quadros.

Ordem: `python3 -B scripts/build_idle_layers.py` → `python3 -B scripts/build_run_layers.py` → `cargo run --example n4_sprite_machine` (e `-- --run-c1`) → `python3 -B scripts/check_bevy.py` → `python3 -B scripts/make_run_gifs.py` (todos a partir da raiz do projeto, com o caminho `art-src/n4-sprite-machine/scripts/...`).

A arte aprovada (`art-src/n3-5-fidelity/`) só é lida. Documentação: `docs/development/n4/`.
