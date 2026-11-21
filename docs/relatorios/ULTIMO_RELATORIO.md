# Último relatório

| | |
|---|---|
| **Tarefa** | Erro-Corretion 03D, Take 03: preparação do pedido |
| **Data** | 2026-11-21 |
| **Status** | CONCLUÍDO (sem imagem ainda; não há aprovação artística a dar) |
| **Relatório completo** | [`docs/relatorios/erro-corretion/03D_TAKE03_PEDIDO.md`](erro-corretion/03D_TAKE03_PEDIDO.md) |
| **Branch** | `art/v1.2-protagonist` |

## Resumo

- **Pedido do Take 03:** gerado com o `ask-pose` do ASF, em
  `evidence/erro_corretion/etapa03d/request_pose_take03/`. A referência é idêntica à dos
  takes anteriores.
- **Mudanças em relação ao Take 02:**
  - pixels da armadura do tamanho dos quadrados do cabelo;
  - ombreira de 2 placas;
  - dourado só em faixas com 2 quadrados ou mais;
  - no máximo 4 roxos e 2 dourados;
  - saída de 1024×1024 com nada repintado fora da armadura.
- **Passo a passo:** `evidence/erro_corretion/etapa03d/prompts/PROMPT_03D_take03.md`, com o
  ChatGPT na Parte A e o texto para o Claude Code na Parte B.

## Próximo passo

O proprietário gera a imagem no ChatGPT e a salva em
`evidence/erro_corretion/etapa03d/inbox/armour_master_03.png`. Depois cola a Parte B no
Claude Code.

O Prompt 04 não foi iniciado.
