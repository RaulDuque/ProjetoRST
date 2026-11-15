# 11 — Criar alavancas e mecanismos

> Situação: **funciona**. Mecanismos disponíveis: alavanca → flag → porta
> (ou objeto) que exige a flag; atalhos. **Pendente:** plataformas móveis,
> elevadores e portas com tempo não existem.

## 1. O que vamos fazer?

Uma alavanca que, puxada com D, abre a grade de uma porta. E um atalho.

## 2. O que você precisa saber antes?

- A alavanca é um objeto `Lever(sets: "nome_da_flag")`. Puxar (D) liga a
  **flag** (um "interruptor" do mundo, guardado no save).
- Qualquer porta ou objeto com `requires: Some(Flag("nome_da_flag"))` passa a
  funcionar.
- Uma porta trancada mostra uma grade até a condição ser cumprida.
- **Atalho:** porta com `shortcut: true` + `requires: Some(Flag(...))`, aberta
  por uma alavanca **do outro lado**. O validador confere que o atalho não
  "pula" a progressão.
- Uma alavanca que liga uma flag que ninguém usa gera aviso `UnusedLever`.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/world/regions/ashen_crypt.ron` | Alavanca e porta da Crypt Stair |
| `assets/world/regions/forgotten_galleries.ron` | Exemplo real de atalho (`fg_trapdoor_lever`) |

## 4. Passo a passo

1. Em `objects`, crie a alavanca com um nome de flag novo.
2. Nas **duas metades** da porta, ponha `requires: Some(Flag("mesmo_nome"))`.
3. Teste puxando a alavanca.

## 5. Exemplo completo

```ron
// exemplo-verificado: Objeto
(id: "ac_stair_lever", kind: Lever(sets: "ac_ossuary_gate_open"),
    area: (min: (300.0, 40.0), size: (16.0, 40.0)))
```

A porta (as duas metades, no tutorial 03):

```text
(id: "to_ossuary", side: Right, to: (room: "ac_bone_ossuary", door: "to_crypt_stair"),
    requires: Some(Flag("ac_ossuary_gate_open"))),
```

Atalho real do jogo (alçapão aberto por alavanca na mesma sala):

```text
(id: "fg_trapdoor_lever", kind: Lever(sets: "fg_trapdoor_open"),
    area: (min: (260.0, 40.0), size: (16.0, 40.0))),
(id: "to_sealed_hall", side: Bottom, to: (room: "vigil_gates_sealed_hall", door: "from_return_bridge"),
    requires: Some(Flag("fg_trapdoor_open")), shortcut: true),
```

## 6. Como testar?

1. `cargo run -j 2 --example world_report` → `Errors (0):`, `Warnings (0):`.
2. `cargo run -j 2`: a porta mostra a grade; puxe a alavanca (D) → "A mechanism
   clanks somewhere." → a grade some. F5/F9: continua aberta.
3. `cargo test -j 2 --test tutorial_examples_tests` (o teste puxa esta alavanca).

## 7. Como editar depois?

Troque o nome da flag nos **três** lugares (alavanca e duas metades da porta).

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `UnusedLever` (aviso) | Flag que nada usa | Use-a numa porta ou tire a alavanca |
| `ImpossibleRequirement` | Porta pede flag que nenhuma alavanca liga | Nome igual nos dois lugares |
| `ShortcutBypassesProgression` | O atalho deixa pular uma exigência | Mude o lado da alavanca ou o requisito |
| `NoWayBack` | O jogador pode ficar preso | Garanta um caminho de volta |

## 9. Como desfazer com segurança?

Apague a alavanca e tire o `requires` das duas metades da porta.
