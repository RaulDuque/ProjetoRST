# DARK CASTLE RPG — MEGA PROMPT V1.6
## Fundação do Mundo, Castelo Gigantesco, Exploração, Transformações, Equipamentos, Magias, NPCs e Progressão Metroidvania

### MISSÃO OFICIAL — NOVA DIREÇÃO ESTRATÉGICA

Você é o arquiteto principal, engenheiro de gameplay, diretor de sistemas e responsável pela continuidade técnica do Dark Castle RPG.

O projeto utiliza:

- Rust 2024.
- Bevy 0.20.1.
- Blender 5.2.4 LTS.
- Arquitetura modular baseada em plugins.
- Dados configuráveis em RON.
- Pipeline de sprites 2D renderizados a partir de modelos 3D.
- Sistema de combate, habilidades, magias, progressão, inventário, inimigos e save/load.
- Protagonista v007 com controlador visual e animações integradas.

**O usuário definiu uma visão estratégica mais ambiciosa para o jogo.**

Dark Castle RPG deverá evoluir para um Action RPG Metroidvania de grande escala, com profundidade comparável à de Castlevania: Symphony of the Night.

O jogo não deverá ser apenas um conjunto de salas com inimigos.

Deverá possuir:

- Castelo gigantesco.
- Mundo interconectado.
- Exploração não linear.
- Mapa complexo e labiríntico.
- Regiões temáticas.
- Cidade habitada.
- NPCs.
- Comerciantes.
- Compra e venda de itens.
- Diversas categorias de armas.
- Equipamentos para diferentes partes do corpo.
- Armaduras que podem alterar a aparência.
- Transformações.
- Magias.
- Técnicas especiais.
- Relíquias.
- Habilidades de exploração.
- Passagens secretas.
- Salas ocultas.
- Portas bloqueadas.
- Atalhos.
- Teletransportes.
- Chefes.
- Inimigos raros.
- Loot.
- Progressão de atributos.
- Missões.
- Sistemas opcionais de criação e melhoria de equipamentos.
- Persistência do estado do mundo.

A inspiração em Symphony of the Night deve orientar a profundidade de exploração, interconectividade, ritmo de descoberta e variedade de sistemas.

**Não copie o mapa, a arte, os personagens, os nomes, a música, a história nem a disposição das salas de Castlevania.**

Crie um universo original.

O nome do projeto permanece Dark Castle RPG.

---

# 1. OBJETIVO REAL DA V1.6

Esta fase possui dois níveis de trabalho.

## Nível A — Planejamento arquitetural completo

Planejar os sistemas necessários para construir o jogo final.

Inclui:

1. Mundo.
2. Regiões.
3. Salas.
4. Mapa.
5. Exploração.
6. Transformações.
7. Equipamentos.
8. Armas.
9. Magias.
10. NPCs.
11. Comércio.
12. Economia.
13. Missões.
14. Relíquias.
15. Segredos.
16. Chefes.
17. Persistência.
18. Ferramentas de criação de conteúdo.

## Nível B — Protótipo vertical limitado

Implementar apenas uma demonstração pequena e verificável da nova arquitetura.

O protótipo deverá demonstrar:

- Algumas salas conectadas.
- Transição entre salas.
- Mapa explorável.
- Uma passagem secreta.
- Uma habilidade de exploração.
- Uma barreira desbloqueável por habilidade.
- Persistência básica de exploração.
- Compatibilidade com o jogador e combate existentes.

Não implemente o castelo completo.

Não crie centenas de salas nesta fase.

Não implemente todos os equipamentos, transformações, NPCs e lojas agora.

O objetivo é construir uma fundação sólida.

---

# 2. REGRAS DE SEGURANÇA

Antes de modificar arquivos, registre:

- Branch.
- HEAD.
- Estado do stage.
- Arquivos modificados.
- Arquivos não rastreados.
- Arquivos ignorados relevantes.
- Hashes dos arquivos protegidos.
- Estado dos assets atuais.
- Estado dos backups artísticos.

O último estado informado era:

- Branch `art/v1.2-protagonist`.
- HEAD `204f965`.
- Um PNG de conceito previamente staged.
- Alterações não commitadas acumuladas desde V1.2-D até V1.5.

Confirme o estado real.

Não presuma que ele permaneceu igual.

É proibido:

- `git clean`.
- `git reset --hard`.
- `cargo clean`.
- `git commit`.
- `git push`.
- Rebase.
- Merge.
- Trocar de branch.
- Criar worktrees.
- Apagar backups.
- Alterar o histórico.
- Desabilitar hooks de segurança.
- Instalar dependências sem autorização.

Não modifique:

- `Md-usario/`.
- `saves/`.
- `.cargo/config.toml`.
- `target/verify/`.
- `target/f4-stage/`.
- Modelos Blender históricos.
- Backups anteriores.
- Arquivos artísticos aprovados.
- Assets publicados sem utilizar o pipeline seguro.

Não faça staging.

Não crie commits.

Não faça push.

---

# 3. PRESERVAÇÃO DO GAMEPLAY

A V1.6 não autoriza reescrever os sistemas existentes.

Preserve:

- Movimento.
- Física.
- Gravidade.
- Colisões.
- Combate.
- Dano.
- Defesa.
- Esquiva.
- Magias existentes.
- Habilidades existentes.
- Reconhecimento de comandos.
- Atributos.
- Experiência.
- Inventário.
- Equipamentos existentes.
- Inimigos.
- Save/load.
- Animações.
- VFX.
- Câmera atual.

O novo mundo deve ser integrado à arquitetura existente.

Se uma mudança estrutural de gameplay for inevitável, documente a proposta e solicite autorização antes de executá-la.

Não modifique a câmera global para acomodar o mapa.

Não modifique o formato dos saves reais.

---

# 4. AUDITORIA COMPLETA DA ARQUITETURA ATUAL

Leia:

- `CLAUDE.md`.
- `README.md`.
- `docs/ARCHITECTURE.md`.
- `docs/GAME_DESIGN.md`.
- `docs/COMBAT_SYSTEM.md`.
- `docs/CHARACTER_ATTRIBUTES.md`.
- `docs/PROGRESSION_SYSTEM.md`.
- `docs/SKILLS_SYSTEM.md`.
- `docs/MAGIC_SYSTEM.md`.
- `docs/INVENTORY_EQUIPMENT.md`.
- `docs/ENEMIES.md`.
- `docs/ASSET_CONTRACT.md`.
- `docs/ART_DIRECTION.md`.
- `docs/PLAYER_VISUAL_ANIMATION_V1_3.md`.
- `docs/PLAYER_ANIMATION_EXPANSION_V1_4.md`.
- `docs/PLAYER_ART_POLISH_V1_5.md`.
- `docs/DECISIONS.md`.

Localize os documentos reais.

Inspecione os módulos existentes em `src/`, os exemplos e os testes.

Crie uma matriz:

| Sistema | Implementado | Parcial | Ausente | Reutilizável | Mudanças necessárias |
|---|---|---|---|---|---|
| Mundo | | | | | |
| Salas | | | | | |
| Transições | | | | | |
| Mapa | | | | | |
| Exploração | | | | | |
| Segredos | | | | | |
| Inventário | | | | | |
| Equipamentos | | | | | |
| Armas | | | | | |
| Magias | | | | | |
| Transformações | | | | | |
| NPCs | | | | | |
| Comércio | | | | | |
| Missões | | | | | |
| Save/load | | | | | |

Não descreva funcionalidades planejadas como se já estivessem implementadas.

---

# 5. VISÃO DO MUNDO

O Dark Castle RPG deverá possuir um grande mundo interconectado.

A estrutura principal será um castelo colossal, cercado por regiões externas.

O mundo poderá incluir:

- Cidade inicial.
- Floresta amaldiçoada.
- Portões do castelo.
- Salão central.
- Biblioteca arcana.
- Catedral profana.
- Torre do relógio.
- Jardins abandonados.
- Laboratório alquímico.
- Quartéis.
- Arsenal.
- Prisões.
- Esgotos.
- Catacumbas.
- Criptas.
- Cavernas.
- Minas.
- Torre astral.
- Observatório.
- Fortaleza subterrânea.
- Salões nobres.
- Torre dos magos.
- Santuários.
- Dimensões alternativas.
- Regiões secretas.

Esses nomes são propostas.

Você pode sugerir nomes originais mais adequados à identidade do jogo.

## 5.1 Escala futura

Planeje uma arquitetura capaz de suportar aproximadamente:

- 15 a 25 regiões principais.
- 600 a 1.000 salas no objetivo de longo prazo.
- Diversas conexões verticais.
- Conexões horizontais.
- Caminhos alternativos.
- Atalhos.
- Salas opcionais.
- Áreas secretas.
- Salas de chefes.
- Salas de save.
- Salas de teletransporte.
- Salas de comércio.
- Salas de NPCs.

Esses números são metas de capacidade e produção futura, não exigências para esta fase.

Não gere 1.000 salas vazias.

Não confunda tamanho com qualidade.

---

# 6. MODELO DE MUNDO BASEADO EM GRAFO

O mundo deve ser representável como um grafo de salas conectadas.

Cada sala deverá possuir:

- Identificador estável.
- Região.
- Dimensões.
- Coordenadas no mapa.
- Geometria.
- Portas.
- Conexões.
- Pontos de entrada.
- Pontos de saída.
- Inimigos.
- Objetos.
- Segredos.
- Eventos.
- Requisitos de acesso.
- Estado persistente.

Modelo conceitual:

```text
World
 ├── Regions
 │    ├── Rooms
 │    │    ├── Geometry
 │    │    ├── Connections
 │    │    ├── Enemies
 │    │    ├── Interactables
 │    │    ├── Secrets
 │    │    └── PersistentState
 │    └── RegionMetadata
 └── GlobalWorldState
```

A arquitetura deve permitir ciclos no grafo.

Não crie um mundo que seja apenas uma sequência linear de fases.

---

# 7. SISTEMA DE SALAS

Cada sala deverá ter um identificador único.

Exemplo conceitual:

```ron
(
    id: "castle_entrance_001",
    region: "castle_entrance",
    map_position: (0, 0),
    width: 640,
    height: 360,
    connections: [
        (
            target: "castle_entrance_002",
            direction: "right",
        ),
    ],
)
```

Esse exemplo não define o esquema final.

Projete o formato conforme os tipos reais de Rust e RON.

## 7.1 Requisitos

- Carregamento de sala.
- Descarregamento seguro.
- Transições.
- Persistência de objetos.
- Identificadores estáveis.
- Validação de conexões.
- Portas.
- Atalhos.
- Geometria de colisão.
- Spawn de inimigos.
- Posição de entrada.
- Retorno à sala anterior.

Evite carregar o castelo inteiro em memória.

Planeje carregamento por região ou por sala.

---

# 8. MAPA ESTILO METROIDVANIA

O mapa deverá ser um sistema central do jogo.

Ele precisa mostrar:

- Salas visitadas.
- Salas descobertas.
- Salas ainda não exploradas.
- Posição atual.
- Regiões.
- Salas de save descobertas.
- Teletransportes descobertos.
- Conexões conhecidas.
- Marcadores.
- Percentual de exploração.

O mapa não deve revelar automaticamente todos os segredos.

## 8.1 Estados de descoberta

Planeje pelo menos:

- Unknown.
- Discovered.
- Visited.
- Completed, se houver definição clara.

Não confunda sala visitada com sala totalmente explorada.

## 8.2 Mapa labiríntico

O sistema deverá suportar:

- Corredores longos.
- Salas verticais.
- Poços.
- Torres.
- Plataformas.
- Elevadores.
- Salas sobrepostas visualmente.
- Conexões ocultas.
- Passagens falsas.
- Caminhos que retornam a áreas anteriores.

Não copie o mapa de Symphony of the Night.

Use as imagens de referência fornecidas pelo usuário apenas para compreender a densidade, a interconectividade e a complexidade desejadas.

## 8.3 Porcentagem de exploração

Defina uma fórmula documentada.

Ela deve diferenciar:

- Salas conhecidas.
- Salas visitadas.
- Salas secretas descobertas.

Evite porcentagens inconsistentes quando novas regiões forem adicionadas.

---

# 9. PROGRESSÃO METROIDVANIA

A exploração deve depender de habilidades adquiridas.

Exemplos:

- Salto duplo.
- Deslizar.
- Escalar.
- Quebrar paredes especiais.
- Atravessar passagens estreitas.
- Resistir a ambientes perigosos.
- Voar.
- Transformar-se.
- Ativar mecanismos mágicos.
- Abrir portas com relíquias.

Não implemente todas essas habilidades agora.

Crie uma arquitetura de requisitos de acesso.

Exemplo conceitual:

```text
Passagem A:
  requer DoubleJump

Passagem B:
  requer MistForm

Passagem C:
  requer AncientKey

Passagem D:
  requer FireResistance
```

A arquitetura deve permitir requisitos combinados, por exemplo:

```text
MistForm AND AncientSealBroken
```

E requisitos alternativos:

```text
DoubleJump OR BatForm
```

Planeje um sistema tipado e validável, sem expressões arbitrárias executadas em runtime.

---

# 10. SISTEMA DE TRANSFORMAÇÕES

Este é um dos pilares do jogo.

O protagonista poderá adquirir formas alternativas.

Propostas iniciais:

## Forma de Morcego

- Voo.
- Acesso a passagens elevadas.
- Mobilidade aérea.
- Consumo de recurso, se apropriado.

## Forma de Névoa

- Passagem por grades.
- Acesso a pequenas aberturas.
- Mobilidade especial.
- Limitações de combate.

## Forma de Fera

- Movimento terrestre diferenciado.
- Acesso a passagens específicas.
- Ataques próprios.
- Interação com obstáculos.

## Formas originais

Proponha outras transformações coerentes com o universo do Dark Castle RPG.

Não copie diretamente as formas, aparências e progressões de Castlevania.

## 10.1 Arquitetura

Cada transformação deverá possuir:

- Identificador.
- Requisitos de desbloqueio.
- Aparência.
- Movimento permitido.
- Ações permitidas.
- Custos.
- Duração, quando aplicável.
- Restrições.
- Interações com o mundo.
- Persistência.

Planeje a integração com:

- Input.
- Movimento.
- Física.
- Combate.
- Magia.
- Equipamentos.
- Animações.
- Save/load.

Não implemente uma transformação completa nesta fase se isso exigir alterar o gameplay existente.

Para o protótipo vertical, utilize uma capacidade de exploração simulada ou uma autorização de passagem testável, sem introduzir física de voo ou névoa.

---

# 11. SISTEMA DE ARMAS

O jogo deverá suportar múltiplas categorias.

Planeje:

- Espadas curtas.
- Espadas longas.
- Sabres.
- Espadas de duas mãos.
- Adagas.
- Lanças.
- Machados.
- Martelos.
- Chicotes.
- Cajados.
- Armas mágicas.
- Armas especiais.

Cada categoria poderá possuir:

- Alcance.
- Velocidade.
- Dano.
- Escalamento.
- Elemento.
- Animações.
- Técnicas.
- Requisitos.
- Raridade.
- Efeitos especiais.

Não implemente todas as armas agora.

O objetivo é garantir que o sistema atual possa evoluir sem precisar ser refeito.

---

# 12. EQUIPAMENTOS POR PARTE DO CORPO

Planeje slots como:

- Cabeça.
- Peitoral.
- Braços.
- Luvas.
- Pernas.
- Botas.
- Capa.
- Arma principal.
- Arma secundária ou escudo.
- Acessórios.
- Anéis.
- Amuletos.

Verifique quais slots já existem.

Não duplique sistemas existentes.

## 12.1 Aparência dinâmica

Um requisito importante:

**Equipamentos deverão poder alterar a aparência do protagonista.**

Avalie três arquiteturas:

### A — Sprites separados por camada

Corpo, armadura, capa e arma renderizados separadamente.

### B — Sprites pré-renderizados

Combinações completas de equipamento.

### C — Sistema híbrido

Corpo e roupas em camadas, com armas e acessórios separados, utilizando composições pré-renderizadas onde necessário.

Compare:

- Memória.
- Número de assets.
- Complexidade.
- Animações.
- Pivôs.
- Direção.
- Performance.
- Compatibilidade com Blender.
- Escalabilidade para centenas de itens.

Recomende uma arquitetura.

Não implemente a troca visual completa nesta fase.

Não produza milhares de combinações.

---

# 13. SISTEMA DE MAGIAS

O jogo já possui uma base de magia.

Planeje a expansão para:

- Fogo.
- Gelo.
- Raio.
- Água.
- Vento.
- Terra.
- Luz.
- Trevas.
- Arcano.
- Magias de suporte.
- Cura.
- Buffs.
- Debuffs.
- Invocações.
- Magias de exploração.

Preserve o reconhecedor de comandos existente.

Planeje:

- Aprendizado.
- Evolução.
- Custos.
- Resistências.
- Fraquezas.
- Escalamento.
- Efeitos visuais.
- Equipamentos que modificam magias.

Não implemente novas magias nesta fase.

---

# 14. CIDADE E NPCs

O mundo deverá possuir pelo menos uma cidade ou assentamento importante.

Ela funcionará como centro de serviços.

Planeje NPCs como:

- Comerciante.
- Ferreiro.
- Alquimista.
- Bibliotecário.
- Explorador.
- Curandeiro.
- Personagens de missões.
- Personagens narrativos.

Cada NPC poderá possuir:

- Identificador.
- Localização.
- Diálogos.
- Condições.
- Serviços.
- Estado persistente.
- Missões associadas.

Não implemente inteligência artificial complexa de NPCs nesta fase.

---

# 15. COMÉRCIO E ECONOMIA

Planeje um sistema de compra e venda.

Deverá suportar:

- Moeda.
- Preço de compra.
- Preço de venda.
- Estoque.
- Itens desbloqueáveis.
- Requisitos.
- Recompensas.
- Serviços.

Avalie:

- Economia fixa.
- Estoque limitado.
- Reposição.
- Itens raros.
- Preços condicionais.
- Melhorias de equipamento.

Não implemente a economia completa nesta fase.

---

# 16. MISSÕES E FACÇÕES

Planeje um sistema de missões com:

- Identificadores.
- Objetivos.
- Condições.
- Recompensas.
- Estados.
- Dependências.
- Persistência.

Estados possíveis:

- Locked.
- Available.
- Active.
- Completed.
- Failed.

Planeje facções e reputação como funcionalidades futuras opcionais.

Não implemente árvores narrativas complexas agora.

---

# 17. SEGREDOS E DESCOBERTAS

O castelo deve recompensar exploração cuidadosa.

Planeje:

- Paredes falsas.
- Pisos quebráveis.
- Passagens invisíveis.
- Salas escondidas.
- Interruptores.
- Alavancas.
- Portas especiais.
- Relíquias.
- Baús raros.
- Atalhos.
- Eventos secretos.
- Chefes opcionais.

Cada segredo deverá possuir estado persistente quando necessário.

Não use aleatoriedade irreproduzível para decidir se um segredo existe.

O mundo principal deverá ter estrutura autoral e estável.

---

# 18. SAVE/LOAD DO MUNDO

Planeje a persistência de:

- Sala atual.
- Região atual.
- Posição.
- Salas visitadas.
- Mapa descoberto.
- Segredos encontrados.
- Portas abertas.
- Chefes derrotados.
- Relíquias.
- Transformações desbloqueadas.
- Equipamentos.
- Inventário.
- NPCs.
- Missões.
- Estado do mundo.

Não altere o save real nesta fase.

Não altere o formato atual sem uma migração planejada.

Crie um contrato de persistência futura.

Para o protótipo, use armazenamento isolado e dados de teste.

Não grave em `saves/`.

---

# 19. FERRAMENTAS PARA CRIAÇÃO DO CASTELO

Um mundo com centenas de salas não deve depender de editar código Rust para cada sala.

Planeje ferramentas para:

- Criar salas.
- Definir conexões.
- Posicionar portas.
- Inserir inimigos.
- Criar segredos.
- Definir requisitos.
- Validar o mapa.
- Detectar salas desconectadas.
- Detectar caminhos impossíveis.
- Gerar relatórios.
- Visualizar o grafo.

Avalie o uso de arquivos RON e ferramentas próprias.

Não introduza uma engine diferente.

Não instale editores externos sem autorização.

---

# 20. PROTÓTIPO VERTICAL V1.6

Após concluir a arquitetura e verificar que a integração é segura, implemente um protótipo pequeno.

Sugestão:

**Região: Portões do Castelo.**

Contendo entre 6 e 10 salas.

O protótipo deverá incluir:

- Entrada.
- Corredor.
- Sala vertical.
- Sala com inimigo.
- Sala com passagem secreta.
- Sala com barreira de progressão.
- Sala de recompensa.
- Conexão de retorno.

As salas podem usar geometria provisória.

Não produza arte final.

Não substitua o mundo atual sem necessidade.

Prefira um exemplo isolado, como:

`examples/world_vertical_slice.rs`

Adapte ao projeto.

## 20.1 Requisitos do protótipo

- Navegação entre salas.
- Identificadores estáveis.
- Transições corretas.
- Mapa atualizado.
- Sala visitada registrada.
- Segredo descoberto.
- Barreira com requisito.
- Requisito desbloqueado em ambiente de teste.
- Retorno à sala anterior.
- Sem perda de estado.
- Sem alteração do gameplay existente.

O desbloqueio de uma passagem poderá ser simulado por um recurso de teste, sem implementar ainda uma transformação real.

---

# 21. VALIDAÇÃO DO GRAFO

Crie testes para:

1. IDs duplicados.
2. Conexões inexistentes.
3. Portas sem destino.
4. Salas desconectadas.
5. Ciclos válidos.
6. Requisitos impossíveis.
7. Caminhos alternativos.
8. Regiões inacessíveis.
9. Segredos opcionais.
10. Progressão principal alcançável.

O validador deverá distinguir:

- Conteúdo intencionalmente secreto.
- Conteúdo opcional.
- Conteúdo obrigatório.
- Conteúdo inacessível por erro.

Não trate toda sala inacessível inicialmente como defeito.

---

# 22. TESTES DE EXPLORAÇÃO

Implemente testes determinísticos para:

- Entrar em sala.
- Sair de sala.
- Voltar.
- Atualizar mapa.
- Descobrir segredo.
- Bloquear passagem.
- Desbloquear passagem.
- Manter estado.
- Evitar duplicação de entidades.
- Evitar spawn inválido.
- Evitar carregar salas incorretas.

Teste transições repetidas.

Teste entradas por portas diferentes.

Teste sala sem saída.

Teste retorno após passagem secreta.

---

# 23. INTEGRAÇÃO COM O JOGO EXISTENTE

O protótipo deve reutilizar:

- Jogador.
- Movimento.
- Física.
- Combate.
- Inimigos, quando aplicável.
- Sistema visual.
- Animações.
- VFX.

Não crie um segundo controlador de jogador.

Não duplique o sistema de combate.

Não crie um novo save global.

Se um exemplo isolado exigir adaptações, mantenha-as restritas ao exemplo e a módulos reutilizáveis de mundo.

---

# 24. DOCUMENTAÇÃO OBRIGATÓRIA

Crie os seguintes documentos, adaptando nomes se já existirem equivalentes:

1. `docs/GAME_VISION_MASTER.md`
2. `docs/WORLD_ARCHITECTURE.md`
3. `docs/CASTLE_WORLD_DESIGN.md`
4. `docs/ROOM_SYSTEM.md`
5. `docs/MAP_EXPLORATION_SYSTEM.md`
6. `docs/TRANSFORMATION_SYSTEM.md`
7. `docs/WEAPON_EXPANSION.md`
8. `docs/EQUIPMENT_VISUAL_SYSTEM.md`
9. `docs/NPC_COMMERCE_SYSTEM.md`
10. `docs/QUEST_FACTION_SYSTEM.md`
11. `docs/WORLD_PERSISTENCE.md`
12. `docs/WORLD_CONTENT_PIPELINE.md`
13. `docs/V1_6_VERTICAL_SLICE.md`

Evite duplicar conteúdo já documentado.

Use referências entre documentos.

Distinga claramente:

- Implementado.
- Protótipo.
- Planejado.
- Proposta.
- Decisão pendente.

Não registre propostas como decisões definitivas.

---

# 25. ROADMAP DE LONGO PRAZO

Produza um roadmap para evolução do jogo.

Sugestão de organização:

**V1.6 — Fundação do mundo**

Arquitetura, grafo, mapa e protótipo.

**V1.7 — Salas e transições**

Carregamento, portas, regiões e persistência.

**V1.8 — Exploração**

Segredos, relíquias, barreiras e atalhos.

**V1.9 — Transformações**

Primeira transformação completa.

**V2.0 — Equipamentos**

Armas, armaduras e aparência dinâmica.

**V2.1 — NPCs e comércio**

Cidade, diálogos, compra e venda.

**V2.2 — Conteúdo do castelo**

Primeiras regiões completas.

**V2.3 — Chefes e progressão**

Chefes, recompensas e desbloqueios.

**V2.4 — Missões e narrativa**

História, personagens e objetivos.

**V2.5 — Expansão do mundo**

Mais regiões e exploração avançada.

Essas versões são propostas, não compromissos definitivos.

Ajuste o roadmap conforme as dependências técnicas identificadas.

---

# 26. TESTES OBRIGATÓRIOS

Antes e depois das alterações, execute as verificações pertinentes.

Inclua:

- `cargo fmt --all -- --check`
- `cargo check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- Testes do grafo.
- Testes das salas.
- Testes do mapa.
- Testes de exploração.
- Autoplay.
- Visual Check.

Respeite o `CLAUDE.md`.

Não execute compilações simultâneas.

Não apague caches.

Não altere os saves reais.

Não enfraqueça testes existentes.

---

# 27. CRITÉRIOS DE ACEITAÇÃO

A V1.6 só será considerada tecnicamente concluída quando:

- [ ] Estado inicial auditado.
- [ ] Arquivos protegidos preservados.
- [ ] Visão mestre documentada.
- [ ] Arquitetura do mundo documentada.
- [ ] Sistema de salas especificado.
- [ ] Grafo de conexões implementado ou prototipado.
- [ ] Mapa explorável prototipado.
- [ ] Requisitos de acesso modelados.
- [ ] Sistema de transformações especificado.
- [ ] Expansão de armas especificada.
- [ ] Equipamentos visuais especificados.
- [ ] NPCs e comércio especificados.
- [ ] Missões e facções especificadas.
- [ ] Persistência futura especificada.
- [ ] Ferramentas de conteúdo planejadas.
- [ ] Protótipo de 6 a 10 salas funcional.
- [ ] Passagem secreta funcional.
- [ ] Barreira de progressão funcional.
- [ ] Retorno entre salas funcional.
- [ ] Testes do grafo passando.
- [ ] Testes de exploração passando.
- [ ] Testes existentes passando.
- [ ] Gameplay preservado.
- [ ] Animações e VFX preservados.
- [ ] Câmera preservada.
- [ ] Saves preservados.
- [ ] Git preservado.
- [ ] Nenhum commit ou push realizado.

Se algum item não puder ser concluído com segurança, informe o impedimento.

Não declare sucesso total com critérios pendentes.

---

# 28. RELATÓRIO FINAL

Entregue o relatório nesta ordem:

**A. Estado inicial e Git**

**B. Auditoria da arquitetura existente**

**C. Visão geral do jogo**

**D. Estrutura do castelo**

**E. Sistema de salas**

**F. Grafo de conexões**

**G. Sistema de mapa**

**H. Progressão Metroidvania**

**I. Transformações**

**J. Armas**

**K. Equipamentos e aparência**

**L. Magias**

**M. NPCs e comércio**

**N. Missões e facções**

**O. Segredos e exploração**

**P. Persistência**

**Q. Ferramentas de criação**

**R. Protótipo vertical**

**S. Testes automatizados**

**T. Validação visual**

**U. Performance**

**V. Arquivos criados e modificados**

**W. Estado final do Git**

**X. Pendências e decisões necessárias**

**Y. Roadmap recomendado**

Inclua os caminhos dos documentos, exemplos e capturas produzidos.

Diferencie resultados realmente executados de propostas futuras.

---

# 29. CONDIÇÕES DE PARADA

Interrompa a operação afetada se:

- A implementação exigir reescrever gameplay.
- For necessário alterar saves reais.
- Uma mudança exigir modificar a câmera global.
- Houver risco de perda de alterações não commitadas.
- O protótipo exigir instalar dependências.
- Uma migração estrutural não prevista for necessária.
- O modelo de mundo entrar em conflito com a física existente.
- Um teste revelar regressão importante.
- O escopo exceder significativamente a proposta de protótipo.

Nesses casos, documente o impedimento e solicite autorização antes de prosseguir com a operação afetada.

Continue apenas atividades independentes e seguras.

---

# 30. INSTRUÇÃO FINAL

Você está autorizado a executar a V1.6 dentro deste escopo.

A ordem de execução deve ser:

1. Auditoria.
2. Visão mestre.
3. Arquitetura do mundo.
4. Planejamento dos sistemas futuros.
5. Grafo de salas.
6. Sistema de mapa.
7. Requisitos de exploração.
8. Protótipo vertical.
9. Testes.
10. Documentação.
11. Relatório final.

Não construa o castelo inteiro.

Não implemente todas as transformações.

Não implemente centenas de armas.

Não crie a cidade completa.

Não implemente toda a economia.

Não faça commits.

Não faça push.

Não inicie a V1.7.

**O objetivo desta fase é transformar Dark Castle RPG de um protótipo de combate em uma base sólida para um Metroidvania RPG de grande escala, mantendo toda a estabilidade conquistada nas versões anteriores.**

Ao concluir, apresente o relatório completo e aguarde minha avaliação.