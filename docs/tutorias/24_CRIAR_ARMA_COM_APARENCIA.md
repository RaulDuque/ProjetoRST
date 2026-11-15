# 24 — Criar uma arma com aparência própria

> Situação: **funciona** (V2.2, arte sem sabre publicada). **Pendente:** a
> arte das armas é **provisória, gerada em código**: uma silhueta por
> categoria, não um desenho por item.

## 1. O que vamos fazer?

Criar a lança **Lantern Pike** com cor própria e ver o personagem segurando
uma lança (e não a espada inicial).

## 2. O que você precisa saber antes?

- O desenho na mão vem da **categoria** do item (tutorial 06):

| `category` | Silhueta na mão |
|---|---|
| `Sword` | Lâmina curva de uma mão, guarda e pomo |
| `Dagger` | Lâmina curta em folha, guarda pequena |
| `Axe` | Cabo de madeira com cabeça barbada na ponta |
| `Spear` | Haste longa segurada no meio, ponta em folha (em repouso fica de pé) |
| `HeavyWeapon` | Montante largo de cabo longo (em repouso vai no ombro) |

- A **cor de destaque** (guarda, empunhadura, anel da haste) vem de
  `visual: Some(Trail(color: (r, g, b)))` — a mesma cor do rastro do golpe.
  Sem `visual`, a cor padrão é latão.
- Cores vão de `0.0` a `1.0` em cada canal.
- Nada disso muda dano, alcance ou tempo do golpe.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/data/items/equipment_v2.ron` | Onde a arma é cadastrada (ou um arquivo novo em `assets/data/items/`) |
| `src/inventory/held_art.rs` | Só leitura: o desenho de cada categoria |

## 4. Passo a passo

1. Abra `assets/data/items/equipment_v2.ron`.
2. Na lista `items`, acrescente a arma da seção 5 (copie um item existente).
3. Escolha `category: Spear` e uma cor em `visual`.
4. Teste (seção 6).

## 5. Exemplo completo

```ron
// exemplo-verificado: Item
(
    id: "lantern_pike",
    name: "Lantern Pike",
    description: "A watch pike with a brass ring that glows like a lamp.",
    category: Spear,
    rarity: "uncommon",
    modifiers: (attack: 12),
    visual: Some(Trail(color: (1.0, 0.85, 0.4))),
    weight: 4.0,
    sell_value: 90,
),
```

| Campo | Efeito visual |
|---|---|
| `category: Spear` | Silhueta de lança na mão |
| `visual: Some(Trail(color: ...))` | Cor do anel/guarda e do rastro do golpe |

Só a parte visual, se quiser trocar a cor de uma arma existente:

```ron
// exemplo-verificado: Visual
Trail(color: (0.55, 0.85, 1.0))
```

## 6. Como testar?

1. `cargo test -j 2 --test data_tests` (o item é válido).
2. `cargo run -j 2 --example equipment_visual_showcase -- --published` e
   confira as capturas em `target/equipment_visual_showcase/published/`
   (tutorial 29). Ou jogue: `cargo run -j 2`, tecla **I** e equipe.
3. Para ver a **sua** arma, ponha o `id` dela na lista `Give(&[...])` do
   exemplo e um `Equip(S::Weapon, "lantern_pike")` no roteiro.

## 7. Como editar depois?

Mude só a cor em `visual`. Para outra silhueta, mude `category` (isso também
muda o golpe: veja a tabela do tutorial 06).

## 8. Problemas comuns

| Mensagem ou sintoma | Causa | Solução |
|---|---|---|
| `visual Trail does not fit category` | `Trail` num item que não é arma | Use `Trail` só em armas |
| `colour channel outside 0..1` | Cor com valor acima de 1 | Use valores de 0.0 a 1.0 |
| A espada inicial aparece com qualquer arma | Arte antiga (com sabre) em `assets/` | Confira `python3 art-src/player/scripts/post/publish_guard.py status` (deve dizer pipeline 2.2) |

## 9. Como desfazer com segurança?

Apague o bloco do item. Se ele estiver em lojas ou baús, tire de lá também
(senão o jogo acusa item desconhecido).
