# N3 — Modelo e rig

Modelo usado na prova jogável: nenhum GLB. Os quadros nascem em `art-src/n3-proof/scripts/build_n3_proof.py`. O motivo é o da N1: a malha branca não tem osso, e gastar a etapa inteira em retopologia não responde se a arma troca de camada.

Referência 3D que foi mesmo renderizada: um quadro do manequim já riggado, o V2.2.

```text
blender --background --factory-startup art-src/player/blender/swordsman_rig_v007.blend \
  --python art-src/player/scripts/bpy/build_game_sprites_v2_2.py -- \
  --module v1_3 --clips idle --limit 1 --overwrite
```

Blender 5.2.4 LTS, cerca de 11 s, saiu com código 0. O PNG ficou em `art-src/player/renders/` (ignorado pelo Git). A cópia recortada da evidência é `docs/development/n3/evidence/blender_idle_v22.png`, 310×820. O script abre o `.blend` sem salvar. Não escreveu em `assets/`.

Esse quadro é o idle do protagonista publicado, de perfil, casaco longo, sem pixel. Não é o mesmo gesto da figura da prova. Não houve rig novo. A "pose" da prova é uma tabela de ângulos no script (ombro, cotovelo, quadril, joelho). Não é um armature em runtime, e o jogo não ganhou esqueleto 2D.
