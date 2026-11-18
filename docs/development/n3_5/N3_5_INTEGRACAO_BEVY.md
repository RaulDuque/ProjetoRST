# N3.5 — Integração isolada no Bevy

## O que mudou

Só `examples/n3_visual_proof.rs`:

- **Fonte 6, `n3.5-fidelity`**, lida de `art-src/n3-5-fidelity/generated/candidate.ron` e `candidate_106.png`, sem capa e sem arma. A tecla **B** percorre sete fontes, então dá para alternar entre a N3.4 (fonte 5) e a N3.5 (fonte 6).
- As fontes 5 e 6 usam o mesmo carregador (`BaseBank` com uma entrada por fonte), sem um sistema paralelo.
- **Enquadramento como enum `View`** no lugar do antigo `close: bool`:
  - `Close`: 360 unidades, densidade 1, 2 px por texel (era `close = true`);
  - `Game`: 405 unidades, densidade 2 (era `close = false`);
  - `GameInteger` (novo): 360 unidades, densidade 2, 1 px por texel.
  - A tecla **V** percorre os três. As capturas antigas continuam com os mesmos enquadramentos.
- Na fonte 6, uma segunda linha do HUD mostra: `art 80x128  figure 106 texels  world 53 units  2 texels/unit  camera 360  1.000 screen px/texel  NOT APPROVED`. Os números saem do manifesto e do `View`, não de texto fixo. Só o tamanho da figura é a constante `BODY_FIGURE_TEXELS`.
- Novas capturas em `docs/development/n3_5/evidence/`: `bevy_n35.png` (Close), `bevy_n35_405.png`, `bevy_n35_360.png`, `bevy_n34_405.png` e `bevy_n34_360.png`.

## O que não mudou

O jogo principal, `src/`, `assets/data/camera.ron`, o save v3, os controles N2, o protagonista publicado e os manifestos de produção.

## Verificação

- O roteiro padrão gravou 32 capturas, com saída 0.
- As capturas publicadas da N3, N3.1, N3.2, N3.3 e N3.4 foram regravadas idênticas: `git status` não mostra nenhuma delas modificada. Por isso a linha extra do HUD só aparece na fonte 6.
- `bevy_n35_360.png` é idêntica ao sprite, pixel a pixel (1 320 texels opacos, 0 diferenças).
