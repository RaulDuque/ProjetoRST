# D22 — Auditoria

Branch `art/v1.2-protagonist`, HEAD `a9b8f96`. A árvore já estava suja com D19–D21. Nada disso foi revertido. Sem commit e sem push.

## O que a D21 já entregava

- `BossDefinition` no RON, `Boss` / `BossRuntime`, `BossEncounter` selando `TransitionLock`.
- Padrões FrontalSlash, Lunge e AshWave, fase 2 abaixo de 50% de HP.
- Barra de HP, `sentinel_brand` uma vez, `defeated_bosses` no Save V3 (sem bump).
- `boss_encounter_tests` e `examples/d21_boss_probe.rs`.

## Lacuna

O probe da D21 escreve `DamageDealt` direto. Isso não prova hitbox, telégrafo nem a fase 2 sob o ataque do jogador. Dano injetado continua só no teste de limiar (`phase_shifts_below_half_hp`), marcado como lógica.

## O que o combate real mostrou

1. Cada golpe cancelava o wind-up. O sentinela não terminava slash nem lunge. Chefe agora mantém o wind-up; inimigos comuns continuam perdendo o golpe.
2. O DPS do jogador atravessava a fase 2 antes do lunge. A fase 2 espera os dois golpes de corpo a corpo, ou HP abaixo de 25%.
3. A patrulha (x 920 ± 200) entrava no vão da porta do chão (x 540–740) e o chefe caía. Spawn passou para (1040, 72), patrulha ±80, ainda longe da chegada.
4. A porta do teto no Reliquary não é um pulo só. Há cinco plataformas one-way entre o chão e a porta.
