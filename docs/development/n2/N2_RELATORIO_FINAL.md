# N2 — Relatório final

A N2 está implementada. A N3 não começou. Não houve commit nem push.

## A. Estado inicial

Branch `art/v1.2-protagonist`, HEAD `ab4e016`, alinhada com `origin`. A árvore já tinha edições não commitadas da aprovação da N1 (`N1_RELATORIO_FINAL.md` e textos de D-036). Elas foram preservadas.

## B. Auditoria

Uma tecla por ação. O gameplay já lia `PlayerInput`. Menus, diálogo, loja, mapa e F5/F9 liam a tecla crua. Detalhe em `N2_AUDITORIA_INPUT.md`.

## C. Arquitetura

Ação lógica, binding, borda combinada, perfil, presets e arquivo. `read_controls` roda com o jogo pausado. `publish_player_input` só em `Playing`. Sem crate nova e sem gamepad. `N2_ARQUITETURA.md`.

## D. Preset híbrido

A/D, W e Espaço, S agacha, seta para cima é a mira, mouse esquerdo e X atacam, E interage, Q guarda, F esquiva. O resto permanece. `N2_PRESETS.md` e `controles.md`.

## E. Preset clássico

Setas, Z, X, C, A, S, D e o restante antigo. Os testes e os exemplos roteirizados instalam esse preset. `play_before_boss` não: usa o perfil real.

## F. Remapeamento

Esc abre a tela. Setas e Enter navegam, para a tela continuar usável com um perfil ruim. Enter captura, Esc cancela a captura, Backspace limpa, Aplicar grava. `N2_REMAPEAMENTO.md`.

## G. Conflitos

Recusa. O controle anterior fica. Não há troca. Enter pode confirmar menu e ser tecla de jogo. Escape não entra no perfil. Ação essencial não fica sem nenhum controle.

## H. Contextos

`GameState::Controls` pausa como o inventário: os sets de jogo só rodam em `Playing`. A borda continua sendo lida, então uma tecla segurada não nasce de novo ao voltar. `N2_CONTEXTOS.md`.

## I. Persistência

`controls.ron` em `$XDG_CONFIG_HOME/dark_castle_rpg/` ou `~/.config/dark_castle_rpg/` (Windows: `%APPDATA%`). Escrita atômica. Arquivo inválido não é sobrescrito. Save do personagem continua na versão 3. `N2_PERSISTENCIA.md`.

## J. Comandos especiais

O parser não mudou. W não é "cima". S é "baixo". A seta para cima é "cima". Crescent Slash, Ember Bolt, Mending Light e Rising Edge continuam as mesmas sequências. `N2_COMANDOS_ESPECIAIS.md`.

## K. Testes unitários

Os de `src/input/` passaram dentro de `cargo test` (código 0).

## L. Testes de integração

`tests/controls_tests.rs`: 12 passaram. A suíte inteira, incluindo as rotas com teclas clássicas, terminou com código 0.

## M. Validação gráfica

`cargo run --example n2_controls_probe` abriu a janela, mostrou a tela no preset híbrido e gravou `docs/development/n2/evidence/controls_screen.png`. O mesmo processo remapeou o ataque alternativo para G, aplicou, e leu o arquivo de volta com esse binding. Não houve um segundo processo do executável. A linha Restaurar não foi clicada na janela; ela copia o perfil aplicado, o mesmo mecanismo dos presets.

## N. Qualidade

`cargo fmt --check` passou. `cargo clippy --all-targets -- -D warnings` passou na árvore final. `cargo test` passou na árvore do código de jogo, antes só do exemplo gráfico.

## O. Arquivos

Código novo: `src/input/{action,binding,edges,presets,profile,store}.rs`, `src/ui/controls_screen.rs`, `tests/controls_tests.rs`, `examples/n2_controls_probe.rs`.

Código alterado para ler a ação, pausar na tela nova, ou instalar o clássico nos harnesses e exemplos roteirizados. Lista no `git status`. `Md-usario/` não foi editado.

## P. Documentação

`docs/development/n2/` (este arquivo e os oito ao lado), `controles.md`, `docs/INPUT_SYSTEM.md`, D-037. A N1 só ganhou o ponteiro de que a N2 foi feita.

## Q. Segurança

Não houve commit, push, reset, clean, nem mudança de câmera, arte, `white_mesh.glb`, `saves/`, save v3 ou `Md-usario/`. Os briefs N-2 a N-5 estão no disco como arquivos novos do usuário, intocados.

## R. Pendências

Q e F são provisórios. As teclas 1–4 da tela de status continuam fixas. Não há gamepad. O HUD continua desenhado por cima do canto da tela de controles. A janela não foi fechada e reaberta num segundo processo.

## S. Decisões em aberto

Nenhuma para a N2. N3, Pixelorama e a malha continuam sem autorização.

## T. Estado final

Branch `art/v1.2-protagonist`, HEAD `ab4e016`, sem commit e sem push desta N2.
