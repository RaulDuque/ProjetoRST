# N3 — Auditoria do pipeline

A prova não substitui o protagonista publicado. O jogo continua a ler `assets/animations/player.ron`, gerado por `art-src/player/scripts/post/game_sprites_v2_2.py`. Esse manifesto tem `weapon_in_body: false`, sockets em `assets/animations/player_sockets.ron` e camadas de arma e capa no código (`held_layers_enabled`, `CapeLayer`). A N3 reutiliza os parsers `AnimationManifest` e `SocketTable`. Não cria um formato paralelo.

O que esse pipeline já faz: o manequim `art-src/player/blender/swordsman_rig_v007.blend` é renderizado em ortográfica pelo Blender, a arma fica numa camada, e o Bevy espelha o sprite e o ângulo do socket. O que ele ainda não faz: pixel art acabada. O quadro renderizado nesta etapa é um desenho liso, sem contorno de pixel.

A malha `Md-usario/assets-user/prota/base/white_mesh.glb` continua com 3.387.148 bytes. Não foi copiada. Não tem esqueleto utilizável (a auditoria da N1 mediu vértices, sem ossos, sem UVs e sem animação). Retopologia dela não entrou nesta etapa.

Pixelorama não está no `PATH`. Não foi instalado. LibreSprite, Aseprite, Spine, Mixamo e Hunyuan não foram usados.
