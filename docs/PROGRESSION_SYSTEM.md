# Progressão

Código: `src/player/progression.rs` (puro), `src/skills/unlock.rs` (requisitos).
Dados: `assets/data/character.ron` → `progression`. Testes: `tests/progression_tests.rs`.

## Curva de experiência

`exp_to_next(L) = round(base · L^exponent)`, mínimo 1; `None` no nível máximo.
Padrão: `base 30`, `exponent 1.6`, `max_level 99` → L1→2: 30, L2→3: 91, L3→4: 174, L10→11: 1194.

## Subir de nível

`Progression::gain_experience` acumula EXP e sobe quantos níveis couberem. A cada nível:
- atributos automáticos: `growth.automatic` (padrão STR+1, CON+1, INT+1, LCK+0);
- pontos livres: `growth.points_per_level` (padrão 2), gastos na tela de status (`1`–`4`).
No nível máximo o EXP excedente só conta no total vitalício.

## Desbloqueio de habilidades

`SkillDefinition.acquisition`:
- `Starting` — conhecida desde o início;
- `OnRequirements` — aprendida automaticamente quando `requirements` é satisfeito (`auto_learn_skills`, a cada frame de progressão);
- `Discovery` — aprendida por livros, pergaminhos, mestres, baús (interações: roadmap).

`Requirements`: `min_level`, `attributes` (mínimos), `items` (possuir), `progress_flags` (história).
Os mesmos requisitos valem para equipar itens.

Exemplo no protótipo: **Rising Edge** (`D U + ATTACK`) é `OnRequirements` com `min_level: 3`.

## Recompensas

Inimigo derrotado → `EnemyDefeated { experience, drops }` → `collect_rewards`
soma EXP (notificação de level up) e rola os drops com LCK (`EQUIPMENT_SYSTEM.md`).
Tudo é balanceável pelos RON, sem recompilar.
