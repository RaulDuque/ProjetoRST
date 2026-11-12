# PROJETO: RPG 2D DE AÇÃO EM RUST — METROIDVANIA

Você será o arquiteto de software, engenheiro de jogos, programador Rust e responsável técnico por estruturar e desenvolver este projeto.

Quero desenvolver um RPG de ação 2D inspirado nas mecânicas de Castlevania: Symphony of the Night, mas com universo, personagens, história, habilidades, equipamentos, inimigos, mapas e identidade visual originais.

O projeto será desenvolvido em Rust utilizando Bevy Engine.

IMPORTANTE: Não quero apenas um planejamento ou uma demonstração. Quero que você trabalhe diretamente na pasta atual, crie a estrutura do projeto, instale as dependências, configure as ferramentas, crie a documentação, prepare as regras do Claude Code e implemente a primeira versão executável.

A pasta atual contém somente o diretório inicial do projeto, chamado "projeto rst". Considere que estamos começando do zero.

==================================================
FASE 0 — REGRAS FUNDAMENTAIS DE EXECUÇÃO
==================================================

Antes de qualquer implementação:

1. Analise o sistema operacional e as ferramentas disponíveis.
2. Verifique se Rust está instalado.
3. Verifique a instalação do Cargo.
4. Verifique se Git está instalado.
5. Verifique se Claude Code está funcionando.
6. Verifique as ferramentas de compilação necessárias para Bevy.
7. Verifique o conteúdo atual da pasta.
8. Não sobrescreva arquivos existentes sem verificar seu conteúdo.
9. Não instale dependências desnecessárias.
10. Não utilize APIs obsoletas.
11. Consulte a documentação oficial das versões escolhidas.

Trabalhe com versões estáveis e compatíveis.

Não escolha versões aleatórias de bibliotecas.

Antes de adicionar qualquer dependência, verifique sua compatibilidade com a versão escolhida do Bevy.

Utilize versões fixadas no Cargo.toml e mantenha Cargo.lock versionado.

Se encontrar algum problema de compatibilidade, resolva antes de continuar.

Não pare depois de apresentar o planejamento. Execute as etapas autorizadas.

Se algum comando exigir autorização, solicite a autorização e continue depois.

==================================================
FASE 1 — DEFINIÇÃO DA TECNOLOGIA
==================================================

Tecnologias principais:

- Rust: linguagem principal.
- Cargo: gerenciamento do projeto e dependências.
- Bevy: engine de jogos 2D.
- Git: controle de versão.
- Serde: serialização e desserialização.
- RON: armazenamento de configurações e dados.
- Thiserror: tratamento de erros.
- Tracing: logs e diagnóstico.

Avalie a necessidade de outras bibliotecas, adicionando somente quando houver justificativa.

Possíveis ferramentas complementares:

- Bevy Inspector: inspeção de entidades e componentes durante o desenvolvimento.
- Ferramentas de gerenciamento de mapas 2D: somente quando a arquitetura precisar.
- Ferramentas de testes: utilizar o sistema nativo de testes do Rust sempre que possível.

A engine deve cuidar de:

- Renderização 2D.
- Sprites.
- Animações.
- Áudio.
- Controles.
- Colisões.
- Câmera.
- Interface.
- Gerenciamento de cenas.
- Atualização dos sistemas do jogo.

Utilize o ECS do Bevy corretamente.

Evite concentrar toda a lógica em main.rs.

Não misture lógica de negócio, renderização e interface em um único sistema.

==================================================
FASE 2 — INSTALAÇÃO E CONFIGURAÇÃO
==================================================

Configure o ambiente de desenvolvimento.

Verifique se é necessário instalar:

- Rustup.
- Rust stable.
- Cargo.
- Rustfmt.
- Clippy.
- Git.
- Ferramentas de compilação do sistema operacional.

No Windows, verifique a disponibilidade do Visual Studio Build Tools e dos componentes necessários para compilação MSVC.

Execute os comandos adequados ao ambiente detectado.

Configure o projeto utilizando a edição estável atual do Rust.

Crie o projeto Cargo na pasta atual, sem criar desnecessariamente outro diretório com o mesmo nome.

O nome técnico do pacote deverá ser:

dark_castle_rpg

Adicione Bevy e as dependências escolhidas através do Cargo.

Crie uma configuração de desenvolvimento e outra de distribuição.

Configure otimizações apropriadas para dependências de Bevy durante o desenvolvimento, sem comprometer a velocidade de compilação desnecessariamente.

Configure:

- cargo fmt.
- cargo clippy.
- cargo test.
- cargo check.
- cargo run.

Todos esses comandos deverão funcionar a partir da raiz do projeto.

==================================================
FASE 3 — ARQUITETURA DO PROJETO
==================================================

Crie uma arquitetura modular.

Estrutura inicial desejada:

dark_castle_rpg/
│
├── Cargo.toml
├── Cargo.lock
├── README.md
├── CLAUDE.md
├── CHANGELOG.md
├── .gitignore
│
├── .claude/
│   ├── rules/
│   │   ├── architecture.md
│   │   ├── rust-conventions.md
│   │   ├── testing.md
│   │   ├── commits.md
│   │   └── game-design.md
│   │
│   ├── skills/
│   │   ├── game-feature/
│   │   │   └── SKILL.md
│   │   ├── combat-system/
│   │   │   └── SKILL.md
│   │   ├── rust-code-review/
│   │   │   └── SKILL.md
│   │   ├── test-and-validate/
│   │   │   └── SKILL.md
│   │   └── atomic-commit/
│   │       └── SKILL.md
│   │
│   ├── agents/
│   │   ├── game-architect.md
│   │   ├── combat-specialist.md
│   │   └── code-reviewer.md
│   │
│   └── settings.json
│
├── docs/
│   ├── GAME_DESIGN.md
│   ├── ARCHITECTURE.md
│   ├── COMBAT_SYSTEM.md
│   ├── CHARACTER_ATTRIBUTES.md
│   ├── SKILL_SYSTEM.md
│   ├── MAGIC_SYSTEM.md
│   ├── EQUIPMENT_SYSTEM.md
│   ├── ENEMY_SYSTEM.md
│   ├── PROGRESSION_SYSTEM.md
│   ├── INPUT_SYSTEM.md
│   ├── SAVE_SYSTEM.md
│   ├── ROADMAP.md
│   └── DECISIONS.md
│
├── assets/
│   ├── sprites/
│   │   ├── player/
│   │   ├── enemies/
│   │   ├── bosses/
│   │   ├── effects/
│   │   └── environment/
│   │
│   ├── animations/
│   ├── audio/
│   │   ├── music/
│   │   └── sfx/
│   │
│   ├── fonts/
│   ├── maps/
│   ├── ui/
│   └── data/
│       ├── skills/
│       ├── spells/
│       ├── items/
│       ├── enemies/
│       └── equipment/
│
├── src/
│   ├── main.rs
│   ├── lib.rs
│   │
│   ├── app/
│   │   ├── mod.rs
│   │   ├── plugins.rs
│   │   └── states.rs
│   │
│   ├── core/
│   │   ├── mod.rs
│   │   ├── constants.rs
│   │   ├── events.rs
│   │   ├── time.rs
│   │   └── resources.rs
│   │
│   ├── player/
│   │   ├── mod.rs
│   │   ├── components.rs
│   │   ├── systems.rs
│   │   ├── movement.rs
│   │   ├── animation.rs
│   │   ├── attributes.rs
│   │   └── progression.rs
│   │
│   ├── combat/
│   │   ├── mod.rs
│   │   ├── attacks.rs
│   │   ├── damage.rs
│   │   ├── hitboxes.rs
│   │   ├── combos.rs
│   │   ├── command_parser.rs
│   │   ├── command_buffer.rs
│   │   └── combat_state.rs
│   │
│   ├── skills/
│   │   ├── mod.rs
│   │   ├── components.rs
│   │   ├── definitions.rs
│   │   ├── execution.rs
│   │   ├── unlock.rs
│   │   └── cooldowns.rs
│   │
│   ├── magic/
│   │   ├── mod.rs
│   │   ├── spells.rs
│   │   ├── casting.rs
│   │   ├── projectiles.rs
│   │   └── elements.rs
│   │
│   ├── inventory/
│   │   ├── mod.rs
│   │   ├── items.rs
│   │   ├── equipment.rs
│   │   ├── inventory.rs
│   │   └── equipment_slots.rs
│   │
│   ├── enemies/
│   │   ├── mod.rs
│   │   ├── components.rs
│   │   ├── ai.rs
│   │   ├── behavior.rs
│   │   └── spawning.rs
│   │
│   ├── world/
│   │   ├── mod.rs
│   │   ├── rooms.rs
│   │   ├── maps.rs
│   │   ├── transitions.rs
│   │   └── collision.rs
│   │
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── hud.rs
│   │   ├── menus.rs
│   │   ├── status_screen.rs
│   │   ├── inventory_screen.rs
│   │   └── skill_screen.rs
│   │
│   └── save/
│       ├── mod.rs
│       ├── serialization.rs
│       └── save_manager.rs
│
└── tests/
    ├── combat_tests.rs
    ├── attributes_tests.rs
    ├── progression_tests.rs
    └── command_parser_tests.rs

Essa estrutura é uma referência inicial. Você pode ajustá-la se houver justificativa técnica, mas documente as decisões.

Não crie arquivos vazios sem finalidade.

Não implemente sistemas gigantes em arquivos únicos.

==================================================
FASE 4 — REGRAS DO CLAUDE CODE
==================================================

Crie e configure o CLAUDE.md.

Ele deverá ser o documento principal de regras do projeto.

Inclua as seguintes regras obrigatórias:

1. Antes de implementar qualquer funcionalidade, analise a arquitetura existente.
2. Nunca reescreva módulos inteiros quando uma alteração pequena for suficiente.
3. Nunca remova funcionalidades existentes sem autorização.
4. Não crie duplicações de sistemas.
5. Não misture responsabilidades.
6. Não coloque toda a lógica no main.rs.
7. Utilize nomes de funções, variáveis, structs e enums em inglês.
8. A documentação explicativa poderá ser em português.
9. Utilize tipagem forte.
10. Evite unwrap() e expect() em caminhos normais de execução.
11. Evite valores mágicos espalhados pelo código.
12. Utilize constantes e configurações centralizadas.
13. Prefira componentes pequenos e sistemas com responsabilidades específicas.
14. Não introduza dependências sem necessidade.
15. Toda funcionalidade importante precisa de testes.
16. Execute cargo fmt, cargo clippy, cargo test e cargo check antes de considerar uma tarefa concluída.
17. Não declare uma funcionalidade concluída se ela não foi implementada e validada.
18. Não utilize código fictício ou funções vazias para simular funcionalidades.
19. Não esconda erros de compilação.
20. Atualize a documentação quando modificar a arquitetura.

REGRAS DE COMMITS ATÔMICOS:

Esta é uma regra extremamente importante.

Cada commit deverá representar uma única alteração lógica e independente.

Exemplos:

feat(player): implement basic movement
feat(combat): add melee attack system
feat(skills): implement command recognition
feat(magic): add fire projectile
fix(combat): correct damage calculation
test(player): add movement tests
docs(architecture): document plugin structure
refactor(inventory): separate equipment logic

Nunca misture:

- Criação de funcionalidades diferentes.
- Refatoração não relacionada.
- Alterações de documentação sem relação com a tarefa.
- Correções não relacionadas ao objetivo do commit.

Antes de realizar um commit:

1. Verifique git status.
2. Examine git diff.
3. Verifique se os arquivos alterados pertencem à mesma tarefa.
4. Execute os testes pertinentes.
5. Execute formatação e verificações.
6. Confira se não existem arquivos temporários ou assets desnecessários.
7. Faça somente o commit correspondente à tarefa.

Não faça commits com código quebrado.

Não faça commits gigantes.

Não faça commit automático a cada alteração de arquivo.

Faça commits após concluir uma unidade funcional coerente.

Nunca utilize git reset --hard, git clean -fd ou operações destrutivas sem autorização explícita.

Se houver alterações anteriores do usuário, preserve-as.

Crie também uma skill /atomic-commit com esse procedimento.

==================================================
FASE 5 — SKILLS E COMANDOS PERSONALIZADOS
==================================================

Utilize o sistema atual de Skills do Claude Code.

Crie as skills dentro de .claude/skills/.

As skills deverão ser realmente funcionais, com instruções completas, não apenas arquivos contendo títulos.

Crie:

/game-feature

Procedimento:
- Analisar o requisito.
- Verificar arquitetura.
- Identificar módulos afetados.
- Planejar implementação.
- Implementar a funcionalidade.
- Criar testes.
- Executar validações.
- Atualizar documentação.
- Apresentar resumo.

 /combat-system

Procedimento:
- Analisar sistemas de combate existentes.
- Verificar colisões e hitboxes.
- Verificar estados do personagem.
- Verificar comandos de habilidades.
- Verificar consumo de recursos.
- Implementar alterações isoladas.
- Testar combos e danos.

 /rust-code-review

Procedimento:
- Procurar código duplicado.
- Identificar problemas de ownership e borrowing.
- Identificar problemas de desempenho.
- Identificar problemas de arquitetura ECS.
- Identificar possíveis bugs.
- Verificar tratamento de erros.
- Apresentar problemas classificados por gravidade.

 /test-and-validate

Procedimento:
- Executar cargo fmt --check.
- Executar cargo clippy.
- Executar cargo test.
- Executar cargo check.
- Verificar se o projeto inicia.
- Relatar problemas encontrados.

 /atomic-commit

Procedimento:
- Analisar git status.
- Examinar git diff.
- Separar alterações por responsabilidade.
- Validar o código.
- Criar mensagem Conventional Commits.
- Criar commit atômico.
- Confirmar resultado.

Crie também agentes especializados quando forem úteis.

Os agentes deverão ter responsabilidades bem delimitadas.

Não permita que agentes trabalhem simultaneamente modificando os mesmos arquivos.

Crie comandos de trabalho reutilizáveis e documente como utilizá-los.

Prefira skills em vez do formato legado .claude/commands/.

==================================================
FASE 6 — CONCEITO E MECÂNICAS DO JOGO
==================================================

O jogo deverá possuir as seguintes características:

- Personagem único.
- Movimentação lateral.
- Exploração de ambientes interconectados.
- Combate em tempo real.
- Armas.
- Equipamentos.
- Armaduras.
- Acessórios.
- Técnicas especiais.
- Magias.
- Sistema de evolução.
- Inimigos com diferentes comportamentos.
- Chefes.
- Drops de itens.
- Baús.
- Segredos.
- Portas e áreas desbloqueáveis.
- Salvamento e carregamento.

A movimentação precisa contemplar:

- Andar.
- Correr.
- Pular.
- Queda.
- Agachar.
- Ataque.
- Defesa.
- Esquiva.
- Receber dano.
- Morte.
- Interação.

O personagem deverá possuir uma máquina de estados bem definida.

Evite permitir ações incompatíveis simultaneamente.

Por exemplo, não poderá executar uma animação de morte e continuar atacando.

==================================================
FASE 7 — SISTEMA COMPLETO DE ATRIBUTOS
==================================================

Implemente um sistema de atributos inspirado em Symphony of the Night.

ATRIBUTOS PRINCIPAIS:

STR — Strength:
Força física. Influencia ataques com armas e dano físico.

CON — Constitution:
Constituição. Influencia HP máximo e resistência.

INT — Intelligence:
Inteligência. Influencia MP, poder mágico e eficácia de magias.

LCK — Luck:
Sorte. Influencia chance crítica, drops e eventos aleatórios.

RECURSOS:

HP:
Pontos de vida atuais e máximos.

MP:
Pontos mágicos atuais e máximos.

HEARTS:
Recurso adicional consumido por determinadas habilidades.

EXP:
Experiência acumulada.

LEVEL:
Nível do personagem.

ATRIBUTOS DERIVADOS:

ATK:
Poder de ataque.

DEF:
Defesa física.

MATK:
Poder de ataque mágico.

MDEF:
Defesa mágica.

HIT:
Precisão.

CRITICAL_CHANCE:
Probabilidade de crítico.

CRITICAL_DAMAGE:
Multiplicador de dano crítico.

ATTACK_SPEED:
Velocidade de ataque.

MOVEMENT_SPEED:
Velocidade de movimento.

HP_REGEN:
Regeneração de vida.

MP_REGEN:
Regeneração de magia.

DODGE:
Probabilidade ou capacidade de esquiva.

RESISTANCES:
Resistências elementais.

STATUS_RESISTANCES:
Resistência a efeitos negativos.

Não utilize fórmulas arbitrárias sem documentá-las.

Crie fórmulas centralizadas e testáveis.

Os equipamentos deverão modificar os atributos.

Os atributos derivados deverão ser recalculados quando necessário.

Evite duplicar valores calculados em diferentes lugares.

Crie uma tela de status mostrando os atributos.

==================================================
FASE 8 — SISTEMA DE EVOLUÇÃO
==================================================

Implemente:

- Ganho de experiência.
- Progressão de níveis.
- Curva de experiência configurável.
- Evolução dos atributos.
- Pontos de atributos distribuíveis.
- Desbloqueio de habilidades.
- Requisitos de nível.
- Requisitos de atributos.
- Requisitos de itens.
- Requisitos de progressão.

A progressão deverá ser independente da lógica de renderização.

O jogador deverá conseguir evoluir seu personagem sem depender de alterações no código.

Os dados deverão permitir balanceamento posterior.

==================================================
FASE 9 — SISTEMA DE COMBATE COM COMANDOS
==================================================

ESTA É UMA DAS PRINCIPAIS MECÂNICAS DO JOGO.

Quero que técnicas e magias sejam ativadas por sequências de botões, como nos jogos de luta e em Symphony of the Night.

Não quero que todas as habilidades sejam executadas simplesmente pressionando uma tecla dedicada.

O personagem poderá aprender novas técnicas e magias, mas precisará conhecer e executar seus comandos.

Exemplo de comandos:

↓ → + ATAQUE

↓ ↘ → + ATAQUE

→ ← → + ATAQUE

↓ ↑ + ATAQUE

← → + MAGIA

Esses são exemplos de notação. O sistema deverá interpretar direções e botões de acordo com o personagem e sua orientação.

Crie um sistema completo de reconhecimento de comandos.

REQUISITOS:

1. Registrar entradas recentes do jogador.
2. Manter um buffer de comandos.
3. Registrar a direção.
4. Registrar o botão pressionado.
5. Registrar o instante de cada entrada.
6. Reconhecer sequências.
7. Permitir tolerância de tempo.
8. Permitir pequenas variações de execução.
9. Considerar a orientação do personagem.
10. Reconhecer diagonais.
11. Impedir execução duplicada do mesmo comando.
12. Permitir cancelamento de comandos.
13. Permitir prioridade entre comandos.
14. Permitir comandos diferentes com sequências semelhantes.
15. Permitir configurar janelas de execução.
16. Permitir comandos durante determinadas animações.
17. Respeitar cooldowns.
18. Respeitar consumo de MP e Hearts.
19. Respeitar requisitos de habilidades desbloqueadas.
20. Permitir configurar comandos sem modificar o código principal.

O sistema deverá ser independente da quantidade de habilidades.

Não crie um if para cada magia.

Utilize definições de comandos orientadas a dados.

Cada habilidade deverá possuir:

- ID.
- Nome.
- Descrição.
- Tipo.
- Sequência de comandos.
- Janela de execução.
- Custo de MP.
- Custo de Hearts.
- Cooldown.
- Dano.
- Multiplicador.
- Alcance.
- Tempo de execução.
- Animação.
- Efeitos visuais.
- Requisitos.
- Nível mínimo.
- Atributos necessários.
- Prioridade de reconhecimento.

Crie um parser capaz de interpretar sequências de comandos.

Exemplo conceitual de configuração:

{
  "id": "arcane_burst",
  "name": "Arcane Burst",
  "command": ["DOWN", "DOWN_FORWARD", "FORWARD", "ATTACK"],
  "mp_cost": 15,
  "cooldown": 2.0,
  "damage_multiplier": 1.8
}

O formato final deverá ser escolhido entre RON e outros formatos adequados, respeitando a arquitetura Rust.

IMPORTANTE:

O reconhecimento deverá diferenciar:

- Comando executado corretamente.
- Comando incompleto.
- Comando inválido.
- Comando fora do tempo.
- Comando sem recursos suficientes.
- Habilidade ainda não aprendida.
- Habilidade em cooldown.

Crie testes automatizados para os casos acima.

==================================================
FASE 10 — SISTEMA DE HABILIDADES
==================================================

O personagem deverá aprender técnicas de combate durante a aventura.

Categorias:

1. Técnicas com espada.
2. Técnicas com armas pesadas.
3. Técnicas defensivas.
4. Técnicas de movimentação.
5. Técnicas especiais.
6. Magias elementais.
7. Magias de suporte.
8. Magias de invocação.
9. Habilidades passivas.

As habilidades deverão possuir níveis ou melhorias quando isso fizer sentido.

O jogador poderá encontrar livros, pergaminhos, mestres, itens e outros meios de aprender técnicas.

Aprender uma técnica não significa necessariamente executá-la automaticamente.

O jogador precisará descobrir ou aprender sua sequência de comandos.

Crie uma estrutura de dados para permitir adicionar novas habilidades sem alterar o motor de combate.

==================================================
FASE 11 — SISTEMA DE MAGIAS
==================================================

Implemente um sistema de magias separado do sistema de ataques físicos, mas integrado ao reconhecedor de comandos.

Categorias elementais:

- Fogo.
- Gelo.
- Raio.
- Água.
- Vento.
- Terra.
- Luz.
- Trevas.
- Arcano.

Cada magia poderá possuir:

- Projéteis.
- Explosões.
- Áreas de efeito.
- Dano contínuo.
- Efeitos de controle.
- Cura.
- Proteção.
- Invocação.

As magias deverão respeitar:

- MP.
- Cooldown.
- Animações.
- Tempo de conjuração.
- Alcance.
- Colisão.
- Resistência elemental.
- Interrupção.
- Estado do personagem.

Não implemente efeitos mágicos diretamente no parser de comandos.

O parser reconhece o comando; o sistema de execução decide o que fazer.

==================================================
FASE 12 — EQUIPAMENTOS E INVENTÁRIO
==================================================

Crie um inventário completo.

Categorias:

- Espadas.
- Adagas.
- Machados.
- Lanças.
- Armas pesadas.
- Armaduras.
- Capacetes.
- Luvas.
- Botas.
- Escudos.
- Anéis.
- Amuletos.
- Consumíveis.
- Materiais.
- Itens de missão.

Slots de equipamento:

- Weapon.
- Offhand.
- Head.
- Body.
- Gloves.
- Boots.
- Accessory 1.
- Accessory 2.

Cada equipamento poderá possuir:

- Nome.
- Raridade.
- Descrição.
- Ataque.
- Defesa.
- Atributos adicionais.
- Resistências.
- Efeitos especiais.
- Requisitos de uso.
- Peso, se utilizado.
- Valor de venda.

Crie raridades configuráveis.

O inventário deverá permitir:

- Coletar itens.
- Equipar.
- Desequipar.
- Consumir.
- Descartar.
- Inspecionar.
- Organizar.

==================================================
FASE 13 — INIMIGOS E CHEFES
==================================================

Crie uma arquitetura genérica para inimigos.

Cada inimigo poderá ter:

- HP.
- MP, quando aplicável.
- Ataque.
- Defesa.
- Resistências.
- Fraquezas.
- Velocidade.
- Alcance.
- Inteligência artificial.
- Comportamento.
- Recompensas.
- Drops.
- Experiência concedida.

Crie comportamentos:

- Patrulhar.
- Perseguir.
- Atacar.
- Defender.
- Fugir.
- Esperar.
- Recuar.
- Executar habilidades.

Chefes deverão possuir máquinas de estados próprias, múltiplas fases e padrões de ataque.

Não misture IA de inimigos com lógica de controle do jogador.

==================================================
FASE 14 — MUNDO E EXPLORAÇÃO
==================================================

O mundo será formado por salas e áreas interconectadas.

Implemente:

- Salas.
- Transições.
- Portas.
- Corredores.
- Plataformas.
- Obstáculos.
- Áreas secretas.
- Pontos de salvamento.
- Baús.
- Objetos interativos.

A arquitetura deverá permitir mapas maiores sem precisar carregar tudo simultaneamente.

Planeje o carregamento de assets e salas.

A câmera deverá acompanhar o personagem de maneira suave.

Utilize colisões adequadas para plataformas 2D.

==================================================
FASE 15 — INTERFACE
==================================================

Crie uma interface com identidade visual gótica, medieval e fantástica.

HUD:

- Barra de HP.
- Barra de MP.
- Hearts.
- Level.
- EXP.
- Indicador de habilidade em cooldown, quando relevante.

TELA DE STATUS:

- STR.
- CON.
- INT.
- LCK.
- HP.
- MP.
- EXP.
- Level.
- ATK.
- DEF.
- Resistências.

TELA DE INVENTÁRIO:

- Equipamentos.
- Consumíveis.
- Materiais.
- Descrição dos itens.
- Comparação de atributos.

TELA DE HABILIDADES:

- Técnicas aprendidas.
- Magias aprendidas.
- Comandos necessários.
- Custos.
- Cooldowns.
- Requisitos.

Não faça a interface inteira em um único sistema.

==================================================
FASE 16 — SALVAMENTO
==================================================

Implemente salvamento e carregamento.

Salvar:

- Posição do personagem.
- Atributos.
- Nível.
- Experiência.
- Habilidades aprendidas.
- Magias aprendidas.
- Inventário.
- Equipamentos.
- Progresso do mundo.
- Portas desbloqueadas.
- Pontos de salvamento.

Utilize versionamento dos dados de salvamento.

Trate arquivos corrompidos e versões incompatíveis.

Não permita que uma atualização do jogo destrua automaticamente os dados antigos.

==================================================
FASE 17 — TESTES E QUALIDADE
==================================================

Todos os sistemas importantes deverão possuir testes.

Prioridades:

- Cálculo de dano.
- Cálculo de atributos.
- Progressão de níveis.
- Reconhecimento de comandos.
- Consumo de recursos.
- Cooldowns.
- Equipamentos.
- Resistências.
- Efeitos negativos.
- Salvamento.
- Carregamento.

Sempre que possível, mantenha as regras de negócio testáveis sem inicializar a engine inteira.

Não aceite código que apenas compila, mas não funciona.

==================================================
FASE 18 — PRIMEIRA ENTREGA EXECUTÁVEL
==================================================

Depois de criar a infraestrutura, implemente um primeiro protótipo funcional.

Não tente implementar o jogo completo de uma vez.

O protótipo inicial deverá possuir:

1. Uma janela executável.
2. Um cenário 2D temporário.
3. Um personagem controlável.
4. Movimentação lateral.
5. Pulo.
6. Gravidade.
7. Colisão com o chão.
8. Animações básicas ou estados visuais temporários.
9. Um ataque físico.
10. Um inimigo de teste.
11. Sistema de dano.
12. HP.
13. MP.
14. Um comando especial executável por sequência de botões.
15. Uma magia de teste.
16. Uma interface com HP e MP.
17. Uma tela de status.
18. Uma estrutura inicial de salvamento.

Utilize recursos gráficos temporários quando não existirem sprites.

Não deixe a ausência de arte impedir o desenvolvimento dos sistemas.

Crie assets de teste simples sem depender de imagens protegidas por direitos autorais.

O protótipo deverá ser executável com:

cargo run

==================================================
FASE 19 — DOCUMENTAÇÃO E ROADMAP
==================================================

Documente a arquitetura, os sistemas e as decisões.

Crie um roadmap dividido em etapas:

ETAPA 1 — Infraestrutura.
ETAPA 2 — Personagem e movimentação.
ETAPA 3 — Combate básico.
ETAPA 4 — Reconhecimento de comandos.
ETAPA 5 — Técnicas especiais.
ETAPA 6 — Magias.
ETAPA 7 — Atributos e progressão.
ETAPA 8 — Inventário e equipamentos.
ETAPA 9 — Inimigos.
ETAPA 10 — Chefes.
ETAPA 11 — Mundo e mapas.
ETAPA 12 — Interface completa.
ETAPA 13 — Salvamento.
ETAPA 14 — Balanceamento.
ETAPA 15 — Polimento e otimização.

Cada etapa deverá ter tarefas menores e verificáveis.

Crie um arquivo ROADMAP.md com checkboxes.

Marque uma tarefa como concluída somente depois de implementar e validar.

==================================================
FASE 20 — RELATÓRIO FINAL
==================================================

Ao finalizar a configuração inicial, apresente:

1. Estrutura criada.
2. Ferramentas instaladas.
3. Versões utilizadas.
4. Dependências adicionadas.
5. Arquivos de configuração criados.
6. Skills criadas.
7. Agentes criados.
8. Regras do CLAUDE.md.
9. Comandos disponíveis.
10. Resultado da compilação.
11. Resultado dos testes.
12. Como executar o jogo.
13. Próxima etapa recomendada.

Se alguma etapa não puder ser executada, explique exatamente o motivo.

Não marque como concluído algo que não foi executado.

==================================================
INSTRUÇÃO FINAL
==================================================

Quero que você execute esse trabalho diretamente no projeto.

Comece verificando o ambiente e a pasta atual.

Depois apresente um plano técnico curto e execute a configuração.

Priorize uma arquitetura sólida, modular, extensível e testável.

Não tente desenvolver todos os sistemas de jogo simultaneamente.

Primeiro construa a fundação e entregue um protótipo executável.

Depois disso, utilizaremos as skills e o roadmap para desenvolver cada sistema de forma incremental.

O objetivo é construir um RPG 2D completo, com profundidade de progressão, combate técnico por sequências de botões, magias, equipamentos e exploração, mantendo qualidade de código profissional.

**Não se limite a explicar como fazer. Faça as alterações, instale o necessário, compile, teste e documente.**