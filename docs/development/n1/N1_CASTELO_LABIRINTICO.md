# N1 — Castelo labiríntico

O grafo já não é só uma fila de salas de uma tela. A célula do mapa é 1280×720. Uma sala com `map.w` ou `map.h` maior que 1 é mais larga ou mais alta que a vista (cerca de 720×405). O Causeway ocupa duas células. O Shaft liga andares. A câmera segue o corpo dentro da sala. Dá para ter corredor longo sem formato novo.

O que falta é desenho: voltas, portas que esperam uma habilidade, e razão para descer de novo. Hoje são quatro regiões e 29 salas, com um laço real (a ponte das galerias cai no Sealed Hall) e vários trechos de uma célula.

## Decisão

A N6 usa o mesmo grafo, a mesma porta e o mesmo `Requirement`. Não há streaming. Uma sala carrega na transição, como hoje. Streaming só entra se uma região passar do que a memória segura, e isso se mede, não se antecipa.

Sala nova precisa de uma função: exploração, combate, plataforma, armadilha, recompensa, segredo, atalho, narrativa, desafio, transição ou chefe. Alongar o caminho sem função está recusado.

## Mapa conceitual

O que já está no jogo permanece e vira a boca do castelo. Os nomes novos são originais. Nada aqui copia planta de outro jogo, e nada aqui foi colocado no RON.

```text
Village of Ashes (descanso, loja, altar)
        |
   Watch Shaft
        |
Vigil Gates ---- Forgotten Galleries ---- Veiled Cloister
        |                 |
        |            Cinder Stair (vertical, proposta)
        |                 |
        |            Lantern Nave (corredor longo, proposta)
        |              /        \
        |     Drowned Cistern   Quiet Scriptoria
        |              \        /
        |            Ossuary Descent
        |                 |
        |            Thorn Garden
        |                 |
        +-------- Crown Hall (chefe principal, ainda sem nome de pessoa)
        |
   Ashen Court (The Ashen Sentinel, minichefe, já existe)
```

Portões de exemplo, com habilidades que o enum `Ability` já conhece e que o mundo ainda não entrega, salvo `VeilStep` e `AstralKey`:

| Obstáculo | Onde | O que pede | Já existe no código |
|---|---|---|---|
| Véu | Gallery e Cloister | `VeilStep` | sim, e está no mapa |
| Selo astral | Forgotten Galleries | `AstralKey` | sim |
| Parede quebrável | Forgotten Galleries | qualquer golpe | sim |
| Vão largo | Cistern, proposta | `DoubleJump` | o nome existe, ninguém concede |
| Grade | Nave, proposta | `WallBreaker` | o nome existe, ninguém concede |
| Fenda | Scriptoria, proposta | `WraithForm` | o nome existe; a Forma Espectral já é outra coisa |
| Poço | Ossuary, proposta | `RavenForm` | só o nome |

## Segredos e recompensa

O mapa já deixa sala desconhecida e sala secreta fora do desenho até a visita, e não marca baú nem segredo antes da hora (`world_map.rs`). Altar descoberto ganha marca ciano. Percentual e contagem de segredos aparecem no HUD.

Persistência que já vale: baú aberto, segredo achado, flag de mundo, chefe derrotado, altar descoberto. Armadilha futura (espinho, piso falso, pêndulo, lâmina, porta com tempo) é apresentação mais colisor. O piso one-way e a parede quebrável mostram o formato. Não se cria um segundo tipo de save para isso.

Recompensa proporcional: atalho devolve tempo, baú secreto devolve peça rara, minichefe devolve um item único. Moeda e tônico não pagam um chefe.

## Alternativas

Um mapa contínuo sem salas. A transição, o save e o despawn de projétil estão escritos por sala. Trocar isso é uma reescrita, e o tamanho da célula já cobre o corredor longo.

## Riscos

Empilhar regiões novas em cima de portas que o validador exige recíprocas e alinhadas ao mapa. Cada ligação nova precisa dos dois lados. A Ashen Court teve de ficar com porta `Bottom` por causa disso.

Colocar `DoubleJump` numa porta antes de existir a física do pulo duplo. O validador aceita o nome e o jogador nunca passa. A região da N7 usa só o que já funciona: `VeilStep`, `AstralKey`, parede quebrável, alavanca e atalho. O vão do pulo duplo espera a N8.

## Recomendação

Aprovar o desenho como norte da N6. A primeira região construída, na N7, é um laço curto saindo das Forgotten Galleries, com uma sala de duas células e uma volta, sem habilidade nova.
