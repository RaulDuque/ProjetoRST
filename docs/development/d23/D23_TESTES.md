# D23 — Testes

Um comando Cargo por vez, `CARGO_BUILD_JOBS=2`. Memória disponível antes
da suíte: ~11 GiB. `target/` da raiz, sem `cargo clean`.

| Comando | Resultado |
|---|---|
| `cargo fmt --check` | exit 0 |
| `cargo check` | exit 0 |
| `cargo clippy --all-targets -- -D warnings` | exit 0 |
| `cargo test` | exit 0, ~423 s, nenhuma suíte com falha |
| `cargo run --example d23_play_probe` | exit 0, derrota registrada |

Testes novos desta etapa, os dois dentro da suíte:

- `jumps_up_the_reliquary_stairs_into_the_court` — só Z a partir do pé da
  escada, zero reposições artificiais, chegada no chão da corte.
- `boss_tell_counts_down_while_the_hurt_flash_is_up` — wind-up 0,50 com
  flash 0,20 desce em 4 frames e não é apagado.

Reusados e ainda verdes: `guard_chips_a_frontal_hit_without_stagger`,
`dodge_iframes_cover_one_committed_swing`,
`player_attacks_deal_real_damage_and_resolve_all_patterns` (esse enche a
vida; é lógica de padrão, não a luta sem cura), `boss_encounter_tests`
(a fase 2 daquele arquivo também pré-marca os padrões; continua só lógica).

A luta sem cura é o probe, separado:

```text
RESULT: player died frames=1074 attacks=14 dodges=5
        player 115->15 (low 15) boss 280->13 (low 13)
        slash=true lunge=true wave=true phase2=true
```

O "15" é a última amostra com o jogador vivo. `10_defeat.png` está em
0/115. O chefe terminou em 13.
