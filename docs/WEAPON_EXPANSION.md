# Expansão de armas

> **V2.0 [Implementado]:** as cinco categorias têm identidade funcional
> (`WeaponClass` em `assets/data/items/weapon_classes.ron`: alcance,
> altura, espaço junto ao corpo, tempo, recuperação e poder), cada arma
> tem elemento e alcance próprios (`weapon`), o ataque básico usa o perfil
> da arma equipada e as técnicas declaram as armas compatíveis. Detalhes e
> balanceamento: [`EQUIPMENT_EXPANSION_V2_0.md`](EQUIPMENT_EXPANSION_V2_0.md)
> §4–7 e §21. V2.1: `two_handed` (Spear e HeavyWeapon) impede escudo junto.
> O texto abaixo é o plano da V1.6, mantido como histórico.

> **[Planejado]**. Nada mudou no combate na V1.6.
> - **Já existe [Implementado]:** `ItemCategory::{Sword, Dagger, Axe, Spear, HeavyWeapon, Shield}`,
>   o slot `Weapon` (e `Offhand`), os modificadores de atributo, requisitos,
>   peso e raridade em dados (`assets/data/items/`).
> - **Hoje:** a categoria só decide o slot. Todo ataque básico usa
>   `BASIC_ATTACK_HITBOX` e `BASIC_ATTACK_TIME`.
>
> Base: [`EQUIPMENT_SYSTEM.md`](EQUIPMENT_SYSTEM.md) e [`COMBAT_SYSTEM.md`](COMBAT_SYSTEM.md).

## 1. Categorias planejadas

| Categoria | `ItemCategory` | Alcance (u) | Tempo do golpe (s) | Escala | Traço |
|---|---|---|---|---|---|
| Espada curta | `Sword` (subtipo) | 40 | 0,24 | STR | Rápida, combo de 3 |
| Sabre (atual) | `Sword` | 46 | 0,28 | STR | Golpe de referência (constantes atuais) |
| Espada longa | `Sword` (subtipo) | 60 | 0,34 | STR | Arco amplo |
| Espada de duas mãos | **nova** `Greatsword` | 78 | 0,48 | STR++ | Bloqueia `Offhand`; quebra guarda |
| Adaga | `Dagger` | 28 | 0,18 | LCK | Crítico alto; arremesso como técnica |
| Lança | `Spear` | 84 (estocada) | 0,36 | STR | Hitbox estreita e longa; para cima |
| Machado | `Axe` | 52 | 0,40 | STR | Arco para cima e para baixo |
| Martelo | `HeavyWeapon` | 56 | 0,52 | STR++ | Atordoa; quebra paredes frágeis |
| Chicote | **nova** `Whip` | 110 | 0,42 | LCK | Hitbox longa e fina; prende em ganchos (exploração) |
| Cajado | **nova** `Staff` | 48 | 0,34 | INT | Ataque mágico; reduz custo de MP |
| Arma mágica | **nova** `Focus` | — | 0,30 | INT | Dispara projétil fraco; para magos |
| Especial | `special_effects` | — | — | — | Efeitos únicos por chave |

Os valores são iniciais e vão para os dados (`assets/data/weapons/*.ron`),
nunca para o código.

## 2. Formato proposto

O perfil de arma fica separado do item, porque várias armas compartilham um perfil:

```ron
// assets/data/weapons/profiles.ron
(
    id: "saber",
    category: Sword,
    hitbox: (size: (46.0, 26.0), offset: (31.0, 4.0)),
    attack_time: 0.28,                 // igual a BASIC_ATTACK_TIME: o perfil do sabre reproduz o jogo atual
    active_fraction: 0.6,
    damage_kind: Physical,
    scaling: (strength: 1.0, luck: 0.0, intelligence: 0.0),
    animation_set: "saber",            // clipes attack_basic, attack_basic_air...
    techniques: ["crescent_slash", "rising_edge"],   // técnicas liberadas por esta arma
    two_handed: false,
)
// no item: weapon_profile: Some("saber"), element: Some(Fire), ...
```

## 3. Migração sem refazer o sistema

1. **V2.0, passo 1:**
   - `ItemDefinition` ganha `weapon_profile: Option<String>` (`serde(default)`, então os itens atuais continuam válidos);
   - o perfil `saber` reproduz exatamente as constantes atuais;
   - teste de "ataque idêntico" bit a bit.
2. **Passo 2:** `basic_attack` lê o perfil da arma equipada (fallback: `saber`).
   É a única mudança de combate, e precisa de autorização.
3. **Passo 3:** técnicas com `requires_weapon: Some(Category)` no RON das
   habilidades. O reconhecedor de comandos não muda.
4. **Passo 4:** animações. O manifesto de sprites ganha conjuntos por arma
   (ver [`EQUIPMENT_VISUAL_SYSTEM.md`](EQUIPMENT_VISUAL_SYSTEM.md)).

## 4. Raridade e efeitos

- **Raridade:** já existe (`rarities.ron`, com tiers).
- **Efeitos especiais:** por chave em `special_effects` (por exemplo
  `"lifesteal_5"`, `"burn_on_hit"`), interpretados por sistemas de combate com
  uma chave cada, sem `if` por item.
- **Elemento da arma:** `element` no item, substituindo `Element::Physical` no `HitSpec`.
