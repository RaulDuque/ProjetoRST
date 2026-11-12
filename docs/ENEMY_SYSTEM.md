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
- dentro de `attack_range` → `Attack` (ou `Wait` em cooldown);
- caso contrário → `Chase`.

Comportamentos existentes no enum: Patrol, Chase, Attack, Defend, Flee, Wait,
Retreat, UseSkill. Executados hoje: patrulha, perseguição, ataque (com
*wind-up* de 0,35 s, cor laranja), espera e fuga/recuo.

A IA nunca lê entrada do jogador nem estados de controle; só a posição e se ele está vivo.

## Ciclo de vida

Spawn a partir da sala → recebe `DamageDealt` (pisca, recuo, cancela o ataque) →
HP 0 → `EnemyDefeated` (EXP + drops) → respawn após `respawn_time` no ponto de origem.

## Protótipo

**Bone Sentry**: 60 HP, ATK 14, DEF 4; fraco a Luz (−50%) e Fogo (−25%),
resistente a Trevas (50%); 25 EXP; 15% de chance de Minor Tonic.

## Chefes (planejado)

Máquina de estados própria por chefe, múltiplas fases por limiar de HP e padrões
de ataque definidos em dados. Ver `ROADMAP.md` etapa 10.
