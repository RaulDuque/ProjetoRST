# N3.8: novo protagonista experimental no jogo

- `stage/`: layout igual ao de `assets/`, lido por `DARK_CASTLE_PROTAGONIST=n3 cargo run` (`app::build_for_protagonist`).
  - `sprites/player/n35_idle.png`: cópia byte a byte de `../n3-5-fidelity/generated/candidate_106.png`.
  - `sprites/player/n36_run6.png`: cópia byte a byte de `../n3-6-animation-proof/generated/run6.png`.
  - `animations/player.ron`: `idle` (pivô 40,118) e `run` (6 × 1/15 s, pivô 56,118, sem `speed_reference`).
  - Sem `player_sockets.ron`: arma e escudo ficam ocultos até haver soquetes medidos.
- `scripts/make_evidence.py`: monta `docs/development/n3_8/evidence/` a partir das capturas de `cargo run --example n3_8_production_run` (e das variantes `-- --production` e `-- --view 360`).
- `scripts/measure_regions.py` → `analysis/regions.json`: regiões e punhos **estimados** nos quadros aprovados (estudo para a N4; não são soquetes).

Não edite os PNG: um teste (`tests/n3_8_production_run_tests.rs`) exige bytes iguais aos originais. Documentação: `docs/development/n3_8/`.
