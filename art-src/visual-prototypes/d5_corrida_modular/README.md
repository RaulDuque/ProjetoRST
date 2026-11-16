# D5 — Corrida modular (pipeline híbrido)

Laboratório em `art-src/` apenas. **Não** altera o jogo.

## Missão

Provar: **um corpo-base + uma animação de corrida compartilhada + equipamentos
independentes**, com look alvo D2 (acabamento ainda parcial).

## Como regenerar

```bash
# 1) Camadas Blender (10 frames × body/coat/cape/pauldron/sword) — pesado
blender --background --factory-startup \
  art-src/visual-prototypes/d5_corrida_modular/blender/swordsman_rig_v007_d5_lab.blend \
  --python art-src/visual-prototypes/d5_corrida_modular/scripts/export_modular_run.py --

# 2) Composição, GIFs, pranchas, testes
python3 art-src/visual-prototypes/d5_corrida_modular/scripts/compose_d5.py
```

## Entregas visuais

| # | Arquivo | Conteúdo |
|---|---|---|
| 01 | `comparacoes/01_identity.png` | D2 vs modular |
| 02 | `comparacoes/02_layers.png` | Camadas separadas |
| 03 | `comparacoes/03_loadouts.png` | A / B / C |
| 04–06 | `previews/run_*.gif` | Corridas por loadout |
| 07 | `previews/run_dynamic_swap.gif` | Troca A→B→C sem reset do ciclo |
| 08 | `comparacoes/08_causeway.png` | Escala V2.3 |
| 09 | `comparacoes/09_acabamento.png` | Raw / toon / outline |
| 10 | `comparacoes/10_problemas.png` | Cape lag + heavy fraco |

## Documentos

`AUDITORIA_D4.md`, `AUDITORIA_BLENDER.md`, `DIRECAO_ARTISTICA.md`,
`ARQUITETURA_MODULAR.md`, `CONTRATO_EXPORTACAO.md`, `COMPARACAO_ACABAMENTO.md`,
`ESTIMATIVA_PRODUCAO.md`, `LIMITACOES.md`, `DECISAO.md`.

## Classificação

**Prova técnica parcial** — modularidade de malha/export/composição demonstrada;
fidelidade artística D2 **não** aprovada (EEVEE toon ≠ pixel D2).
