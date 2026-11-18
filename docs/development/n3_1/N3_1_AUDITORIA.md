# N3.1 — Auditoria do manequim V2.2

Leitura somente do `art-src/player/blender/swordsman_rig_v007.blend`. O arquivo não foi salvo. A data de modificação continua `2026-11-13 17:35:11 -0300`.

Inventário em `art-src/n3-proof/blender/audit_v22.json`, gerado por `art-src/n3-proof/scripts/audit_v22_rig.py` no Blender 5.2.4 LTS.

- Armadura `rig_v007`: 43 ossos, 36 de deformação.
- 132 malhas. Há um casaco (`coat`), capa (`cape_clasp`, `cape_cord`, `cape_stud` e ossos `cape_*`), mãos, pernas e a espada (`blade`, `sword_collar`, `sword_grip`, `sword_guard`, `sword_pommel`).
- Não há uma segunda malha de armadura. Recolorir o mesmo casaco não cria armadura pesada.
- Materiais nomeados no arquivo incluem pele, calça, colete, sola, ponto e metal escuro. A luz de trabalho é a do arquivo.
- Câmeras ortográficas de lado, frente e costas, além de closes. `cam_three_quarter` é perspectiva e não foi usada.
- O clipe publicado `attack_basic` (módulo `v1_3`, 7 quadros, 24 fps, 0,28 s) não tem antecipação. O quadro 0 já é o golpe. O comentário em `animations.py` diz que a hitbox está ativa desde o quadro 0.
- O ciclo `walk` de 16 quadros está em `animations.py` e não entra nos clipes publicados `v1_3`. O jogo publicado usa `run`, não esse ciclo.
- `jump_rise` e `fall` são os clipes `v1_4`: 4 quadros cada, locomoção, 12 fps.
- A espada fica escondida nos renders desta prova, como no pipeline publicado (`weapon_in_body: false`). Os soquetes são medidos pelo script existente.

A malha branca `Md-usario/assets-user/prota/base/white_mesh.glb` não foi aberta nem usada.
