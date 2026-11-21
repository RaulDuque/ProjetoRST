# Erro-Corretion 02 — Prova de encaixe do rig nas quatro poses

**Status:** CONCLUÍDO COM RESSALVAS. No original, o resultado aparece como "PRECISA DE AJUSTE".
**Aprovação artística:** não se aplica, porque nenhuma armadura foi criada. O proprietário
aceitou seguir para a 02B (`00_LEIA_PRIMEIRO.md`).

> **Relatório recuperado em 2026-11-21.** Ele resume
> [`evidence/erro_corretion/etapa02/RELATORIO.md`](../../../evidence/erro_corretion/etapa02/RELATORIO.md),
> onde estão todos os detalhes e tabelas. A data e hora da execução e as versões das
> ferramentas, além do Blender, não foram registradas.

## 1. Identificação
- Etapa: Erro-Corretion 02, encaixe do rig v007 nas poses idle, C1 #0, #2 e #4.
- Branch: `art/v1.2-protagonist`.

## 2. Data e hora
Não registradas. A etapa foi versionada no commit `8b9a2b1` em 2026-11-19 17:52 (-0300).

## 3. Objetivo
Provar que o rig serve como manequim invisível nas quatro poses aprovadas, sem redesenhar
o personagem e sem criar armadura.

## 4. Prompt
`Md-usario/Erro-Corretion/02_TESTAR_ENCAIXE_DAS_POSES.md`.

## 5. Contexto inicial
A etapa 01 concluiu que o rig é reaproveitável com ajustes.

## 6. Ferramentas
Blender em modo background (Workbench) e Python com Pillow. As versões não foram
registradas no relatório.

## 7. Arquivos consultados
- `run_poses.py` (esqueletos da N4-1);
- `reconstruct.py` e `partition.py` (N4-A);
- `art-src/player/blender/swordsman_rig_v007.blend`;
- `candidate_106.png` e `run6.png`.

## 8. Arquivos criados (commit `8b9a2b1`, 30 arquivos)
- `scripts/build_targets.py`, `scripts/fit_rig.py` e `scripts/analyze.py`;
- `out/targets.json`, `out/fit_results.json` e `out/measurements.json`;
- `out/guide_*.png`, `out/fit_*.png`, `out/occlusion_*.png` e `out/summary.png`;
- `out/sha256_before.txt`;
- `work/swordsman_rig_v007_fit_poses.blend`, uma cópia de trabalho.

Todos os caminhos ficam em `evidence/erro_corretion/etapa02/`.

## 9–10. Modificados e removidos
Nenhum. O `sha256sum -c` confirmou que o rig e os sprites aprovados ficaram intactos.

## 11. Alterações
FK no plano lateral, com 4 passes de reajuste da raiz. Câmera ortográfica com escala de
58,53 px/m e redução para 1× pelo critério da N3.5.

## 12. Decisões
- Os comprimentos dos ossos não mudam.
- As máscaras de oclusão do braço devem sair das **camadas aprovadas**, não do render do rig.

## 13. Comandos
Estão no relatório original, na seção "Para reproduzir".

## 14–16. Testes e métricas

| Pose | Erro médio | Erro máximo |
|---|---|---|
| idle | 1,63 px | 6,14 (joelho) |
| C1 #0 | 2,53 px | 7,07 |
| C1 #2 | 3,07 px | 6,07 |
| C1 #4 | 2,12 px | 5,78 |

- Os erros vêm de **diferença de proporção nas pernas**, não da câmera.
- Na corrida, os ombros do rig ficaram de 3 a 4,6 px altos demais.
- O volume externo é viável: 42 a 61 texels novos por pose.

## 17–19. Problemas
- **Encontrados:** ombros altos na corrida; erros de joelho e tornozelo entre 4 e 7 px; a
  manga do idle dentro de `shirt`; de 47 a 53 texels de cabelo classificados como braço no
  #0 e no #2.
- **Corrigidos:** nenhum nesta etapa.
- **Pendentes:** todos foram levados para a 02B.

## 20. Limitações
- Os marcos do idle foram derivados, não marcados à mão.
- A torção do tronco é rígida e aproximada.

## 21–22. Evidências visuais
- `evidence/erro_corretion/etapa02/out/summary.png`
- `evidence/erro_corretion/etapa02/out/fit_*.png`
- `evidence/erro_corretion/etapa02/out/occlusion_*.png`
- `evidence/erro_corretion/etapa02/out/guide_*.png`

## 23. Links
- Relatório original: https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/erro_corretion/etapa02/RELATORIO.md
- Pasta: https://github.com/RaulDuque/ProjetoRST/tree/art/v1.2-protagonist/evidence/erro_corretion/etapa02
- Commit: https://github.com/RaulDuque/ProjetoRST/commit/8b9a2b1

## 24. Resultado
O rig encaixa tronco e ombros e precisa de ajuste localizado.

## 25. Recomendação
Etapa 02B: clavículas, manga do idle e cabelo. Já foi executada.

## 26. Decisões do proprietário
A separação da manga do idle foi decidida na 02B.

## 27. Git
Commit `8b9a2b1`, já publicado.
