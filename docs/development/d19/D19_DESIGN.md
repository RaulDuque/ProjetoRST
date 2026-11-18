# D19 — Design

## Fluxo

1. `D` no altar → descanso D17 (cura HP/MP, descoberta, save).
2. Se existir **outro** altar em `discovered_save_points` → `GameState::FastTravel`.
3. ↑/↓ seleciona, Enter confirma, Esc cancela.
4. Confirmação → `FastTravelRequest` → `plan_travel` → `RoomRequest(Arrival::Position)`.
5. Ciclo de salas despawna a sala antiga e posiciona o jogador no altar de destino.

## Política de salvamento

| Ação | Cura | Grava slot | Atualiza `last_save_point` |
|---|---|---|---|
| Descanso (`D`) | Sim | Sim | Sim |
| Viagem rápida | Não | Não | Não |
| F5 | Não | Sim | Não |

## Regras

- Só destinos descobertos; origem ≠ destino.
- Bloqueado se morto, ocupado (não locomoção) ou `TransitionLock`.
- Sem teleporte livre no mapa; menu só no altar.
