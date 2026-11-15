# Plano V2.4 — Combat Feel / Peso do Golpe

**Status:** direção aprovada (`DECISIONS.md`, D-028). **Nada aqui foi
implementado.** Cada etapa só começa com autorização específica do
proprietário.

Base: DCR @ `9f7616d` (`art/v1.2-protagonist`). Evidência SOTN: sotn-decomp
@ `3a0957b` (ver [`REFERENCIAS_SOTN.md`](REFERENCIAS_SOTN.md)). Estudos de
apoio: [`ESTUDO_SOTN_COMBATE.md`](ESTUDO_SOTN_COMBATE.md),
[`ESTUDO_SOTN_ANIMACOES.md`](ESTUDO_SOTN_ANIMACOES.md),
[`ESTUDO_SOTN_EQUIPAMENTOS.md`](ESTUDO_SOTN_EQUIPAMENTOS.md). Este plano
**substitui** o pacote P1 de [`PLANO_MELHORIAS_SOTN.md`](PLANO_MELHORIAS_SOTN.md)
§2 em nomes de campos e ordem (a correspondência está na §9).
Legenda: **[F]** fato · **[I]** inferência · **[R]** recomendação · **[?]**
pendente.

---

## 0. Princípios

1. **Padrão = comportamento atual.** Todo campo novo é opcional
   (`#[serde(default)]`), e o padrão reproduz exatamente o jogo de hoje.
   Uma etapa só é aceita se os testes atuais continuarem passando **sem
   alteração**.
2. **Nenhuma arma muda sem decisão explícita.** A linha `Sword` de
   `weapon_classes.ron` (a Iron Sword inicial) e as demais classes ficam com
   os valores atuais. Os valores de ajuste aparecem só como **sugestões**
   na §6.
3. **Mecanismo separado de números.** O commit que cria um mecanismo usa
   padrões neutros; mudar dados de balanceamento é outro commit, com
   decisão registrada.
4. **Regras puras primeiro.** Cada etapa começa em módulo puro testável e
   só depois passa a um sistema Bevy.
5. **Gameplay em dados, arte segue**
   ([`ESTUDO_SOTN_ANIMACOES.md`](ESTUDO_SOTN_ANIMACOES.md) §3: modelo
   híbrido; só a parte de dados entra na V2.4).
6. Constantes de sensação em `src/core/constants.rs`; balanceamento por
   arma em `assets/data/items/weapon_classes.ron`; por inimigo em
   `assets/data/enemies/*.ron`.
7. Sem dependências novas. Save V3 intacto.
8. Validação de cada etapa: `cargo fmt --check`, `cargo clippy
   --all-targets`, `cargo test`, `cargo check`, um comando Cargo por vez
   (`CLAUDE.md`, "Compilação segura").

---

## 1. Verificação de compatibilidade (feita sobre o código real)

| # | Item P1 | Situação atual no DCR | Compatível? | Observação |
|---|---|---|---|---|
| 1 | Linha do tempo por categoria | `WeaponClass{attack_time, recovery}`; hitbox nasce no 1º quadro e vive `swing × 0,6` (`inventory/weapons.rs:180-181`; `combat/attacks.rs:98-123`) | **Sim** | `startup = 0` e `active_fraction = 0,6` reproduzem o atual |
| 2 | Regras de cancelamento | Ataque só é cancelado por técnica (`Casting` aceita sair de `Attacking`, `player/state.rs:205-208`); skills declaram `usable_in: [.., Attacking]` | **Sim** | Hoje o cancelamento por técnica vale **a qualquer momento** do golpe |
| 3 | Ações por fase | `allows()` só olha o estado, não a fase do golpe (`state.rs:111-135`) | **Sim** | A máquina guarda só `remaining`; a fase exige conhecer o total |
| 4 | Recuo em intensidades | Jogador: `PLAYER_HURT_KNOCKBACK` fixo (`player/systems.rs:170-173`); inimigo: `ENEMY_KNOCKBACK` fixo (`enemies/behavior.rs:249`) | **Sim** | Intensidade `Normal` = valores atuais |
| 5 | Proteção contra dano simultâneo | **Jogador: já existe e está confirmada.** O primeiro acerto que causa dano liga `PLAYER_INVULNERABILITY`, e os demais do mesmo quadro são ignorados (`player/systems.rs:149-154`); `contact_damage` nem gera acerto com invulnerabilidade ativa (`enemies/behavior.rs:174-176`). **Inimigo: não há**: golpe básico e técnica no mesmo quadro somam | **Parcial** | Lado do jogador: só o refinamento opcional "maior dano do quadro" (Fase 1). Lado do inimigo: opcional (etapa 6) |
| 6 | Atordoamento e knockback | Inimigo: `flash` 0,15 s pula a IA, mas **mantém a velocidade** (desliza), e todo dano cancela o `windup` (`behavior.rs:76-79, 246-249`) | **Sim** | Congelar e resistir viram campos com padrão atual |
| 7 | Intervalo de reacerto | Cada hitbox acerta cada alvo **uma vez** (`already_hit`, `combat/hitboxes.rs:216-247`) | **Sim** | `rehit_interval: None` = atual |
| 8 | Controle aéreo no ataque | Em `Attacking` no ar a velocidade é **mantida sem controle**; no chão há atrito (`player/movement.rs:118-123`) | **Sim** | `air_control = 0` = atual |
| 9 | Hurtbox por postura | Hurtbox = tamanho do corpo, centrada, fixa; agachar não muda nada (`player/systems.rs:90-99`) | **Sim** | `Hurtbox` é componente separado de `Body`; reduzir altura exige deslocamento (campo novo) |

**Impacto no Save V3:** nenhum. O save não serializa `ActionState`,
`Invulnerable` nem perfis de golpe; só consulta o estado para recusar
salvar com o personagem caído (`save/systems.rs:79-82`).

**Ordem de sistemas** (não muda): `Commands → Actions → Combat → Resolution
→ Progression → Presentation` (`combat/mod.rs`, `player/mod.rs`,
`enemies/mod.rs`).

**Técnicas:** `melee_techniques` usa `execution_time × HITBOX_ACTIVE_FRACTION`
(`combat/attacks.rs:157`). As etapas 1 a 3 mudam **só o golpe básico**;
técnicas continuam iguais.

---

## 2. Modelo proposto (resumo)

```text
WeaponClass (RON, campos opcionais)       StrikeProfile (puro, resolvido)
  attack_time, recovery, power      ──►     action_time, active_time (atuais)
  + startup: f32           = 0.0             + startup
  + active_fraction: f32   = 0.6             + cancel_from: Option<f32>
  + cancel_after: Option<f32> = None         + knockback, stun, rehit_interval
  + knockback: Knockback   = Normal          + air_control
  + stun: Option<f32>      = None (= ENEMY_HURT_FLASH)
  + rehit_interval: Option<f32> = None
  + air_control: f32       = 0.0

PlayerStateMachine (puro)                  regras puras novas
  + total da ação (para saber a fase)        strike_phase(profile, elapsed)
                                             allowed_in(phase, action)
                                             hurt_reaction(dano, hp_max, postura)
                                             hurtbox_for(estado)
```

`StrikePhase { Startup, Active, Recovery }` não substitui
`PlayerState::Attacking`: é uma subfase consultada por quem precisa.

---

## 3. Dependências entre etapas

São 11 etapas (0 a 10). Cada uma é independente **ou** tem as dependências
abaixo explícitas; nenhuma depende de etapa posterior.

| Etapa | Depende de | Pode ir em paralelo com |
|---|---|---|
| 0 Caracterização | — | — (vem antes de todas) |
| 1 Linha do tempo | 0 | 4, 5, 7, 9 |
| 2 Fases e ações permitidas | 1 | 4, 5, 7, 9 |
| 3 Cancelamento | 2 | 4, 5, 7, 9 |
| 4 Controle aéreo | 0 | 1, 2, 3, 5, 7, 9 |
| 5 Reacerto | 0 | 1–4, 7, 9 |
| 6 Dano simultâneo | 0 (usa 5 se existir, mas não exige) | 1–4, 7, 9 |
| 7 Recuo por intensidade | 0 | 1–6, 9 |
| 8 Atordoamento | 7 (usa a intensidade no `DamageDealt`) | 9 |
| 9 Hurtbox por postura | 0 | 1–8 |
| 10 Retorno visual (opcional) | 0 | qualquer uma |

"Em paralelo" significa **ordem livre**; a execução continua sequencial
(um agente e um Cargo por vez).

```text
0 ─► 1 ─► 2 ─► 3
0 ─► 4
0 ─► 5 ─► (6)
0 ─► 7 ─► 8
0 ─► 9
0 ─► 10 (opcional, por último)
```

**Rollback geral:** cada etapa é um ou dois commits próprios (`feat` + `test`)
sem mudança de save; desfazer = `git revert <hash>` dos commits da etapa
(nunca `reset --hard`). Como os padrões reproduzem o atual, uma etapa
revertida não deixa dados órfãos. Se um RON já tiver campos novos, o revert
do mecanismo deve vir **depois** do revert do commit de dados
(`deny_unknown_fields` recusaria o campo).

---

## 4. Etapas

### Etapa 0 — Testes de caracterização

- **Objetivo:** fixar em testes a sensação atual, antes de qualquer mudança.
- **Arquivos:** `tests/combat_tests.rs` (ECS headless), testes de unidade
  em `src/inventory/weapons.rs` e `src/player/state.rs`.
- **Alterações permitidas:** só testes novos. Nenhuma linha de código de
  jogo ou dado.
- **Comportamento a preservar (o que os testes travam):**
  `StrikeProfile` da Iron Sword (`action_time 0,28`, `active_time 0,168`);
  hitbox presente no primeiro quadro do golpe; uma batida por alvo por
  hitbox; inimigo desliza durante o flash; dano cancela `windup`;
  velocidade aérea mantida durante o golpe; hurtbox = corpo; segundo acerto
  no mesmo quadro ignorado pelo jogador.
- **Testes:** os próprios.
- **Aceite:** todos passam no código atual.
- **Riscos:** teste frágil a ordem de iteração do ECS (usar mensagens
  determinísticas e `GameRandom` com semente).
- **Rollback:** reverter o commit de testes.

### Etapa 1 — Linha do tempo do golpe (P1.1)

- **Objetivo:** golpe básico com antecipação (`startup`) e janela ativa
  configuráveis por classe de arma.
- **Evidência SOTN:** liberação por pose e arma como entidade criada no
  golpe (`src/dra/6E42C.c:372-841`; `ESTUDO_SOTN_COMBATE.md` §1.4). **[F]**
- **Arquivos:** `src/inventory/weapons.rs` (campos, `validate`,
  `StrikeProfile::resolve`), `src/combat/attacks.rs` (`basic_attack`),
  `src/combat/hitboxes.rs` (componente opcional `PendingStrike { delay }`;
  `detect_hits` e `expire_hitboxes` ignoram hitboxes com ele). **Não**
  acrescentar campo a `Hitbox`: ele é construído literalmente em
  `tests/world_objects_tests.rs:132`, `tests/app_smoke_tests.rs:1269` e
  `tests/world_integration_tests.rs:36`, além de `magic/casting.rs` e
  `enemies/behavior.rs`,
  `assets/data/items/weapon_classes.ron` (só comentário do cabeçalho),
  `docs/COMBAT_SYSTEM.md`, `docs/EQUIPMENT_SYSTEM.md`.
- **Alterações permitidas:** campos opcionais `startup` e
  `active_fraction`; validação `startup + ativo ≤ attack_time`; criação da
  hitbox atrasada. **Proibido:** mudar valores das cinco classes.
- **Comportamento a preservar:** com os padrões, `StrikeProfile` idêntico
  ao atual para todas as classes e para o golpe desarmado; técnicas
  intactas; `cancel_interrupted_hitboxes` e `clear_on_room_change` também
  removem hitboxes pendentes.
- **Testes:** unidade: perfil padrão = atual (5 classes + desarmado);
  validação recusa `startup` negativo ou maior que o swing. ECS: com
  `startup > 0`, nenhum acerto antes do tempo; dano no jogador durante
  `startup` cancela a hitbox pendente.
- **Aceite:** etapa 0 verde sem edição; testes novos verdes.
- **Riscos:** hitbox pendente sobreviver a troca de sala ou ao recuo;
  `attack_speed` precisa escalar `startup` como escala o swing. Campos novos
  em `WeaponClass` exigem acrescentar o padrão no literal de teste
  `sword()` (`src/inventory/weapons.rs:217-218`), edição mecânica (ou
  usar `..Default`/construtor de teste).
- **Rollback:** reverter os commits da etapa; nenhum RON usa os campos
  ainda.

### Etapa 2 — Fases e ações permitidas (P1.3)

- **Objetivo:** uma tabela pura e única que diz o que pode ser feito em
  cada fase do golpe.
- **Evidência SOTN:** máscara de ações permitidas por estado
  (`src/dra/6FDF8.c:105-145`). **[F]**
- **Arquivos:** `src/player/state.rs` (guardar o total da ação; fase),
  novo `src/player/action_rules.rs` se `state.rs` crescer demais
  (`PLANO_MELHORIAS_SOTN.md` §3, risco 4), `src/skills/execution.rs`
  (só leitura da fase, se necessário).
- **Alterações permitidas:** `strike_phase`, `allowed_in(phase, action)`;
  campo de total na máquina de estados (não serializado).
- **Comportamento a preservar:** a tabela padrão replica `allows()` atual:
  durante `Attacking`, só técnicas (`Casting`) são aceitas, em qualquer
  fase.
- **Testes:** teste exaustivo estado × fase × ação comparando a tabela nova
  com o `allows()` atual; fases nas bordas (0, fim do startup, fim do
  ativo, fim).
- **Aceite:** nenhuma mudança jogável; tabela é a fonte única.
- **Riscos:** baixo (puro).
- **Rollback:** reverter; nada depende dela fora da etapa 3.

### Etapa 3 — Regras de cancelamento (P1.2)

- **Objetivo:** permitir sair da recuperação a partir de um ponto
  configurável por arma.
- **Evidência SOTN:** `lockDuration` libera pulo, backdash e novo ataque;
  `chainLimit` controla repetição (`src/dra/71830.c:365-431`,
  `6E42C.c:334-339`). **[F]**
- **Arquivos:** `src/inventory/weapons.rs` (`cancel_after`),
  `src/player/state.rs` (`start_action`/intenções usando `allowed_in`),
  `src/player/systems.rs` (pulo/esquiva/guarda), `src/combat/attacks.rs`.
- **Alterações permitidas:** `cancel_after: Option<f32>` (fração da ação);
  validação `cancel_after ≥ fim da janela ativa` (evita duas hitboxes do
  mesmo golpe vivas ao mesmo tempo). Conjunto liberado após o ponto:
  pulo, esquiva, guarda e novo ataque (proposta da Fase 1, §1.5).
- **Decisão pendente do proprietário:** técnicas continuam canceláveis a
  qualquer momento (proposta: **sim**, preserva o atual).
- **Comportamento a preservar:** `None` = atual; buffer de comandos
  inalterado.
- **Testes:** `None` ≡ atual; com valor, esquiva recusada antes e aceita
  depois; técnica aceita em qualquer fase; novo ataque depois do ponto cria
  uma só hitbox ativa.
- **Aceite:** testes verdes; autoplay com padrões sem diferença.
- **Riscos:** médio: interação com reconhecimento de comandos
  (o mesmo botão de ataque gera técnica ou golpe) e com a esquiva
  invulnerável virando "saída de tudo".
- **Rollback:** reverter; se algum RON usar `cancel_after`, reverter os
  dados antes.

### Etapa 4 — Controle durante o golpe (P1.8)

- **Objetivo:** controle horizontal opcional no golpe aéreo e janela
  opcional para virar no início do golpe no chão.
- **Evidência SOTN:** controle aéreo pleno durante o golpe; no chão, virar
  só nas primeiras poses (`src/dra/71830.c:356-362, 696-701`). **[F]**
- **Arquivos:** `src/player/movement.rs` (ramo `Attacking | Casting`),
  `src/player/movement.rs::update_facing`, `src/inventory/weapons.rs`.
- **Alterações permitidas:** `air_control: f32` (0..1) e `turn_window: f32`
  (s), ambos padrão 0.
- **Comportamento a preservar:** 0 = velocidade aérea mantida sem controle;
  sem virar durante o golpe; frenagem no chão igual.
- **Testes:** ECS: `air_control 0` mantém `velocity.x`; 1 iguala a
  velocidade de andar; `turn_window` vira só dentro da janela; hitbox
  presa acompanha a nova orientação.
- **Aceite:** padrões ≡ atual.
- **Riscos:** baixo; virar com hitbox já ativa pode acertar atrás (por isso
  a janela deve terminar antes do fim do startup).
- **Rollback:** reverter.

### Etapa 5 — Intervalo de reacerto (P1.7)

- **Objetivo:** hitboxes longas (técnicas, magias de área) podem acertar o
  mesmo alvo de novo após um intervalo.
- **Evidência SOTN:** imunidade por par atacante × alvo e
  `enemyInvincibilityFrames` por arma (`src/st/collision.h:551-570`). **[F]**
- **Arquivos:** `src/combat/hitboxes.rs` (componente opcional
  `RehitTimers { interval, recent }` ao lado do `Hitbox`; `detect_hits`
  consulta `Option<&mut RehitTimers>` e, sem ele, usa `already_hit` como
  hoje), `src/combat/attacks.rs`, definições de skill
  (`src/skills/definitions.rs`) se técnicas usarem. Não mudar o tipo de
  `already_hit` (literais nos testes, ver etapa 1).
- **Alterações permitidas:** `rehit_interval: Option<f32>` com mínimo
  validado (constante `MIN_REHIT_INTERVAL`).
- **Comportamento a preservar:** `None` = uma batida por alvo (D-009).
- **Testes:** `None` ≡ atual; 0,2 s numa hitbox de 0,5 s acerta 3 vezes;
  alvos diferentes independentes.
- **Aceite:** padrões ≡ atual.
- **Riscos:** médio: dano multiplicado se mal configurado.
- **Rollback:** reverter.

### Etapa 6 — Dano simultâneo (P1.5)

- **Objetivo:** decidir o que acontece quando vários acertos chegam no
  mesmo quadro.
- **Situação:** no jogador a proteção **já existe** (§1, linha 5). Restam
  dois refinamentos opcionais:
  - (a) **Jogador, maior dano do quadro** (Fase 1, §1.20; SOTN aplica o de
    maior ataque, `src/st/collision.h:366-376`): escolher o maior entre os
    acertos do quadro, em vez do primeiro.
  - (b) **Inimigo, imunidade curta por fonte:** `hit_immunity` por inimigo
    (s).
- **Arquivos:** `src/player/systems.rs::receive_damage` (a);
  `src/enemies/behavior.rs::receive_damage`,
  `src/enemies/definitions.rs` (b).
- **Alterações permitidas:** (a) atrás de uma constante/flag com padrão
  "primeiro acerto" (atual); (b) campo opcional padrão 0.
- **Comportamento a preservar:** primeiro acerto vale no jogador; golpe +
  técnica somam no inimigo.
- **Testes:** (a) dois `DamageDealt` (5 e 30) no mesmo quadro: padrão
  aplica o primeiro; ativado, aplica 30. (b) padrão soma; com valor, só o
  primeiro.
- **Aceite:** padrões ≡ atual.
- **Recomendação:** (a) sim, ligado só após decisão; (b) só se testes de
  jogo mostrarem abuso. A etapa pode ficar fora da V2.4 sem afetar as
  outras.
- **Riscos:** baixo.
- **Rollback:** reverter.

### Etapa 7 — Recuo por intensidade (P1.4)

- **Objetivo:** golpes mais fortes empurram mais; dano pequeno pode não
  tirar o jogador do controle.
- **Evidência SOTN:** recuo do jogador em três intensidades pelo dano
  relativo ao HP máximo, variando em pé/agachado/no ar
  (`src/dra/5D5BC.c:593-759`, `71830.c:1237-1644`); inimigo recebe
  intensidade 1/2/3 (`src/st/collision.h:525-533`). **[F]**
- **Arquivos:** `src/combat/damage.rs` ou novo `src/combat/reaction.rs`
  (puro: `Knockback`, `hurt_reaction`), `src/core/events.rs`
  (`DamageDealt` ganha a intensidade, com construtor padrão),
  `src/combat/hitboxes.rs`, `src/player/systems.rs::receive_damage`,
  `src/enemies/behavior.rs::receive_damage`, `src/enemies/definitions.rs`
  (`knockback_resistance`), `src/core/constants.rs`.
- **Alterações permitidas:** `enum Knockback { None, Light, Normal, Heavy }`
  (padrão `Normal` = constantes atuais); `hurt_reaction(dano, hp_max,
  postura)` com limiares em constantes; `stagger_threshold` (Fase 1,
  "golpe fraco não derruba") padrão 0 = todo dano derruba, como hoje.
- **Comportamento a preservar:** com padrões, recuo do jogador
  `(220, 260)` e do inimigo `160` para qualquer dano; todo dano tira o
  jogador do controle (exceto guarda).
- **Testes:** `Normal` ≡ atual; `Heavy > Normal > Light`; resistência 1
  anula o recuo; limiares nas bordas; `stagger_threshold 0` ≡ atual.
- **Aceite:** padrões ≡ atual; todos os emissores de `DamageDealt`
  compilam com o padrão.
- **Riscos:** médio. `DamageDealt` é construído literalmente em
  `src/combat/hitboxes.rs`, `src/enemies/behavior.rs`, `src/core/events.rs`,
  `tests/app_smoke_tests.rs`, `tests/world_transform_tests.rs`,
  `tests/player_vfx_tests.rs` e `examples/player_demo.rs`. Um campo novo
  obriga a editar esses literais. Duas saídas:
  - (i) **recomendada:** campo novo + construtor `DamageDealt::new(..)` com
    a intensidade padrão; a edição dos testes é **mecânica** (só acrescenta
    o padrão) e é a única exceção permitida ao critério "testes sem edição",
    registrada no commit;
  - (ii) mensagem paralela (`HitFeel`) com a intensidade, sem tocar
    `DamageDealt`; evita editar testes, mas exige casar as duas mensagens.
- **Rollback:** reverter (sem dados novos nos RON até haver decisão).

### Etapa 8 — Atordoamento do inimigo (P1.6)

- **Objetivo:** atordoamento por arma, com opção de congelar o inimigo
  (o *hitstop* do SOTN) e de resistir à interrupção.
- **Evidência SOTN:** `stunFrames` faz o laço pular a entidade atingida
  (`src/st/st_update.h:122-127`, `src/st/collision.h:560-562`). **[F]**
- **Arquivos:** `src/enemies/behavior.rs` (`think_and_move`,
  `receive_damage`), `src/enemies/definitions.rs`, `src/inventory/weapons.rs`
  (`stun`), `src/core/events.rs` (se `stun` viajar no `DamageDealt`).
- **Alterações permitidas:** `stun: Option<f32>` por classe (padrão
  `ENEMY_HURT_FLASH`); `freeze_on_stun: bool` por inimigo (padrão
  `false`); `interruptible: bool` por inimigo (padrão `true`).
- **Comportamento a preservar:** 0,15 s de flash, deslizando com o
  knockback; todo dano cancela o `windup`.
- **Testes:** padrão ≡ atual; `freeze_on_stun` mantém a posição e aplica
  o knockback ao fim; `interruptible: false` preserva o `windup`.
- **Aceite:** padrões ≡ atual.
- **Riscos:** médio: inimigo preso em atordoamento contínuo (mitigar com a
  etapa 5/6 ou um teto de atordoamento acumulado).
- **Rollback:** reverter; se algum RON de inimigo usar os campos, reverter
  os dados antes.

### Etapa 9 — Hurtbox por postura (P1.9)

- **Objetivo:** agachar reduz a área atingível do jogador.
- **Evidência SOTN:** caixa por quadro de animação, inclusive no jogador
  (`src/dra/alu_anim.c:191-214`; `692E8.c:1401-1410`). **[F]**; tamanho
  exato agachado **[I]**.
- **Arquivos:** `src/combat/hitboxes.rs` (`Hurtbox` ganha `offset`, padrão
  zero; `detect_hits` aplica), `src/enemies/behavior.rs::contact_damage`
  (aplica o offset), `src/player/systems.rs` (sistema pequeno
  `update_hurtbox` em `GameSet::Actions`), `src/core/constants.rs`
  (`PLAYER_CROUCH_HURTBOX`), `src/enemies/spawning.rs` (inicializa
  `offset`).
- **Alterações permitidas:** tabela `hurtbox_for(estado)` com escala 1 e
  offset 0 por padrão; só `Hurtbox` muda, nunca `Body`.
- **Comportamento a preservar:** hurtbox = corpo em todos os estados.
- **Testes:** padrão ≡ atual; com valor, agachado evita golpe alto e ainda
  recebe golpe baixo; inimigos sem mudança.
- **Aceite:** padrões ≡ atual; visual do agachar coerente com a caixa
  (conferir no autoplay).
- **Riscos:** médio: esquecer o offset em algum ponto de colisão (dois
  locais: `detect_hits` e `contact_damage`). `Hurtbox` é construído
  literalmente em `tests/app_smoke_tests.rs:1248`; o campo novo exige a
  mesma edição mecânica da etapa 7 (ou um construtor `Hurtbox::new`).
- **Rollback:** reverter.

### Etapa 10 — Retorno visual (opcional)

- **Objetivo:** reforçar a leitura do impacto sem mudar regras.
- **Arquivos:** `src/ui/hud.rs` (HP escorrendo), `src/core/constants.rs`;
  opcional `src/player/systems.rs` (micro-pausa da ação do jogador).
- **Alterações permitidas:** barra "sombra" de HP com `HP_DRAIN_TIME`;
  `hit_pause` (s) que pausa só o temporizador da ação do jogador ao
  acertar, padrão 0. Nada de pausa global (`Time<Virtual>`).
- **Comportamento a preservar:** barra vai direto ao valor (o número
  continua exato); duração das ações igual com `hit_pause 0`.
- **Testes:** unidade da sombra (converge ao valor em `HP_DRAIN_TIME`);
  ECS: `hit_pause 0` não muda a duração.
- **Aceite:** só apresentação; screenshots do autoplay revisadas.
- **Riscos:** baixo; `hit_pause > 0` interage com janelas de comando.
- **Rollback:** reverter. Pode ir para versão posterior sem prejuízo.

---

## 5. Commits sugeridos

`test(combat): pin current strike feel` (0) → `feat(inventory): add strike
timeline fields` (1) → `feat(player): add attack phases` (2) →
`feat(player): add attack cancel window` (3) → `feat(player): add control
during attacks` (4) → `feat(combat): add rehit interval` (5) →
`feat(combat): configurable simultaneous hit rule` (6) → `feat(combat): add
knockback intensities` (7) → `feat(enemies): add stun and interruptibility`
(8) → `feat(player): stance-dependent hurtbox` (9) → `feat(ui): hp drain
bar` (10). Testes de cada etapa no mesmo commit ou num `test(...)` logo
depois. Dados de balanceamento: commits separados, depois de decisão.

---

## 6. Valores sugeridos (NÃO aplicar sem decisão do proprietário)

A Iron Sword (`Sword`) **não** está na lista. `cancel_after` é fração da
ação inteira (swing + recuperação) e precisa ficar depois do fim da janela
ativa.

| Classe | `startup` (s) | `cancel_after` | `knockback` | `air_control` | Intenção |
|---|---|---|---|---|---|
| Dagger | 0,00 | 0,65 | Light | 0,6 | Rápida e encadeável |
| Axe | 0,08 | — | Heavy | 0,2 | Peso perceptível |
| Spear | 0,05 | 0,7 | Normal | 0,3 | Alcance com compromisso |
| HeavyWeapon | 0,14 | — | Heavy | 0,0 | Lenta e poderosa |

---

## 7. Critérios de aceite da versão

1. Todos os testes existentes passam sem edição, exceto a edição mecânica
   de literais de `WeaponClass`/`DamageDealt`/`Hurtbox` (etapas 1, 7 e 9),
   que só acrescenta o valor padrão.
2. Cada etapa tem testes de padrão (≡ atual) e de efeito novo.
3. `cargo run --example autoplay` roda até o fim com os padrões, e o
   roteiro (acertos, mortes, salas) não muda.
4. Nenhuma linha de `weapon_classes.ron` ou de inimigos alterada sem decisão
   registrada em `docs/DECISIONS.md`.
5. `docs/ARCHITECTURE.md`, `COMBAT_SYSTEM.md`, `ENEMY_SYSTEM.md` e
   `EQUIPMENT_SYSTEM.md` atualizados conforme as etapas feitas.

---

## 8. Fora da V2.4

Chefes, IA avançada, combos de golpes distintos (`combo_chain`), golpes com
forma própria no ar/agachado, escudo com bloqueio ativo, invulnerabilidade
`recuo + cauda`, tremor de câmera, som, marcas de animação no manifesto.
Proposta em [`PROPOSTA_ROADMAP_POS_V2_4.md`](PROPOSTA_ROADMAP_POS_V2_4.md).

---

## 9. Correspondência com o P1 da Fase 1

| `PLANO_MELHORIAS_SOTN.md` §2 (P1) | Neste plano |
|---|---|
| 1. Linha do tempo (`startup`, `active`, `cancel_after`) | Etapa 1 (`active` virou `active_fraction`); `cancel_after` na etapa 3 (`None` = "fim da ação") |
| 2. Ações permitidas por fase | Etapas 2 e 3 |
| 3. Recuo por intensidade + `stagger_threshold` | Etapa 7 |
| 4. Maior dano do quadro | Etapa 6 (a), opcional, padrão "primeiro acerto" |
| 5. Reação do inimigo por arma (`stun`, `knockback`, resistência, `rehit_interval`) | Etapas 5, 7 e 8 |
| 6. Controle aéreo + hurtbox de agachado | Etapas 4 e 9 (+ `turn_window`, Fase 1 §1.6) |

## 10. Pendências

- Valores reais de `stunFrames`/`lockDuration` por arma do SOTN: não estão
  no repositório (dados gerados do disco). **[?]**
- Hurtbox do jogador agachado no SOTN: inferida pela caixa por quadro;
  dimensões exatas não levantadas. **[I]**
- Decisões do proprietário: técnicas canceláveis a qualquer momento
  (etapa 3); ligar "maior dano do quadro" (etapa 6a); valores da §6.
