# Ataque básico — fases

Contrato confirmado: `attack_basic` = **7** frames, `FitToAction`,
`attack_time` Iron Sword = **0,28 s**, `HITBOX_ACTIVE_FRACTION` = **0,6**.

## Visual (D3)

| Frames | Rótulo visual |
|---|---|
| 0–1 | ant (preparação) |
| 2–4 | active (corte) |
| 5–6 | rec (recuperação) |

## Mecânico (jogo atual)

Sem `startup` na Iron Sword, a hitbox pode causar dano **desde o início**
do strike. Os rótulos “ant” **não** movem a janela mecânica.

Não alterar timings para acomodar a arte.
