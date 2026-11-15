# Integração do mundo ao jogo principal (V1.7)

Status: **implementado e validado** (detalhes e pendências nas §18 e §19).
Decisão correspondente: D-019 em [`DECISIONS.md`](DECISIONS.md).

## 1. Estado inicial

- **Git:** branch `art/v1.2-protagonist`, HEAD `204f965`. Um PNG de conceito
  estava staged e havia alterações não commitadas desde a V1.2-D. O índice não
  foi alterado.
- **Linha de base:** 204 testes Rust e 17 testes Python passando, autoplay 23/23.
- **Jogo principal:** iniciava na sala única `entry_hall` (`assets/maps/`).
- **Grafo V1.6:** existia em `assets/world/` e era usado só pelo exemplo
  `world_vertical_slice` e pelos testes.
- **Hashes:** os arquivos protegidos (`saves/`, `Md-usario/`,
  `.cargo/config.toml`) foram registrados antes e conferidos depois: estão
  idênticos.

## 2. Arquitetura integrada

```text
app::run()
  GameData::load(assets/)
  app::load_world(root, &mut data)    carrega assets/world/, valida (erros recusam
                                      o início), confere inimigos e itens citados
                                      e instala as salas em GameData (install_world)
  app::build_with_world(data, world)  = app::build(data) + activate_world
                                      (recursos WorldGraph + Exploration)
```

- **Um só mundo:** o grafo usa os mesmos `RoomDefinition`, `spawn_room`,
  `RoomEntity`, `RoomGeometry` e o mesmo jogador. Não há uma segunda
  arquitetura de mundo.
- **Plugins sempre registrados:** `WorldExplorationPlugin` (em `WorldPlugin`) e
  `WorldMapUiPlugin` (em `UiPlugin`). Os sistemas deles só agem quando existe o
  recurso `WorldGraph`.
- **`app::build(data)` sem mundo:** continua montando o jogo de sala única. Os
  testes headless antigos, o `autoplay` e o `visual_check` o usam como fixture.
- **Sala inicial:** `world.ron` → `start_room: "vigil_gates_approach"`,
  `start_door: "to_sealed"`.
- **`entry_hall`:** continua em `assets/maps/` como fixture. Também é sala
  legada: `legacy_rooms` leva um save v1 feito nela até a porta inicial.

## 3. Ciclo de vida de salas (`src/world/transitions.rs`)

Só `apply_room_requests` troca a sala ativa. Portas e quick load apenas
escrevem um `RoomRequest`. A ordem dentro do frame é sempre a mesma:

```text
GameSet::Resolution   door_transitions        → RoomRequest (Arrival::Door)
GameSet::Rooms
  RoomStep::Request   save::quick_load        → RoomRequest (Arrival::Position)
  RoomStep::Apply     apply_room_requests     escolhe 1 pedido, valida, despawn de
                                              RoomEntity, spawn_room, posiciona,
                                              snap da câmera, trava portas,
                                              escreve RoomChanged
  RoomStep::React     cada dono limpa o que é seu; inimigos novos nascem;
                      barreiras, segredos e coletas são preparados; portas
                      são reativadas quando o jogador está fora dos gatilhos
```

`GameSet::Rooms` também roda em pausa. Assim, um load feito na tela de status
troca a sala no mesmo frame.

## 4. Transições

- **No máximo uma troca por frame.** Um load vence portas, e a primeira porta
  vence as seguintes (mensagens lidas na ordem em que foram escritas). Os
  demais pedidos são descartados e contados em `RoomChangeStats::discarded`.
- **Validação do destino:** antes de tocar em qualquer entidade, o sistema
  confere a definição da sala e a porta de chegada. Um destino inválido é
  descartado com aviso, e a sala atual fica como estava.
- **`TransitionLock`:** depois de uma troca, as portas ignoram o jogador até
  ele ser visto fora de todos os gatilhos da sala nova. Isso evita ricochete
  entre duas salas.
- **Morte:** um jogador morto não atravessa portas (`door_transitions` checa
  `PlayerState::Dead`).
- **Tipos de conexão:** bidirecional, mão única (`entry_only`), atalho
  (requisito por relíquia), passagem secreta (`hidden_by`), portão bloqueado
  (barreira) e vertical (portas `Top`/`Bottom`). O retorno nunca é presumido,
  o validador usa o grafo real.

## 5. Limpeza de entidades

| Categoria | Marcador | Na troca de sala |
|---|---|---|
| Persistentes | `Player`, `MainCamera`, UI | Mantidas |
| Da sala | `RoomEntity` (geometria, barreiras, segredos, coletas, inimigos) | Despawn em `RoomStep::Apply` |
| Temporárias de combate | `Hitbox` (golpes do jogador e de inimigos, técnicas), `Projectile` (magias) | Despawn em `RoomStep::React` (`combat::hitboxes::clear_on_room_change`, `magic::projectiles::clear_on_room_change`) |
| Visuais no mundo | `DamageNumber` | Despawn em `RoomStep::React` (`ui::damage_numbers::clear_on_room_change`) |
| Respawns pendentes | `PendingRespawns` | Esvaziados em `enemies::spawning::enter_new_room` |

Cada dono limpa o que é seu ao ler `RoomChanged`. O módulo de mundo não
conhece hitboxes nem projéteis.

## 6. Hitboxes (correção crítica)

- **Bug da V1.6:** uma hitbox de inimigo continuava viva por até 0,15 s
  (`ATTACK_ACTIVE`) depois da troca de sala.
- **Correção:** todas as `Hitbox` são removidas no próprio frame da troca, em
  `RoomStep::React`, antes do próximo `GameSet::Combat`. A remoção não depende
  do timer de expiração.
- **Dentro da sala nada mudou:** duração, tamanho e dano continuam iguais.
- **O teste detecta o bug:** com o sistema de limpeza desligado, 3 testes
  falham (`an_enemy_strike_never_survives_a_room_change`,
  `player_strikes_projectiles_and_spells_stay_in_their_room` e
  `quick_load_works_after_death_during_a_skill_with_projectiles_and_mid_transition`).
  Com a limpeza ligada, os três passam.

## 7. Spawn

- **Chegada por porta:** o jogador aparece a 72 u da borda da porta-alvo,
  fora do gatilho dela.
- **Validação:** `arrival_points_are_safe_in_every_room` confere em todas as
  salas e em todas as portas três coisas:
  - o corpo do jogador cabe na sala e não sobrepõe sólidos
    (`restore::position_is_valid`);
  - ele não fica sobre nenhum gatilho de porta;
  - ele está a mais de 200 u, na horizontal, de qualquer inimigo.
- **Fora do teste:** não há checagem automática de chão abaixo do ponto. Nas
  portas `Top` a queda é intencional, e o resto foi coberto pela rota real.
- **Sem mudanças no jogador:** o collider não mudou.

## 8. Save V2 (`src/save/serialization.rs`)

```text
SaveFile { version: 2, data: SaveData {
    player:      PlayerSave  { room, position, atributos, progressão, inventário,
                               equipamentos, técnicas aprendidas, ... },   // igual ao v1
    world:       WorldProgress,                                            // igual ao v1
    exploration: ExplorationRecord { rooms, secrets, pickups, granted },   // novo
    play_time,
}}
```

- **IDs estáveis:** só ids de conteúdo, nunca `Entity` do Bevy.
- **Dados derivados não são salvos:** a porcentagem é recalculada, as
  travas de porta vêm das concessões (`granted`), e a sala atual é
  `player.room`.

## 9. Migração V1 → V2

- **Leitura tipada, sem substituição de texto:** o documento v1 é lido em
  `SaveFileV1`/`SaveDataV1`, structs congeladas com `deny_unknown_fields`.
  Um campo desconhecido é rejeitado, nunca descartado em silêncio.
- **Conversão:** `upgrade_v1` copia todos os campos e cria `exploration`
  vazio.
- **Sala:** a sala do save é resolvida à parte (`world::restore`, §10).
- **Load nunca grava:** `load_versioned` devolve `DecodedSave { data,
  source_version }`, e o arquivo de origem não é tocado.
- **Primeira gravação sobre um v1:** antes de gravar, `SaveStorage` copia o
  arquivo antigo para `slotN.ron.vN.bak` (por exemplo `slot1.ron.v1.bak`).
  Essa cópia nunca é sobrescrita. O `.bak` rotativo continua existindo.
- **Fixture:** `tests/fixtures/save_v1_entry_hall.ron` é sintético, escrito à
  mão no formato v1. Os saves reais não foram lidos nem copiados.
- **V2 → V3 (futuro):** congelar o `SaveFile` atual como `SaveFileV2`, criar
  `upgrade_v2` e um braço `2 =>` em `migrate`. Arquivos v1 passam por
  `upgrade_v1` e depois por `upgrade_v2`.

## 10. Quick load (`src/save/systems.rs`, `src/world/restore.rs`)

O load nunca move o jogador diretamente:

1. Lê e valida o save. Em caso de erro, nada muda.
2. Resolve onde o jogador retoma (`world::restore`, módulo puro):
   - sala do grafo: usa essa sala;
   - sala legada: vai para a porta mapeada;
   - sala desconhecida: vai para a porta inicial do mundo;
   - sem mundo: a sala precisa existir no `GameData` (comportamento do v1).
3. Valida a posição. Se o corpo ficar fora dos limites ou dentro de parede,
   usa o spawn da sala.
4. Substitui o estado persistente: jogador, flags do mundo e exploração.
   O estado salvo **substitui** o atual, nunca se mistura com ele.
5. Pede a sala com `RoomRequest { Arrival::Position }`. A sala é carregada e o
   jogador é posicionado no mesmo frame.

Toda substituição gera uma `PlacementNote`, que aparece na tela e no log.
Nenhuma posição é inventada em silêncio.

## 11. Exploração

`ExplorationState` (em memória) e `ExplorationRecord` (no save) registram
salas visitadas ou descobertas, segredos, coletas e concessões
(`Grant::{Ability, Item, Flag}`). A porcentagem é sempre calculada.

## 12. Relíquias e inventário

| Conceito | Onde fica | Exemplo |
|---|---|---|
| Item de inventário | `Inventory` do jogador | `minor_tonic` da recompensa |
| Relíquia de progressão | `ExplorationRecord::granted` (`Grant::Ability`) | Relíquia do Véu → `VeilStep` |
| Habilidade de exploração | Derivada de `granted`, avaliada por `Requirement` | `VeilStep` abre a grade |
| Flag do mundo | `granted` (`Grant::Flag`) | `vigil_gates_cleared` |

**Decisão:** a Relíquia do Véu é progressão permanente e não ocupa slot de
inventário. Cada coleta é registrada uma vez pelo módulo de mundo (id em
`pickups`). Itens comuns chegam ao inventário por `PickupCollected`
(`player::systems::receive_world_items`). Assim há um único registro por
conceito. `VeilStep` não é a Forma Espectral, que fica para a V1.9.

## 13. Mapa

- **Tecla:** `M` (`InputBindings::map`, antes livre) abre e fecha o painel.
  Está documentada em `INPUT_SYSTEM.md` e na dica do HUD.
- **Conteúdo:** sala atual (borda clara), salas visitadas (preenchidas),
  descobertas (contorno), portas conhecidas (marcas, azuis se trancadas),
  segredo encontrado (marca dourada), porcentagem, salas e segredos.
- **Sem mundo:** o painel fica oculto (jogo de sala única).
- **Redesenho:** só quando a exploração, a sala ou o painel mudam. O mapa não
  é reconstruído a cada frame.

## 14. Segredos

- **Antes da descoberta:** salas secretas e portas `hidden_by` não aparecem
  no mapa (`the_map_never_reveals_secrets_early`).
- **Persistência:** parede falsa encontrada, grade aberta e atalho
  desbloqueado continuam assim depois de save/load. Coletas não reaparecem.

## 15. Rota sem atalhos

- **Onde está:** `tests/support/vigil_route.rs`, compartilhada pelo teste
  `the_main_route_is_playable_with_real_input_and_no_shortcuts` e pelo
  exemplo com janela `world_main_route`.
- **Como funciona:** o `route_driver` só pressiona teclas (andar, pular,
  interagir, F5 e F9). Ele registra sala, posição, ação, tempo e resultado
  em `target/tmp/world_route.log` e `target/world_main_route/route.log`.
- **Contador de reposicionamentos:** a cada frame, conta como artificial
  todo salto de posição maior do que a física permite num frame, desde que
  o jogo não tenha aplicado uma troca de sala (`RoomChangeStats`). O driver
  nunca escreve `Transform`, velocidade ou recursos do jogo.

**Percurso:**

1. Entrada.
2. Corredor (inimigo derrotado).
3. Base do poço.
4. Esconderijo pela parede falsa.
5. Barbacã.
6. Topo do poço.
7. Relíquia do Véu.
8. Postigo.
9. Grade aberta.
10. F5 (save).
11. Salão selado.
12. F9 (load).
13. Relicário.
14. Retorno ao salão selado.

**Resultado:** 14 trocas de sala, **0 reposicionamentos artificiais**, mapa
100%.

**Geometria ajustada (física intacta):** o pulo real alcança 130 a 136 u,
conforme a taxa de quadros. Para a rota manter margem nessa faixa, a
geometria do protótipo foi ajustada em `vigil_gates.ron`:

- primeiro degrau do poço: de 150 para 130;
- desnível entre degraus: no máximo 110 u;
- um obstáculo: de 120 para 100 u de altura.

## 16. Testes

- **Total:** 238 testes Rust (204 da linha de base + 34 novos).
- **Testes novos:**
  - `world_integration_tests`: início no mundo, mundo inválido recusado,
    hitboxes, projéteis e números de dano, morte, duas portas no mesmo frame,
    spawn seguro, load entre salas, load em sala secreta, save v1, saves
    inválidos, mapa e inventário;
  - `world_route_tests`: rota real e combate determinístico no mundo
    integrado;
  - `world_stress_tests`;
  - novos casos em `world_graph_tests` e `save_tests`.
- **Testes antigos:** os 10 da V1.6 continuam passando. Só a montagem do
  harness mudou, porque o plugin agora é registrado pelo jogo. Nenhuma
  asserção foi alterada.
- **Exemplos:** `autoplay` 23/23, `visual_check` PASS, `world_report` e
  `world_vertical_slice` PASS, `world_main_route` PASS com janela.

## 17. Performance (medido em `world_stress_tests`, perfil dev)

| Medida | Resultado |
|---|---|
| Carregar o mundo | 0,2 a 3,3 ms |
| Validar o grafo | ~0,01 ms |
| Entidades por sala | 9 a 21 |
| Frame com troca de sala | 0,6 a 1,0 ms |
| 100 transições | 47 → 47 entidades vivas, RSS sem crescimento (~60 MB) |
| Save | 0,6 a 1,2 ms |
| Porta + load + 1 frame | ~1,1 ms |

- **Por sala:** só a sala ativa é instanciada.
- **Mapa:** é redesenhado só quando há mudança.
- **Arquivos:** o jogo não lê arquivos durante o gameplay, exceto no F9.
- **Custo por frame:** nenhum sistema por frame percorre todas as salas.

## 18. Riscos

- **Margem do pulo:** a geometria do protótipo foi ajustada à física. Salas
  novas precisam respeitar a faixa de 130 a 136 u até existir um validador de
  alcance físico.
- **Fixture sintético:** o fixture v1 foi escrito à mão. Ele segue o formato
  v1 e passa pelo `deny_unknown_fields`, mas não é cópia de um save real (a
  cópia foi negada por segurança).
- **Arquivo de exploração da V1.6:** o formato próprio (`ExplorationState::
  to_ron`) continua no exemplo `world_vertical_slice`. O jogo usa o save v2.

## 19. Pendências

- **Câmera:** não há teste automatizado para cada direção de entrada
  (esquerda, direita, alto, baixo) nem para cada tamanho de sala. Esses casos
  foram validados nas capturas (`world_main_route`, `world_vertical_slice`).
  A câmera agora encaixa no primeiro frame com sala carregada
  (`camera::snap_on_start`); antes ela deslizava a partir da origem.
- **Transição sem fade:** a troca de sala ainda é um corte seco.
- **Validador de alcance físico:** o validador ainda não simula o pulo.

## 20. Próximas etapas

V1.8 (não iniciada): mapa em tela cheia, validador de alcance físico, fade de
transição e pontos de save no mapa.
