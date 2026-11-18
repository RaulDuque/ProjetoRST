# N3.7 — Sincronização da corrida e pipeline de animação

Experimento isolado. Não é carregado pelo jogo e não está aprovado. A arte aprovada (N3.5 e `run6` da N3.6) é só lida, nunca regravada. Toda variante é um arquivo novo.

| Caminho | Conteúdo |
|---|---|
| `scripts/animation_pipeline.py` | pipeline reutilizável: vídeo + configuração JSON → grades, atlas, manifesto, caixas, proveniência e prévia |
| `configs/run_n36.json` | reconstrói a corrida N3.6 (13/13 grades idênticas) |
| `configs/run_start.json` | partida real do vídeo (quadros 0–13), clipe `run_start` = 3, 5, 7, 9, 11, 13 |
| `pipeline/<nome>/` | `source/` (grades), `generated/` (atlas, `<nome>.ron`, `boxes.ron`, `provenance.json`), `preview/` |
| `scripts/foot_analysis.py` | contato dos pés quadro a quadro → `analysis/foot_metrics.json` |
| `scripts/sync_study.py` | cenários A–E e o teste de fps a 310 u/s (12/15/18/23,7) → `generated/sync.ron` e evidências de velocidade |
| `scripts/contrast_study.py` | variantes B/C/D → `generated/run6_rim.png`, `run6_boots.png`, `backlight.png` |
| `scripts/transition_study.py` | parado → partida → corrida → parado → `generated/transition.ron` |
| `scripts/pipeline_evidence.py` | folha `pipeline_output.png` |
| `cache/` | quadros decodificados e recortes em resolução cheia (ignorado pelo git) |

Ordem de execução (cada script é determinístico):

```bash
python3 -B art-src/n3-7-run-sync/scripts/animation_pipeline.py \
    art-src/n3-7-run-sync/configs/run_n36.json art-src/n3-7-run-sync/configs/run_start.json
python3 -B art-src/n3-7-run-sync/scripts/foot_analysis.py
python3 -B art-src/n3-7-run-sync/scripts/sync_study.py
python3 -B art-src/n3-7-run-sync/scripts/contrast_study.py
python3 -B art-src/n3-7-run-sync/scripts/transition_study.py
python3 -B art-src/n3-7-run-sync/scripts/pipeline_evidence.py
N3_INTERACTIVE=1 cargo run --example n3_visual_proof   # B até "n3.7-sync-proof"; G, J, H, L, K, O, T, V, Espaço, "."
```

Depende só do Python com Pillow e do GStreamer do sistema (`gst-launch-1.0`, `gst-discoverer-1.0`), como na N3.6. Detalhes em `docs/development/n3_7/`.
