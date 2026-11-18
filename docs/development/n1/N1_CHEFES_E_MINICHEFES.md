# N1 — Chefes e minichefes

Decisão do proprietário, registrada aqui e ainda não aplicada ao RON: **The Ashen Sentinel é um minichefe de entrada.** Não é o chefe principal do jogo.

O código não foi reclassificado. O encontro continua `boss: Some(...)` em `ashen_sentinel`, com arena `vigil_gates_ashen_court`, fases, barra, `defeated_bosses`, flag `ashen_sentinel_fallen` e recompensa `sentinel_brand`. Save v3 guarda o id no conjunto de chefes derrotados. Tirar o marcador `Boss` agora mudaria spawn, tranca e recompensa sem um campo novo para "minichefe".

## Categorias

| Categoria | O que é | Exemplo hoje |
|---|---|---|
| Comum | patrulha, respawn | `bone_sentry` |
| Elite | o mesmo sistema, número mais alto, sem arena | `ash_warden` pode ocupar esse papel; o dado não tem o rótulo |
| Minichefe | arena pequena, padrão próprio, recompensa única, não reaparece | Ashen Sentinel, na visão nova |
| Principal | arena dedicada, identidade, vários padrões, recompensa que abre região | nenhum |
| Opcional | fora do caminho, recompensa rara | nenhum |

Chefe principal, quando existir, reusa `BossDefinition`: limiar de fase, tranca de sala, barra, flag e item único. O que ele acrescenta é autoria, não uma segunda máquina de estados.

Minichefe pode usar o mesmo encontro. A diferença de desenho é o lugar (sala lateral, como a Court já é) e a recompensa (marca, não a chave da região seguinte).

## Alternativas

Um componente `MiniBoss` paralelo. Duplica lock, HUD e persistência. Recusado.

Gravar já `category: Mini` no RON, com default `Main` para o sentinela atual. É a mudança pequena da N10, e só depois de um teste de save: saves que já derrotaram `ashen_sentinel` continuam sem respawnar o encontro.

## Riscos

Reclassificar antes de ler os saves reais. Quem derrotou o sentinela tem o id em `defeated_bosses` e o `sentinel_brand` na bolsa. A N10 não apaga nenhum dos dois.

Tratar a arena atual como modelo do chefe principal. A Court é uma sala de uma célula, com um vão de descida. Um chefe principal pede sala maior e uma entrada que não dependa de uma escada de cinco pulos. Isso é N6 e N10, não um ajuste de HP.

## Recomendação

Deixar o sentinela como está até a N10. No texto do jogo, a partir de agora, chamá-lo de minichefe. O primeiro chefe principal ainda não tem nome, arena nem padrão, e não começa nesta etapa.
