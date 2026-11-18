# N3.7 — Transição parado → corrida → parado

Script: `art-src/n3-7-run-sync/scripts/transition_study.py`. Dados: `analysis/transition_metrics.json` e `generated/transition.ron`. Evidências: `idle_run_transition.gif`, `idle_run_alignment.png` e `bevy_n37_transition.png`.

## Material real disponível

- **Parado:** o candidato N3.5 (80×128, pivô 40,118) e o quadro 0 do vídeo (`stand.png` da N3.6).
- **Partida:** o vídeo começa parado e sai para a corrida. Os quadros 1–2 são iguais ao 0, e o 3 é o primeiro com movimento. O pipeline extraiu os quadros 0–13 com a mesma escala e o mesmo deslocamento da corrida. O clipe `run_start` usa 3, 5, 7, 9, 11 e 13 (um a cada dois, como o `run6`), a 12 fps, em 0,5 s.
- **Parada:** o vídeo não tem. Ele termina correndo.

Nenhum quadro foi inventado ou interpolado.

## Onde a partida entra no ciclo

O quadro 13 tem a silhueta mais parecida com o 58 do ciclo (diferença 0,166). O passo seguinte do 58 é o 48, então o `run_start` termina e o `run6` começa no 48 (#0). A emenda 13 → 48 tem diferença de silhueta de 0,544, e a emenda 58 → 48 do próprio ciclo aprovado tem 0,515: a entrada é tão contínua quanto o loop.

## Três sequências comparadas (GIF, mesma câmera, guias de chão, quadril, cabeça e pivô)

1. **Troca direta:** N3.5 → `run6` → N3.5.
2. **Só o vídeo:** quadro 0 → `run_start` → `run6` → quadro 0.
3. **Misto:** N3.5 → `run_start` → `run6` → N3.5.

A parada é sempre um corte direto. Ele acontece depois do `run6` #3 (vídeo 54), o quadro de corrida mais parecido com o parado (diferença 0,59; os outros ficam entre 0,65 e 0,75).

## Medidas (texels, canvas 112×128, cada imagem pelo próprio pivô)

| | topo da cabeça | chão | quadril x | peito x | cabelo | texels opacos |
|---|---|---|---|---|---|---|
| N3.5 parado | 13 | 118 | 55,5 | 53,0 | 207 | 1320 |
| vídeo 0 parado | 12 | 117 | 56,0 | 55,0 | 177 | 1253 |
| partida #0 (vídeo 3) | 12 | 117 | 57,0 | 55,0 | 211 | 1527 |
| partida #5 (vídeo 13) | 14 | 118 | 55,0 | 59,5 | 284 | 2524 |
| run6 #0 (vídeo 48) | 16 | 118 | 57,5 | 57,5 | 318 | 2219 |

Diferença de silhueta: N3.5 × vídeo 0 = 0,154; vídeo 0 × partida #0 = 0,243; N3.5 × partida #0 = 0,289.

## O que foi encontrado

- **Pivô:** alinhar pelo pivô funciona. Os dois parados ficam com o quadril a 0,5 texel e a cabeça a 1 texel um do outro, sem nenhum ajuste por quadro.
- **Chão:** o quadro 0 do vídeo (e a partida #0) fica 1 texel acima do chão, como já registrado na N3.6. O N3.5 pisa na linha 118, então a troca N3.5 → partida sobe o personagem 1 texel.
- **Cabeça:** na corrida, o topo desce até a linha 16 (o tronco se inclina). Na troca direta a cabeça cai 3 texels de uma vez. Com a partida, ela desce aos poucos (12 → 14 → 16).
- **Cabelo:** o maior salto. Parado, o cabelo cai reto (177–207 texels). Na corrida, ele voa para trás (284–318). A partida faz essa passagem em 0,5 s. A troca direta faz o cabelo "explodir" num quadro.
- **Torso:** o peito avança 2,5–4,5 texels na corrida. Com a partida a mudança é gradual.
- **Paleta e acabamento:** o N3.5 veio da ficha, com 10 correções à mão, e o vídeo veio de uma redução automática. A silhueta é próxima (0,154), mas o N3.5 tem mais cabelo na frente do ombro e uma camisa um pouco mais clara. A troca N3.5 → vídeo é perceptível de perto, mas não muda a identidade.
- **Parada:** é o pior momento. Do `run6` #3 para o parado, as pernas fecham e o cabelo cai num único quadro. Para uma parada convincente faltam quadros de frenagem (de 3 a 6, de preferência do mesmo vídeo ou de um novo). Não há como resolver isso sem material novo.

## Recomendação

- **Partida:** usar o `run_start` (quadros reais do vídeo). A sequência 2 (só o vídeo) é a mais contínua. A 3 (misto) também funciona.
- **Parada:** pedir no próximo vídeo (`N3_7_BRIEFING_IDLE.md`) um trecho de frenagem que termine parado.
- Nenhum efeito visual foi usado para esconder os saltos.
