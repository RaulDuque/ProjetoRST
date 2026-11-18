# N3.6 — Prova de animação (corrida)

Experimento isolado. Não é carregado pelo jogo e não está aprovado.

| Caminho | Conteúdo |
|---|---|
| `scripts/segment_sheets.py` | numera as poses das folhas do proprietário → `analysis/poses.json` |
| `scripts/extract_video.py` | decodifica o MP4 (GStreamer do sistema) em `cache/` (ignorado) e recorta com chroma key os quadros escolhidos → `frames/video_NNN.png` |
| `scripts/build_run.py` | redução N3.5 em escala única, grades, atlas, manifesto, caixas, métricas, GIFs e folhas da corrida |
| `scripts/analyze_references.py` | folhas de análise das referências (visão geral, identidade, corrida, salto, ataque, idle) |
| `frames/` | intermediários em resolução cheia (quadro 0 e quadros 48–59 do vídeo) |
| `source/video_NNN.txt` | grade 112×128 de cada quadro (chaves da paleta N3.4), a fonte editável |
| `generated/run.ron` | manifesto: atlas `stand`, `run6`, `run10`; clipes `idle`, `run6` (12 fps), `run10` (tempo do vídeo) |
| `generated/boxes.ron` | caixa opaca de cada quadro, para a sobreposição do exemplo |
| `generated/metrics.json` | escala, deslocamento, caixas, durações, velocidade estimada |

```bash
python3 art-src/n3-6-animation-proof/scripts/segment_sheets.py
python3 art-src/n3-6-animation-proof/scripts/extract_video.py
python3 art-src/n3-6-animation-proof/scripts/build_run.py
python3 art-src/n3-6-animation-proof/scripts/analyze_references.py
N3_INTERACTIVE=1 cargo run --example n3_visual_proof   # B até "n3.6-run-proof"; R, F, O, T, Espaço, "."
```

`build_run.py` reescreve as grades a partir dos quadros. Se uma grade for editada à mão, não rode o build sem antes salvar a edição. Detalhes em `docs/development/n3_6/`.
