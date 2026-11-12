# Sistema de magia

Código: `src/magic/` — `elements.rs` (puro), `casting.rs`, `projectiles.rs`.

## Separação de responsabilidades

O reconhecedor de comandos (`combat/command_parser.rs`) **só** identifica a
sequência e emite o pedido `SkillActivated`. `skills/systems.rs::begin_skill`
revalida o pedido com o estado atual; se confirmar, coloca o personagem em
`Casting`, cobra MP/Hearts, inicia o cooldown e emite `SkillStarted`. Só então
`magic/casting.rs::cast_spells` executa o efeito das habilidades cujas categorias
são magias (`ElementalSpell`, `SupportSpell`, `SummonSpell`). Ataques físicos
ficam em `combat/attacks.rs`.

**Momento do efeito:** hoje o efeito é **instantâneo**, aplicado no frame da
confirmação (o projétil nasce e a cura do Mending Light é aplicada nesse frame).
`Casting` é só o tempo de recuperação depois do efeito, não um preparo: ser
atingido durante `Casting` encerra a recuperação, mas não desfaz o efeito.
Conjuração com preparo interrompível é a decisão D-GAME-002, ainda pendente.
Projéteis já lançados continuam existindo quando o conjurador leva dano.

## Elementos

Fogo, Gelo, Raio, Água, Vento, Terra, Luz, Trevas, Arcano (+ Físico para armas).
Cada um tem uma cor provisória (`element_color`) usada em projéteis e números de dano.
Resistências e fraquezas: ver `CHARACTER_ATTRIBUTES.md`.

## O que uma magia respeita hoje

| Requisito | Onde |
|---|---|
| MP e Hearts | `validate_activation` (no reconhecedor e de novo em `begin_skill`); pagos juntos só na confirmação |
| Cooldown | `Cooldowns` (`SkillBook`) |
| Recuperação / animação | `execution_time` → estado `Casting` (depois do efeito instantâneo) |
| Alcance | `Projectile.lifetime · speed` (e `range` informativo) |
| Colisão | projétil some em sólidos; acerta hurtboxes inimigas |
| Resistência elemental | `calculate_damage` |
| Interrupção | dano coloca o personagem em `Hurt`, encerrando a recuperação (`Casting`); o efeito já aplicado não é desfeito |
| Estado do personagem | `usable_in` |

## Tipos de efeito

Implementados: projétil (`Projectile`), cura (`Heal`).
Planejados: explosão/área, dano contínuo, controle (Freeze/Stun), proteção, invocação.
