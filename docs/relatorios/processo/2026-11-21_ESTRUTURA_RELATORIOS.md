# Processo — Estrutura de relatórios e regra permanente de documentação

**Status:** CONCLUÍDO COM RESSALVAS
**Aprovação artística:** não se aplica.

**Ressalva:** os prompts de `Md-usario/Erro-Corretion/` **não foram editados**. A pasta é
bloqueada para o Claude por `Edit(/Md-usario/**)` em `.claude/settings.json`, e já existe uma
decisão do proprietário, de 2026-11-15, de que só ele mexe nela. O bloco a acrescentar e um
comando para aplicá-lo estão em `docs/relatorios/BLOCO_PARA_PROMPTS.md`.

## 1. Identificação
- Tarefa: criar `docs/relatorios/`, recuperar o histórico do Erro-Corretion e registrar a
  regra permanente de relatórios.
- Frente: processo.
- Branch: `art/v1.2-protagonist`.

## 2. Data e hora
2026-11-21, por volta das 14:45–15:10 (-0300).

## 3. Objetivo original
- Centralizar os relatórios de execução em uma pasta versionada, para que o ChatGPT possa
  lê-los no GitHub.
- Tornar obrigatório um relatório ao fim de toda tarefa.
- Recuperar o histórico das etapas 01 a 03D Take 02.

## 4. Prompt executado
Pedido feito no chat pelo proprietário, em 2026-11-21. Não está versionado como arquivo.

## 5. Contexto técnico inicial
- Os relatórios só existiam dentro de `evidence/erro_corretion/etapaNN/`, cada um com um formato.
- A etapa 01 não tinha relatório.
- O Take 02 da 03D acabara de ser publicado (`5fe8101`) e rejeitado.

## 6. Ferramentas
Claude Code (Claude Opus 5.5), git e shell. Nenhum plugin ou skill de arte foi usado.

## 7. Arquivos consultados
- `evidence/erro_corretion/etapa02/RELATORIO.md`
- `evidence/erro_corretion/etapa02b/RELATORIO.md`
- `evidence/erro_corretion/etapa03/RELATORIO.md`
- `evidence/erro_corretion/etapa03b/RELATORIO.md`
- `evidence/erro_corretion/etapa03c/RELATORIO.md`
- `evidence/erro_corretion/etapa03d/RELATORIO.md`
- `evidence/erro_corretion/etapa03d/runs/take01/RELATORIO_take01.md`
- `evidence/erro_corretion/etapa03d/runs/take02/RELATORIO_take02.md`
- `evidence/erro_corretion/etapa01/`, com o conteúdo e o commit `ec8bdac`
- `Md-usario/Erro-Corretion/*.md`, só leitura: títulos, `00_LEIA_PRIMEIRO.md` e a abertura
  do 03B
- `.claude/settings.json`
- o histórico `git log` das etapas

## 8. Arquivos criados
- `docs/relatorios/README.md`
- `docs/relatorios/PADRAO_RELATORIO.md`
- `docs/relatorios/ULTIMO_RELATORIO.md`
- `docs/relatorios/INDICE.md`
- `docs/relatorios/BLOCO_PARA_PROMPTS.md`
- `docs/relatorios/erro-corretion/01_VERIFICAR_RIG.md`
- `docs/relatorios/erro-corretion/02_ENCAIXE_POSES.md`
- `docs/relatorios/erro-corretion/02B_MANEQUIM_CAMADAS.md`
- `docs/relatorios/erro-corretion/03_ARMADURA_PILOTO.md`
- `docs/relatorios/erro-corretion/03B_REFINAMENTO.md`
- `docs/relatorios/erro-corretion/03C_NOVAS_SKILLS.md`
- `docs/relatorios/erro-corretion/03D_TAKE01.md`
- `docs/relatorios/erro-corretion/03D_TAKE02.md`
- `docs/relatorios/processo/2026-11-21_ESTRUTURA_RELATORIOS.md` (este arquivo)
- `.claude/rules/relatorios.md`

## 9. Arquivos modificados
`CLAUDE.md`: nova seção "Relatório ao final de toda tarefa (regra permanente)".

## 10. Arquivos removidos
Nenhum.

## 11. Descrição das alterações
- **Estrutura e regra:** criadas a estrutura pedida, o modelo de 27 itens com os 6 estados e
  a regra no `CLAUDE.md` e em `.claude/rules/relatorios.md`. Os arquivos de
  `.claude/rules/` são carregados automaticamente em toda sessão.
- **Histórico:** os 8 relatórios históricos seguem o padrão, apontam para os originais e
  marcam o que não foi registrado.
- **Relatórios originais:** os de `evidence/` **não foram alterados**.

## 12. Decisões
- **Pasta `processo/`:** criada para relatórios que não são etapas do Erro-Corretion, como
  este.
- **Bloco para `Md-usario/`:** o bloco para os prompts ficou em um arquivo versionado, com
  um comando que o proprietário roda. Editar `Md-usario/` contornaria o deny do
  `settings.json` e a decisão do proprietário de 2026-11-15.
- **Status das etapas antigas:** foram mapeados com base em textos existentes.
  - Etapa 02: "PRECISA DE AJUSTE" virou CONCLUÍDO COM RESSALVAS.
  - Etapa 03: CONCLUÍDO COM RESSALVAS, porque foi "aprovada como piloto técnico" segundo
    o prompt 03B.
  - Etapa 03B: AGUARDANDO APROVAÇÃO VISUAL, porque não há registro de aprovação.
  - A aprovação da 02B foi **inferida** do pré-requisito do Prompt 03, e isso está escrito
    no relatório.
- **Horários antigos:** onde a hora da execução não foi registrada, usei a hora do commit,
  identificada como tal.

## 13. Comandos relevantes

```bash
git log --reverse --format='%h %ad %s' -- evidence/erro_corretion/etapaNN
git show --name-status <commit>
git diff --check
```

## 14–15. Testes e resultado
`git diff --check` sem problemas. Não houve mudança de código, então não houve compilação.

## 16. Métricas
- 8 relatórios recuperados.
- 1 etapa sem relatório original: a 01.
- 1 etapa sem prompt versionado: a 03C.

## 17. Problemas encontrados
- `Md-usario/` não pode ser editado pelo Claude.
- A etapa 01 não tinha relatório.
- A 03C não tinha arquivo de prompt.
- Faltavam datas e horas de execução em todas as etapas antigas.

## 18. Problemas corrigidos
O histórico agora está centralizado e marca explicitamente as lacunas.

## 19. Problemas pendentes
O proprietário precisa acrescentar o bloco aos prompts de `Md-usario/Erro-Corretion/`.

## 20. Limitações
Os relatórios recuperados só têm o que estava nos documentos e commits. Nada foi completado
de memória.

## 21–22. Evidências visuais
Não se aplica, porque a tarefa é de documentação. As imagens das etapas estão citadas nos
relatórios de cada uma.

## 23. Links do GitHub
- https://github.com/RaulDuque/ProjetoRST/tree/art/v1.2-protagonist/docs/relatorios
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/docs/relatorios/ULTIMO_RELATORIO.md

## 24. Resultado final
A estrutura está criada, o histórico recuperado e a regra registrada. Fica a ressalva sobre
`Md-usario/`.

## 25. Recomendações
Gerar a imagem real do próximo take da 03D, com `request_pose/take02_prompt.txt`. O Prompt
04 não foi iniciado.

## 26. Decisões do proprietário
1. Rodar o comando de `docs/relatorios/BLOCO_PARA_PROMPTS.md` para atualizar os prompts, ou
   autorizar outra forma.
2. Decidir o nome do próximo take da 03D: `take02` ou `take03`.

## 27. Estado do Git
- Branch: `art/v1.2-protagonist`.
- Commit: o mesmo que adicionou este arquivo (`git log -1 -- docs/relatorios/processo/`).
  O hash está na resposta final do Claude.
- Push: feito sem force.
- `Md-usario/`: fora do commit.
