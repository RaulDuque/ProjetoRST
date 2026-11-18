# N3.7 — Pipeline reutilizável de animação

`art-src/n3-7-run-sync/scripts/animation_pipeline.py`: um script, uma configuração JSON por vídeo. Ele é o método da N3.6 sem mudanças, tirado dos dois scripts avulsos e posto atrás de uma configuração. Evidência: `pipeline_output.png`.

## Etapas

1. **Metadados:** `gst-discoverer-1.0 -v` fornece largura, altura, fps, duração e codec.
2. **Hash:** SHA-256 do vídeo. Se a configuração traz `video_sha256` e ele não bate, o pipeline para.
3. **Extração:** `gst-launch-1.0 … decodebin ! videoconvert ! pngenc ! multifilesink` decodifica todos os quadros uma vez, em PNG sem perda, em `cache/<sha12>/` (ignorado pelo git). O vídeo original é só lido.
4. **Remoção do fundo:** a chave verde é fundo quando g > r + margem e g > b + margem. A máscara é erodida em `erode` px e o excesso de verde é limitado a max(r, b). A mesma janela de recorte vale para todos os quadros. Os recortes ficam em `cache/<sha12>/keyed_<nome>/`.
5. **Escala única:** `fit_frame` + `figure_texels` (o quadro de referência vira N texels), ou `fixed`.
6. **Redução:**
   - filtro BOX do Pillow;
   - alfa ≥ 128 vira opaco, o resto transparente, então a transparência é binária e não sobra borda verde;
   - cor mais próxima da paleta N3.4, sem as chaves de acabamento manual `M` e `e`.
7. **Pivô lógico:** um único deslocamento global, nunca por quadro. `hip_frame` + `ground_frames` (quadril na coluna do pivô, sola mais baixa na linha do pivô) ou `fixed`. O balanço natural fica.
8. **Saídas** em `pipeline/<nome>/`:
   - `source/video_NNN.txt`: grades de texto editáveis;
   - `generated/<clipe>.png`: um atlas por clipe;
   - `generated/<nome>.ron`: `AnimationManifest` válido;
   - `generated/boxes.ron`: caixas opacas;
   - `generated/provenance.json`: proveniência, métricas e durações (`N3_7_PROVENIENCIA.md`).
9. **Validação:** o tamanho de cada atlas deve bater com o manifesto, e nenhum quadro de clipe pode passar da linha do chão. O pipeline para se algo falhar.
10. **Prova visual:** `preview/frames.png` (cada quadro a 2× com caixa e pivô) e um GIF por clipe.

## Configuração (exemplo `configs/run_start.json`)

```json
{
 "name": "run_start",
 "video": "Md-usario/assets-user/prota/base/trimmed_fd4c76cf-….mp4",
 "video_sha256": "29cdbcb8…",
 "crop": [0, 150, 496, 702],
 "key": {"margin": 30, "erode": 1},
 "frames": [0, 1, 2, …, 13],
 "excluded": {"1": "still", "2": "still"},
 "scale": {"fit_frame": 0, "figure_texels": 106},
 "offset": {"fixed": [-3, -5], "why": "the N3.6 run offset"},
 "canvas": [112, 128], "pivot": [56, 118], "density": 2,
 "clips": {
  "idle": {"frames": [0], "durations": [1.0], "fps": 1.0},
  "run_start": {"frames": [3, 5, 7, 9, 11, 13], "fps": 12.0, "loop": false}
 }
}
```

Um clipe pode usar `fps` (durações iguais), `durations` explícitas ou `video_timing` + `cycle_end` (cada quadro dura até o próximo mantido, como o `run10`).

## Verificação

- **`run_n36.json`** reconstrói a corrida aprovada: **13/13 grades idênticas texel a texel**, e atlas `run6`, `run10` e parado idênticos pixel a pixel. Esse é o teste de que o pipeline reproduz a aprovação (`tests/n3_7_sync_tests.rs`).
- **`run_start.json`** produziu um material novo e real, a partida (`N3_7_TRANSICAO.md`), com a mesma escala (0,236607) e o mesmo deslocamento (−3, −5) da corrida.
- O pipeline todo roda em cerca de 5 s por configuração nesta máquina.

## Para um vídeo novo (salto, ataque, idle)

1. Copiar uma configuração, trocar `video`, `video_sha256`, `crop` e `frames`.
2. Se o vídeo tiver um quadro parado de corpo inteiro, usar `fit_frame` nele com 106 texels, para manter a escala. Se não tiver, fixar a escala da corrida (`fixed: 0.236607`), desde que o personagem tenha o mesmo tamanho no vídeo novo.
3. Escolher o canvas e o pivô. O salto e o ataque provavelmente precisam de um canvas mais alto ou mais largo, com o mesmo pivô lógico.
4. Rodar, abrir `preview/frames.png`, marcar os quadros com fantasma em `excluded` e montar os clipes.

## Limites

- A chave é verde-fixa: um vídeo com fundo de outra cor precisa de outra regra.
- A remoção pode deixar 1 px de cor de fundo misturada em cabelo muito fino. A erosão de 1 px tira a maior parte.
- Não há rastreamento de pés nem de soquetes. Isso está em `foot_analysis.py`, à parte.
- Não há interpolação e não deve haver: quadros faltando significam material faltando.
