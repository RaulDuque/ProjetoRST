# Arquitetura do mundo (V1.6)

> Status: legenda em [`GAME_VISION_MASTER.md`](GAME_VISION_MASTER.md).
> - **[Protótipo]**: grafo, validação, exploração e o plugin de transições.
>   Existem e são testados, mas **não** estão no `GamePlugin`.
> - **O jogo principal** continua com a sala única `entry_hall` (`cargo run`).

## 1. Auditoria do estado atual (2026-11-13)

Conferida no código, não na documentação.

| Sistema | Implementado | Parcial | Ausente | Reutilizável | Mudanças necessárias |
|---|---|---|---|---|---|
| **Mundo** | `WorldPlugin`: câmera, física e uma sala | — | Regiões, grafo no jogo | `RoomGeometry`, `RoomEntity`, `CurrentRoom` | Registrar `WorldExplorationPlugin` no jogo (V1.7) |
| **Salas** | `RoomDefinition` (geometria, spawn, inimigos), `maps::spawn_room` | Uma sala só, carregada no Startup | Portas no jogo principal | Loader, `Platform`, `EnemySpawn` | **Protótipo:** `WorldRoom` → `RoomDefinition` (paredes geradas) |
| **Transições** | — | **Protótipo** (`exploration_systems::door_transitions`) | No jogo principal | Despawn por `RoomEntity`, `snap_to` da câmera | Fade, load entre salas, salas de save |
| **Mapa** | — | **Protótipo** (painel `ui::world_map`) | Tela cheia, marcadores | Estado de descoberta | Mapa em tela cheia, ícones (V1.8) |
| **Exploração** | — | **Protótipo** (`ExplorationState`, porcentagem) | Integração com o save | — | Save v2 ([`WORLD_PERSISTENCE.md`](WORLD_PERSISTENCE.md)) |
| **Segredos** | — | **Protótipo** (parede ilusória) | Paredes quebráveis, interruptores | `SecretKind` extensível | Interação, dano em parede (V1.8) |
| **Inventário** | `Inventory` com pilhas, consumíveis e drops com LCK | — | Moeda | Tudo | Moeda e itens-chave (V2.1) |
| **Equipamentos** | `Equipment` com 8 slots (Weapon, Offhand, Head, Body, Gloves, Boots, Accessory1/2), categorias `Helmet`, `Armor`, `Gloves`, `Boots`, `Ring`, `Amulet`, bônus de atributo, requisitos | Sem aparência | Braços, pernas, capa, anéis e amuleto como slots próprios | Slots, regras de categoria | [`EQUIPMENT_VISUAL_SYSTEM.md`](EQUIPMENT_VISUAL_SYSTEM.md) |
| **Armas** | `ItemCategory` já tem `Sword`, `Dagger`, `Axe`, `Spear`, `HeavyWeapon`, `Shield`; slot `Weapon`; bônus de ATK; uma espada nos dados (`iron_sword`) | As categorias só decidem o slot: o ataque básico é igual para todas (`BASIC_ATTACK_HITBOX`, `BASIC_ATTACK_TIME`) | Alcance, velocidade, escala e técnicas por categoria; chicote, cajado, duas mãos | `ItemDefinition`, `DamageKind`, `Element`, `AttributeModifiers` | [`WEAPON_EXPANSION.md`](WEAPON_EXPANSION.md) |
| **Magias** | Ember Bolt (projétil), Mending Light (cura), 10 elementos, resistências, reconhecedor de comandos | Efeitos `Projectile`/`Heal`/`MeleeStrike` | Área, buffs, invocação, exploração | Formato de dados único para técnicas e magias | [`MAGIC_SYSTEM.md`](MAGIC_SYSTEM.md) |
| **Transformações** | — | — | Tudo | Máquina de estados, `ActionPhase`, VFX | [`TRANSFORMATION_SYSTEM.md`](TRANSFORMATION_SYSTEM.md) |
| **NPCs** | — | — | Tudo (nem tecla de interação; ver `ROADMAP` Etapa 2) | `Notification`, HUD | [`NPC_COMMERCE_SYSTEM.md`](NPC_COMMERCE_SYSTEM.md) |
| **Comércio** | — | `ItemDefinition.sell_value` já existe nos dados | Moeda, lojas, preços de compra | Inventário, `sell_value` | Idem |
| **Missões** | — | `WorldProgress.flags` (flags livres) | Estados, objetivos | Flags | [`QUEST_FACTION_SYSTEM.md`](QUEST_FACTION_SYSTEM.md) |
| **Save/load** | `SaveData` v1: jogador, `WorldProgress` (portas, baús, pontos de save, chefes, flags), versão com migração | Load só funciona na sala atual (não troca de sala) | Exploração, sala por grafo | Versionamento, migração, backup, armazenamento isolável | Save v2 ([`WORLD_PERSISTENCE.md`](WORLD_PERSISTENCE.md)) |

**Observação:** `WorldProgress` já previa portas, baús, pontos de save e
chefes, mas nada no jogo os escreve. É um contrato vazio, mantido.

## 2. Modelo de grafo **[Protótipo]**

```text
WorldDefinition (assets/world/)
 ├── manifest: start_room, start_door, goal_room          world.ron
 ├── regions: id → RegionDef (nome, cor do mapa)          regions/<região>.ron
 └── rooms: id → WorldRoom
      ├── region, role (Required | Optional | Secret), map (x, y, w, h em células)
      ├── platforms (interior), enemies (EnemySpawn existente)
      ├── doors: id, side, cell, to (room, door), requires, hidden_by, entry_only
      ├── pickups: id, posição, grants (Ability | Item | Flag)
      └── secrets: id, kind (IllusoryWall), área
ExplorationState (por jogador): descoberta por sala, segredos, coletas, concessões
```

- **Nós e arestas:** os nós são salas; as arestas são pares de portas
  recíprocas (A.d → B.e e B.e → A.d).
- **Ciclos:** são permitidos. O protótipo tem um ciclo
  approach → causeway → shaft → gallery → postern → approach.
- **Identificadores estáveis:** cada sala, porta, segredo e coleta tem um id
  textual único, nunca derivado de posição ou de ordem.
- **Requisitos:** são tipados (`Requirement::{Ability, Item, Flag, All, Any}`)
  e avaliados por casamento de padrões. Nenhuma expressão é interpretada em runtime.

## 3. Camadas

| Arquivo | Camada | Papel |
|---|---|---|
| `world/graph.rs` | Pura | Tipos, carga com rejeição de ids duplicados, geometria derivada (paredes, aberturas, gatilhos, entradas, barreiras), `to_room_definition` |
| `world/validation.rs` | Pura | Checagens estruturais e simulação de progressão |
| `world/exploration.rs` | Pura | Descoberta, segredos, coletas, estado das portas, porcentagem, arquivo RON versionado |
| `world/exploration_systems.rs` | Bevy | `WorldExplorationPlugin`, `install_world`, transições, barreiras, segredos e coletas |
| `ui/world_map.rs` | Bevy (UI) | Painel do mapa |
| `enemies/spawning.rs::enter_new_room`, `magic/projectiles.rs::clear_on_room_change` | Bevy | Reações dos donos a `RoomChanged` |

## 4. Fluxo de uma transição **[Protótipo]**

```text
GameSet::Resolution (depois da física):
  door_transitions
    jogador vivo, porta Open e centro do corpo além da borda, na abertura
    → despawn de todo RoomEntity (geometria, inimigos, barreiras, extras)
    → maps::spawn_room(alvo)            (loader existente; insere RoomGeometry e CurrentRoom)
    → jogador na entrada da porta-alvo (velocidade mantida)
    → câmera: snap_to (função existente; a câmera global não mudou)
    → RoomChanged { from, to, door }
  [ponto de sincronização]
  prepare_room   visita, barreiras, paredes falsas, coletas, sólidos das barreiras
  find_secrets → collect_pickups → open_barriers
GameSet::Progression:
  enemies::enter_new_room   descarta respawns pendentes e spawna os inimigos da sala nova
  magic::clear_on_room_change   remove projéteis
```

- **Dono de cada entidade:** o mundo só remove o que marcou como `RoomEntity`.
  Inimigos e projéteis são tratados pelos módulos donos, que reagem à mensagem.
  É a regra de [`ARCHITECTURE.md`](ARCHITECTURE.md).
- **No jogo principal:** ninguém escreve `RoomChanged`, então essas reações nunca rodam.

## 5. Escala e carregamento

| Questão | Decisão (protótipo) | Plano para 600–1.000 salas |
|---|---|---|
| Memória | Só a sala atual tem entidades e geometria. O grafo inteiro (metadados) fica em memória: 9 salas ≈ alguns kB | ~1.000 salas × ~2 kB de metadados ≈ 2 MB. Cabe em memória, e o grafo continua inteiro para o mapa e a validação |
| Geometria | Gerada de `WorldRoom` na entrada | Igual; geometria pesada (tiles) por sala, carregada sob demanda |
| Arte das salas | Retângulos provisórios | Por região: texturas da região carregadas ao entrar nela e liberadas na saída (handles fracos) |
| Arquivos | Um RON por região | Um RON por região (15–25 arquivos), salas longas podem ir para arquivos próprios |
| Validação | Na carga (exemplo) e nos testes | No CI, e no `world_report` a cada mudança de conteúdo |

## 6. Integração com o jogo principal (V1.7) **[Implementado]**

Feita na V1.7, com outra organização que a planejada abaixo (mantida como
registro). O detalhamento está em [`WORLD_INTEGRATION_V1_7.md`](WORLD_INTEGRATION_V1_7.md):

- **Início:** `app::run` carrega e valida `assets/world/`, e o jogo começa em
  `vigil_gates_approach`.
- **`entry_hall`:** continua em `assets/maps/` como fixture e sala legada.
- **Troca de sala:** só por `RoomRequest`, uma por frame
  (`world::transitions`).
- **Save:** o save v2 e o quick load trocam de sala.

Plano original (V1.6):

1. `entry_hall` vira uma sala do grafo (região "Vigil Gates" ou uma região própria).
2. `WorldExplorationPlugin` entra no `GamePlugin`, depois do save v2 ([`WORLD_PERSISTENCE.md`](WORLD_PERSISTENCE.md)).
3. O `quick_load` passa a trocar de sala (hoje ele só move o jogador dentro da sala atual).
4. Transição com fade e trava de entrada de alguns frames.

**Condição de parada respeitada:** nada disso foi feito na V1.6, porque alteraria
o jogo e o save real. Tudo está documentado e aguarda autorização.

## 6.1 Exploração avançada (V1.8) **[Implementado]**

- **Objetos interativos** (`graph::WorldObject`, camada Bevy em
  `world::interactions`): paredes quebráveis, interruptores ocultos, baús
  e alavancas. O estado deles é derivado dos ids do save.
- **Região nova:** Galerias Esquecidas, com 11 salas.
- **Atalhos:** `Door::shortcut`.
- **Validador:** verificações de progressão `NoWayBack` e
  `ShortcutBypassesProgression`.
- **Mapa:** marcas novas.
- **Transição:** fade de entrada.
- **Detalhes:** [`WORLD_EXPLORATION_V1_8.md`](WORLD_EXPLORATION_V1_8.md).

## 6.2 Transformações (V1.9) **[Implementado]**

- **Forma Espectral:** módulo `transform`, liberada pela Relíquia do Véu
  (`Ability::VeilStep`).
- **Véus:** `graph::Veil` em `WorldRoom::veils`, em portas laterais ou de
  piso; o manifesto declara `passages`.
- **Validador:** uma porta com véu exige a passagem; problemas de véu são
  `BadVeil`.
- **Load:** `app::load_world` confere `passages` contra as definições e o
  custo de MP de cada travessia.
- **Região nova:** Claustro Velado, com 2 salas.
- **Detalhes:** [`TRANSFORMATIONS_V1_9.md`](TRANSFORMATIONS_V1_9.md).

## 7. Limitações conhecidas do protótipo

- **Validador e física:** o validador não simula alcance físico (altura de
  pulo, vãos). O protótipo marca como `entry_only` as portas que a física não
  permite usar. Uma anotação de movimento por porta (`requires: Ability(DoubleJump)`
  para subir) é o caminho previsto.
- **Load entre salas:** resolvido na V1.7 (save v2, `RoomRequest`).
- **Hitboxes de inimigo:** corrigido na V1.7. Hitboxes, projéteis e números de
  dano são removidos no frame da troca de sala.
- **Câmera:** troca de sala sem fade (corte seco).
