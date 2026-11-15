# 12 — Criar relíquias

> Situação: **funciona** com as habilidades que já existem.
> **Pendente:** criar uma habilidade **nova** (com efeito novo, como pulo
> duplo de verdade) exige mudança no código.

## 1. O que vamos fazer?

Colocar uma relíquia numa sala. Ao tocá-la, o jogador ganha uma habilidade
que abre caminhos (portas, paredes, objetos).

## 2. O que você precisa saber antes?

- Uma relíquia é uma **coleta** (`pickups`): o jogador passa por cima e pega.
  Ou a recompensa de um baú (`reward: Ability(...)`).
- `grants: Ability(Nome)`. Habilidades que existem:

| Habilidade | O que faz hoje |
|---|---|
| `VeilStep` | Abre grades seladas e libera a Forma Espectral (tecla T) |
| `AstralKey` | Abre portas/objetos que a exigem |
| `DoubleJump`, `WallBreaker`, `WraithForm`, `RavenForm`, `BeastForm`, `EmberWard` | **Só** abrem portas/objetos que as exigem (sem efeito próprio ainda) |

- Exemplo: a parede rachada da cripta exige `Ability(WallBreaker)`.
- `id` da coleta: único no mundo (o save guarda "já pego").
- `position`: o centro da relíquia; no chão, `y` por volta de 70.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/world/regions/ashen_crypt.ron` | Sala `ac_bone_ossuary` |

## 4. Passo a passo

1. Em `pickups` da sala, acrescente a relíquia.
2. Use a habilidade num `requires` (porta, parede ou baú).
3. Teste.

## 5. Exemplo completo

```ron
// exemplo-verificado: Coleta
(id: "ac_grave_relic", name: "Grave Relic", position: (400.0, 70.0), grants: Ability(WallBreaker))
```

## 6. Como testar?

1. `cargo run -j 2 --example world_report`: a linha `Collected by the
   progression` mostra a habilidade.
2. `cargo run -j 2`: passe pela relíquia → "Grave Relic: WallBreaker learned".
3. `cargo test -j 2 --test tutorial_examples_tests` (o teste pega esta relíquia).

## 7. Como editar depois?

Mude `grants` ou `position`. Não troque o `id` depois de lançado.

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `invalid data ... Ability` | Nome de habilidade inexistente | Use um da tabela |
| `DuplicateId` | ID repetido | Use outro |
| `ImpossibleRequirement` | Algo exige uma habilidade que ninguém dá | Crie a relíquia |

## 9. Como desfazer com segurança?

Apague a coleta **e** os `requires` que dependem dela (senão o validador acusa).
