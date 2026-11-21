# Padrão de relatório de execução

Todo prompt, tarefa, implementação, correção, investigação, teste ou etapa artística termina
com um relatório neste formato, **inclusive** quando a tarefa ficou bloqueada, parcial ou
rejeitada. As regras gerais estão em [`README.md`](README.md).

## Onde salvar e como nomear

| Tipo de trabalho | Pasta | Nome do arquivo |
|---|---|---|
| Etapas do Erro-Corretion | `docs/relatorios/erro-corretion/` | `<etapa>_<TITULO_CURTO>.md`, por exemplo `03D_TAKE03.md` |
| Processo, documentação e regras | `docs/relatorios/processo/` | `AAAA-MM-DD_<TITULO_CURTO>.md` |
| Outras frentes (código, arte, mundo...) | `docs/relatorios/<frente>/` | `AAAA-MM-DD_<TITULO_CURTO>.md` |

Depois de salvar o relatório:
1. acrescente uma linha **no fim** de [`INDICE.md`](INDICE.md);
2. substitua o conteúdo de [`ULTIMO_RELATORIO.md`](ULTIMO_RELATORIO.md) pelo resumo desta tarefa;
3. faça commit e push junto com os arquivos da tarefa.

## Status (escolha exatamente um)

| Status | Quando usar |
|---|---|
| **CONCLUÍDO** | Tudo o que foi pedido foi feito e validado. |
| **CONCLUÍDO COM RESSALVAS** | Feito e validado, mas com limitações ou defeitos conhecidos, registrados no relatório. |
| **PARCIALMENTE CONCLUÍDO** | Só parte do pedido foi feita. |
| **BLOQUEADO** | Não foi possível avançar por um impedimento externo (ferramenta, permissão, dado ausente). |
| **REJEITADO** | O resultado foi executado, mas não serve como entrega válida. |
| **AGUARDANDO APROVAÇÃO VISUAL** | Execução técnica concluída; falta o julgamento artístico do proprietário. |

**Execução técnica bem-sucedida não é aprovação artística.** Quando houver arte envolvida,
informe as duas coisas separadamente: o status técnico e se a arte já foi aprovada.

## Modelo

Copie o bloco abaixo. Se um item não se aplicar, escreva "Não se aplica." Se a informação
não existir, escreva "Não registrado." Nunca invente.

```markdown
# <Etapa ou tarefa> — <título>

**Status:** <um dos seis estados>
**Aprovação artística:** <aprovada | pendente | reprovada | não se aplica>

## 1. Identificação
- Etapa: <código e nome>
- Frente: <erro-corretion, processo, ...>
- Branch: <branch>

## 2. Data e hora da execução
<AAAA-MM-DD HH:MM (fuso)>

## 3. Objetivo original
<o que foi pedido, em poucas linhas>

## 4. Prompt executado
<caminho do prompt no repositório, ou o texto, ou "pedido feito no chat (não versionado)">

## 5. Contexto técnico inicial
<estado do projeto e das etapas anteriores antes de começar>

## 6. Ferramentas, plugins, skills e versões
<Blender x.y, Python x.y, Agent Sprite Forge x.y, Rust/Bevy, etc.>

## 7. Arquivos consultados
## 8. Arquivos criados
## 9. Arquivos modificados
## 10. Arquivos removidos

## 11. Descrição detalhada das alterações
## 12. Decisões técnicas e artísticas
## 13. Comandos relevantes executados
## 14. Testes realizados
## 15. Resultado dos testes
## 16. Métricas e comparações
## 17. Problemas encontrados
## 18. Problemas corrigidos
## 19. Problemas pendentes
## 20. Limitações técnicas

## 21. Evidências visuais
<o que cada imagem mostra>

## 22. Caminhos das imagens, GIFs e vídeos
<caminhos relativos à raiz do repositório>

## 23. Links do GitHub
<somente para arquivos que já estão versionados e publicados>

## 24. Resultado final
## 25. Recomendações para a próxima etapa
## 26. Decisões que dependem do proprietário

## 27. Estado do Git e publicação
- Branch:
- Commit(s):
- Push: <feito / não feito e por quê>
- Local x remoto: <sincronizado / divergente>
```

## Evidências visuais

- Tarefas com sprites, personagens, equipamentos, animações, cenários, efeitos ou interface
  sempre produzem imagens de avaliação, como prévias em 1×, 2× e ampliadas, comparações
  antes e depois, ou GIFs.
- Use caminhos relativos à raiz do repositório, como
  `evidence/erro_corretion/etapa03d/runs/take02/takes_dark_preview.png`.
- Links do GitHub seguem o formato
  `https://github.com/RaulDuque/ProjetoRST/blob/<branch>/<caminho>`, e só valem depois do push.
- Não versione caches, dependências, `target/`, arquivos temporários nem arquivos grandes
  sem necessidade.

## Correções em relatórios antigos

Relatórios publicados não são reescritos para esconder erros ou mudar resultados. Para
corrigir um relatório, acrescente ao final uma seção `## Correção (AAAA-MM-DD)` dizendo o
que estava errado, o valor correto e onde está a evidência. Registre a correção também no
`INDICE.md`.
