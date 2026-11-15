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

## Expansão (V1.6, planejada)

Nada mudou no sistema de magia; o reconhecedor de comandos e o formato único
de técnicas e magias são preservados. Plano:

| Família | Novo `SkillEffect` | Exemplos | Exploração |
|---|---|---|---|
| Fogo, gelo, raio, água, vento, terra | `Area { radius, duration }`, `Beam`, `Projectile` com `pierce`/`bounce` | Explosão de brasa, lança de gelo, corrente | Gelo cria plataforma temporária; vento move objetos leves |
| Luz e trevas | `Projectile`, `Area` | Selo de luz, véu de sombra | Luz revela paredes falsas próximas (alternativa à parede ilusória) |
| Arcano | `Projectile` com `homing` | Mísseis arcanos | Ativa selos arcanos (`requires: Flag`) |
| Suporte | `Buff { stat, amount, duration }`, `Heal` | Égide, pressa | Égides de ambiente (fogo, veneno) |
| Debuff | `inflicts` (já existe nos dados) aplicado de fato | Queimadura, lentidão | — |
| Invocação | `Summon { entity, duration }` | Familiar de cinza | Familiar ativa interruptores distantes |

- **Aprendizado:** livros, mestres (`command_revealed_on_learn` já existe) e relíquias.
- **Evolução:** ranks por uso, já previstos na etapa 5 do roadmap.
- **Equipamentos que modificam magias:** chaves em `special_effects`
  (`"fire_cost_-20"`) lidas pelo `cast_spells`.
- **Efeitos visuais:** pela camada de VFX da V1.5 (`vfx` nos dados).
- **Resistências e fraquezas:** já existem por elemento (`ElementalResistances`).
