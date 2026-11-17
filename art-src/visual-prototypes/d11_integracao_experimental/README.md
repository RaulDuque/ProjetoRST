# D11 — Integração experimental Bevy 0.20.1

Lab isolado e **reversível**: prova que o protagonista modular (D10) carrega e anima no Bevy sem substituir o visual publicado.

## Como rodar

```bash
# republicar frames D10 → assets/experimental/d11/ (se necessário)
python3 art-src/visual-prototypes/d11_integracao_experimental/scripts/publish_d11_assets.py

cargo run --example modular_visual_lab
```

Screenshots: `evidencias_bevy/`. Regras puras: `src/player/visual/modular_lab.rs` (não entram no `PlayerVisualPlugin`).

## Isolamento

| Caminho | Papel |
|---|---|
| `assets/experimental/d11/` | Frames bakeados experimentais |
| `assets/animations/player.ron` | Visual **default** (intocado) |
| `examples/modular_visual_lab.rs` | App Bevy isolado |
| `cargo run` (jogo) | Continua com o sprite publicado |

## Docs

- `AUDITORIA_D10.md` — ponto de controle 0  
- `DECISAO_ARQUITETURA.md` — estratégia híbrida C  
- `CONTRATO_ATLAS.md` — 128×160, pivot, nearest  
- `REGRESSAO.md` — checklist  
- `RELATORIO.md` — relatório A–S  
