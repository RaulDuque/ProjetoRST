# Controles

Teclas padrão de `src/input/mod.rs`. Não há gamepad.

## No jogo

| Tecla | O que faz |
|---|---|
| ← → | Andar. A direção para a qual o personagem olha é a "frente" dos comandos. |
| ↑ | Cima, nos comandos. |
| ↓ | Agachar no chão. Também é "baixo" nos comandos. |
| Shift esquerdo | Correr, enquanto segura. |
| Z | Pular. Segurar no início do pulo faz o pulo cheio. |
| X | Ataque. No fim de uma sequência, dispara a técnica. Sozinho, é o golpe básico. |
| C | Magia. No fim de uma sequência, dispara o feitiço. |
| A | Guarda, enquanto segura. Um golpe de frente tira só uma lasca de HP. Também apaga a sequência que você estava digitando. |
| S | Esquiva, com invulnerabilidade curta. Só sai da locomoção (parado, andando, correndo, agachado). |
| D | Usar o que está nos pés: baú, alavanca, interruptor, altar de save. Perto de um NPC, conversa. |
| T | Entrar e sair da Forma Espectral. Exige a Relíquia do Véu. |
| M | Abrir e fechar o mapa. |
| Tab | Abrir e fechar a tela de status. O jogo pausa. |
| I | Abrir e fechar equipamento e bolsa. O jogo pausa. |
| R | Levantar depois de morrer. Enche HP e MP no spawn da sala. |
| F5 | Salvar. |
| F9 | Carregar. |
| Esc | Fechar inventário, diálogo, loja ou viagem rápida. |

## Telas

| Onde | Teclas |
|---|---|
| Status (Tab) | 1 Força, 2 Constituição, 3 Inteligência, 4 Sorte, se houver ponto sobrando. Tab fecha. |
| Equipamento (I) | ↑ ↓ escolhe. Enter equipa, remove ou usa o consumível (tônico, éter). I ou Esc fecha. |
| Diálogo | Enter ou D avança e confirma. ↑ ↓ escolhe. Esc sai. |
| Loja | ↑ ↓ escolhe. ← compra, → vende. Enter confirma. Esc sai. |
| Viagem rápida | Abre depois de descansar num altar, se já houver outro altar descoberto. ↑ ↓ escolhe. Enter viaja. Esc sai. |

## Técnicas e magias iniciais

A sequência é relativa à frente do personagem. `→` aqui é a direção para a qual ele olha.

| Nome | Sequência | Efeito |
|---|---|---|
| Crescent Slash | ↓ ↘ → depois X | Corte que avança. |
| Ember Bolt | ↓ ↘ → depois C | Projétil de fogo. Custa MP. |
| Mending Light | ↓, solta, ↓, depois C | Cura. Custa MP. |

Rising Edge (↓ ↑ depois X) só é aprendida no nível 3.

Na Forma Espectral, ataque, magia, guarda, esquiva, interação, pulo e corrida ficam bloqueados.
