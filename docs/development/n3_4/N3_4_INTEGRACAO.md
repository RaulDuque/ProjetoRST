# N3.4 — Integração isolada no Bevy

## O que mudou

Só `examples/n3_visual_proof.rs`:

- Nova fonte 5, `BASE_SOURCE`, que lê `art-src/n3-4-original-protagonist/generated/base.ron` (o mesmo formato de `AnimationManifest` da N3.3) e `base_idle.png`.
- A tecla **B** passa a percorrer seis fontes quando o sprite existe. O texto da tela diz `n3.4-base`, `weapon none (base body)` e `cape none (base body)`.
- Na fonte 5, a capa fica escondida e a arma tem tamanho zero. A base é mostrada sem equipamento.
- Duas capturas novas no roteiro padrão: `bevy_n34.png` (visão fechada 360, 2× inteiro) e `bevy_n34_game.png` (enquadramento 405, desenhado em 0,5×, como as fontes anteriores).
- O exemplo espera a textura carregar e sai com erro se ela falhar, como nas outras fontes.

## O que não mudou

O jogo principal, `src/`, `camera.ron`, o save v3, os controles N2, o protagonista publicado e os manifestos de produção.

## Verificação

- `cargo run --example n3_visual_proof` gerou as duas capturas. O personagem aparece em pé, voltado para a direita, com o pivô na sola, sem capa e sem espada.
- As capturas anteriores (N3, N3.1, N3.2 e N3.3) foram regravadas pelo mesmo roteiro e saíram idênticas byte a byte: `git status` não mostrou nenhuma alteração nelas.
- O fundo da janela é escuro. Os fundos neutro e claro estão nas folhas geradas em Python (`native_scale.png`, `hair_study.png`).

## Escala

Na visão fechada, 1 texel equivale a 2 px de tela. No enquadramento 405, o sprite é reduzido. Ver `N3_4_RESOLUCAO.md`.
