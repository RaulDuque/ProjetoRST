# N3.1 — Animações

Nenhum tempo de combate do jogo foi alterado. As poses são as do blend.

## Attack

Sete quadros, sem antecipação no clipe de origem. A folha `attack_sequence.png` mostra impacto, seguimento e recuperação. O braço sobe e desce. A arma não está na imagem: o soquete foi medido à parte e o exemplo pode apoiar a espada geométrica da N3 nesse ponto. O HUD avisa que essa espada é o sprite da N3, não a malha.

O `attack.ron` registra o quadro 6 também como um clipe `idle` de um quadro. O leitor de manifesto exige um clipe chamado `idle`. Isso não é uma animação de respiração.

## Jump

`jump_rise` e `fall`, oito quadros no total, em `jump_sequence.png`. O corpo agacha, estende e volta. A capa de malha fica atrás do casaco nos oito quadros. Não há um grande arco no ar: a câmera de locomoção mantém a figura dentro da célula, e o deslocamento vertical do clipe é curto.

## Walk

O ciclo plantado de 16 quadros de `animations.py`, não o `run` publicado. Os 16 PNG estão em `art-src/n3-proof/blender/walk/` e a tira é `walk_sequence.png`. O quadro de contato abre a passada, com um pé à frente e o outro atrás, e a capa fica atrás do casaco. Não houve correção de pixel em cima desses quadros, então deslize residual de pé, se existir, continua o do clipe 3D.

## Idle

Não foi re-renderizado um ciclo de respiração. O idle publicado já tinha um quadro de referência na N3 (`docs/development/n3/evidence/blender_idle_v22.png`). Repetir esse quadro não responde a transferência de peso.
