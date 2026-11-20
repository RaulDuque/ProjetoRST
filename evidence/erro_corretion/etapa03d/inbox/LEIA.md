# Caixa de entrada da Etapa 03D

Coloque aqui a imagem gerada no ChatGPT, com este nome exato:

    evidence/erro_corretion/etapa03d/inbox/armour_master.png

Como pedir no ChatGPT:

1. Anexe `evidence/erro_corretion/etapa03d/request_pose/reference.png`. É o protagonista
   N3.5 aprovado, ampliado 9x sobre verde #00FF00 (1024x1024).
2. Cole como pedido o texto de `evidence/erro_corretion/etapa03d/request_pose/prompt.txt`.
3. Baixe o resultado como PNG, sem cortar, redimensionar nem editar (o ideal é 1024x1024
   com fundo verde liso). JPG e WebP também são aceitos, mas o PNG preserva melhor a grade.

Se vierem várias versões, salve-as como `armour_master_02.png`, `armour_master_03.png` e
assim por diante. Cada uma vira um teste separado.

Processamento (eu executo): `scripts/import_external.py --image inbox/<arquivo> --run takeNN`.
