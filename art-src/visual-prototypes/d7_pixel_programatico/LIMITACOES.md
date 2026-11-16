# Limitações D7

1. **A** não alcança a densidade de shading do D2 só com primitivas.
2. **B** depende de arte-fonte D2/D4; o código não inventa identidade nova.
3. **C** ainda mostra bandas de cor; o controle `quantize_d6` prova que só
   recolorir toon **não** resolve.
4. Equipamentos D4 não têm todas as peças pedidas (luvas/botas separadas):
   swaps usam proxies documentados.
5. Animação piloto empilha extras **estáticos** sobre frames D2 — capa/armadura
   não deformam com o passo.
6. Run D2 tem **8** frames (não 10).
7. Sem publicação em `assets/` / Bevy.
8. Testes automáticos não julgam qualidade artística.
