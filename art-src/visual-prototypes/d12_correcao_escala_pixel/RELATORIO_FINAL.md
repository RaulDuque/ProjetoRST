# Relatório final D12

**Sem commit. Sem push. Visual padrão intacto.**

## A. Estado inicial do Git

Branch `art/v1.2-protagonist`, HEAD `1115f0d`. Working tree já tinha D9–D11 untracked.

## B. Problemas D11

Escala baixa (52 wu), âncora central, luminância baixa, espada pouco separada, avaliação difícil na escala real. Ver `AUDITORIA_D11.md`.

## C. Causa-raiz escala

World height 52 ≠ locomotion publicado 64; ausência de `Anchor` de pivô.

## D. Causa-raiz “pixels”

PNG limpo (alfa binário). Percepção ≠ residual de chroma.

## E–J. Correções

Proporção 64 wu; sockets/pivô via anchor; silhueta/contraste A/B/C; espada boost; rim suave; alfa documentado sem threshold cego.

## K. Blender

Rim A/B/C em `blender/rim_*.png` (5.2.4 LTS). Não escolhido como look final.

## L. Pipelines

**A recomendado**; B descartado como identidade; C opcional.

## M–O. Sombra, after-image, idle/run

Implementados no lab Bevy + GIFs em `previews/`.

## P. Testes

`test_d10`, `test_d12`, `cargo test --lib modular_lab` (7), fmt/clippy/check OK.

## Q. Evidências Bevy

`evidencias_bevy/01`…`07` (idle/run/heavy/flip/sombra/ar).

## R. Limitações

Ver `LIMITACOES.md`.

## S. Estado final Git

HEAD ainda `1115f0d`; lab D12 + `modular_lab`/`example`/assets experimentais untracked/modificados. **Sem commit.**

## T. D13

1. Aprovação visual do proprietário (contraste B vs C).  
2. Pintura manual pontual (rosto/espada) se B não bastar.  
3. Bake modular sem guia D2.  
4. Só então flag off-by-default no jogo.
