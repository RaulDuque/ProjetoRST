# D23 — Balanceamento

Nenhum número de HP, ataque, defesa, esquiva ou guarda foi alterado.

## Ficha (nível 1, equipamento inicial)

Jogador: STR/CON/INT 6, LUK 4. HP máximo 50 + 6×10 + 5 = 115.
Espada de ferro (+8) e gibão de couro (+4). ATK 6×2+8 = 20. DEF 6+4 = 10.
Bolsa inicial: 3 `minor_tonic` (+40 HP), 1 `minor_ether` (+30 MP),
20 hearts, 120 coins.

Chefe: HP 280, ATK 18, DEF 10. Melee: alcance 58, wind-up 0,55 s,
cooldown 1,65 s. Fase 2: alcance 400, mínimo 80, wind-up 0,4 s,
cooldown 1,15 s, phase lock 0,85 s.

Fórmula (`src/combat/damage.rs`): `raw * 100 / (100 + DEF)`, variância
0,9–1,1. Média sem crítico: jogador→chefe ~18, chefe→jogador ~16.
Na média, ~16 acertos derrubam 280 e ~8 acertos derrubam 115.
Guarda deixa passar 30% (~5 nesse golpe médio).

## O que o probe mediu

`d23_play_probe`, sem `hp.refill()` e sem abrir o inventário.
Relógio no momento da morte: frame 1074 (passo de 16 ms). Esse relógio
inclui a subida e a aproximação, não só a troca de golpes.

- Ataques do script: 14. Esquivas: 5.
- Jogador: 115 no início da luta; última amostra vivo 15; o quadro
  `10_defeat` está em 0/115.
- Chefe: 280 → 13.
- Slash, lunge, ash wave e fase 2 aconteceram.
- Os 3 tônicos não foram usados.

Números lidos nos quadros: 17 no chefe durante o slash; 19 e 17 no
jogador no phase shift; 27 crítico na ash wave; 18 no quadro da morte.

## Leitura

O script morreu com o chefe a 13 HP e com três tônicos de 40 ainda na
bolsa. Isso é um piso, não um veredito de que a luta está impossível
nem de que está justa para uma pessoa. Não há dado para um retune.
Recomendação: esperar o playtest humano. Se a pessoa, usando guarda e
tônicos, ainda cair sem ler os golpes, o próximo ajuste é de janela ou
de alcance — com o número que ela mediu — e não um corte geral de HP.
