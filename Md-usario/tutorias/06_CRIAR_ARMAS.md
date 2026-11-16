# 06 — Criar armas

> Situação: **funciona** para as 5 categorias existentes. **Pendente:** uma
> categoria nova de arma (ex.: chicote) exige mudança no código. **V2.2:** a
> arma na mão é a equipada (silhueta provisória por categoria, tutorial 24).

## 1. O que vamos fazer?

Criar uma arma (o machado **Crypt Cleaver**) e entender o que muda entre
espada, adaga, machado, lança e arma pesada.

## 2. O que você precisa saber antes?

- Uma arma é um **item** (tutorial 05) com `category` de arma.
- O **dano** vem de `modifiers.attack` (soma no ATK do personagem).
- A **forma e o ritmo** do golpe vêm da **classe** da categoria, em
  `assets/data/items/weapon_classes.ron`:

| Classe | Alcance | Tempo do golpe | Recuperação | Poder (× ATK) | Duas mãos |
|---|---|---|---|---|---|
| Sword | 46 | 0,28 s | 0 | 1,00 | não |
| Dagger | 28 | 0,17 s | 0 | 0,55 | não |
| Axe | 50 | 0,38 s | 0,14 s | 1,45 | não |
| Spear | 78 (começa a 26 do corpo) | 0,32 s | 0,06 s | 1,10 | **sim** |
| HeavyWeapon | 62 | 0,52 s | 0,26 s | 2,10 | **sim** |

- **Duas mãos** (`two_handed: true`): não dá para usar escudo junto. O jogo
  recusa e explica; nada é tirado sozinho.
- Por arma, opcional: `weapon: Some((element: Fire, reach_bonus: 4.0))`
  (elemento do golpe e alcance a mais) e `visual: Some(Trail(color: (r, g, b)))`
  (cor do rastro do golpe).
- **Técnicas** (Crescent Slash, Rising Edge) só funcionam com `Sword`.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/data/items/crypt_gear.ron` | A arma (criado no tutorial 05) |
| `assets/data/items/weapon_classes.ron` | Existe. Só mude se quiser rebalancear **todas** as armas da categoria |

## 4. Passo a passo

1. No arquivo de itens, crie o item com `category: Axe` (ou outra das 5).
2. Dê o dano em `modifiers: (attack: 11)`.
3. Opcional: elemento e alcance em `weapon`, cor em `visual`.
4. Opcional: requisito (`requirements: (attributes: {Strength: 10})`).
5. Coloque a arma num baú ou loja e teste.

## 5. Exemplo completo

```ron
// exemplo-verificado: Item
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
)
```

A classe Spear como está no jogo (para entender os campos; não precisa copiar):

```ron
// exemplo-verificado: ClasseArma
(category: Spear, name: "Spear", reach: 78.0, height: 16.0, gap: 26.0, raise: 2.0,
    attack_time: 0.32, recovery: 0.06, power: 1.1, two_handed: true)
```

## 6. Como testar?

1. `cargo test -j 2 --test data_tests`.
2. `cargo run -j 2`: pegue a arma, `I`, selecione-a. A comparação mostra
   "Strike power", "Reach" e "Swing time". Equipe (`Enter`), feche (`I`) e
   ataque (`X`): o retângulo do golpe muda de tamanho e cor.
3. `cargo test -j 2 --test equipment_tests`.

## 7. Como editar depois?

- Mais dano: aumente `attack`. Mais alcance: `reach_bonus`.
- Mudar todas as lanças: edite a linha `Spear` de `weapon_classes.ron`
  (afeta todas; rode todos os testes depois).

## 8. Problemas comuns

| Mensagem / sintoma | Causa | Solução |
|---|---|---|
| `weapon ... has no weapon class` | Categoria sem classe | Use uma das 5 categorias |
| `out of range` (classe) | Alcance, tempo ou poder inválido | Valores positivos e razoáveis |
| "Two-handed: remove the off-hand item first" | Escudo equipado | Remova o escudo antes |
| "needs a sword" ao usar técnica | Arma não é espada | Equipe uma espada |
| "Requirement not met" | Falta força/nível | Suba de nível ou mude o requisito |

## 9. Como desfazer com segurança?

Apague o bloco do item (e tire-o de baús e lojas). Se mexeu em
`weapon_classes.ron`, volte a cópia que você fez antes.
