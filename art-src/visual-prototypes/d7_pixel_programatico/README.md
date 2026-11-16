# D7 — Pixel art programática (laboratório)

Investigação: produzir aparência próxima do protagonista **D2** por código 2D
(Pillow), com equipamentos independentes, **sem** usar o acabamento EEVEE/toon
das fases D5/D6 como look final.

## Como regenerar

```bash
python3 art-src/visual-prototypes/d7_pixel_programatico/scripts/run_d7.py
```

## Candidatos Marco A

| ID | Pasta | Método |
|----|-------|--------|
| D7-A | `estrategias/A_primitivas/` | Primitivas Pillow puras |
| D7-B | `estrategias/B_sprites_masks/` | Máscaras + restack de arte D2/D4 |
| D7-C | `estrategias/C_hibrido/` | Máscara de silhueta D5/D6 + pintura D2 |

**Selecionado para Marco B:** D7-B (fidelidade visual via arte-fonte D2).

## Avaliação rápida

1. `comparacoes/02_D7_tres_estrategias.png`
2. `comparacoes/03_D7_identidade_escala_real.png`
3. `comparacoes/10_D7_resultado_final.png`
4. `DECISAO.md` e `COMPARACAO_VISUAL.md`

## Fora de escopo

Sem alterações em `src/`, `assets/`, saves, ou publicação no jogo. Sem commit/push
nesta fase.
