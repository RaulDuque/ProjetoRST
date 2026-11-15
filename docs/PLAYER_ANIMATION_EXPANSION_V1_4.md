# Expansão das animações do protagonista (V1.4)

> **Atualização V1.5:** `hurt` e `fall` têm novas poses, existe
> `attack_basic_air` e há VFX (atlas `effects`). Os atlas agora só são
> publicados por `publish_guard.py`. A morte ajoelhada continua, e a deitada
> está em avaliação. Ver [`PLAYER_ART_POLISH_V1_5.md`](PLAYER_ART_POLISH_V1_5.md).

> Fase de apresentação. Nenhuma regra mudou: dano, hitboxes, física,
> velocidades, invulnerabilidade, custos, cooldowns, durações, progressão,
> save/load e câmera continuam idênticos. Base: o controlador da V1.3
> ([`PLAYER_VISUAL_ANIMATION_V1_3.md`](PLAYER_VISUAL_ANIMATION_V1_3.md)),
> ampliado, não substituído. Modelo: v007, sem mudança no rig.

## 1. Objetivo

Dar uma animação própria a cada estado que o jogador já tem: dano, morte,
pulo, queda, aterrissagem, agachar, guarda, as duas técnicas (Crescent Slash
e Rising Edge) e as duas magias (Ember Bolt e Mending Light).

## 2. Auditoria dos estados (evidência no código)

| Estado | Existe no gameplay? | Duração real | Prioridade (máquina de estados) | Antes (V1.3) | V1.4 |
|---|---|---|---|---|---|
| Idle | `PlayerState::Idle`, escolhido pela física a cada frame (`movement::locomotion`) | — | Locomoção (a mais baixa) | `idle` | — |
| Run | `Walking` (190 px/s) e `Running` (310 px/s, Shift) | — | Locomoção | `run` | — |
| Basic Attack | `Attacking` (`combat::attacks::basic_attack`) | `BASIC_ATTACK_TIME` 0,28 s ÷ `attack_speed` | Ação: trava a locomoção | `attack_basic` | — |
| Dodge | `Dodging` (`movement::guard_and_dodge`), invulnerável o tempo todo | `PLAYER_DODGE_TIME` 0,28 s | Ação; não pode ser ferido | `dodge` | — |
| Hurt | `Hurt` (`state.hurt`), interrompe tudo menos esquiva, guarda e morte | `PLAYER_HURT_TIME` 0,35 s (stagger) | Acima das ações | Fallback | **`hurt`** |
| Death | `Dead` (`state.die`), sem tempo; sai com o respawn (R) ou o load | Até o revive | A mais alta | Fallback | **`death`** |
| Jump | `Jumping`: no ar e subindo (vy > 0) | Física (pulo variável, `JUMP_CUT_FACTOR`) | Locomoção | Fallback | **`jump_rise`** |
| Fall | `Falling`: no ar e não subindo | Física | Locomoção | Fallback | **`fall`** |
| Land | **Não existe** como estado: a física passa de `Falling` a `Idle`, `Walking` ou `Running` | — | — | — | **`land`** (só apresentação, §6) |
| Crouch | `Crouching`: no chão com ↓, velocidade 0, sem tempo | Enquanto ↓ | Locomoção | Fallback | **`crouch`** |
| Guard | `Guarding` (segurar A, no chão), sem tempo; reduz 70 % do dano frontal (`GUARD_DAMAGE_REDUCTION`), sem parry | Enquanto A | Ação (trava locomoção) | Fallback | **`guard_start`** |
| Crescent Slash | `Casting` + `SkillStarted { skill_id: "crescent_slash" }`; avanço físico próprio (`lunge_speed` 260) | `execution_time` 0,40 s ÷ `attack_speed` | Ação | Fallback (`Casting`) | **`slash_crescent`** |
| Rising Edge | `Casting` + `"rising_edge"` (nível 3; também no ar) | 0,45 s ÷ `attack_speed` | Ação | Fallback | **`slash_rising`** |
| Ember Bolt | `Casting` + `"ember_bolt"`; o projétil nasce em `cast_spells` | 0,30 s ÷ `attack_speed` | Ação | Fallback | **`cast_forward`** |
| Mending Light | `Casting` + `"mending_light"`; a cura é instantânea, no início | 0,60 s ÷ `attack_speed` | Ação | Fallback | **`cast_up`** |

**Como a identidade da habilidade chega ao visual:**
- O estado só diz `Casting`.
- `begin_skill` emite `SkillStarted { caster, skill_id }` no mesmo frame em que trava o jogador.
- O visual **lê** essa mensagem (o leitor tem cursor próprio e não consome a
  mensagem de mais ninguém) e busca o campo `animation` da habilidade em
  `GameData`.
- O mapeamento é feito pelos dados: uma habilidade nova com `animation`
  existente no manifesto não exige código novo.

**Prioridade real**, verificada em `player/state.rs`:
- `Dead` > `Hurt` > ações (`Attacking`, `Casting`, `Dodging`, `Guarding` travam a locomoção) > locomoção.
- A locomoção é escolhida pela física: `Jumping`, `Falling`, `Crouching`, `Running`/`Walking`, `Idle`.
- `Hurt` não acontece em `Dodging`, `Guarding` ou `Dead`.
- Guarda e esquiva só começam no chão; atacar e conjurar Rising Edge são permitidos no ar.
- A ordem hipotética do briefing (guarda acima de pulo) não muda nada, porque a
  guarda termina ao sair do chão. O visual só traduz a máquina de estados.

## 3. Animações produzidas

| Clipe | Grupo / célula | Quadros | Temporização | Observação |
|---|---|---|---|---|
| `hurt` | locomoção 64×64 | 9 (8 × 1/24 s + 0,0167 s) | `FitToAction` (0,35 s) | O impacto (tronco jogado para trás) já no quadro 0, depois a recuperação |
| `death` | locomoção | 8 a 12 fps | `Once`, segura o último | Desequilíbrio, cai sobre um joelho e o tronco dobra para a frente |
| `jump_rise` | locomoção | 4 a 12 fps | `Once` | Impulso, depois pernas encolhidas |
| `fall` | locomoção | 4 a 12 fps | `Loop` | Pernas buscando o chão; capa e casaca sobem e tremulam |
| `land` | locomoção | 4 a 24 fps | `Once` (0,167 s) | Absorção do impacto |
| `crouch` | locomoção | 3 a 24 fps | `Once`, segura | Agachado, sabre baixo à frente |
| `guard_start` | locomoção | 3 a 24 fps | `Once`, segura | Sabre erguido em diagonal à frente, mão livre em guarda |
| `slash_crescent` | combate 128×96 | 10 (9 × 1/24 + 0,025 s) | `FitToAction` (0,40 s) | Arco amplo do alto à frente até embaixo, em avanço |
| `slash_rising` | combate | 11 (10 × 1/24 + 0,0333 s) | `FitToAction` (0,45 s) | Corte vertical subindo até acima da cabeça, sem pular |
| `cast_forward` | combate | 8 (7 × 1/24 + 0,0083 s) | `FitToAction` (0,30 s) | Mão livre estendida já no quadro 0 (o projétil nasce no início) |
| `cast_up` | combate | 15 (14 × 1/24 + 0,0167 s) | `FitToAction` (0,60 s) | Mão livre erguida, cabeça para cima, recuperação |

**Regras comuns:**
- **Impacto primeiro:** os efeitos (hitbox, projétil, cura) começam com a ação,
  então o quadro 0 já é o golpe ou o gesto.
- **Duração exata:** as ações duram o tempo do gameplay, com o último quadro mais curto.
- **Sem movimento de raiz:** quem desloca o personagem é o gameplay, inclusive o
  avanço do Crescent Slash e o recuo do dano.
- **Morte:** a primeira versão, deitada de bruços, foi descartada. Sem simulação
  de tecido, a casaca ficava em pé como uma tenda e o corpo flutuava. O colapso
  sobre um joelho mantém a roupa caindo e o corpo dentro da célula.

## 4. Durações

O build lê as durações do jogo, só para leitura: `PLAYER_HURT_TIME` em
`src/core/constants.rs` e o `execution_time` de cada RON. O build e o
`data_tests` falham se clipe e gameplay divergirem. Com `attack_speed`, o
controlador estica o mesmo clipe pelo tempo restante da ação (V1.3).

## 5. Prioridades e interrupções

O controlador segue o estado (`selection::select`):

| Situação | O que aparece |
|---|---|
| Ataque interrompido por dano | `hurt` no mesmo frame, a partir do quadro 0 |
| Morte durante uma habilidade | `death` no mesmo frame |
| Ataque no ar | `attack_basic` (o estado é `Attacking`; não há variante aérea, §11) |
| Rising Edge no ar | `slash_rising` |
| Esquiva durante a corrida | `dodge` |
| Fim de uma ação | volta ao clipe do estado que o gameplay escolher (idle, run, fall) |
| Revive (R) | Sai de `death`; o respawn põe o jogador no ar, então `fall` e depois `land` |
| Mudança de direção | Espelhamento imediato (V1.3) |
| Guarda e agachar soltos | Voltam a `idle` ou `run` |

Um clipe só recomeça quando muda o clipe, quando muda o estado ou quando o
tempo restante da ação sobe (uma ação nova). Nunca recomeça a cada frame.

## 6. Aterrissagem (`land`)

O gameplay não tem estado de aterrissagem; a física vai direto de `Falling` a
um estado no chão. Por isso o `land` é só apresentação:
- **Detecção:** o visual estava em `Falling` no frame anterior, o estado agora é
  `Idle`, o jogador ficou pelo menos 0,1 s em `Falling` e se moveu menos de 48 u
  desde o frame anterior. Por movimento, o máximo possível é 30 u (900 u/s × 1/30 s).
- **Por que o tempo mínimo de queda:** o teste novo com F9 durante a queda
  mostrou que, logo depois do load, o estado lê `Falling` por um frame, porque o
  contato com o chão só é atualizado no passo de física. Sem o tempo mínimo, isso
  virava um pouso falso.
- **Sem falso pouso:** um load (F9) ou respawn é teleporte e não conta como aterrissagem.
- **Não bloqueia:** dura 0,167 s e é cancelado por qualquer outro estado.
- **Andando:** ao aterrissar em movimento, vai direto para `run`.

Testado com F9 durante a queda, sem `land` (`tests/player_visual_tests.rs`).

## 7. Pipeline Blender

- **Script:** `art-src/player/scripts/bpy/build_game_sprites_v1_4.py` e os clipes
  em `game_sprites_v1_4/clips_v1_4.py`.
  - Reaproveita, sem alterar, o IK sagital, a amostragem Catmull-Rom e as poses
    da V1.2-G, além das verificações da V1.3.
- **Fonte:** `swordsman_rig_v007.blend`, aberto só para leitura; o rig não muda.
- **Saída:** `swordsman_rig_v007_game_v1_4.blend` (novo, com as ações `GAME_*`) e `renders/v1_4_game_sprites/`.
- **Nada sobrescrito:** v004 a v007, o arquivo de animações da V1.2-G e os da V1.3.
- **Validação de cada quadro:**
  - interseções e volume (escala do v005);
  - espada presa à mão;
  - cascas estáveis;
  - saltos entre quadros;
  - aterramento;
  - novo: **sabre acima do chão**, porque o aterramento ignora a lâmina.
- **Pós-processamento:** `art-src/player/scripts/post/game_sprites_v1_4.py`
  junta os masters da V1.3 e da V1.4.

## 8. Spritesheets

| Arquivo | Tamanho | Conteúdo |
|---|---|---|
| `assets/sprites/player/player_locomotion.png` | 1038×1038 (8×8 células de 128×128) | 64 quadros: idle 0–11, run 12–21, dodge 22–28 (índices da V1.3 mantidos), hurt 29–37, death 38–45, jump_rise 46–49, fall 50–53, land 54–57, crouch 58–60, guard_start 61–63 |
| `assets/sprites/player/player_combat.png` | 1804×1550 (7×8 células de 256×192) | 51 quadros: attack_basic 0–6, slash_crescent 7–16, slash_rising 17–27, cast_forward 28–35, cast_up 36–50 |

- **Formato:** PNG RGBA, sRGB, alfa reto, acabamento C, densidade 2, lâmina ×1,7.
- **Limites:** padding 2 entre células, ≤ 2048 px por lado.
- **Uma textura por grupo**, sem duplicar.
- **Contrato:** os clipes de locomoção cabem na célula 64×64 e os de habilidade
  na 128×96, sem mudar o contrato.
  - Na primeira versão, a lâmina do `crouch` e da guarda saía da célula, e o
    pós-processamento acusou. As poses foram ajustadas, não a célula.

## 9. Metadados

O mesmo `assets/animations/player.ron` da V1.3, com o modo `Once(fps)` já
previsto no contrato.

**Quadro final curto:** quando a sobra do último quadro tem menos de 25 ms
(1,5 frame a 60 Hz), os dois últimos quadros dividem o tempo igualmente, e a
duração total continua exata.
- Isso só afeta o `cast_forward` (0,30 s = 6 × 1/24 + 2 × 0,025 s).
- Antes, o último quadro durava 8 ms e nunca aparecia a 60 Hz; o teste de duração mostrou isso.
- A regra está em `game_sprites_v1_4.display_durations`. Não há formato novo. Os ids seguem o contrato §2: `hurt`,
`death`, `jump_rise`, `fall`, `land`, `crouch`, `guard_start`, e os
`animation` dos RON (`slash_crescent`, `slash_rising`, `cast_forward`,
`cast_up`). Todos têm `fallback: Some("idle")`.

## 10. Integração Bevy

Mudanças em `src/player/visual/`, todas de apresentação:

| Arquivo | Mudança |
|---|---|
| `playback.rs` | `once_frame`: toca uma vez e segura o último quadro |
| `selection.rs` | Constantes dos clipes novos; `SelectionContext` (animação da habilidade e aterrissagem); `Timing::Once`; tom usado só quando falta o clipe |
| `systems.rs` | Lê `SkillStarted` e o `animation` do `GameData`; detecta a aterrissagem com a guarda de teleporte; lê `Transform` do jogador; clipe ausente no manifesto → idle parado com tom |
| `mod.rs` | `VisualPlayback` ganha `skill_clip`, `landing_left`, `last_position` |
| `manifest.rs` | `resolve` devolve a chave do próprio manifesto (correção de tempo de vida) |

- O visual continua só lendo.
- Nenhum sistema novo e nenhuma ordem nova: `PlayerVisualSystems` continua
  depois do `quick_load`, em `Presentation`.

## 11. Testes

| Verificação | Antes (linha de base) | Depois |
|---|---|---|
| `cargo fmt --all -- --check` | OK | OK |
| `cargo clippy --all-targets -- -D warnings` | 0 avisos | 0 avisos |
| `cargo test` | 146 testes, 0 falhas | **158 testes, 0 falhas** |
| `cargo check` | OK | OK |
| `cargo run --example autoplay` | PASS 23/23 (V1.3) | PASS 23/23, com linhas PASS/INFO **idênticas** às da V1.3 (movimento, dano, magias, custos, save/load) |
| `cargo run --example visual_check` | PASS 13/13 | **PASS 30/30** |

Testes novos (12):
- **Unitários:**
  - `once_frame` (segura o último quadro);
  - seleção com todos os estados;
  - habilidade pelos dados;
  - aterrissagem só no idle;
  - prioridades: dano interrompendo ataque, morte durante habilidade, revive.
- **Integração** (`tests/player_visual_tests.rs`):
  - `hurt` acompanha o stagger;
  - ataque interrompido por dano;
  - morte toca uma vez, segura e o revive volta;
  - pulo, queda e aterrissagem seguem a física;
  - ataque no ar;
  - agachar e guarda seguram a pose;
  - cada habilidade com o seu clipe pela duração inteira;
  - morte durante habilidade;
  - F9 durante a queda não é aterrissagem.
- **Dados:** todo `animation` dos RON tem clipe com o `execution_time`, e `hurt` = `PLAYER_HURT_TIME`.

Teste adaptado: o caso de "estados sem sprite" da V1.3 deixou de existir, porque
agora todo estado tem clipe. O teste passou a remover a morte e a queda do
manifesto carregado, para exercitar o mesmo caminho de fallback. A asserção
não foi enfraquecida. O `visual_check` foi ampliado com agachar, guarda, pulo, queda,
aterrissagem, as quatro habilidades com teclas reais, o Crescent Slash virado
para a esquerda, e dano e morte forçados no estado (atalho de teste, nunca usado
pelo jogo).

## 12. Desempenho

| | V1.3 | V1.4 |
|---|---|---|
| Texturas | 2 | 2 |
| Quadros | 36 | 115 |
| Memória na GPU (RGBA) | 3,5 MB | 4,3 + 11,2 = 15,5 MB (~31 MB com a cópia em RAM que o Bevy mantém por padrão) |
| PNG em disco | 301 kB | 498 kB + 507 kB = 1,0 MB |
| Por frame | 2 sistemas | Os mesmos 2; mais a leitura das mensagens `SkillStarted` (normalmente zero) e uma comparação de posição |

O carregamento continua uma vez só, no Startup.

## 13. Limitações

- **Sem variantes:** `attack_basic_air` (o ataque no ar usa o clipe do chão), `turn`, `guard_impact`, `guard_end`, `jump_apex` e `respawn`.
- **Morte:** termina numa pose ajoelhada, não deitada (§3).
- **Espelhamento:** troca a mão da espada (V1.3).
- **VFX:** os arcos do Crescent Slash e do Rising Edge, a luz do Mending Light e
  o projétil do Ember Bolt continuam com o visual atual do jogo. A cura não tem
  evento visual próprio; isso fica para uma etapa de VFX.
- **Aterrissagem:** só de `Falling` para `Idle` (§6).
- **Avaliação artística:** as poses foram validadas por medidas e por inspeção
  visual, mas não passaram por aprovação artística.

## 14. Pendências e próximas etapas

- **Aprovação artística** das 11 animações.
- **VFX** das habilidades.
- **Variantes:** a aérea do ataque e as de transição da guarda.
- **Decisões em aberto:** D2, D3 e a taxa definitiva.
- **Git:** commits atômicos quando autorizados.
