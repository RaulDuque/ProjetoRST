# Estimativa de produção (intervalos, não garantias)

Catálogo real: clips em `assets/animations/player.ron` / publish (~22 clips
jogador + VFX). Run = 10 @ 24 fps (confirmado).

## Expandir pipeline modular a todos os clips

| Etapa | Hipótese |
|---|---|
| Modelagem (armadura/elmo/escudo reais) | 3–8 dias |
| Rig/weights extras (capa pesada, plates) | 2–5 dias |
| Animação (já existe para body; retocar secundárias) | 1–3 dias / clip crítico |
| Export camadas (automatizado) | 0.5–1 dia setup + minutos/clip |
| Acabamento 2D D2 por clip × camada | **maior custo**: 2–6 h / clip se pixel manual |
| QA visual | 0.5–1 dia / milestone |
| Integração Bevy (HeldLayer + novas camadas) | 2–4 dias (fora do escopo D5) |

## Custo de um item novo

| Item | Modelagem | Skin/rig | Export | Acabamento D2 | QA |
|---|---|---|---|---|---|
| Espada | 0.5–1 d | baixa (bone sword) | baixo | 0.5–1 d | 0.25 d |
| Escudo | 0.5–1 d | hand.L | baixo | 0.5–1 d | 0.25 d |
| Elmo | 1–2 d | head + hair mask | médio | 1–2 d | 0.5 d |
| Armadura set | 3–6 d | vários ossos | médio | 2–4 d | 1 d |
| Capa nova | 1–2 d | cape bones + lag | médio | 1–2 d | 0.5 d |
| Set completo (A→B) | 1–2 semanas | incluso | incluso | 1+ semana | 2–3 d |

Hipótese: **acabamento D2** domina o cronograma; a modularidade Blender reduz
retrabalho de animação corporal, não elimina arte.
