# N1 — Direção artística

Fantasia sombria gótica, em pixel art. A frase "32/64 bits" é clima e densidade de detalhe, não um limite de sprite, de cor ou de resolução. O tamanho útil já está em `N1_PIXEL_ART_32_64_BITS.md`.

## Decisão

Cada região já tem uma cor no grafo. A paleta futura parte dela, em vez de inventar um segundo mapa de cores:

| Região | Cor no dado | Materiais | Luz |
|---|---|---|---|
| Vigil Gates | roxo acinzentado | pedra, ferro, tocha | quente e baixa, fundo quase preto |
| Forgotten Galleries | verde-cinza | pedra úmida, madeira velha | fria, vinda de frestas |
| Veiled Cloister | azul | véu, pedra clara | difusa, sem sol direto |
| Village of Ashes | ocre | madeira, cinza, tecido | âmbar de lamparina |
| Ashen Court | já é a arena do sentinela | laje, pilar, brasa | um foco alto, chão legível |

Superfície sólida, plataforma de mão única, perigo e objeto usável precisam de silhueta diferente. Hoje a plataforma one-way é um retângulo e o chão é outro. Na arte nova, a borda de cima da plataforma one-way leva um tracinho claro; espinho e lâmina levam o mesmo vermelho seco em todas as regiões, para o perigo não depender da paleta local.

## Camadas

O cenário da V2.3 já separa fundo em parallax, decoração e névoa (`docs/WORLD_VISUAL_V2_3.md`). A ordem que a produção deve respeitar:

1. Fundo distante, sem colisão, parallax lento.
2. Arquitetura do meio, também sem colisão.
3. Geometria jogável, a única que a física lê.
4. Decoração da frente, por cima do personagem, rala.
5. Névoa e partículas, por cima, com alpha baixo.

Inimigo e golpe ficam entre a geometria e a decoração da frente. Se a frente cobrir o corpo, a sala está errada.

## Identidade

Relíquia: objeto pequeno, com um núcleo claro e um nome na notificação que já existe (`Relic found`). Efeito mágico: a cor do elemento que o dado da magia já usa, não um estilo novo por feitiço. Armadilha: o vermelho seco acima. Inimigo: silhueta mais larga que o protagonista e uma cor de região, para não sumir no pilar.

## Alternativas

Uma paleta única para o castelo inteiro. Facilita a produção e apaga a memória do lugar. Ou paletas sem regra de contraste. O fundo e a plataforma passam a competir.

## Riscos

Decoração na frente do golpe. Partícula brilhante em cima do telégrafo do chefe. A D23 acabou de separar a cor do golpe do flash branco; a arte nova não pode desfazer isso.

## Recomendação

Travar estas cinco regras de leitura antes de desenhar a primeira sala nova. A paleta fina de cada região entra com a sala, não num guia separado antecipado.
