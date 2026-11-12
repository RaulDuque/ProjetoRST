# Sistema de magia

Código: `src/magic/` — `elements.rs` (puro), `casting.rs`, `projectiles.rs`.

## Separação de responsabilidades

O reconhecedor de comandos (`combat/command_parser.rs`) **só** identifica a
sequência. `skills/systems.rs::begin_skill` cobra MP/Hearts, inicia o cooldown e
coloca o personagem em `Casting`. Depois `magic/casting.rs::cast_spells` executa o
efeito das habilidades cujas categorias são magias (`ElementalSpell`,
`SupportSpell`, `SummonSpell`). Ataques físicos ficam em `combat/attacks.rs`.

## Elementos

Fogo, Gelo, Raio, Água, Vento, Terra, Luz, Trevas, Arcano (+ Físico para armas).
Cada um tem uma cor provisória (`element_color`) usada em projéteis e números de dano.
Resistências e fraquezas: ver `CHARACTER_ATTRIBUTES.md`.

## O que uma magia respeita hoje

| Requisito | Onde |
|---|---|
| MP e Hearts | `validate_activation` + `begin_skill` |
| Cooldown | `Cooldowns` (`SkillBook`) |
| Tempo de conjuração / animação | `execution_time` → estado `Casting` |
| Alcance | `Projectile.lifetime · speed` (e `range` informativo) |
| Colisão | projétil some em sólidos; acerta hurtboxes inimigas |
| Resistência elemental | `calculate_damage` |
| Interrupção | dano coloca o personagem em `Hurt`, encerrando `Casting` |
| Estado do personagem | `usable_in` |

## Tipos de efeito

Implementados: projétil (`Projectile`), cura (`Heal`).
Planejados: explosão/área, dano contínuo, controle (Freeze/Stun), proteção, invocação.
