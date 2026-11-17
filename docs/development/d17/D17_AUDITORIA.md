# D17 — Auditoria

Data: 2026-11-17  
Branch: `art/v1.2-protagonist`  
HEAD: `1115f0d` (`docs(art): add D8 modular 2D atlas prototypes`)  
Rustc: 1.99.1 · Bevy: 0.20.1

## Estado Git (preservado)

Modificado pré-existente: `src/player/visual/mod.rs`  
Não rastreados relevantes: labs D9–D15, `assets-user/`, `assets/experimental/`,
`modular_lab`, briefings em `Md-usario/` (não alterados nesta fase).

Sem `reset`/`clean`/`commit`/`push`.

## Sistemas (código = verdade)

| Área | Estado | Evidência |
|---|---|---|
| Movimentação / estados / câmera | Completo | Roadmap etapa 2 `[x]` |
| Combate básico, comandos, técnicas, magias núcleo | Completo / parcial | Etapas 3–6; combos/status pendentes |
| Progressão EXP/níveis | Completo | `collect_rewards`, tela de status |
| Inventário / equipar | Completo | V2.0–V2.2; consumíveis em jogo pendentes |
| Inimigos genéricos | Completo | Arquétipos extras pendentes |
| Portas / transições | Completo | V1.7 `world::transitions` |
| Interações (baús, alavancas, paredes) | Completo | V1.8; checkbox etapa 2 desatualizado |
| NPCs / comércio | Completo (parcial save economia) | V2.1 |
| Pontos de save no mapa | **Parcial → entregue D17** | Campos existiam sem uso |
| Chefes | Ausente | Etapa 10 |
| Menu principal / pausa | Ausente | Etapa 12 |
| Arte definitiva | Adiada | D11–D16 labs; não executar |

## Ciclo jogável mínimo

Passos 1–11 e 13–15 já tinham caminho no código. O buraco claro no loop
Metroidvania era o **passo 12 (ponto de salvamento no mundo)**: F5/F9
funcionavam, mas `discovered_save_points` / `last_save_point` não eram
preenchidos por gameplay.

## Problemas confirmados (pré-D17)

1. Campos de save point sem escritor de gameplay (`ESTUDO_SOTN_EXPLORACAO` §1.19).
2. Roadmap etapa 2 ainda marcava “Interação” como pendente apesar da V1.8.

## Riscos

- Colocar altares em zonas de fixtures de teste (NPC em x≈120) quebra testes
  de prioridade objeto/NPC — mitigado na posição do altar D17.
- Restringir F5 só a altares quebraria fluxo de desenvolvimento e vários testes;
  F5 permanece livre.
