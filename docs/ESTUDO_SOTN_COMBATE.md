# Estudo SOTN — combate, movimentação, magias e inimigos

Referências, licenças e convenções: [`REFERENCIAS_SOTN.md`](REFERENCIAS_SOTN.md).
Todos os caminhos `src/...` sem prefixo nesta seção de evidência são do
**sotn-decomp @ `3a0957b`**; os nossos estão marcados como **DCR** (Dark
Castle RPG @ `9f7616d`). Valores do SOTN convertidos a 60 Hz.

Legenda: **[F]** fato confirmado no código · **[I]** inferência ·
**[R]** recomendação para o DCR · **[?]** questão pendente.

---

## 0. Resumo em dez linhas

1. No SOTN o **ataque não é um estado**: é um sub-estado (`step_s` 0x40–0x5D) de "em pé", "agachado" ou "no ar". O mesmo botão gera golpes diferentes conforme o estado. **[F]**
2. Cada arma tem um **ponto de liberação** (`lockDuration`, índice de pose): antes dele o personagem está preso; depois, pode pular, agachar, dar backdash ou atacar de novo, mesmo com a animação ainda tocando. **[F]**
3. Todas as transições de controle passam por **uma função com máscara de bits** de "o que é permitido agora" (`func_8010FDF8`). **[F]**
4. Hitbox e hurtbox vêm **do quadro de animação** (tabela por quadro), não de constantes. **[F]**
5. O recuo ao ser atingido tem **três intensidades**, escolhidas pelo dano relativo ao HP máximo, e muda conforme o jogador está em pé, agachado ou no ar. **[F]**
6. A invencibilidade pós-dano dura **o recuo inteiro + ~12 quadros**; não é um valor fixo longo. **[F/I]**
7. No inimigo, cada arma define **hitstun** (`stunFrames`) e **intervalo para acertar de novo o mesmo alvo** (i-frames por atacante). **[F]**
8. O DCR já tem: máquina de estados pura, hitbox/hurtbox, guarda frontal, esquiva invulnerável, interrupção de golpes, perfil de golpe por classe de arma, reconhecedor de comandos mais flexível que o do SOTN.
9. Lacunas de maior impacto no DCR: golpe ativo desde o quadro 0 (sem *startup*), sem janela de cancelamento por arma, recuo único para todo dano, sem controle aéreo durante o ataque, hurtbox não muda ao agachar, hitstun do inimigo constante.
10. Plano priorizado em [`PLANO_MELHORIAS_SOTN.md`](PLANO_MELHORIAS_SOTN.md).

---

## 1. Combate (os 20 itens pedidos)

### 1.1 Máquina de estados do protagonista

- **SOTN [F]:** `PLAYER.step` (enum `PlayerSteps`, `include/player_steps.h:1-48`: Stand, Walk, Crouch, Fall, Jump, Hit, Kill, HighJump, Slide, formas, magias...) + `PLAYER.step_s` (sub-estado). Despacho em `EntityAlucard` (`src/dra/692E8.c:618+`, `case Player_Hit` em `:1121`). Estados "de ação" como magia (`Player_SpellHellfire`...) e recuo (`Player_Hit`) são steps próprios; **ataque e subarma não são** (ver 1.2).
- **DCR:** `src/player/state.rs` — `PlayerState` com 6 estados de locomoção livres e 6 de ação temporizados (`Attacking`, `Casting`, `Guarding`, `Dodging`, `Hurt`, `Dead`); `is_locked`, `start_action`, `hurt`. Pura e testada.
- **Diferença:** no DCR, `Attacking` substitui a locomoção; no SOTN o ataque é sobreposto à locomoção (em pé/agachado/no ar).
- **[R]** Não reescrever a máquina. Acrescentar ao `PlayerStateMachine` uma noção de **fase de ataque** (`Startup/Active/Recovery`) e um **ponto de cancelamento** (ver 1.5), e uma **postura** (`Stance::{Standing, Crouching, Airborne}`) registrada no início do golpe para escolher variante. Mantém o enum atual e os testes existentes.
- **Testes:** unidade em `state.rs` (fases avançam com `tick`; cancelamento só após o ponto); propriedade "morto nunca age" preservada.

### 1.2 Ataques normais

- **SOTN [F]:** `func_8010EDB8` (`src/dra/6E42C.c:372-841`) inicia o golpe da mão pressionada (Quadrado = mão 0, Círculo = mão 1). Lê do item: `playerAnim`, `lockDuration`, `unk11` (tipo de golpe → `step_s = 0x40 + tipo`), `chainLimit`, `mpUsage`. No chão, **corta a velocidade horizontal pela metade** (`velocityX >>= 1`, `:696`) e a frenagem de "em pé" (1/8 px/quadro², `71830.c:261`) faz o resto. A arma é uma **entidade separada** (`E_WEAPON`) criada por fábrica, com hitbox própria.
- **DCR:** `combat/attacks.rs::basic_attack` + `inventory/weapons.rs::StrikeProfile` (alcance, altura, recuo do corpo, tempo, recuperação, potência, elemento por classe em `assets/data/items/weapon_classes.ron`). Hitbox presa ao dono (`AttachedToOwner`). Frenagem `ACTION_FRICTION = 10/s` em `player/movement.rs:22`.
- **Diferença:** equivalente em essência. O DCR não tem variante por postura nem ataque com a "outra mão".
- **[R]** Manter. Ver 1.4, 1.5, 1.14, 1.15.

### 1.3 Ataques especiais (artes de arma)

- **SOTN [F]:** arma pode ter `specialMove` acionado por **↓↘→ + ataque** (`g_ButtonCombo[COMBO_QCF]`, `6E42C.c:464-480`), com custo de MP (`HasEnoughMp`) e só no chão; e um segundo golpe alternativo por **←→ + ataque** (`COMBO_BF`, campo `unk17`, `:485-494`). Se faltar MP ou o limite de instâncias for atingido, cai para o golpe normal (sem falha silenciosa).
- **DCR:** técnicas em dados (`assets/data/skills/sword_techniques.ron`), restritas por categoria de arma (`weapons`, `CommandOutcome::WrongWeapon`), custo MP/Hearts, cooldown, `usable_in`. Reconhecedor genérico (`combat/command_parser.rs`).
- **Diferença:** o DCR já é mais geral. Uma diferença de design útil: no SOTN o comando especial **pertence à arma**; no DCR pertence ao personagem e a arma filtra.
- **[R]** Opcional: campo `techniques: [...]` no item/classe para técnicas que só existem com aquela arma (sem mudar o reconhecedor). Prioridade baixa.

### 1.4 Recuperação após ataques

- **SOTN [F]:** depois do ponto `lockDuration` (`g_Player.unk54`), o bit de bloqueio `unk46 & 0x8000` é limpo (`71830.c:365-366, 407-409, 429-431`) e as flags locais passam a permitir andar/virar (`local_flags 0xB/0x1B/0xF`). Quando a animação termina (`poseTimer < 0`), volta a "em pé". A recuperação é **parte do mesmo golpe**, só que cancelável.
- **DCR:** `StrikeProfile.action_time = swing + recovery`, tudo travado até o fim (`state.rs::is_locked`). Hitbox vive `HITBOX_ACTIVE_FRACTION = 0.6` do swing **a partir do quadro 0**.
- **Diferença:** no DCR não existe *startup* (antecipação) — o golpe acerta no primeiro quadro — e a recuperação não pode ser cancelada.
- **[R]** Em `WeaponClass` (RON), campos opcionais com `#[serde(default)]` que reproduzem o comportamento atual: `startup` (padrão 0), `active` (padrão 60% do swing), `cancel_after` (padrão = fim da ação). `StrikeProfile` passa a expor as três janelas. Hitbox nasce em `startup` (componente `DelayedHitbox` ou spawn por sistema de fase).
- **Testes:** `weapons.rs` — perfil padrão idêntico ao atual (garante que a Iron Sword não muda); fase em cada instante; ECS: nenhum acerto antes de `startup`.

### 1.5 Cancelamento de animações

- **SOTN [F]:** `func_8010FDF8(branchFlags)` (`src/dra/6FDF8.c`) é chamado no topo de cada estado com a máscara do que é permitido: `0x4` cair se sem chão, `0x8` virar, `0x10` pular, `0x20` pulo duplo, `0x1000` atacar, `0x2000` agachar, `0x40000` backdash, `0x8000/0x10000/0x200` perfis de gravidade. Pular/agachar/backdash ficam **bloqueados enquanto `unk46 & 0x8000`** (`:116`). Atacar é testado **antes** do bloqueio (`:111`) e o próprio `CheckChainLimit` decide: armas com `chainLimit & 0x80` só atacam de novo após a liberação (`6E42C.c:334-339`); outras permitem N instâncias simultâneas.
- **[I]** Consequência: depois de um backdash (`func_8010FD88` zera `unk46`, `6E42C.c:920`), pular ou atacar já é permitido no quadro seguinte — o "cancelamento de backdash" conhecido pelos jogadores sai dessa regra.
- **DCR:** cancelamento só de `Attacking` para técnica (`usable_in: [.., Attacking]`). Esquiva, pulo e novo ataque esperam o fim.
- **[R]** Introduzir uma função pura `allowed_actions(state, phase) -> ActionMask` (ou método em `PlayerStateMachine`) e usar `cancel_after` (1.4). Política inicial sugerida: após `cancel_after`, permitir `Jump`, `Dodge`, `Guard`, `Attack` e técnicas. **Esquiva não cancelável** nos primeiros N ms (para manter a invulnerabilidade como recurso, não como "sair de tudo").
- **Testes:** tabela de transições permitidas por fase; ECS: atacar → apertar pulo antes/depois do ponto.

### 1.6 Controle durante ataques

- **SOTN [F]:** no chão, o golpe herda metade da velocidade e freia; **sem controle horizontal**, mas pode **virar e converter em golpe agachado** nas primeiras poses (`71830.c:356-362, 396-404`: se `pose < 2..3` → `CheckMoveDirection`; `↓` muda para o anim agachado). No ar, **controle horizontal pleno** a 1,5 px/quadro durante o golpe (`71830.c:696-701`) e a gravidade continua.
- **DCR:** no chão, frenagem (`ACTION_FRICTION`); no ar, `locomotion` retorna cedo e **mantém a velocidade horizontal sem controle** (`movement.rs:118-123`); `update_facing` só vira em locomoção.
- **[R]** (a) Controle aéreo durante `Attacking` quando no ar (velocidade de andar, sem correr). (b) Janela curta de virar no início do golpe no chão (`turn_window`, padrão 0 = comportamento atual).
- **Testes:** ECS — atacar no ar segurando → move; atacar no chão segurando → não acelera.

### 1.7 Hitboxes

- **SOTN [F]:** cada entidade tem **uma** caixa (`hitboxOffX/Y`, `hitboxWidth/Height`) e o papel dela vem de `hitboxState` (bits: atinge o jogador, pode ser atingido, etc.). A caixa é **atualizada pelo quadro de animação** (`frameProps`, `src/dra/alu_anim.c:191-214`). Inimigos trocam a caixa no quadro ativo do golpe (esqueleto: quadro 12 vira 20×17 deslocada, `src/st/e_bone_scimitar.h:150-161`).
- **DCR:** `Hitbox` e `Hurtbox` separadas (`combat/hitboxes.rs`), tamanho fixo por golpe; `already_hit` impede reacerto (D-009, golpe atravessa vários alvos uma vez cada).
- **Diferença:** separar hitbox de hurtbox é melhor que o modelo do SOTN (mais claro). A lacuna é temporal (sem *startup*/forma por fase), não estrutural.
- **[R]** Não migrar para hitbox por quadro de arte. Manter a forma por dados e acrescentar (opcional) **várias caixas por fase** de golpe (`shape_by_phase`) só se as armas pesadas pedirem (arco do machado). Prioridade média-baixa.

### 1.8 Hurtboxes

- **SOTN [F]:** a do jogador também vem do quadro (`frameProps`), então muda ao agachar, no ar, etc. Fica ativa todo quadro vivo (`692E8.c:1401-1410`).
- **DCR:** `Hurtbox { size }` fixa = corpo inteiro; agachar só muda a arte (`player/visual/selection.rs:117`, `player/animation.rs:84`).
- **[R]** Hurtbox de agachado reduzida (altura e centro) em `constants.rs` (`PLAYER_CROUCH_HURTBOX`), aplicada por um sistema pequeno no módulo `player`. Isso dá função tática ao agachar (desviar de golpes altos), como no SOTN.
- **Testes:** ECS — golpe inimigo alto acerta em pé e erra agachado.

### 1.9 Frames ativos

- **SOTN [F]:** ativo = quadros em que a arma/inimigo tem caixa com `hitboxState` de ataque; para inimigos, normalmente só os quadros do golpe (exemplo do esqueleto). Os números exatos por arma estão nos dados de animação (não lidos).
- **DCR:** ativo = primeiros 60% do swing (`HITBOX_ACTIVE_FRACTION`); inimigo: `ATTACK_WINDUP 0,35 s` + `ATTACK_ACTIVE 0,15 s` (`enemies/behavior.rs:22-25`).
- **[R]** Ver 1.4 (startup/active/recovery por classe de arma) e §4, recomendação 4 (por inimigo, em RON).

### 1.10 Invulnerabilidade

- **SOTN [F]:** dois temporizadores (`ALU_T_INVINCIBLE`, `ALU_T_INVINCIBLE_CONSUMABLES`); se qualquer um > 0, o dano **nem é processado** (`692E8.c:887-907`). `func_8010E168(1, n)` garante pelo menos `n` quadros (`6DF70.c:79-91`). Durante `Player_Hit` o jogo renova 12 quadros **a cada quadro** (`692E8.c:1255-1262`) → invulnerável o recuo inteiro. **[I]** Ao sair do recuo, mais 12 quadros com pisca (`692E8.c:1391-1398`, transição de status). Dano bloqueado por DEF alta: 12 quadros (`692E8.c:931-947`). Magia Hellfire: 16 quadros ao começar (`6E42C.c:852`).
- **DCR:** `PLAYER_INVULNERABILITY = 0,9 s` fixo após dano (`constants.rs`), `PLAYER_HURT_TIME = 0,35 s`; esquiva invulnerável 0,28 s.
- **Diferença:** SOTN ≈ duração do recuo + 0,2 s (≈ 0,4–0,7 s conforme o recuo); DCR 0,9 s fixos, mais generoso.
- **[R]** Trocar o valor fixo por `hurt_time + PLAYER_INVULNERABILITY_TAIL` (constante) **só depois** de existir recuo por intensidade (1.11); senão o jogador perde proteção sem ganhar legibilidade. Decisão de balanceamento: registrar em `DECISIONS.md` quando for feita.

### 1.11 Recuo ao receber dano

- **SOTN [F]:** `HandleDamage` (`src/dra/5D5BC.c:593-759`) classifica a gravidade: `dano·2 ≥ HPmáx` → **pesado** (lança −8 px/quadro, colide com parede/teto e sofre **meio dano extra** no impacto, `71830.c:1282-1293, 1488-1560, 1146-1171`); `bruto·50 ≥ HPmáx` (≥ 2% do HP máximo) → **médio** (arco: vy −4, vx −5/3; no ar vy −3); senão **leve** (no chão: escorrega −2,5 px/quadro sem subir; agachado: anim própria; no ar: vy −2, vx −1,25). Direção pelo lado do atacante, com correção para inimigo em movimento muito perto (`func_80113F7C`, `71830.c:1195-1235`). Efeito visual e duração extra do "flash" por elemento (`:1385-1468`).
- **DCR:** um único recuo `PLAYER_HURT_KNOCKBACK = (220, 260)` e `PLAYER_HURT_TIME = 0,35 s` para qualquer dano (`player/systems.rs::receive_damage`, ~linha 170).
- **[R]** Função pura `hurt_reaction(damage, max_hp, stance) -> HurtReaction { tier, knockback, duration }` em `player/` (ou `combat/`), limiares e velocidades em `constants.rs`. Três tiers: leve (sem subir, curto), médio (arco atual), pesado (arco longo). **Não** copiar o impacto em parede com dano extra (opcional futuro).
- **Testes:** unidade — limiares nas bordas; agachado/no ar escolhem variantes; ECS — dano pequeno não tira o jogador do chão.

### 1.12 Knockback dos inimigos

- **SOTN [F]:** `HitDetection` aplica ao inimigo `stunFrames` da arma (`src/st/collision.h:560-562`, exceto `FLAG_SUPPRESS_STUN`) e marca `hitFlags` com intensidade 1/2/3 pelo dano em relação ao **HP restante** (`<2×` dano → 3; `<4×` → 2; senão 1; `:525-533`). O deslocamento em si é decidido por cada inimigo lendo esses campos (varia por inimigo). **[?]** valores por arma não lidos.
- **DCR:** `ENEMY_KNOCKBACK = 160` e `ENEMY_HURT_FLASH = 0,15 s` iguais para tudo; o flash também é o "stun" (`enemies/behavior.rs:76-79, 246-249`).
- **[R]** `stun` e `knockback` por classe de arma/técnica (dados) e `knockback_resistance`/`super_armor` por inimigo (dados, padrão = comportamento atual). Mensagem `DamageDealt` ganha `stun`/`knockback` opcionais, preenchidos no `Hitbox`.
- **Testes:** inimigo com resistência 1.0 não recua; arma pesada atordoa mais que adaga.

### 1.13 Combos

- **SOTN [F]:** não há "combo de três golpes" para Alucard. O ritmo vem de (a) cancelar a recuperação com novo ataque após `lockDuration`, (b) armas com várias instâncias (`chainLimit` numérico, ex.: armas de arremesso por mão), (c) duas mãos alternadas. Segurar o botão em certas poses **mantém o golpe estendido** (`71830.c:367-372, 378-380`).
- **DCR:** sem combos (pendente no `ROADMAP` etapa 3).
- **[R]** Implementar "combo" primeiro como **cancelamento de recuperação em novo ataque** (1.5): resolve 80% da sensação com quase nenhum sistema novo. Cadeia de golpes distintos (1-2-3) pode vir depois como `combo_chain` por classe de arma.

### 1.14 Ataques aéreos

- **SOTN [F]:** mesma arma, anim variante "no ar" (`animVariant 4`), variante "caindo" (`5`) e, para alguns tipos, "caindo + ↓" (`6`) (`6E42C.c:710-723`). Fim do golpe volta a pulo/queda (`71830.c:702-711`). Chute mergulhado (↓ + pulo no ar) com quique ao acertar (`71830.c:713-747`).
- **DCR:** `PlayerAction::Attack` permitido no ar (`state.rs:129`), mas o golpe é o mesmo do chão, sem controle (1.6).
- **[R]** Variantes de forma por postura no `WeaponClass` (`air: Option<StrikeShape>`, `crouch: Option<StrikeShape>`), padrão = forma atual. Ataque para baixo/chute: técnica separada, prioridade baixa.

### 1.15 Ataques agachados

- **SOTN [F]:** variante própria (`animVariant 2`, `+1` se segurando ←/→), e o golpe em pé pode virar agachado nas primeiras poses (1.6).
- **DCR:** golpe agachado usa a mesma altura do golpe em pé; hurtbox não muda (1.8).
- **[R]** Junto com 1.8 e 1.14: `crouch` em `WeaponClass` (golpe mais baixo) e hurtbox agachada.

### 1.16 Armas de diferentes categorias

- **SOTN [F]:** o comportamento vem de `unk11` (família de golpe: espada comum, estocada com avanço `SetSpeedX(4)>>1`, arremesso, punho, consumível, escudo, comida/remédio...) + `playerAnim` + `lockDuration` + `chainLimit` (`6E42C.c:563-839`). Golpe pode **falhar** com armas "amaldiçoadas" (chance 1/8 de anim de falha, `:571-597`) e quando o jogador está amaldiçoado (`:527-530`).
- **DCR:** `WeaponClass` por categoria (Sword, Dagger, Axe, Spear, HeavyWeapon) com forma/ritmo/potência; `two_handed` (V2.1).
- **[R]** Bom como está. Acrescentar por classe: `startup`, `cancel_after`, `stun`, `knockback`, `rehit_interval` (1.4, 1.12, 4.2) e **impulso de avanço** opcional (`lunge`, para lança/estocada).

### 1.17 Armas de duas mãos

- **SOTN [F]:** não há bloqueio de "duas mãos" explícito no código lido; armas grandes ocupam uma mão e a outra pode ter escudo — **[?]** confirmar em `src/weapon/w_051.c` (Claymore etc.) e na lógica de equipamento do menu.
- **DCR:** `two_handed: true` em Spear e HeavyWeapon impede escudo (V2.1).
- **[R]** Manter (é decisão de design nossa).

> **Atualização (Fase 2):** a regra de duas mãos foi **confirmada** no menu
> de equipamento (`src/dra/menu.c:3209-3216, 3301-3310`). Ver
> [`ESTUDO_SOTN_EQUIPAMENTOS.md`](ESTUDO_SOTN_EQUIPAMENTOS.md).

### 1.18 Escudos

- **SOTN [F]:** escudo é item de mão (categoria 128 em `func_8010EDB8`, `6E42C.c:764-788`): **segurar** o botão da mão mantém a pose de defesa (`padPressed`, não `padTapped`, `:398-405`), `g_Player.unk48 = 1` mantém "em pé com escudo" (`71830.c:312-325`). O escudo é uma **entidade própria** e cai quando o jogador entra em recuo (`src/weapon/w_008.c` ~`:91-115`). Botão de ataque + escudo juntos dispara a magia do escudo (combinação `D_8013AEE4`, `6E42C.c:419-436`). Item de defesa entra na DEF total (`menu.c:346-349`). **[?]** o bloqueio de golpes/projéteis pela caixa do escudo não foi confirmado.
- **DCR:** guarda (tecla A) bloqueia 70% do dano frontal sem stagger (`GUARD_DAMAGE_REDUCTION`), escudo equipado dá DEF/resistências.
- **[R]** Manter a guarda. Ideias opcionais: redução de guarda vinda do escudo equipado (dados) em vez de constante global; "quebra de guarda" se o golpe for pesado (tier 1.11).

> **Atualização (Fase 2):** bloqueio pelo escudo **parcialmente
> confirmado**: caixa própria do escudo, mantida enquanto o botão é segurado
> e que cai ao sofrer dano (`src/weapon/w_008.c:60-65, 91-113, 188-203`);
> não há redução de dano no caminho de dano do jogador. Ver
> [`ESTUDO_SOTN_EQUIPAMENTOS.md`](ESTUDO_SOTN_EQUIPAMENTOS.md).

### 1.19 Interrupções

- **SOTN [F]:** dano leva a `Player_Hit` de qualquer estado não invencível, destruindo a arma em uso (`func_80113EE0` → `func_8010FAF4` se aplicável; o escudo cai). Golpes em andamento do jogador terminam.
- **DCR:** `cancel_interrupted_hitboxes` remove hitboxes presas ao dono em `Hurt`/`Dead`; troca simultânea vale (D-014).
- **Diferença:** equivalente. **Implementado adequadamente.**

### 1.20 Prioridade entre ações

- **SOTN [F]:** ordem fixa por quadro em `EntityAlucard`: timers → entrada → (invencível? pula dano) → agarrão de chefe → **dano** → **comandos especiais** (`CheckSpecialMoveInputs`) → **transformações** → estado atual (`692E8.c:887-1075`). Dentro de `func_8010FDF8`: gravidade/pouso → cair → **ataque** → pulo → pulo duplo → agachar → backdash (`6FDF8.c:105-145`). Vários inimigos no mesmo quadro: o jogador recebe só o golpe de **maior ataque** (`collision.h:366-376`).
- **DCR:** `GameSet` encadeados (`Input → Commands → Actions → Physics → Combat → Resolution → ...`); dano recebido: **a primeira** mensagem que causa dano vale, as demais são ignoradas pela invulnerabilidade (`COMBAT_SYSTEM.md`).
- **[R]** Pequena mudança em `player::systems::receive_damage`: entre os acertos do mesmo quadro, aplicar **o de maior dano** (ordem estável). Evita que um golpe fraco "gaste" a invulnerabilidade antes de um forte no mesmo quadro.
- **Testes:** ECS — dois `DamageDealt` (5 e 30) no mesmo quadro → aplica 30.

---

## 2. Movimentação

Valores do SOTN em px/quadro (60 Hz). Os nossos em unidades de mundo/s
(`src/core/constants.rs`). As escalas diferem (tela de ~256 px no SOTN; câmera
nossa com 405 u de altura visível), então compare **proporções e tempos**.

| Item | SOTN (evidência) | DCR | Observação |
|---|---|---|---|
| Andar | 1,5 instantâneo (`SetSpeedX`, `71830.c:557`; 90 px/s) | 190 u/s instantâneo | Ambos sem aceleração |
| Correr | não existe para Alucard (só itens/relíquias) **[I]** | 310 u/s (Shift) | Escolha nossa |
| Frenagem no chão | `DecelerateX(1/8)` em pé (`71830.c:261`; 450 px/s²) | parada instantânea | DCR mais seco |
| Pulo | vy −4,875 (`6E42C.c:166`; 292 px/s); apogeu em ~28 quadros (0,47 s), ~69 px | 720 u/s, gravidade 1900 → apogeu 0,38 s, ~136 u | |
| Corte do pulo | soltar com vy < −1 → **vy = −1** (`71830.c:586-590`) | `vy *= 0,45` | SOTN dá pulo mínimo muito menor (~8 px ≈ 11% do máximo; DCR ≈ 20%) |
| Flutuação no ápice | segurando pulo com vy ∈ (−1/8, 3/8): gravidade ÷5 (`6FDF8.c:17-24`) | não existe | "Hang time" que dá precisão |
| Gravidade | 22/128 (≈619 px/s²) (`6FDF8.c:14-16`) | 1900 u/s² | |
| Queda máxima | 7 (420 px/s) (`6FDF8.c:42-44`) | 900 u/s | Razão queda/pulo: 1,44 (SOTN) × 1,25 (DCR) |
| Teto | bater no teto → vy −0,25 (`71830.c:591-594`) | colisão zera | |
| Controle aéreo | pulo: ±1,5 instantâneo; sem entrada, freia 1/16 (`71830.c:585, 602-608`) | velocidade de chão instantânea | Equivalentes |
| Cair de borda | vx = 0, vy +2, controle ±0,75 (metade) (`6E42C.c:131-149`, `71830.c:754-763`) | mantém velocidade e controle total | DCR mais moderno; manter |
| Coyote time | 8 quadros após sair andando (`timers[5]`, `71830.c:755`) (133 ms) | `COYOTE_TIME = 0,08 s` | Próximos |
| Pouso | vy > 6,875 → pouso pesado com som/efeito; \|vx\| > 2 → pouso "duro" (`6FDF8.c:60-96`) | sem distinção | Polimento |
| Pulo duplo | vy −4,25 (`6E42C.c:182-190`) com relíquia | não existe | Candidato a habilidade de exploração |
| Backdash | vx −3,5, freia 1/8 (~28 quadros, ~49 px), cancelável por pulo/ataque (`6E42C.c:913-922`) **sem invencibilidade encontrada** | esquiva **para frente**, 480 u/s, 0,28 s, invulnerável, não cancelável | Ver 1.5 |
| Agachar | estado próprio; hurtbox menor via quadro | estado; hurtbox igual | Ver 1.8 |
| Escadas | o castelo usa rampas (`TOUCHING_ANY_SLOPE`), não escadas de subir | plataformas atravessáveis | — |

**[R] (movimento):**

1. **Flutuação no ápice** ao segurar pulo (gravidade × fator perto de vy = 0), constantes `APEX_GRAVITY_FACTOR` e `APEX_SPEED_BAND`. Pequeno, alto impacto na precisão. Teste: altura máxima maior com pulo segurado; tempo de ar maior.
2. **Corte de pulo por teto** (`vy = min(vy, JUMP_CUT_SPEED)`) em vez de multiplicador — dá mais amplitude entre pulo curto e longo. Teste: soltar no 1º quadro dá altura < X.
3. **Pouso pesado** acima de uma velocidade de queda: só apresentação (VFX/som/“squash”), sem travar controle no início.
4. **Esquiva cancelável** após uma fração (1.5); considerar oferecer também esquiva para trás (←+esquiva) — decisão de design, não cópia.
5. Não adotar: aceleração zero ao cair de borda, ausência de corrida.

---

## 3. Magias e comandos

- **Reconhecimento [F]:** uma máquina de estados **por comando** (`src/dra/704D0.c`): cada passo tem um timer de ~20 quadros (333 ms; 15 para ←→, 16 para a bota), a direção "frente" é **travada no início** (`g_WasFacingLeft*`), a primeira entrada às vezes exige `padTapped == ↑` exato (Hellfire, `:363`). Todas rodam todo quadro (`CheckSpecialMoveInputs`, `:787-828`); a primeira que dispara encerra o laço (prioridade = ordem fixa).
- **Execução [F]:** precisa do botão de ataque, **fora do bloqueio de ataque** (`!(unk46 & 0x8000)`) e em estados de chão (`:404-409`). `CastSpell` só confere MP (`5D5BC.c:256-265`) — **não há cooldown**. A magia é **aprendida ao ser executada pela primeira vez** (`LearnSpell`, `:268-280`; chamado em `704D0.c:416`).
- **Conjuração [F]:** Hellfire para o jogador, dá 16 quadros de invencibilidade, e as fases do jogador são **dirigidas pela entidade do efeito** (`g_Player.unk5C`); o efeito sai num quadro específico (pose 10) e segurar ↓ cancela (`71830.c:2241-2310`).
- **Dano de magia [F]:** `ataque = base + (INT·2 + rand 0..11)/10` (`GetSpellDef`, `5D5BC.c:200-206`).
- **Subarmas [F]:** ↑ + ataque, custo em Hearts (reduzido por acessório), **limite de instâncias na tela** em vez de cooldown (`6E42C.c:233-321`, `5D5BC.c:482-528`).
- **Transformações [F]:** custo contínuo de MP por tempo (morcego 1/60 quadros, névoa 10/8 quadros, lobo 1/120 quadros; `5D5BC.c:790-843`); entrada por botão dedicado, permitida de vários estados de locomoção (`692E8.c:1026-1075`).

**DCR:** buffer temporizado relativo à orientação, notação textual, tolerância a entradas extras, `total_window`/`step_window` por habilidade (Crescent Slash: 0,6 s / 0,3 s — próximo dos 333 ms do SOTN), prioridade e desempate, 8 resultados com feedback, revalidação em `begin_skill`, custo MP/Hearts, cooldown, efeito **instantâneo** no quadro da confirmação (D-GAME-002 pendente). Transformação (Forma Espectral) por tecla T, perfis de locomoção.

**[R] (magias):**

1. Manter o reconhecedor (superior: tolerante, data-driven, com diagnóstico).
2. Resolver D-GAME-002 com um campo `effect_delay` (padrão 0 = comportamento atual) e invulnerabilidade curta opcional `cast_invulnerability` (padrão 0). Teste: dano durante o atraso cancela sem cobrar? (decidir: SOTN cobra antes). Registrar a decisão.
3. Ideia de exploração: `acquisition: Discovery` já existe; "aprender ao executar" pode ser um modo para comandos secretos (comando funciona antes de ser listado). Baixa prioridade.
4. Considerar **limite de instâncias** (`max_active`) para projéteis como alternativa/complemento a cooldown. Baixa prioridade.

---

## 4. Inimigos e chefes

### 4.1 IA de inimigo comum (esqueleto com cimitarra, `src/st/e_bone_scimitar.h`) [F]

- Estados: iniciar → esperar chão → **aproximar** (0,5 px/quadro) até distância < 76 → **recuar andando de costas** até > 92 → aproximar (faixa com **histerese**, `:113-147`).
- Ataque só se o timer zerou, distância < 64 **e** está virado para o jogador (`:54-71`). Depois do golpe, o timer recebe o próximo valor de uma **tabela cíclica** por variante (`{128, 8, 8, 64}` quadros = ataques em rajada; `{240, 192, 160, 128}` = lento e acelerando) (`:45-48, 169-177`).
- Sensores de chão/parede: obstáculo → pula (vx ±2, vy −3, chance 1/4 de pular para frente) (`:57-60, 180-215`).
- Caixa de ataque só no quadro ativo da animação (`:150-161`).
- Morte: desmonta em partes com velocidades próprias; variante especial deixa item; marca "não reaparecer" na sala (`PreventEntityFromRespawning`, `st_common.h:61-67`).

### 4.2 Reações a dano [F]

- `HitDetection` (`src/st/collision.h:209+`): i-frames **por par atacante×alvo** (`entity->unk6D[slot] = atacante.nFramesInvincibility`, `:555-559`) — o mesmo golpe pode acertar de novo depois de N quadros, e atacantes diferentes são independentes; hitstun `stunFrames` da arma; intensidade `hitFlags` (1.12); elemento "corte" faz sangue (`:535-549`); golpe fraco em inimigo com flag de metal dá "clang" e `hitFlags = 2` (`:319-328`).
- Só entidades na tela (±32 px) participam da colisão (`:243-246, 276-278`).
- Ao matar: EXP (`func_800FE044`), contador de mortes, drops com sorte (`:577-620`).

**DCR:** `decide(profile, perception)` puro com Patrol/Chase/Attack/Wait/Flee; aggro × leash; *wind-up* 0,35 s com cor; flash 0,15 s; knockback fixo; cooldown único; `already_hit` (um acerto por hitbox por alvo); respawn por tempo (`respawn_time`) e ao reentrar na sala (correção da Fase 2, ver item 6 abaixo). Um arquétipo (`bone_sentry`).

**[R] (inimigos, sem reescrever a IA):**

1. **Faixa de espaçamento** no `AiProfile`: `preferred_range: (min, max)` com histerese → comportamento `Retreat` (já existe no enum, sem uso). Teste em `ai.rs`: dentro de `min` recua; só volta a aproximar além de `max`.
2. **Ritmo de ataque por tabela**: `attack_rhythm: [f32]` cíclica (padrão `[attack_cooldown]`). Teste: sequência de cooldowns segue a tabela.
3. **Atacar só virado para o jogador** (já olha para ele em Attack/Wait; manter como regra explícita no puro).
4. **Telegrafia por fase em dados**: `windup`, `active`, `recovery` por inimigo (hoje constantes em `behavior.rs`).
5. **Reação por arma** (1.12) e `rehit_interval` em hitboxes (golpes multi-hit e magias de área).
6. **Respawn**: hoje por tempo. O SOTN impede respawn na sala depois de morto **[?]** (quando zera não foi confirmado no PS1). Decidir: manter tempo ou "reaparece ao reentrar na sala" — este segundo combina com o ciclo de vida de salas da V1.7.

> **Correção (Fase 2):** a descrição do DCR acima estava incompleta. O DCR
> **já** recria todos os inimigos da sala ao reentrar
> (`src/enemies/spawning.rs::enter_new_room`; `docs/WORLD_PERSISTENCE.md`
> §4) e, **além disso**, faz reaparecer por tempo dentro da sala
> (`respawn_time`). No SOTN, confirmado: o morto não volta durante a
> visita, e a marca é zerada a cada carga de sala
> (`src/dra/5087C.c:1833-1834, 1843-1844, 1885-1886, 1895-1896`). A decisão
> que resta é só sobre o respawn **por tempo** (`respawn_time: 0` como
> padrão de conteúdo). Ver
> [`ESTUDO_SOTN_EXPLORACAO.md`](ESTUDO_SOTN_EXPLORACAO.md) §1.17.

### 4.3 Chefes **[?] — não investigado**

`src/boss/` não foi lido. Ver pendências no plano.

> **Atualização (Fase 2):** investigado em
> [`ESTUDO_SOTN_CHEFES.md`](ESTUDO_SOTN_CHEFES.md) (Medusa, Olrox forma
> normal, Scylla, Hippogryph e a infraestrutura de luta).

---

## 5. Exploração e mundo (parcial)

Confirmado no decomp: inimigos mortos marcados por sala (`PreventEntityFromRespawning`); bestiário marcado na primeira vez que um inimigo é atingido (`g_CastleFlags[ENEMY_LIST]`, `collision.h:404-409`); relíquia que mostra o nome do inimigo atingido (`:410-418`); dano/HUD com atraso de barra (`5F60C.c:1363-1381`).

**Não investigado:** formato de salas, portas, transições, mapa, câmera, distribuição de inimigos e recompensas. O DCR já tem grafo de mundo validado, 4 regiões (`assets/world/regions/`), transições com fade, segredos, mapa por tecla M, câmera por dados com zona morta e *look-ahead* (V2.3). Ver pendências em [`PLANO_MELHORIAS_SOTN.md`](PLANO_MELHORIAS_SOTN.md) §6.

> **Atualização (Fase 2):** investigado em
> [`ESTUDO_SOTN_EXPLORACAO.md`](ESTUDO_SOTN_EXPLORACAO.md). A marca por sala
> vale só durante a visita (ver a correção em §4.2, item 6).
