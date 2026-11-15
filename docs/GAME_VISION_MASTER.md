# Visão mestre — Dark Castle RPG

> Documento de direção (V1.6). Define **para onde** o jogo vai; o que já
> existe está em [`ARCHITECTURE.md`](ARCHITECTURE.md) e
> [`WORLD_ARCHITECTURE.md`](WORLD_ARCHITECTURE.md).
>
> **Legenda de status** (usada em todos os documentos da V1.6):
> - **[Implementado]**: existe no jogo principal, testado;
> - **[Protótipo]**: existe e é testado, mas só no exemplo `world_vertical_slice` ou em módulos ainda não ligados ao jogo;
> - **[Planejado]**: arquitetura definida, nada implementado;
> - **[Proposta]**: ideia de design para discussão;
> - **[Decisão pendente]**: precisa da escolha do usuário.

## 1. O jogo

Dark Castle RPG é um **Action RPG Metroidvania 2D de grande escala**:
- um único protagonista, com combate em tempo real e técnicas e magias por sequência de botões;
- um castelo colossal e interconectado, cercado de regiões externas e de um assentamento habitado;
- uma progressão em que **explorar e ficar mais forte são a mesma coisa**:
  - habilidades e formas abrem caminhos;
  - equipamentos, relíquias e níveis tornam esses caminhos sobrevivíveis.

**Inspiração mecânica:** Symphony of the Night, pela profundidade de
exploração, pela interconectividade, pelo ritmo de descoberta e pela variedade
de sistemas. **Tudo é original:** universo, nomes, personagens, mapa,
disposição das salas, arte, música e história.

**Proibido copiar:** mapa, formas, nomes, personagens, músicas, textos, a ideia
do "castelo invertido" e a disposição das salas de Castlevania ou de qualquer
outro jogo. As imagens de referência do usuário servem só para entender a
densidade e a complexidade desejadas.

## 2. Pilares

1. **Descoberta:**
   - o mapa é o centro do jogo;
   - cada região tem segredos, atalhos e caminhos que só fazem sentido depois.
2. **Progressão dupla:**
   - habilidades de exploração (portas) e atributos e equipamentos (força) avançam juntos;
   - nenhum dos dois sozinho termina o jogo.
3. **Combate de comando:** técnicas e magias por sequências, relativas à orientação (sistema atual, preservado).
4. **Mundo autoral e estável:**
   - nada essencial é aleatório;
   - os segredos existem sempre no mesmo lugar;
   - o loot raro pode ter sorte (LCK).
5. **Atmosfera gótica original:** castelo vivo, decadente e cerimonial, com uma cidade que reage ao progresso.

## 3. Universo (proposta)

**[Proposta]** Ao pé do castelo **Vael Morrow** fica **Vauvrel**, a última
cidade de uma linhagem de guardiões. O castelo despertou, e suas regiões se
deslocam a cada eclipse. O protagonista, um espadachim com sangue da antiga
vigília, entra para selar o coração do castelo. Os nomes de regiões estão em
[`CASTLE_WORLD_DESIGN.md`](CASTLE_WORLD_DESIGN.md). A história completa fica
para a V2.4.

## 4. Escala-alvo (capacidade, não meta desta fase)

| Item | Alvo de longo prazo | V1.6 |
|---|---|---|
| Regiões principais | 15 a 25 | 1 (protótipo) |
| Salas | 600 a 1.000 | 9 |
| Habilidades de exploração | 8 a 12 | 1 (simulada) |
| Transformações | 3 a 5 | 0 (especificadas) |
| Categorias de armas | 10 a 12 | 1 (espada) |
| NPCs com serviço | 8 a 15 | 0 |
| Chefes | 20 a 30 | 0 |

- **Estrutura:** a arquitetura (grafo por ids, carregamento por sala, dados em
  RON, validador) foi desenhada para essa escala ([`WORLD_ARCHITECTURE.md`](WORLD_ARCHITECTURE.md) §5).
- **Conteúdo:** ninguém deve gerar salas vazias para atingir números. Tamanho não é qualidade.

## 5. Mapa dos documentos

| Tema | Documento | Status dominante |
|---|---|---|
| Arquitetura do mundo, auditoria | [`WORLD_ARCHITECTURE.md`](WORLD_ARCHITECTURE.md) | Protótipo + Planejado |
| Regiões e estrutura do castelo | [`CASTLE_WORLD_DESIGN.md`](CASTLE_WORLD_DESIGN.md) | Proposta |
| Salas, portas, transições | [`ROOM_SYSTEM.md`](ROOM_SYSTEM.md) | Protótipo |
| Mapa e exploração | [`MAP_EXPLORATION_SYSTEM.md`](MAP_EXPLORATION_SYSTEM.md) | Protótipo |
| Transformações | [`TRANSFORMATION_SYSTEM.md`](TRANSFORMATION_SYSTEM.md) | Planejado |
| Armas | [`WEAPON_EXPANSION.md`](WEAPON_EXPANSION.md) | Planejado |
| Equipamentos e aparência | [`EQUIPMENT_VISUAL_SYSTEM.md`](EQUIPMENT_VISUAL_SYSTEM.md) | Planejado + Decisão pendente |
| NPCs, comércio, economia | [`NPC_COMMERCE_SYSTEM.md`](NPC_COMMERCE_SYSTEM.md) | Planejado |
| Missões e facções | [`QUEST_FACTION_SYSTEM.md`](QUEST_FACTION_SYSTEM.md) | Planejado |
| Persistência do mundo | [`WORLD_PERSISTENCE.md`](WORLD_PERSISTENCE.md) | Protótipo + Planejado |
| Ferramentas de conteúdo | [`WORLD_CONTENT_PIPELINE.md`](WORLD_CONTENT_PIPELINE.md) | Protótipo + Planejado |
| Protótipo vertical | [`V1_6_VERTICAL_SLICE.md`](V1_6_VERTICAL_SLICE.md) | Protótipo |
| Magias (expansão) | [`MAGIC_SYSTEM.md`](MAGIC_SYSTEM.md) §"Expansão (V1.6)" | Planejado |

## 6. Roadmap recomendado

São propostas, não compromissos. A ordem segue as dependências técnicas
encontradas na V1.6:
- o grafo vem antes das transições no jogo principal;
- a persistência v2 vem antes de qualquer conteúdo que precise ser salvo;
- a interação (tecla de usar) vem antes de NPCs e baús;
- os perfis de movimento vêm antes das transformações.

| Versão | Foco | Entregas principais | Depende de |
|---|---|---|---|
| **V1.6** | Fundação do mundo | Grafo, validador, exploração, mapa, protótipo de 9 salas, especificações (**esta fase**) | — |
| **V1.7** | Salas e transições no jogo | `WorldExplorationPlugin` no jogo principal; save v2 com migração (mundo + exploração); load entre salas; salas de save; transição com fade; entrada `entry_hall` integrada ao grafo | V1.6 |
| **V1.8** | Exploração | Interação (tecla), baús e alavancas, paredes quebráveis, relíquias reais, atalhos, teleporte entre salas de teleporte, marcadores no mapa, mapa em tela cheia | V1.7 |
| **V1.9** | Primeira transformação | Perfis de movimento por forma, Forma Espectral completa (física, combate, animação), recurso de forma | V1.8 |
| **V2.0** | Equipamentos | Categorias de armas (3 a 4 primeiro), slots novos, aparência híbrida (capa e arma em camada), técnicas por arma | V1.7 (save v2) |
| **V2.1** | NPCs e comércio | Vauvrel (cidade), diálogos por condição, loja (compra e venda), ferreiro (melhoria) | V1.8 (interação), V2.0 |
| **V2.2** | Conteúdo do castelo | 3 a 4 regiões completas (~80 a 120 salas) com o pipeline de conteúdo | V1.8, V1.9 |
| **V2.3** | Chefes e progressão | Chefes com fases, recompensas, desbloqueios de habilidade, inimigos raros | V2.2 |
| **V2.4** | Missões e narrativa | Missões, estados, NPCs narrativos, história | V2.1 |
| **V2.5** | Expansão do mundo | Mais regiões, Limiar Onírico, exploração avançada, conteúdo opcional | V2.2–V2.4 |

O [`ROADMAP.md`](ROADMAP.md) continua registrando o que está feito, etapa por
etapa. Esta tabela é a direção.
