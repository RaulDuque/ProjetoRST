# Bloco para acrescentar aos prompts de `Md-usario/Erro-Corretion/`

O Claude Code não edita `Md-usario/`. A pasta é bloqueada por `Edit(/Md-usario/**)` em
`.claude/settings.json`, e o combinado é que só o proprietário mexe nela. Por isso o trecho
abaixo fica aqui, para o proprietário acrescentar ao final de cada prompt. O restante de
cada prompt continua igual.

Para acrescentar em todos os prompts de uma vez, rode da raiz do projeto (no Claude Code,
com `!` na frente):

```bash
for f in Md-usario/Erro-Corretion/0*.md; do printf '\n\n' >> "$f"; sed -n '/^<!-- BLOCO -->$/,/^<!-- FIM -->$/p' docs/relatorios/BLOCO_PARA_PROMPTS.md | sed '1d;$d' >> "$f"; done
```

O comando acrescenta o bloco sem apagar nada. Rode uma vez só, porque rodar de novo
duplica o bloco.

<!-- BLOCO -->
## Relatório obrigatório (regra permanente)

Ao finalizar esta etapa, inclusive se ficar bloqueada, parcial ou rejeitada:

1. Escreva o relatório completo em `docs/relatorios/erro-corretion/<etapa>_<TITULO>.md`,
   seguindo `docs/relatorios/PADRAO_RELATORIO.md` (27 itens e um dos status: CONCLUÍDO,
   CONCLUÍDO COM RESSALVAS, PARCIALMENTE CONCLUÍDO, BLOQUEADO, REJEITADO ou AGUARDANDO
   APROVAÇÃO VISUAL). Execução técnica não é aprovação artística.
2. Liste as imagens, GIFs e vídeos com caminhos relativos à raiz do repositório e, depois
   do push, com links do GitHub. Não invente links nem evidências.
3. Acrescente a linha em `docs/relatorios/INDICE.md` e atualize
   `docs/relatorios/ULTIMO_RELATORIO.md`.
4. Faça commit dos arquivos da etapa e dos relatórios (nunca de `Md-usario/`), push sem
   force para a branch de trabalho, e confirme a sincronização.
5. No terminal, responda só com: etapa, status, resumo, caminho do relatório, principais
   imagens, branch, hash do commit, confirmação do push e próxima decisão.
<!-- FIM -->
