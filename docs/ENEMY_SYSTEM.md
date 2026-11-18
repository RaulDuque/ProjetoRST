# Inimigos

Código: `src/enemies/` — `ai.rs` e `definitions.rs` (puros), `components.rs`,
`spawning.rs`, `behavior.rs`. Dados: `assets/data/enemies/*.ron`; posições nas salas
(`assets/maps/*.ron` → `enemies`).

## Definição (`EnemyDefinition`)

HP, MP, ATK, DEF, MDEF, HIT, esquiva, resistências (negativas = fraquezas),
resistências a status, velocidade, tamanho, cooldown e hitbox de ataque, dano por
contato, EXP, tabela de drops, perfil de IA, cor provisória, tempo de respawn.

## IA (`ai.rs`, pura)

`decide(profile, perception) → EnemyBehavior`:
- sem jogador visível (ou morto) → `Patrol`;
- HP abaixo de `flee_below_hp` → `Flee`;
- distância > alcance (aggro para começar, *leash* para continuar) → `Patrol`;
- conforme `AiStyle` (D20):
  - `melee` — dentro de `attack_range` → `Attack`/`Wait`; senão `Chase`;
  - `ranged` / `keep_distance` — abaixo de `min_range` → `Retreat`; dentro de
    `attack_range` → `Attack`/`Wait`; acima de `preferred_range` → `Chase`;
    senão `Defend`.

`projectile_speed > 0` faz o ataque spawnar um `Projectile` (para em sólidos).
`windup` (segundos) configura o telegraph (cor laranja).

Comportamentos no enum: Patrol, Chase, Attack, Defend, Flee, Wait, Retreat,
UseSkill. A IA nunca lê entrada do jogador; só posição e se ele está vivo.

## Ciclo de vida

Spawn a partir da sala → recebe `DamageDealt` (pisca, recuo pela intensidade do
acerto — V2.4, `Medium` = 160 —, cancela o ataque; sem invulnerabilidade, todos
os acertos do frame somam salvo `SimultaneousHitPolicy::HighestDamagePerFrame`) →
HP 0 → `EnemyDefeated` (EXP + drops) → respawn após `respawn_time` no ponto de origem.

## Protótipo / D20

| Id | Estilo | Onde |
|---|---|---|
| `bone_sentry` | melee | Causeway / Galleries |
| `bone_bolt` | ranged | Watch Shaft (ledge) |
| `ash_warden` | keep_distance | Upper Gallery |

## Chefes (D21, validado em D22)

Campo opcional `boss: Some(BossDefinition)` na definição: limiar de fase,
AI/hitbox da fase 2, recompensa garantida e flag. Marcadores `Boss` /
`BossRuntime`; `BossEncounter` sela `TransitionLock`. Spawn respeita
`WorldProgress.defeated_bosses`. Barra: `ui/boss_hud.rs`.

D22: o wind-up do chefe não é cancelado por dano (inimigos comuns continuam
perdendo o golpe). A fase 2 espera FrontalSlash e Lunge, salvo HP abaixo de
25%. Cores de preparação: laranja (slash), amarelo (lunge), ciano (ash wave),
roxo (phase lock). A patrulha fica na laje direita da Ashen Court, longe do
vão da porta de saída.

D23: essa cor de preparação também cobre o flash branco, e a contagem do
wind-up do chefe segue durante o flash. A chegada pela porta do chão cai
numa plataforma (x 600–820); o vão x 540–600 continua sendo a descida.

| Id | Arena | Notas |
|---|---|---|
| `ashen_sentinel` | `vigil_gates_ashen_court` | 2 fases; reward `sentinel_brand` |

Detalhe: `docs/development/d21/`.
