# Sistema de combate

## Fluxo

```
PlayerInput ─► record_and_resolve_commands ─► resolve_command (puro)
                 │                                  │
                 │                 CommandOutcome ──┴─► CommandResolved (HUD)
                 ├─ Executed ─► SkillActivated ─► begin_skill (custos, cooldown, estado Casting)
                 │                               ├─► melee_techniques (físico)  ─┐
                 │                               └─► cast_spells (magia)         ├─► Hitbox
                 └─ ATTACK sem comando ─► BasicAttackRequested ─► basic_attack ──┘
Hitbox × Hurtbox ─► detect_hits (calculate_damage) ─► DamageDealt ─► receive_damage (alvo)
```

## Fórmula de dano (`src/combat/damage.rs`, puro)

```
hit_chance = clamp(HIT/100 − DODGE_alvo, 0.5, 1)          → erro = MISS
bruto      = POWER · multiplicador + dano_base             (POWER = ATK ou MATK)
mitigado   = bruto · 100 / (100 + DEF)                     (DEF ou MDEF; 100 DEF = metade)
elemental  = mitigado · (1 − resistência[elemento])
crítico    = elemental · CRITICAL_DAMAGE  se rolagem < CRITICAL_CHANCE
final      = round(crítico · variância[0.9, 1.1)), mínimo 1 (0 se imune)
```

Aleatoriedade injetada via `DamageRolls` (testes determinísticos); no jogo vem de `GameRandom`.
Testes: `tests/combat_tests.rs`.

## Máquina de estados do personagem (`src/player/state.rs`)

| Estado | Tipo | Sai por |
|---|---|---|
| Idle, Walking, Running, Crouching, Jumping, Falling | locomoção | física/entrada a cada frame |
| Attacking | temporizado (`BASIC_ATTACK_TIME / ATTACK_SPEED`) | timer |
| Casting (técnicas e magias) | temporizado (`execution_time / ATTACK_SPEED`) | timer |
| Dodging | temporizado, invulnerável | timer |
| Guarding | mantido (A) | soltar a tecla / sair do chão |
| Hurt | temporizado (stagger) | timer |
| Dead | final | `R` (respawn) ou F9 (carregar) |

Regras: morto não age; dano interrompe tudo menos esquiva; guarda bloqueia 70%
do dano frontal sem stagger; não se pula nem esquiva no ar; técnicas podem
cancelar o ataque básico (`usable_in: [.., Attacking]`).

## Hitboxes (`src/combat/hitboxes.rs`)

- `Hitbox { owner, team, size, attack, hit, lifetime, pierce, already_hit }`.
- `AttachedToOwner { offset }` mantém o golpe à frente do dono (espelhado pela orientação).
- `Hurtbox { team, size }`; `Invulnerable { remaining }` ignora acertos.
- Times: `Player` × `Enemy` — sem fogo amigo.
- Dano por contato com inimigos: `enemies/behavior.rs::contact_damage`.

## Constantes de sensação

`src/core/constants.rs`: gravidade, velocidades, pulo variável, *coyote time*,
esquiva, knockback, invulnerabilidade, redução da guarda, duração do ataque.

## Pendências

Combos encadeados, efeitos de status (Burn, Poison...), defesa direcional para
inimigos, parry. Ver `ROADMAP.md`.
