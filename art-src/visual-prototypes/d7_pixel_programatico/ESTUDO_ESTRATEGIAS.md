# Estudo das três estratégias

## A — Primitivas

- **O quê:** polígonos, elipses e pontos Pillow com paleta D2.
- **Prós:** 100% reproduzível; camadas fáceis; zero dependência de render 3D.
- **Contras:** tende ao geométrico (risco D3); falta textura/dither do D2.
- **Veredito esperado:** prova técnica / rejeitado como identidade final.

## B — Sprites + máscaras

- **O quê:** corta `idle_f00` D2 em máscaras semânticas; restack; também empilha
  camadas D4.
- **Prós:** fidelidade máxima à identidade aprovada; peças trocáveis.
- **Contras:** qualidade limitada pela arte-fonte; não “gera” D2 do zero.
- **Honestidade:** documentado em `estrategias/B_sprites_masks/FONTES.md`.
- **Veredito esperado:** aprovável para avaliação / promissor para modularidade.

## C — Híbrido

- **O quê:** silhueta D5/D6 como máscara; preenchimento por bandas de paleta;
  carimbo de cabeça D2; sabre por socket; controle negativo = quantize do toon.
- **Prós:** caminho para animação com dados de pose; separa anatomia de finish.
- **Contras:** bandas planas ainda longe do D2; risco de parecer “recolor”.
- **Veredito esperado:** promissor como arquitetura, incompleto visualmente.
