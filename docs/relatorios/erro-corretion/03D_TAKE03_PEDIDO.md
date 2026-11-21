# Erro-Corretion 03D — Take 03: preparação do pedido (sem imagem ainda)

**Status:** CONCLUÍDO
**Aprovação artística:** não se aplica. Nenhuma imagem foi gerada ou processada; isso
depende do proprietário no ChatGPT.

## 1. Identificação
- Etapa: Erro-Corretion 03D, Take 03, só a preparação do pedido.
- Branch: `art/v1.2-protagonist`.

## 2. Data e hora
2026-11-21, por volta das 15:15–15:30 (-0300).

## 3. Objetivo original
Criar o prompt do Take 03 a pedido do proprietário ("pode criar um prompt para meu take 3").

## 4. Prompt executado
Pedido feito no chat. Os prompts produzidos estão em:
- `evidence/erro_corretion/etapa03d/request_pose_take03/prompt.txt`, que vai para o ChatGPT;
- `evidence/erro_corretion/etapa03d/prompts/PROMPT_03D_take03.md`, com o passo a passo e o
  texto para o Claude Code.

## 5. Contexto técnico inicial
- O Take 01 teve a arte não aprovada: dourado fino, quilha ilegível e ombreira sobre o cabelo.
- O Take 02 foi REJEITADO porque usou a mesma imagem do Take 01.
- O pedido do Take 02 (`request_pose/take02_prompt.txt`) nunca foi usado.

## 6. Ferramentas
Agent Sprite Forge 0.6.0, `generate2dsprite/scripts/pixel_look.py ask-pose`, rodado offline
no ambiente isolado `~/.local/share/dc-art-tools/asf-venv`.

## 7. Arquivos consultados
- `evidence/erro_corretion/etapa03d/request_pose/` e `request_pose_take02/`, comparando
  `request.json` e `master.png`;
- `evidence/erro_corretion/etapa03d/runs/take01/RELATORIO_take01.md`, para os defeitos;
- `evidence/erro_corretion/etapa03d/inbox/LEIA.md`.

## 8. Arquivos criados
- `evidence/erro_corretion/etapa03d/request_pose_take03/`, com `reference.png`,
  `prompt.txt`, `request.json` e `master.png`;
- `evidence/erro_corretion/etapa03d/prompts/PROMPT_03D_take03.md`;
- `docs/relatorios/erro-corretion/03D_TAKE03_PEDIDO.md` (este arquivo).

## 9. Arquivos modificados
- `evidence/erro_corretion/etapa03d/inbox/LEIA.md`: seção "Take 03" acrescentada no fim,
  sem mudar o resto;
- `docs/relatorios/INDICE.md`: nova linha;
- `docs/relatorios/ULTIMO_RELATORIO.md`.

## 10. Arquivos removidos
Nenhum. As pastas de teste do `--room` ficaram no scratchpad da sessão, fora do projeto.

## 11. Descrição das alterações
O `ask-pose` foi rodado com o mesmo master, um texto de mudança novo e
`--room 45,5,45,2`. Esse valor reproduz a grade dos takes anteriores: célula de 9 px,
master em (45, 5) e canvas de 1024×1024.

## 12. Decisões técnicas e artísticas
- **Pixels inteiros:** a armadura deve usar quadrados do mesmo tamanho dos do cabelo e das
  botas. No Take 01, o ChatGPT desenhou com meia célula.
- **Ombreira com 2 placas em vez de 3:** com cerca de 6 a 8 texels de altura em 1×, três
  lâminas com dourado de 2 px não cabem e viram listras.
- **Dourado restrito:** só na gola, nas bordas das placas da ombreira, no cinto e nas
  escarcelas, sempre com 2 quadrados ou mais. Sem listras, rebites ou gravação. No máximo
  4 roxos e 2 dourados.
- **Mantidos do pedido do Take 02:** quilha em duas colunas e cabelo por cima da armadura.
- **Saída de 1024×1024 com nada repintado fora da armadura:** o Take 01 veio com
  1254×1254 px e cerca de 40 % das células fora da área pedida repintadas.
- **Novo nome de arquivo** (`armour_master_03.png`): evita repetir o erro do Take 02.
- **Conferência de hash:** o prompt para o Claude manda conferir o hash antes de processar.

## 13. Comandos relevantes

```bash
~/.local/share/dc-art-tools/asf-venv/bin/python -I -B \
  ~/.claude/plugins/cache/agent-sprite-forge/agent-sprite-forge/0.6.0/skills/generate2dsprite/scripts/pixel_look.py \
  ask-pose --master request_pose/master.png --change "<texto do request.json>" \
  --room 45,5,45,2 --key green --outline rule --output-dir request_pose_take03
```

## 14. Testes realizados
- Três valores de `--room` comparados com `request_pose/reference.png`.
- Comparação de `reference.png`, `master.png` e `request.json` com os do Take 02.

## 15. Resultado dos testes
- `--room 4,8,4,0` deu célula 8: diferente, descartado.
- `--room 45,5,45,2` deu uma **referência idêntica byte a byte**.
- O `master.png` também é idêntico.
- No `request.json`, só o campo `change` difere do Take 02.

## 16. Métricas
- O prompt tem 410 palavras, segundo o `ask-pose`.
- Célula de 9 px, master em (45, 5), canvas de 1024×1024.

## 17–19. Problemas
- **Encontrado:** sem `--room`, o `ask-pose` recusa a célula de 9 px.
- **Corrigido:** o valor de `--room` que reproduz a grade foi identificado.
- **Pendente:** a imagem do ChatGPT.

## 20. Limitações
O ChatGPT pode não obedecer à grade nem ao tamanho de 1024×1024. O processamento lê
grades proporcionais, mas pinceladas menores que a célula continuam se perdendo.

## 21–22. Evidências visuais
- `evidence/erro_corretion/etapa03d/request_pose_take03/reference.png`: a imagem a anexar no ChatGPT.

## 23. Links do GitHub (válidos depois do push)
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/erro_corretion/etapa03d/prompts/PROMPT_03D_take03.md
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/erro_corretion/etapa03d/request_pose_take03/prompt.txt

## 24. Resultado final
O pedido do Take 03 está pronto para o ChatGPT.

## 25. Recomendações
Gerar a imagem em uma conversa nova, conferir a lista da Parte A e, então, colar a Parte B
no Claude Code.

## 26. Decisões do proprietário
- Gerar a imagem.
- Aceitar a mudança de 3 para 2 placas na ombreira, ou pedir para manter 3.

## 27. Estado do Git
- Branch: `art/v1.2-protagonist`.
- Commit: o que adicionou este arquivo. O hash está na resposta final.
- Push: sem force.
