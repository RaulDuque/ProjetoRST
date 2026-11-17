# D18 — Relatório final

## Entrega

Uso de consumíveis pela tela de inventário (`I` → Enter): HP (`minor_tonic`)
e MP (`minor_ether`), atômico, com feedback e persistência Save V3.

## Aceitação

| Critério | Estado |
|---|---|
| Auditoria registrada | Concluído |
| Uso HP/MP funcional | Concluído |
| Recusa sem consumir | Concluído |
| UI + notificação | Concluído |
| Persistência sem bump | Concluído |
| fmt / clippy / test / check | Concluído |
| Arte / Hunyuan / white_mesh | Preservados |
| Sem commit / push | Concluído |

## Como testar manualmente

1. `cargo run` (ou `--features dev`).
2. Tomar dano (inimigo) ou gastar MP (magia).
3. `I` — descer até `[use]` (após equipamentos da bolsa).
4. Enter — notificação “Used … (+N HP/MP)”; quantidade cai.
5. Com HP/MP cheios, Enter de novo — “Already at full”; qty intacta.
6. `F5` / `F9` — quantidades e vitais batem.
7. Loja de Hessa vende Minor Ether; start traz 1×.

## Próxima etapa (D19)

**Viagem rápida entre altares descobertos** (`discovered_save_points` da D17).

## Arte futura

**FUTURO — NÃO EXECUTAR AGORA.**
