# Sistema de habilidades

Código: `src/skills/` — `definitions.rs` (dados), `unlock.rs` (aprendizado e requisitos),
`cooldowns.rs`, `execution.rs` (validação), `systems.rs` (Bevy).
Dados: `assets/data/skills/*.ron` (técnicas) e `assets/data/spells/*.ron` (magias, mesmo formato).

## Formato (`SkillDefinition`)

```ron
(
    id: "crescent_slash",              // único
    name: "Crescent Slash",
    description: "...",
    category: SwordTechnique,          // ver categorias
    command: Some("D DF F + ATTACK"),  // None = passiva
    timing: (total_window: 0.6, step_window: 0.3, max_extra_inputs: 1),
    priority: 10,                      // desempate entre comandos parecidos
    mp_cost: 0, hearts_cost: 0,
    cooldown: 0.8,                     // segundos
    damage_kind: Physical,             // Physical (ATK/DEF) | Magical (MATK/MDEF)
    element: Physical,
    base_damage: 4, damage_multiplier: 1.8,
    range: 70.0,                       // informativo (UI/IA)
    execution_time: 0.4,               // tempo travado executando
    animation: "slash_crescent", vfx: "arc_white",  // chaves para a futura camada de arte
    effect: MeleeStrike(width: 70.0, height: 44.0, lunge_speed: 260.0),
    inflicts: None,                    // Some((Burn, 0.2)) — aplicação: roadmap
    requirements: (min_level: 0, attributes: {}, items: [], progress_flags: []),
    usable_in: [Grounded, Attacking],  // Grounded | Airborne | Attacking | Guarding
    acquisition: Starting,             // Starting | OnRequirements | Discovery
    command_revealed_on_learn: true,   // false = jogador precisa descobrir o comando
)
```

Campos com valor padrão podem ser omitidos. Erros (comando inválido, id
duplicado, campo errado) param o carregamento com o caminho do arquivo e são
detectados por `tests/data_tests.rs`.

**V2.0 — `weapons`:** lista de categorias de arma com que a técnica pode
ser executada (vazia = qualquer uma ou nenhuma). `begin_skill` confere com
`execution::check_weapon`; outra arma gera `CommandOutcome::WrongWeapon`
sem gastar nada. Crescent Slash e Rising Edge: `[Sword]`. `GameData`
rejeita categorias que não sejam de arma.

## Categorias

`SwordTechnique`, `HeavyWeaponTechnique`, `DefensiveTechnique`, `MovementTechnique`,
`SpecialTechnique`, `ElementalSpell`, `SupportSpell`, `SummonSpell`, `Passive`.
As três `*Spell` são executadas pelo sistema de magia.

## Efeitos (`SkillEffect`)

| Variante | Executor |
|---|---|
| `MeleeStrike { width, height, lunge_speed }` | `combat/attacks.rs::melee_techniques` |
| `Projectile { speed, radius, lifetime }` | `magic/casting.rs::cast_spells` |
| `Heal { amount }` | `magic/casting.rs::cast_spells` |
| `None` | passivas |

Nova mecânica = nova variante + executor no módulo dono. O motor de combate e o
parser não mudam para adicionar habilidades.

## Aprender ≠ saber executar

`LearnedSkills` guarda, por habilidade, `rank` (melhorias) e `command_revealed`.
Uma habilidade aprendida com `command_revealed_on_learn: false` aparece como
`[???]` na tela de status até ser revelada (`reveal_command`), mas o comando já
funciona se o jogador o descobrir sozinho.

## Conteúdo do protótipo

| Habilidade | Comando | Custo | Aquisição |
|---|---|---|---|
| Crescent Slash | `D DF F + ATTACK` | — (CD 0,8 s) | início |
| Rising Edge | `D U + ATTACK` | 2 Hearts | automático no nível 3 |
| Ember Bolt | `D DF F + MAGIC` | 8 MP | início |
| Mending Light | `D N D + MAGIC` | 15 MP (CD 5 s) | início |
