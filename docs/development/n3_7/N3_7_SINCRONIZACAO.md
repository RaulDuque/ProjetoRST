# N3.7 — Estudo de sincronização

Script: `art-src/n3-7-run-sync/scripts/sync_study.py`. Dados para o Bevy: `generated/sync.ron`. Evidências: `run_speed_comparison.png`, `run_speed_variant.gif` e `run_12fps.gif`.

O pé apoiado fica parado no mundo quando `velocidade = fator × 157`, onde o fator multiplica os 12 fps do `run6`. O deslizamento é `|velocidade − fator × 157|`. Todos os cenários usam os mesmos seis quadros aprovados, sem nenhum quadro novo.

| Cenário | fps | Fator | Velocidade | Deslizamento | Ciclo | Cadência |
|---|---|---|---|---|---|---|
| A original | 12 | 1,00 | 310 u/s (jogo) | **153 u/s (49 %)** | 0,50 s | 4 passos/s |
| B clipe acelerado | 23,7 | 1,975 | 310 u/s (jogo) | 0 | 0,25 s | 7,9 passos/s |
| C deslocamento reduzido | 12 | 1,00 | 157 u/s (só no exemplo) | 0 | 0,50 s | 4 passos/s |
| D moderado | 18 | 1,50 | 236 u/s (experimental) | 0 | 0,33 s | 6 passos/s |
| D na velocidade do jogo | 18 | 1,50 | 310 u/s | 74 u/s (24 %) | 0,33 s | 6 passos/s |
| E passada maior | — | — | 310 u/s | — | — | ver abaixo |

## A comparação visual

- **`run_speed_variant.gif`:** quatro faixas, A, B, C e D, durante 2 s. Mesma câmera fixa, mesmo fundo, escala de 1 texel = 1 px e os mesmos quadros. Cada faixa mostra:
  - a régua do chão, com marcas a cada 20 u;
  - pegadas verdes: a posição no mundo da área de contato em cada passo de 20 ms, que somem em 0,6 s;
  - fps efetivo, fator, velocidade, posição, fase e deslizamento.
- **`run_speed_comparison.png`:** uma exposição múltipla de um ciclo de cada cenário.
- **Como ler:** na A as pegadas de cada passo ficam espalhadas, porque o pé é arrastado. Na B, C e D elas ficam em pares juntos, com o pé parado no chão.
- B, C e D produzem os mesmos lugares no chão, só em tempos diferentes. O que muda entre eles é a cadência.

## Leitura de cada cenário

- **A:** a arte e o tempo aprovados, mas o pé patina metade do caminho. É o defeito que a N3.6 previa.
- **B:** sem deslizamento e sem tocar na física. O clipe dura 0,25 s, ou seja, quase 8 passos por segundo. Isso é rápido, mas coerente com um corpo que anda 5,8 alturas por segundo. O jogo já tem esse mecanismo: `speed_reference: Some(157.0)` num clipe `Loop` faz `locomotion_rate` = 310/157 = 1,97, dentro do limite de 2,5.
- **C:** a corrida mais natural, mas a 157 u/s, o que é mais lento que a caminhada do jogo (190). Serve só de referência.
- **D:** 18 fps parece uma corrida forte sem virar "pedalada". Só fica sem deslizamento a 236 u/s, que exigiria mudar a física. Na velocidade do jogo, desliza 24 %.
- **E:** a 12 fps e 310 u/s, cada passo cobriria 78 u (155 texels), o dobro da passada atual (39 u). Isso exige:
  - um novo vídeo de corrida com passada de velocista (pernas abertas quase 2×, voo maior);
  - um canvas mais largo, perto de 160 texels, porque a passada atual já ocupa 85 dos 112;
  - extrair e reduzir de novo, com o risco de a identidade variar entre vídeos;
  - nova avaliação artística.

  O pipeline (`N3_7_PIPELINE.md`) cobre a parte técnica, mas o custo principal é gerar e aprovar outro vídeo. Nenhum quadro foi gerado nesta etapa.

## Recomendação

1. **Curto prazo: B, com o contrato existente.** Na integração futura, o clipe de corrida leva `speed_reference: Some(157.0)`. Na velocidade máxima o pé fica parado, e na aceleração (abaixo de 310) a cadência cai junto com a velocidade. Não muda física, arte nem quadros.
2. **Se o proprietário achar a cadência de B frenética**, o caminho é E (um vídeo com passada maior, reproduzido perto de 12–16 fps). D só funciona se a velocidade do jogo mudar, e isso precisa de autorização.

**Teste adicional a 15 fps** (pedido do proprietário): os mesmos quadros a 310 u/s em 12, 15, 18 e 23,7 fps. A 15 fps o deslizamento é de 114 u/s (37 %). Ver `N3_7_TESTE_15FPS.md`. Sem decisão.

A prova interativa (`N3_7_BEVY.md`, tecla G) mostra A, B, C e D em movimento, na mesma câmera, para o proprietário decidir. Nada foi aplicado ao jogo.
