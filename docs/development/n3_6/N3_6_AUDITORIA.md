# N3.6 — Auditoria inicial

## Estado do Git

- Branch `art/v1.2-protagonist`. HEAD local e remoto: `1d628f28caafa19d0ab97d58cf246f77fff8eaeb`, o mesmo informado no briefing.
- O commit `1d628f2` ("im") é do proprietário e vem depois do último da N3.5 (`6c0cc29`). Ele só adiciona sete arquivos em `Md-usario/assets-user/prota/base/`: as quatro folhas `Gemini_Generated_Image_*.jpg`, `al.jpg`, `side.png` e o vídeo `trimmed_fd4c76cf-…mp4`. Não altera código nem documentos.
- Ficaram não rastreados, intocados: os briefings N3.4, N3.5, N3.6, N4 e N5 e `Ficha_de_Modelagem_do_Protagonista_Gotico.png`.

## Documentos lidos

`CLAUDE.md`, `docs/ARCHITECTURE.md`, `docs/DECISIONS.md` (até D-043), os relatórios da N3 à N3.5, `src/player/visual/manifest.rs` (regras do manifesto: o clipe `idle` é obrigatório, as identificações em snake_case, uma duração por quadro, densidade inteira) e `examples/n3_visual_proof.rs`.

## Arquivos que não podem mudar

`Md-usario/`, `assets/` (incluindo `assets/data/camera.ron` e os manifestos de produção em `assets/animations/`), `src/` (combate, magia, save v3, controles N2, protagonista publicado), os `.blend` do Blender V2.2 e as pastas `art-src/` e `docs/development/` da N3 à N3.5. Nenhum deles foi alterado nesta etapa (`git status` e `git diff --stat 1d628f2 -- src assets Md-usario` vazios no fim).

## Evidências da N3.5 preservadas

`dark_background.png`, `bevy_n35_360.png`, `final_candidate.png` e `palette_test.png` de `docs/development/n3_5/evidence/` não foram regravados. Todas as capturas antigas do exemplo saíram idênticas byte a byte.

## Ferramentas

| Ferramenta | Situação | Uso |
|---|---|---|
| Python 3 + Pillow 12.1.1 | disponível; `resize(NEAREST)` quebrado no sistema, com o contorno de `resample.py` | recorte, chroma key, redução, paleta, folhas, GIF |
| GStreamer 1.x (`gst-launch-1.0`, `gst-discoverer-1.0`) | disponível no sistema | decodificar o MP4 |
| ffmpeg, OpenCV, PyAV, imageio, NumPy | **ausentes** | — |
| Pixelorama, editores gráficos | o agente não opera interface gráfica | — |
| Gerador de imagens | **não usado** | — |

Nada foi instalado e nenhum arquivo foi enviado para fora.
