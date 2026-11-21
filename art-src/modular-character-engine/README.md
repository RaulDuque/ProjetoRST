# Modular Character Engine — protótipo 01 (laboratório isolado)

- `stage/rig_lab.ron`: marcos do esqueleto nos quadros aprovados (idle N3.5 e corrida N3.6 de
  6 quadros), clipes e itens presos a ossos. Lido por `src/player/visual/rig/data.rs`.
- `stage/equipment/pauldron_*_half.png`: ombreiras da fábrica N41-B reduzidas 2:1, derivadas
  e **provisórias**. São geradas por `scripts/derive_pauldrons.py`, que não cria cor nova.
- `scripts/make_evidence.py`: monta `evidence/modular_character_engine/01_prototipo/` a
  partir das capturas do Bevy, que ficam em `target/modular_character_engine/`.

A arte aprovada (`art-src/n3-8-production-run/stage/`) e as espadas da N41-B
(`art-src/n4-sprite-machine/stage/factory/isolated/`) são só lidas, por fontes de asset do
exemplo.

```bash
cargo run --example modular_character_engine \
    > target/modular_character_engine/run.log          # roteiro + verificações, RESULT: PASS
MCE_INTERACTIVE=1 cargo run --example modular_character_engine
~/.local/share/dc-art-tools/asf-venv/bin/python -B art-src/modular-character-engine/scripts/derive_pauldrons.py
~/.local/share/dc-art-tools/asf-venv/bin/python -B art-src/modular-character-engine/scripts/make_evidence.py
cargo test --test modular_character_engine_tests
```

O Python só roda na produção de assets e evidências. O jogo e o exemplo não dependem dele.
Documentação: `docs/development/modular_character_engine/01_ARQUITETURA.md`.
