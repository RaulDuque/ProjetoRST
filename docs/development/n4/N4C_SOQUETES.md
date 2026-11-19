# N4-1 (C1) — Preparação da marcação manual dos soquetes

**Nada disto está no jogo.** É a ficha para o proprietário (ou um artista) confirmar ou corrigir à mão antes de qualquer arma ou escudo entrar na prova. Armas e escudos continuam fora da prova.

Coordenadas: texels do canvas 112×128 (origem no canto superior esquerdo), pivô (56, 118). Os candidatos são os pontos marcados à mão no esqueleto de cada pose (`run_poses.py`), conferidos sobre o quadro aprovado (`evidence/n4c/n4c_skeletons.png`). **Não foram calculados automaticamente.** Mão parcialmente oculta não é marcada.

Lado: o personagem olha para a direita, então o braço do lado da câmera ("perto") é o **direito** físico. A proposta é que ele seja a mão principal, mas falta a confirmação do proprietário.

| Soquete | #0 contato | #2 passada | #4 recuperação |
|---|---|---|---|
| Mão principal (punho perto) | (75, 55) candidato, visível | (81, 39) candidato, visível | (57, 58) candidato, visível sobre o quadril |
| Mão secundária (punho longe) | **não marcar**: parcialmente oculta atrás do antebraço perto | (48, 58) candidato, atrás do tronco, com bracelete | (82, 40) candidato, visível à frente do peito |
| Cabeça (centro) | (66, 26) | (66, 25) | (67, 24) |
| Ombro perto | (61, 39) | (61, 38) | (57, 38) |
| Ombro longe | (57, 39), oculto: só referência | (56, 38), oculto: só referência | (63, 38), oculto: só referência |
| Costas (gola, para capa) | a marcar na borda de trás do tronco, linha 36 | a marcar, linha 35 | a marcar, linha 34 |
| Cintura (cinto) | (58, 59) | (58, 59) | (58, 59) |

Para cada mão, o soquete do jogo também precisa de **ângulo** (do pomo para a guarda) e de **frente/trás** (`sockets.rs`). Isso não foi estimado: depende de como o punho segura a arma em cada pose, e precisa ser desenhado.

## Formato quando forem confirmados

A tabela existente (`assets/animations/player_sockets.ron`, `SocketTable`) já tem `weapon` (com ângulo e frente), `hand_l`, `head`, `chest` e `back`. Faltam `shoulder` e `waist`, que seriam uma extensão do formato (com versão nova), decidida quando a C2 e os equipamentos entrarem.
