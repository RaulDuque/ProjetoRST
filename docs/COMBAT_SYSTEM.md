# Sistema de combate

## Fluxo

```
PlayerInput ─► record_and_resolve_commands ─► resolve_command (puro, estado do frame anterior)
                 ├─ Executed ─► SkillActivated (pedido) ─► begin_skill
                 │      revalida com o estado atual (validate_activation) e trava em Casting
                 │      ├─ rejeitado ─► CommandResolved(motivo)          (nenhum efeito)
                 │      └─ confirmado ─► paga MP+Hearts, cooldown, avanço
                 │                       ─► SkillStarted + CommandResolved(Executed)
                 │                            ├─► melee_techniques (físico) ─┐
                 │                            └─► cast_spells (magia)        ├─► Hitbox
                 ├─ outro resultado ─► CommandResolved (HUD)                 │
                 └─   … no botão ATTACK ─► BasicAttackRequested ─► basic_attack ┘
Hitbox × Hurtbox ─► detect_hits (calculate_damage) ─► DamageDealt ─► receive_damage (alvo)
```

Cada tentativa gera exatamente um `CommandResolved`: o do reconhecedor, ou o de
`begin_skill` quando o reconhecedor achou uma habilidade. Uma habilidade
rejeitada nunca é anunciada como executada e não cobra custo, não inicia
cooldown, não aplica avanço nem cria hitbox/projétil/cura. Os custos de MP e
Hearts são pagos juntos, só depois de os dois serem validados. O ataque básico
de reserva (D-GAME-003, pendente) só vem do reconhecedor: uma rejeição feita por
`begin_skill` não gera ataque básico.

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

Dano recebido pelo jogador (`player::systems::receive_damage`):
- a invulnerabilidade é conferida **a cada mensagem**: num frame com vários
  acertos, só o primeiro que causa dano é aplicado; ele inicia a
  invulnerabilidade (`PLAYER_INVULNERABILITY`) e os seguintes são ignorados;
- `Miss` e acertos de dano 0 não consomem a invulnerabilidade;
- com o jogador morto, as mensagens restantes são descartadas (não ficam para
  o frame seguinte nem atingem o jogador depois do respawn).

## Hitboxes (`src/combat/hitboxes.rs`)

- `Hitbox { owner, team, size, attack, hit, lifetime, pierce, already_hit }`.
- `AttachedToOwner { offset }` mantém o golpe à frente do dono (espelhado pela orientação).
- `Hurtbox { team, size }`; `Invulnerable { remaining }` ignora acertos.
- Interrupção: `cancel_interrupted_hitboxes` (set `Resolution`, depois do
  `receive_damage` do jogador) remove as hitboxes com `AttachedToOwner` cujo
  dono entrou em `Hurt` ou `Dead`. Assim o golpe não continua acertando durante
  o recuo, depois da morte nem no ponto de respawn. Um acerto do jogador no mesmo
  frame em que ele leva dano vale (troca simultânea: `Combat` roda antes de
  `Resolution`). Dano bloqueado (invulnerabilidade, `Miss`, dano 0) não
  interrompe; a guarda impede o `Hurt`, mas um golpe letal mata e cancela.
  Hitboxes de inimigos e projéteis já lançados não são afetados.
- Times: `Player` × `Enemy` — sem fogo amigo.
- Dano por contato com inimigos: `enemies/behavior.rs::contact_damage`.

## Ataque básico por arma (V2.0)

O ataque básico usa o `StrikeProfile` da arma equipada
(`inventory/weapons.rs`): tamanho e posição do golpe, tempo comprometido
(golpe + recuperação, divididos pela velocidade de ataque), duração da
hitbox (`HITBOX_ACTIVE_FRACTION` do golpe), multiplicador de ATK e elemento.
Sem arma, vale `StrikeProfile::unarmed` (as constantes abaixo); a classe
Sword tem os mesmos valores, então a Iron Sword não mudou. A fórmula de
dano não mudou. O hitbox leva `WeaponStrike { category }`. Técnicas com
`weapons` recusam outras armas (`CommandOutcome::WrongWeapon`). Detalhes:
[`EQUIPMENT_EXPANSION_V2_0.md`](EQUIPMENT_EXPANSION_V2_0.md) §6–7, §21.

## Constantes de sensação

`src/core/constants.rs`: gravidade, velocidades, pulo variável, *coyote time*,
esquiva, knockback, invulnerabilidade, redução da guarda, duração do ataque.

## Pendências

Combos encadeados, efeitos de status (Burn, Poison...), defesa direcional para
inimigos, parry. Ver `ROADMAP.md`.
