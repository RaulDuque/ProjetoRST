# Design do castelo e do mundo

> Quase tudo aqui é **[Proposta]**. Só a região **Portões da Vigília** existe,
> como **[Protótipo]** de 9 salas ([`V1_6_VERTICAL_SLICE.md`](V1_6_VERTICAL_SLICE.md)).
> Os nomes são originais e podem mudar. Nenhuma disposição de sala de outro jogo é usada.

## 1. Estrutura macro

```text
                 [Observatório do Eclipse]
                          │
 [Torre dos Sete Selos]─[Galerias da Linhagem]─[Torre do Pêndulo Partido]
          │                     │                       │
 [Arquivo Velado]──────[Átrio dos Juramentos]────[Basílica Afundada]
          │                     │                       │
 [Destilaria Rubra]   [Portões da Vigília]*      [Claustro de Espinhos]
          │                     │                       │
 [Casernas de Ferro Frio] [Bosque das Cinzas Mudas] [Fosso dos Condenados]
          │                     │                       │
 [Cisternas Afogadas]────[Vauvrel (cidade)]      [Ossuário Profundo]
          │                                             │
 [Veios de Hematita]────────[Bastião Soterrado]─────────┘
                                 │
                         [Limiar Onírico] (fim de jogo, opcional)
* protótipo V1.6
```

O diagrama mostra adjacências de região, não salas. Cada região tem duas ou
mais conexões, e pelo menos uma volta depois de uma habilidade nova.

## 2. Regiões (proposta)

| Região | Tema | Papel na progressão | Habilidade ou porta típica |
|---|---|---|---|
| **Vauvrel** | Cidade ao pé do castelo, sob neblina | Hub: loja, ferreiro, curandeira, arquivista | — |
| **Bosque das Cinzas Mudas** | Floresta queimada que não cai | Introdução, caminho até os portões | — |
| **Portões da Vigília** | Muralha, ponte quebrada, guaritas | Primeira região do castelo; primeira relíquia | VeilStep (protótipo) |
| **Átrio dos Juramentos** | Salão central de colunas | Hub interno; elevador central | — |
| **Arquivo Velado** | Biblioteca de estantes móveis | Magias de suporte; segredos de leitura | Selos arcanos |
| **Basílica Afundada** | Catedral parcialmente afundada na rocha | Chefe intermediário; poços verticais | Forma de Corvo |
| **Torre do Pêndulo Partido** | Engrenagens e plataformas cíclicas | Plataformas temporizadas | Salto duplo |
| **Claustro de Espinhos** | Jardins murados, estufas | Inimigos venenosos | Égide (veneno) |
| **Destilaria Rubra** | Laboratório alquímico | Alquimista; poções | Égide de Brasa (calor) |
| **Casernas de Ferro Frio** | Quartéis e arsenal | Armas pesadas; inimigos armados | Quebra-muros |
| **Fosso dos Condenados** | Prisões verticais | Forma Espectral (grades) | Forma Espectral |
| **Cisternas Afogadas** | Esgotos e reservatórios | Água: correntes, respiração | Égide da Maré |
| **Ossuário Profundo** | Catacumbas e criptas | Mortos-vivos; chaves de cripta | Chaves |
| **Veios de Hematita** | Minas e cavernas | Escuridão; mineração de material | Luz (magia) |
| **Bastião Soterrado** | Fortaleza subterrânea | Chefe de região; atalhos para o Ossuário | Forma de Fera |
| **Galerias da Linhagem** | Salões nobres e retratos | Lore; quadros-portal | Teleporte |
| **Torre dos Sete Selos** | Torre dos magos | Sete selos espalhados (opcional) | Sete selos |
| **Observatório do Eclipse** | Cúpula astral | Clímax | Todas |
| **Limiar Onírico** | Plano de sonho, salas que se repetem com variações | Pós-jogo opcional | — |
| **Câmara do Eco** | Região secreta | Segredo de longo prazo | Combinação de formas |

São 20 regiões, dentro da meta de 15 a 25.

## 3. Princípios de layout

- **Densidade:**
  - de 30 a 60 salas por região;
  - 15 % a 25 % das salas opcionais ou secretas;
  - pelo menos 1 atalho por região que reconecte ao hub.
- **Verticalidade:**
  - cada região tem pelo menos um poço (sala 1×N);
  - conexões verticais usam portas `Top`/`Bottom`, e quedas sem volta usam `entry_only`.
- **Labirinto com legibilidade:**
  - corredores longos (salas 3×1 a 6×1) alternam com salas de decisão (3 ou mais portas);
  - nenhuma região tem só um caminho.
- **Retorno:**
  - toda porta bloqueada é vista antes de a habilidade existir (o mapa mostra a sala como descoberta);
  - é a promessa de volta.
- **Segredos autorais:**
  - posição fixa nos dados;
  - aleatoriedade só no loot de inimigos (com seed);
  - nunca na existência de um segredo.
- **Salas especiais:** save, teleporte, loja (só em Vauvrel e em 1 ou 2 pontos
  do castelo), NPC, chefe (sala-arena com portas que fecham) — **[Planejado]**,
  como campo `features` em `WorldRoom`.

## 4. Métricas de mapa

| Métrica | Alvo |
|---|---|
| Células por sala | 1 a 6 (cada célula = 1280×720 u, uma tela na câmera 1:1) |
| Salas de save | 1 a cada 12–20 salas; nunca mais de 3 minutos de caminhada entre saves |
| Teleportes | 1 por região, no ponto mais distante do hub |
| Salas sobrepostas no mapa | Permitidas (camadas: torres internas); o validador só exige que portas encostem |

## 5. Portões da Vigília (protótipo)

Veja [`V1_6_VERTICAL_SLICE.md`](V1_6_VERTICAL_SLICE.md) §2. O protótipo inclui:
- entrada e corredor com inimigo;
- poço vertical;
- barbacã com parede falsa e esconderijo secreto;
- galeria superior com a relíquia;
- poterna (queda de mão única, atalho de volta);
- salão selado (exige VeilStep);
- relicário (objetivo).
