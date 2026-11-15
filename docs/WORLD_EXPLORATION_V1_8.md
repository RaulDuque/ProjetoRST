# Exploração avançada (V1.8)

Status: **implementado e validado**. Pendências e riscos estão nas §20.
Decisão correspondente: D-020 em [`DECISIONS.md`](DECISIONS.md).

## 1. Estado inicial

- **Git:** branch `art/v1.2-protagonist`, HEAD `204f965`. Só o PNG de
  conceito está staged; o índice não foi alterado.
- **`Md-usario/`:** você reorganizou os briefings em `DarkCastleR-F/` e
  `DarkCastleR-V/`. Conferi pelos hashes que o conteúdo é idêntico.
- **Arquivos protegidos:** saves, `.cargo/config.toml` e os assets
  publicados estão com os mesmos hashes da linha de base da V1.7.
- **Linha de base:** 238 testes Rust, 17 Python, autoplay 23/23, visual
  check aprovado, rota V1.7 com 14 trocas e 0 reposicionamentos.
- **Compilação:** sempre com `-j 2` e um comando Cargo por vez. Não houve
  pressão de memória (9 a 11 GiB disponíveis).

## 2. Arquitetura de interações

O contrato é `graph::WorldObject`, puro e lido de RON:

| Campo | Significado |
|---|---|
| `id` | Id estável, único no mundo (o validador confere) |
| `kind` | `ObjectKind`: `BreakableWall { breaks_on }`, `HiddenSwitch`, `Chest { tier, reward }`, `Lever { sets }` |
| `area` | Retângulo na sala: desenho, colisão (parede intacta) e alcance da interação |
| `requires` | `Option<Requirement>` (por exemplo, uma relíquia) |

- **Sala proprietária:** a lista `objects` da sala.
- **Estado:** nunca é guardado no objeto. É derivado dos ids que o save v2
  já tem (`ExplorationState::object_state`):

| Objeto | Estado vem de |
|---|---|
| Parede quebrável, interruptor oculto | `secrets` (o id do objeto é o id do segredo) |
| Baú | `pickups` (recompensas de uma vez) |
| Alavanca | `granted` contém `Flag(sets)` |

- **Resultado:** `ExplorationState::{break_wall, interact}` registram a
  mudança e devolvem `Interaction::{SecretFound, Granted}` numa única
  chamada. Recusas vêm como `InteractError::{AlreadyDone, Sealed,
  WrongAttack, NotAvailable}`, e nesse caso nada é registrado.
- **Camada Bevy:** `src/world/interactions.rs`.
  - `break_walls`, `interact_with_objects` e `sync_object_visuals` rodam
    em `GameSet::Resolution`.
  - `spawn_objects` é chamado no preparo de cada sala.
  - As entidades são `RoomEntity` e morrem com a sala.

## 3. Paredes quebráveis

- **O que quebra:** só ataques reais do jogador. Hitboxes de golpes e
  técnicas contam, e magias também.
  - O projétil de uma magia é removido pelo módulo de magia ao tocar a
    geometria.
  - Para isso, o módulo de magia agora emite a mensagem `ProjectileBlocked`
    (V1.8), e o mundo a trata como um golpe.
  - Isso não é um segundo sistema de dano: a parede não tem vida, e o
    primeiro acerto válido a quebra.
- **Condições (`BreakCondition`):**
  - `AnyAttack`, usada no protótipo;
  - `Kind(DamageKind)`;
  - `Element(Element)`;
  - uma relíquia necessária se expressa com `requires` (a parede responde
    "não cede").
- **Intacta:** é sólida (`room_solids`), e a porta que ela esconde fica
  fechada com alvenaria.
- **Quebrada:** vira escombro translúcido, sem colisão, e a passagem abre.
- **Persistência:** o estado vem de `secrets`. Ao voltar à sala ou após um
  load, a parede continua quebrada e a colisão não volta.
- **Sem efeito na parede:** ataque de inimigo, efeito visual sem hitbox,
  golpe em outra sala, ataque fora do alcance e ataque repetido. Os
  testes do §18 cobrem cada caso.
- **Hitboxes inalteradas:** alcance, dano e duração do ataque são os
  mesmos.

## 4. Segredos

| Método | Exemplo | Descoberta |
|---|---|---|
| Parede ilusória (V1.6) | `barbican_false_wall` | Andar para dentro dela |
| Parede quebrável (V1.8) | `fg_cracked_wall` | Atacar |
| Interruptor oculto (V1.8) | `fg_loose_brick` | Tecla de interação estando sobre ele (não basta estar perto) |

- **Antes da descoberta:** a porta escondida não aparece no mapa, a sala
  secreta não é descoberta e a abertura fica fechada com alvenaria (exceto
  na parede ilusória).
- **Sem aleatoriedade:** a existência da passagem vem dos dados.

## 5. Baús

- **Tipos (`ChestTier`):** `Common`, `Rare` e `Secret` estão em uso;
  `BossReward` já é aceito nos dados, mas não tem uso ainda.
- **Interação:** a tecla de interação abre o baú uma única vez, e ele
  continua aberto ao voltar e após um load.
- **Uma tecla, um objeto:** cada pressão usa no máximo um objeto.
- **Sem duplicação:** o registro e a concessão acontecem numa única
  chamada pura.
  - 100 pressões, 50 retornos e 50 loads não duplicam nada (testado).
  - Abrir e salvar no mesmo frame grava as duas coisas ou nenhuma
    (testado).
  - O item é aplicado em `GameSet::Progression`, e o F5 roda em
    `Presentation`, que vem depois.

## 6. Recompensas

- **Itens:** `Grant::Item` chega ao inventário existente por
  `PickupCollected` (`player::systems::receive_world_items`, a mesma via
  das coletas da V1.7). O inventário não tem limite, e itens
  desconhecidos impedem o mundo de carregar (`app::load_world`).
- **Item novo:** `warden_greaves` (Boots, rare, +3 DEF), no baú raro.
- **Itens existentes:** `minor_tonic` (dois baús) e `ashen_ring` (baú
  secreto).
- **Fora de escopo:** moeda e economia não foram criadas.

## 7. Relíquias

- **Relíquia nova:** o Emblema Astral concede `Ability::AstralKey`, que abre
  selos astrais. Como o `VeilStep`, não tem física própria.
- **Modelo:** uma relíquia é um `Grant::Ability`, entregue por coleta ou
  por baú, e fica em `granted`, fora do inventário (D-019).
- **Relíquias futuras:** basta uma variante nova de `Ability` e uma coleta.
- **Fora de escopo:** a Forma Espectral, o voo e atravessar paredes.

## 8. Mecanismos

- **Alavanca:** `Lever { sets }` define uma flag do mundo uma única vez,
  registrada em `granted` e em `WorldState`, como as coletas da V1.7.
- **Feedback:** a alavanca muda de cor (cinza para verde) e mostra a
  notificação "A mechanism clanks somewhere.".
- **Persistência:** vem da flag.
- **Interruptor:** o oculto está no §4. Placas de pressão estão planejadas.

## 9. Portas

Uma porta pode exigir qualquer `Requirement`:

| Exemplo | Requisito |
|---|---|
| Relicário → galerias | `Flag("vigil_gates_cleared")` (selo da Vigília) |
| Portão da Sala da Alavanca | `Flag("fg_hall_gate_open")` (mecanismo) |
| Selo astral | `Ability(AstralKey)` (relíquia) |
| Chave | `Item(...)`, já suportado pelo modelo, sem uso no protótipo |

- **Fechada:** a barreira é azul e sólida.
- **Aberta:** a barreira some no mesmo frame em que o requisito é cumprido.

## 10. Atalhos

- **Marcação:** `Door::shortcut`.
- **No protótipo:** o alçapão da Ponte de Retorno (`fg_return_bridge` →
  salão selado da Vigília).
  - Fica fechado, como piso firme, até a alavanca da própria ponte.
  - É de mão única: no salão selado a porta é só de chegada.
  - Continua aberto após save, load e reinício (testado e percorrido na
    rota).
- **No mapa:** o atalho aberto aparece em verde.
- **Validador:** os atalhos não podem mudar o que a progressão principal
  alcança (`ShortcutBypassesProgression`).

## 11. Região nova: Galerias Esquecidas

São 11 salas, em `assets/world/regions/forgotten_galleries.ron`:

```text
  y=2  [stair]  [relic chapel]<->[collapsed walk]<~>[hidden oratory]
  y=1  [stair]  [ upper gallery (2) ]<->[lever hall]=>[astral gate]=>[return bridge]
  y=0  [cache]<~[stair]<->[ long gallery (2) ]<->[threshold]<->(relicário da Vigília)
```

| Sala | Papel | Conteúdo |
|---|---|---|
| Gallery Threshold | Required | Entrada, ligada ao relicário pela porta do selo |
| Long Gallery | Required | Corredor, inimigo, baú comum |
| Hollow Stair | Required | Sala vertical (1×3) e parede quebrável |
| Cracked Cache | Secret | Baú secreto |
| Upper Gallery | Required | Recebe a queda do Collapsed Walk |
| Relic Chapel | Required | Emblema Astral |
| Collapsed Walk | Optional | Ponte sobre o piso caído e interruptor oculto |
| Hidden Oratory | Secret | Baú secreto |
| Lever Hall | Required | Alavanca no ressalto e portão |
| Astral Gate | Required | Inimigo e selo astral |
| Return Bridge | Required, objetivo do mundo | Baú raro, alavanca e alçapão-atalho |

Requisitos do briefing atendidos:

- **Ciclo:** Stair → Chapel → Walk → queda → Upper Gallery → Stair.
- **Caminhos:** um principal e um opcional (Walk).
- **Segredos:** dois.
- **Atalho:** um.
- **Barreiras:** selo astral e portão da alavanca.
- **Recompensas:** quatro baús.
- **Ligação com a região anterior:** pelo relicário e pelo atalho.

O `goal_room` do mundo passou a ser `fg_return_bridge`.

A geometria foi feita para a física atual: pulo de cerca de 136 u e
degraus com subida de no máximo 110 u. Gravidade, velocidade, pulo e
collider não mudaram. Os Portões da Vigília ganharam duas portas: a saída
do relicário e a chegada do atalho no teto do salão selado.

## 12. Mapa

- **Novos elementos:**
  - baús das salas visitadas (laranja se fechados, apagados se abertos);
  - atalho aberto em verde;
  - marca de segredo também para paredes quebráveis e interruptores;
  - segunda linha com a porcentagem da região atual.
- **Nada revelado antes:** salas não visitadas mostram só o contorno, e
  portas escondidas e salas secretas só aparecem depois da descoberta.
- **Porcentagem:** a fórmula não mudou.
  - É salas visitadas dividido pelo total de salas de todas as regiões,
    salas secretas incluídas.
  - Uma região nova entra no denominador, por isso o valor global cai de
    100% para 45% ao final da rota da V1.7.
  - Salas opcionais e secretas contam igual.
  - `secrets_total` conta os três métodos de segredo.
  - `region_stats` dá os mesmos números por região.
- **Testes antigos:** os da V1.6 que contavam o recorte da Vigília passaram
  a usar `region_stats("vigil_gates")`, com os valores esperados
  inalterados, mais asserções globais novas (20 salas, 3 segredos).

## 13. Save/load

- **Sem schema novo:** não há save v3. Paredes e interruptores usam
  `secrets`, baús usam `pickups` e alavancas usam `granted`
  (`Flag`).
- **Fixture V2 novo:** `tests/fixtures/save_v2_galleries.ron`, sintético
  e escrito à mão. Ele é decodificado sem migração, faz ida e volta, e
  carrega no jogo nas galerias com todos os objetos.
- **Fixture V1:** o teste da V1.7 foi mantido.
- **Ainda não comprovado:** a compatibilidade com um save histórico real.
  Os saves reais não foram lidos nem copiados.
- **Testado:** cada objeto após save, load e uma sessão nova; save mais
  antigo (o baú volta a fechar e o item sai); ids desconhecidos (mantidos e
  ignorados); 50 ciclos de save e load.

## 14. Câmera

Testes automatizados (`tests/world_camera_tests.rs`), todos verificando que
a vista fica dentro da sala, que o jogador aparece na tela e que o zoom e a
escala não mudam:

- início do jogo;
- entradas pela esquerda, direita, alto e baixo;
- sala pequena, grande e vertical (topo e base);
- quick load em outra sala;
- 20 idas e voltas.

A entrada "por baixo" usa um `RoomRequest` para a porta do piso do
postigo, porque nenhuma porta do mundo é atravessada para cima. O sistema
global de câmera não mudou.

## 15. Transições visuais

- **Fade de entrada:** depois de cada troca de sala bem-sucedida, um overlay
  preto some em `ROOM_FADE_TIME` (0,22 s), pelo `ui::room_fade`.
- **Ordem:** a troca continua em um frame, e o fade só começa com
  `RoomChanged`. Um load que falha não troca de sala nem faz fade.
- **Sem efeito no jogo:** o jogador não é movido, e a entrada não é
  bloqueada (testado: ele anda durante o fade).
- **Camadas:** o HUD e o mapa ficam por cima do overlay.
- **Fade-out antes da troca:** não implementado. Ele exigiria adiar a
  troca alguns frames no ciclo de vida da V1.7 (risco de entidades da sala
  antiga viverem mais tempo). Fica como proposta.

## 16. Testes de progressão

O validador (`world::validation`) agora:

- **Simula objetos:** baús e alavancas concedem, e paredes e interruptores
  são segredos (seus requisitos contam).
- **Expõe funções:** `reachable_from` (cenários com capacidades fixas),
  `stages` (progressão por estágio) e `Rules { secrets, shortcuts }`.
- **Reporta problemas novos:**
  - `DuplicateId`;
  - `ObjectOutOfRoom`;
  - `ImpossibleObjectRequirement`;
  - `UnusedLever` (aviso);
  - `ShortcutBypassesProgression`;
  - `NoWayBack`: de cada sala alcançada, em cada estágio, com as
    capacidades daquele estágio, o início precisa continuar alcançável.

Cenários testados em `world_graph_tests::v1_8`:

- sem e com relíquia;
- segredo não descoberto e descoberto;
- alavanca desligada e ligada;
- atalho fechado e aberto;
- progressão igual com e sem atalhos;
- um mundo quebrado para cada problema novo (o validador acusa todos).

## 17. Rota real

- **Onde está:** `tests/support/galleries_route.rs`, rodada em
  `tests/world_secrets_route_tests.rs` (headless) e em
  `examples/world_secrets_route.rs` (com janela).
- **Percurso:** começa no início do jogo, percorre a rota da V1.7 e depois
  as galerias.
- **Etapas:**
  1. Baú comum.
  2. Inimigo.
  3. Parede quebrada por golpe real.
  4. Baú secreto.
  5. Subida da escada (12 degraus e 2 ressaltos).
  6. Emblema.
  7. Interruptor e oratório.
  8. Queda em ciclo.
  9. Alavanca e portão.
  10. Selo astral e inimigo.
  11. Baú raro.
  12. Alavanca do alçapão.
  13. F5.
  14. Atalho de volta à Vigília.
  15. F9 para a ponte.
  16. Atalho de novo.
  17. Volta às galerias, com a parede ainda quebrada.
- **Resultado:** PASS, com **37 trocas de sala e 0 reposicionamentos
  artificiais**, mapa em 100% (20/20 salas, 3/3 segredos).
- **Saídas:** o log fica em `target/world_secrets_route/route.log` e as 36
  capturas reais do Bevy em `target/world_secrets_route/*.png`.

## 18. Stress

Em `tests/world_objects_tests.rs`, além do stress da V1.7, que continua
passando:

- 100 interações com um baú aberto;
- 50 puxadas de alavanca;
- 50 retornos às salas do baú e da parede (entidades constantes, nenhuma
  hitbox remanescente, nada concedido de novo);
- 50 ciclos de save e load com objetos;
- 100 aberturas do mapa (teste da V1.7).

## 19. Performance (perfil dev, headless)

| Medida | V1.7 | V1.8 |
|---|---|---|
| Salas | 9 | 20 (só a ativa é instanciada) |
| Entidades por sala | 9 a 21 | até 25 (`fg_stair`) |
| Troca de sala | 0,6 a 1,0 ms (1 frame) | ~1,4 ms em média (pedido + 2 frames) |
| Validação do grafo | ~0,01 ms | ~0,35 ms (verificação `NoWayBack`) |
| Save / load | ~0,6 a 1,2 ms | ~0,7 / 0,8 ms (frame do F5 / F9) |
| Frame com interação e redesenho do mapa | — | ~0,8 ms (frame comum: ~0,5 ms) |
| Objetos persistentes | 1 segredo, 3 coletas | 8 objetos, 3 segredos, 8 recompensas |
| Entidades vivas (100 transições) | 47 → 47 | 48 → 48 (+1: overlay do fade) |

- **Leitura de arquivos:** nenhuma por frame.
- **Texturas:** nenhuma gerada em runtime.
- **Mapa:** é redesenhado só quando há mudança.

## 20. Riscos e pendências

- **Custo do `NoWayBack`:** cresce com estágios × salas² e roda só no
  carregamento. Para centenas de salas, convém medir de novo (ou limitar a
  verificação ao `world_report`).
- **Save histórico real:** a compatibilidade com um save real ainda não foi
  comprovada (§13).
- **Fade-out antes da troca:** não implementado (§15).
- **Fixture V2:** é sintético.
- **Arte provisória:** a arte dos objetos é de retângulos coloridos (regra
  do projeto).
- **Tecla de interação:** a interação usa `D`, uma tecla livre. A seta
  `↑` foi descartada porque, no comando `↓ ↑ + ATTACK`, o frame do `↑` já
  não é agachado e usaria objetos sem querer (testado).
- **HUD:** a barra de dicas cobre a faixa do piso na base da tela. Isso já
  existia antes e não foi alterado.
- **Próximas etapas (V1.9, não iniciada):** forma de exploração, placas de
  pressão, baús de chefe, validador de alcance físico e mapa em tela
  cheia.
