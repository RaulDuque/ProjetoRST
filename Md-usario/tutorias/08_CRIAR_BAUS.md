# 08 — Criar baús

> Situação: **funciona**.

## 1. O que vamos fazer?

Pôr um baú numa sala, com uma recompensa que só pode ser pega **uma vez**,
mesmo depois de salvar e carregar.

## 2. O que você precisa saber antes?

- Baú é um **objeto** da sala (lista `objects`).
- `id`: **único no mundo inteiro** (é ele que o save guarda como "aberto").
- `tier`: só a cor do baú: `Common` (marrom), `Rare` (azul), `Secret`
  (dourado, para salas secretas) ou `BossReward`.
- `reward`: `Item("id_do_item")`, `Ability(...)` (tutorial 12) ou `Flag("nome")`.
- `area`: onde o baú fica. No chão: `min: (x, 40.0)`, tamanho típico `(48.0, 36.0)`.
- Para abrir: ficar em cima (ou encostado) e apertar **D**.
- Opcional: `requires: Some(...)` (baú "selado" até ter algo).

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/world/regions/ashen_crypt.ron` | A sala `ac_bone_ossuary` tem um baú |

## 4. Passo a passo

1. Escolha a sala e um lugar livre no chão (ex.: x = 640).
2. Em `objects: [ ... ]` da sala, acrescente o baú (seção 5).
3. Confirme que o item da recompensa existe (tutorial 05).
4. Teste abrir, salvar (F5), carregar (F9) e tentar abrir de novo.

## 5. Exemplo completo

```ron
// exemplo-verificado: Objeto
(id: "ac_ossuary_chest", kind: Chest(tier: Common, reward: Item("crypt_cleaver")),
    area: (min: (640.0, 40.0), size: (48.0, 36.0)))
```

## 6. Como testar?

1. `cargo run -j 2 --example world_report` → `Errors (0):`.
2. `cargo run -j 2`: vá até o baú, aperte D → "Found Crypt Cleaver". Aperte D
   de novo: nada. F5, saia da sala, F9: o baú continua aberto e o item continua
   no inventário (`I`).
3. `cargo test -j 2 --test tutorial_examples_tests`.

## 7. Como editar depois?

- Outra recompensa: mude `reward`. Mover: mude `area.min`.
- **Não troque o `id`** de um baú que já existe (o save perderia a referência
  e o baú poderia ser aberto de novo).

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `DuplicateId` | ID usado por outro baú, segredo ou coleta | Use outro |
| `ObjectOutOfRoom` | `area` fora da sala | Ajuste `min` |
| `grants unknown item` | Item não existe | Crie o item ou corrija o nome |
| D não abre | Longe demais, ou em forma espectral | Fique em cima; volte à forma humana (T) |

## 9. Como desfazer com segurança?

Apague o bloco do baú. (Saves antigos que já o abriram guardam só o `id`:
nada quebra.)
