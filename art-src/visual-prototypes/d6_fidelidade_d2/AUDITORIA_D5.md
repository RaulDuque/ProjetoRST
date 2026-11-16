# Auditoria D5 (obrigatória antes da D6)

Fontes: docs e artefatos em `art-src/visual-prototypes/d5_corrida_modular/`.

## Checklist técnico

| # | Pergunta | Resultado |
|---|---|---|
| 1 | Camadas independentes? | **Sim** — body/coat/cape/pauldron/sword exportados com `hide_render` |
| 2 | 3 loadouts, mesmo ciclo run? | **Sim** — ação `GAME_run` / clips V1.3, 10 frames @ 24 fps |
| 3 | Troca dinâmica preserva quadro? | **Sim** — `run_dynamic_swap.gif` usa `t % 10` |
| 4 | Equipamentos nos pontos certos? | **Parcial** — malhas seguem ossos; props 2D usavam bbox (impreciso) |
| 5 | Corpo-base compartilhado? | **Sim** — mesmos `camadas/body/run_fXX.png` |
| 6 | Regenerável? | **Sim** — `export_modular_run.py` + `compose_d5.py`; testes ok |
| 7 | Clipping? | **Sim, presente** — coat vs pernas; capa vs braço |
| 8 | Pivôs/dimensões compatíveis? | **Parcial** — 256²→128×160 compartilhado; sockets Bevy não calibrados |
| 9 | Armadura pesada modelada? | **Não** — só `pauldron` + props 2D |
| 10 | Qualidade vs D2? | **Distante** — EEVEE toon ≠ pixel D2 (`DIRECAO_ARTISTICA.md`) |

## Veredito

- **Técnico:** PoC modular de corrida válida (parcial).
- **Artístico:** **Não aprovado.** Testes automáticos ≠ qualidade D2.

## O que a D6 deve atacar

1. Fidelidade visual (masters D6-A/B/C).
2. Armadura pesada com peças reais.
3. Roupa leve com peças separadas documentadas.
4. Troca individual de peças.
5. Props/elmo/escudo coerentes (malha ou camada, não só círculos).
