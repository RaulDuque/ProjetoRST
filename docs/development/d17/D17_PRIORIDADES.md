# D17 — Prioridades

## Matriz (pós-auditoria)

| Prioridade | Item | Impacto | Complexidade | Escolhido |
|---|---|---|---|---|
| P0 | Compilar / iniciar / mover / combater / salvar F5 | Já ok | — | — |
| P1 | Portas/transições | Já ok (V1.7) | — | Não (completo) |
| P1 | **Pontos de save no mapa** | Fecha ciclo Metroidvania | Baixa–média | **Sim** |
| P2 | Consumíveis em jogo | Loop de combate | Média | D18 candidata |
| P2 | Drops de equipamento isolados | Recompensa | Média | Depois |
| P3 | Combos / status | Combate | Alta | V2.4 autorizado à parte |
| P4 | Chefes | Conteúdo | Alta | Futuro |
| P5 | NPCs/missões extras | Narrativa | Alta | Futuro |
| FUTURO | Arte definitiva | Visual | — | Não executar |

## Justificativa da entrega D17

Preferência do briefing (interações/portas) já estava implementada. O próximo
bloqueador do ciclo vertical com melhor custo/benefício era **save point
jogável**, reusando `ObjectKind`, tecla `D`, campos de `WorldProgress` e o
escritor de save existente — sem bump de versão de save.
