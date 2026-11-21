# Erro-Corretion 03B — Refinamento visual da armadura e prova de redução

**Status:** AGUARDANDO APROVAÇÃO VISUAL
**Aprovação artística:** pendente. Não há registro de aprovação ou reprovação da 03B.
Depois dela, o trabalho seguiu para avaliar ferramentas novas (03C) e para um experimento
independente (03D).

> **Relatório recuperado em 2026-11-21.** Ele resume
> [`evidence/erro_corretion/etapa03b/RELATORIO.md`](../../../evidence/erro_corretion/etapa03b/RELATORIO.md).
> A data e hora da execução não foram registradas.

## 1. Identificação
Erro-Corretion 03B, branch `art/v1.2-protagonist`.

## 2. Data e hora
Não registradas. A etapa foi versionada no commit `bc4e9e5` em 2026-11-19 22:52 (-0300).

## 3. Objetivo
Refinar a armadura da 03 (ombreiras, dourados, materiais, gola e faixa de camisa no #4) e
testar a legibilidade em pixel art, sem iniciar o Prompt 04.

## 4. Prompt
`Md-usario/Erro-Corretion/03B_MODELAR_ARMADURA_PILOTO.md`.

## 5. Contexto inicial
A etapa 03 estava aprovada como piloto técnico.

## 6. Ferramentas
Blender 5.2.4 com EEVEE e Python com Pillow.

## 7. Consultados
Saídas das etapas 02, 02B e 03, somente para leitura.

## 8. Criados (commit `bc4e9e5`: 45 arquivos novos e 1 alterado)
- `scripts/build_armor_refined.py`, `scripts/compose_refined.py` e `scripts/pixel_readability.py`;
- `work/armor_refined_v002.blend`, versionado com 1,5 MB;
- `out/armor_build.json`, `out/fit_metrics_refined.json` e `out/pixel_palette_03b.json`;
- imagens em `out/`.

## 9. Modificados
`evidence/erro_corretion/.gitignore`, com uma exceção para versionar o `.blend` v002.
Segundo o próprio relatório, essa é a única alteração.

## 10. Removidos
Nenhum.

## 11. Alterações
- **Quadro #4:** inclinação da coluna de +10° e −7°. A camisa visível caiu de 29 para 1 texel.
- **Ombreiras:** facetadas, com ponta gótica.
- **Dourados:** faixas planas de 30 mm e 20 mm no lugar dos tubos.
- **Materiais e luz:** metal esmaltado e luz controlada.
- **Gola:** mais alta.

## 12. Decisões
- Uma borda dourada contínua na ombreira foi removida porque formava um "aro".
- As costas mais altas da gola foram descartadas.

## 13. Comandos
Estão no relatório original.

## 14–16. Métricas

| Pose | Camisa visível (antes → depois) | Silhueta nova (antes → depois) |
|---|---|---|
| idle | 0 → 0 | 239 → 285 |
| C1 #0 | 1 → 0 | 158 → 182 |
| C1 #2 | 1 → 1 | 132 → 167 |
| C1 #4 | 29 → 1 | 108 → 145 |

A prova de redução foi feita com a paleta N3.4 e com uma paleta complementar proposta.

## 17–19. Problemas
- **Corrigidos:** faixa do #4, frisos de arame e brilho de plástico.
- **Pendentes:**
  - a ombreira perto lê como medalhão no #0 e no #2;
  - sobra 1 texel de camisa no #4;
  - a gola quase some no #2;
  - as abas formam um "V" no #2;
  - a paleta complementar não foi aprovada.

## 20. Limitações
A prévia mistura resoluções. A prova de redução é experimental.

## 21–22. Evidências
- `evidence/erro_corretion/etapa03b/out/model_sheet_before_after.png`
- `evidence/erro_corretion/etapa03b/out/armor_isolated_refined.png`
- `evidence/erro_corretion/etapa03b/out/fit_summary_refined.png`
- `evidence/erro_corretion/etapa03b/out/fit_<pose>_refined.png`
- `evidence/erro_corretion/etapa03b/out/pixel_readability_test.png`

## 23. Links
- Relatório original: https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/erro_corretion/etapa03b/RELATORIO.md
- Commit: https://github.com/RaulDuque/ProjetoRST/commit/bc4e9e5

## 24. Resultado
A armadura está refinada. A legibilidade em pixel art é parcial.

## 25. Recomendação
Decidir se a rota Blender da 03B segue para o Prompt 04 ou se fica aguardando o resultado
da 03D.

## 26. Decisões do proprietário
- Aprovar ou reprovar visualmente a 03B.
- Aprovar ou não a paleta complementar.

## 27. Git
Commit `bc4e9e5`, já publicado.
