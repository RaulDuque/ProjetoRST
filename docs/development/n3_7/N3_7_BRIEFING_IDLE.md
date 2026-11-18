# N3.7 — Briefing de vídeo: idle lateral (e frenagem)

Para o proprietário usar numa ferramenta de geração de vídeo. **Não foi executado pelo agente, e nenhuma imagem foi enviada a serviço externo.** O candidato parado N3.5 continua sendo a referência principal. Este vídeo existe para dar respiração a ele e para dar os quadros de **frenagem** que faltam na transição (`N3_7_TRANSICAO.md`).

## Texto para a ferramenta: idle

> Side view, facing right, of the same character as the reference: a tall adult man with long straight silver-white hair falling down his back, serious face, dark-grey button shirt with rolled sleeves, black leather bracers, dark brown-black trousers, tall black boots. Stylised 2D game art with detailed shading, same rendering as the reference. Pure flat chroma-green background (#00B140), no floor, no shadow, no text. Fixed camera, no zoom, no pan, full body inside the frame. He stands relaxed and still, arms at his sides, breathing slowly: the chest and shoulders rise and fall subtly, the hair sways very slightly, and he shifts his weight minimally. He does not move his feet, does not turn, does not raise his fists, does not look at the camera. The last frame should match the first frame so the clip can loop.

## Texto para a ferramenta: frenagem (vídeo separado, opcional)

> Same character, same style, same background and fixed camera. He runs in place facing right (the same run as the reference video), then slows down over about half a second, plants his front foot, brings the rear foot next to it and ends standing still in the relaxed idle pose, hair settling down his back. No turning, no sliding effects, no dust.

## Especificação

| Item | Idle | Frenagem |
|---|---|---|
| Orientação | perfil direito | perfil direito |
| Duração | 3–4 s, em loop | 2–3 s |
| FPS | 24 | 24 |
| Sequência | parado, respiração, parado igual ao início | corrida → desaceleração (6–12 quadros) → parado |
| Câmera | fixa | fixa |
| Escala | a mesma da corrida (cerca de 450 px em pé) | a mesma |
| Fundo | verde chapado | verde chapado |

## Restrições

- **Mesmo rosto, cabelo, roupa, escala, orientação e enquadramento** da corrida aprovada.
- **Movimentos permitidos:** respiração sutil, ombros, cabelo discreto, transferência mínima de peso.
- **Proibido:** pose de combate, punhos fechados erguidos, equipamento, capa, arma, vento forte no cabelo, olhar para a câmera, virar o corpo.

## Critérios de aceitação

1. O primeiro e o último quadro do idle são quase iguais (loop sem salto).
2. A altura do corpo varia no máximo 1 % (a respiração não pode virar agachamento).
3. Os pés não saem do lugar no idle.
4. Na frenagem, o parado final tem a mesma silhueta do parado do vídeo de corrida (quadro 0).
5. O cabelo mantém comprimento e massa.

## Defeitos que invalidam o vídeo

- Pose de combate ou guarda.
- Cabelo crescendo, mudando de cor ou ganhando mechas novas.
- Mudança de roupa, de acessórios ou de proporções.
- Câmera móvel, zoom ou corte.
- Pernas ou braços duplicados por interpolação.
- Fundo não uniforme.
