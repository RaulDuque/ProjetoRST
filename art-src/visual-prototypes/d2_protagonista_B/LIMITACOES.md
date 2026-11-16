# Limitações — Fase 2

1. **Não é atlas publicável.** Idle (6), run (8) e attack (7) são pilotos de
   direção. A identidade entre quadros ainda deriva (conceito gerado +
   quantização), sem a estabilidade de pixel art humana.

2. **Não há spritesheet “pronta para o jogo”.** As PNG finais têm alpha
   verdadeiro (matte magenta removido por chroma+flood) e pivô aproximado
   nos pés, mas **não** passam pelo contrato de células/pivôs de
   `assets/animations/player.ron`.

3. **Contagem ≠ clips oficiais.** Idle oficial = 12 @ 12 fps; run = 10 @ 24 fps;
   attack = 7 FitToAction. O piloto aproxima o ataque (7 fases alinhadas a
   0,28 s / 60 % ativo). Idle/run têm menos quadros que o publicado.

4. **Sockets não remedidos.** Equipamentos e capa D2 são estudos de camada;
   `player_sockets.ron` permanece intacto e não foi gerado a partir destes
   frames.

5. **Jump/fall** da referência A não foram reanimados nesta fase (só idle,
   run, attack, como pedido nos três pilotos).

6. **Forma espectral e equipamentos** = estudos estáticos, sem clips nem
   integração.

7. **Visualizador isolado no Bevy** não foi criado (evitaria risco de tocar
   no app). As comparações usam composição sobre captura real V2.3.

8. **Qualidade de conceito:** downscale nearest-neighbor a partir de JPG
   comprime bordas; a limpeza de magenta é automática e pode comer 1 px de
   contorno em alguns frames.
