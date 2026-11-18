# D23 — Legibilidade

## Prioridade visual

Em `update_enemy_visuals`, phase lock e wind-up do chefe vencem
`Color::WHITE`. Ordem: phase lock (roxo), senão wind-up do chefe
(laranja / amarelo / ciano), senão flash branco, senão wind-up de inimigo
comum, senão a cor base.

O número de dano continua flutuando, então o acerto ainda se lê.

## A contagem não congela

D22 impedia apagar o wind-up. O `continue` do flash ainda pulava a
contagem, então a cor aparecia e o golpe não saía no tempo que a cor
prometia. Agora um wind-up de chefe já iniciado conta durante o flash.
Sem wind-up, o chefe continua sem decidir um golpe novo enquanto está
atordoado. Inimigo comum continua perdendo o wind-up ao apanhar.

Teste: `boss_tell_counts_down_while_the_hurt_flash_is_up`.

## O que os quadros mostram

| Arquivo | Leitura |
|---|---|
| `05_frontal_slash.png` | Coluna laranja, HUD "Phase I · slash", boss 280/280 |
| `08_tell_under_hit.png` | Corpo laranja com o número 17 e um retângulo claro por cima; HUD ainda "slash"; boss 263/280 |
| `06_lunge.png` | Coluna amarela, HUD "Phase I · lunge"; jogador 98, boss 186 |
| `09_phase2.png` | Coluna roxa, HUD "Phase shift", texto "enters a second form"; jogador 75, boss 130 |
| `07_ash_wave.png` | Projétil ciano, HUD "Phase II · ash wave"; jogador 69, boss 51, crítico 27 |

O quadro da ash wave é o projétil já lançado, não o corpo ciano no
wind-up. O rótulo da barra nomeia o padrão.

## Esquiva e guarda

Nada disso foi alterado. Esquiva: 0,28 s, só a partir da locomoção,
`PLAYER_DODGE_TIME`. Guarda: A segurado, 70% do dano vira lasca, estado
continua `Guarding` (teste da D22). O probe da D23 esquivou 5 vezes e
não segurou A. Uma esquiva no fim do wind-up ainda pode levar o golpe:
a janela ativa do melee é 0,15 s.
