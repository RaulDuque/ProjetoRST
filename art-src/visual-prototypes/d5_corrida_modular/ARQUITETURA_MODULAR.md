# Arquitetura modular D5

## Ideia

```text
AÇÃO run (10 frames, 24 fps)
        │
        ├─ body     (sem coat/cape/sword/pauldron)
        ├─ light    (coat)
        ├─ cape     (half_cape + clasp/cord/stud)  [opcional lag 1 frame]
        ├─ heavy    (pauldron + lame)              [silhueta ainda fraca]
        ├─ weapon   (blade + sword_*)
        └─ props2D  (helm/hat/shield/gauntlet via âncora bbox)
                │
                ▼
        Loadout A / B / C  (mesma ação, stacks diferentes)
```

## Loadouts

| ID | Camadas Blender | Props 2D | Cape lag |
|---|---|---|---|
| A viajante | body+light+cape+weapon | — | 0 |
| B cavaleiro | body+light+heavy+cape+weapon | helm+shield | 1 |
| C misto | body+light+heavy+weapon | hat+shield+gauntlet | 0 |

## Sockets

Publicados (`player_sockets.ron`): weapon, hand_l, head, chest, back — **não
alterados**. No lab, props 2D usam âncoras derivadas da bbox do body por quadro
(mapeamento Bevy↔célula 128×160 dos renders 256 ainda impreciso).

## Ordem de desenho

cape (atrás) → body → light → heavy → weapon → props cabeça/mãos.

## Troca dinâmica

`run_dynamic_swap.gif`: índice de frame `t % 10` contínuo; só muda o stack de
camadas a cada ciclo (A→B→C). A animação corporal **não** reinicia.
