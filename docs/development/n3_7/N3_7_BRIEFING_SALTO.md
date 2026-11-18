# N3.7 — Briefing de vídeo: salto

Para o proprietário usar numa ferramenta de geração de vídeo. **Não foi executado pelo agente, e nenhuma imagem foi enviada a serviço externo.** Anexe as mesmas referências usadas no vídeo da corrida: a ficha original do protagonista e, se a ferramenta aceitar, um quadro parado do vídeo de corrida.

## Texto para a ferramenta

> Side view, facing right, of the same character as the reference: a tall adult man with long straight silver-white hair falling to the middle of the back, serious face, dark-grey button shirt with rolled sleeves, black leather bracers on both wrists, dark brown-black trousers, tall black boots with straps. Stylised 2D game art with detailed shading, same rendering as the reference. Pure flat chroma-green background (#00B140), no floor, no shadow, no text. Fixed camera, no zoom, no pan, the full body always inside the frame with margin above the hair and below the boots. He performs ONE normal vertical jump in place: stands still, bends the knees (anticipation), pushes off, rises with legs tucking slightly, reaches the apex, falls with legs extending, lands on both feet, absorbs the impact by bending the knees, and returns to the same standing pose. No flips, no spins, no acrobatics, no cape, no weapon, no extra clothing, no effects. Hair follows the motion naturally and keeps its length.

## Especificação

| Item | Requisito |
|---|---|
| Orientação | perfil direito, a mesma da corrida |
| Personagem | o mesmo rosto, cabelo, roupa e proporções da corrida aprovada |
| Sequência | parado (≥ 0,5 s) → preparação → impulsão → subida → ápice → queda → aterrissagem → recuperação → parado (≥ 0,5 s) |
| Duração | 3–4 s |
| FPS | 24 (como a corrida) |
| Resolução | ≥ 496×864 na vertical, ou o mesmo formato da corrida |
| Câmera | fixa: sem zoom, sem pan, sem corte |
| Escala | igual à da corrida. No quadro parado, o personagem deve ocupar cerca de 450 px de altura |
| Fundo | verde chapado e uniforme, sem gradiente, sem chão desenhado, sem sombra |
| Anatomia | as pernas mantêm o comprimento. Nada de esticar membros no ápice |
| Cabelo | o mesmo comprimento. Sobe na queda e cai na subida, sem mechas novas |
| Roupa | a mesma. Sem capa, casaco, cinto novo, tecido solto ou faixas |

## Critérios de aceitação

1. O parado inicial tem a mesma altura (±3 %) do quadro 0 do vídeo de corrida.
2. A cabeça, o cabelo, a mão e a bota ficam inteiros dentro do quadro em todos os quadros.
3. O corpo sobe e desce no lugar: a coluna do quadril varia menos de 10 % da largura do quadro.
4. A volta ao parado repete a pose inicial.
5. As fases (preparação, ápice, aterrissagem) são distinguíveis quadro a quadro.

## Defeitos que invalidam o vídeo

- Mortal, giro, chute, pose de ataque ou qualquer acrobacia.
- Um objeto que não existe no personagem (a peça de tecido do `idle_run_jump` #17 da N3.6).
- Quadros com pernas ou braços duplicados por interpolação em mais de 2 quadros seguidos.
- Mudança de rosto, de cor de cabelo ou de roupa durante o vídeo.
- Câmera que se move, dá zoom ou corta o personagem.
- Fundo com sombra, gradiente, chão ou objetos.
