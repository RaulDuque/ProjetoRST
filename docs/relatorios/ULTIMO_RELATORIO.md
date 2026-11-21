# Último relatório

| | |
|---|---|
| **Tarefa** | Erro-Corretion 03D, Take 03 |
| **Data** | 2026-11-21 |
| **Status** | AGUARDANDO APROVAÇÃO VISUAL. Técnica OK; avaliação do Claude: melhor que o Take 01, mas insuficiente |
| **Relatório completo** | [`docs/relatorios/erro-corretion/03D_TAKE03.md`](erro-corretion/03D_TAKE03.md) |
| **Branch** | `art/v1.2-protagonist` |

## Resumo

- **Fluxo:** a imagem nova do ChatGPT foi lida pelo `pose` e encaixada em 80×128 (pivô
  40,118). A imagem estava em `request_pose_take03/` e foi movida para `inbox/`.
- **Preservação:** cabeça, rosto e pernas ficaram 100 % idênticos. O cabelo teve 3 texels
  trocados, que voltaram a 0 com a N4.
- **Ganhos:** a armadura ficou muito mais limpa e legível em 1× que a do Take 01.
- **Falhas:**
  - lê como túnica ou gibão;
  - a ombreira não tem volume de placa (silhueta +46 texels);
  - a quilha parece borda;
  - cerca de 60 % das faixas douradas têm só 1 px.

## Imagens principais

- `evidence/erro_corretion/etapa03d/runs/take03/takes_01_vs_03_comparison.png`
- `evidence/erro_corretion/etapa03d/runs/take03/takes_dark_preview.png`
- `evidence/erro_corretion/etapa03d/runs/take03/take03_torso_source_vs_1x.png`

## Próxima decisão

Escolher entre três caminhos:
- aprovar o Take 03;
- pedir um Take 04 (recomendações na seção 25 do relatório);
- voltar à rota Blender da 03B.

O Prompt 04 não foi iniciado.
