# N3.7 — Câmera 360 × 405

`assets/data/camera.ron` não mudou (`view_height: 405.0`). As duas câmeras existem só no exemplo isolado.

## Fidelidade dos pixels

- **`camera_360.png`** (vista `game-360-1x`, `run6` #2): a captura é idêntica ao sprite. São **0 diferenças em 2458 texels opacos**, conferido texel a texel na posição (584, 242) da janela. 1 texel = 1 px de tela.
- **`camera_405.png`** (vista `game-405`): 0,889 px por texel. O quadro de passada cai de 85×101 para cerca de 76×89 px (medida da N3.6), com colunas e linhas perdidas de forma irregular.

## Impacto no enquadramento (cálculo a partir dos dados do jogo)

Janela de 1280×720. A largura visível é 16/9 da altura.

| | 405 (jogo) | 360 (hipótese) | Diferença |
|---|---|---|---|
| área visível | 720 × 405 u | 640 × 360 u | −11 % por eixo, −21 % em área |
| personagem (53 u) | 13,1 % da altura | 14,7 % | maior e mais legível |
| acima do centro do jogador (`vertical_offset` 40) | 242 u | 220 u | −22 u |
| abaixo do centro | 162 u | 140 u | −22 u |
| à frente, com `look_ahead` 70 | 430 u | 390 u | −40 u |
| tempo para algo a 310 u/s cruzar a distância à frente | 1,39 s | 1,26 s | −0,13 s |
| sala de 640 u de altura (Vigil Gates) | 63 % visível | 56 % visível | mais rolagem vertical |

- **Pulo:** a altura máxima é 720² / (2 × 1900) ≈ 136 u. Com a câmera parada (a `dead_zone_up` de 90 u absorve parte da subida), a margem acima do centro do jogador é de 242 u na 405 e de 220 u na 360. O pulo cabe nas duas, com menos folga na 360.
- **Plataformas:** as passarelas de 16 u de espessura e os blocos de 36–50 u continuam legíveis. Uma passarela a 300 u acima do chão já fica fora da tela nas duas câmeras; na 360, uma a 260 u também sai.
- **Inimigos e projéteis:** veem-se 40 u a menos à frente. Projéteis vindos de longe aparecem cerca de 10 % mais tarde.
- **HUD:** desenhado em pixels de interface, não escala com o zoom (`world_scenery_tests.rs`). Ocupa a mesma área nas duas câmeras e cobre proporcionalmente mais mundo na 360.
- **Distância de leitura:** na 360 cada texel é um pixel inteiro. O rosto e as mechas aparecem como foram reduzidos. Na 405, detalhes de 1 texel (olho, fivelas) podem sumir ou duplicar.
- **Cenários internos:** salas estreitas e altas sentem mais o corte vertical.

## Riscos (não implementados)

Trocar para 360 exigiria rever a altura das plataformas e dos pulos que dependem de ver a plataforma de destino, o `dead_zone_up` (90) e o `look_ahead` (70), e repetir os testes de câmera (`world_camera_tests.rs`). A câmera 360 continua sendo a referência de fidelidade da prova isolada, mas não está aprovada para produção.
