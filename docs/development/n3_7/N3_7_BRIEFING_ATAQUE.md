# N3.7 — Briefing de vídeo: ataque desarmado

Para o proprietário usar numa ferramenta de geração de vídeo. **Não foi executado pelo agente, e nenhuma imagem foi enviada a serviço externo.** Anexe as mesmas referências da corrida.

## Texto para a ferramenta

> Side view, facing right, of the same character as the reference: a tall adult man with long straight silver-white hair, serious face, dark-grey button shirt with rolled sleeves, black leather bracers, dark brown-black trousers, tall black boots. Stylised 2D game art with detailed shading, same rendering as the reference. Pure flat chroma-green background (#00B140), no floor, no shadow, no text. Fixed camera, no zoom, no pan, full body always inside the frame. He starts in a relaxed fighting guard (fists up, knees slightly bent), pulls the rear fist back as anticipation, steps forward slightly and throws one straight punch with the rear hand at shoulder height, holds the fully extended arm for an instant, pulls the arm back, and returns to the same guard. The feet stay grounded. No weapon, no glow, no motion trails, no impact flash, no sparks, no effects of any kind. Hair moves naturally with the body.

## Especificação

| Item | Requisito |
|---|---|
| Orientação | perfil direito |
| Personagem | o mesmo da corrida aprovada: rosto, cabelo, camisa, braceletes, calça, botas |
| Sequência | guarda (≥ 0,5 s) → antecipação → execução → impacto (braço estendido, 2–3 quadros) → recuperação → guarda (≥ 0,5 s) |
| Duração | 2–3 s |
| FPS | 24 |
| Câmera | fixa: sem zoom, sem pan |
| Escala | a mesma da corrida (cerca de 450 px de altura em pé) |
| Fundo | verde chapado |
| Anatomia | o braço estendido não pode ficar mais comprido que o braço parado. Ombro e cotovelo coerentes |
| Cabelo | o mesmo comprimento e massa. Acompanha o tronco |
| Roupa | a mesma. Mangas dobradas, braceletes visíveis |

## Separação conceitual (para a produção futura)

- **Corpo:** tudo o que o vídeo mostra.
- **Mãos:** a posição do punho em cada quadro será medida depois, como estimativa experimental, para um futuro soquete de arma.
- **Arma futura:** não aparece no vídeo. Será uma camada à parte.
- **Efeitos visuais:** não aparecem no vídeo. Impacto, rastro e brilho serão camadas do jogo, nunca desenhados no corpo (o defeito da folha `attack` da N3.6).

Nenhuma hitbox será criada a partir deste vídeo, e o sistema de combate não muda.

## Critérios de aceitação

1. A guarda inicial e a final são a mesma pose.
2. A antecipação é visível: o punho recua antes de avançar, em pelo menos 2 quadros.
3. O impacto tem o braço totalmente estendido, sem borrão.
4. Os pés ficam no chão. O passo à frente é pequeno, e o corpo volta ao lugar.
5. Nenhum efeito no quadro.

## Defeitos que invalidam o vídeo

- Efeito de impacto, faísca, rastro ou brilho desenhado.
- Arma, luva, manopla ou qualquer objeto novo.
- Dois golpes, ou um golpe com troca de lado (espelhamento).
- Braço esticado além do comprimento real, ou mão deformada no impacto.
- Marcadores, guias ou texto no quadro (como os pontos verdes da `al.jpg`).
- Câmera móvel ou personagem cortado.
