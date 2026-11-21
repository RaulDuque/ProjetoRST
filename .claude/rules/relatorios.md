# Relatórios de execução (regra permanente)

O ChatGPT planeja e revisa lendo o GitHub; o Claude Code executa. Por isso **toda** tarefa
termina com um relatório versionado em `docs/relatorios/`. Isso vale para prompt, tarefa,
implementação, correção, investigação, teste ou etapa artística, inclusive quando a tarefa
fica bloqueada, parcial ou rejeitada.

## Fluxo ao finalizar

1. Conferir a branch e o `git status`.
2. Rodar as verificações pertinentes:
   - código: `cargo fmt`, `clippy`, `test` e `check`;
   - documentação e arte: `git diff --check`.
3. Escrever o relatório seguindo `docs/relatorios/PADRAO_RELATORIO.md`, com os 27 itens e
   exatamente um status:
   - CONCLUÍDO
   - CONCLUÍDO COM RESSALVAS
   - PARCIALMENTE CONCLUÍDO
   - BLOQUEADO
   - REJEITADO
   - AGUARDANDO APROVAÇÃO VISUAL
4. Onde salvar:
   - etapas do Erro-Corretion: `docs/relatorios/erro-corretion/<etapa>_<TITULO>.md`;
   - outros trabalhos: `docs/relatorios/<frente>/AAAA-MM-DD_<TITULO>.md`.
5. Acrescentar uma linha **no fim** de `docs/relatorios/INDICE.md` e reescrever
   `docs/relatorios/ULTIMO_RELATORIO.md`.
6. Commit só dos arquivos da tarefa e dos relatórios. Nunca `Md-usario/`.
7. `git push` para a branch de trabalho, sem force, e confirmar a sincronização com
   `git status -sb`.

## Conteúdo

- Execução técnica bem-sucedida **não** é aprovação artística: informe as duas separadamente.
- Tarefas visuais (sprites, equipamentos, animações, cenários, efeitos, UI) produzem
  evidências em `evidence/`, citadas por caminho relativo à raiz.
- Links do GitHub só para arquivos já publicados. Não invente links, números nem evidências.
- Item sem informação: escreva "Não registrado".
- Não versionar caches, dependências, `target/`, temporários nem arquivos grandes sem necessidade.
- Relatórios antigos não são reescritos para esconder erros. Correções entram como seção
  `## Correção (AAAA-MM-DD)` e são registradas no índice.

## Resposta final no terminal

Curta. Só estes itens:
- etapa;
- status;
- resumo;
- caminho do relatório;
- principais imagens;
- branch;
- hash do commit;
- confirmação do push;
- próxima decisão.

O resto fica no relatório.

## Limites

- `Md-usario/` é do proprietário. O Claude não edita (há um deny em `.claude/settings.json`)
  e não commita essa pasta.
- O trecho que o proprietário acrescenta aos prompts dele está em
  `docs/relatorios/BLOCO_PARA_PROMPTS.md`.
- Sem force-push, merge ou rebase sem autorização. Nada de credenciais nos commits.
