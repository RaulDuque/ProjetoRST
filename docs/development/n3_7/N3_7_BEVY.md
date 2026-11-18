# N3.7 — Prova isolada no Bevy

Evidências: `bevy_n37.png`, `camera_405.png`, `camera_360.png`, `bevy_n37_rim.png`, `bevy_n37_backlight.png` e `bevy_n37_transition.png`.

## O que mudou

Só `examples/n3_visual_proof.rs`, sem uma segunda arquitetura:

- **Fonte 8, `n3.7-sync-proof`.**
  - Lê `art-src/n3-7-run-sync/generated/sync.ron` (cenários, velocidade de referência, fase de cada quadro) e `transition.ron` (tempo parado, quadros de corrida antes da parada, quadro de saída).
  - Lê `pipeline/run_start/generated/run_start.ron` e `boxes.ron` (a partida real) e as variantes `run6_rim.png`, `run6_boots.png` e `backlight.png`.
  - Reaproveita o `run6` aprovado (do `RunBank` da N3.6) e o parado N3.5 (do `BaseBank`).
  - O B leva a ela depois da N3.6.
- **Sistema próprio, `sync_n37`.** Fica na cadeia logo depois de `sync`, que não faz nada quando a fonte é a 8. As outras fontes continuam desenhadas pelo mesmo código de antes.
- **Teclas novas** (só valem na fonte 8):
  - G troca o cenário: A original, B clipe acelerado, C deslocamento reduzido, D moderado;
  - H alterna entre o modo velocidade e o modo transição (parado N3.5 → partida do vídeo → corrida → corte para o parado, em loop);
  - L troca os quadros: original, borda de luz ou botas +1 tom;
  - K troca o fundo: padrão, claro, cinza, quase preto, ou quase preto com contraluz.
- **Teclas que já existiam:**
  - O liga e desliga a sobreposição (caixa, pivô e chão; no estágio parado, só pivô e chão);
  - T alterna entre correr no lugar e atravessar a vista;
  - V troca a câmera;
  - Espaço pausa;
  - `.` avança um quadro, inclusive na troca de estágio da transição.
- **Régua do chão:** marcas a cada 20 unidades de jogo, fixas no mundo. Com T ligado, o corpo anda na velocidade do cenário (em unidades de jogo, desenhadas a 2 texels por unidade em todas as vistas), e dá para ver o pé andar sobre a régua.
- **HUD:**
  - fonte, cenário, modo, estágio, clipe, quadro i/n, vista e pausa;
  - fps efetivo, fator, velocidade (e a do jogo, 310), posição, fase do pé e deslizamento estimado. No lugar, mostra quanto o pé apoiado recua;
  - quadros, fundo, câmera, px de tela por texel, sobreposição, no lugar ou em movimento, e `NOT APPROVED`.
- **Fundos:** a fonte 8 troca o `ClearColor`. Ao sair dela, a cor padrão volta. A contraluz é um sprite atrás do personagem, com o mesmo pivô.
- **Capturas do roteiro** (pastas `docs/development/n3_7/evidence/`):

  | Arquivo | Conteúdo |
  |---|---|
  | `bevy_n37.png` | cenário D, `run6` #2, vista fechada, sobreposição |
  | `camera_405.png` | cenário A, vista do jogo 405 |
  | `camera_360.png` | cenário A, vista 360 inteira |
  | `bevy_n37_rim.png` | borda de luz no quase preto, 360 |
  | `bevy_n37_backlight.png` | contraluz no quase preto, 360 |
  | `bevy_n37_transition.png` | modo transição, partida #2 |

## Verificação

- O roteiro padrão gerou 47 capturas (41 antigas + 6 novas), com saída 0.
- As capturas da N3 à N3.6 saíram com bytes idênticos: `git status` não mostra nenhuma evidência antiga modificada.
- **Câmera 360:** a captura do `run6` #2 é idêntica ao sprite (0 diferenças em 2458 texels).
- `cargo clippy --all-targets -- -D warnings` limpo.
- O movimento (A × B × C × D andando sobre a régua, a transição em loop) só pode ser visto no modo interativo, `N3_INTERACTIVE=1 cargo run --example n3_visual_proof`. Os GIFs mostram as mesmas sequências fora do Bevy.

## Limites

- A contraluz é a placa de 112×128 do canvas, então a borda dela aparece. Uma luz de cena real seria maior e suave.
- As pegadas da evidência GIF não existem no Bevy. Lá, o deslizamento aparece no HUD e na régua.

## O que não mudou

O jogo principal, `src/`, `assets/` (incluindo `camera.ron`), o save, os controles, a física, o combate, a magia, o protagonista publicado e os manifestos de produção.
