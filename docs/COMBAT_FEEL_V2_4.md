# V2.4 — Combat Feel: linha do tempo, cancelamento, controle e reacerto

Plano: [`PLANO_V2_4_COMBAT_FEEL.md`](PLANO_V2_4_COMBAT_FEEL.md).

| Bloco | Etapas | Situação |
|---|---|---|
| A | 0 caracterização, 1 linha do tempo, 2 ações por fase, 3 cancelamento | concluído |
| B1 | política de hitbox no cancelamento, 4 controle aéreo e orientação, 5 reacerto | concluído |
| — | 6 a 10 | não iniciadas |

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

- Save V3: nada novo é salvo (`ActiveStrike`, `PendingStrike`, `SideLock` e
  `RehitTimers` são transitórios); nenhum arquivo de `src/save/` mudou.
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
