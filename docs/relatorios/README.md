# Relatórios de execução

Esta pasta reúne, em um só lugar versionado, os relatórios de tudo o que o Claude Code
executa no Dark Castle RPG.

- **ChatGPT:** planeja, acompanha, revisa a parte técnica e avalia a arte. Lê estes
  relatórios direto do GitHub, sem precisar de cópias das respostas do Claude.
- **Claude Code:** executa as tarefas e escreve os relatórios.
- **Proprietário:** decide e aprova.

## Por onde começar

| Arquivo | Para quê |
|---|---|
| [`ULTIMO_RELATORIO.md`](ULTIMO_RELATORIO.md) | **A última tarefa executada**: status, resumo e link do relatório completo |
| [`INDICE.md`](INDICE.md) | Histórico cronológico de todos os relatórios |
| [`PADRAO_RELATORIO.md`](PADRAO_RELATORIO.md) | Modelo obrigatório, estados de status e convenção de nomes |
| [`erro-corretion/`](erro-corretion/) | Relatórios das etapas do Erro-Corretion (armadura e equipamentos) |
| [`processo/`](processo/) | Relatórios de tarefas de processo e documentação |

## Regra permanente

Ao terminar **qualquer** prompt, tarefa, implementação, correção, investigação, teste ou
etapa artística, inclusive se ficou bloqueada, parcial ou rejeitada:

1. conferir a branch e o `git status`;
2. rodar as verificações pertinentes;
3. escrever o relatório completo seguindo o [`PADRAO_RELATORIO.md`](PADRAO_RELATORIO.md);
4. acrescentar uma linha ao [`INDICE.md`](INDICE.md) e atualizar o [`ULTIMO_RELATORIO.md`](ULTIMO_RELATORIO.md);
5. fazer commit só com os arquivos da tarefa e a documentação;
6. fazer push para a branch de trabalho, sem force, e confirmar que local e remoto estão sincronizados;
7. responder no terminal só com o resumo curto: etapa, status, resumo, caminho do
   relatório, principais imagens, branch, hash do commit, confirmação do push e próxima decisão.

A mesma regra está no [`CLAUDE.md`](../../CLAUDE.md) e em
[`.claude/rules/relatorios.md`](../../.claude/rules/relatorios.md).

## O que fica fora

- **`Md-usario/`:** guarda os prompts pessoais do proprietário. Não entra nos commits do
  Claude, e o Claude não edita essa pasta (é bloqueada em `.claude/settings.json`). O
  trecho a acrescentar aos prompts está em [`BLOCO_PARA_PROMPTS.md`](BLOCO_PARA_PROMPTS.md).
- **Imagens, GIFs, modelos e testes:** ficam em `evidence/`. Os relatórios apontam para lá
  com caminhos relativos à raiz do repositório.
- **Relatórios técnicos originais:** os de cada etapa continuam em
  `evidence/erro_corretion/etapaNN/`. Os desta pasta seguem o padrão, resumem a etapa e
  apontam para o original.

## Estrutura

```
docs/relatorios/
├── README.md               este arquivo
├── PADRAO_RELATORIO.md     modelo e regras
├── ULTIMO_RELATORIO.md     última tarefa
├── INDICE.md               histórico
├── BLOCO_PARA_PROMPTS.md   trecho para os prompts de Md-usario/
├── erro-corretion/         01, 02, 02B, 03, 03B, 03C, 03D...
└── processo/               tarefas de processo e documentação
```
