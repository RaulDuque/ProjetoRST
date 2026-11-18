# N3.7 — Relatório final

A corrida aprovada continua byte a byte igual. A medição dos pés corrige a velocidade da passada para 157 u/s (a N3.6 estimava 78): a 12 fps e 310 u/s, o pé desliza 49 %. Quatro cenários foram comparados com os mesmos seis quadros. A recomendação é acelerar o clipe pelo `speed_reference` que o contrato já tem, sem mexer na física. A borda de luz de 1 texel mais que dobra o contraste das pernas no escuro. O vídeo tinha uma partida real, que virou o clipe `run_start`. O pipeline reutilizável reconstrói a N3.6 texel a texel. Nenhum gate artístico foi aprovado pelo agente, e a N4 e a N5 continuam bloqueadas.

## A. Estado inicial

`art/v1.2-protagonist` em `d4e5e046c3ae1e10c1cb5fd9f3f2945867e343c4`, local igual ao remoto. A árvore estava limpa, exceto arquivos não rastreados em `Md-usario/`. `N3_7_AUDITORIA.md`.

## B. Baseline

O candidato N3.5, os seis quadros do `run6` (vídeo 48, 50, 52, 54, 56, 58), o `run.ron` (12 fps, 0,5 s, 112×128, pivô 56,118, densidade 2), o `run10`, o quadro de pé e as grades não mudaram: 21 arquivos conferidos por hash num teste. `N3_7_BASELINE.md`, `run_baseline.png` e `run_12fps.gif`.

## C. Velocidade

O jogo corre a 310 u/s, ou 5,8 alturas de corpo por segundo. A passada do vídeo cobre 157 u/s, uma razão de 1,97. `PLAYER_RUN_SPEED` não mudou. `N3_7_VELOCIDADE.md`.

## D. Passada

Pés segmentados quadro a quadro, com fases de apoio, impulsão, voo e contato. O pé recua 150 u/s no passo 58→50 e 164 u/s no passo 53→56. O quadril varia 2 u (corrida no lugar). `N3_7_DESLIZAMENTO.md`, `foot_contact_analysis.png` e `foot_sliding_metrics.png`.

## E. Deslizamento

| Cenário | Deslizamento |
|---|---|
| A (12 fps, 310 u/s) | 153 u/s (49 %) |
| D na velocidade do jogo | 74 u/s (24 %) |
| B, C, D experimental | 0 |

A estimativa de 78 u/s da N3.6 descartava os deslocamentos grandes.

## F. Alternativas

| Cenário | fps | Velocidade | Ciclo |
|---|---|---|---|
| A | 12 | 310 | 0,5 s |
| B | 23,7 | 310 | 0,25 s, 7,9 passos/s |
| C | 12 | 157 (só exemplo) | 0,5 s |
| D | 18 | 236 (experimental) | 0,33 s |
| E | — | 310 | passada de 78 u, o dobro; exige vídeo novo, sem quadros gerados |

`N3_7_SINCRONIZACAO.md`, `run_speed_variant.gif` e `run_speed_comparison.png`.

## G. Recomendação

**B**, pelo contrato existente: `speed_reference: Some(157.0)` no clipe de corrida, na integração futura. Não muda a física, a arte nem os quadros. Se a cadência parecer frenética, **E** (um vídeo com passada de velocista). D exigiria mudar a velocidade do jogo.

## H. Contraste

Borda das pernas no quase preto: A 12,3; B 29,2; C 18,9; D 27,9. No castelo: A 10,1; B 21,1; C 9,7; D 10,1 (sem luz na cena). `N3_7_CONTRASTE.md`, `dark_original.png`, `dark_comparison.png` e `dark_comparison.gif`.

## I. Borda de luz

512 texels nos 6 quadros, só nas costas, do quadril para baixo, com a cor `q` da própria paleta, num arquivo novo (`run6_rim.png`). Mais que dobra a borda no escuro e quase não muda nada no claro. É a candidata, mas não foi aplicada. `dark_rim_light.png` e `bevy_n37_rim.png`.

## J. Transição

- **Partida:** os quadros reais do vídeo, 3 a 13, viraram `run_start` (6 quadros, 0,5 s). Ele emenda no `run6` #0 tão bem quanto o próprio loop (0,544 contra 0,515 de diferença de silhueta).
- **Parada:** não há material, então é um corte direto depois do `run6` #3. É o ponto fraco, e ficou documentado.
- **Cabelo:** é a maior mudança (177 → 318 texels).

`N3_7_TRANSICAO.md` e `idle_run_transition.gif`.

## K. Alinhamento

Cada fonte é alinhada pelo próprio pivô: N3.5 (40,118) → (56,118). Os dois parados ficam com quadril a 0,5 texel e cabeça a 1 texel um do outro. O quadro 0 do vídeo fica 1 texel acima do chão. Não há ajuste por quadro. `idle_run_alignment.png`.

## L. Câmera

- **360:** 0 diferenças em 2458 texels.
- **405:** reduz para 0,889 px por texel.
- **Enquadramento:** a 360 mostra 11 % menos por eixo (640×360 u contra 720×405), 40 u a menos à frente e 22 u a menos acima e abaixo.

`camera.ron` não mudou. `N3_7_CAMERA.md`, `camera_405.png` e `camera_360.png`.

## M. Pipeline

`animation_pipeline.py` + uma configuração JSON fazem: metadados, hash, decodificação, chave, escala única, redução, pivô global, grades, atlas, manifesto, caixas, proveniência, validação e prévia. Ele reconstrói a N3.6 com 13/13 grades idênticas e produziu a partida real. `N3_7_PIPELINE.md` e `pipeline_output.png`.

## N. Proveniência

Por quadro: arquivo, SHA-256, índice, tempo, extração, chave, escala, canvas, pivô, ajustes (nenhum), edição manual (não), gerado pelo agente (não), fonte gerada por IA (sim, pelo proprietário) e versão do pipeline. Um teste confere. `N3_7_PROVENIENCIA.md`.

## O. Contrato

O `AnimationManifest` atual já cobre o pedido, incluindo `speed_reference`. Caixas, proveniência e status ficam em arquivos ao lado. Há uma lacuna registrada: a regra de "próximo clipe" para partida → corrida. Nenhum manifesto de produção mudou. `N3_7_CONTRATO_ANIMACAO.md`.

## P. Salto

Briefing de vídeo com texto para a ferramenta, sequência (preparação → aterrissagem → recuperação), 24 fps, 3–4 s, câmera fixa, fundo verde e critérios de aceitação e defeitos. Não foi executado. `N3_7_BRIEFING_SALTO.md`.

## Q. Ataque

Direto desarmado: guarda → antecipação → execução → impacto → recuperação → guarda, sem efeitos desenhados no corpo. Corpo, mãos, arma e efeitos ficam separados. Não foi executado. `N3_7_BRIEFING_ATAQUE.md`.

## R. Idle

Respiração lateral em loop, mais um vídeo opcional de frenagem para completar a transição. Não foi executado. `N3_7_BRIEFING_IDLE.md`.

## S. Modularidade

Cabelo assado (até 318 texels atrás do tronco), capa sem folga no canvas, punho sem soquete por quadro, cabeça e peito variando, espelho e sincronização sob `speed_reference`. Nada foi implementado. `N3_7_AVALIACAO.md`.

## T. Bevy

Fonte `n3.7-sync-proof` com o sistema `sync_n37`:
- teclas G (cenário), H (transição), L (variante) e K (fundo), mais O, T, V, Espaço e `.`;
- régua do chão e HUD com fps, fator, velocidade, posição, fase, deslizamento e `NOT APPROVED`.

As fontes 0 a 7 não mudaram. `N3_7_BEVY.md`.

## U. Evidências

Em `docs/development/n3_7/evidence/`:
- folhas PNG: `run_baseline`, `run_speed_comparison`, `foot_contact_analysis`, `foot_sliding_metrics`, `idle_run_alignment`, `dark_original`, `dark_rim_light`, `dark_comparison` e `pipeline_output`;
- capturas do Bevy: `camera_405`, `camera_360`, `bevy_n37`, `bevy_n37_rim`, `bevy_n37_backlight` e `bevy_n37_transition`;
- GIFs com movimento real: `run_12fps`, `run_speed_variant`, `idle_run_transition` e `dark_comparison`.

As variantes experimentais (B, C, D) estão identificadas em cada imagem.

## V. Testes

fmt, check e clippy (`-D warnings`) limpos. `cargo test -j 2`: 51 suítes, 623 passaram, 0 falharam, incluindo `tests/n3_7_sync_tests.rs` (novo, 7 testes). O exemplo gerou 47 capturas, e as antigas saíram idênticas. `N3_7_TESTES.md`.

## W. Segurança

Os SHA-256 dos 11 arquivos de `Md-usario/assets-user/prota/base/` são iguais aos registrados na N3.6. Nada de `Md-usario/` foi alterado ou commitado. `src/`, `assets/` e as evidências anteriores não mudaram. Nenhum arquivo foi enviado a serviço externo.

## X. GitHub

Push em `origin/art/v1.2-protagonist`, sem force, merge ou rebase:

- `41f2ad40e219b7f56f19306341c948e052a833ee` feat(visual): add n3.7 run synchronization experiments
- `62aeb2b7a8b82e7359141687ab9078e939f418da` docs(n3): record n3.7 animation pipeline and evidence
- O commit que grava este parágrafo é o registro da publicação. O hash remoto final é o HEAD depois do push dele.

## Y. Limitações

- O vídeo de referência foi gerado por IA e tem fantasmas (49, 55). As medições são aproximadas.
- B toca a quase 8 passos por segundo. Só o olhar do proprietário diz se convence.
- Não há quadros de parada: a parada é um corte.
- O quadro 0 e a partida #0 a #3 ficam 1 texel acima do chão.
- A contraluz do Bevy é uma placa do tamanho do canvas.
- Não há salto, ataque, idle animado nem soquetes.

## Z. Próxima decisão

1. **Gate B:** escolher o cenário. A recomendação é B (`run_speed_variant.gif` e Bevy, tecla G).
2. **Gate C:** escolher B (borda de luz), D (luz de cena) ou nenhum (`dark_comparison.gif`).
3. **Gate D:** a partida real convence? Pedir o vídeo de frenagem?
4. **Gate A:** confirmar que as variantes e a partida preservam a identidade.
5. **Câmera 360:** continua pendente.
6. Gerar os vídeos de salto, ataque e idle pelos briefings e passá-los pelo pipeline.

Nenhum recurso vai para o jogo principal e a N4 não começa sem autorização.
