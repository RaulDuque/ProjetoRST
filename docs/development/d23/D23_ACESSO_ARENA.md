# D23 — Acesso à Ashen Court

## Escada

O teste `jumps_up_the_reliquary_stairs_into_the_court` pede a sala do
Reliquary só para começar no pé da escada (x 630, y 63). Dali em diante
só segura Z, oito pulos parados. Nenhuma reposição artificial.

Resultado: `RouteStatus::Passed`, `artificial_repositions == 0`, encontro
ativo, chefe com HP > 200, jogador no chão com centro entre y 50 e 120
(em cima da plataforma de chegada, não preso no teto).

Quadros: `01_stair_foot`, `02_stair_climb`, `03_stair_top`.

## Chegada

A porta continua `Bottom`. A correção é uma plataforma sólida
`(min: (600, 32), size: (220, 16))`, topo em y 48, de x 600 a 820, cobrindo
o ponto (640, 72) e emendando no chão que começa em x 740. O vão
x 540–600 permanece a descida. O chefe não se moveu.

`04_court_arrival` mostra o jogador de pé nessa plataforma, HP 115/115,
barra do sentinela 280/280, rótulo "Phase I · slash".

## Câmera

A vista tem ~720 px de largura. O chefe está em x 1040. Na porta, e ainda
em `04b_boss_in_frame` (o script disparou cedo demais, com o jogador longe),
o corpo do sentinela fica fora da borda direita. A barra já está na tela.
Do slash em diante (`05` até `10`) os dois cabem no quadro. A câmera global
não foi alterada: o que faltava na entrada é distância, não um defeito de
enquadramento da luta.

## Caminho de uma pessoa

O probe teleporta até o Reliquary. Uma pessoa não entra por aí. A porta
esquerda do Gate Approach exige `VeilStep`. O caminho real é: Approach →
direita pelo Causeway e o Shaft → Upper Gallery, relíquia em (500, 70) →
voltar ao Approach → Forma Espectral (`T`) pela porta do véu → Sealed Hall
→ esquerda até o Reliquary → subir a escada com Z.
