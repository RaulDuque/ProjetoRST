# N2 — Persistência

O perfil não entra em `SaveFile` e a versão do save continua 3.

Caminho, nesta ordem:

1. `$XDG_CONFIG_HOME/dark_castle_rpg/controls.ron` se a variável não for vazia
2. `%APPDATA%/dark_castle_rpg/controls.ron` se a variável não for vazia
3. `$HOME/.config/dark_castle_rpg/controls.ron`
4. `dark_castle_rpg/controls.ron` no diretório corrente, só se não houver home

Não fica em `assets/`, nem em `Md-usario/`, nem ao lado do executável quando há home.

Arquivo ausente: cria-se o híbrido na memória. O arquivo só nasce no Aplicar.

Arquivo presente e válido: é o perfil do jogo, inclusive depois de fechar e abrir.

Arquivo inválido (texto quebrado, versão diferente de 1, ação faltando, conflito, essencial vazio): o jogo sobe no híbrido, escreve um aviso (`ControlsNotice`, também no log) e não regrava o arquivo.

A gravação escreve `controls.ron.tmp` no mesmo diretório, faz `sync` e só então renomeia por cima do destino. Uma falha no meio deixa o arquivo anterior.

Testes não leem o arquivo do desenvolvedor. `use_legacy_controls` coloca o clássico e um caminho debaixo de `CARGO_TARGET_TMPDIR` antes do startup. O teste do arquivo corrompido aponta `ControlsPath` para um temporário próprio.
