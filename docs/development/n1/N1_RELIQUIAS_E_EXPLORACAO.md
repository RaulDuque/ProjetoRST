# N1 — Relíquias e exploração

Relíquia é uma `Ability` permanente em `WorldProgress`. Não é equipamento, não gasta, não se vende. O save já a carrega. A porta e o objeto perguntam por `Requirement::Ability`. O mapa não precisa de um canal novo: a sala continua secreta até a visita, e a relíquia em si é um pickup com nome.

## O que já está no mundo

| Id | Nome no mapa | Efeito real | Persistência |
|---|---|---|---|
| `VeilStep` | Relic of the Veil | abre o véu e libera a Forma Espectral | `abilities` |
| `AstralKey` | Astral Emblem | abre o selo astral; não muda a física | `abilities` |
| `vigil_gates_cleared` | Gatekeeper's Sigil | flag, não habilidade; abre a porta das galerias | flag |

Consumível (`minor_tonic`), item de equipar e tesouro de baú seguem no inventário. O sigilo mostra o terceiro tipo: item-chave que é flag, não ability. Material de craft não existe e não entra nesta fundação.

## Nomes já reservados e ainda sem pickup

`DoubleJump`, `WallBreaker`, `WraithForm`, `RavenForm`, `BeastForm`, `EmberWard`. O validador sabe deles. Nenhum quarto os concede. Criar o pickup antes da física mente para o jogador.

Cada relíquia nova, quando for feita, precisa de: id no enum (já é finito, sem string solta), nome, uma frase, o efeito na física ou na porta, o requisito para aparecer, o estado em `abilities`, a notificação que o pickup já dispara, e uma sala que só ela abre. Sem a sala, a relíquia é um número.

## Diferença prática

| | Onde mora | Some? | Abre caminho? |
|---|---|---|---|
| Relíquia | `abilities` | não | sim |
| Equipamento | slot e inventário | pode ser trocado | não, salvo requisito de item numa porta |
| Consumível | pilha | ao usar | não |
| Flag / sigilo | `flags` | não | a porta que cita a flag |
| Tesouro | item ou moeda | pode ser vendido | não |

## Alternativas

Relíquia como item no inventário, para poder largar. Isso desfaz a exploração no momento em que o jogador vende o pulo duplo. Recusado.

## Riscos

`WraithForm` ao lado da Forma Espectral. São duas coisas. A espectral já existe e usa `VeilStep`. Uma forma nova precisa de outro id e de outro movimento, senão o jogador ganha duas chaves para a mesma porta.

`DoubleJump` mudar `PLAYER_JUMP_SPEED` global. O pulo atual é o que faz a escada do Reliquary funcionar por pouco. A relíquia adiciona um segundo impulso no ápice, não mexe na constante do primeiro pulo.

## Recomendação

A próxima relíquia implementada é o pulo duplo, na N8, com uma sala de vão que já esteja desenhada e fechada. Antes disso, nenhuma porta nova cita `DoubleJump`.
