# Decisão de arquitetura — D11 integração experimental

## Estratégia escolhida: **C (híbrida)**

| Parte | Escolha | Motivo |
|---|---|---|
| Frames idle/run classic/heavy | **Bake (B)** — PNGs pré-compostos do lab D10 em `assets/experimental/d11/` | Prova Bevy com arte já validada; evita compositor runtime pesado no primeiro passo |
| Troca de loadout / clip / facing | **Runtime (A)** — índice de frame e handles trocados em sistemas | Troca sem reiniciar animação; flip independente |
| After-image | **B (trail)** — fantasmas tintados com fade | Já escolhido e gerado no D10; portado para Bevy no lab |

## Isolamento (reversível)

1. Assets em `assets/experimental/d11/` — **não** substituem `assets/animations/player.ron`.
2. Código puro em `src/player/visual/modular_lab/` — regras testáveis, sem plugin no jogo default.
3. Exemplo `examples/modular_visual_lab.rs` — app Bevy isolado; `cargo run` do jogo **não** ativa o visual modular.
4. Nenhuma mudança em Save V3, câmera, parallax, combate ou slots funcionais.

## Contrato de atlas (resumo)

- Célula: 128×160 px; pivot sola: (64, 156); altura mundo alvo: 52 px.
- Filtro: nearest **por imagem** (não `ImagePlugin::default_nearest` global).
- Loadouts experimentais: `classico`, `pesado`.
- Clips: idle 6 @ 10 fps; run 8 @ 12 fps.

Detalhe: `assets/experimental/d11/contract.json`.

## O que NÃO fazer neste passo

- Não marcar como visual final aprovado.
- Não trocar o default do `PlayerVisualPlugin`.
- Não publicar este experimento como asset de produção.
