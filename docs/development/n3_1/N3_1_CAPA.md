# N3.1 — Capa

## Malha do manequim

No ataque e no pulo renderizados, a capa vermelha fica atrás do casaco. `cape_jump.png`, `cape_attack.png` e `jump_sequence.png` mostram isso. A capa não foi desligada para esconder um cruzamento. Nestes renders ela faz parte da malha, então o exemplo não a troca por um sprite separado: o HUD da fonte renderizada diz que a capa está na imagem.

## Sprite da N3

O cruzamento da N3 era a faixa de pano desenhada na frente do corpo virado para a direita durante o pulo. Os valores de balanço da capa no pulo passaram a ser negativos em `art-src/n3-proof/scripts/build_n3_proof.py`, e as folhas foram geradas de novo. Em `docs/development/n3/evidence/bevy_jump.png` (quadro 1 de 6, vista próxima) a faixa roxa fica atrás do tronco. Essa captura é um quadro, não o clipe inteiro.
