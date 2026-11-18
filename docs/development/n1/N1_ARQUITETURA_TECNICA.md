# N1 — Arquitetura técnica

Bevy 0.20.1 permanece. A proposta estende os módulos que `docs/ARCHITECTURE.md` já separa em regra pura e sistema. Não há reescrita.

## Render

Sprites em camada já existem: corpo, capa, arma, escudo, VFX, cenário em parallax. Atlas é `TextureAtlasLayout` a partir do manifesto. Filtro nearest existe para a arte gerada. A câmera é regra pura em `camera_rules` e sistema em `camera.rs`.

O que a fundação acrescenta, depois da prova:

- um buffer interno 640×360, só no experimento, com escala inteira (`N1_PIXEL_ART_32_64_BITS.md`);
- `draw_order` por clipe, quando a arma cruzar o torso;
- snap do resultado da câmera no texel, no mesmo experimento, não no jogo atual.

Rotação de corpo fica de fora. Luz e texto ficam por cima do buffer.

## Equipamento

O dado visual opcional descrito em `N1_EQUIPAMENTOS_VISUAIS.md` mora no item que o catálogo já carrega. A troca em runtime continua sendo o sistema que lê o slot e atualiza a camada. Família e âncora já têm tipo (`WeaponForm`, sockets).

## Mundo

Sala de vários tamanhos já é `map.w` e `map.h` vezes a célula de 1280×720. Conexão é porta recíproca. Progresso é `WorldProgress`. Carregar a sala na transição basta para as 29 salas e para a região da N7. Streaming por região não tem medida que o justifique.

Requisito de progressão já é `Requirement`: ability, item, flag, all, any. Relíquia nova é um variante de `Ability`, não uma string.

## Combate

Hitbox, cancelamento e custo seguem a ação. O quadro pode emitir um marcador; se não emitir, o golpe sai no tempo da ação. Projétil e número de dano já são sistemas à parte. A arte nova não ganha um combat system paralelo.

## Persistência

Save v3 guarda abilities, flags, baús, segredos, `defeated_bosses`, equipamento por id e altares. Configuração de tecla fica num arquivo local, não neste save (`N1_CONTROLES.md`). Bump de versão só se um campo obrigatório entrar no personagem. Campo visual com default não bumpa.

## Desempenho

Meta, sem otimizar agora:

- Não carregar o produto das combinações. O residente do jogador é o atlas do corpo, uma folha por família de arma em uso, a capa e o VFX.
- Medir, na prova, o tamanho em bytes dessas texturas e o número de sprites do jogador (corpo + capas + armas). O teto da prova é quatro camadas e menos de 32 MB de textura de personagem.
- Sala grande: a câmera já recorta a vista. O teste da N7 é uma sala de duas células com os inimigos que a sala já spawnaria, e olhar se o frame segura. Se segurar, não há streaming.

## Alternativas

Um crate de animação 2D por osso. Sem evidência de que funciona no Bevy 0.20.1 deste repositório, não entra no `Cargo.toml`.

## Riscos

Dois caminhos de arte ao mesmo tempo: a silhueta em código e a folha nova. A chave `held_layers_enabled` / `weapon_in_body` já existe para não desenhar a arma duas vezes. A prova usa essa chave.

## Recomendação

Cada etapa N2–N12 mexe num módulo da tabela de arquitetura e deixa os outros. Se uma etapa pedir módulo novo, ela justifica no `DECISIONS.md` na hora, não aqui.
