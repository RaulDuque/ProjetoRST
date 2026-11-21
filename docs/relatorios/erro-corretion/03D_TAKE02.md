# Erro-Corretion 03D — Take 02: leitura repetida da imagem do Take 01

**Status:** REJEITADO
**Aprovação artística:** não se aplica. **Este take não é uma segunda tentativa artística
válida.**

> **Motivo da rejeição.** A imagem de entrada, `inbox/armour_master.png`, é **a mesma do
> Take 01**, byte a byte:
> - o sha256 `d5aed573380b521e1293c79bc45e2507a584133cda14ab4b64869ccfda2a7d9c` é igual
>   para `runs/take01/source.png` e `runs/take02/source.png`;
> - o texto `request_pose/take02_prompt.txt` nunca foi usado para gerar uma imagem.
>
> O fluxo rodou por pedido explícito do proprietário, e por isso o resultado é idêntico ao
> do Take 01.
>
> Relatório original:
> [`evidence/erro_corretion/etapa03d/runs/take02/RELATORIO_take02.md`](../../../evidence/erro_corretion/etapa03d/runs/take02/RELATORIO_take02.md).

## 1. Identificação
Erro-Corretion 03D, Take 02. Branch `art/v1.2-protagonist`.

## 2. Data e hora
2026-11-21, entre cerca de 14:30 e 14:41 (-0300). O commit `5fe8101` é das 14:41.

## 3. Objetivo
Processar a imagem mestre do Take 02 pelo fluxo oficial, restaurar o cabelo com as camadas
N4, comparar com o Take 01, avaliar e publicar.

## 4. Prompt
`evidence/erro_corretion/etapa03d/prompts/PROMPT_03D_take02.md`, salvo a pedido do proprietário.

## 5. Contexto inicial
- O Take 01 estava pronto e a preparação do Take 02 já fora versionada (`4feec7f`).
- A conferência inicial mostrou que a inbox ainda tinha a imagem do Take 01.
- O processamento foi suspenso e o problema relatado.
- O proprietário pediu para processar mesmo assim.

## 6. Ferramentas
Agent Sprite Forge 0.6.0 (`pixel_look.py pose`), offline, e Python com Pillow.

## 7. Consultados
- `evidence/erro_corretion/etapa03d/inbox/LEIA.md`
- `evidence/erro_corretion/etapa03d/request_pose_take02/`
- os resultados do Take 01
- o N3.5 aprovado e as camadas N4 de cabelo, só leitura

## 8. Criados (commit `5fe8101`)
- `evidence/erro_corretion/etapa03d/runs/take02/`: tudo o que está nesta pasta;
- `evidence/erro_corretion/etapa03d/scripts/compare_take02_sheet.py`: folha com o
  aprovado, o Take 01, o Take 02 bruto e o Take 02 com cabelo N4;
- `evidence/erro_corretion/etapa03d/prompts/PROMPT_03D_take02.md`.

## 9–10. Modificados e removidos
Nenhum.

## 11. Alterações
- Rodado o fluxo oficial completo: `import_external`, `take_report`, `restore_hair` e `compare_takes`.
- Gerada a folha extra de comparação.

## 12. Decisões
- Processar só depois da confirmação do proprietário.
- Marcar o resultado como rejeitado.

## 13. Comandos
Estão no relatório original, na seção "Reprodução".

## 14–15. Testes
- Comparação de hash entre as entradas: idênticas.
- Comparação entre os sprites finais: **0 pixels diferentes** entre o Take 01 e o Take 02,
  ambos com cabelo N4.

## 16. Métricas
Todas iguais às do Take 01:

| Medida | Valor |
|---|---|
| Grade | célula de 11,021 px |
| Encaixe | (26, 13) |
| Cabeça | 175 de 175 idênticos |
| Pernas | 465 de 465 idênticos |
| Tronco | 465 alterados |
| Total alterado | 562 (538 depois do cabelo) |
| Silhueta | +97 e −2 |
| Cores | +12 |
| Cabelo | 24 restaurados, 0 diferentes depois |

## 17–19. Problemas
- **Encontrado:** a entrada estava errada, porque a imagem nova não foi colocada na inbox.
- **Corrigidos:** nenhum.
- **Pendente:** gerar a imagem real do Take 02.

## 20. Limitações
Sem uma imagem nova, não há o que avaliar artisticamente.

## 21–22. Evidências
- `evidence/erro_corretion/etapa03d/runs/take02/takes_01_vs_02_comparison.png`
- `evidence/erro_corretion/etapa03d/runs/take02/takes_dark_preview.png`
- `evidence/erro_corretion/etapa03d/runs/take02/take02_comparison_sheet.png`
- `evidence/erro_corretion/etapa03d/runs/take02/take02_torso_source_vs_1x.png`
- `evidence/erro_corretion/etapa03d/runs/take02/armour_on_canvas_80x128.png`

## 23. Links
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/erro_corretion/etapa03d/runs/take02/RELATORIO_take02.md
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/erro_corretion/etapa03d/runs/take02/takes_01_vs_02_comparison.png
- https://github.com/RaulDuque/ProjetoRST/commit/5fe8101

## 24. Resultado
**REJEITADO.** É uma repetição do Take 01. Serve só como prova de que o fluxo é
reproduzível.

## 25. Recomendação
Gerar no ChatGPT uma imagem nova com `request_pose/reference.png` e
`request_pose/take02_prompt.txt`, salvar como `inbox/armour_master_02.png` e processar como
um novo take.

## 26. Decisões do proprietário
- Gerar a imagem nova.
- Decidir se o próximo take reaproveita o nome `take02` ou passa a se chamar `take03`. O
  histórico deste take continua registrado de qualquer forma.

## 27. Git
Commit `5fe8101` na branch `art/v1.2-protagonist`, publicado e sincronizado com o remoto.
