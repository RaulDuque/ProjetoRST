# Plano de integração Bevy 0.20.1 (somente documento)

## Estado atual (lido do repo)

- Dependência: `bevy = "=0.20.1"` em `Cargo.toml`.
- Visual do jogador: `src/player/visual/` — `PlayerSpriteSet`, atlas via
  `assets/animations/player.ron`, `Sprite` + `TextureAtlas`.
- VFX existentes: `src/player/visual/vfx/`.
- Sockets: `assets/animations/player_sockets.ron` (não alterar na D10).
- Câmera / causeway V2.3: **não mudar** altura de visão nem parallax.

## Proposta futura (D11+)

1. Publicar atlas D10 (camadas) sob `assets/` com guard de publish existente.
2. Entidade jogador com filhos por slot visual (capa, arma, elmo…) OU atlas
   composto offline por frame (mais barato no GPU).
3. After-image: spawn de 2–3 `Sprite`s tintados com fade; reutilizar composição
   já montada do frame atual (não só o corpo).
4. Filtragem: manter nearest-neighbor para pixel art.
5. Sem bloom, sem crates novas, sem tilemap migration nesta linha.

## Fora de escopo agora

Qualquer edição em `src/`, `assets/`, Save V3, HUD.
