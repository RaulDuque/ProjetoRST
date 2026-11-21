# Erro-Corretion 03 — Piloto de armadura gótica roxa e dourada

**Status:** CONCLUÍDO COM RESSALVAS
**Aprovação artística:** aprovada **como piloto técnico**, com refinamento artístico
pendente. Fonte: a abertura de `Md-usario/Erro-Corretion/03B_MODELAR_ARMADURA_PILOTO.md`.

> **Relatório recuperado em 2026-11-21.** Ele resume
> [`evidence/erro_corretion/etapa03/RELATORIO.md`](../../../evidence/erro_corretion/etapa03/RELATORIO.md).
> A data e hora da execução e as versões não foram registradas no original.

## 1. Identificação
Erro-Corretion 03, branch `art/v1.2-protagonist`.

## 2. Data e hora
Não registradas. A etapa foi versionada no commit `77fecdc` em 2026-11-19 18:46 (-0300).

## 3. Objetivo
Uma armadura de tronco e ombros no Blender, encaixada nas 4 poses sobre o protagonista
**vestido**.

## 4. Prompt
`Md-usario/Erro-Corretion/03_MODELAR_ARMADURA_PILOTO.md`.

## 5. Contexto inicial
A 02B estava aprovada, com as camadas e a ordem de sobreposição definidas.

## 6. Ferramentas
Blender e Python com Pillow. As versões não foram registradas neste relatório; a 03B
registra Blender 5.2.4.

## 7. Consultados
Saídas das etapas 02 e 02B: `targets.json`, `fit_results.json`, `layers_report.json` e o
`.blend` de ombros.

## 8. Criados (commit `77fecdc`, 38 arquivos)
- `scripts/build_armor.py` e `scripts/compose_preview.py`;
- `out/armor_build.json`, `out/fit_metrics.json` e `out/sha256_before.txt`;
- `out/model_sheet.png`, `out/fit_summary.png` e `out/fit_<pose>.png`.

Os arquivos `.blend` de trabalho não foram versionados.

## 9–10. Modificados e removidos
Nenhum.

## 11. Alterações
- 90 objetos de geometria própria: couraça com quilha, gorjal, 3 lâminas, 4 pares de abas
  e ombreiras.
- Cada peça presa rigidamente a um osso.
- Nova ação `FIT03_idle`.

## 12. Decisões
- Materiais roxo metálico e dourado.
- Ordem de camadas da 02B.
- A redução para pixel art ficou para o Prompt 04.

## 13. Comandos
Estão no relatório original.

## 14–16. Métricas
Camisa ainda visível: 0 texels no idle, 1 no #0, 1 no #2 e **29 no #4**. Silhueta nova:
238 no idle, 158 no #0, 132 no #2 e 107 no #4.

## 17–19. Problemas
- **Encontrados:**
  - faixa de camisa visível no #4;
  - visual de CG, com frisos finos como arame e ombreiras esféricas;
  - a prévia mistura resoluções.
- **Corrigidos:** nenhum nesta etapa. Foram tratados na 03B.
- **Pendentes:** quantização na paleta, integração no Bevy e GIF.

## 20. Limitações
A prévia não representa o resultado final em pixel art.

## 21–22. Evidências
- `evidence/erro_corretion/etapa03/out/model_sheet.png`
- `evidence/erro_corretion/etapa03/out/fit_summary.png`
- `evidence/erro_corretion/etapa03/out/fit_idle.png`, `out/fit_run_c1_0.png`,
  `out/fit_run_c1_2.png` e `out/fit_run_c1_4.png`

## 23. Links
- Relatório original: https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/erro_corretion/etapa03/RELATORIO.md
- Commit: https://github.com/RaulDuque/ProjetoRST/commit/77fecdc

## 24. Resultado
O piloto técnico está pronto; a arte ainda precisa de refinamento.

## 25. Recomendação
Etapa 03B. Já foi executada.

## 26. Decisões do proprietário
Já tomada: refinar na 03B.

## 27. Git
Commit `77fecdc`, já publicado.
