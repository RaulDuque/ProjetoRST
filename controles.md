# Controles

O jogo lê ações, não teclas fixas. Sem um arquivo de preferências, vale o preset híbrido. O preset clássico continua disponível na tela de controles. Não há gamepad.

A tela abre com Esc durante o jogo. A simulação para, do mesmo jeito que o inventário: os sistemas de jogo só rodam em `Playing`. Esc durante uma captura cancela só a captura. Esc fora da captura fecha e descarta o rascunho. As setas e o Enter navegam essa tela mesmo se os bindings de jogo estiverem estranhos. Aplicar grava o arquivo local. Restaurar volta ao que estava salvo. Híbrido e Clássico carregam o preset no rascunho; só ficam valendo depois de Aplicar.

O arquivo é `controls.ron`, em `$XDG_CONFIG_HOME/dark_castle_rpg/` ou `~/.config/dark_castle_rpg/` (no Windows, `%APPDATA%/dark_castle_rpg/`). Não entra no save do personagem. Um arquivo ilegível não é apagado: o jogo usa o híbrido e avisa.

## Híbrido (padrão)

| Ação | Controle |
|---|---|
| Esquerda / direita | A / D |
| Pular | W e Espaço. Os dois juntos são um pulo só. |
| Agachar e "baixo" dos comandos | S |
| "Cima" dos comandos | Seta para cima. Não pula. |
| Correr | Shift esquerdo, enquanto segura. |
| Ataque | Mouse esquerdo, e também X. Os dois no mesmo instante são um ataque. |
| Magia | C |
| Guarda | Q. Era A; A agora anda. |
| Esquiva | F. Era S; S agora agacha. |
| Interagir | E. Baú, alavanca, interruptor, altar. Perto de um NPC, conversa. D anda e não usa. |
| Forma espectral | T |
| Mapa | M |
| Status | Tab. O jogo pausa. |
| Inventário | I. O jogo pausa. |
| Levantar | R |
| Salvar / carregar | F5 / F9 |
| Confirmar menu, diálogo e loja | Enter |
| Tela de controles | Esc |

Q e F são escolhas provisórias para não deixar guarda e esquiva sem tecla. Dá para trocar na tela.

## Clássico

Setas andam. Seta para cima é o "cima" do comando. Seta para baixo agacha e é o "baixo". Shift corre. Z pula. X ataca. C magia. A guarda. S esquiva. D usa e também avança o diálogo. T forma. M mapa. Tab status. I inventário. R levantar. F5 salva. F9 carrega. Enter confirma.

## Telas

| Onde | Como navegar |
|---|---|
| Controles (Esc) | Setas, Enter captura, Backspace limpa o slot, Esc cancela. 1–4 não são desta tela. |
| Status (Tab) | 1 Força, 2 Constituição, 3 Inteligência, 4 Sorte. Essas quatro teclas não entram no perfil. Tab fecha. |
| Inventário (I) | Cima e baixo do perfil escolhem. Enter equipa, remove ou usa. I ou Esc fecha. |
| Diálogo | Enter ou Interagir avança. Cima e baixo escolhem. Esc sai. |
| Loja | Cima e baixo escolhem. Esquerda compra, direita vende. Enter confirma. Esc sai. |
| Viagem rápida | Cima e baixo escolhem. Enter viaja. Esc fica. |

No híbrido, "cima" do menu é a seta para cima e "baixo" é S. No clássico, são as setas.

## Técnicas e magias iniciais

A sequência é relativa à frente do personagem. O parser não mudou.

| Nome | Sequência | Como fazer no híbrido |
|---|---|---|
| Crescent Slash | ↓ ↘ → depois ataque | S, S+D, D, depois mouse ou X |
| Ember Bolt | ↓ ↘ → depois magia | S, S+D, D, depois C |
| Mending Light | ↓, solta, ↓, depois magia | S, solta, S, depois C |
| Rising Edge (nível 3) | ↓ ↑ depois ataque | S, seta para cima, depois mouse ou X |

W não é "cima". Segurar W e apertar Espaço não dá outro pulo. Soltar uma das duas teclas de pulo, com a outra ainda segurada, não conta como soltar o pulo.

Na Forma Espectral, ataque, magia, guarda, esquiva, interação, pulo e corrida ficam bloqueados.
