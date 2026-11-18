# N2 — Contextos

Não há uma segunda máquina de estados. Entrou `GameState::Controls`.

`GameSet::Input`, `Commands`, `Actions`, `Physics`, `Combat`, `Resolution` e `Progression` só rodam em `Playing`. `Rooms` e `Presentation` continuam. Abrir a tela de controles para a simulação pelo mesmo motivo que o inventário para. Inimigo, física e hitbox não avançam. Isso é o que o código já fazia com as outras telas, não uma pausa à parte.

`read_controls` roda em todo `Update`, antes do `GameSet::Input`, também com a tela aberta. O latch guarda se a ação estava segurada. Ao voltar para `Playing`, uma tecla que não soltou não gera `just_pressed`.

Confirmar diálogo é `Confirm` ou `Interact`. No híbrido isso é Enter ou E, não o ataque. No clássico continua Enter ou D. A tecla de inventário não passa pelo `PlayerInput` enquanto o inventário está aberto, porque `publish_player_input` não roda.

A navegação das telas de jogo usa as ações de mira e de movimento, então remapear "cima" remapeia o cursor. A tela de controles não: ela lê seta e Enter crus, para continuar utilizável com um perfil quebrado.

O mapa não abre por cima da tela de controles. Status e inventário não abrem a partir dela.
