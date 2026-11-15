# 10 — Criar paredes quebráveis

> Situação: **funciona**.

## 1. O que vamos fazer?

Criar uma parede rachada que bloqueia uma passagem e quebra com um ataque.

## 2. O que você precisa saber antes?

- É um objeto `BreakableWall` em `objects`. Enquanto inteira, é **sólida**.
- `breaks_on`:
  - `AnyAttack`: qualquer ataque do jogador;
  - `Kind(Physical)` ou `Kind(Magical)`: só golpes físicos ou só magias;
  - `Element(Fire)` (ou outro elemento): só ataques desse elemento.
- Um acerto válido quebra; a quebra é salva (fica quebrada após F9).
- Quebrar a parede **acha o segredo de mesmo `id`**: portas com
  `hidden_by: Some("id_da_parede")` aparecem.
- `requires: Some(...)`: a parede não quebra antes disso (ex.: uma relíquia).
- Para tapar uma porta lateral: largura 80 colada na parede da sala, altura
  160 a partir do chão (`min: (1200.0, 40.0)` no lado direito, `(0.0, 40.0)`
  no esquerdo).

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/world/regions/ashen_crypt.ron` | Sala `ac_bone_ossuary` |

## 4. Passo a passo

1. Em `objects` da sala, acrescente a parede (seção 5).
2. Se ela esconde uma porta, ponha `hidden_by: Some("ac_cracked_wall")` na porta.
3. Opcional: `requires` para exigir uma relíquia (tutorial 12).
4. Teste.

## 5. Exemplo completo

```ron
// exemplo-verificado: Objeto
(id: "ac_cracked_wall", kind: BreakableWall(breaks_on: AnyAttack),
    area: (min: (1200.0, 40.0), size: (80.0, 160.0)),
    requires: Some(Ability(WallBreaker)))
```

Uma parede de gelo que só derrete com fogo:

```ron
// exemplo-verificado: Objeto
(id: "ac_ice_wall", kind: BreakableWall(breaks_on: Element(Fire)),
    area: (min: (0.0, 40.0), size: (80.0, 160.0)))
```

## 6. Como testar?

1. `cargo run -j 2 --example world_report` → `Errors (0):`.
2. `cargo run -j 2`: chegue perto (X a menos de ~50 da parede) e ataque. Sem a
   relíquia, a parede resiste; com ela, quebra e a porta aparece.
3. `cargo test -j 2 --test tutorial_examples_tests` (o teste quebra esta parede).

## 7. Como editar depois?

Mude `breaks_on`, a `area` ou o `requires`.

## 8. Problemas comuns

| Mensagem / sintoma | Causa | Solução |
|---|---|---|
| `ImpossibleObjectRequirement` | Ninguém dá o que a parede exige | Crie a relíquia/alavanca |
| `ObjectOutOfRoom` | Parede fora da sala | Ajuste `min` e `size` |
| Ataque não quebra | Elemento/tipo errado, ou falta o requisito | Confira `breaks_on` e `requires` |

## 9. Como desfazer com segurança?

Apague o objeto e o `hidden_by` da porta (ou a porta inteira).
