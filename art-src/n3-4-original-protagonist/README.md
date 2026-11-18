# N3.4 — Protagonista-base original

Experimento isolado. Não é carregado pelo jogo e não está aprovado.

| Caminho | Conteúdo |
|---|---|
| `source/base_idle_draft.txt` | redução automática da vista lateral da ficha modular (rascunho) |
| `source/base_idle.txt` | **fonte do sprite**: o rascunho corrigido pixel a pixel à mão |
| `scripts/n34_palette.py` | as 19 cores (um caractere por cor, `.` transparente) |
| `scripts/reduce_reference.py` | gera o rascunho; lê `Md-usario/` só para leitura |
| `scripts/build_n3_4.py` | converte a fonte em PNG e monta as evidências em `docs/development/n3_4/evidence/` |
| `generated/base_idle.png` | sprite 80×128, pivô `(40, 118)` |
| `generated/base.ron` | manifesto no formato `AnimationManifest` |
| `generated/metrics.json` | medidas e contagem de pixels editados |

```bash
python3 art-src/n3-4-original-protagonist/scripts/build_n3_4.py
cargo run --example n3_visual_proof        # fonte "n3.4-base" (tecla B no modo interativo)
```

Não rode `reduce_reference.py` sem necessidade: ele regrava o rascunho, não a fonte. Detalhes em `docs/development/n3_4/`.
