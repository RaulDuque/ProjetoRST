# N3.7 — Teste adicional: corrida a 15 fps

Pedido do proprietário depois da publicação da N3.7: uma cadência entre os 12 fps originais e os 23,7 fps do cenário B. **Nenhuma velocidade foi escolhida.** Esta página só registra o teste para avaliação visual.

## O que foi feito

- **Os mesmos seis quadros aprovados** (`n3-6-animation-proof/generated/run6.png`, vídeo 48–58). Nenhum pixel novo. O teste de hash da baseline continua passando.
- **Velocidade horizontal de 310 u/s** (`PLAYER_RUN_SPEED`, sem mudança), em todas as variantes. Só muda o fator de reprodução.
- **Uma lista nova em `generated/sync.ron`, `game_speed_steps`.** Os cenários A–D e as evidências anteriores ficaram iguais (os arquivos regerados são idênticos).

## Números (referência do pé apoiado: 157 u/s)

| fps | Fator | Ciclo | Cadência | Deslizamento estimado |
|---|---|---|---|---|
| 12 | 1,00 | 0,500 s | 4,0 passos/s | 153 u/s (49 %) |
| **15** | **1,25** | **0,400 s** | **5,0 passos/s** | **114 u/s (37 %)** |
| 18 | 1,50 | 0,333 s | 6,0 passos/s | 74 u/s (24 %) |
| 23,7 | 1,975 | 0,253 s | 7,9 passos/s | 0 (= cenário B) |

Deslizamento = |310 − fator × 157|. É uma estimativa: a referência vem de um vídeo gerado por IA e o pé rola (`N3_7_DESLIZAMENTO.md`). A 15 fps o pé apoiado recua 196 u/s, enquanto o corpo anda 310: ainda escorrega para a frente cerca de 1/3 do caminho.

## Evidências

| Arquivo | Conteúdo |
|---|---|
| `run_fps_comparison.gif` | 12, 15, 18 e 23,7 fps em quatro faixas, 2 s. Mesma câmera fixa, mesma escala (1 texel = 1 px), mesma velocidade e, portanto, a mesma distância percorrida. Régua a cada 20 u e pegadas verdes |
| `run_fps_comparison.png` | um ciclo de cada fps em exposição múltipla, com pegadas |
| `run_15fps.gif` | só 15 fps, atravessando a régua por 4 s: 10 ciclos e 3,4 travessias |
| `run_15fps_in_place.gif` | 15 fps no lugar, 3×. Os atrasos de GIF são centésimos inteiros, então os 6 quadros alternam 70/70/60 ms (400 ms por ciclo = 15 fps exatos) |
| `bevy_n37_15fps.png` | a prova no Bevy a 15 fps, vista 360 inteira, HUD com fator, velocidade e deslizamento |

Os GIFs em movimento são amostrados a cada 20 ms (o menor atraso que os navegadores respeitam). A 15 fps cada quadro fica 60 ou 80 ms na tela, e em média 66,7 ms. No Bevy a troca é exata.

## No exemplo isolado

`N3_INTERACTIVE=1 cargo run --example n3_visual_proof`, B até `n3.7-sync-proof`:

1. **J** entra no teste e alterna 12 → 15 → 18 → 23,7 fps, sempre a 310 u/s;
2. **G** volta aos cenários A–D;
3. **T** liga o deslocamento sobre a régua, e **V** troca a câmera.

O HUD mostra o fps efetivo, o fator, a velocidade e o deslizamento.

## Leitura (sem decisão)

15 fps fica no meio do caminho: a cadência de 5 passos por segundo ainda parece corrida, mas o pé escorrega 37 %, menos que os 49 % dos 12 fps e mais que os 24 % dos 18 fps. A escolha entre naturalidade da cadência e pé parado fica com o proprietário, depois de ver `run_15fps.gif` e `run_fps_comparison.gif`, ou o Bevy com a tecla J.

## Publicação

Push em `origin/art/v1.2-protagonist`, sem force, merge ou rebase:

- `f033c015ce440092cda9208ba1cd4632505df34e` feat(visual): add n3.7 15 fps run test at game speed
- `f94b14f586103981e32361a32c2fe806f5cf83bf` docs(n3): record the n3.7 15 fps run test and evidence
- O commit que grava esta seção é o registro da publicação. O hash remoto final é o HEAD depois do push dele.
