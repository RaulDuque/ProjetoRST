# N3.7 — Contrato de animação

## O contrato que já existe (inspecionado, não alterado)

`src/player/visual/manifest.rs`, `AnimationManifest` em RON:

| Campo do briefing | Onde já está |
|---|---|
| identificador, nome | chave do clipe (snake_case, validada) |
| quantidade de quadros | `count` (+ `first` no atlas) |
| duração por quadro | `durations` (uma por quadro, validadas > 0) |
| duração total | `ClipDef::total_duration()` |
| loop | `mode: Loop { fps } / Once { fps } / FitToAction` + `fallback` |
| canvas | atlas `cell`, `columns`, `rows`, `padding` |
| pivô | `pivot` (linha da sola, em texels a partir do canto) |
| direção | `facing` do manifesto (+ espelho na apresentação) |
| densidade, escala | atlas `density` (texels por unidade) |
| sincronização com a velocidade | `speed_reference` + `playback::locomotion_rate` (0,5–2,5×) |
| sockets | `SocketTable` à parte (`sockets.rs`) |

Ele cobre quase tudo. **Não é preciso um sistema paralelo.** Os manifestos produzidos pelo pipeline (`run_n36.ron`, `run_start.ron`) são `AnimationManifest` válidos e carregam no Bevy pelo mesmo código.

## O que fica fora do manifesto, em arquivos ao lado

| Campo | Arquivo | Por quê |
|---|---|---|
| caixas opacas | `boxes.ron` | só a prova e as ferramentas usam |
| proveniência | `provenance.json` | rastreabilidade, não é dado de jogo |
| status de aprovação | `provenance.json` (`approval`) | muda sem mexer no dado de jogo |
| fases e contatos dos pés | `foot_metrics.json`, `sync.ron` (`run6_phase`) | estudo |

## Proposta (não implementada)

1. **Corrida:** ao integrar, usar `speed_reference: Some(157.0)` no clipe de corrida (`N3_7_SINCRONIZACAO.md`). Não precisa de campo novo.
2. **Eventos visuais por quadro:** uma lista opcional, num arquivo ao lado, por exemplo `events.ron` com `"run6": [(frame: 0, event: FootPlant), (frame: 1, event: PushOff)]`. Serve para pó de passos, som de passos e o quadro de impacto de um ataque. Só entra no manifesto se a produção precisar e com autorização, porque os manifestos de produção não podem mudar nesta etapa.
3. **Transição:** `run_start` é um clipe `Once` com `fallback: Some("idle")`. Na produção, o estado de corrida tocaria `run_start` e depois o loop. Isso pede uma regra de "próximo clipe" além de `fallback`, que hoje volta para idle. Fica registrado como lacuna, sem mudança.
4. **Status:** `NOT APPROVED` / `APPROVED (owner, data)` só em `provenance.json`, nunca no manifesto.

Os manifestos de produção (`assets/animations/*.ron`) não foram tocados.
