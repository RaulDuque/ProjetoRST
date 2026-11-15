# Protótipo vertical V1.6 — Portões da Vigília

> **[Protótipo]**. Roda com o jogo real (jogador, física, combate, inimigos,
> sprite, animações, VFX, HUD), mais `WorldExplorationPlugin` e o painel do
> mapa. **O jogo principal não mudou:** `cargo run` continua na sala `entry_hall`.

## 1. Como rodar

```bash
cargo run --example world_vertical_slice                    # rota roteirizada + capturas + verificações
cargo run --example world_vertical_slice -- --interactive   # jogar à mão (setas, Z, X, C, A, S)
cargo run --example world_report                            # validação + grafo
cargo test --test world_graph_tests --test world_exploration_tests
```

As saídas vão para `target/world_vertical_slice/`: 14 capturas, `route.log`,
`validation.txt` e `exploration.ron`. Os saves do exemplo também ficam lá
(`saves/` não é tocado).

## 2. Região

```text
 y=2           [  Upper Gallery  ]<──>[     ]
 y=1  [Postern]                       [Watch]
 y=0  [Reliq.][Sealed]#[Approach]<──>[Broken Causeway]<──>[Shaft]<──>[Barbican]~>[Cache]
        x=-2    x=-1     x=0           x=1..2             x=3        x=4        x=5
 #  grade (exige VeilStep)    ~>  parede falsa    Postern ↓ Approach: queda de mão única
```

| Sala | Papel | Conteúdo |
|---|---|---|
| Gate Approach | Entrada (início, junto à grade) | Bloco de 50 u (exige pulo), grade para o oeste, buraco no teto (chegada da poterna) |
| Broken Causeway (2×1) | Corredor, sala com inimigo | `bone_sentry` (inimigo existente) |
| Watch Shaft (1×3) | Sala vertical | Escada de 12 plataformas atravessáveis, porta alta para a galeria |
| Barbican | Sala com passagem secreta | Parede falsa sobre a porta leste |
| Hidden Cache | Secreta, recompensa | "Bundle of tonics" |
| Upper Gallery (2×1) | Recompensa de progressão | **Relíquia do Véu** (concede `VeilStep`) |
| Postern Drop (1×2) | Atalho de retorno | Queda até o teto da entrada (mão única) |
| Sealed Hall | Barreira de progressão | Só depois da relíquia |
| Gate Reliquary | Objetivo | "Gatekeeper's Sigil" (flag `vigil_gates_cleared`) |

São 9 salas, com um ciclo (Approach → … → Postern → Approach). O validador
dá 0 erros e 0 avisos; Cache = `Secret`, as demais = `Main`.

## 3. Requisitos do briefing

| Requisito | Como | Evidência |
|---|---|---|
| Navegação entre salas | Portas recíprocas, `door_transitions` | Testes + rota |
| Ids estáveis | Ids de sala, porta, segredo e coleta nos dados | `world_graph_tests` |
| Transições corretas | Entrada na porta-alvo, fora de sólidos e do gatilho | `going_through_doors_and_back_keeps_one_room_loaded`, `different_doors_lead_to_different_entries` |
| Mapa atualizado | `prepare_room` → `enter_room`; painel redesenhado | Capturas 01–14 |
| Sala visitada registrada | `Discovery::Visited` | Testes |
| Segredo descoberto | Parede ilusória | `the_false_wall_hides_a_room_until_walked_into`, capturas 04–08 |
| Barreira com requisito | `Requirement::Ability(VeilStep)` | `the_barrier_blocks_until_the_relic_and_then_opens` |
| Requisito desbloqueado em teste | Relíquia real nos dados (sem transformação nem física nova) | Idem + captura 10 |
| Retorno à sala anterior | Portas recíprocas; poterna de mão única | Testes + capturas 08 e 12 |
| Sem perda de estado | Estado no recurso; arquivo RON | `exploration_survives_a_restart_through_its_file` |
| Gameplay existente intacto | O plugin não está no `GamePlugin`; reações só a `RoomChanged` | `the_game_without_the_plugin_keeps_its_single_room`; autoplay 23/23; visual_check 30/30; todos os testes antigos |

## 4. Rota roteirizada (resultado real)

PASS: 9/9 salas, segredo 1/1, `VeilStep` e `vigil_gates_cleared` obtidos,
mapa em 100 %.

**Com entrada e física reais:**
- pulo do bloco da entrada;
- caminhada até o corredor;
- entrada na parede falsa e ida ao esconderijo;
- as duas coletas;
- a queda pela poterna até o teto da entrada;
- a volta pela grade já aberta (com pulo);
- salão selado e relicário.

**Atalhos de teste** (registrados como `SHORTCUT` no `route.log`):
- 6 colocações na soleira de uma porta;
- 2 posicionamentos dentro de uma sala;
- usados para a escalada do poço e algumas caminhadas longas;
- a troca de sala é sempre feita pelo sistema de portas do jogo.

**A escalada do poço:**
- não é roteirizada com teclas;
- os degraus têm desnível ≤ 120 u, abaixo do pulo de 136 u;
- dá para jogá-la no modo interativo.

**Inspeção visual:** as capturas foram abertas e conferidas. Entre o que se vê:
a grade azul, a parede falsa esmaecida, a coleta dourada, o painel do mapa e a
queda da poterna. O inimigo do corredor não aparece na captura 02, porque
patrulha fora do quadro naquele momento.

## 5. Testes

| Arquivo | Testes | O que cobre |
|---|---|---|
| `src/world/graph.rs` | 3 | Requisitos `All`/`Any`, aberturas nas paredes, gatilhos só na abertura |
| `tests/world_graph_tests.rs` | 16 | O mundo real válido e classificado; inimigos e entradas seguras; ids duplicados; portas sem destino, não recíprocas, desalinhadas; desconexão e região inacessível; ciclos válidos; requisitos impossíveis; alternativas (`Any`); segredos opcionais e secretas expostas; progressão sem a relíquia; sala sem saída; descoberta, barreiras, porcentagem, arquivo RON |
| `tests/world_exploration_tests.rs` | 10 | Jogo sem o plugin intacto; início na porta inicial; ida e volta 5× sem crescer entidades ou inimigos; portas diferentes, entradas diferentes; barreira e relíquia; parede falsa e volta; respawns e projéteis descartados na troca; persistência por arquivo; parede não é porta; **movimento real** (pulo + caminhada até o corredor, queda pela poterna) |

## 6. Desempenho

| Item | Valor |
|---|---|
| Mundo em memória | 9 salas de metadados (poucos kB) |
| Entidades por sala | Só a sala atual: 9 a 21 `RoomEntity` medidos (Barbican 9, Approach e Causeway 11, Shaft 21: paredes, plataformas, extras, inimigos) |
| Transição | Despawn e spawn dessas entidades, uma vez; nenhum carregamento de arquivo (o mundo é lido no início) |
| Por frame | `door_transitions`: testa 1–3 portas; `find_secrets`/`collect_pickups`: 0–1 entidades; `prepare_room` e `open_barriers` retornam cedo; o painel do mapa só é redesenhado quando muda |
| Testes | `world_exploration_tests` roda em ~0,2 s (10 testes, centenas de frames headless) |

## 7. Limitações e riscos

- **Fora do jogo principal:** integrar exige o save v2 e o load entre salas (V1.7).
- **Load entre salas:** `quick_load` dentro do exemplo funciona só na mesma sala (comportamento do save v1).
- **Alcance físico:** o validador não simula pulo; portas inalcançáveis devem ser `entry_only` ou exigir habilidade.
- **Hitboxes de inimigo:** ainda no ar na troca de sala, não são removidas (duram `ATTACK_ACTIVE` = 0,15 s).
- **Coletas de item:** vão para as capacidades de exploração, não para o inventário.
- **Arte:** geometria provisória (retângulos); nenhuma arte final.
