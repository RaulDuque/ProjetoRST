# Contrato de sockets 2D (laboratório)

- Arquivo: `sockets/sockets_lab.json` (+ `idle_sockets.json`, `run_sockets.json`)
- Espaço: pixels da célula, origem top-left, Y para baixo
- Pivô sola: (64, 156)
- Derivados de `assets/animations/player_sockets.ron` (**somente leitura**)
  + estimativas anatômicas no bbox D2
- Clips: idle 6 frames, run 8 frames
- Campos: name, x, y, space, mirror, attaches[, angle_deg para weapon]

**Não altera** contratos do jogo nem Save V3.
