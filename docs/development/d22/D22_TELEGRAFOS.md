# D22 — Telégrafos

Placeholders. Sem sprite novo.

| Padrão | Cor do corpo | Barra |
|---|---|---|
| FrontalSlash | laranja | `Phase I · slash` |
| Lunge | amarelo | `Phase I · lunge` |
| Troca de fase | roxo (`phase_lock` 0,85 s) | `Phase shift` |
| AshWave | ciano no projétil | `Phase II · ash wave` |

O flash branco de dano cobre a cor naquele frame. Em `04_phase2.png` a barra diz “Phase shift” e o corpo está branco. Em `02_frontal_slash.png` o retângulo laranja aparece com o HP ainda cheio. Em `03_lunge.png` o retângulo amarelo aparece com o rótulo de lunge. Em `05_ash_wave.png` o projétil ciano está no ar, HP 129/280, aviso “enters a second form”.

O projétil já morre em sólido (`magic/projectiles.rs`). A D22 não tirou um quadro desse impacto.

Hitbox corpo a corpo só existe depois do wind-up (`MELEE_ACTIVE` 0,15 s). O lunge anda a 220 durante o tell e para na parede.
