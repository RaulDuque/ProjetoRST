# N3.6 — Prova isolada no Bevy

Evidências: `bevy_n36.png` (visão fechada com sobreposição), `bevy_n36_405.png`, `bevy_n36_360.png`, `run_405.png`, `run_360.png` e `run_bevy_frames.png` (os 6 quadros do `run6` capturados um a um).

## O que mudou

Só `examples/n3_visual_proof.rs`, sem uma segunda arquitetura:

- **Fonte 7, `n3.6-run-proof`.** Lê `art-src/n3-6-animation-proof/generated/run.ron` com `AnimationManifest` (atlas `stand`, `run6` e `run10`; clipes `idle`, `run6` e `run10`), uma textura e um layout de grade por atlas, e `boxes.ron` com as caixas por quadro. O B leva a ela, e o N3.5 continua na fonte 6.
- **Teclas novas** (só valem na fonte 7):
  - R troca o clipe entre `run6`, `run10` e o quadro de pé;
  - F troca a temporização entre a do manifesto, 8, 10 e 12 fps;
  - O liga e desliga a sobreposição;
  - T alterna entre correr no lugar e atravessar a vista a 78 unidades/s.
- **Teclas que já existiam:** Espaço pausa, `.` avança um quadro com a animação pausada e V troca a câmera (fechada 360 / jogo 405 / jogo 360 inteira).
- **Temporização por quadro:** com a temporização do manifesto, cada quadro dura o que diz `durations` (o `run10` alterna 2/24 e 1/24 s). As outras fontes continuam como antes.
- **Sobreposição:** caixa opaca do quadro (azul), cruz do pivô e linha do chão (vermelhas). O projeto compila o Bevy só com as features `2d`, `ui` e `audio`, sem o renderizador de gizmos. Por isso a sobreposição é feita com sprites finos (`OverlayBar`). Nenhuma feature ou dependência foi adicionada.
- **HUD:** fonte, clipe, quadro i/n, arte 112×128, figura de 106 texels, texels por unidade, câmera, px de tela por texel, temporização (duração do ciclo ou fps forçado), no lugar ou em movimento, sobreposição e `NOT APPROVED`.
- **Capturas do roteiro:** `bevy_n36*.png` (quadro 2, pausado) e os 6 quadros do `run6` na câmera 360, gravados em `target/n3_6_bevy_frames/` e montados em `run_bevy_frames.png`. O `capture` agora cria a pasta de destino se ela não existir.

## Verificação

- O roteiro padrão gerou 41 capturas, com saída 0.
- As capturas publicadas da N3 à N3.5 saíram idênticas: `git status` só mostra o exemplo modificado.
- **Câmera 360:** cada um dos 6 quadros capturados é idêntico ao sprite (0 diferenças).
- **Câmera 405:** o quadro 2 cai de 85×101 para 76×89 px de tela.
- A reprodução contínua (Espaço) e o movimento (T) só podem ser vistos no modo interativo, `N3_INTERACTIVE=1 cargo run --example n3_visual_proof`. O roteiro captura quadros pausados, e o GIF mostra a sequência real fora do Bevy.

## O que não mudou

O jogo principal, `src/`, `assets/` (incluindo `camera.ron`), o save, os controles, o combate, a magia e o protagonista publicado.
