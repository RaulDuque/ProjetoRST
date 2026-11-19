# N3.8 — Transições de estado (modo experimental)

Cada transição foi gravada no jogo real (`cargo run --example n3_8_production_run`, câmera 405, updates de 20 ms) e conferida no jogo headless (`tests/n3_8_production_run_tests.rs`).

| Transição | O que o jogo mostra | Evidência | Situação |
|---|---|---|---|
| Idle → corrida | Corte direto da pose N3.5 para `run` #0. O ciclo começa do quadro 0. | `n38_idle_to_run.gif` | Funciona. **Abrupta**: o cabelo passa de ~207 para ~318 texels de uma vez |
| Corrida → idle | Corte direto do quadro em curso para a pose N3.5 | `n38_run_to_idle.gif` | Funciona. **Abrupta**: não há quadros de frenagem |
| Corrida → mudança de direção | Espelha no mesmo update; o ciclo continua sem reiniciar | `n38_run_turn.gif` | Funciona |
| Corrida → salto | Pose N3.5 parada (subida e queda), selo `PLACEHOLDER: "jump_rise"` / `"fall"` | `n38_run_to_jump.gif` | Representação temporária |
| Corrida → ataque | Pose N3.5, selo `"attack_basic"`. O retângulo cinza do golpe é o mesmo da produção | `n38_run_to_attack.gif` | Representação temporária |
| Corrida → guarda | Pose N3.5 com o tom de guarda, selo `"guard_start"` | `n38_run_to_guard.gif` | Representação temporária |
| Corrida → esquiva | Pose N3.5 deslizando (480 u/s da física), selo `"dodge"` | `n38_run_to_dodge.gif` | Representação temporária |
| Corrida → dano | Pose N3.5 com o tom de dano, selo `"hurt"` | teste `every_state_resolves_to_an_approved_clip_with_a_visible_marker` | Representação temporária. **Sem captura**: o roteiro não leva dano |
| Corrida → morte | Pose N3.5 **de pé** com o tom de morte, selo `"death"` | mesmo teste | Representação temporária. **Sem captura.** Legibilidade ruim (ver abaixo) |

## Como a representação temporária funciona

Não há código novo de seleção. O manifesto experimental só tem `idle` e `run`. Para qualquer outro estado, `AnimationManifest::resolve` cai no `idle`, e o sistema mostra o quadro parado (`Timing::Hold`) com o tom que a seleção já define para clipe ausente (guarda, magia, dano, morte). A N3.8 só acrescentou `VisualPlayback.placeholder_for`, que guarda o nome do clipe que faltou, e o selo de UI que o mostra. Nenhum quadro V2.2 é usado: o teste verifica a imagem do atlas em salto, ataque, esquiva e guarda.

## Gameplay intacto

`gameplay_is_identical_with_the_production_and_the_experimental_art` roda o mesmo roteiro (correr, saltar, atacar, virar, esquivar, parar) com a arte de produção e com a experimental e compara posição, velocidade e estado em todos os 220 updates: **idênticos**. A animação não bloqueia nenhum comando: duração de ação, hitbox e invulnerabilidade continuam vindo do gameplay.

## Problemas documentados (não escondidos)

1. **Partida e parada são cortes.** A N3.7 extraiu a partida real (`run_start`, vídeo 3–13), mas ela **não foi usada**: tocar `run_start` e depois o loop exige uma regra de "próximo clipe" que o contrato não tem (`N3_7_CONTRATO_ANIMACAO.md`, lacuna 3), e um clipe de 0,5 s antes do loop atrasaria a leitura da corrida. Fica para a N4, com a regra de sequência no controlador único.
2. **Salto, ataque, esquiva e guarda parados** são legíveis como "ação acontecendo" só pelo deslocamento, pelo tom e pelo selo. Servem para testar o jogo, não para avaliar arte.
3. **Morte de pé** é o caso pior: o personagem morto continua em pé, só tingido. Para a prova experimental é aceitável. Para qualquer promoção, é bloqueante.
4. **Abaixar** também mostra a pose de pé, mas a colisão abaixa normalmente.
