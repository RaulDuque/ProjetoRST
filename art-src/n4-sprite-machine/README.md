# N4-A: máquina de sprites (prova isolada no idle N3.5)

- `stage/protagonist.ron`: composição de dados do protagonista (6 peças, 8 camadas), lida por `src/player/visual/composition.rs`.
- `stage/parts/idle/<camada>.png` e `.txt`: uma folha por camada, no canvas 80×128 com pivô (40, 118); o `.txt` é a grade de paleta usada pelos testes.
- `scripts/partition.py`: separa cada texel do idle aprovado em uma camada (sem mudar nenhum).
- `scripts/reconstruct.py`: anatomia escondida, **interpretação do agente** (não aprovada).
- `scripts/build_idle_layers.py`: gera as folhas, confere a recomposição (0 diferenças exigidas) e faz as folhas de evidência em `docs/development/n4/evidence/`.
- `scripts/check_bevy.py`: compara original × modular nas capturas reais de `cargo run --example n4_sprite_machine`.

Ordem: `python3 -B scripts/build_idle_layers.py` → `cargo run --example n4_sprite_machine` → `python3 -B scripts/check_bevy.py` (todos a partir da raiz do projeto, com o caminho `art-src/n4-sprite-machine/scripts/...`).

A arte aprovada (`art-src/n3-5-fidelity/`) só é lida. Documentação: `docs/development/n4/`.
