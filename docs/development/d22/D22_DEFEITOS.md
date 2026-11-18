# D22 — Defeitos

## Corrigidos

- Wind-up do chefe era cancelado por qualquer hit. O padrão não completava.
- Fase 2 podia pular o lunge se o jogador batia durante o cooldown.
- Patrulha caía no vão da porta inferior. Spawn e patrulha ficam na laje direita.
- A porta do teto no Reliquary não era alcançável num pulo. Escada one-way adicionada.

## Abertos

- Esquiva nos últimos ~0,14 s do tell pode levar o golpe. I-frames duram 0,28 s e a hitbox fica 0,15 s depois do wind-up. A regra global de esquiva não foi mudada. O teste cobre a esquiva iniciada com 0,18–0,32 s de tell ainda de pé, para longe do golpe.
- `01_arena.png` mostra a chegada e a barra, não o corpo do sentinela: a câmera segue o jogador e o chefe está à direita.
- A subida da escada do Reliquary não foi filmada. Os testes de rota e de chegada passaram com as plataformas novas.
- Não há quadro do projétil batendo na parede.
- Dificuldade para um humano não foi avaliada. O script cura o jogador.
