# Registro de decisões

## D-001 — Bevy 0.20.1 com features mínimas
Versão estável mais recente no crates.io em 2026-11-11 (MSRV 1.97.1).
Usamos `default-features = false` com `2d`, `ui` e `audio`: o jogo não tem 3D,
então evitamos compilar PBR, glTF etc. Versões fixadas com `=` no `Cargo.toml`
e `Cargo.lock` versionado.

## D-002 — Dependências diretas
`serde 1.0.229`, `ron 0.12.3`, `thiserror 2.0.21`, `tracing 0.1.44` — as
mesmas versões que o próprio Bevy 0.20.1 usa (sem duplicação no grafo).
`tracing` é usado diretamente nos módulos puros para não depender de Bevy.
**Não adicionados:** `bevy-inspector-egui` (0.38 não foi validado com Bevy 0.20 e
não é necessário ainda), crates de tilemap (salas simples em RON bastam),
`rand` (um SplitMix64 de 20 linhas em `core/rng.rs` é suficiente e determinístico).

## D-003 — RON para dados e saves
Formato nativo do ecossistema Rust/Bevy, legível, com enums e structs.
Comandos de habilidades usam uma string de notação (`"D DF F + ATTACK"`, ou
setas `"↓ ↘ → + ATTACK"`) convertida por `TryFrom<String>` — erros de digitação
são detectados no carregamento e no `cargo test` (`tests/data_tests.rs`).

## D-004 — Regras puras separadas dos sistemas Bevy
Fórmulas, parser, progressão, inventário e save não importam Bevy. Ganhos:
testes rápidos e determinísticos, regras reutilizáveis em ferramentas, e
nenhuma lógica de negócio presa a sistemas de renderização.

## D-005 — Física própria (AABB) em vez de engine de física
Plataforma 2D precisa de colisão de caixas, plataformas atravessáveis e
resposta previsível. `world/collision.rs` resolve por eixo; uma engine de
física (avian/rapier) traria peso e comportamento menos "arcade".

## D-006 — Comandos relativos à orientação
O personagem vira para a direção pressionada **antes** da gravação do comando
(`update_facing` roda antes de `record_and_resolve_commands`). Assim "→" é
sempre "para frente" do ponto de vista do jogador. Consequência: "trás" só é
gravado quando o personagem não pode virar (atacando, guardando...).

## D-007 — Ajustes na estrutura de pastas do briefing
- `core/time.rs` contém só o *clamp* do delta (evita atravessar plataformas em travadas).
- Não criados (sem finalidade ainda): `combat/combos.rs`, `combat/combat_state.rs`
  (o estado de combate é a `PlayerStateMachine` em `player/state.rs`),
  `skills/components.rs` (o componente `SkillBook` está em `player/components.rs`),
  `magic/spells.rs` (magias usam `SkillDefinition`), `inventory/equipment_slots.rs`
  (slots ficam em `equipment.rs`), `world/transitions.rs`, `ui/menus.rs`,
  `ui/inventory_screen.rs`, `ui/skill_screen.rs` (a lista de habilidades está na tela de status).
- Adicionados: `input/` (mapeamento de teclas), `player/state.rs`, `world/{physics,camera,progress}.rs`,
  `enemies/definitions.rs`, `ui/{theme,damage_numbers}.rs`, `save/systems.rs`,
  `examples/autoplay.rs`, `tests/{data,inventory,save,app_smoke}_tests.rs`.
- Pastas de assets sem arquivos não foram criadas (git não versiona pastas vazias); ver `assets/README.md`.

## D-008 — Lints do clippy
`unwrap_used`/`expect_used` = warn. `type_complexity` e `too_many_arguments`
liberados: parâmetros de sistemas Bevy (queries com filtros) disparam esses
lints por natureza, e o próprio projeto Bevy os desativa.

## D-009 — Hitboxes perfurantes no corpo a corpo
Golpes corpo a corpo atingem cada alvo uma vez (`already_hit`) e podem atingir
vários alvos; projéteis (`pierce: false`) somem no primeiro acerto.

## D-010 — Teste de fumaça visual
`examples/autoplay.rs` injeta teclas no app real após `InputSystems`, registra
resultados e salva screenshots com a API de screenshot do Bevy. Complementa
`tests/app_smoke_tests.rs` (headless, roda no `cargo test`).

## D-011 — Compilação limitada a 4 jobs, sempre no `target/` da raiz
O build do Bevy com o paralelismo padrão (12 jobs) e artefatos em `/tmp`
(tmpfs, que fica na RAM) esgotou os 14 GiB da máquina e travou o computador
duas vezes. `.cargo/config.toml` fixa `build.jobs = 4` como padrão do projeto.
Não fixamos `target-dir`: `target/` já é o padrão, e `CARGO_TARGET_DIR`
sobrescreveria a config de qualquer forma. A proibição de `/tmp` e de diretórios
de build extras é uma regra de processo ("Compilação segura" no `CLAUDE.md`).
Custo: um build do zero fica mais lento. Os builds incrementais não mudam.

## D-012 — Política de segurança do Claude Code em duas camadas
A auditoria (A6) mostrou que as permissões liberavam `git diff --output=<arquivo>`,
`rm -r`, `find -delete` e escritas em `Md-usario/` e `saves/`. Regras de permissão
comparam o **texto** do comando; a própria documentação do Claude Code avisa que
não são uma fronteira de segurança (`/bin/rm`, `bash -c '...'` e variáveis escapam).
Por isso usamos duas camadas em `.claude/settings.json`:

1. **Permissões:** `deny` para os padrões destrutivos conhecidos e
   `Edit(/Md-usario/**)`/`Edit(/saves/**)` (cobre Edit, Write, NotebookEdit e
   redirecionamentos `>` reconhecidos); `ask` para `git push`, `--amend` e builds
   `--release`/`--profile`; `allow` só para `cargo fmt --check` (não mais `cargo fmt`)
   e comandos de leitura/compilação; `disableBypassPermissionsMode`.
2. **Hook `PreToolUse` (`.claude/hooks/guard.py`):** tokeniza o comando, remove
   wrappers (`sudo`, `env`, `timeout`, `command`, `xargs`, atribuições) e analisa
   `bash -c`/`eval`. Bloqueia `rm` recursivo, `find -delete`, git destrutivo
   (`reset --hard`, `clean`, push forçado ou `+refspec`, `checkout --`/`.`,
   `restore` fora do `--staged`, `stash drop`, `branch -D`, `worktree add`, `rebase`,
   `--output`), `git add` de `Md-usario/`/`saves/`, escrita nessas pastas,
   `cargo clean`, `--target-dir`/`CARGO_TARGET_DIR` fora do `target/` da raiz,
   mais de 4 jobs e `cargo run --example autoplay` quando `examples/autoplay.rs`
   não contém o isolamento de saves (`isolated_save_dir` + override de
   `SaveSlots`; ver D-013). Também separa comandos por quebra de linha e crase, acompanha `cd`
   dentro do mesmo comando e bloqueia `git stash -u/-a` (tiraria os arquivos não
   rastreados de `Md-usario/` da árvore). Se não conseguir analisar a chamada
   (JSON inválido, campos com tipo errado), bloqueia (falha fechada).
   Testes: `python3 -B .claude/hooks/test_guard.py` (testam só o hook, não as
   regras de `settings.json`).

**Regras de texto × hook:** quando divergem, vale a mais restritiva. Ex.:
`git clean -n` passa pelo hook mas é negado pela regra `Bash(git clean *)`;
`--target-dir` é negado mesmo apontando para o `target/` da raiz (a exceção de
worktree usa `CARGO_TARGET_DIR`, que o hook aceita só para esse `target/`);
`git rebase --abort` também fica negado e precisa ser feito pelo usuário.
**Falsos positivos conhecidos:** o corpo de um heredoc é analisado como comandos,
então uma linha que comece com `rm -r` ou `git reset --hard` dentro de um heredoc
é bloqueada; `ln` que apenas aponte para um arquivo protegido também.

**Limitações:** continua possível escapar com scripts e interpretadores
(`python -c`, `perl -e`), variáveis expandidas (`"$PWD"/saves`), globs
parciais (`sa*/`), aliases e programas que gravam
arquivos por conta própria (o próprio jogo grava em `saves/` no F5). Sem
`python3` no PATH o hook falha sem bloquear. A proteção real de sistema de
arquivos seria o sandbox do Claude Code (`sandbox.enabled` +
`sandbox.filesystem.denyWrite`), que no Linux exige `bubblewrap` e `socat`;
`socat` não está instalado, então ele não foi ativado. `disableAllHooks`
desativa a camada 2.

## D-013 — Autoplay com verificações e saves isolados
O autoplay gravava no `saves/slot1.ron` real e saía com sucesso mesmo quando um
golpe errava (A5, M4). Agora `examples/autoplay.rs`:
- substitui o recurso `SaveSlots` por `target/autoplay/saves/` antes do primeiro
  frame, confere que o override está ativo e recusa iniciar se o diretório não
  estiver dentro de `target/` ou se sobrepuser ao `saves/` real. O jogo normal
  não muda: `SaveSlots::default()` continua apontando para `saves/`;
- roda com passo fixo de 16 ms (`TimeUpdateStrategy::ManualDuration`), para o
  roteiro não depender do FPS;
- confere jogador, sala inicial, movimento, ataque básico, Crescent Slash,
  Ember Bolt, Mending Light (cura só quando o HP não está cheio), custos de
  MP/Hearts (tolerância de 1 ponto de regeneração no frame), Rising Edge
  rejeitado, save e load no diretório isolado e `saves/` byte a byte igual;
- termina com código ≠ 0 se uma verificação obrigatória falhar ou se o app
  fechar antes das verificações. Linhas `INFO` registram pontos ainda sem regra
  aprovada (ataque básico após comando rejeitado, D-GAME-003).
O mecanismo também é coberto sem janela por
`tests/app_smoke_tests.rs::quick_save_and_load_use_the_configured_save_dir`.

**Liberação pelo hook:** `guard.py` só deixa o Claude rodar
`cargo run --example autoplay` se o arquivo contiver `fn isolated_save_dir()` e
`insert_resource(SaveSlots(`. Essa checagem é **textual e protege apenas contra
acidentes** (um exemplo antigo ou reescrito sem o override); não comprova o
isolamento e não cobre a execução direta do binário em `target/`. A garantia
vem do próprio autoplay: a recusa em iniciar sem o override e a comparação
byte a byte de `saves/` ao final, que reprova a execução.
**Limitações conhecidas:** a cura do Mending Light não é exercitada (o jogador
não toma dano no roteiro, então a verificação fica "não aplicável"); o roteiro
depende do passo de 16 ms, porque a física varia com o intervalo entre frames
(M17).

## D-014 — Ativação confirmada, dano por mensagem e cancelamento de golpes (Fase 4)
Correções dos defeitos A1, A2 e A3 da auditoria E2, com decisões do usuário
(Q1–Q6 em `Md-usario/DarkCastleR-F4-3.md`).

**A1 — pedido × confirmação.** `SkillActivated` passou a ser só um **pedido**
do reconhecedor, que valida com o estado do frame anterior. `begin_skill` é o
único leitor: revalida com o estado **atual** (`validate_activation`, que aplica
`usable_in`), trava o personagem em `Casting` e só então cobra MP e Hearts
(juntos: os dois já foram validados, então não há cobrança parcial), inicia o
cooldown, aplica o avanço e emite `SkillStarted`. Os executores de efeito
(`melee_techniques`, `cast_spells`) leem só `SkillStarted`. Cada tentativa gera
exatamente um `CommandResolved`: o reconhecedor deixou de emitir `Executed`;
quem emite o resultado definitivo (executado ou o motivo da rejeição) é
`begin_skill` (Q1, opção B). `begin_skill` passou a rodar depois de
`tick_cooldowns` (os dois usam `SkillBook`; a ordem era ambígua). Efeito: o
cooldown começa cheio no frame da ativação e o reconhecedor, que lê o valor do
frame anterior, só aceita a habilidade de novo um frame depois de ele zerar
(ex.: 0,8 s ≈ 51 frames de 16 ms, em vez de 50 numa das ordens antigas). Consequência
aprovada (Q2): pulo ou esquiva no mesmo frame do comando de uma habilidade que
não é permitida nesse estado não faz nada. Assimetria registrada, ligada à
D-GAME-003 (pendente, não alterada): quando o próprio reconhecedor rejeita no
botão ATAQUE, sai um ataque básico; quando a rejeição vem de `begin_skill`, não.

**A3 — invulnerabilidade por mensagem.** `player::systems::receive_damage`
confere `Invulnerable` a cada `DamageDealt`: num frame com vários acertos só o
primeiro que causa dano é aplicado. `Miss` e dano 0 não consomem a
invulnerabilidade. Com o jogador morto, as mensagens restantes são descartadas
(antes, um `break` as deixava para o frame seguinte). Fórmulas e durações não
mudaram. Consequência: quando dois acertos chegam no mesmo frame, vale o que foi
escrito primeiro, e a ordem entre `contact_damage` e `detect_hits` continua
ambígua (já apontada na auditoria como BAIXO).

**A2 — cancelamento de golpes interrompidos.** `cancel_interrupted_hitboxes`
(set `Resolution`, depois do `receive_damage` do jogador) remove as hitboxes com
`AttachedToOwner` cujo dono está em `Hurt` ou `Dead`. A regra olha o estado, não
o evento: hoje nenhuma hitbox nasce com o dono nesses estados. Com
`attack_speed` acima de ~1,67 a hitbox de uma técnica dura mais que o `Casting`
(o tempo de vida não é dividido pela velocidade, M18 da auditoria) e também é
removida se o jogador for atingido nessa janela. Um acerto no
mesmo frame em que o jogador leva dano vale (troca simultânea). Dano bloqueado
não interrompe. Hitboxes de inimigos (sem `ActionState`) e projéteis já lançados
não são afetados (Q5). O `quick_load` (F9) ainda revive o jogador sem remover
hitboxes: adiado para a Fase 6 (Q3).

**Mending Light (Q4):** continua com efeito instantâneo; `Casting` é tempo de
recuperação. Conjuração com preparo interrompível segue pendente (D-GAME-002).

## D-015 — Visual animado do protagonista (V1.3)

**Contexto:** primeira integração dos sprites do modelo v007 (idle, run,
attack_basic, dodge), só de apresentação.

**Decisão:**
- O sprite é um filho `PlayerVisual` do jogador (`src/player/visual/`), com
  manifesto puro e testável (`manifest.rs`, `playback.rs`, `selection.rs`).
- Ele só lê o estado do gameplay. O único acréscimo fora da apresentação é o
  getter de leitura `PlayerStateMachine::remaining()`, usado para acompanhar a
  duração real das ações (inclusive com `attack_speed`).
- O manifesto fica em `assets/animations/player.ron`, fora de `assets/data/`,
  por ser dado de apresentação (contrato §7). É carregado pelo próprio plugin
  visual, não por `GameData`.
- Ausente ou inválido, o jogo mantém o visual provisório (log de info ou de
  erro) em vez de encerrar. Um export de arte quebrado não derruba o jogo, e
  `data_tests` impede entregar um manifesto inválido.
- Os atlas ficam em `assets/sprites/player/player_{locomotion,combat}.png`, no
  acabamento C, densidade 2. As escolhas de acabamento, espada (×1,7) e fps são
  provisórias; D2 e D3 continuam pendentes.
- `Walking` e `Running` usam o mesmo ciclo `run`, feito para 190 px/s e tocado
  numa taxa proporcional à velocidade real.

**Consequências:**
- Estados sem sprite usam o primeiro quadro do idle com um tom de feedback.
- O espelhamento troca a mão da espada.
- Detalhes em `docs/PLAYER_VISUAL_ANIMATION_V1_3.md`.

## D-016 — Expansão das animações do protagonista (V1.4)

**Decisão:**
- A habilidade em execução é identificada pela mensagem existente
  `SkillStarted` e pelo campo `animation` da habilidade nos RON.
  - O visual lê a mensagem, sem consumi-la de outros leitores.
  - Nenhum componente nem estado de gameplay foi criado.
- A aterrissagem é só apresentação: `Falling` (≥ 0,1 s) → `Idle`, sem teleporte.
- Novos clipes nas células do contrato, sem mudar o contrato. Os clipes de
  locomoção (dano, morte, pulo, queda, aterrissagem, agachar, guarda) ficam na
  célula 64×64, e os de habilidade na 128×96.
- Quando a sobra do último quadro tem menos de 25 ms, os dois últimos quadros
  dividem o tempo.

**Consequências:**
- Uma habilidade nova com `animation` já existente no manifesto não exige código.
- A morte termina ajoelhada, não deitada: as roupas não têm simulação.
- Detalhes em `docs/PLAYER_ANIMATION_EXPANSION_V1_4.md`.

## D-017 — Publicação segura dos sprites e camada de VFX (V1.5)

**Decisão:**
- Só `art-src/player/scripts/post/publish_guard.py` escreve
  `assets/sprites/player/` e `assets/animations/player.ron`.
  - Os scripts de pós-processamento geram um diretório de staging; um destino
    dentro de `assets/` é recusado antes de qualquer leitura ou escrita.
  - A publicação valida PNG, manifesto, pivôs, durações do jogo e quadros não
    vazios; exige versão do gerador ≥ a publicada e todos os clipes já
    publicados; faz backup, escreve por arquivos temporários com journal e
    desfaz tudo em caso de falha.
  - O que está publicado fica registrado em `assets/animations/player.publish.json`
    (versão, gerador, clipes, SHA-256). Substituir exige `--confirm <state id>`;
    não há opção genérica de sobrescrita.
- Os VFX são clipes do mesmo manifesto (atlas `effects`, densidade 1, chave
  `vfx_<vfx>` dos RON de habilidades), tocados por filhos `PlayerVfx` do jogador.
  - Sinais usados: `SkillStarted`, `DamageDealt` com o jogador em guarda e de
    frente (regra única `Facing::faces_hit`, também usada por `receive_damage`),
    e a saída do estado `Dead`.
  - Nenhum sistema de gameplay lê os efeitos.
- `attack_basic_air` é escolhido pelo contato com o chão (`Grounded`, só leitura).

**Consequências:**
- Um script histórico (V1.3, V1.4) não consegue mais substituir os atlas atuais.
- A ordem dos clipes nos atlas só cresce: clipes novos entram no fim.
- Uma habilidade nova com `vfx` já existente ganha efeito sem código.
- Sem dependência nova: o hash fica no teste Python do guarda; o teste Rust
  confere clipes e arquivos do lock.
- Detalhes em `docs/PLAYER_ART_POLISH_V1_5.md`.

## D-018 — Grafo de mundo isolado do jogo principal (V1.6)

**Decisão:**
- O mundo de grande escala é um **grafo de salas** em `assets/world/`
  (`world.ron` + uma região por arquivo), separado de `assets/maps/`.
  - Por isso `GameData` e o jogo atual não mudam.
  - O exemplo e os testes copiam as salas para o `GameData` em memória (`install_world`).
- **Requisitos:** tipados (`Requirement::{Ability, Item, Flag, All, Any}`),
  avaliados por casamento de padrões. Nada de expressões em runtime.
- **Paredes:** o contorno de cada sala é gerado a partir das portas (aberturas
  e soleiras padronizadas). Os arquivos descrevem só o interior.
- **Validação:** uma simulação de progressão (com e sem segredos) classifica
  cada sala; salas fechadas no início não são erro.
- **Troca de sala:** anunciada por `RoomChanged`. Inimigos e magia limpam o que
  é deles, seguindo a regra de comunicação.
- **Exploração:** salva num arquivo próprio (versão 1) até o save v2.
- **O que ficou de fora:** física, câmera, combate e save real não mudaram.
  Transformações e economia foram especificadas, não implementadas.

**Consequências:**
- Integrar ao jogo principal (V1.7) pede o save v2 com migração e o load entre salas.
- Única escrita do mundo num componente do jogador: a transição posiciona o
  `Transform` na porta de chegada, como já fazem o `quick_load` (save) e o
  `respawn`. Velocidade, estado e vida não são tocados.
- O validador não simula o alcance físico; portas inalcançáveis precisam ser
  `entry_only` ou exigir uma habilidade.
- Detalhes: `WORLD_ARCHITECTURE.md`, `ROOM_SYSTEM.md`, `WORLD_PERSISTENCE.md`.

## D-019 — Mundo no jogo principal, ciclo de vida de salas e save v2 (V1.7)

**Decisão:**
- **Início:** `app::run` carrega e valida `assets/world/`. Um mundo com erros
  impede o início.
- **Sala única:** `app::build` sem mundo continua sendo o jogo de sala única,
  usado como fixture. `entry_hall` é mapeada em `legacy_rooms` para a porta
  inicial.
- **Troca de sala:** só `world::transitions::apply_room_requests` troca a sala,
  com no máximo um `RoomRequest` por frame (um load vence portas, a primeira
  porta vence as seguintes). Cada dono limpa suas entidades temporárias ao ler
  `RoomChanged` em `RoomStep::React`: hitboxes, projéteis, números de dano e
  respawns pendentes.
- **Save v2:** acrescenta `exploration: ExplorationRecord`.
  - **Migração v1:** lê `SaveFileV1` congelado com `deny_unknown_fields`.
  - **Load:** nunca grava no arquivo.
  - **Backup:** a primeira gravação sobre um v1 guarda `slotN.ron.v1.bak`
    permanente.
- **Relíquias:** relíquias de progressão vivem em `granted`, não no
  inventário. Itens comuns coletados no mundo entram no inventário uma única
  vez, por `PickupCollected`.
- **Mapa:** a tecla `M`, que estava livre, abre e fecha o painel.

**Consequências:**
- O protótipo ganhou ajustes de geometria (poço e um obstáculo) para caber no
  pulo real de 130 a 136 u. A física não mudou.
- O arquivo de exploração próprio da V1.6 ficou só no exemplo
  `world_vertical_slice`.
- Detalhes: `WORLD_INTEGRATION_V1_7.md`.

## D-020 — Objetos interativos sem schema novo, interação na tecla D (V1.8)

**Decisão:**
- **Contrato único:** paredes quebráveis, interruptores ocultos, baús e
  alavancas usam um só contrato tipado (`graph::WorldObject` +
  `ObjectKind`), lido de RON.
- **Estado derivado:** o estado de cada objeto é derivado dos ids que o
  save v2 já guarda.
  - Paredes e interruptores usam `secrets`.
  - Baús usam `pickups`.
  - Alavancas usam a `Flag` em `granted`.
  - Por isso não existe save v3.
- **Uma chamada pura:** cada interação registra o estado e concede a
  recompensa na mesma chamada (`ExplorationState::interact` e
  `break_wall`). Recusas não registram nada.
- **Paredes:** quebram com hitboxes reais do jogador. As magias contam por
  meio da mensagem `ProjectileBlocked`, emitida pelo módulo de magia (dono
  do projétil), que a remove ao tocar a geometria. O dano e a duração das
  hitboxes não mudaram.
- **Tecla de interação:** `D`, livre. A seta `↑` foi descartada porque o
  comando `↓ ↑ + ATTACK` usaria objetos sem querer.
- **Fade de entrada:** só de entrada (0,22 s, apresentação). O fade-out
  antes da troca exigiria adiar o ciclo de vida da sala.
- **Porcentagem:** a fórmula global não mudou. Cada região nova entra no
  denominador, e `region_stats` dá os números por região.

**Consequências:**
- O validador ganhou `NoWayBack` e `ShortcutBypassesProgression` (erros),
  além de `DuplicateId`, `ObjectOutOfRoom`,
  `ImpossibleObjectRequirement` e `UnusedLever` (aviso).
- O `goal_room` do mundo passou a ser `fg_return_bridge`.
- Os testes da V1.6 que contavam o recorte da Vigília agora usam
  `region_stats("vigil_gates")`, com os mesmos valores esperados.
- Detalhes: `WORLD_EXPLORATION_V1_8.md`.

## D-021 — Transformações por perfis opcionais e véus nas portas (V1.9)

**Contexto:** o briefing da V1.9 pede a Forma Espectral, liberada pela
Relíquia do Véu, que atravesse barreiras específicas sem alterar o
movimento humano, o combate nem o save V2, e sem softlocks.

**Decisão:**
- **Forma humana = ausência de forma:** a forma aplica seu perfil por três
  componentes opcionais e genéricos:
  - `LocomotionProfile` (player);
  - `GravityProfile` e `Phasing` (world/physics).
  - Sem eles, o código executado é o da V1.8.
  - Não foi preciso refatorar as constantes humanas em perfis de dados
    (a decisão pendente da V1.6).
- **Colisão por tipo de barreira:** `Solid::barrier: Option<BarrierKind>`.
  - Um véu é sólido para todos, menos para quem tem `Phasing` daquele tipo.
  - Paredes nunca têm tipo, então nenhuma forma as atravessa.
- **Véus na abertura de portas, nunca no meio da sala:**
  - o grafo sabe o que cada véu protege;
  - o validador trata a porta com véu como exigência da passagem do tipo
    (`world.ron` `passages`);
  - `app::load_world` confere `passages` contra as definições e o custo
    real de MP de cada travessia.
- **Fim seguro em duas etapas:** o fim só acontece quando o corpo humano
  cabe.
  - Até lá, a forma continua (mesma física e aparência, sem custo).
  - Na morte dentro de um véu, o corpo fica parado até o respawn.
  - O sistema de forma nunca move o jogador.
- **Bloqueio de ações por máscara de entrada:** `mask_form_input` zera as
  entradas das ações bloqueadas antes dos sistemas de jogo. Combate,
  habilidades e magia não foram alterados.
- **Save:** o schema V2 não mudou. A forma não é salva (o load volta
  humano) e F5 é recusado dentro de um véu.
- **Relíquia:** `VeilStep` passou a liberar a forma e continua abrindo a
  grade do Salão Selado. Não foi criada uma `Ability` nova, para não mudar
  saves nem a rota principal aprovada.
- **Tecla `T`:** estava livre.
- **Visual sem asset:** o sprite humano recebe tom e transparência, com
  halo de gradiente gerado em memória, contorno e partículas.

**Consequências:**
- **Mundo:** passou a 22 salas, com a região Claustro Velado (2 salas) e
  3 véus. Os totais globais de três testes da V1.8 mudaram de 20 para 22.
- **HUD:** a dica de controles foi para o painel de status.
- **Pendências:** arte própria da forma, outras formas e a escolha de forma
  quando houver mais de uma.
- **Detalhes:** `TRANSFORMATIONS_V1_9.md`.

## D-022 — Equipamento por dados, sem schema novo, aparência em camadas de runtime (V2.0)

**Contexto:** a V2.0 pede armas com identidade, armaduras, acessórios,
inventário jogável, comparação, aparência por equipamento e compatibilidade
com a Forma Espectral, preservando o Save V2, os 8 slots e as animações
aprovadas.

**Decisão:**
- **Classe por categoria + dados por item:** `WeaponClass` (forma e ritmo do
  golpe) em `weapon_classes.ron`; o item só acrescenta elemento e alcance. O
  dano continua no ATK do item. A classe Sword repete as constantes antigas.
- **Sem instâncias de item:** itens iguais são intercambiáveis; o save
  continua guardando IDs por slot e pilhas na bolsa.
- **Equipado sai da bolsa:** contrato que o equipamento inicial já seguia.
- **Slots preservados:** sem `Cloak`; capas são peças de `Body` com camada
  visual. Rótulos de tela mudaram, nomes persistidos não.
- **Único escritor:** `apply_equip_requests` a partir de `EquipRequest`;
  recalcula o bônus dos slots (nunca soma ao anterior).
- **Trava conservadora:** só em locomoção; nunca em ação, com golpe vivo,
  transformado ou em transição.
- **Técnicas declaram armas** (`weapons`), em vez de cada arma listar técnicas.
- **Teto de resistência** (0,75) nas fórmulas do personagem.
- **Aparência sem arte nova:** rastro (cor + forma da classe), capa
  (textura gerada em memória) e tom do tronco. Arma na mão e elmos ficam
  para quando o pipeline exportar sockets por quadro.
- **Tecla `I`** (livre) e `Enter` dentro da tela; tela pausada como Status.
- **Sem drops novos:** um drop a mais consome números aleatórios e mudaria o
  combate determinístico das rotas aprovadas.

**Consequências:**
- 27 equipamentos; 3 baús novos nos Portões da Vigília.
- `SkillDefinition` ganhou `weapons`; dois literais de teste ganharam o campo.
- Raridade Epic; Legendary passou ao tier 4 (só ordenação).
- **Pendências:** arma e elmo desenhados, slot de capa (exige Save V3),
  regra de duas mãos, drops de equipamento.
- **Detalhes:** `EQUIPMENT_EXPANSION_V2_0.md`.

## D-023 — NPCs e lojas por dados; economia sem persistência até autorização (V2.1)

**Contexto:** a V2.1 pede cidade, NPCs, diálogos, lojas e uma economia
persistente, sem Save V3 não autorizado e sem improvisar persistência.

**Decisão:**
- **NPCs, nós de diálogo e lojas em RON** (`assets/data/{npcs,dialogues,shops}`),
  validados uns contra os outros e contra o mundo.
  - Condições formam um conjunto fechado e tipado; não existe linguagem de script.
- **Objetos têm prioridade sobre NPCs no `D`:** a conversa é decidida antes do
  sistema de objetos, olhando o mesmo `usable_object`.
- **Diálogo e loja são `GameState`s pausados:** não há combate nem troca de
  sala; um load fecha os dois.
- **Moeda única `Purse(u32)`** com teto; `price > sell_value` obrigatório; o
  único escritor é `apply_trades`.
- **Persistência da economia adiada:** o Save V2 não tem campo para ela.
  - A invariante "nenhum save contém uma troca" (F5 recusado após negociar;
    load restaura moedas iniciais e estoque cheio) impede duplicação e
    dinheiro infinito.
  - A migração v3 proposta está em `NPC_COMMERCE_V2_1.md` §10 e aguarda o proprietário.
- **Duas mãos por classe de arma** (`two_handed`), recusa nos dois sentidos,
  sem schema novo.
- **Vila opcional** ligada à porta direita do Watch Shaft (célula 1), com um degrau novo.
- **Drops de equipamento adiados:** mudariam o consumo do RNG das rotas aprovadas.
- **Tutoriais em `docs/tutorias/`:** `Md-usario/**` está em `deny` nas permissões
  do projeto; o teste dos tutoriais procura primeiro em `Md-usario/tutorias/`.

**Consequências:**
- O mundo passou a 28 salas; três totais de `world_graph_tests` foram atualizados.
- `quick_save` passou a consultar o `EconomyLedger`.
- **Pendências:** Save v3, mover os tutoriais, âncoras de arte, drops, ícones
  no mapa, arte de NPCs.
- **Detalhes:** `NPC_COMMERCE_V2_1.md`.

## D-024 — Save v3 com economia; exceção de `Md-usario/tutorias/` não aplicada (V2.1.1)

**Contexto:** o proprietário autorizou (`DarkCastleR-V2-1-a.md`) o Save v3 só
para moedas e estoque, e uma exceção restrita para `Md-usario/tutorias/*.md`
que funcione nas duas camadas de proteção, sem enfraquecê-las.

**Decisão:**
- **Save v3:** campo `economy: Option<EconomyRecord>`; v1 e v2 migrados em
  memória com `None` (moedas iniciais, estoque cheio); backups pela política
  já existente. A trava provisória da V2.1 foi removida após os testes.
- **Watch Buckler `Limited(1)`:** uma mercadoria limitada ao alcance das
  moedas iniciais, para a rota real demonstrar o estoque persistido.
- **Permissões não alteradas:** a documentação do Claude Code diz que `deny`
  sempre vence `allow`, que a negação `!` não reabre arquivos sob uma pasta
  bloqueada por `**` nem alcança regras ancoradas em `/`, e não documenta
  classes de caractere. A exceção não pode ser expressa com segurança na
  camada de permissões; pelo briefing, a negação foi preservada e o hook
  também não foi alterado (uma exceção só no hook seria inútil e enfraqueceria
  a defesa em profundidade).

**Consequências:** economia persistente concluída; tutoriais continuam em
`docs/tutorias/` até o proprietário escolher uma alternativa
(`NPC_COMMERCE_V2_1.md` §25).

## D-025 — Arma visível por camada sobre um corpo sem arma, com âncoras medidas no Blender (V2.2)

**Contexto:** os atlas publicados (V1.3–V1.5) têm o sabre renderizado junto
do corpo; qualquer arma equipada mostrava a espada inicial. O briefing
`DarkCastleR-V2-2.md` proíbe esconder a espada com sobreposição e exige
corrigir na origem, sem sobrescrever assets publicados sem autorização.

**Decisão:**
- **Origem:** `bpy/build_game_sprites_v2_2.py` renderiza de novo **as
  mesmas poses** (módulos de clipes V1.3/V1.4/V1.5 sem mudança; hash dos
  keyframes conferido contra cada build original) com os seis objetos do
  sabre ocultos, e mede por quadro as âncoras (mão direita = empunhadura e
  ângulo da lâmina; mão esquerda; cabeça; peito; costas) projetando os
  pontos na câmera do contrato. Nenhum `.blend` é salvo.
- **Staging:** `post/game_sprites_v2_2.py` monta atlas com o layout
  publicado idêntico (índices, durações, pivôs), o manifesto com
  `weapon_in_body: false` e `player_sockets.ron`. `publish_guard.py` passou a
  aceitar a linha opcional e a publicar/validar a tabela junto (falha
  fechada: tabela ausente ou com contagem errada é recusada).
- **Runtime:** `weapon_in_body` (padrão `true` para manifestos antigos)
  desliga as camadas enquanto a arte publicada tiver o sabre, evitando arma
  duplicada. Arte das armas: uma silhueta por categoria, gerada em código
  (provisória). Lança e arma pesada em repouso são desenhadas de ponta para
  cima (não atravessam o chão); nos clipes de golpe seguem o sabre.
- **Duas mãos:** a regra de equipar não mudou; se um save editado tiver
  arma de duas mãos e escudo, o escudo é mostrado nas costas (não some).
- **Sem save novo:** posições visuais são derivadas da arte.

**Consequências:** publicada com autorização (`DarkCastleR-V2-2-a.md`,
estado `1cd66a2977d0`, backup do anterior pelo guarda). Os testes que
assumiam "publicado = sabre no corpo" passaram a usar um staging legado
explícito para essa verificação. Nenhuma dependência nova.

## D-026 — Figuras de NPC por dados, marcadores no mapa e HUD empilhado (V2.2)

**Contexto:** NPCs eram retângulos; o mapa não indicava lojas; notificações
no canto inferior esquerdo, sem fundo, passavam sobre o personagem e os avisos
"D Talk".

**Decisão:**
- `look: Option<NpcLook>` opcional (serde default, compatível) gera uma
  figura com o mesmo centro do retângulo antigo: alcance e avisos inalterados.
- O mapa marca NPCs (rosa) e lojas (violeta, quem tem `shop`) só em salas
  visitadas, como os baús. Sem marcador de ferreiro (não há serviço).
- HUD: status, feedback de comando e notificações empilhados à esquerda, com
  painel de fundo; com loja, inventário ou status abertos (jogo pausado), as
  notificações vão para uma faixa inferior acima do overlay.
- O teste de tutoriais passou a ler só `docs/tutorias/` (fonte oficial,
  regra do proprietário); `Md-usario/tutorias/` é sincronizada à mão.

**Consequências:** sem mudança de save, de diálogo ou de progresso.

## D-027 — Câmera por dados e cenário só de apresentação, gerado em código (V2.3)

**Contexto:** a gravação analisada no briefing `DarkCastleR-V2-3.md` mostrou
protagonista pequeno, muito espaço vazio e salas com cara de teste.

**Decisão:**
- **Câmera:** `assets/data/camera.ron` (todos os campos opcionais, padrões
  seguros e validados) e regras puras em `world::camera_rules`; projeção
  `FixedVertical` com altura de mundo 405 (×1,78), escolhida por varredura em
  três salas. Física e colisões independentes do zoom.
- **Cenário:** campos opcionais `visual` (região) e `decor` (sala) no formato
  de mundo existente; nada de segundo sistema de salas. Peças só de
  apresentação (`RoomEntity` sem colisão), validadas pelo validador do mundo.
- **Arte:** gerada em código (`core::pixel_art`), sem PNG novo em `assets/`
  (não exige publicação pelo guarda); provisória.
- **Iluminação:** brilhos e névoa por sprites; sem pós-processamento nem
  dependências novas; tudo desligável (`effects`).
- **Pilotos:** três salas de três regiões antes de qualquer expansão.

**Consequências:** `world_camera_tests` passaram a usar a vista configurada
(as mesmas checagens). Sem mudança de save.

## D-028 — V2.4 = Combat Feel; roadmap realinhado ao que foi entregue (consolidação SOTN)

**Contexto:** a tabela de `GAME_VISION_MASTER.md` §6 previa V2.2 = conteúdo
do castelo, V2.3 = chefes, V2.4 = missões e V2.5 = expansão. As versões
entregues foram outras (V2.2 = equipamentos visuais, D-025/D-026; V2.3 =
câmera e atmosfera, D-027). A pesquisa técnica SOTN (Fases 1 e 2,
`docs/REFERENCIAS_SOTN.md` e documentos ligados) apontou a sensação de
combate como a maior lacuna. O proprietário aprovou a direção na tarefa de
consolidação da pesquisa.

**Decisão:**
- **V2.4 = Combat Feel / Peso do Golpe**, conforme
  `docs/PLANO_V2_4_COMBAT_FEEL.md`. A **direção** está aprovada; a
  **implementação** de cada etapa exige autorização específica.
- A tabela-direção passa a registrar V2.2 e V2.3 como foram entregues.
- Conteúdo do castelo, chefes e progressão, missões e narrativa e expansão do
  mundo continuam como **itens futuros sem número**. A ordem sugerida está em
  `docs/PROPOSTA_ROADMAP_POS_V2_4.md` e depende de aprovação.
- Princípios do plano: campos novos opcionais com padrão igual ao
  comportamento atual; nenhuma arma existente (inclusive a Iron Sword) muda
  sem nova decisão; o gameplay continua guiado por temporizadores em dados,
  e a arte o acompanha (`docs/ESTUDO_SOTN_ANIMACOES.md` §3).
- Pesquisa: nenhum código, tabela, asset ou nome do SOTN entra no projeto
  (sotn-decomp é AGPL-3.0; SymphonyRecomp não tem licença).

**Consequências:** só documentação. Referências "V2.4 = missões" em
`QUEST_FACTION_SYSTEM.md` e `WORLD_PERSISTENCE.md` passaram a apontar para
o item futuro. Os documentos da Fase 1 receberam notas de correção (respawn
na reentrada já existente no DCR) sem perder o texto original. Sem mudança
de código, dados ou save.

## D-029 — Pontos de save jogáveis sem restringir F5 (D17)

**Contexto:** `discovered_save_points` e `last_save_point` existiam no save
sem escritor de gameplay. Portas/interações já cobriam a preferência inicial
do briefing D17. O ciclo Metroidvania ainda pedia um altar no mundo.

**Decisão:**
- Novo `ObjectKind::SavePoint` reutilizável; progresso só em `WorldProgress`
  (não em pickups de exploração).
- Interação `D` cura HP/MP, descobre, grava o slot via a mesma rota do F5.
- F5 permanece livre (dev + acessibilidade); “salvar só no altar” fica
  pendente e explícito no roadmap.
- Sem bump de versão de save.

**Consequências:** um altar em `vigil_gates_approach`; mapa mostra altares
descobertos; testes headless cobrem cura/persistência. Viagem rápida entre
altares fica para etapa posterior.

## D-030 — Uso de consumíveis pela tela de inventário (D18)

**Contexto:** efeitos `ConsumableEffect` e obtenção (start/baús/loja) já
existiam; a tela `I` só equipava. O loop de combate pedia cura de HP/MP
sem altar.

**Decisão:**
- Regras puras em `inventory/consume.rs`; único escritor Bevy
  `apply_use_item_requests`.
- Uso permitido com inventário aberto (exceto morto) ou em locomoção.
- Recusar e não consumir se o recurso alvo já está no máximo.
- Bolsa listada: equipamentos primeiro, depois consumíveis.
- Sem bump de Save V3; sem hotbar nesta entrega.

**Consequências:** `minor_ether` (RestoreMp) no catálogo/start/loja;
testes `consumable_app_tests`. Viagem rápida permanece para D19.

## D-031 — Viagem rápida só entre altares descobertos (D19)

**Contexto:** D17 deixou `discovered_save_points` sem deslocamento. Teleporte
livre quebraria o Metroidvania; duplicar o loader de salas seria perigoso.

**Decisão:**
- Reutilizar `RoomRequest` + `Arrival::Position` (mesmo caminho do F9).
- Abrir menu só após descanso, se houver outro altar descoberto.
- Viajar não cura, não grava save e não altera `last_save_point`.
- Segundo altar em Broken Causeway; sem bump de Save V3.

**Consequências:** `world::fast_travel` + `GameState::FastTravel`; testes
`fast_travel_app_tests`. Viagem pelo mapa fica para etapa futura.

## D-032 — Arquétipos de IA por estilo em dados (D20)

**Contexto:** só o Bone Sentry melee existia; Defend/Retreat no enum estavam
subutilizados. Chefes (D21) precisam de variedade de combate comum primeiro.

**Decisão:**
- Estender `AiProfile` com `style`, faixas e `projectile_speed` (defaults
  preservam RON antigo).
- Dois arquétipos novos via dados + execução compartilhada; sem FSM por inimigo.
- Spawns fora das rotas de chão críticas (ledge do shaft; gallery leste).

**Consequências:** testes `enemy_archetype_tests`; doc de chefes em
`D20_PREPARACAO_CHEFES.md` sem implementar chefe.

## D-033 — Primeiro chefe no pipeline de inimigos (D21)

**Contexto:** `defeated_bosses` existia no Save V3 sem escritor; D20.1 pediu
arena plana e fases sem FSM nova.

**Decisão:**
- `BossDefinition` opcional na definição + `Boss`/`BossRuntime` no spawn.
- Fases por limiar de HP; padrões slash/lunge/ash-wave no mesmo `think_and_move`.
- `BossEncounter` mantém `TransitionLock` durante a luta; vitória grava
  `defeated_bosses` e recompensa única sem bump de versão de save.
- Arena acima do Reliquary (porta Top) para não interceptar rotas de chão.

**Consequências:** `boss_encounter_tests`, `d21_boss_probe`, docs `d21/`.

## D-034 — Luta do chefe legível e no chão (D22)

**Contexto:** o probe da D21 injetava `DamageDealt`. Em combate real, os golpes
do jogador cancelavam o wind-up (o chefe nunca completava o padrão) e a
patrulha entrava no vão da porta do chão.

**Decisão:**
- Chefe mantém o wind-up já iniciado; inimigos comuns continuam perdendo o golpe.
- Fase 2 só após FrontalSlash e Lunge, ou com HP abaixo de 25%.
- Cores de preparação distintas; a barra nomeia o padrão.
- Spawn/patrulha na laje direita (x 1040, meia-largura 80). Escada one-way no
  Reliquary até a porta do teto. Save V3 sem bump.

**Consequências:** `boss_real_fight_tests`, `d22_boss_probe`, docs `d22/`.
O probe da D21 não é prova de combate real.

## D-035 — Chegada na arena e telégrafo visível sob o dano (D23)

**Contexto:** a porta da Ashen Court precisa ser `Bottom` (o mapa exige o
lado oposto ao `Top` do Reliquary). O ponto de entrada cai no vão do chão
(x 540–740), então o corpo voltava para a sala de baixo. No combate, o flash
branco cobria a cor do golpe e a contagem do wind-up parava enquanto o flash
estava ativo, então o telégrafo não dizia quando o golpe saía.

**Decisão:**
- Plataforma sólida em (600, 32), 220×16, cobrindo a entrada (x 640) e
  emendando no chão da direita (a partir de x 740). O vão x 540–600 fica
  aberto como descida. A porta não muda de lado. A patrulha continua em
  x 1040 ± 80.
- A cor do phase lock e do wind-up do chefe substitui o flash branco.
  Inimigos comuns continuam brancos, inclusive sobre o próprio wind-up.
- O wind-up já iniciado de um chefe continua contando durante o flash.
  Inimigos comuns continuam atordoados e ainda perdem o wind-up ao apanhar.
- Números de dano, HP, esquiva e guarda não mudam. A câmera global não muda:
  o chefe fora de quadro na porta é distância (vista ~720 px, chefe em
  x 1040), e os quadros de corpo a corpo já mostram os dois.

**Consequências:** `arena_access_tests`, `boss_tell_counts_down_while_the_hurt_flash_is_up`,
`d23_play_probe` (derrota sem cura, boss em 13 HP), docs `d23/`.
O probe não é um playtest humano.

## D-036 — Aprovação da fundação N1, sem implementação

**Contexto:** a N1 propôs controles, escala, ferramentas, o papel do Ashen Sentinel e a primeira região nova. O proprietário aprovou as cinco em 17 de novembro de 2026 e pediu para não escrever código sem uma autorização nova.

**Decisão:**
- Controles futuros: A/D andam, W e Espaço pulam, S agacha, o mouse esquerdo ataca, E usa. O preset antigo permanece.
- 640×360 fica restrito a um protótipo isolado. A câmera do jogo não muda.
- Blender, Pixelorama e Bevy são a hipótese de pipeline, válidos só depois da prova.
- The Ashen Sentinel é minichefe na documentação. A mudança no dado espera a N10 e não apaga saves.
- A primeira região nova usa só habilidades que já funcionam e fecha um laço.

**Consequências:** docs `n1/`. Nenhum sistema de jogo muda por esta decisão. A implementação veio depois, autorizada pelo briefing N2, e está em D-037.

## D-037 — Controles híbridos remapeáveis (N2)

**Contexto:** D-036 aprovou o esquema e pediu para não escrever código. O briefing `DarkCastleR-N-2.md` autorizou a implementação. O jogo tinha uma tecla por ação (`InputBindings`). Menus, diálogo, loja, mapa e o save rápido liam essa tecla direto.

**Decisão:**
- A ação lógica é o que o jogo lê. Tecla e mouse só alimentam essa ação. Várias teclas da mesma ação viram um único pressionar, segurar ou soltar.
- Sem arquivo de controles, o padrão é o preset híbrido: A/D andam, W e Espaço pulam, S agacha e é o "baixo" do comando, seta para cima é o "cima" do comando e não pula, mouse esquerdo ataca e X continua como ataque alternativo, E interage. Guarda foi para Q e esquiva para F, porque A e S passaram a ser movimento. O jogador pode remapear as duas.
- O preset clássico reproduz as teclas antigas (setas, Z, X, C, A, S, D, e o resto). Editar um preset copia para um perfil customizado. O preset em si não muda.
- Conflito no mesmo contexto é recusado. A tecla anterior fica. Enter pode confirmar menu e, ao mesmo tempo, ser uma tecla de jogo. Escape não entra em binding: abre a tela no jogo e, durante a captura, só cancela a captura.
- A tela de controles usa as setas e Enter, não os bindings de jogo. Aplicar grava `controls.ron` fora do save. O save do personagem continua na versão 3 e não guarda teclas.
- Arquivo inválido não derruba o jogo e não é sobrescrito. O preset híbrido entra na memória e a tela avisa.
- Testes e exemplos que apertam as teclas antigas instalam o preset clássico. `play_before_boss` usa o perfil real.
- Não há gamepad. Um dispositivo futuro teria de alimentar o mesmo combinador de bordas. As teclas 1–4 da tela de status continuam fixas, fora do perfil.

**Consequências:** `src/input/`, `GameState::Controls`, docs `development/n2/`, `controles.md`. Câmera, arte, save v3 e o grafo de salas não mudam.

## D-038 — Prova visual isolada (N3)

**Contexto:** D-036 deixou Blender, Pixelorama e Bevy como hipótese até uma prova. O briefing `DarkCastleR-N-3.md` autorizou essa prova sem mudar o jogo.

**Decisão:**
- A prova mora em `examples/n3_visual_proof.rs` e em `art-src/n3-proof/`. Não entra no `cargo run`, não altera `camera.ron` e não troca o protagonista publicado.
- A malha branca não foi usada: não tem esqueleto. Um quadro do manequim V2.2 foi renderizado só como referência e não foi publicado em `assets/`.
- Os clipes da prova são pixels autorados por script. Pixelorama não estava instalado e não foi instalado. Isso não é aprovação do estilo.
- Nenhuma dependência nova. O save continua na versão 3.

**Consequências:** docs `development/n3/`. A N4 não começa com esta figura.

## D-039 — Segunda prova visual, ainda isolada (N3.1)

**Contexto:** D-038 mostrou o contrato de camadas com uma figura geométrica e deixou o estilo sem aprovação. O briefing `DarkCastleR-N-3-1.md` pediu a comparação do manequim real, no mesmo gesto, sem promover a arte.

**Decisão:**
- Os renders saem do blend V2.2 aberto só para leitura. O arquivo não é salvo e não substitui o protagonista publicado.
- A redução para pixel (BOX, 16 cores, contorno de script, vizinho 2×) é automática. Não é pintura no Pixelorama. O binário 1.2.4 foi baixado e não pintou nenhum quadro.
- Não há desenho independente nesta execução. Não há segunda malha de armadura. O ataque publicado não ganha antecipação.
- Nenhuma dependência nova. A câmera do jogo e o save v3 ficam como estão. A N4 não começa com este resultado.

**Consequências:** docs `development/n3_1/`, `examples/n3_visual_proof.rs` também mostra o ataque renderizado. O estilo continua sem aprovação.

## D-040 — Referência gerada, ainda sem sprite (N3.2)

**Contexto:** A N3.1 não aprovou o manequim reduzido. O briefing `DarkCastleR-N-3-2.md` pediu conceitos por geração de imagens e um teste de conversão para sprite modular.

**Decisão:**
- As ilustrações ficam em `art-src/n3-2-art-direction/`. Não substituem o protagonista publicado e não entram em `assets/animations/`.
- A redução a 100 px e o snap de paleta são automáticos. Não são pintura. As três poses geradas não formam um clipe.
- O exemplo isolado pode mostrar o quadro reduzido do conceito B, rotulado como referência sem arma e sem capa separadas.
- Nenhuma dependência nova. A câmera do jogo e o save v3 ficam como estão. A N4 não começa com este resultado.

**Consequências:** docs `development/n3_2/`. O estilo continua sem aprovação.

## D-041 — Gabarito nativo, sem pintura (N3.3)

**Contexto:** A N3.2 mostrou que reduzir a ilustração não produz o sprite. O briefing `DarkCastleR-N-3-3.md` pediu pixel desenhado para cerca de 100 px e mandou parar se o primeiro quadro ainda parecesse um manequim.

**Decisão:**
- O gabarito fica em `art-src/n3-3-native-pixel/`. Não substitui o protagonista publicado.
- Os pixels vêm de tabelas, não de uma sessão do Pixelorama e não de uma redução. O primeiro quadro não passou do manequim de blocos, então não há clipe de ataque.
- A capa e a espada são imagens separadas. O exemplo isolado pode mostrar esse quadro, rotulado como construção.
- Nenhuma dependência nova. A câmera do jogo e o save v3 ficam como estão. A N4 não começa.

**Consequências:** docs `development/n3_3/`. O estilo continua sem aprovação.

## D-042 — Base fiel à ficha, corrigida à mão (N3.4)

**Contexto:** A N3.3 ficou num manequim de blocos sem rosto. O briefing `DarkCastleR-N-3-4.md` fez de `image.png` e da ficha modular a fonte da verdade e pediu um único quadro lateral do personagem-base antes de qualquer animação.

**Decisão:**
- A base fica em `art-src/n3-4-original-protagonist/`. Não substitui o protagonista publicado.
- Os pixels partem da redução da vista lateral da ficha e foram corrigidos à mão como grade de texto (`source/base_idle.txt`). Não houve sessão em programa gráfico.
- Camisa, cinto, calça, botas e braceletes ficam no corpo, como em `image.png`. Casaca, armadura, capa, elmo, espada e escudo ficam de fora.
- O contrato N3.3 (80×128, pivô `(40, 118)`) é mantido. A câmera 405 reduz o sprite na tela; a troca para 360 é só proposta.
- Nenhuma dependência nova. A câmera, o save v3 e os controles ficam como estão. A N4 e a N5 não começam.

**Consequências:** docs `development/n3_4/`. O estilo continua sem aprovação.

## D-043 — Redução fiel em vez de redesenho, câmera 360 só como proposta (N3.5)

**Contexto:** O proprietário reprovou a N3.4 e apontou como alvo a `ficha (perfil)`, a `ficha lateral` e a `reducao`. A `reducao` é o rascunho automático da N3.4, com a mesma resolução de 106 px.

**Decisão:**
- O candidato fica em `art-src/n3-5-fidelity/` e não substitui nem a N3.4 nem o protagonista publicado.
- Os pixels vêm da redução da ficha lateral (máscara erodida, BOX, paleta da N3.4) com poucas correções à mão. Um teste limita as edições a 16 pixels.
- As resoluções 96×160 e 72×112 ficam como evidência. A recomendação é 80×128 com densidade 2 e câmera 360 (1 texel = 1 px). `camera.ron` não muda sem nova autorização.
- O exemplo isolado ganha o enquadramento `game-360-1x`. Nenhuma dependência nova.

**Consequências:** docs `development/n3_5/`. O estilo continua sem aprovação; a N4 e a N5 não começam.

## D-044 — Corrida a partir do vídeo, folhas só como estudo (N3.6)

**Contexto:** O proprietário publicou folhas de movimento geradas e um vídeo de corrida em fundo verde. O briefing `DarkCastleR-N-3-6.md` pediu uma primeira prova de corrida sem inventar quadros.

**Decisão:**
- As poses das folhas são independentes: não têm ordem, escala nem cabelo comuns. A corrida vem só do vídeo, que tem um ciclo medido de 12 quadros.
- Os quadros passam pela redução da N3.5 numa escala única (quadro de pé = 106 texels), num canvas de 112×128 com pivô (56, 118) e densidade 2. Os quadros com fantasma de interpolação são descartados.
- O exemplo isolado ganha a fonte `n3.6-run-proof`, com sobreposição feita por sprites, sem adicionar features do Bevy. O GStreamer do sistema decodifica o vídeo só no pipeline de arte, e não entra como dependência do projeto.
- O combate, a câmera e o jogo não mudam. Salto, ataque e idle ficam como estudo.

**Consequências:** docs `development/n3_6/`. A velocidade de corrida do jogo (310 u/s) não casa com a passada do vídeo, e a decisão fica com o proprietário. O estilo continua sem aprovação; a N4 e a N5 não começam.

## D-045 — Sincronização da corrida pela apresentação e pipeline por configuração (N3.7)

**Contexto:** O proprietário aprovou a direção da N3.5 e a corrida da N3.6. O briefing `DarkCastleR-N-3-7.md` pediu o estudo do deslizamento dos pés sem mudar a física, contraste, transição e um pipeline reutilizável.

**Decisão:**
- A velocidade da passada é medida pelo pé apoiado em resolução cheia: 157 u/s, contra 310 u/s do jogo. A estimativa de 78 u/s da N3.6 é substituída.
- A correção recomendada usa o contrato existente: `speed_reference` (o clipe acelera com a velocidade, até 2,5×). Não há campo novo nem mudança de física. A passada maior (vídeo novo) fica como alternativa se a cadência não convencer.
- A arte aprovada é só lida; variantes de contraste e a partida do vídeo são arquivos novos. A partida real (quadros 3–13) vira `run_start`; a parada fica sem material e é um corte.
- O método da N3.6 vira `art-src/n3-7-run-sync/scripts/animation_pipeline.py`, guiado por JSON, com proveniência por quadro. Ele tem que reconstruir a N3.6 texel a texel. Continua usando só Pillow e o GStreamer do sistema, sem dependência nova no projeto.

**Consequências:** docs `development/n3_7/`. O exemplo isolado ganha a fonte `n3.7-sync-proof`. Cenário, contraste, transição e câmera 360 esperam o proprietário; a N4 e a N5 não começam.

## D-046 — Motor de personagem modular: corpo aprovado + equipamento preso a ossos (Modular Character Engine 01)

**Contexto:** O briefing `Md-usario/engine/Engine-01.md` suspendeu a geração de imagens completas do protagonista equipado (03D, Prompt 04) e pediu um sistema próprio de personagem 2D articulado, em que um equipamento criado uma vez acompanhe poses e animações.

**Decisão:**
- Abordagem **híbrida**: o corpo continua sendo os quadros aprovados (N3.5 e corrida N3.6), com um esqueleto marcado à mão em cada quadro. Os equipamentos são imagens únicas presas aos ossos. A abordagem totalmente articulada (peças do idle + FK/IK) fica só como comparação, porque perde a qualidade da arte aprovada.
- Para pôr equipamento entre partes do corpo, cada quadro é cortado em 5 camadas (braço de trás, pernas, tronco, cabelo, braço da frente) por cápsulas do esqueleto e pela paleta N3.4. É uma partição: a recomposição é idêntica texel a texel.
- Regras puras em `player::visual::rig`, Bevy só no exemplo isolado. Nenhuma dependência nova, nenhuma mudança no jogo.
- FK e IK foram reescritos em Rust a partir do Agent Sprite Forge e do pixelfx (ambos MIT), com créditos no código e em `docs/development/modular_character_engine/01_ARQUITETURA.md`. JavaScript e Python não entram no jogo.

**Consequências:** um item novo é só dados (RON) e uma imagem. Poses novas do corpo ainda exigem arte. A integração ao `CompositeCharacter`, um osso de pulso e os equipamentos articulados e flexíveis ficam para depois da avaliação do proprietário.
