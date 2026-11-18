# D20.1 — Testes visuais

**Tipo:** janela real Bevy (não headless).  
**Evidências:** `docs/development/d20_1/evidence/`  
**Log da rota:** `evidence/route.log`

## Comandos

| Comando | Início (local) | Resultado | Duração ≈ |
|---|---|---|---|
| `timeout 20 cargo run` | 10:17:53 | Janela OK, room `vigil_gates_approach` | 20 s |
| `cargo run --example autoplay` | 10:18:45 | **PASS** 23/23 | ~6 s |
| `cargo run --example d20_1_probe` | 10:22–10:23 | **PASS** (0 artificial repositions) | ~75 s sim / ~42 s wall |

Saves isolados: `target/autoplay/saves/`, `target/d20_1/saves/`.

## Catálogo de capturas

| Arquivo | Cenário | Resultado | Limitação |
|---|---|---|---|
| `01_start_approach.png` | Spawn Vigil Gates + HUD | OK | Arte placeholder |
| `02_inventory_consumable.png` | Inventário, cursor no tonic | OK | — |
| `03_altar_approach.png` | Altar `vg_approach_save` | OK | Bloco ciano |
| `04_bone_sentry.png` | Bone Sentry no Causeway | OK | Bloco bege |
| `05_consumable_selected.png` | Tonic selecionado (HP baixo) | OK | — |
| `06_tonic_used.png` | Após Enter (bag 3→2) | OK | — |
| `07_sentry_defeated.png` | Sala limpa | OK | — |
| `08_fast_travel_menu.png` | Menu FT no Causeway | OK | — |
| `09_after_fast_travel.png` | Chegada Approach | OK | — |
| `10_map_altars.png` | Mapa com altares | OK | Zoom esquemático |
| `11_shaft_bottom.png` | Base do Watch Shaft | OK | — |
| `12_bone_bolt.png` | Ledge y≈409, bolt vivo | OK | Inimigo pouco contrastado |
| `13_ash_warden.png` | Gallery, warden + dano | OK | Bloco laranja |
| `14_ash_warden_defeated.png` | Após Fight | OK | — |
| `15_done.png` | Fim da rota | OK | — |
| `autoplay_*.png` | Combate/HUD fixture | OK | Não é mundo Vigil Gates |

## Cenários por sistema

| Área | Veredito visual | Como |
|---|---|---|
| Movimentação / jump / climb | OK | Rota shaft + WalkTo |
| Combate básico / skills | OK | autoplay PASS |
| D17 altar + save isolado | OK | probe + log save |
| D18 consumível | OK | bag 3→2, HP restorável |
| D19 fast travel | OK | menu + `last_save_point` intacto |
| D20 sentry / bolt / warden | OK | EnemyAlive + screenshots |

## Distinção de tipos de teste

| Tipo | Status nesta etapa |
|---|---|
| Compilação (`check`) | Passou |
| Unitário / integração / ECS headless | `cargo test` PASS |
| Janela (`cargo run`) | Passou (smoke 20 s) |
| Interação automatizada com janela | autoplay + d20_1_probe PASS |
| Manual humano prolongado | Não exigido; automação cobriu os fluxos D17–D20 |
