# D22 — Combate real

Prova: `tests/boss_real_fight_tests.rs` e `examples/d22_boss_probe.rs`. Os dois só apertam `X` (ataque). Nenhum escreve HP do chefe.

O script cura o jogador para a luta chegar ao fim. Isso está no teste (`top_up`) e no probe. Não é dano no sentinela.

Sequência observada:

1. Entrada em `vigil_gates_ashen_court` com a barra em 280/280 e a saída selada.
2. FrontalSlash resolve (`resolved_slash`).
3. Lunge resolve (`resolved_lunge`) sem o jogador cancelar o tell.
4. Fase 2 quando o HP cai abaixo da metade, com os dois golpes já feitos.
5. AshWave: `last_resolved == AshWave` ou um `Projectile` de `Team::Enemy`.
6. HP chega a 0 pelo mesmo pipeline. `BossEncounter` solta o lock, `defeated_bosses` ganha `ashen_sentinel`, inventário fica com um `sentinel_brand`.

Na janela, a barra cai 280 → 241 no primeiro ciclo e 129 na fase 2. A vitória mostra “Claimed Sentinel's Brand”, “The Ashen Sentinel falls” e o nível 2. A barra some.

O probe da D21 não entra nessa prova.
