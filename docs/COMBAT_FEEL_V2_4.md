# V2.4 — Combat Feel: linha do tempo, cancelamento, controle, reacerto, dano simultâneo e recuo

Plano: [`PLANO_V2_4_COMBAT_FEEL.md`](PLANO_V2_4_COMBAT_FEEL.md).

| Bloco | Etapas | Situação |
|---|---|---|
| A | 0 caracterização, 1 linha do tempo, 2 ações por fase, 3 cancelamento | concluído |
| B1 | política de hitbox no cancelamento, 4 controle aéreo e orientação, 5 reacerto | concluído |
| B2 | caracterização do dano, 6 dano simultâneo, 7 recuo por intensidade e limiar (§12) | implementado, aguardando revisão |
| — | 8 a 10 | não iniciadas |

**Nenhum valor de dados mudou.** Com os campos novos omitidos (como estão
em todos os RON), o jogo se comporta exatamente como antes da V2.4. Os
valores sugeridos no plano (adaga, machado e lança) **não** foram aplicados.

## 1. Modelo

```text
WeaponClass (RON, campos opcionais)          StrikeProfile (resolvido, puro)
  startup: f32              = 0.0     ──►      startup / attack_speed
  active_fraction: f32      = 0.6              active_time = swing * active_fraction
  cancel_after: Option<f32> = None             cancel_after (fração, não escala)
  air_control: f32          = 0.0              air_control
  turn_window: f32          = 0.0              turn_window / attack_speed
  rehit_interval: Option<f32> = None           rehit_interval (segundos, não escala)
  knockback: KnockbackIntensity = Medium       knockback (B2)

StrikeTimeline (combat/strike_timeline.rs, puro)
  startup, active, total, cancel_at, turn_window
  phase(elapsed) -> StrikePhase { Startup, Active, Recovery }
  cancelable(elapsed), can_turn(elapsed)

player/action_rules.rs (puro)
  while_attacking(phase, cancelable, intent) -> Permission
  steer_in_air(current, wanted, air_control, dt) -> f32

Componentes (combat/attacks.rs, combat/hitboxes.rs)
  ActiveStrike { timeline, air_control }   no atacante (transitório)
  PendingStrike { delay }                   hitbox em antecipação
  SideLock { sign }                         lado travado do golpe básico
  RehitTimers { interval, recent }          reacerto opcional
Recurso: StrikeCancelPolicy { Preserve (padrão), RemoveOnCancel }

B2 (§12)
  combat/reaction.rs (puro)      KnockbackIntensity { Light, Medium (padrão), Heavy }
                                 StaggerThreshold (fração do HP máximo, padrão 0)
  combat/simultaneous.rs (puro)  HitRank, strongest_per_target
  DamageDealt                    + knockback: KnockbackIntensity
  HitKnockback(intensidade)      na hitbox do golpe básico (só se ≠ Medium)
  Recursos: SimultaneousHitPolicy { Legacy (padrão), HighestDamagePerFrame }
            PlayerStagger(StaggerThreshold::ALWAYS)
```

`StrikePhase` é uma subfase de `PlayerState::Attacking`, não um estado
novo. A máquina de estados ganhou só `cancel_attack()`.

### Fontes de tempo (sem relógio novo)

- A **ação** continua no temporizador do `ActionState` (`remaining`); o
  tempo decorrido do golpe é `total - remaining`.
- A **hitbox** continua com o seu `lifetime`; a antecipação fica em
  `PendingStrike`, e as esperas de reacerto em `RehitTimers`.
- Nada depende de quadros de animação nem da física.

## 2. Campos de `WeaponClass` (`assets/data/items/weapon_classes.ron`)

| Campo | Padrão (= antes) | Unidade | Validação |
|---|---|---|---|
| `startup` | `0.0` | s com velocidade 1, **dentro** de `attack_time` | `≥ 0` e `startup + attack_time × active_fraction ≤ attack_time` |
| `active_fraction` | `0.6` | fração de `attack_time` | `(0, 1]` |
| `cancel_after` | `None` | fração da ação (`attack_time + recovery`) | `(0, 1]` e `≥ (startup + ativo) / ação` |
| `air_control` | `0.0` | fração do controle normal no ar | `[0, 1]` |
| `turn_window` | `0.0` | s com velocidade 1, desde o início do golpe | `[0, attack_time]` |
| `rehit_interval` | `None` | s entre dois acertos no mesmo alvo | `[MIN_REHIT_INTERVAL = 0,1; 1,5]` |
| `knockback` (B2) | `Medium` | `Light`, `Medium` ou `Heavy` | enum (valor desconhecido recusa o RON) |

Valores fora da faixa recusam o catálogo (`WeaponError::OutOfRange` com o
nome do campo). Os valores por classe continuam os mesmos de antes.

## 3. Linha do tempo (etapa 1)

- `basic_attack` grava `ActiveStrike` no atacante. Ele só vale enquanto o
  estado é `Attacking`; só o ataque básico entra nesse estado.
- Com `startup > 0`, a hitbox nasce com `PendingStrike` e
  `Visibility::Hidden`. `arm_pending_strikes` (set `Combat`, antes de
  `detect_hits`) desconta a antecipação e a mostra ao armar. Enquanto
  pendente, ela segue o dono, mas `detect_hits` e `break_walls` a ignoram e
  o `lifetime` não corre.
- Com `startup = 0` nada disso é inserido: a hitbox acerta desde o primeiro
  quadro, como antes.
- Dano (`cancel_interrupted_hitboxes`) e troca de sala
  (`clear_on_room_change`) removem hitboxes pendentes como qualquer outra.

## 4. Ações por fase (etapa 2)

`player::action_rules::while_attacking` é a tabela única durante o ataque
básico. Fora dele, `PlayerState::allows` continua respondendo.

| Intenção | Startup | Active | Recovery antes do ponto | Recovery depois do ponto |
|---|---|---|---|---|
| Andar, agachar, interagir | negado | negado | negado | negado |
| Pulo, esquiva, guarda, novo ataque | negado | negado | negado | **permitido** (cancela) |
| Técnica, magia | pelos dados | pelos dados | pelos dados | pelos dados |

"Pelos dados" = `usable_in` contém `Attacking` (validação de `begin_skill`,
inalterada). Hoje só técnicas de espada listam `Attacking`; nenhuma magia.
Sem `cancel_after`, a tabela é idêntica ao `allows()` anterior (teste
exaustivo). Virar e o controle aéreo não passam pela tabela: são regidos por
`turn_window` e `air_control` (§7).

## 5. Cancelamento (etapa 3)

`ActiveStrike::try_cancel` consulta a tabela e chama `cancel_attack()`; a
ação segue então o caminho normal, com as mesmas condições de antes.

| Ação | Onde | Hitbox do golpe |
|---|---|---|
| Pulo | `player::movement::locomotion` | já expirou (ver abaixo) |
| Guarda, esquiva | `player::movement::guard_and_dodge` | já expirou |
| Novo ataque | `combat::attacks::basic_attack` | removida sempre que um golpe novo começa, com ou sem cancelamento (uma hitbox de golpe por atacante) |
| Técnica ou magia permitida | `skills::systems::begin_skill` (inalterado) | segue `StrikeCancelPolicy` (§6) |

**Por que a hitbox já expirou** no pulo, na esquiva e na guarda: ela desconta
o `lifetime` no próprio quadro em que nasce, e o temporizador da ação só no
seguinte; com o ponto de cancelamento validado para depois da janela ativa,
a hitbox some pelo menos um quadro antes do primeiro quadro em que o
cancelamento é aceito. Os testes apertam a ação a cada quadro e conferem que
nenhuma hitbox do golpe sobra. Quando as durações são múltiplos exatos do
quadro, o arredondamento pode deixar a hitbox viva no quadro em que o golpe
termina; por isso `basic_attack` remove qualquer hitbox de golpe anterior do
atacante ao iniciar um golpe novo (correção da revisão do checkpoint; com a
Iron Sword nunca acontece, pois a hitbox dura 0,168 s de um golpe de 0,28 s).
No pulo, na esquiva e na guarda, esse caso extremo deixaria a hitbox no
máximo o último quadro da sua própria vida.

Prioridade preservada: no mesmo quadro, guarda vence esquiva, que vence
pulo. Um comando reconhecido vira técnica e não gera ataque básico.
Custos e cooldown só passam por `begin_skill`: cancelar não cobra MP nem
Hearts e não ignora cooldowns.

## 6. Política da hitbox quando uma habilidade cancela o golpe

**Camada escolhida: um recurso global** `combat::attacks::StrikeCancelPolicy`,
iniciado com `Preserve` pelo `CombatPlugin`. Motivos:

- é uma regra do jogo, não de uma arma: um campo por arma não teria uso
  diferente entre armas;
- um campo por habilidade exigiria mudar `SkillDefinition`, montada
  literalmente em testes antigos (`app_smoke_tests`, `command_parser_tests`);
- um recurso pode ser trocado em teste sem tocar nos dados. Nenhum RON foi
  alterado e `RemoveOnCancel` não está ativo no jogo.

| Política | Efeito |
|---|---|
| `Preserve` (padrão) | a hitbox do golpe vive até o fim do seu `lifetime`; golpe e habilidade podem acertar o mesmo alvo (comportamento anterior) |
| `RemoveOnCancel` | `apply_strike_cancel_policy` (set `Actions`, depois de `begin_skill`) remove as hitboxes `WeaponStrike` do lançador no quadro em que a habilidade começa, antes de `GameSet::Combat`; acertos anteriores continuam aplicados |

Vale para técnicas e para magias com `Attacking` em `usable_in`. O novo
ataque já remove a hitbox anterior em qualquer política (§5). A remoção é
restrita a hitboxes com `WeaponStrike` cujo dono é o lançador: hitboxes de
técnicas, projéteis e de inimigos nunca têm `WeaponStrike`. Uma hitbox
`WeaponStrike` viva quando uma habilidade começa só pode ser do golpe
interrompido, porque todos os outros caminhos para fora de `Attacking` não
deixam nenhuma (§5).

Para mudar a política do jogo, troque o padrão do enum (ou insira o recurso
no app). Se um dia for preciso por habilidade, a proposta é um campo
opcional em `SkillDefinition` (exige editar os literais de teste).

## 7. Controle aéreo e orientação (etapa 4)

### Controle aéreo (`air_control`)

No ramo `Attacking | Casting` de `locomotion`, só para o ataque básico no ar:

- `0` (padrão): velocidade horizontal mantida, sem controle (antes);
- `1`: a velocidade vira a de andar/correr na direção apertada, como num
  pulo normal (sem tecla, para);
- entre os dois: fecha essa fração da diferença por quadro de referência de
  1/60 s (`AIR_CONTROL_REFERENCE_FRAME`), independente da taxa de quadros.

No chão a frenagem (`ACTION_FRICTION`) é a mesma de antes. Técnicas
(`Casting`) não têm controle aéreo. A física e as colisões não mudaram: a
velocidade passa pelo mesmo `world::physics` (paredes e plataformas).

### Virar durante o golpe (`turn_window`)

`update_facing` deixa virar quando `allows(Move)` (como antes) **ou** quando o
golpe básico está dentro de `turn_window` (`ActiveStrike::may_turn`). Virar
não faz andar. Casting, Dodging, Guarding, Hurt e Dead nunca viram.

### Orientação das hitboxes (`SideLock`)

- A hitbox do golpe básico leva `SideLock`. Enquanto pendente, ela acompanha
  a orientação (o jogador ainda está mirando); ao armar, o lado fica travado
  e virar não a inverte.
- Hitboxes **sem** `SideLock` continuam espelhando o dono a cada quadro, como
  antes: golpes de inimigos e técnicas. Essa é a declaração explícita de
  "acompanhar a orientação".
- Com os padrões, nada muda: sem janela de virar, a orientação não muda
  durante o golpe nem durante a técnica que o cancela.

Comandos: `update_facing` continua antes do reconhecimento; dentro da
janela, as direções do comando são lidas em relação à orientação já
atualizada, como ao andar.

## 8. Reacerto (etapa 5)

- `rehit_interval` na classe → `RehitTimers` na hitbox do golpe básico.
- Sem `RehitTimers`, `detect_hits` usa `already_hit` como antes (D-009: um
  acerto por alvo por hitbox).
- Com `RehitTimers`, cada par (esta hitbox, alvo) espera `interval` segundos
  entre acertos. O registro é por hitbox, então dois ataques nunca se
  confundem, e por entidade alvo, então dois inimigos são independentes. O
  alvo entra na espera no próprio acerto, então nunca há dois acertos no
  mesmo quadro.
- Limpeza: as esperas vencidas saem do registro a cada quadro, e o registro
  desaparece junto com a hitbox. Um alvo que some deixa no máximo uma entrada
  até a espera vencer.
- O tempo de espera não corre enquanto a hitbox está pendente.
- `MIN_REHIT_INTERVAL = 0,1 s` (valor próprio) impede acertos a cada quadro.
- Projéteis, técnicas e inimigos não recebem `RehitTimers`: continuam com um
  acerto por alvo. Ligar reacerto em técnicas exigiria um campo em
  `SkillDefinition` (fora do escopo, mesma razão do §6).

## 9. Compatibilidade

- Save V3: nada novo é salvo (`ActiveStrike`, `PendingStrike`, `SideLock`,
  `RehitTimers` e, no B2, `HitKnockback`, `SimultaneousHitPolicy` e
  `PlayerStagger` são transitórios ou de configuração); nenhum arquivo de
  `src/save/` mudou.
- RON: nenhum arquivo usa os campos; `deny_unknown_fields` continua.
- Iron Sword (classe `Sword`): `action_time 0,28`, `active_time 0,168`,
  hitbox no primeiro quadro, sem cancelamento, sem virar, sem controle
  aéreo, um acerto por alvo — inalterada.
- Técnicas, magias, transformações, inimigos, NPCs, comércio, exploração,
  câmera e cenários: sem alteração de dados.

## 10. Testes

| Arquivo | O que trava |
|---|---|
| `tests/combat_feel_tests.rs` — caracterização (12) | perfil da Iron Sword; hitbox no 1º quadro e duração (10 quadros de hitbox, 18 de golpe a 16 ms); só técnicas cancelam; guarda mantida assume ao fim; freio no chão; golpe aéreo sem controle; golpe agachado = em pé, hurtbox = corpo; técnica cancela e cobra uma vez; cooldown e magia não cancelam; entrada real ↓ ↘ → + X; dano remove o golpe, recuo e invulnerabilidade |
| `tests/combat_feel_tests.rs` — Bloco A (11) | linha do tempo padrão; `startup`; dano na antecipação; esquiva, pulo, guarda e novo ataque no quadro exato do ponto; uma hitbox por vez; técnica ainda cancela; magia e andar negados |
| `tests/combat_feel_tests.rs` — B1 (20) | política `Preserve` e `RemoveOnCancel` (técnica, magia, outras hitboxes intactas, nada muda sem cancelamento); queda sem controle; controle total; frenagem no chão igual; paredes; janela de virar dentro e fora; hitbox armada não inverte; hitbox pendente acompanha e trava; golpe agachado vira; técnica e esquiva não viram; comandos com janela; sem reacerto = 1 acerto; reacerto 0,2 s = 3 acertos em 0,5 s, por alvo; dois golpes = um acerto cada; limpeza do registro; técnicas sem reacerto |
| `tests/combat_feel_tests.rs` — revisão do checkpoint (1) | golpe novo a partir de `Idle` remove uma hitbox de golpe remanescente do mesmo atacante, nunca a de outro dono |
| `src/inventory/weapons.rs` | campos omitidos ≡ antes; validação; escala por velocidade |
| `src/combat/strike_timeline.rs` | fases e bordas; ponto de cancelamento; janela de virar |
| `src/combat/hitboxes.rs` | `RehitTimers` por alvo e limpeza; `PendingStrike` |
| `src/player/action_rules.rs` | tabela ≡ `allows()` sem ponto; o que o ponto libera; `steer_in_air` |
| `src/player/state.rs` | `cancel_attack` só encerra `Attacking` |
| B2: ver §12.7 | caracterização do dano, dano simultâneo, recuo e limiar |

Já cobertos antes e mantidos: comandos e prioridade
(`command_parser_tests.rs`), custos, cooldowns, dano recebido,
invulnerabilidade e interrupção (`app_smoke_tests.rs`). O plano previa a
caracterização em `tests/combat_tests.rs`; esse arquivo só tem testes puros
da fórmula de dano, por isso os testes ECS ficaram num arquivo próprio.

Resultados ao fim do B1: `cargo fmt --all -- --check` ok;
`cargo check -j 2` ok; `cargo clippy --all-targets -j 2 -- -D warnings` ok;
`cargo test -j 2`: 483 testes, 0 falhas (122 de unidade, 43 em
`combat_feel_tests`); `cargo run --example autoplay`: PASS 23/23, mesmos
acertos e gastos de antes da V2.4, saves reais intactos.

## 11. Limitações conhecidas

1. Com `Preserve` (padrão), golpe e técnica podem acertar o mesmo alvo na
   mesma janela (comportamento anterior). `RemoveOnCancel` existe, testado,
   mas não está ativo.
2. Sem buffer de entrada: uma esquiva, pulo ou ataque apertado antes do ponto
   de cancelamento se perde (a guarda é mantida, então assume sozinha).
3. Andar no chão continua negado durante o golpe; no ar, só com
   `air_control`.
4. Um novo ataque por cancelamento só vira se `turn_window` permitir.
5. No ar, só um novo ataque cancela (guarda e esquiva exigem chão; pulo,
   chão ou coyote time).
6. Com `turn_window` maior que `startup`, o personagem pode virar com a
   hitbox armada travada do outro lado (intencional: a hitbox não inverte).
   Recomenda-se `turn_window ≤ startup`.
7. A hitbox some cerca de um quadro antes do fim teórico da janela, porque
   desconta já no quadro de criação (como antes da V2.4).
8. Reacerto e política de cancelamento não são configuráveis por
   habilidade (§6, §8).
9. Plataformas de mão única não têm teste dedicado; a física não foi tocada.

## 12. Bloco B2 — dano simultâneo e recuo por intensidade (etapas 6 e 7)

Tudo é opcional e desligado: com `SimultaneousHitPolicy::Legacy`,
`KnockbackIntensity::Medium` e `StaggerThreshold::ALWAYS` (os padrões), o
jogo é o de antes. Nenhum RON mudou, nem a fórmula de dano, nem a
invulnerabilidade, nem a física.

### 12.1 Caracterização do dano atual (etapa 0 do B2)

Onde o dano nasce, muda e é consumido:

```text
GameSet::Combat      detect_hits ──┐ (fórmula pura combat::damage, no produtor)
                     contact_damage ┴─► DamageDealt { attacker, target, outcome, direction, knockback }
GameSet::Resolution  player::systems::receive_damage    (guarda, invulnerabilidade, hurt, recuo)
                     enemies::behavior::receive_damage  (soma, flash, wind-up, recuo, morte)
GameSet::Presentation  números de dano, faísca da guarda (só leitura)
```

| Aspecto | Jogador | Inimigo |
|---|---|---|
| Vários acertos no mesmo quadro | só o **primeiro** que causa dano (a invulnerabilidade que ele inicia descarta os demais) | **todos somam** |
| Invulnerabilidade | 0,9 s após dano (também em guarda); `detect_hits` e o contato nem geram acertos com ela ativa | não tem |
| Guarda | −70 % de frente, sem `Hurt` | não tem |
| Reação | `Hurt` 0,35 s + recuo (±220, 260) para **qualquer** dano | flash 0,15 s (IA pausada, desliza), *wind-up* cancelado, recuo ±160 para qualquer dano; o último acerto do quadro decide a direção |
| Morte | `Dead`, velocidade x zerada, mensagens restantes descartadas | `EnemyDefeated` uma vez, despawn, respawn agendado uma vez |
| Contato | um acerto por quadro (o primeiro inimigo encostado) | — |
| Projéteis | `pierce: false`: param no primeiro alvo e somem | idem |

`Miss` e dano 0 não têm efeito nos dois lados. Fórmula (defesa física e
mágica, resistências, crítico, erro, dano mínimo): `tests/combat_tests.rs`.

### 12.2 Política de dano simultâneo (etapa 6)

Recurso global `combat::hitboxes::SimultaneousHitPolicy`, iniciado com
`Legacy` pelo `CombatPlugin` (mesma escolha de camada do §6).

| Política | Jogador | Inimigo | Contato |
|---|---|---|---|
| `Legacy` (padrão) | primeiro acerto com dano | soma | um por quadro |
| `HighestDamagePerFrame` | só o mais forte do quadro | só o mais forte do quadro | um candidato por inimigo encostado |

**Onde agrega:** no consumidor de cada alvo (`receive_damage`), porque só
ele conhece o dano **efetivo** (a guarda do jogador é aplicada ali). Os
produtores mantêm a fórmula; o único descarte anterior ao consumidor, o
`break` do contato, só vale em `Legacy`. A mesma função
(`SimultaneousHitPolicy::select`) serve aos dois consumidores; a regra
pura é `combat::simultaneous::strongest_per_target`.

**Regras de agregação**

1. Grupo = um alvo (`DamageDealt::target`) num quadro (as mensagens lidas
   por aquele `receive_damage` naquele `Update`). Alvos e quadros nunca se
   misturam.
2. Só competem acertos com dano efetivo > 0 (`Miss` e 0 nunca vencem).
3. Comparação pelo **dano efetivo**: o `amount` já calculado (defesa,
   resistência, crítico e variação incluídos) e, no jogador, depois da
   guarda. Um crítico só vence se causar mais.
4. Desempate determinístico: intensidade de recuo maior → `attacker` com
   menor `Entity::to_bits()` → mensagem anterior.
5. O vencedor é aplicado **uma vez** pelo caminho normal: dano, invulnerabilidade
   (iniciada uma vez), morte ou `Hurt` + recuo **do próprio vencedor**. A
   comparação acontece antes de aplicar, então a invulnerabilidade iniciada no
   quadro não impede a comparação; uma invulnerabilidade já ativa no início do
   quadro descarta todos.
6. Ordem dos `GameSet`s inalterada; nenhum sistema novo.

### 12.3 Recuo por intensidade e limiar (etapa 7)

**Modelagem escolhida: campo novo em `DamageDealt`.** Comparação:

| Opção | Por que não / por que sim |
|---|---|
| Componente auxiliar no alvo ou na hitbox | o consumidor só vê a mensagem; projéteis e golpes sem `pierce` já sumiram |
| Recurso de contexto | não liga a intensidade a um acerto específico (quebraria a etapa 6) |
| Mensagem paralela (`HitFeel`) | exigiria casar duas mensagens por acerto |
| **Campo `knockback` em `DamageDealt`** | menor complexidade: viaja com o acerto; os construtores literais (6, em testes e no exemplo) ganharam `knockback: Default::default()`, edição mecânica |

Na hitbox, a intensidade é o componente opcional `HitKnockback`, inserido
pelo golpe básico só quando a classe da arma pede algo diferente de
`Medium`; `detect_hits` copia para a mensagem (sem ele, `Medium`). Contato,
golpes de inimigos, técnicas e projéteis usam `Medium`.

| Intensidade | Jogador (x, y) | Inimigo (x) |
|---|---|---|
| `Light` | × `KNOCKBACK_LIGHT_SCALE` (0,5): (110, 130) | 80 |
| `Medium` (padrão) | (220, 260), como antes | 160, como antes |
| `Heavy` | × `KNOCKBACK_HEAVY_SCALE` (1,5): (330, 390) | 240 |

**A intensidade é declarada pela fonte do golpe** (classe da arma); a
fórmula de dano não foi alterada nem consultada para decidir a intensidade.

**Limiar (`stagger_threshold`)**: `PlayerStagger(StaggerThreshold)`, fração
do HP máximo do jogador, padrão 0. Um acerto com dano aplicado
`≥ limiar × HP máximo` entra em `Hurt` com recuo (igual ao limiar também
derruba); abaixo dele o dano e a invulnerabilidade continuam, mas não há
`Hurt` nem recuo, e a ação em curso (golpe, magia) continua. Golpe fatal
mata sempre. Valores fora de `[0, 1]` são recusados por `StaggerThreshold::new`.
Inimigos não têm limiar no B2: a reação deles (flash, *wind-up*) é a etapa 8.

Garantias da reação: duração de `Hurt` e invulnerabilidade inalteradas;
morte zera a velocidade x; esquiva (sem `Hurt`), guarda (sem `Hurt`) e
morte respeitadas por `PlayerStateMachine::hurt`; ataque e magia são
interrompidos como antes; a transformação recebe o mesmo recuo e continua
ativa; o recuo é velocidade e passa pela física normal (paredes param o
corpo, testado com `Heavy` contra a parede oeste).

### 12.4 Interação entre as etapas 6 e 7

O acerto escolhido traz **o seu** dano e **o seu** recuo; nada é somado nem
emprestado de outro acerto. Com limiar, quem decide o *stagger* é só o
acerto escolhido (um acerto fraco com `Heavy` não empresta o recuo ao
escolhido). Na política `Legacy`, a combinação é a de sempre: o primeiro
acerto com dano, com o recuo dele.

### 12.5 Novos campos, recursos e padrões

| Item | Onde | Padrão (= antes) |
|---|---|---|
| `WeaponClass::knockback`, `StrikeProfile::knockback` | `inventory/weapons.rs` | `Medium` |
| `DamageDealt::knockback` | `core/events.rs` | `Medium` em todos os emissores |
| `HitKnockback` (componente) | `combat/hitboxes.rs` | ausente |
| `SimultaneousHitPolicy` (recurso) | `combat/hitboxes.rs`, `CombatPlugin` | `Legacy` |
| `PlayerStagger` (recurso) | `player/systems.rs`, `PlayerPlugin` | `StaggerThreshold::ALWAYS` (0) |
| `KNOCKBACK_LIGHT_SCALE`, `KNOCKBACK_HEAVY_SCALE` | `core/constants.rs` | 0,5 e 1,5 (só usados fora do padrão) |

Para ligar: insira o recurso no app (como os testes fazem) ou troque o
padrão do enum; para uma arma, `knockback: Heavy` na classe do RON (não
feito: decisão de balanceamento do proprietário).

### 12.6 Casos de borda e limitações

1. Com `HighestDamagePerFrame`, as hitboxes dos acertos descartados contam o
   alvo como atingido (`already_hit`/`RehitTimers`) e projéteis descartados
   somem: o acerto foi "gasto", só não aplicado.
2. A apresentação lê as mensagens brutas: com `HighestDamagePerFrame`, os
   números de dano mostram também os candidatos descartados (a faísca da
   guarda já é uma por quadro). Inofensivo enquanto a política estiver
   desligada; filtrar a UI é recomendado antes de ligá-la.
3. A comparação do jogador usa o estado do início do quadro (guarda); como só
   um acerto é aplicado, não há diferença com o estado durante a aplicação.
4. O desempate por `Entity::to_bits()` é estável numa partida, mas não tem
   significado de jogo (dano e recuo iguais tornam a escolha irrelevante para
   o resultado, exceto a direção do recuo).
5. Sem limiar para inimigos (etapa 8) e sem intensidade em técnicas,
   projéteis e golpes de inimigos (exigiria campos em `SkillDefinition` e
   `EnemyDefinition`; mesma razão do §6).
6. A intensidade só muda a velocidade do recuo; a duração de `Hurt` é a mesma
   para as três.

**Pré-requisitos para ligar `HighestDamagePerFrame`** (não ligada: o padrão
continua `Legacy`). Os itens 1 e 2 estão travados por testes
(`limitation_*` em `tests/damage_reaction_tests.rs`). Se um desses testes
falhar, a limitação foi resolvida e o teste deve ser atualizado.

- **Números de dano:** a UI (`ui::damage_numbers`) lê as mensagens brutas e
  mostra também os acertos descartados. Antes de ligar a política, a
  apresentação precisa ler só os acertos aplicados, por exemplo com uma
  mensagem de "acerto aplicado" emitida pelo `receive_damage`.
- **Hitboxes e projéteis gastos:** `detect_hits` registra o alvo
  (`already_hit`/`RehitTimers`) e remove a hitbox sem `pierce` no momento do
  acerto, antes da seleção. Um projétil ou golpe descartado some sem efeito.
  Antes de ligar, decida se esse gasto é aceitável ou mova o consumo para
  depois da seleção.
- **Comportamento atual:** com `Legacy`, nenhum inimigo descarta acertos (um
  número por acerto aplicado), e o jogador descarta pela invulnerabilidade
  como antes da V2.4. Nada muda enquanto a política estiver desligada.

### 12.7 Testes do B2

| Arquivo | O que trava |
|---|---|
| `tests/damage_characterization_tests.rs` (12, passam também no código anterior) | inimigo soma, sem invulnerabilidade, recuo fixo e pela última direção, flash e *wind-up*; `Miss`/0 sem efeito; morte uma vez; hitbox de golpe = dano relatado, um acerto; duas hitboxes inimigas no jogador = só a primeira; contato um por quadro e nada com invulnerabilidade; recuo do jogador fixo; guarda −70 % sem `Hurt` com invulnerabilidade; golpe fatal sem recuo; projétil para no primeiro alvo |
| `tests/damage_reaction_tests.rs` (27) | padrões; jogador e inimigo `Legacy` × `HighestDamagePerFrame`; alvos e quadros separados; desempate; crítico × normal; físico × fogo por hitboxes reais (resistência); guarda na comparação; invulnerabilidade prévia; quadro letal; contato com dois inimigos; recuo leve/médio/forte no jogador e no inimigo; abaixo, no e acima do limiar; fatal abaixo do limiar; durante ataque e magia; parede; `Hurt` termina como antes; combinação dano + recuo do mesmo acerto; limiar com a política nova; limitações de §12.6 (números de descartados, hitbox gasta, `Legacy` sem descarte) |
| `tests/combat_feel_tests.rs` (+2) | Iron Sword = `Medium` sem `HitKnockback`; `knockback` da classe chega aos acertos |
| `tests/world_transform_tests.rs` (+1) | forma espectral recebe o recuo `Heavy` e continua ativa |
| `src/combat/reaction.rs` (5), `src/combat/simultaneous.rs` (3) | escalas, ordem, limiar nas bordas e validação; escolha por alvo, misses, desempates |

Resultados ao fim do B2: `cargo fmt --all -- --check` ok;
`cargo check -j 2` ok; `cargo clippy --all-targets -j 2 -- -D warnings` ok;
`cargo test -j 2`: **534 testes, 0 falhas** (484 antes + 50: 130 de
unidade, 46 em `combat_feel_tests`, 12 de caracterização, 27 de reação, 27 em
`world_transform_tests`); `cargo run --example autoplay`: PASS 23/23, saves
reais intactos.
