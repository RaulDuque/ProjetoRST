# 05 — Criar itens

> Situação: criar itens **funciona**; equipamentos se equipam na tela `I`.
> **Pendente:** usar consumíveis (poções) durante o jogo ainda não existe:
> eles podem ser criados, encontrados, comprados e vendidos, mas não bebidos.

## 1. O que vamos fazer?

Cadastrar os itens da região de exemplo: uma arma, uma capa, um amuleto e
uma poção, e vê-los no inventário.

## 2. O que você precisa saber antes?

- Todo `.ron` de `assets/data/items/` é lido. Cada arquivo tem
  `( items: [ ... ] )` (e, opcionalmente, `rarities` e `weapon_classes`).
- **Campos obrigatórios:** `id` (único em todos os arquivos), `name`,
  `description`, `category`, `rarity`.
- **Raridades** (em `starter_gear.ron`): `common`, `uncommon`, `rare`, `epic`,
  `legendary`. Elas só dão a cor e a ordem; não mudam atributos.
- **Categorias e onde equipam:**

| `category` | Slot na tela `I` |
|---|---|
| `Sword`, `Dagger`, `Axe`, `Spear`, `HeavyWeapon` | Weapon (tutorial 06) |
| `Shield` | Off-hand |
| `Helmet` | Head |
| `Armor` | Body (também capas, tutorial 07) |
| `Gloves` | Arms |
| `Boots` | Legs |
| `Ring`, `Amulet` | Accessory 1 / 2 |
| `Consumable`, `Material`, `QuestItem` | não equipam (`QuestItem` não se vende nem se descarta) |

- **Atributos (`modifiers`)**, escreva só os que quiser:
  inteiros: `strength`, `constitution`, `intelligence`, `luck`, `max_hp`,
  `max_mp`, `attack`, `defense`, `magic_attack`, `magic_defense`, `hit`;
  decimais (com ponto): `critical_chance`, `critical_damage`, `attack_speed`,
  `movement_speed`, `hp_regen`, `mp_regen`, `dodge`; e resistências:
  `resistances: (values: [(Fire, 0.2), (Ice, 0.1)])`
  (elementos: `Fire`, `Ice`, `Lightning`, `Water`, `Wind`, `Earth`, `Light`,
  `Dark`, `Arcane`). A soma de resistências do personagem para em 0,75.
- Outros campos: `requirements: (min_level: 3, attributes: {Strength: 10})`,
  `weight`, `sell_value` (preço que as lojas pagam, tutorial 22),
  `max_stack` (padrão 1), `consumable: Some(RestoreHp(40))` /
  `RestoreMp(20)` / `RestoreHearts(5)`, `weapon` (06), `visual` (06, 07).

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/data/items/crypt_gear.ron` | **Novo.** Os itens do exemplo |
| `assets/data/items/equipment_v2.ron` | Existe. Bons modelos para copiar |

## 4. Passo a passo

1. Crie `assets/data/items/crypt_gear.ron` (seção 5).
2. Para cada item, escolha um `id` novo (confira: `grep -rn "\"crypt_cleaver\"" assets/data/items/`
   não pode achar outro).
3. Escreva nome, descrição, categoria e raridade.
4. Some os atributos em `modifiers`.
5. Ponha o item no jogo: num baú (08), numa loja (21), numa coleta ou num drop (14).
6. Teste.

## 5. Exemplo completo

```ron
// exemplo-verificado: Itens
// Itens da região de exemplo Ashen Crypt.
(
    items: [
        (
            id: "crypt_cleaver",
            name: "Crypt Cleaver",
            description: "A broad cleaver once used to cut grave stones.",
            category: Axe,
            rarity: "uncommon",
            modifiers: (attack: 11),
            weapon: Some((element: Earth)),
            visual: Some(Trail(color: (0.7, 0.6, 0.45))),
            weight: 4.0,
            sell_value: 90,
        ),
        (
            id: "gravecloth_mantle",
            name: "Gravecloth Mantle",
            description: "A grey burial cloth worn as a cloak. Shadows slide off it.",
            category: Armor,
            rarity: "rare",
            modifiers: (defense: 4, magic_defense: 3, resistances: (values: [(Dark, 0.2)])),
            visual: Some(Cape(color: (0.30, 0.28, 0.26), length: 36.0)),
            weight: 2.0,
            sell_value: 150,
        ),
        (
            id: "bone_charm",
            name: "Bone Charm",
            description: "A carved knucklebone on a cord.",
            category: Amulet,
            rarity: "common",
            modifiers: (max_mp: 8, luck: 1),
            sell_value: 60,
        ),
        (
            id: "crypt_draught",
            name: "Crypt Draught",
            description: "Restores 20 MP.",
            category: Consumable,
            rarity: "common",
            consumable: Some(RestoreMp(20)),
            max_stack: 9,
            sell_value: 15,
        ),
    ],
)
```

## 6. Como testar?

1. `cargo test -j 2 --test data_tests` (lê todos os dados reais).
2. `cargo run -j 2`: pegue o item e aperte `I`. Ele aparece com a cor da
   raridade; ao selecioná-lo, a coluna da direita compara os atributos.
3. `cargo test -j 2 --test tutorial_examples_tests`.

## 7. Como editar depois?

- Mude os números de `modifiers`: o bônus é recalculado ao equipar e ao
  carregar o jogo (nunca fica "acumulado").
- **Não troque o `id`** de um item que já pode estar em saves.

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `duplicate id` | ID repetido | Troque um deles |
| `uses unknown rarity` | Raridade escrita errada | Use uma das 5 |
| `weapon data on a non-weapon category` | `weapon` num item que não é arma | Tire o `weapon` |
| `visual ... does not fit category` | `Cape` em espada, `Trail` em anel... | Veja os tutoriais 06 e 07 |
| `max_stack must be at least 1` | `max_stack: 0` | Use 1 ou mais |
| `invalid data in ...` | Campo com nome errado ou vírgula faltando | Compare com o exemplo |

## 9. Como desfazer com segurança?

Apague `assets/data/items/crypt_gear.ron` **e** tire os itens dos baús,
lojas, coletas e drops que os usam (senão o jogo avisa `unknown item`).
