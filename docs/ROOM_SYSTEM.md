# Sistema de salas (V1.6)

> **[Protótipo]**: formato, validação e transições implementados e
> testados em `src/world/{graph,validation,exploration_systems}.rs`. Usados
> pelo exemplo `world_vertical_slice` e pelos testes, não pelo jogo
> principal. Arquitetura geral: [`WORLD_ARCHITECTURE.md`](WORLD_ARCHITECTURE.md).

## 1. Arquivos

```text
assets/world/world.ron              start_room, start_door, goal_room
assets/world/regions/<região>.ron   (region: RegionDef, rooms: [WorldRoom])
```

- **Fora do `GameData`:** `assets/world/` não é lido por `GameData::load`, que
  continua lendo só `assets/maps/`. Por isso o jogo atual não muda.
- **Carga:** `WorldDefinition::load_dir` carrega o mundo e
  `exploration_systems::install_world` copia as salas para o `GameData` em memória.

## 2. Formato de uma sala (tipos reais)

```ron
(
    id: "vigil_gates_shaft",           // único no mundo, estável
    name: "Watch Shaft",
    region: "vigil_gates",
    role: Required,                    // Required | Optional | Secret
    map: (x: 3, y: 0, w: 1, h: 3),     // células de 1280x720 u; y para cima
    background: (0.05, 0.06, 0.09),
    platforms: [ (min: (560.0, 150.0), size: (240.0, 16.0), one_way: true), ... ],  // só o interior
    enemies: [ (enemy: "bone_sentry", position: (1300.0, 70.0), patrol_half_width: 220.0) ],
    doors: [
        (id: "to_gallery", side: Left, cell: 2, to: (room: "vigil_gates_gallery", door: "from_shaft")),
        (id: "to_sealed", side: Left, to: (...), requires: Some(Ability(VeilStep))),
        (id: "to_cache", side: Right, to: (...), hidden_by: Some("barbican_false_wall")),
        (id: "from_postern", side: Top, to: (...), entry_only: true),
    ],
    pickups: [ (id: "veil_relic", name: "Relic of the Veil", position: (500.0, 70.0), grants: Ability(VeilStep)) ],
    secrets: [ (id: "barbican_false_wall", kind: IllusoryWall, area: (min: (1160.0, 40.0), size: (120.0, 240.0))) ],
)
```

- **Coordenadas:** de sala, com (0, 0) no canto inferior esquerdo, como em `assets/maps/`.
- **Tamanho:** `w·1280 × h·720`, derivado do mapa, sem divergência possível.

## 3. Geometria gerada

`WorldRoom::layout()`:
- **Contorno:** parede esquerda, parede direita, piso e teto com espessura
  `WALL` = 40 u, todos dentro da sala.
- **Aberturas:**
  - porta lateral: um vão de 160 u, a partir do piso da célula indicada;
  - porta de piso ou teto: um vão de 200 u, centralizado na célula.
- **Soleira:** sob uma porta lateral acima da primeira célula é gerada uma
  plataforma de 220 × 20 u, para que a porta seja usável.
- **Interior:** as `platforms` do arquivo vêm depois do contorno.

Assim, conteúdo novo nunca esquece uma parede nem tapa uma porta. Esse
cuidado é testado em `graph.rs` e em `tests/world_graph_tests.rs`, onde
nenhuma entrada cai em um sólido.

## 4. Portas e transições

| Campo | Significado |
|---|---|
| `side` + `cell` | Onde fica a abertura |
| `to` | Sala e porta do outro lado (recíprocas: a porta-alvo aponta de volta) |
| `requires` | `Requirement` tipado. Enquanto não é atendido, uma **barreira** sólida (azulada) fecha a abertura |
| `hidden_by` | Id de um segredo da sala. Até ser encontrado, a porta não aparece no mapa e não pode ser usada |
| `entry_only` | Só chegada (buraco no teto de uma queda). A física não permite usar a porta para sair, e o validador também não |

**Transição:**
- **Quando ocorre:** o centro do jogador passa a menos de 16 u da borda, dentro
  da abertura, numa porta `Open`, com o jogador vivo.
- **Chegada:** o jogador aparece a 72 u da borda da porta-alvo (nunca sobre o
  gatilho dela, o que evita ricochete) e mantém a velocidade.
- **Câmera:** é reposicionada com `snap_to`; nada da câmera global mudou.
- **No jogo principal (V1.7):**
  - a porta escreve um `RoomRequest`, e `world::transitions` aplica no
    máximo um pedido por frame (um load vence portas, a primeira porta vence
    as seguintes);
  - `TransitionLock` evita ricochete entre salas;
  - jogador morto não atravessa portas;
  - hitboxes, projéteis, números de dano e respawns pendentes da sala
    anterior são removidos no mesmo frame.

  Ver [`WORLD_INTEGRATION_V1_7.md`](WORLD_INTEGRATION_V1_7.md).

**Retorno:**
- **Portas comuns:** voltar é usar a porta-alvo, que leva à mesma porta de origem.
- **Mesma sala, portas diferentes:** levam a pontos de entrada diferentes (testado).

## 5. Barreiras, segredos e coletas

- **Barreira:** é criada quando a sala é carregada, se a porta estiver
  `Locked`. Some (entidade e sólido) no frame em que o requisito passa a ser atendido.
- **Parede ilusória:**
  - é desenhada como parede, sem colisão;
  - entrar na área registra o segredo, revela a sala atrás no mapa e deixa a
    parede translúcida (persistente);
  - a porta escondida só passa a funcionar no frame seguinte, então o
    jogador "atravessa" a parede.
- **Coleta:** tocar (≤ 36 u) concede `Ability`, `Item` ou `Flag` uma vez só. Não reaparece na volta.
  - **Itens no protótipo:** um `Item` vai só para as capacidades de
    exploração (chaves), não para o inventário. A ligação com o `Inventory`
    é da V1.8.

## 5.1 Objetos interativos (V1.8)

```ron
objects: [
    (id: "fg_cracked_wall", kind: BreakableWall(breaks_on: AnyAttack),
        area: (min: (0.0, 40.0), size: (80.0, 160.0))),
    (id: "fg_gallery_chest", kind: Chest(tier: Common, reward: Item("minor_tonic")),
        area: (min: (2200.0, 40.0), size: (48.0, 36.0))),
    (id: "fg_hall_lever", kind: Lever(sets: "fg_hall_gate_open"),
        area: (min: (400.0, 146.0), size: (16.0, 40.0))),
    (id: "fg_loose_brick", kind: HiddenSwitch, area: (min: (1150.0, 50.0), size: (24.0, 24.0))),
],
```

- **Parede quebrável e interruptor oculto:** são segredos. Uma porta com
  `hidden_by` igual ao id deles fica fechada com alvenaria até a
  descoberta.
- **Objetos em geral:** aceitam `requires`.
- **Portas:** ganharam `shortcut: true` (atalho).
- **Detalhes:** [`WORLD_EXPLORATION_V1_8.md`](WORLD_EXPLORATION_V1_8.md).

## 5.2 Véus espectrais (V1.9)

```ron
veils: [
    (id: "vc_well_west_veil", door: "to_shrine", kind: Spectral, depth: 80.0),
],
```

- **Posição:** um véu preenche a abertura de uma porta.
  - **Porta lateral:** a abertura mais `depth` unidades (≤ 120) para dentro.
  - **Porta de piso:** a abertura, com `depth` 0.
  - **Porta de teto:** proibido.
- **Layout:** os véus entram no layout como `Platform { barrier: Some(Spectral) }`:
  - sólidos para humanos, inimigos e projéteis;
  - atravessados por um corpo com `Phasing`.
- **Chegada:** quem chega por uma porta com véu aparece depois dele.
- **Passagem:** `world.ron` diz o que deixa passar cada tipo
  (`passages: { Spectral: Ability(VeilStep) }`).
- **Detalhes:** [`TRANSFORMATIONS_V1_9.md`](TRANSFORMATIONS_V1_9.md) §15.

## 6. Validação (`world::validation::validate`)

| Regra | Tipo |
|---|---|
| Ids duplicados de sala, região ou porta | Erro na carga (`WorldError`) |
| Região inexistente, sala sem portas, alvo inexistente (sala ou porta), porta não recíproca, portas desalinhadas no mapa, célula fora do lado, segredo inexistente, as duas pontas `entry_only` | Erro |
| Requisito que nenhuma coleta do mundo concede (`Any` aceita um ramo possível) | Erro |
| Sala nunca alcançável, região sem sala alcançável, objetivo inalcançável sem segredos | Erro |
| Sala `Required`/`Optional` só alcançável por segredo | Erro |
| Sala `Secret` alcançável sem o segredo | Aviso |
| Sala sem nenhuma porta de saída | Aviso |

**Simulação:**
- **Ponto de partida:** a sala inicial, sem nenhuma capacidade.
- **Passo:** atravessa toda porta utilizável e coleta tudo nas salas
  alcançadas. Repete até não mudar mais nada.
- **Duas rodadas:** sem segredos e com segredos. É isso que classifica cada
  sala como `Main`, `Secret` ou `Unreachable`.
- **Salas fechadas no início:** uma sala fechada no começo do jogo **não** é erro.

## 7. Limites

- **Pulo:** o validador não sabe se o jogador alcança uma porta no alto.
  Portas assim devem ser `entry_only` ou exigir uma habilidade. Uma checagem
  de alcance físico (pulo de 136 u) é pendência para a V1.8.
- **Escala:** o carregamento é por sala. Para 1.000 salas, ver [`WORLD_ARCHITECTURE.md`](WORLD_ARCHITECTURE.md) §5.
