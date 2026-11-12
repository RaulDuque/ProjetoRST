---
name: atomic-commit
description: Separa as alterações pendentes por responsabilidade e cria commits atômicos validados no padrão Conventional Commits. Use quando o usuário pedir para commitar.
disable-model-invocation: true
argument-hint: "[escopo ou observação opcional]"
---

# /atomic-commit

Observação do usuário: $ARGUMENTS

## Procedimento

1. **Analisar** — `git status` e `git log --oneline -5` (estilo das mensagens).
2. **Examinar** — `git diff` e `git diff --staged`; leia arquivos não rastreados antes de incluí-los.
3. **Separar por responsabilidade** — agrupe arquivos (e, se preciso, trechos) em unidades lógicas: uma funcionalidade, uma correção, uma refatoração, um conjunto de testes, uma doc. Apresente o plano de commits antes de executar quando houver mais de um grupo.
   - Arquivos alterados que não pertencem à tarefa atual (ex.: mudanças do usuário) **não** entram: avise e deixe-os intactos.
   - Nunca inclua `target/`, `saves/`, temporários, `.claude/settings.local.json`.
4. **Validar** — para cada grupo, o código commitado precisa compilar e passar nos testes **sozinho**:
   `cargo fmt --check`, `cargo clippy --all-targets`, `cargo test`.
   Se um grupo depende de outro, commite o outro primeiro.
5. **Mensagem** — `tipo(escopo): descrição imperativa em inglês` (≤ 72 caracteres); corpo opcional explicando o *porquê*. Inclua as linhas de atribuição exigidas pelo ambiente, se houver.
6. **Commitar** — `git add <arquivos do grupo>` (nunca `git add -A` às cegas) e `git commit`.
7. **Confirmar** — `git log --oneline -n <k>` e `git status` final; relate commits criados e o que ficou pendente.

## Proibido sem autorização explícita
`git reset --hard`, `git clean -fd`, `git push --force`, `git checkout -- .`, `git rebase -i`, amend de commits já publicados, `--no-verify`.
