# N3 — Pixel art

Os PNG da prova foram escritos pelo script com Pillow 12.1.1, já instalado na máquina. Cada forma é um disco ou um segmento de discos, numa paleta fechada, com um contorno de um pixel em volta do opaco. Não houve pincel no Pixelorama. Não houve hora de artista.

O corpo em idle mede 97 texels de altura opaca (`metrics.json`, campo `idle_opaque_texels_tall`). Isso cai na faixa de 80–110 texels da N1. Não é o tamanho na janela: a janela mostra essa figura em escala 2×.

`docs/development/n3/evidence/pipeline_compare.png` coloca lado a lado, no mesmo golpe:

- à esquerda, a palavra "filtro": elipses suavizadas e reduzidas com Lanczos. É um filtro. Não é pixel art e não é render do Blender;
- à direita, a palavra "pixels": o quadro autorado.

O ouro da arma não aparece no corpo (0 pixels nas duas folhas; 24 na espada e 6 na lança). O acabamento ainda é geométrico: membros de cápsula, rosto de dois pixels, sem o desenho do casaco do manequim publicado. Serve para testar camada e escala. Não serve como aprovação do estilo.
