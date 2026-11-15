# Estudo SOTN — exploração e mundo

Fase 2 da pesquisa. Convenções: [`REFERENCIAS_SOTN.md`](REFERENCIAS_SOTN.md).
Caminhos `src/...`/`include/...` do **sotn-decomp @ `3a0957b`**; os nossos, do
**DCR @ `9f7616d`**. Legenda: **[F]** fato · **[I]** inferência · **[R]**
recomendação · **[?]** pendente.

> Não copiar mapas, salas, distribuição de itens nem nomes de áreas. O
> que interessa são as **regras** de organização.

---

## 0. Resumo

1. O mundo do SOTN é uma **grade de células do tamanho de uma tela**. Cada
   sala é um **retângulo de células**, e as regiões ("estágios") são
   carregadas como **pacotes de código e dados**. **[F]**
2. O mapa marca **duas informações por célula**: visitada e revelada (por
   item de mapa). O total de células visitadas é um número de jogo, usado em
   fórmulas. **[F]**
3. Inimigos são criados **conforme a rolagem revela** a posição deles. Um
   inimigo morto não renasce enquanto o jogador está na sala; ao **trocar de
   sala**, todos voltam. **[F]** Isso fecha a pendência da Fase 1.
4. Segredos, coletas e eventos são **bits persistentes** (`g_CastleFlags`,
   768 bytes). Chefes têm registro próprio. **[F]**
5. O DCR já tem um modelo **mais explícito e validável** (grafo com ids
   estáveis, requisitos tipados, validador de progressão). As oportunidades
   estão em **conteúdo e ritmo**, não em reescrever o modelo.

**Correção da Fase 1:** `PLANO_MELHORIAS_SOTN.md` §1.5 descreve o respawn do
DCR como "por tempo". Na verdade, o DCR **já faz** como o SOTN: recria os
inimigos ao entrar na sala (`src/enemies/spawning.rs::enter_new_room`;
`docs/WORLD_PERSISTENCE.md` §4, "Inimigos reaparecem ao entrar na sala").
**Além disso**, reaparece por tempo (`respawn_time`) **dentro** da sala, o
que o SOTN não faz. O documento da Fase 1 não foi alterado (regra desta
pesquisa); a correção vale a partir daqui.

---

## 1. Os 20 itens

### 1.1 Estrutura de regiões

- **SOTN [F]:** cada estágio é um pacote (`Overlay`, `include/game.h:1403-1420`)
  com funções próprias (`Update`, `HitDetection`, `UpdateRoomPosition`,
  `InitRoomEntities`), a lista de salas (`RoomHeader* rooms`), camadas de
  tiles e bancos de sprites. Os chefes ficam em pacotes de estágio próprios
  (`src/boss/*`). O castelo invertido usa outros estágios (`src/st/r*`).
- **DCR:** um RON por região (`assets/world/regions/*.ron`, quatro regiões),
  grafo único em memória (`docs/WORLD_ARCHITECTURE.md` §2, §5).
- **Classificação:** Implementado adequadamente. Não há motivo para pacotes
  de código por região em Rust.

### 1.2 Identificadores de salas

- **SOTN [F]:** sala = índice na lista do estágio, mais posição na grade do
  castelo (`RoomHeader { left, top, right, bottom, load }`, `game.h:820-826`).
  Entidades de layout têm um índice de sala (`LayoutEntity.entityRoomIndex`,
  `include/stage.h:42-48`).
- **DCR:** ids textuais estáveis por sala, porta, segredo e coleta, nunca
  derivados de posição (`WORLD_ARCHITECTURE.md` §2).
- **Classificação:** Implementado adequadamente (melhor para save e
  ferramentas).

### 1.3 Geometria

- **SOTN [F]:** camadas `fg`/`bg` de tiles (`LayerDef`, `game.h:1357-1370`)
  com definição por tile (`TileDefinition`); a colisão vem do tile. Uma
  entidade pode **trocar tiles em tempo de execução** para abrir ou fechar
  passagens (portas de chefe, `src/boss/bo0/e_boss_doors.c:54-60, 103-114`).
- **DCR:** plataformas retangulares (`RoomDefinition.platforms`,
  `src/world/rooms.rs:54-64`); paredes e aberturas derivadas das portas
  (`world/graph.rs`); barreiras como sólidos extras.
- **Classificação:** Implementado adequadamente para o estágio atual. Tiles
  são uma decisão de arte futura (ROADMAP etapa 11, "carregamento de assets
  por sala").

### 1.4 Transições

- **SOTN [F]:** troca de sala dentro do estágio recarrega a camada, recalcula
  os limites, destrói as entidades do estágio (`DestroyEntitiesFromIndex(0x40)`)
  e chama `InitRoomEntities` (`src/dra/5087C.c:1805-1896`). Entre estágios
  usa `RoomTeleport` (`game.h:828-835`; o comentário cita a sala de
  carregamento do CD). **[I]** As salas-corredor escondem a leitura do disco.
- **DCR:** `RoomRequest` uma por frame, despawn por `RoomEntity`, fade de
  entrada, `RoomChanged` (`WORLD_ARCHITECTURE.md` §4, §6).
- **Classificação:** Implementado adequadamente.

### 1.5 Portas

- **SOTN [F]:** portas são entidades: portas vermelhas comuns (`e_red_door.c`
  em todo estágio), portas seladas por relíquia (`src/st/e_sealed_door.h:182`
  confere `RELIC_JEWEL_OF_OPEN` e mostra uma mensagem), portas de névoa
  (`e_mist_door.h`), portas de chefe (§1.12).
- **DCR:** `Door { side, cell, to, requires, hidden_by, entry_only, shortcut }`
  (`src/world/graph.rs:179-200`), `DoorBarrier`, véus da forma espectral.
- **Classificação:** Implementado adequadamente.
- **[R]** Pequena melhoria de legibilidade: **mensagem ao tocar uma porta
  trancada** dizendo o tipo de requisito (como faz a porta selada), se ainda
  não existir. **[?]** Conferir no DCR antes de implementar.

### 1.6 Limites da câmera

- **SOTN [F]:** `g_Tilemap.left/top/right/bottom` definem o limite de rolagem
  da sala (`game.h:1908-1931`). **Zonas de trava** dentro de uma sala grande
  mudam esses limites conforme o lado de saída (`src/st/e_lock_camera.h`).
- **DCR:** câmera presa ao retângulo da sala (`camera_rules.rs::clamp_axis`),
  zona morta e *look-ahead* (V2.3). Sem zonas de trava.
- **Classificação:** Implementado, mas melhorável.
- **[R]** `camera_zones` opcional por sala (retângulos com limites próprios),
  útil para arenas e salas grandes com "câmaras". Prioridade baixa: hoje as
  salas são pequenas.

### 1.7 Salas verticais

- **SOTN [F]:** sala com mais células na vertical; a rolagem segue o jogador
  dentro dos limites; as entidades são criadas também por faixa vertical
  (`CreateEntitiesAbove/Below`, `src/st/create_entity.h:320-403`).
- **DCR:** portas `Top`/`Bottom`, `entry_only` para quedas, câmera com
  `dead_zone_up/down` e `fall_look_down` (V2.3).
- **Classificação:** Implementado adequadamente.

### 1.8 Plataformas

- **SOTN [I]:** plataformas atravessáveis e rampas vêm dos tiles
  (`TOUCHING_ANY_SLOPE`, Fase 1). Não foi aprofundado. **[?]**
- **DCR:** plataformas sólidas e atravessáveis (Fase 1, §2).
- **Classificação:** Implementado adequadamente (rampas ausentes, por escolha).

### 1.9 Salas secretas

- **SOTN [F]:** salas inteiras só alcançáveis atrás de paredes quebráveis ou
  de tetos falsos. O estado é guardado em bits por segredo
  (`g_CastleFlags[NO2_SECRET_WALL_OPEN]`, com bit 1 = aberta e bit 2 =
  destroços já resolvidos; `src/st/e_secrets_no2.h:178-380`).
- **DCR:** `RoomRole::Secret` (`graph.rs:206-213`), validador garante que a
  rota principal não depende de segredos.
- **Classificação:** Implementado adequadamente.

### 1.10 Passagens ocultas

- **SOTN [F]:** paredes e tetos que quebram com golpes; o mapa só desenha a
  passagem depois de aberta (`RevealSecretPassageOnMap`, `5087C.c:860-895`).
- **DCR:** `IllusoryWall`, paredes quebráveis com `BreakCondition`
  (`AnyAttack`, `Kind`, `Element`), interruptores ocultos, `hidden_by` nas
  portas (V1.8).
- **Classificação:** Implementado adequadamente. O DCR é até mais rico
  (parede que só quebra com um elemento).

### 1.11 Atalhos

- **SOTN [F]:** passagens que se abrem de um lado (ex.: `e_dai_no2_shortcut.c`
  em `src/boss/bo5/`) e salas de teletransporte. Ao visitar uma sala de
  teletransporte, ela é **liberada** (`g_CastleFlags[WRP_UNLOCKS] |= 1 << n`,
  `src/st/wrp/warp.c:141-142`). Usar o teletransporte leva à **próxima sala
  liberada**, em ciclo (`:214-222`).
- **DCR:** `Door::shortcut` aberto pelo lado de lá; validador
  `ShortcutBypassesProgression` (V1.8). Sem teletransporte.
- **Classificação:** Parcial (atalhos sim; viagem rápida não).
- **[R]** Viagem rápida entre **pontos de save descobertos**
  (`discovered_save_points` já existe no save). Escolha de destino por menu,
  não em ciclo. Prioridade média, depois dos pontos de save.

### 1.12 Bloqueios por habilidade

- **SOTN [F]:** checagem de relíquia na própria entidade bloqueadora (porta
  selada) e passagens que só a forma de névoa atravessa.
- **DCR:** `Requirement::{Ability, Item, Flag, All, Any}` tipado
  (`graph.rs:76-84`), véus por forma (V1.9), simulação de progressão no
  validador. `Ability::DoubleJump` e `WallBreaker` existem só como nomes.
- **Classificação:** Implementado adequadamente (modelo). O conteúdo ainda é
  pequeno.
- **[R]** O validador não simula alcance físico (`WORLD_ARCHITECTURE.md` §7).
  Ao criar habilidades de movimento, anotar as portas com o requisito de
  movimento (`requires: Ability(DoubleJump)`), como já previsto.

### 1.13 Backtracking

- **SOTN [I]:** combina habilidades novas com salas antigas, atalhos de mão
  única, teletransportes e o castelo invertido (o mesmo mundo com regras
  novas).
- **DCR:** grafo com ciclos, atalhos, segredos que exigem habilidade, 4 regiões.
- **Classificação:** Implementado, mas melhorável (pouco conteúdo).
- **[R]** Regra de conteúdo: cada habilidade nova deve abrir **pelo menos
  dois** pontos em regiões já visitadas, e o validador pode relatar quantos
  requisitos de cada habilidade existem (métrica, não erro).

### 1.14 Mapa

- **SOTN [F]:** grade de bits `g_CastleMap[0x800]` (`game.h:2211`), **2 bits
  por célula**. Visitar a célula liga um bit e soma 1 a `g_RoomCount`
  (`all_map_check`, `5087C.c:899-918`). Um item de mapa revela células com
  outro bit, sem contar como visita (`5087C.c:1015-1035`, máscara `0xAA`).
  O castelo invertido usa a outra metade da grade.
- **DCR:** `Discovery::{Unknown, Discovered, Visited}` por sala
  (`src/world/exploration.rs:45-50`), painel com a tecla M, porcentagem por
  região, marcadores (V2.2).
- **Classificação:** Implementado adequadamente (os mesmos dois níveis:
  revelada e visitada).
- **[R]** Item de mapa por região que marca salas como `Discovered` (o estado
  já existe). Mapa em tela cheia segue pendente no roadmap.

### 1.15 Salas descobertas

- **SOTN [F]:** `g_RoomCount` é usado em **fórmulas**: bônus de crítico
  ligado a INT soma `√(salas)` (`5D5BC.c:949`) e um equipamento soma
  `salas/60` à defesa (`menu.c:406`). É exibido no menu (`menu.c:1347`).
- **DCR:** porcentagem de exploração e por região (V1.8). Não entra em fórmulas.
- **[R]** Opcional e original: **um item raro** que escala com a porcentagem
  explorada (recompensa para quem explora), via `special_effects` em dados.
  Prioridade baixa.

### 1.16 Eventos persistentes

- **SOTN [F]:** `g_CastleFlags[0x300]` (`include/castle_flags.h:4`): bits para
  segredos, coletas, eventos de história, bestiário e drops raros já vistos.
  Chefes usam o registro de derrota (`ESTUDO_SOTN_CHEFES.md` §1.4).
- **DCR:** `WorldProgress.flags`, `exploration.secrets/pickups/granted`
  (`WORLD_PERSISTENCE.md` §3.1).
- **Classificação:** Implementado adequadamente (ids textuais).

### 1.17 Reaparecimento de inimigos

- **SOTN [F]:**
  - Inimigos são criados **quando a rolagem revela** a posição de layout
    deles (`CreateEntitiesToTheRight/Left/Above/Below`,
    `src/st/create_entity.h:183-403`).
  - Ao morrer, o inimigo liga seu bit em `D_80097428`
    (`PreventEntityFromRespawning`, `src/st/st_common.h:61-67`). O gerador
    pula posições com o bit ligado (`create_entity.h:205, 256, 340, 383`).
  - Os bits são **zerados a cada carga de sala**, antes e depois de
    `InitRoomEntities` (`src/dra/5087C.c:1833-1834, 1843-1844, 1885-1886,
    1895-1896`), e ao iniciar o estágio (`:1061-1062`).
  - **Conclusão [F]:** o inimigo morto não volta enquanto o jogador estiver
    na sala (nem rolando a tela de volta); ao sair e voltar, todos voltam.
- **DCR:** ao entrar na sala, todos voltam (igual ao SOTN). Dentro da sala,
  voltam após `respawn_time` (6 s no `bone_sentry`) (`spawning.rs:118-140`).
- **Classificação:** Implementado, mas melhorável (decisão de design).
- **[R]** Avaliar `respawn_time: 0` (sem respawn dentro da sala) como padrão
  de conteúdo. O sistema já suporta (0 = nunca). Respawn por tempo pode
  ficar para salas de "fazenda" específicas. Decisão do proprietário; só
  dados.

### 1.18 Posicionamento de itens

- **SOTN [F]:** itens fixos do mapa são entidades de layout; cada coleta liga
  um bit persistente (`CASTLE_COLLECTIBLES_100`, `src/st/e_collect.h:1484-1522`).
  Itens raros soltos por inimigos marcam um bit próprio
  (`ENEMY_LIST_RAREDROP_1B0`, `e_collect.h:468-471`).
- **DCR:** `pickups` por sala com `grants`, baús com níveis (`ChestTier`),
  drops por inimigo com LCK.
- **Classificação:** Implementado adequadamente.

### 1.19 Pontos de salvamento

- **SOTN [F]:** salas de save têm gráficos próprios (`g_SaveRoomGfx`,
  `src/dra/d_37d8.c:105, 327`); a gravação passa por `StoreSaveData`
  (`5087C.c:1476-1480`). **[?]** O fluxo completo (cura, animação, menu) não
  foi lido.
- **DCR:** save rápido (F5) em qualquer lugar, recusado em véu
  (`WORLD_PERSISTENCE.md` §3.2). `discovered_save_points` e
  `last_save_point` existem no save, **sem uso**. ROADMAP etapa 11: "Pontos
  de save no mapa" pendente.
- **Classificação:** Parcial.
- **[R]** Pontos de save como objeto de mundo (`ObjectKind`) que: grava,
  restaura HP/MP, marca `discovered_save_points` e aparece no mapa. Decidir se
  o F5 livre continua (útil em desenvolvimento) ou vira opção de
  acessibilidade. Base para a viagem rápida (§1.11).

### 1.20 Progressão não linear

- **SOTN [I]:** várias ordens possíveis entre áreas, limitadas por relíquias,
  com sequência mínima obrigatória.
- **DCR:** `RoomRole::{Required, Optional, Secret}`; validador com simulação
  de progressão, `NoWayBack` e `ShortcutBypassesProgression`.
- **Classificação:** Implementado adequadamente (modelo).
- **[R]** Quando houver mais habilidades: relatório do validador com **as
  ordens possíveis** de obtenção (quantas habilidades podem ser a 2ª, a 3ª
  etc.), para medir a não linearidade.

---

## 2. Tabela-resumo

| Item | Classificação | Melhoria sugerida | Prioridade |
|---|---|---|---|
| Regiões | Implementado adequadamente | — | — |
| Ids de salas | Implementado adequadamente | — | — |
| Geometria | Implementado adequadamente | Tiles: decisão de arte futura | — |
| Transições | Implementado adequadamente | — | — |
| Portas | Implementado adequadamente | Mensagem do requisito (confirmar antes) | Baixa |
| Câmera | Implementado, mas melhorável | `camera_zones` opcional | Baixa |
| Salas verticais | Implementado adequadamente | — | — |
| Plataformas | Implementado adequadamente | — | — |
| Salas secretas | Implementado adequadamente | — | — |
| Passagens ocultas | Implementado adequadamente | — | — |
| Atalhos | Parcial | Viagem rápida entre saves descobertos | Média |
| Bloqueios por habilidade | Implementado adequadamente | Anotar requisitos de movimento | Com as habilidades |
| Backtracking | Implementado, mas melhorável | Regra de conteúdo e métrica | Média |
| Mapa | Implementado adequadamente | Item de mapa por região | Baixa |
| Salas descobertas | Implementado adequadamente | Item que escala com a exploração | Baixa |
| Eventos persistentes | Implementado adequadamente | — | — |
| Respawn | Implementado, mas melhorável | `respawn_time: 0` como padrão de conteúdo | Média (decisão) |
| Itens | Implementado adequadamente | — | — |
| Pontos de save | Parcial | Objeto de save + cura + mapa | **Alta** (fora do V2.4) |
| Não linearidade | Implementado adequadamente | Relatório de ordens possíveis | Baixa |

---

## 3. Impacto das recomendações

| Melhoria | Save V3 | Animações | Controles | Testes |
|---|---|---|---|---|
| Pontos de save | Usa `discovered_save_points`/`last_save_point` (já existem) | Arte do objeto | Tecla de interação existente (D) | Gravar e carregar no ponto; marca no mapa; cura |
| Viagem rápida | Nenhum (lê os campos acima) | Fade existente | Menu | Só destinos descobertos; troca por `RoomRequest` |
| `respawn_time: 0` | Nenhum | Nenhum | Nenhum | Inimigo morto não volta na sala; volta ao reentrar |
| Item de mapa | Usa `exploration` | Nenhum | Nenhum | Salas viram `Discovered`, não `Visited`; % de visita não muda |
| `camera_zones` | Nenhum | Nenhum | Nenhum | Câmera respeita a zona; sai dela suave |

---

## 4. Pendências

- Salas de save do SOTN: fluxo completo não lido. **[?]**
- Rampas e plataformas atravessáveis: só por menção indireta. **[?]**
- Distribuição de inimigos por sala e curva de dificuldade: não estudadas
  (e não devem ser copiadas).
