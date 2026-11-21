# Erro-Corretion 02B — Manequim ajustado e camadas da base vestida

**Status:** CONCLUÍDO
**Aprovação artística:** aprovada para seguir. O Prompt 03 exige a aprovação explícita da
02B como pré-requisito, e a etapa 03 foi executada. Essa aprovação está inferida a partir
do pré-requisito; não há registro escrito separado.

> **Relatório recuperado em 2026-11-21.** Ele resume
> [`evidence/erro_corretion/etapa02b/RELATORIO.md`](../../../evidence/erro_corretion/etapa02b/RELATORIO.md).
> A data e hora da execução e as versões não foram registradas.

## 1. Identificação
Erro-Corretion 02B, branch `art/v1.2-protagonist`.

## 2. Data e hora
Não registradas. A etapa foi versionada no commit `ba5ce2a` em 2026-11-19 18:20 (-0300).

## 3. Objetivo
Corrigir os ombros da corrida, separar a manga do idle e devolver à cabeça o cabelo
classificado como braço. Tudo sem armadura e sem corpo nu.

## 4. Prompt
`Md-usario/Erro-Corretion/02B_AJUSTAR_MANEQUIM_E_CAMADAS.md`.

## 5. Contexto inicial
O resultado da etapa 02 era "PRECISA DE AJUSTE".

## 6. Ferramentas
Blender em modo background e Python com Pillow. As versões não foram registradas.

## 7. Consultados
- Saídas da etapa 02: `targets.json`, `fit_results.json` e o `.blend` de trabalho.
- Camadas da N4.

## 8. Criados (commit `ba5ce2a`, 70 arquivos)
- `scripts/fit_shoulders.py` e `scripts/build_layers.py`;
- `out/shoulders.json` e `out/layers_report.json`;
- `out/layers/<pose>/*.png`;
- `out/reconstruction_<pose>.png`, `out/preview_<pose>.png`, `out/zones_<pose>.png` e `out/hair_fix.png`;
- `work/swordsman_rig_v007_02b_*.blend`.

Todos os caminhos ficam em `evidence/erro_corretion/etapa02b/`.

## 9–10. Modificados e removidos
Nenhum. O `sha256sum -c` da 02B e da 02 passou.

## 11. Alterações
- **Ombros:** clavículas ajustadas nos quadros #0 e #2.
- **Manga do idle:** separada em `sleeve_near` (192 texels) e `shirt_torso` (105).
- **Cabelo e rosto:** texels devolvidos à cabeça, 53 no #0, 47 no #2 e 34 no #4.
- **Ordem de camadas:** definida com os slots de armadura (seção 4 do original).

## 12. Decisões
A ordem de sobreposição aprovada é:

`membros de trás → ombreira longe → calça/botas/tronco → couraça → manga perto → ombreira perto → mão/rosto/cabelo`.

## 13. Comandos
Estão no relatório original.

## 14–16. Testes e métricas
- Ombro perto: de 2,85 para 0,03 px no #0 e de 4,63 para 0,01 px no #2.
- **Reconstrução sem armadura: 0 pixels divergentes nas 4 poses.**

## 17–19. Problemas
- **Corrigidos:** ombros, manga e cabelo.
- **Pendentes:**
  - cotovelo perto entre 4,7 e 4,8 px;
  - pescoço entre 2,7 e 3,9 px;
  - pernas sem ajuste;
  - `hand_near` do idle carregando cinto e calça.

## 20. Limitações
Os recortes da manga e das partições são interpretações e dependem de aprovação visual.

## 21–22. Evidências
- `evidence/erro_corretion/etapa02b/out/preview_<pose>.png`
- `evidence/erro_corretion/etapa02b/out/zones_<pose>.png`
- `evidence/erro_corretion/etapa02b/out/hair_fix.png`
- `evidence/erro_corretion/etapa02b/out/reconstruction_<pose>.png`

## 23. Links
- Relatório original: https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/erro_corretion/etapa02b/RELATORIO.md
- Commit: https://github.com/RaulDuque/ProjetoRST/commit/ba5ce2a

## 24. Resultado
A base vestida foi dividida em camadas sem perda.

## 25. Recomendação
Etapa 03, o piloto de armadura. Já foi executada.

## 26. Decisões do proprietário
Já tomada: seguir para a etapa 03.

## 27. Git
Commit `ba5ce2a`, já publicado.
