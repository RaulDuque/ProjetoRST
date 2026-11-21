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

## Take 02

- Imagem esperada: `evidence/erro_corretion/etapa03d/inbox/armour_master_02.png`.
- O que enviar ao ChatGPT: a mesma `request_pose/reference.png` (idêntica à do take01) e o
  texto de `request_pose/take02_prompt.txt` (gerado por `ask-pose` em `request_pose_take02/`).
- Processamento (eu executo), da raiz do projeto, uma linha por vez:

      python3 -I -B evidence/erro_corretion/etapa03d/scripts/import_external.py --image evidence/erro_corretion/etapa03d/inbox/armour_master_02.png --run take02 --request evidence/erro_corretion/etapa03d/request_pose_take02
      python3 -I -B evidence/erro_corretion/etapa03d/scripts/take_report.py --run take02
      python3 -I -B evidence/erro_corretion/etapa03d/scripts/restore_hair.py --run take02
      python3 -I -B evidence/erro_corretion/etapa03d/scripts/compare_takes.py --takes take01 take02 --out take02

## Take 03

- **Imagem esperada:** `evidence/erro_corretion/etapa03d/inbox/armour_master_03.png`.
- **O que enviar ao ChatGPT:**
  - a imagem `request_pose_take03/reference.png`, idêntica à dos takes 01 e 02;
  - o texto de `request_pose_take03/prompt.txt`, gerado por `ask-pose`.
- **Passo a passo completo:** `prompts/PROMPT_03D_take03.md`, que inclui o texto a colar no
  Claude Code.
- **Observação:** o take02 usou por engano a mesma imagem do take01 e foi rejeitado. Por
  isso o arquivo agora tem outro nome: `armour_master_03.png`.
