# Atributos do personagem

Código: `src/player/attributes.rs` (puro). Coeficientes: `assets/data/character.ron` → `formulas`.
Testes: `tests/attributes_tests.rs`.

## Atributos primários (`BaseAttributes`)

| Sigla | Nome | Influencia |
|---|---|---|
| STR | Strength | ATK |
| CON | Constitution | HP máximo, DEF, MDEF, regeneração de HP |
| INT | Intelligence | MP máximo, MATK, MDEF, regeneração de MP |
| LCK | Luck | HIT, crítico, esquiva, chance de drop |

Valores efetivos = base + modificadores (`AttributeModifiers`, vindos de equipamentos).

## Recursos

| Recurso | Tipo | Observação |
|---|---|---|
| HP, MP, Hearts | `ResourcePool` (atual ≤ máx, nunca negativo) | Hearts iniciais vêm de `start.hearts` |
| EXP, Level | `Progression` | ver `PROGRESSION_SYSTEM.md` |

## Fórmulas (valores padrão)

`L` = nível; atributos já incluem modificadores; `mod.x` = bônus direto de equipamento.

| Derivado | Fórmula | Limites |
|---|---|---|
| HP máx | `50 + CON·10 + L·5 + mod.max_hp` | ≥ 1 |
| MP máx | `20 + INT·5 + L·2 + mod.max_mp` | ≥ 0 |
| Hearts máx | `99` | — |
| ATK | `STR·2 + mod.attack` | ≥ 0 |
| DEF | `CON·1 + mod.defense` | ≥ 0 |
| MATK | `INT·2 + mod.magic_attack` | ≥ 0 |
| MDEF | `INT·1 + ⌊CON/2⌋ + mod.magic_defense` | ≥ 0 |
| HIT | `100 + LCK·1 + mod.hit` | ≥ 0 |
| CRITICAL_CHANCE | `0.05 + LCK·0.005 + mod` | [0, 0.5] |
| CRITICAL_DAMAGE | `1.5 + LCK·0.01 + mod` | [1, 3] |
| ATTACK_SPEED | `1 + mod.attack_speed` | ≥ 0.25 |
| MOVEMENT_SPEED | `1 + mod.movement_speed` | ≥ 0.25 |
| HP_REGEN (por s) | `CON·0.02 + mod` | ≥ 0 |
| MP_REGEN (por s) | `0.5 + INT·0.05 + mod` | ≥ 0 |
| DODGE | `LCK·0.002 + mod` | [0, 0.25] |
| RESISTANCES | soma das resistências dos equipamentos | cada uma em [−1, 1] |
| STATUS_RESISTANCES | idem, para efeitos negativos | [−1, 1] |

Exemplo (STR 10, CON 8, INT 6, LCK 5, nível 1, sem equipamento): HP 135, MP 52,
ATK 20, DEF 8, MATK 12, MDEF 10, HIT 105, crítico 7,5% ×1,55, esquiva 1%.

## Recalcular sem duplicar

`CharacterSheet` (componente) guarda base, progressão e bônus de equipamento;
`DerivedStats` é recalculado **dentro** de cada método que altera esses dados
(`gain_experience`, `allocate_point`, `set_equipment_bonus`). Ninguém mais
calcula atributos. `sync_derived_stats` propaga o resultado para os máximos de
`Vitals` e para o `CombatProfile` quando a ficha muda.

## Resistências

Escala: `0` neutro · `0.5` metade do dano · `1` imune · `−0.5` fraqueza (150%).
Elementos: Physical, Fire, Ice, Lightning, Water, Wind, Earth, Light (sagrado),
Dark (sombrio), Arcane. Efeitos: Poison, Curse, Petrify, Burn, Freeze, Stun
(chance final = `base · (1 − resistência)`; aplicação dos efeitos: roadmap).

**V2.0 — teto de resistência:** `StatFormulas.max_resistance` (0,75 em
`character.ron`) limita cada resistência elemental somada dos
equipamentos a `[MIN_RESISTANCE, max_resistance]` em `derive`
(`ElementalResistances::capped`). Equipamento nunca dá imunidade; um golpe
resistido ainda causa dano (sem cura involuntária). O piso continua −1.

**V2.0 — HP/MP ao trocar equipamento:** o novo máximo recorta o valor atual
(`ResourcePool::set_max`); um máximo maior não restaura nada.

## Tela de status

`Tab` abre/fecha (`ui/status_screen.rs`). Mostra atributos (com bônus), HP/MP/Hearts,
todos os derivados, resistências e habilidades. Teclas `1`–`4` gastam pontos em STR/CON/INT/LCK.
