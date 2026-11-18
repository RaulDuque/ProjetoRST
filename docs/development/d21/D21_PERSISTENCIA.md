# D21 — Persistência

Save **V3** inalterado. Em `WorldProgress.defeated_bosses` + `flags`:

1. Primeira morte → `record_defeat` + `sentinel_brand` + flag.
2. Spawn consulta `should_spawn_boss` — derrotado não reaparece.
3. Segunda morte impossível sem trampas; recompensa idempotente.
4. Morte do jogador: limpa encontro, despawna boss vivo; reentrar reinicia HP cheio.
