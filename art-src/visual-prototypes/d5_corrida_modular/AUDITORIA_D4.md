# Auditoria D4 → o que D5 atacou

Fonte: `art-src/visual-prototypes/d4_modular_d2/LIMITACOES.md` e GIFs D4.

| Problema D4 | Status D5 |
|---|---|
| 1. Corpo-base estreito (AI) | **Mitigado** — body vem do Blender v007 (vest/trousers/boots), silhueta legível |
| 2. Casaco com partes de outras camadas | **Melhor** — `coat` é objeto separado; body exportado sem coat/cape/sword |
| 3. Armadura/capa como silhueta completa | **Parcial** — capa e coat separados; “heavy” ainda é só pauldron (+ props 2D) |
| 4. Chapéu/elmo sem animação por quadro | **Parcial** — props 2D seguem âncora da bbox da cabeça por quadro |
| 5. Corrida = composites inteiros | **Resolvido tecnicamente** — 10 frames × camadas; mesma ação `GAME_run` |
| 6. Sem teste real de rig modular | **Resolvido** — lab blend + hide_render por grupo de malhas |
| 7. Oclusão/alinhamento | **Melhor no idle de frame**, ainda há overlap coat/body e props aproximados |

D4 permanece preservada; D5 não a substitui como arte conceitual D2.
