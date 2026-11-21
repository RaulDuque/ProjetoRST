# Modular Character Engine — laboratório isolado (protótipos 01 e 02)

- `stage/rig_lab.ron`: marcos do esqueleto nos quadros aprovados (idle N3.5 e corrida N3.6 de
  6 quadros), clipes e itens presos a ossos. Lido por `src/player/visual/rig/data.rs`.
- `stage/equipment/pauldron_*_half.png`: ombreiras da fábrica N41-B reduzidas 2:1, derivadas
  e **provisórias**. São geradas por `scripts/derive_pauldrons.py`, que não cria cor nova.
- `scripts/make_evidence.py`: monta `evidence/modular_character_engine/01_prototipo/` a
  partir das capturas do Bevy, que ficam em `target/modular_character_engine/`. Ele foi
  escrito para o roteiro da 01 e não serve para o roteiro atual.
- 02:
  - `stage/history/rig_lab_01.ron`: os dados da 01, preservados para as métricas;
  - `stage/layers/run_{0,2,4}.txt`: mapas de camadas gerados das camadas da N4-C por
    `scripts/n4c_label_maps.py`;
  - `scripts/run_external_tools.py`: execuções práticas do ASF e do pixelfx (com
    `scripts/tools/asf_rig_crosscheck.py`, `asf_motion_measure.py` e `pixelfx_probe.mjs`);
  - `scripts/make_evidence_02.py`: monta `evidence/modular_character_engine/02_equipamentos/`.

A arte aprovada (`art-src/n3-8-production-run/stage/`) e as espadas da N41-B
(`art-src/n4-sprite-machine/stage/factory/isolated/`) são só lidas, por fontes de asset do
exemplo.

```bash
cargo run --example modular_character_engine \
    > target/modular_character_engine/run.log          # roteiro + verificações, RESULT: PASS
MCE_INTERACTIVE=1 cargo run --example modular_character_engine
~/.local/share/dc-art-tools/asf-venv/bin/python -B art-src/modular-character-engine/scripts/derive_pauldrons.py
cargo test --test modular_character_engine_tests -- --nocapture | grep -E "spread|IoU|clamped|test result" \
    > target/modular_character_engine/test_metrics.log
PATH=~/.local/share/dc-art-tools/asf-venv/bin:$PATH python -B art-src/modular-character-engine/scripts/run_external_tools.py
~/.local/share/dc-art-tools/asf-venv/bin/python -B art-src/modular-character-engine/scripts/make_evidence_02.py
```

O Python só roda na produção de assets e evidências. O jogo e o exemplo não dependem dele.
Documentação: `docs/development/modular_character_engine/01_ARQUITETURA.md` e `02_EQUIPAMENTOS.md`.
