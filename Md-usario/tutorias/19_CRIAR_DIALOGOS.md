# 19 — Criar diálogos

> Situação: **funciona**. Não existem missões nem "efeitos" (dar item,
> ligar flag) a partir do diálogo nesta versão.

## 1. O que vamos fazer?

Escrever as falas do Morrow: uma saudação com escolhas, uma explicação que
volta às escolhas, e uma fala diferente depois que o jogador acha a relíquia.

## 2. O que você precisa saber antes?

- Falas ficam em `assets/data/dialogues/*.ron` como uma lista de **nós**.
- Um nó tem: `id` (único), `speaker` (o `id` do NPC), `lines` (frases, uma
  por vez; **Enter** ou **D** avança), e no fim **ou** `choices` **ou** `next`.
  Sem os dois, a conversa termina.
- Escolhas (`choices`): `text` e `action`:
  - `Goto("outro_no")`: vai para outro nó;
  - `OpenShop`: abre a loja do NPC;
  - `End`: encerra.
  `↑`/`↓` escolhem; **Esc** sai a qualquer momento.
- **Condições** (`when: [...]`, todas precisam valer), em saudações e escolhas:
  `Ability(VeilStep)`, `Flag("nome")`, `Item("id")`, `Visited("id_da_sala")`,
  `RegionVisited("id_da_regiao")`. Não existe "linguagem de script": só essas.
- Em cada lista de escolhas, pelo menos uma **sem** condição (para o jogador
  nunca ficar preso).

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/data/dialogues/ashen_crypt.ron` | **Novo.** As falas do Morrow |
| `assets/data/dialogues/ash_village.ron` | Existe. Modelos com condições |

## 4. Passo a passo

1. Crie o arquivo (seção 5).
2. O NPC aponta para os nós em `greetings` (tutorial 18).
3. Ligue os nós com `Goto` e `next`.
4. Teste conversando.

## 5. Exemplo completo

```ron
// exemplo-verificado: Dialogos
// Falas do Morrow (região de exemplo Ashen Crypt).
[
    (
        id: "morrow_greet",
        speaker: "morrow_crypt_keeper",
        lines: ["Few come down here alive. Fewer leave with empty pockets."],
        choices: [
            (text: "Show me what you sell.", action: OpenShop),
            (text: "What is this place?", action: Goto("morrow_place")),
            (text: "Goodbye.", action: End),
        ],
    ),
    (
        id: "morrow_place",
        speaker: "morrow_crypt_keeper",
        lines: [
            "The village buried its dead here, before the castle woke.",
            "A relic sleeps in the ossuary. The cracked wall beyond it hides more.",
        ],
        next: Some("morrow_greet"),
    ),
    (
        id: "morrow_relic",
        speaker: "morrow_crypt_keeper",
        lines: ["You carry the Grave Relic. That cracked wall will not stop you now."],
        choices: [
            (text: "Show me what you sell.", action: OpenShop),
            (text: "Goodbye.", action: End),
        ],
    ),
]
```

Uma escolha que só aparece depois de visitar a vila:

```ron
// exemplo-verificado: Dialogo
(
    id: "morrow_village_example",
    speaker: "morrow_crypt_keeper",
    lines: ["Did you meet old Wenna up there?"],
    choices: [
        (text: "Yes, at her cottage.", when: [Visited("av_cottage")], action: End),
        (text: "Not yet.", action: End),
    ],
)
```

## 6. Como testar?

1. `cargo test -j 2 --test data_tests`.
2. `cargo run -j 2`: D no NPC; Enter avança; ↑/↓ e Enter escolhem.
3. `cargo test -j 2 --test tutorial_examples_tests` e
   `cargo test -j 2 --test npc_commerce_tests`.

## 7. Como editar depois?

Mude as frases à vontade. Ao renomear um `id` de nó, troque também todos os
`Goto`, `next` e `greetings` que apontam para ele.

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `unknown speaker` | `speaker` não é um NPC | Corrija o id |
| `node ... is spoken by ...` | Nó de outro NPC | Cada NPC usa os próprios nós |
| `has both choices and next` | Os dois juntos | Use só um |
| `every choice is conditional` | Todas as escolhas com `when` | Uma sem condição |
| `empty line` | Frase vazia | Escreva algo ou apague |
| `unknown variant` em `when` | Condição que não existe | Use as 5 da lista |

## 9. Como desfazer com segurança?

Apague os nós novos e tire as referências a eles em `greetings`, `Goto` e `next`.
