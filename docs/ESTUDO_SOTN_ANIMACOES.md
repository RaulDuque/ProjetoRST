# Estudo SOTN — animações e sensação de combate

Fase 2 da pesquisa. Convenções: [`REFERENCIAS_SOTN.md`](REFERENCIAS_SOTN.md).
Caminhos `src/...` do **sotn-decomp @ `3a0957b`**; os nossos, do **DCR @
`9f7616d`**. Legenda: **[F]** fato · **[I]** inferência · **[R]**
recomendação · **[?]** pendente.

---

## 0. Resumo

1. No SOTN, **a animação é o relógio do gameplay**. Tabelas de
   `{duração, pose}` avançam um quadro por vez, e a lógica dispara eventos em
   **poses exatas** (projétil na pose 4, liberação do golpe em
   `lockDuration`). A hitbox também vem do quadro. **[F]**
2. O **congelamento de impacto existe, mas só no alvo**: `stunFrames` faz o
   laço de atualização **pular** a entidade atingida (sem IA, animação ou
   movimento). Não há pausa global por golpe nem congelamento do jogador.
   **[F]**
3. Flash de dano por paleta durante 15 quadros, tremor de câmera em ataques
   grandes, som no início do estado e em poses-chave. **[F]**
4. O DCR usa **temporizadores de gameplay** e estica a arte sobre eles
   (`player/visual/playback.rs::action_frame`). A recomendação da Fase 1
   (manter temporizadores) **continua válida**, com um complemento: um
   **modelo híbrido** em que o manifesto da arte declara marcas opcionais
   que servem para validar e alinhar, sem governar o gameplay (§4).

---

## 1. Como o SOTN coordena cada aspecto

| Aspecto | Mecanismo | Evidência |
|---|---|---|
| Animação | `AnimateEntity(tabela, self)` avança `{duração, pose}`; retorna 0 no fim | `e_medusa.c:16-22, 72-75` (tabelas e uso) |
| Eventos de animação | `if (!self->poseTimer && self->pose == N)` → cria projétil, toca som, liga ataque | `e_medusa.c:157-170, 180-183`; `e_scylla.c:302-305` |
| Movimento | Velocidade por estado; animação e movimento correm juntos no mesmo quadro | `e_medusa.c:93-99`; jogador em `71830.c` |
| Colisão | Caixa por quadro de animação (tabela indexada por `animCurFrame`) | `e_medusa.c:271-277`; `alu_anim.c:191-214`; `w_008.c:60-65` |
| Ataque | A arma é entidade própria, criada no início do golpe; o ataque só fica ofensivo em quadros marcados | `e_medusa.c:394-398`; Fase 1 §1.2 |
| Recuperação | Parte final da animação do golpe; termina com `poseTimer < 0` | `71830.c` (Fase 1 §1.4) |
| Cancelamento | Pose ≥ `lockDuration` libera o bloqueio; a máscara de transições decide o resto | `6FDF8.c:105-145`; Fase 1 §1.5 |
| Recuo | Estado `Player_Hit` com velocidades por intensidade | `71830.c:1237-1644` |
| Invulnerabilidade | Temporizadores do jogador; nos inimigos, `unk6D[]` por atacante | `692E8.c:887-907`; `collision.h:551-570` |
| Efeitos visuais | Flash: os 4 bits baixos de `flags` contam 15 quadros alternando paletas e depois restauram | `collision.h:565-566`; `st_update.h:95-103` |
| Efeitos sonoros | Som ao entrar no estado (`step_s == 0`) e em poses-chave; som posicional | `e_medusa.c:128-135, 145-148, 166-169` |
| Congelamento de impacto | `stunFrames > 0` → `continue` no laço: a entidade não roda no quadro | `st_update.h:122-127`; `collision.h:560-562` |
| Pausa global | `pauseEnemies` só em cenas roteirizadas e no relógio (subarma) | `692E8.c:830-845`; `6E42C.c:288` |
| Tremor | `ShakeCamera(n)` em ataques de área | `e_scylla.c:648` |
| Transições | `SetStep`/`SetSubStep` zeram `pose` e `poseTimer` (a animação recomeça em toda troca de estado) | `e_hippogryph.c:177-183` |

**[I]** Como o atordoamento congela o alvo **inclusive na animação**, ele é
o *hitstop* do SOTN, mas só de um lado: o jogador continua se movendo, e
o inimigo fica parado no quadro em que foi atingido. Daí vem a sensação de
"golpe que prende".

---

## 2. O que o DCR faz hoje

| Aspecto | DCR | Evidência |
|---|---|---|
| Relógio | Temporizadores em segundos na máquina de estados | `src/player/state.rs` (`tick`) |
| Arte | Clipe esticado sobre a ação: `action_frame(durações, progresso)` mantém as proporções autorais | `src/player/visual/playback.rs:25-51` |
| Seleção | O visual segue o estado; nunca o muda | `src/player/visual/selection.rs:1-17` |
| Âncoras | `player_sockets.ron` por quadro (arma, mão, cabeça, peito, costas); opcional | `docs/VISUAL_EQUIPMENT_V2_2.md` §5 |
| Hitbox | Vem de `WeaponClass`, independente da arte | `src/inventory/weapons.rs:34-57` |
| Efeitos | VFX por chave nos dados; "só observam o gameplay" | `src/player/visual/vfx/rules.rs:1-7` |
| Flash do inimigo | Branco por `ENEMY_HURT_FLASH` (0,15 s); **continua se movendo** com o knockback | `src/enemies/behavior.rs:76-79, 269-276` |
| Jogador invulnerável | Pisca | `src/player/animation.rs:69` |
| Números de dano | Sim | `src/ui/damage_numbers.rs` |
| Som | **Nenhum** (o recurso `audio` do Bevy está ligado no `Cargo.toml`, mas nenhum sistema toca som) | busca por `AudioPlayer`/`AudioSource` sem resultado |
| Tremor de câmera | **Nenhum** | busca por `shake` sem resultado |
| Congelamento de impacto | **Nenhum** | idem |

---

## 3. Avaliação crítica da recomendação "manter temporizadores"

### 3.1 Opção A — temporizadores como fonte da verdade (atual)

| Vantagens | Desvantagens |
|---|---|
| Balanceamento em dados (`weapon_classes.ron`), em segundos, sem depender da arte | A arte pode **dessincronizar**: o quadro de contato desenhado não coincide com o início da hitbox |
| A arte **é provisória** (V2.2, `VISUAL_EQUIPMENT_V2_2.md` §15); trocar a arte não muda o jogo | Ninguém "vê" o tempo ativo ao desenhar; ajustar exige tentativa e erro |
| `attack_speed` escala a ação inteira trivialmente | O esticamento uniforme distorce a antecipação e a recuperação quando o tempo muda muito |
| Testes headless rodam **sem sprites** (o jogo funciona sem manifesto) | Eventos de efeito (som, VFX) não têm um quadro "oficial" para disparar |

### 3.2 Opção B — animação como relógio (modelo SOTN)

| Vantagens | Desvantagens |
|---|---|
| Sincronia perfeita entre arte, hitbox e eventos | O gameplay **muda toda vez que a arte muda**, e a arte é provisória |
| O artista controla a sensação diretamente | Testes passariam a depender do manifesto e dos quadros |
| Eventos por quadro naturais (som, VFX, projétil) | `attack_speed` exigiria pular ou repetir quadros |
| | Sem sprites (headless, CI), não haveria relógio: exigiria um caminho paralelo |
| | Mudança grande em `player::state`, `combat` e no pipeline de arte |

**Risco de B:** alto. Contraria a separação "visual observa gameplay", que
hoje está documentada e testada.

### 3.3 Opção C — híbrido: dados mandam, arte declara marcas (recomendada)

1. A **linha do tempo do golpe** fica em `WeaponClass` (segundos:
   `startup`, `active_fraction`, `cancel_after`), como no plano V2.4.
2. O manifesto do clipe ganha **marcas opcionais** por quadro
   (`contact`, `active_end`, `cancel`), sem efeito no gameplay.
3. Um **teste de consistência** (ou relatório no `autoplay`) compara as
   marcas da arte com as fases dos dados e avisa se a diferença passar de
   uma tolerância. Isso transforma a dessincronia em erro visível.
4. O **esticamento por fase**: em vez de esticar o clipe inteiro de forma
   uniforme, `action_frame` estica **cada trecho** (antecipação, ativo,
   recuperação) para a fase correspondente. Assim o quadro de contato
   desenhado sempre cai no início da hitbox, qualquer que seja a
   velocidade. Sem marcas, o comportamento atual se mantém.
5. VFX e (futuramente) som disparam **na transição de fase** do gameplay,
   não em quadros da arte.

| Vantagens | Desvantagens / riscos |
|---|---|
| Mantém dados como fonte da verdade e os testes headless | Mais um campo opcional no manifesto (precisa de versão/compatibilidade) |
| Sincronia visual garantida pelo esticamento por fase | O esticamento por fase precisa de testes próprios em `playback.rs` |
| Dessincronia vira aviso automático | Ferramenta de checagem a manter |
| Arte final pode chegar depois sem mudar código | — |

**Conclusão:** a recomendação da Fase 1 se mantém. O V2.4 deve fazer **só o
passo 1** (fases nos dados) e preparar `action_frame` para receber fases. As
marcas e o esticamento por fase vêm junto com a arte nova (posterior),
porque com a arte atual não há quadro de contato para marcar.

---

## 4. Sensação de impacto: propostas originais

| Proposta | Inspiração SOTN | Desenho para o DCR | Padrão | Versão |
|---|---|---|---|---|
| **Congelamento do alvo** | `stunFrames` pula a atualização | Durante o atordoamento, o inimigo **para** (velocidade zero e sem IA), e o knockback é aplicado **ao fim** do congelamento ou em parte. Hoje ele desliza durante o flash | Desligado (comportamento atual) | V2.4, etapa de reação do inimigo |
| **Micro-pausa do atacante** | Não existe no SOTN | Temporizador da ação do jogador pausa N ms ao acertar (só a ação, não o mundo) | `0` | Opcional no V2.4 (último passo) ou posterior |
| **Tremor de câmera** | `ShakeCamera` | Recurso pequeno em `world::camera` com intensidade/duração; só apresentação; desligável em `camera.ron → effects` | Desligado | Posterior (chefes) |
| **Flash por fases** | Paleta alternada 15 quadros | Alternar branco e cor base em vez de branco fixo | Atual | Posterior (arte) |
| **Som** | Som no início do estado e em eventos | Módulo `audio` observando mensagens (`DamageDealt`, início de ação). Precisa de assets próprios | — | Posterior (decisão de assets) |

**[R]** Não usar `Time<Virtual>` para pausa global: congelaria UI, câmera e
partículas e afetaria o reconhecedor de comandos (janelas em segundos).
Congelamento **local** por entidade é mais seguro.

---

## 5. Testes que essas mudanças exigiriam

- `playback.rs`: com fases, o quadro de contato cai em `startup`; sem fases,
  resultado idêntico ao atual (teste de regressão com as durações reais).
- ECS: inimigo atordoado não se move nem decide; ao fim, retoma.
- ECS: micro-pausa com padrão 0 não muda a duração da ação.
- Consistência arte × dados (quando houver marcas): tolerância configurável.

---

## 6. Pendências

- Lista completa de sons por ação do jogador no SOTN: não levantada. **[?]**
- Valores de `stunFrames` por arma: **não estão no repositório** (dados do
  disco). Ver [`ESTUDO_SOTN_EQUIPAMENTOS.md`](ESTUDO_SOTN_EQUIPAMENTOS.md) §0.
