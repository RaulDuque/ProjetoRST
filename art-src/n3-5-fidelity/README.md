# N3.5 — Prova de fidelidade e resolução

Experimento isolado. Não é carregado pelo jogo e não está aprovado.

| Caminho | Conteúdo |
|---|---|
| `scripts/reduce_fidelity.py` | redução da vista lateral da ficha (máscara erodida, BOX, paleta N3.4) em 106, 132 e 94 px, mais duas variantes de paleta |
| `scripts/build_n3_5.py` | candidato, manifesto, métricas e evidências em `docs/development/n3_5/evidence/` |
| `source/reduction_106.txt` | redução de 106 px, sem edições |
| `source/candidate_106.txt` | **fonte do candidato**: a redução com 10 edições à mão (listadas em `generated/metrics.json`) |
| `generated/candidate_106.png` | candidato 80×128, pivô `(40, 118)` |
| `generated/candidate.ron` | manifesto no formato `AnimationManifest`, `density: 2` |
| `generated/reduction_{106,132,94}.png` | as três hipóteses de resolução |
| `generated/variant_*.png` | paletas rejeitadas, para comparação |

As cores ficam em `art-src/n3-4-original-protagonist/scripts/n34_palette.py`.

```bash
python3 art-src/n3-5-fidelity/scripts/reduce_fidelity.py
python3 art-src/n3-5-fidelity/scripts/build_n3_5.py
cargo run --example n3_visual_proof        # fonte "n3.5-fidelity"; B troca a fonte, V a câmera
```

Detalhes em `docs/development/n3_5/`.
