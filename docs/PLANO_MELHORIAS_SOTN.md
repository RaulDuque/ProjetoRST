# Plano de melhorias inspiradas no SOTN

Documento de **planejamento**. Nada aqui está implementado. Evidências e
detalhes: [`ESTUDO_SOTN_COMBATE.md`](ESTUDO_SOTN_COMBATE.md),
[`ESTUDO_SOTN_ATRIBUTOS.md`](ESTUDO_SOTN_ATRIBUTOS.md),
[`REFERENCIAS_SOTN.md`](REFERENCIAS_SOTN.md). Base: DCR @ `9f7616d`
(branch `art/v1.2-protagonist`), sotn-decomp @ `3a0957b`.

> **A V2.4 não deve começar sem aprovação explícita do proprietário**
> (`Md-usario/Md-usario/DarkCastleR-Estudo-tecnico.md`). Este plano é
> insumo para essa decisão, não uma ordem de serviço.

> **Atualização (Fase 2 e consolidação):** o proprietário aprovou a
> **direção** "V2.4 = Combat Feel / Peso do Golpe" (`DECISIONS.md`, D-028).
> O plano executável é [`PLANO_V2_4_COMBAT_FEEL.md`](PLANO_V2_4_COMBAT_FEEL.md),
> que incorpora o pacote P1 abaixo e **prevalece** sobre ele em nomes de
> campos e ordem. A implementação ainda depende de autorização específica.
> Correções desta tabela estão marcadas como "Fase 2".

Regras que valem para toda melhoria abaixo:

- **Ideias, não código:** nenhuma linha do sotn-decomp (AGPL-3.0) ou do
  SymphonyRecomp (sem licença) entra no DCR. Implementações são próprias,
  com números próprios.
- **Comportamento atual como padrão:** campos novos em RON usam
  `#[serde(default)]` com o valor que reproduz o jogo de hoje. A Iron Sword
  e o `bone_sentry` não mudam até que alguém altere os dados de propósito.
- **Regra pura primeiro**, com testes, depois o sistema Bevy (padrão do projeto).
- Toda escolha de balanceamento vira uma entrada `D-0xx` em `DECISIONS.md`.
- Commits atômicos (um por melhoria/teste/documento), pela skill `/atomic-commit`.

Prioridades: **P1** = próximo pacote de combate (maior ganho de sensação);
**P2** = pacote seguinte; **P3** = futuro/opcional; **—** = nada a fazer.
Dificuldade e risco: **B**aixo, **M**édio, **A**lto.

---

## 1. Tabela comparativa

### 1.1 Combate

| Sistema | Evidência SOTN | DCR atual | Diferença | Melhoria sugerida | Dif. | Risco | Prior. | Classificação |
|---|---|---|---|---|---|---|---|---|
| Máquina de estados | `player_steps.h`; `692E8.c:618+` | `player/state.rs` (12 estados, pura) | Ataque é sub-estado da locomoção no SOTN | Fase de ataque + postura no início do golpe | M | M | P1 | Implementado, mas melhorável |
| Ataques normais | `6E42C.c:372-841` | `combat/attacks.rs::basic_attack` + `StrikeProfile` | Sem variante por postura | Ver aéreo/agachado | — | — | — | Implementado adequadamente |
| Artes de arma | `6E42C.c:464-494` | `skills/` + RON + reconhecedor | DCR mais geral | `techniques` por arma (opcional) | B | B | P3 | Implementado adequadamente |
| Recuperação | `71830.c:365-431` (`lockDuration`) | `action_time` todo travado | Sem *startup*, recuperação não cancelável | Linha do tempo por `WeaponClass` | M | M | P1 | Implementado, mas melhorável |
| Cancelamento | `6FDF8.c` (máscara) | Só técnica cancela `Attacking` | Sem ponto de cancelamento | `allowed_actions` + `cancel_after` | M | M | P1 | Parcial |
| Controle durante ataque | `71830.c:356-404, 696-701` | Chão freia; ar sem controle | SOTN tem controle aéreo pleno | Controle aéreo; `turn_window` | B | B | P1 | Parcial |
| Hitboxes | `alu_anim.c:191-214` | `Hitbox` separada, forma fixa | Forma não varia no tempo | `shape_by_phase` opcional | M | M | P3 | Implementado, mas melhorável |
| Hurtboxes | `frameProps` | Fixa no corpo inteiro | Não encolhe ao agachar | Hurtbox de agachado | B | B | P1 | Parcial |
| Frames ativos | `e_bone_scimitar.h:150-161` | 60% do swing desde o quadro 0 | Acerta no 1º quadro | Coberto pela linha do tempo | — | — | P1 | Implementado, mas melhorável |
| Invulnerabilidade | `692E8.c:887-907, 1255-1262` | 0,9 s fixos | SOTN ≈ recuo + 0,2 s | `hurt_time + cauda` (depois dos tiers) | B | M | P2 | Implementado, mas melhorável |
| Recuo ao receber dano | `5D5BC.c:593-759`; `71830.c:1237-1644` | Um recuo para todo dano | SOTN: 3 intensidades por dano/HP | `hurt_reaction` com tiers | M | B | P1 | Implementado, mas melhorável |
| Golpe fraco não derruba | `HandleDamage` (DEF ≥ 100), tier leve | Todo dano tira do controle (exceto guarda) | — | `stagger_threshold` | B | B | P1 | Ausente |
| Knockback/hitstun dos inimigos | `collision.h:525-562` | `ENEMY_KNOCKBACK`/`HURT_FLASH` constantes | Sem valor por arma | `stun`/`knockback` por arma; resistência por inimigo | M | M | P1 | Implementado, mas melhorável |
| Reacerto do mesmo alvo | `collision.h:555-559` | `already_hit` (uma vez por hitbox) | Sem multi-hit temporizado | `rehit_interval` opcional | M | M | P1 | Parcial |
| Combos | `71830.c:367-380` (cancelar em novo ataque) | Nenhum (ROADMAP etapa 3) | — | Cancelar recuperação com novo ataque; `combo_chain` depois | M | M | P2 | Ausente |
| Ataques aéreos | `6E42C.c:710-723` | Mesmo golpe do chão | Sem forma própria | `air: Option<StrikeShape>` | B | B | P2 | Parcial |
| Ataques agachados | `6E42C.c:710-723` | Mesmo golpe do chão | Sem forma própria | `crouch: Option<StrikeShape>` | B | B | P2 | Parcial |
| Categorias de arma | `6E42C.c:563-839` | `WeaponClass` (5 classes) | — | Campos novos + `lunge` | B | B | P2 | Implementado adequadamente |
| Armas de duas mãos | não confirmado **[?]** (Fase 2: confirmado, `menu.c:3209-3216, 3301-3310`) | `two_handed` impede escudo | — | — | — | — | — | Implementado adequadamente |
| Escudos | `6E42C.c:764-788`; `w_008.c` | Guarda frontal 70% (constante) | SOTN: escudo é entidade, segurar botão | Redução vinda do escudo; quebra de guarda | M | M | P3 | Implementado, mas melhorável |
| Interrupções | `func_80113EE0` | `cancel_interrupted_hitboxes` | — | — | — | — | — | Implementado adequadamente |
| Prioridade entre ações | `692E8.c:887-1075`; `collision.h:366-376` | `GameSet`; primeiro dano do quadro vale | SOTN aplica o mais forte | Maior dano no quadro | B | B | P1 | Implementado, mas melhorável |

### 1.2 Atributos e fórmulas

| Sistema | Evidência SOTN | DCR atual | Diferença | Melhoria sugerida | Dif. | Risco | Prior. | Classificação |
|---|---|---|---|---|---|---|---|---|
| Atributos primários | `menu.c:176-260` | STR/CON/INT/LCK somados | SOTN tem pesos ocultos (CON ×8, INT ×4) | Não adotar pesos | — | — | — | Implementado adequadamente |
| Pesos ocultos de atributo | `menu.c:253-254` | — | — | — | — | — | — | Não recomendado |
| HP/MP/Hearts | `5D5BC.c:308-362, 482-505` | Fórmulas lineares em `attributes.rs` | SOTN cresce por itens de exploração | Itens de HP/MP máx em segredos | B | B | P2 | Implementado adequadamente |
| Ataque físico | `menu.c:262-322` | `STR·2 + mod`, × potência da classe | SOTN: arma domina, STR com retorno decrescente | Teto suave opcional | M | M | P3 | Implementado adequadamente |
| Defesa | `menu.c:334-419`; `HandleDamage` | Proporcional `100/(100+DEF)` | SOTN subtrativa | Manter a nossa | — | — | — | Implementado adequadamente |
| Defesa subtrativa | `HandleDamage` | — | — | — | — | — | — | Não recomendado |
| Ataque mágico | `5D5BC.c:200-206` | `INT·2 + mod`, × multiplicador | INT pesa mais no DCR | — | — | — | — | Implementado adequadamente |
| Elementos | `game.h:315-334`; `DealDamage` | 1 elemento, resistência contínua | SOTN: bits, vários por golpe | Elemento secundário (futuro) | A | A | P3 | Implementado adequadamente |
| Críticos | `5D5BC.c:530-548, 925-959` | Chance e multiplicador por LCK | LCK bem mais forte no DCR | Crítico por classe de arma; revisar peso de LCK | B | M | P2 | Implementado, mas melhorável |
| Crítico por atributo sorteado | `5D5BC.c:926-959` | — | — | — | — | — | — | Não recomendado |
| EXP | `5D5BC.c:420-445` | EXP fixa por inimigo | SOTN escala por diferença de nível | `level` opcional + `scaled_experience` | B | B | P2 | Implementado, mas melhorável |
| Level up | `5D5BC.c:308-362` | Automático fixo + 2 pontos | SOTN aleatório | — | — | — | — | Implementado adequadamente |
| Atributos aleatórios no level up | `5D5BC.c:336-356` | — | — | — | — | — | — | Não recomendado |
| Status | `5D5BC.c:209-254` | Tipos e resistências existem; aplicação não | — | Duração por CON; efeitos que mudam o jogo | M | M | P2 | Parcial |
| Drops | `5D5BC.c:978-1026` | `drop_chance` com LCK | LCK quase irrelevante no SOTN | — (ver revisão de LCK) | — | — | — | Implementado adequadamente |

### 1.3 Movimentação

| Sistema | Evidência SOTN | DCR atual | Diferença | Melhoria sugerida | Dif. | Risco | Prior. | Classificação |
|---|---|---|---|---|---|---|---|---|
| Andar/correr | `71830.c:557` | 190/310 instantâneos | SOTN não corre | — | — | — | — | Implementado adequadamente |
| Flutuação no ápice | `6FDF8.c:17-24` | Não existe | — | `APEX_GRAVITY_FACTOR` | B | B | P2 | Ausente |
| Corte do pulo | `71830.c:586-590` | `vy *= 0,45` | SOTN usa teto (`vy = −1`) | Corte por teto | B | B | P2 | Implementado, mas melhorável |
| Coyote time | `71830.c:755` | 0,08 s | Próximos | — | — | — | — | Implementado adequadamente |
| Cair de borda | `6E42C.c:131-149` | Mantém velocidade e controle | SOTN zera vx | Manter o nosso | — | — | — | Implementado adequadamente |
| Pouso pesado | `6FDF8.c:60-96` | Não existe | — | Só apresentação (VFX/som) | B | B | P3 | Ausente |
| Pulo duplo | `6E42C.c:182-190` | Só o nome `Ability::DoubleJump` em `world/graph.rs` | Sem física | Habilidade de exploração | M | M | P3 | Ausente |
| Esquiva/backdash | `6E42C.c:913-922` | Esquiva à frente, invulnerável, não cancelável | SOTN: para trás, cancelável, sem invencibilidade | Cancelável após fração; esquiva para trás opcional | B | M | P2 | Implementado, mas melhorável |

### 1.4 Magias e comandos

| Sistema | Evidência SOTN | DCR atual | Diferença | Melhoria sugerida | Dif. | Risco | Prior. | Classificação |
|---|---|---|---|---|---|---|---|---|
| Reconhecimento de comandos | `704D0.c` | `command_parser.rs` + buffer | DCR mais tolerante e por dados | — | — | — | — | Implementado adequadamente |
| Tempo de conjuração | `71830.c:2241-2310` | Efeito instantâneo (D-GAME-002 pendente) | SOTN: fases, efeito em pose fixa, invencibilidade | `effect_delay`, `cast_invulnerability` | M | M | P2 | Parcial |
| Cooldown × limite de instâncias | `5D5BC.c:256-265`; `6E42C.c:233-321` | Cooldown | SOTN não tem cooldown | `max_active` opcional | B | B | P3 | Implementado adequadamente |
| Aprender ao executar | `5D5BC.c:268-280` | `Acquisition::Discovery` = achar no mundo | Modo não existe | Modo para comandos secretos | B | B | P3 | Ausente |
| Transformação com MP contínuo | `5D5BC.c:790-843` | `mp_per_second` em `transformations.ron` | — | — | — | — | — | Implementado adequadamente |

### 1.5 Inimigos, chefes e exploração

| Sistema | Evidência SOTN | DCR atual | Diferença | Melhoria sugerida | Dif. | Risco | Prior. | Classificação |
|---|---|---|---|---|---|---|---|---|
| Espaçamento | `e_bone_scimitar.h:113-147` | `decide` sem faixa; `Retreat` sem uso | — | `preferred_range` com histerese | B | B | P2 | Parcial |
| Ritmo de ataque | `e_bone_scimitar.h:45-48, 169-177` | Cooldown único | — | `attack_rhythm` cíclico | B | B | P2 | Implementado, mas melhorável |
| Telegrafia | `e_bone_scimitar.h:150-161` | `ATTACK_WINDUP`/`ACTIVE` constantes | — | windup/active/recovery por inimigo em RON | B | B | P2 | Implementado, mas melhorável |
| Pular obstáculos | `e_bone_scimitar.h:180-215` | Não encontrado em `src/enemies/` | — | Sensor + pulo (com novos arquétipos) | M | M | P3 | Ausente |
| Respawn | `st_common.h:61-67` | Por tempo (`respawn_time`) | SOTN marca por sala **[?]** | Decidir "reaparece ao reentrar na sala" | M | M | P3 | Implementado, mas melhorável |
| ↳ *Correção (Fase 2)* | marca zerada a cada carga de sala (`5087C.c:1833-1834` e outras) | **Também ao reentrar** (`spawning.rs::enter_new_room`), além do tempo | Igual ao SOTN na reentrada; o DCR tem a mais o respawn por tempo | Decidir só `respawn_time: 0` como padrão de conteúdo | B | B | P3 | Implementado adequadamente (reentrada) |
| Colisão só na tela | `collision.h:243-246` | — | Otimização de PS1 | — | — | — | — | Não recomendado |
| Chefes | **não investigado** (Fase 2: [`ESTUDO_SOTN_CHEFES.md`](ESTUDO_SOTN_CHEFES.md)) | `defeated_bosses` em `world/progress.rs`; `BossReward` planejado | — | Pesquisar `src/boss/` antes | — | — | — | [?] não avaliado |
| Bestiário | `collision.h:404-409` | Não encontrado | — | Registro na primeira vez que acerta (P3) | B | B | P3 | Ausente |
| Salas, portas, mapa, câmera | **não investigado** (Fase 2: [`ESTUDO_SOTN_EXPLORACAO.md`](ESTUDO_SOTN_EXPLORACAO.md)) | Grafo validado, 4 regiões, mapa, câmera V2.3 | — | — | — | — | — | [?] não avaliado |

---

## 2. Pacotes propostos

### P1 — "Peso do golpe" (combate)

Ordem pensada para cada passo funcionar sozinho e ser commitado separado.

1. **Linha do tempo do golpe por classe de arma.**
   - Dados: `WeaponClass` ganha `startup`, `active`, `cancel_after`
     (`#[serde(default)]`; padrões = 0, 60% do swing, fim da ação).
   - Puro: `StrikeProfile` expõe as fases; função `strike_phase(profile, elapsed)`.
   - Bevy: a hitbox nasce em `startup` (sistema em `combat/attacks.rs`).
   - Arquivos: `src/inventory/weapons.rs`, `assets/data/items/weapon_classes.ron`,
     `src/combat/attacks.rs`, `docs/COMBAT_SYSTEM.md`, `docs/EQUIPMENT_SYSTEM.md`.
   - Testes: perfil padrão idêntico ao atual (Iron Sword); fases nas bordas;
     ECS: nenhum acerto antes de `startup`.
2. **Ações permitidas por fase (cancelamento).**
   - Puro: `allowed_actions(state, phase)` em `player/state.rs`.
   - Política inicial: após `cancel_after`, Jump/Dodge/Guard/Attack/técnicas.
   - Testes: tabela de transições; ECS: pular antes/depois do ponto.
3. **Recuo por intensidade + limiar de recuo.**
   - Puro: `hurt_reaction(damage, max_hp, stance) -> HurtReaction`; abaixo de
     `stagger_threshold` não há `Hurt`.
   - Constantes em `src/core/constants.rs`; aplicação em `player/systems.rs::receive_damage`.
   - Testes: bordas dos limiares; ECS: dano pequeno não tira do chão.
4. **Maior dano do quadro.** Em `receive_damage`, escolher a maior
   `DamageDealt` do quadro (ordem estável). Teste ECS: 5 e 30 → aplica 30.
5. **Reação do inimigo por arma.**
   - Dados: `stun`, `knockback` por classe/técnica; `knockback_resistance` por
     inimigo; `rehit_interval` opcional no `Hitbox`.
   - Mensagem `DamageDealt` ganha os campos opcionais.
   - Arquivos: `src/combat/hitboxes.rs`, `src/enemies/behavior.rs`,
     `assets/data/enemies/castle_undead.ron`, `docs/ENEMY_SYSTEM.md`.
   - Testes: resistência 1,0 não recua; machado atordoa mais que adaga;
     reacerto só após o intervalo.
6. **Controle aéreo no golpe e hurtbox de agachado.**
   - `movement.rs`: controle horizontal (velocidade de andar) em `Attacking` no ar.
   - `PLAYER_CROUCH_HURTBOX` e sistema pequeno no módulo `player`.
   - Testes ECS: mover no ar atacando; golpe alto erra o agachado.

Ao fim do P1: rodar `/test-and-validate` e o `autoplay`, comparar
screenshots, e registrar `D-0xx` com os números escolhidos.

### P2 — Movimento, magia, inimigos, atributos

- **Movimento:** flutuação no ápice; corte do pulo por teto; esquiva cancelável.
- **Invulnerabilidade:** `hurt_time + cauda`, só depois dos tiers do P1.
- **Combos:** cancelar recuperação com novo ataque (já sai do P1.2); cadeia
  `combo_chain` só se ainda faltar ritmo.
- **Golpes aéreos/agachados:** `air`/`crouch: Option<StrikeShape>`.
- **Magia (D-GAME-002):** `effect_delay`, `cast_invulnerability` (padrão 0).
  Decidir se o custo é cobrado antes (como no SOTN) ou no efeito.
- **Inimigos:** `preferred_range` (usa `Retreat`), `attack_rhythm`, fases em RON.
- **Atributos:** `scaled_experience`; crítico por classe de arma; revisão
  do peso de LCK com a ferramenta de tempo-para-matar (ROADMAP etapa 14);
  itens de HP/MP máx; durações de status por CON quando os status forem aplicados.

### P3 — Futuro/opcional

`shape_by_phase`; técnicas por arma; escudo com redução própria e quebra
de guarda; pouso pesado; pulo duplo; `max_active`; aprender ao executar;
pulo de obstáculo nos inimigos; respawn por sala; bestiário; teto suave do
ATK da arma; elemento secundário.

---

## 3. Riscos arquiteturais

1. **Tempo × animação.** O SOTN dirige o gameplay pelo quadro de animação.
   O DCR usa temporizadores e arte separada (manifest, sockets). **Manter
   temporizadores** como fonte da verdade; a arte segue o tempo. Senão,
   trocar arte muda o balanceamento.
2. **Compatibilidade de dados e saves.** Todos os campos novos são
   opcionais com padrão; nada entra no save (estado de combate é transitório).
   Se algo precisar persistir (bestiário, respawn por sala), exige versão de
   save nova e decisão (D-024 é o precedente).
3. **Regressão de sensação.** Mudar `startup`/invulnerabilidade altera a
   dificuldade. Cada mudança numérica separada da mudança de mecanismo
   (commit do mecanismo com padrões neutros; commit de dados depois).
4. **Crescimento do `PlayerStateMachine`.** Fases e máscara de ações devem
   ficar puras e pequenas; não transformar em "deus-objeto". Se crescer,
   extrair `player/action_rules.rs`.
5. **Ordem dos `GameSet`s.** Hitbox atrasada e "maior dano do quadro" mexem
   em `Combat → Resolution`; testar com o harness ECS headless existente.
6. **Licença.** Implementar a partir das descrições destes documentos, sem
   abrir o código externo ao escrever o nosso; não copiar tabelas numéricas.
7. **Docs.** Atualizar `COMBAT_SYSTEM.md`, `ENEMY_SYSTEM.md`,
   `EQUIPMENT_SYSTEM.md`, `CHARACTER_ATTRIBUTES.md` e `ARCHITECTURE.md`
   quando a arquitetura mudar (regra 20).

---

## 4. Decisões que dependem do proprietário

- Aprovar (ou não) o início da V2.4 e se ela é o pacote P1.
- Invulnerabilidade pós-dano: manter 0,9 s ou adotar `recuo + cauda`.
- Esquiva: continuar para frente, ganhar variante para trás, ser cancelável?
- Custo de magia com atraso: cobrar ao confirmar ou ao sair o efeito.
- Respawn: por tempo (hoje) ou ao reentrar na sala. *(Fase 2: o DCR já faz
  os dois; resta decidir se o respawn por tempo continua no conteúdo.)*
- Peso de LCK (crítico, drops, HIT, esquiva) — rever ou manter.

---

## 5. Validação esperada de cada pacote

`cargo fmt --check`, `cargo clippy --all-targets`, `cargo test`, `cargo check`
e `cargo run --example autoplay`, um comando Cargo por vez, conforme
`CLAUDE.md` ("Compilação segura"). Os testes novos citados em cada item
precisam existir antes de considerar o item concluído.

---

## 6. Pendências de pesquisa

Não investigado ou não confirmado nesta sessão (não tratar como concluído):

1. **Chefes** (`src/boss/`): padrões, fases, telegrafia, arenas.
2. **Salas, portas, transições, mapa e câmera** do SOTN.
3. **Overlays de arma** (`src/weapon/w_*.c`): números de `stunFrames`,
   `enemyInvincibilityFrames`, `lockDuration` e `criticalRate` por arma
   (tabelas de dados não lidas); confirmação de duas mãos (`w_051.c`) e do
   bloqueio pelo escudo (`w_008.c`).
4. **Quando zera a marca de "não reaparecer"** (`D_80097428`) no PS1.
5. **Petrificação:** como o jogador sai do estado.
6. **Medição em execução:** nenhum valor foi medido rodando o jogo; todos
   vêm da leitura do código decompilado (pode haver diferenças de versão
   regional/PSP marcadas com `#if` no decomp).

Como continuar: [`REFERENCIAS_SOTN.md`](REFERENCIAS_SOTN.md) §7.

> **Atualização (Fase 2):** itens 1, 2, 4 e 5 resolvidos; item 3 parcial
> (duas mãos confirmado, escudo parcial, valores por arma fora do
> repositório); item 6 continua (nenhuma medição em execução).
> Classificação completa em
> [`PROPOSTA_ROADMAP_POS_V2_4.md`](PROPOSTA_ROADMAP_POS_V2_4.md) §3.
