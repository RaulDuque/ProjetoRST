# N3.6 — Estudo de ataque desarmado

Evidência: `attack_pose_analysis.png`. Nenhum clipe, hitbox ou comando foi criado. Combate e magia não foram tocados.

## `attack` (folha `p5nj`), olhando à direita

| Pose | Fase |
|---|---|
| #1 | guarda (punhos junto ao rosto) |
| #2 | extensão do direto |
| #3 | impacto, com rastro de velocidade desenhado |
| #4 | recuperação para a guarda |
| #5, #6 | guarda (retorno ao repouso de combate) |
| #7 | guarda (segunda tomada) |
| #8 | extensão do direto |
| #9 | impacto com arco branco de golpe |
| #10 | recuperação |
| #11, #12 | guarda |

A folha traz **duas tomadas da mesma sequência**, e não um golpe de 12 quadros.

## Consistência

- **Punhos e braceletes:** consistentes. O punho fechado tem o mesmo tamanho em todas as poses.
- **Cotovelo e ombro:** na extensão, o ombro avança e o torso gira, mostrando as costas (#3, #9). É coerente com um direto.
- **Pernas:** a base quase não muda entre as poses, o que ajuda um clipe parado no lugar.
- **Cabelo:** solto para trás no impacto (#3, #9) e assentado na guarda. Coerente.
- **Direção:** sempre à direita.
- **Efeitos:** o rastro (#3) e o arco (#9) estão desenhados junto do corpo. Num sprite, teriam de ser camada de efeito separada (o jogo já tem o atlas `effects`). Recortar o corpo sem eles exige cuidado, porque o arco encosta no punho.
- **Antecipação:** não existe pose de recuo do braço antes do golpe, porque #1 e #2 vão direto da guarda à extensão. Faltam também uma retração intermediária e o retorno ao idle normal, sem guarda.

## `fight_left` (`al.jpg`), olhando à esquerda

Golpes retos (#1–4, #9–12), guarda (#5, #8), agachamento (#6), avanço com garras (#13–15), rasteira e golpes baixos (#16–19, #24–27), postura de força (#20–23) e saltos (#28–31). Tem marcadores verdes de pivô. Para a direção do jogo, precisaria ser espelhada, e o espelho troca o lado dos braceletes e do cabelo.

## Conclusão

Há material para um direto curto (guarda → extensão → impacto → recuperação → guarda) em `attack` #1–6, mas faltam a antecipação e o efeito separado. Como nas outras animações, um vídeo lateral contínuo resolveria a continuidade melhor que poses soltas.
