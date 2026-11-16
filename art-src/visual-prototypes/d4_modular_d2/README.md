# D4 — Arquitetura visual modular (look D2)

Protótipo **somente em `art-src/`**. Não altera o jogo.

## Identidade

- Visual alvo: **D2 detalhado** (não o geométrico D3).
- Objetivo: peças combináveis (corpo, peitoral, capa, chapéu/elmo, óculos,
  arma, escudo) sem redesenhar o personagem inteiro.

## Pastas

| Pasta | Conteúdo |
|---|---|
| `referencia/` | Idle D2 âncora + refs de equip |
| `camadas/` | Peças isoladas + body |
| `combinacoes/` | Idle bare / light / heavy / full |
| `movimento/` | Frames idle+run leve e pesado |
| `previews/` | GIFs motion |
| `comparacoes/` | Boards em escala de gameplay (causeway) |
| `scripts/` | Compositor Python |
| `testes/` | Reserva |

## Documentos

- `AUDITORIA_SLOTS.md` — slots funcionais vs visuais, sockets, Save V3
- `ARQUITETURA_VISUAL.md` — ordem Z, pivôs, oclusão
- `COMPARACAO_PIPELINES.md` — A/B/C com demo
- `LIMITACOES.md` — o que não está pronto
- `DECISAO.md` — recomendação híbrida C

## Como regenerar composites

```bash
python3 scripts/compose_d4.py
```

Requer PNGs em `camadas/` e refs em `referencia/`.
