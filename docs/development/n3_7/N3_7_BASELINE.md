# N3.7 — Baseline aprovada

A N3.5 (parado) e o `run6` da N3.6 (corrida) são a referência imutável. Nenhum desses arquivos foi regravado. Toda alternativa desta etapa é um arquivo novo em `art-src/n3-7-run-sync/`.

| Arquivo | Papel |
|---|---|
| `art-src/n3-5-fidelity/generated/candidate_106.png` + `candidate.ron` + `source/candidate_106.txt` | parado, 80×128, pivô (40, 118) |
| `art-src/n3-6-animation-proof/generated/run6.png` | corrida aprovada: quadros 48, 50, 52, 54, 56, 58 do vídeo |
| `art-src/n3-6-animation-proof/generated/run.ron` | 12 fps, ciclo de 0,5 s, canvas 112×128, pivô (56, 118), densidade 2 |
| `art-src/n3-6-animation-proof/generated/run10.png`, `stand.png`, `boxes.ron` e `source/video_*.txt` | variante de 10 quadros, quadro de pé e grades |

## Como a preservação é garantida

- `tests/n3_7_sync_tests.rs::the_approved_n35_and_n36_art_is_byte_identical` confere o FNV-1a 64 de 21 arquivos (atlas, manifestos, caixas, grades N3.6 e o candidato N3.5), gravado no início da N3.7. Qualquer byte alterado quebra o teste.
- `the_pipeline_reproduces_the_n36_run_texel_for_texel` reconstrói a corrida pelo pipeline novo e compara as 13 grades: são idênticas.
- `git diff d4e5e04 -- art-src/n3-5-fidelity art-src/n3-6-animation-proof docs/development/n3_5 docs/development/n3_6` está vazio. As evidências anteriores, incluindo as quatro que o proprietário destacou na N3.5, não mudaram, e o roteiro do Bevy regravou as capturas antigas com bytes idênticos.

Evidência: `run_baseline.png` (o parado N3.5 e os seis quadros, com o SHA-256 de cada arquivo) e `run_12fps.gif` (a sequência aprovada, a 12 fps).
