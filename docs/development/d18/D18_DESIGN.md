# D18 — Design

## Fluxo

1. Jogador abre inventário (`I`).
2. Linhas: 8 slots → equipamentos da bolsa → consumíveis.
3. `Enter` em consumível → `UseItemRequest`.
4. `apply_use_item_requests` chama `use_consumable` (puro).
5. Sucesso → restaura recurso, remove 1, `ItemUsed` + `Notification`.
6. Recusa (cheio / morto / ocupado / sem item) → notificação, nada muda.

## Política de estado

| Contexto | Permitido |
|---|---|
| Inventário aberto | Sim (exceto `Dead`) |
| Playing + locomoção | Sim |
| Playing + ataque/cast/etc. | Não (`Busy`) |
| `Dead` | Não |

## Atomicidade

1. Validar efeito aplicável (`current < max` do recurso alvo).
2. Aplicar `restore` (já faz clamp).
3. Só então `inventory.remove(id, 1)`.

Recurso já cheio → `NoEffect`, quantidade intacta.

## Dados

- `minor_tonic` — RestoreHp(40) (já existia).
- `minor_ether` — RestoreMp(30) (novo; start 1×; loja Hessa).

## Fora de escopo

Viagem rápida, arte, animações de uso em combate, hotbar, bump Save V3.
