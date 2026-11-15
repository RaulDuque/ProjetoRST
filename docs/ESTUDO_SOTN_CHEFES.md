# Estudo SOTN — chefes

Fase 2 da pesquisa. Convenções, licenças e regras de uso:
[`REFERENCIAS_SOTN.md`](REFERENCIAS_SOTN.md). Caminhos `src/...` são do
**sotn-decomp @ `3a0957b`**; os nossos, do **DCR @ `9f7616d`**.

Legenda: **[F]** fato confirmado no código · **[I]** inferência · **[R]**
recomendação · **[?]** pendente. Valores em quadros (60 Hz) e px.

> Os nomes dos chefes do SOTN aparecem aqui **só para localizar o código**.
> Nada de nomes, aparência, ataques específicos ou padrões exatos deve ir
> para o Dark Castle RPG. O que interessa são os **princípios**.

---

## 0. Resumo

1. Cada chefe é uma **entidade comum** (mesmo `Entity`, mesma colisão de
   inimigos, mesmo `HitDetection`) com uma máquina de estados própria. Não
   existe um "motor de chefes" separado. **[F]**
2. Uma entidade **gerente da luta** (`EntityBossFightManager`) coordena
   início, música, derrota, recompensa e encerramento por uma variável de
   bits compartilhada (`g_BossMgrState`). **[F]**
3. Padrões vistos: escolha de ataque por **distância com histerese**, por
   **posição na arena**, por **tabela cíclica**, por **janela aleatória
   deslocada pela distância** e por **alternância**. A troca de fase pode
   ser só **trocar de tabela** abaixo de um limiar de HP. **[F]**
4. Chefes têm **armadura**: só reagem a golpes em estados escolhidos. **[F/I]**
5. Chefes grandes são **várias entidades ligadas** (`nextPart`). Reacerto,
   atordoamento e morte se propagam por todas as partes. **[F]**
6. A derrota é **persistente**: um registro por chefe no save impede a luta
   de recomeçar e muda a sala que o mapa carrega. **[F]**
7. O DCR já tem quase toda a base (inimigos por dados, `DamageDealt`,
   `defeated_bosses` no save, barreiras de porta, baú `BossReward`). Falta a
   camada de chefe. Proposta no §6, sem módulo paralelo de inimigos.

---

## 1. Infraestrutura comum dos chefes

### 1.1 Gerente da luta **[F]**

`src/boss/rbo3/unk_12B38.c:13-84` (`EntityBossFightManager`), estados:

| Passo | Condição de saída | Efeito |
|---|---|---|
| 1 | Jogador entra na faixa da arena (`128 < x < 384`) | `g_BossMgrState = FIGHT_ON`, marca visita, para a música |
| 2 | Música parou | Toca o tema da luta |
| 3 | `g_BossMgrState & DEFEAT` | Grava o registro de derrota, troca a música |
| 4 | `g_BossMgrState & TERMINATED` | — |
| 5 | — | Cria a **recompensa** (`LIFE_UP_SPAWN`, `:66-69`) no centro da arena |
| 6 | Música parou | Toca o tema pós-luta |

Bits (`src/boss/rbo4/rbo4.h:63-66`): `IDLE = 0`, `FIGHT_ON = 1`,
`DEFEAT = 2` ("assim que o chefe é derrotado"), `TERMINATED = 4` ("a
entidade foi destruída"). Alguns chefes têm bits extras para encadear a
morte: `FLAMES_1_START/END`, `FLAMES_2_START/END` (Scylla, `e_scylla.c:366-449`).

**Princípio [I]:** o chefe só sinaliza (`DEFEAT`, `TERMINATED`); quem cuida
de música, recompensa e portas é o gerente. As partes do chefe e os
projéteis também leem os bits (ex.: projéteis somem em `DEFEAT`,
`e_scylla.c:758, 855, 971`).

### 1.2 Portas da arena **[F]**

`src/boss/bo0/e_boss_doors.c:28-161`: quando o jogador passa de um ponto
(`x < 0x1E8`), as portas **trocam os tiles de frente** (que têm colisão) e
deslizam a 0,5 px/quadro até fechar (`:63-116`). Ficam esperando uma
variável (`D_us_80181190`, `:117-120`); depois **restauram os tiles** e
deslizam para fora (`:121-155`). Se a variável já está ligada ao entrar, a
porta nem é criada (`:36-39`). **[I]** A variável indica "luta encerrada".
O nome dela ainda não foi identificado pela comunidade.

### 1.3 Trava de câmera **[F]**

`src/st/e_lock_camera.h` (incluída por `src/boss/bo0/e_lock_camera.c`):
uma zona invisível (`hitboxWidth/Height`). Ao atravessá-la, os limites de
rolagem da câmera mudam conforme o lado de saída (`:47-60`), o que prende a
tela na arena. A configuração vem de tabelas por parâmetro.

### 1.4 Persistência e "teleporte" para a arena **[F]**

- `TimeAttackController` (`src/dra/timeattack.c:4-27`) guarda **um registro
  por chefe** em `g_Settings.timeAttackRecords[evento]`: o tempo de jogo na
  primeira derrota. O comentário do decomp diz que o mesmo valor decide se
  o jogador deve ser levado para a sala do chefe.
- `FindBossTeleport` (`src/dra/5087C.c:220-240`) percorre a tabela
  `g_RoomBossTeleports` (`:186-218`: coordenadas de mapa, estágio, evento).
  Se o jogador entra numa célula listada e o chefe **não** tem registro, o
  jogo carrega a **versão-arena** da sala. Depois da derrota, carrega a sala
  normal.
- Gerentes de luta conferem o registro ao nascer e se destroem se o chefe
  já foi vencido (`src/boss/bo3/unk_26370.c:13-18`,
  `src/st/nz1/bossfight.c:18-23`).
- Alguns chefes também gravam uma flag do castelo
  (`g_CastleFlags[SCYLLA_DEFEATED] = 1`, `e_scylla.c:352`), usada por
  eventos do mundo.

### 1.5 Recompensa **[F]**

A Medusa deixa um item de aumento de HP máximo (`LIFE_UP_SPAWN`,
`rbo3/unk_12B38.c:59-69`). Cada estágio de chefe tem `e_life_up.c` e
`d_prize_drops.c`. **[?]** A tabela completa de recompensas por chefe não
foi levantada.

---

## 2. Chefes estudados

### 2.1 Medusa — `src/boss/rbo3/e_medusa.c` (decompilado por completo)

Chefe compacto e completo. Melhor exemplo de estrutura mínima.

| Passo | Comportamento | Linhas |
|---|---|---|
| 0 | Inicia **sem hitbox** (`hitboxState = 0`), cria uma entidade de **hitbox de arma** que acompanha o corpo | `:55-62` |
| 1 | Espera `FIGHT_ON` | `:64-68` |
| 2 | Animação de entrada, **invulnerável**; ao terminar, hitbox ligada (`= 3`) | `:70-76` |
| 3 | **Neutro**: anda 0,5 px/quadro; histerese de distância (≤104 recua, >128 aproxima); vira se o jogador está a mais de 32 px; temporizador de 64 quadros | `:78-126` |
| 3 → 6 | Se foi atingido (`hitFlags & 3`) → **reação de dor** | `:100-102` |
| 3 → 5 | **Contra-ataque**: jogador vindo na direção dele a menos de 80 px com um certo estado (`PLAYER_STATUS_UNK2000`) | `:104-110` |
| 3 → 4/5 | Fim do temporizador: longe (>64) → **projétil**; perto → **golpe corpo a corpo** | `:112-125` |
| 4 | Projétil sai numa pose exata (`pose == 4`); se o jogador ficou **petrificado**, encadeia o passo 8 | `:143-171` |
| 8 | **Castigo**: dois projéteis **teleguiados** com curva limitada (`LimitAngleChange`) por 64 quadros, depois seguem reto | `:172-197`, `:416-503` |
| 5 | Golpe: a hitbox da arma só fica **ofensiva** enquanto `flag = 1` | `:127-142`, `:394-398` |
| 6 | Dor → vira para o jogador e **responde com projétil** | `:198-212` |
| 7 | Morte: `DEFEAT`, sem hitbox, animação, 80 quadros de explosões, `TERMINATED` | `:213-253` |
| — | A posição é **presa à arena** (`128 ≤ x ≤ 384`) | `:260-269` |
| — | Hitbox do corpo **por quadro de animação** (tabela) | `:271-277` |

**[I]** `PLAYER_STATUS_UNK2000` tem nome ainda não identificado. Pelo
contexto (jogador vindo na direção do chefe), pode ser "atacando" ou
"correndo". Não foi confirmado.

### 2.2 Olrox (forma normal) — `src/boss/bo0/e_normal_form.c:182-818`

Decisão no passo 3 (`:259-292`), na ordem:

1. **Encurralado** (perto da parede e jogador perto) → **teleporte** (passo 13).
2. Jogador **muito perto** (<36 px) → golpe próximo (passo 5).
3. Perto da borda da arena e jogador a menos de 112 px → passo 4.
4. <64 px ou >176 px → passo 6 (com variante).
5. Senão: próximo ataque de uma **tabela cíclica de 8 entradas**
   (`D_us_80180BAC`, `:40-42`). Se o HP está **abaixo de 3/4** do máximo, usa
   a **outra metade** da tabela (`unk88 |= 8`, `:284-290`).

Outros fatos:
- O teleporte (`:293-341`) desliga a hitbox enquanto some e volta num ponto
  aleatório **do outro lado do jogador**, preso aos limites da sala. A hitbox
  só religa numa pose avançada da reaparição (`:336-338`).
- **Limite de projéteis** na tela: o portal só é criado se houver no máximo
  4 entidades nos slots de projétil (`:418-434`).
- **Lacuna:** a segunda forma (`e_true_form.c`) ainda não está decompilada
  (36 `INCLUDE_ASM`). Não foi possível estudar a troca de forma por dentro.

### 2.3 Scylla — `src/boss/bo3/e_scylla.c`, `e_scylla_wyrm.c`

- **Chefe de várias partes:** corpo (`EntityScylla`, `:163`), cabeça de
  lobo (`:489`), tentáculos, serpentes de fundo (`EntityWyrm`). As partes
  são ligadas por `nextPart`/`parent` (`:185-191`).
- Início por **proximidade ou dano**: menos de 176 px **ou** ser atingido
  (`:199`).
- Neutro de 80 quadros; ataques **alternados** por um bit `flip`
  (`:237-243`): duas famílias de ataque, sempre uma depois da outra.
- **Telegráfico:** animação de preparação, depois projéteis em posições
  semi-aleatórias, depois animação de recuperação (`:258-342`).
- A cabeça tem **ritmo próprio** (96 quadros, investida aleatória 1/128 por
  quadro, `:514-543`) e um ataque de área: pedras caindo do teto **com
  tremor de câmera** (`ShakeCamera(1)`, `:645-659`).
- Morte coreografada pelo gerente (`FLAMES_1`/`FLAMES_2`); as partes
  encolhem e somem em sincronia (`:662-690`).

### 2.4 Hipogrifo — `src/boss/bo5/e_hippogryph.c`

- **Seleção por janela deslocada pela distância** (`:469-480`):
  `índice = rand 0..7`, +4 se a distância for >80, +4 se for >128. O índice
  lê uma tabela de 16 entradas (`attack_steps`, `:170-174`) **ordenada** de
  ataques de curta para de longa distância. Resultado: aleatório, mas
  coerente com o espaçamento.
- **Armadura por estado:** só reage a golpe (`hitParams`) se o número do
  passo for **ímpar** (`self->step & 1`, `:266-274`). Os passos ímpares
  são parado, preparando e correndo (`:147-168`). **[I]** Durante voo,
  investida e sopro ele **não recua**: é a armadura por estado.
- A reação lança o chefe para trás (vx 1,5, vy −4, `:267-273`).
- **Parte extra** (`EntityHippogryphHitbox`, `:1374-1395`): sem ataque,
  sem atordoamento (`FLAG_SUPPRESS_STUN`), `hitPoints = 0x7FFE` (não perde
  HP), ligada ao corpo por `nextPart`, com caixa por quadro do corpo.
  **[I]** Serve para dar ao chefe uma área de colisão maior/extra sem
  duplicar o HP.

### 2.5 Não investigados ou incompletos

- Minotauro e Lobisomem (`src/boss/bo2/`): só os auxiliares estão
  decompilados (hitbox de ataque, bola de fogo). A IA principal ainda tem
  `INCLUDE_ASM` (`unk_224DC.c`, `unk_26EF8.c`, `unk_337D0.c`). **[?]**
- Granfaloon, Doppelganger, Richter, Death e outros: não lidos.
- Forma final de Olrox: não decompilada.

---

## 3. Regras de colisão específicas de chefes **[F]**

Em `src/st/collision.h`:

- `hitPoints == 0x7FFF` → a parte **não recebe dano** (`:454`); golpes
  ainda contam como contato. **Parte imune.**
- `hitPoints == 0x7FFE` → recebe o golpe, **não perde HP** (`:525`).
- Ao acertar uma parte viva, a proteção de reacerto por atacante
  (`unk6D[slot]`), o atordoamento e o flash de dano são copiados para **todas
  as partes do anel `nextPart`** (`:551-570`). Partes depois da primeira
  ganham +1 quadro. **Um golpe não acerta duas partes do mesmo chefe ao
  mesmo tempo.**
- Ao morrer, **todas as partes** do anel são marcadas como mortas
  (`:651-662`).
- O dano em si vai para o HP **da parte atingida** (`:533`). **[I]** Um
  chefe com HP único usa partes 0x7FFE/0x7FFF e o HP real fica no corpo.

---

## 4. Princípios reutilizáveis (para chefes originais)

| # | Princípio | Por que funciona |
|---|---|---|
| 1 | Entrada **invulnerável** com animação | Dá leitura ao jogador e evita dano "de graça" na porta |
| 2 | **Neutro curto** entre ataques (≈1–1,5 s), com reposicionamento | Janela de punição clara |
| 3 | Seleção por **distância** (histerese ou janela deslocada) | O chefe reage ao espaçamento do jogador sem parecer roteirizado |
| 4 | **Tabela cíclica**; fase = **outra tabela** abaixo de um limiar de HP | Fases baratas, testáveis e legíveis em dados |
| 5 | **Anti-encurralamento** (teleporte ou salto ao ser preso na parede) | Evita que o jogador "trave" o chefe num canto |
| 6 | **Armadura por estado** (reação só em neutro/preparação) | Golpes do chefe não são cancelados à toa; ainda há janelas de atordoamento |
| 7 | **Contra-ataque** quando o jogador avança de forma previsível | Pune insistência sem aumentar o dano |
| 8 | **Castigo** encadeado após o jogador sofrer um status | Cria tensão e ensina a evitar o status |
| 9 | Projéteis com **limite de instâncias** e **curva limitada** | Controla a dificuldade e o custo |
| 10 | **Várias partes** com reacerto e morte compartilhados | Chefes grandes sem golpes multiplicados |
| 11 | **Gerente da luta** separado do chefe | Música, portas, câmera, recompensa e persistência ficam fora da IA |
| 12 | **Derrota persistente** e arena que "vira sala normal" depois | Backtracking limpo |
| 13 | Morte **coreografada** (sem hitbox, efeitos, só depois a recompensa) | Clímax legível; nenhum dano depois da vitória |
| 14 | **Tremor de câmera** e efeitos de arena nos ataques grandes | Peso, sem mexer na regra |

---

## 5. O que o DCR tem hoje (verificado)

| Peça | Onde | Situação |
|---|---|---|
| Inimigo por dados | `src/enemies/definitions.rs`, `assets/data/enemies/castle_undead.ron` | Um arquétipo (`bone_sentry`) |
| IA pura | `src/enemies/ai.rs::decide` (`AiProfile`: aggro, leash, ataque, fuga por HP) | Sem padrões, fases nem seleção de ataque |
| Execução | `src/enemies/behavior.rs` (`ATTACK_WINDUP 0,35`, `ATTACK_ACTIVE 0,15`, `ENEMY_HURT_FLASH`, `ENEMY_KNOCKBACK` constantes) | Um ataque corpo a corpo por inimigo; todo dano interrompe |
| Dano e morte | `receive_damage` → `EnemyDefeated` (EXP, drops) → respawn | Reutilizável |
| Respawn | `spawning.rs::enter_new_room` recria **todos** os inimigos da sala ao entrar; `process_respawns` reaparece por tempo dentro da sala | Chefe precisaria ser excluído dos dois |
| Persistência | `WorldProgress::defeated_bosses` (`src/world/progress.rs:14`), já no save | **Ninguém escreve** (contrato vazio, `WORLD_ARCHITECTURE.md:35`) |
| Bloqueio de porta | `DoorBarrier` (`src/world/exploration_systems.rs:42`), `Requirement::Flag` (`src/world/graph.rs:76-84`) | Base para trancar a arena |
| Recompensa | `ChestTier::BossReward` (`src/world/graph.rs:293-294`) | Planejado; já aceito nos dados |
| Câmera | Presa aos limites da sala (`src/world/camera_rules.rs:173`, `RoomDefinition.size`) | Arena = sala; não há zona de trava |
| Roadmap | `ROADMAP.md` etapa 10; `ENEMY_SYSTEM.md` §"Chefes (planejado)" | Pendente |

**Classificação:** chefes **Ausente**. A infraestrutura de suporte é
**Parcial** (persistência, barreiras e baú existem, sem uso).

---

## 6. Proposta de arquitetura de chefes para o DCR **[R]**

Objetivo: **um chefe é um inimigo** com uma camada extra. Nada de segundo
sistema de dano, de HP ou de IA de inimigo comum.

### 6.1 Dados

`assets/data/bosses/*.ron` (novo), referenciando um `EnemyDefinition`
existente para HP, ATK, DEF, resistências, tamanho e EXP:

```ron
(
    id: "exemplo_guardiao",          // nome original nosso
    enemy: "exemplo_guardiao_corpo", // EnemyDefinition (stats, hurtbox)
    arena_room: "sala_da_arena",
    intro_time: 1.2,                 // invulnerável
    neutral_time: (0.9, 1.3),        // intervalo entre ataques
    poise_states: [Neutral, Windup], // onde reage a golpes
    stagger_threshold: 0.08,         // fração do HP máx por golpe
    attacks: {
        "investida": (windup: 0.5, active: 0.35, recovery: 0.6,
                      range: (60.0, 300.0), hitbox: (60.0, 40.0),
                      motion: Dash(speed: 420.0)),
        "rajada":    (windup: 0.6, active: 0.2, recovery: 0.8,
                      range: (120.0, 600.0), projectile: "orbe_lento",
                      max_active: 3),
    },
    phases: [
        (below_hp: 1.0,  selection: Cycle(["investida", "rajada", "investida"])),
        (below_hp: 0.5,  selection: DistanceWindow(near: ["investida"], far: ["rajada"])),
    ],
    corner_escape: Some(Leap),
    reward: (chest: Some("bau_da_arena"), flag: Some("guardiao_vencido"), item: None),
)
```

Todos os números acima são ilustrativos. Nenhum vem do SOTN.

### 6.2 Regras puras (sem Bevy), em `src/enemies/boss_rules.rs`

- `current_phase(phases, hp_ratio) -> usize`.
- `select_attack(phase, distance, cursor, roll) -> AttackId`: ciclo ou
  janela por distância.
- `BossTimeline::{Intro, Neutral, Windup, Active, Recovery, Staggered, Dying}`
  com `tick(dt)` e transições.
- `reacts_to_hit(state, poise_states, damage, max_hp, threshold) -> bool`.
- `is_cornered(position, arena, distance) -> bool`.

### 6.3 Sistemas Bevy, em `src/enemies/boss.rs` (ou `boss_systems.rs`)

- Componente `Boss { definition, phase, timeline, cursor }` **somado** ao
  `Enemy` existente. `think_and_move` ignora entidades com `Boss` (filtro
  `Without<Boss>`), e um sistema próprio as dirige.
- `BossFight` (recurso ou entidade gerente): `Idle → Engaged → Defeated →
  Rewarded`. Fecha as portas da arena (`DoorBarrier` por flag temporária),
  aciona a barra de HP do chefe, escreve `defeated_bosses` e a flag, libera o
  baú `BossReward`.
- `receive_damage` atual continua aplicando o dano. Com `Boss`, o recuo e a
  interrupção passam por `reacts_to_hit`.
- Chefe de várias partes (opcional): `BossPart { root }` com `Hurtbox`. O
  dano é redirecionado para o `EnemyHealth` da raiz. O reacerto é controlado
  por raiz, ligado ao `rehit_interval` do plano V2.4.
- Exclusão do respawn: `enter_new_room` e `process_respawns` não criam
  chefes vencidos (`defeated_bosses.contains`).

### 6.4 Impactos

| Área | Impacto |
|---|---|
| Save V3 | **Nenhuma mudança de formato**: `defeated_bosses` e `flags` já existem. Só passam a ser escritos |
| Dados | Novo diretório `assets/data/bosses/`; validação de referências (inimigo, sala, baú) no validador do mundo |
| Animações | Fases de tempo (`windup/active/recovery`) dirigem a arte, como no plano V2.4. Arte provisória (cores) até haver sprites |
| Controles | Nenhum |
| UI | Barra de HP do chefe no HUD (pequena, separada do HUD do jogador) |
| Câmera | Arena = sala; opcional: `shake` em ataques grandes (constante de "sensação") |

### 6.5 Testes

- Puros: fase muda exatamente no limiar; ciclo dá a volta; janela por
  distância nunca escolhe ataque fora do alcance; armadura impede o recuo
  fora dos estados listados; intro invulnerável.
- ECS headless: entrar na arena fecha as portas; vencer escreve
  `defeated_bosses` e reabre; reentrar não recria o chefe; o baú só
  aparece depois da morte; nenhum dano do chefe depois de `Dying`.
- Save: salvar e carregar depois da vitória mantém a arena vazia.
- Validação de dados: chefe referencia inimigo, sala e baú existentes.

### 6.6 Riscos

- **Duplicar a IA de inimigos:** evitar com `Boss` como componente extra e
  regras puras separadas, reaproveitando `decide` só onde fizer sentido.
- **Escopo:** começar com **um** chefe, uma fase e dois ataques. Fases e
  partes vêm depois.
- **Dependência do V2.4:** armadura, linhas do tempo e reacerto dependem
  das peças do plano de sensação de combate. Fazer os chefes **depois** do
  V2.4 evita retrabalho.
