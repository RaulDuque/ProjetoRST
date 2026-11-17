# Auditoria D11 — causas reais

## Cadeia medida

Arte D10 (128×160, pivot 64,156) → publish `assets/experimental/d11/` →
`examples/modular_visual_lab.rs` → `Sprite.custom_size` → câmera `view_height=405` → janela 1280×720.

## Problemas confirmados

| Problema reportado | Causa-raiz medida |
|---|---|
| Escala inconsistente / personagem “miúdo” | `WORLD_HEIGHT=52` vs locomotion publicado **64** wu (`density:2`, cell 128→64). Tela: ~92 px vs ~114 px. |
| Pés / pivô estranhos | Exemplo D11 **não** usava `Anchor` de sola — âncora default = centro. |
| Personagem escuro demais | `lum_p50≈21` no PNG clássico; ClearColor ~0.08. |
| Espada pouco legível | Lâmina com contraste próximo do casaco; sem realce metálico local. |
| “Pixels irregulares” / alfa | PNG D10/D11: `semi_alpha=0`, `rgb_in_clear=0`. Artefatos percebidos vêm sobretudo de **escala pequena + inspeção** (e compressão de vídeo), não de residual de alfa no arquivo. |
| Equipamentos desproporcionais | Pesado bbox mais largo (83×133 vs 54×110); leitura piorada pela escala D11. |
| After-image fraco | Ghosts no mesmo ponto sem drift suficiente; tint ok em regra. |

## O que D11 acertou

- Bevy carrega frames bakeados.
- Troca classic↔heavy sem reset de `elapsed`.
- Isolamento: default `player.ron` intacto.

## Correção D12 (direção)

1. World height **64** + `Anchor` de pivô.  
2. Pipeline A: contraste B + boost de espada.  
3. Sombra de contato + after-image avaliados no lab Bevy.  
4. Blender = estudo comparativo (não identidade).
