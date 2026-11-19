# N4-A: máquina de sprites (prova isolada no idle N3.5)

- `stage/protagonist.ron`: composição de dados do protagonista (6 peças, 15 slots de camada: 8 no idle, 14 na corrida C1 e o slot de equipamento `armor_torso`), lida por `src/player/visual/composition.rs`.
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

N4-1-A (equipamento visual por dados, base vestida):

- `stage/equipment/families/<família>.ron` e `stage/equipment/items/<item>.ron`: manifestos lidos pelo jogo (`src/player/visual/equipment_visual.rs`).
- `stage/equipment/families/<família>/<clipe>_mask.png|txt`: silhueta compartilhada da família, gerada da arte vestida.
- `stage/equipment/art/<item>/<clipe>.png|txt`: folhas do item (mesmo canvas e pivô do clipe).
- `equipment/families/*.json` (regras das máscaras) e `equipment/recipes/*.json` (paleta e padrão de cada item): receitas de autoria.
- `scripts/build_equipment.py`: máscaras, folhas, verificação de cobertura (recusa HIDE que exponha algo), evidências, spritesheet composta e GIF em `docs/development/n4/evidence/n41a/`.

N41-B (fábrica de equipamentos orientada por dados):

- `factory/templates/`, `factory/materials/`, `factory/details/`, `factory/profiles/` (bibliotecas JSON) e `factory/recipes/<id>.json` (uma receita por item).
- `scripts/factory.py`: valida, gera (relevo, luz, rampa do material, contorno seletivo, detalhes), grava `stage/factory/` (`items/*.ron` do tronco, `art/`, `preview/`, `isolated/`, `relief/`, `provenance/`) com seed, proveniência e cache; `--rebuild` para sobrescrever.
- `scripts/factory_report.py`: QA técnico/artístico, diversidade, determinismo, desempenho, checagem das capturas do Bevy e evidências em `docs/development/n4/evidence/n41b/` (`--spotlight id ...` para uma folha de itens escolhidos).
- `scripts/tests/test_factory.py`: `python3 -B -m unittest discover -s art-src/n4-sprite-machine/scripts/tests`.
- Bevy: `cargo run --example n4_sprite_machine -- --factory [--run-c1]`.

Ordem: `python3 -B scripts/build_idle_layers.py` → `python3 -B scripts/build_run_layers.py` → `python3 -B scripts/build_equipment.py` → `cargo run --example n4_sprite_machine` (e `-- --run-c1`, `-- --equipment [--run-c1]`) → `python3 -B scripts/check_bevy.py` → `python3 -B scripts/make_run_gifs.py` (todos a partir da raiz do projeto, com o caminho `art-src/n4-sprite-machine/scripts/...`).

A arte aprovada (`art-src/n3-5-fidelity/`) só é lida. Documentação: `docs/development/n4/`.
