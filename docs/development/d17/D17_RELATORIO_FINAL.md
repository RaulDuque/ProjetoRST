# D17 — Relatório final

## Entrega

Pontos de salvamento jogáveis (`ObjectKind::SavePoint`) no Gate Approach,
integrados a cura HP/MP, `WorldProgress`, gravação do slot e mapa.

## Aceitação

| Critério | Estado |
|---|---|
| Auditoria registrada | Concluído |
| Uma entrega funcional | Concluído |
| Testes + fmt/clippy/check | Concluído |
| Arte / Hunyuan / white_mesh preservados | Concluído |
| Sem commit / push | Concluído |

## Como testar manualmente

1. `cargo run` (ou `--features dev`).
2. Em Gate Approach, ir até o altar ciano (~x 340–380, chão).
3. `D` — notificação de descoberta/rest; HP/MP cheios; save escrito.
4. `M` — marca ciano na sala visitada após descoberta.
5. Ferir-se, `D` de novo — cura e “Rested and saved”.
6. `F9` — progresso do altar permanece.
7. `F5` ainda salva fora do altar.

## Próxima etapa (D18)

Consumíveis pela UI **ou** viagem rápida entre `discovered_save_points`.

## Arte futura

Pipeline visual definitivo documentado como **FUTURO — NÃO EXECUTAR AGORA**
(`D17_ROADMAP_FUTURO.md` + briefing §17).
