# Salvamento

Código: `src/save/serialization.rs` (formato), `save_manager.rs` (disco, std),
`systems.rs` (F5/F9). Testes: `tests/save_tests.rs`.

## O que é salvo (`SaveData`, versão 1)

- Jogador: sala, posição, atributos base, progressão (nível, EXP, pontos), HP/MP/Hearts,
  habilidades e magias aprendidas (com rank e comando revelado), inventário e equipamentos.
- Mundo (`WorldProgress`): portas destrancadas, baús abertos, pontos de save
  descobertos, chefes derrotados, flags de história, último ponto de save.
- Tempo de jogo.

Arquivo: `saves/slot1.ron` na raiz do projeto (ignorado pelo git).

## Save v3 (V2.1.1)

Formato atual: **v3** = v2 + `economy: Option<EconomyRecord>` (moedas e
unidades vendidas pelas entradas limitadas das lojas, por id de loja e item).

| Lido de | Conversão (em memória; o arquivo nunca é reescrito pelo load) |
|---|---|
| v1 | `SaveFileV1` → `upgrade_v1_to_v2` → `upgrade_v2` → `economy: None` |
| v2 | `SaveFileV2` (struct congelado, campos desconhecidos recusados) → `upgrade_v2` → `economy: None` |
| v3 | direto |

`economy: None` = sem economia registrada: o jogo usa `start.coins` de
`character.ron` (120) e estoque cheio, igual a cada load daquele arquivo
(nada se acumula). O primeiro F5 grava v3 com `Some(...)`; antes, o arquivo
antigo é copiado uma única vez para `slot<N>.ron.v<versão>.bak` (nunca
sobrescrito) e para o `.bak` rotativo; a gravação usa `.tmp` + rename.
Detalhes: `NPC_COMMERCE_V2_1.md` §10.

## Versionamento

O documento começa com `version`. Ao carregar:
1. lê só a versão;
2. versão maior que `CURRENT_SAVE_VERSION` → `TooNew` (o arquivo não é tocado);
3. menor que `MIN_SUPPORTED_SAVE_VERSION` → `TooOld`;
4. senão `migrate(version, texto)` converte passo a passo até a versão atual.

Ao mudar `SaveData` de forma incompatível: incremente `CURRENT_SAVE_VERSION`,
mantenha a struct antiga e adicione um braço em `migrate`. Campos novos opcionais
com `#[serde(default)]` não exigem nova versão.

## Segurança dos dados

- Escrita: `slotN.ron.tmp` → cópia do save anterior para `slotN.ron.bak` → `rename`.
  Uma queda no meio nunca deixa o save corrompido nem apaga o anterior.
- Arquivo corrompido → `Corrupted` com a mensagem do parser; o arquivo e o backup ficam intactos.
- Atualizações do jogo nunca sobrescrevem saves automaticamente.

## Controles

F5 salva (não permitido enquanto caído). F9 carrega o slot 1 e retoma o jogo
(funciona também na tela de status e após a morte).

## Pontos de save no mapa (D17)

Objetos `ObjectKind::SavePoint` no grafo de mundo (ex.: `vg_approach_save`
em Gate Approach). Com `D` no alcance:

1. Cura HP e MP (Hearts inalterados).
2. Insere o id em `world.discovered_save_points` (primeira vez).
3. Define `world.last_save_point`.
4. Grava o slot 1 (mesmo escritor do F5).

Mensagem `SavePointActivated` → `save::systems::rest_at_save_point`.
Altares descobertos aparecem no painel do mapa (`M`) em ciano.

O F5 livre permanece para desenvolvimento; “salvar apenas em pontos de
save” continua pendente no roadmap.

## Viagem rápida (D19)

Após o descanso, se existir outro altar em `discovered_save_points`, abre
`GameState::FastTravel`. Enter envia `FastTravelRequest` → `RoomRequest` com
`Arrival::Position` no altar de destino (mesmo ciclo do F9). A viagem **não**
cura, **não** grava o slot e **não** altera `last_save_point`. Esc cancela.
Segundo altar piloto: `vg_causeway_save` (Broken Causeway).
