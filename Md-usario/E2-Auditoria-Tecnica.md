# ETAPA 2 — AUDITORIA TÉCNICA DA FUNDAÇÃO DO DARK CASTLE RPG

## 1. CONTEXTO DO PROJETO

Estamos desenvolvendo um RPG de ação 2D / Metroidvania em Rust, utilizando Bevy.

**Informações técnicas atuais:**
- Linguagem: Rust 2024.
- Engine: Bevy 0.20.1.
- Nome do projeto: `dark_castle_rpg`.
- Arquitetura baseada em módulos e plugins.
- Regras de domínio separadas dos sistemas ECS do Bevy.
- Conteúdo configurável por arquivos RON.
- Testes unitários, testes de integração e testes ECS headless.
- Git com commits atômicos e mensagens Conventional Commits.

O projeto possui uma documentação consolidada em `docs/RESUMO_PROJETO.md`, além de documentação específica dos sistemas.

O commit mais recente informado é:

`9dcdfcc5759860b834c3bc45343dfde2e776ec76`

Mensagem:

`docs: add consolidated project summary`

A branch utilizada é `master`.

O resumo consolidado deve ser tratado como ponto de partida, mas todas as informações importantes precisam ser conferidas diretamente nos arquivos e no código atual.

## 2. OBJETIVO DESTA ETAPA

Realizar uma auditoria técnica completa da fundação atual do jogo.

Antes de desenvolver novas funcionalidades, precisamos confirmar que:

1. A arquitetura está organizada e consistente.
2. Os módulos possuem responsabilidades bem definidas.
3. A separação entre regras puras e sistemas Bevy está sendo respeitada.
4. Os sistemas ECS estão corretamente organizados e executados na ordem esperada.
5. Os sistemas de combate, habilidades e magia estão integrados corretamente.
6. Os dados configuráveis estão sendo validados.
7. Os testes realmente verificam os comportamentos importantes.
8. As configurações do Claude Code estão adequadas para o desenvolvimento contínuo.
9. A documentação representa o comportamento real do código.
10. A fundação permite adicionar novas funcionalidades sem exigir refatorações constantes.

**Esta etapa é uma auditoria, não uma etapa de implementação de funcionalidades do jogo.**

## 3. REGRAS OBRIGATÓRIAS

Antes de começar:

- Leia integralmente `CLAUDE.md`.
- Leia `docs/RESUMO_PROJETO.md`.
- Consulte `docs/ARCHITECTURE.md`.
- Consulte `docs/ROADMAP.md`.
- Consulte `docs/DECISIONS.md`.
- Consulte a documentação dos sistemas existentes.
- Inspecione as regras, skills, agentes e configurações do Claude Code.
- Analise o código-fonte e os testes relacionados a cada sistema.

Não considere uma funcionalidade implementada apenas porque está marcada como concluída na documentação.

A confirmação deve ser feita por meio do código, dos testes e, quando necessário, da execução real.

### Restrições

- Não implemente novas funcionalidades do jogo.
- Não altere mecânicas de combate, movimentação ou progressão.
- Não faça refatorações preventivas sem autorização.
- Não altere arquivos de `Md-usario/`.
- Não remova dependências ou arquivos sem autorização.
- Não altere versões de dependências.
- Não crie novos agentes ou skills nesta etapa.
- Não faça commits sem autorização.
- Não execute comandos destrutivos, como `git reset --hard`, `git clean` ou remoções amplas.
- Preserve todas as alterações existentes do usuário.
- Não compile o projeto em diretórios temporários diferentes do ambiente de compilação já utilizado.
- Evite compilações completas duplicadas do Bevy.
- Não execute operações que possam consumir toda a memória ou causar instabilidade no computador.

Se encontrar um problema, registre-o. Não tente corrigi-lo automaticamente.

## 4. AUDITORIA DA ARQUITETURA

Analise a estrutura atual de `src/`.

Verifique:

- Se `main.rs` permanece enxuto.
- Se `app/` centraliza corretamente a inicialização e o registro dos plugins.
- Se os plugins possuem responsabilidades coerentes.
- Se os módulos estão organizados de maneira consistente.
- Se existem dependências circulares ou acoplamento excessivo.
- Se módulos estão acessando diretamente responsabilidades que deveriam pertencer a outros módulos.
- Se existem sistemas duplicados.
- Se existem componentes, recursos, eventos ou mensagens sem utilização.
- Se existem abstrações desnecessárias.
- Se existem responsabilidades concentradas em arquivos muito grandes.
- Se a organização permite adicionar novos sistemas sem comprometer os existentes.

Verifique especificamente os módulos:

- `core`
- `app`
- `input`
- `state`
- `physics`
- `camera`
- `player`
- `combat`
- `skills`
- `magic`
- `inventory`
- `enemies`
- `world`
- `save`
- `ui`
- `progress`

Confira também se a lista de plugins registrada na aplicação corresponde à arquitetura documentada.

Não considere a ausência de um plugin como erro automaticamente. Primeiro verifique se o módulo realmente precisa de um plugin próprio.

## 5. AUDITORIA DO ECS E DA EXECUÇÃO DOS SISTEMAS

Analise a organização dos conjuntos de execução do jogo.

A documentação descreve a seguinte sequência:

1. Input
2. Commands
3. Actions
4. Physics
5. Combat
6. Resolution
7. Progression
8. Presentation

Verifique:

- Se a ordem real corresponde à documentação.
- Se as dependências entre sistemas estão corretamente declaradas.
- Se existem condições de execução incompatíveis.
- Se os sistemas executam mais vezes do que deveriam.
- Se existem conflitos de acesso a componentes ou recursos.
- Se mensagens e eventos são lidos no momento correto.
- Se mensagens podem ser perdidas ou processadas em frames inesperados.
- Se os estados do jogo bloqueiam corretamente a simulação.
- Se a tela de status realmente pausa os sistemas previstos.
- Se morte, respawn e transições de estado possuem prioridades coerentes.

Analise especialmente o comportamento de `GameState::Playing` e `StatusScreen`.

Não altere a ordem dos sistemas durante a auditoria.

## 6. AUDITORIA DOS SISTEMAS DE JOGO

Analise os sistemas existentes, comparando documentação, código e testes.

### 6.1. Jogador

Verifique:

- Movimentação horizontal.
- Corrida.
- Pulo variável.
- Coyote time.
- Gravidade.
- Colisões.
- Plataformas.
- Agachamento.
- Guarda.
- Esquiva.
- Máquina de estados.
- Dano recebido.
- Morte e respawn.
- Atualização da câmera.

Identifique problemas de integração entre movimentação, combate e estados do jogador.

### 6.2. Combate

Verifique:

- Ataque básico.
- Hitboxes e hurtboxes.
- Detecção de colisão.
- Cálculo de dano.
- Defesa.
- Resistências.
- Acertos críticos.
- Invulnerabilidade.
- Knockback.
- Dano de contato dos inimigos.
- Interrupção de ações.
- Remoção de hitboxes.
- Prevenção de dano duplicado.

Confira se a documentação da fórmula de dano corresponde à implementação real.

### 6.3. Sistema de comandos

Verifique:

- Buffer de comandos.
- Tempo de validade das entradas.
- Capacidade do buffer.
- Reconhecimento de diagonais.
- Entradas neutras.
- Comandos relativos à direção do personagem.
- Prioridade entre comandos.
- Comandos incompletos.
- Comandos inválidos.
- Comandos executados com atraso.
- Feedback apresentado ao jogador.

Analise os possíveis problemas relacionados à mudança de direção durante a execução de uma sequência.

### 6.4. Habilidades e técnicas

Verifique:

- Definições das habilidades.
- Requisitos de aprendizado.
- Desbloqueio automático.
- Aprendizado por descoberta.
- Cooldowns.
- Consumo de recursos.
- Categorias.
- Efeitos.
- Execução das técnicas.
- Interrupções.
- Validação de requisitos.
- Sistema de rank, caso esteja implementado.

Confira especialmente as habilidades existentes:

- Crescent Slash.
- Rising Edge.

### 6.5. Magia

Verifique:

- Reconhecimento de comandos mágicos.
- Separação entre reconhecimento e execução.
- Consumo de MP.
- Tempo de conjuração.
- Cooldown.
- Interrupções.
- Projéteis.
- Colisões.
- Alcance.
- Cura.
- Resistências elementais.

Confira especialmente:

- Ember Bolt.
- Mending Light.

### 6.6. Atributos e progressão

Verifique:

- STR.
- CON.
- INT.
- LCK.
- HP.
- MP.
- Hearts.
- EXP.
- Level.
- Pontos de atributos.
- Crescimento automático.
- Cálculo de atributos derivados.
- Regeneração.
- Resistências.
- Limites dos atributos.
- Recompensas por derrotar inimigos.

Confira se as alterações de equipamento atualizam corretamente os atributos derivados e os valores máximos de HP e MP.

### 6.7. Inventário e equipamentos

Verifique:

- Adição e remoção de itens.
- Pilhas e limites.
- Proteção de itens de missão.
- Organização do inventário.
- Equipamento e desequipamento.
- Compatibilidade dos slots.
- Requisitos de equipamentos.
- Modificadores de atributos.
- Recálculo de atributos.
- Sistema de raridade.
- Tabelas de drops.

Não implemente interface ou funcionalidades que estejam marcadas como pendentes.

### 6.8. Inimigos

Verifique:

- Definições dos inimigos.
- Inteligência artificial.
- Patrulha.
- Perseguição.
- Ataque.
- Tempo de preparação do ataque.
- Fuga.
- Dano de contato.
- Resistências.
- Fraquezas.
- EXP.
- Drops.
- Morte.
- Respawn.

Confira se as recompensas são entregues uma única vez e se o estado do inimigo é corretamente reiniciado no respawn.

### 6.9. Mundo

Verifique:

- Carregamento das salas.
- Definições RON.
- Geometria.
- Plataformas.
- Posições de nascimento.
- Instanciação dos inimigos.
- Estrutura para futuras transições entre salas.

Não implemente transições, portas ou novas áreas nesta etapa.

### 6.10. Salvamento

Verifique:

- Estrutura dos dados salvos.
- Serialização.
- Desserialização.
- Versão do save.
- Migração.
- Arquivos corrompidos.
- Backup.
- Escrita segura.
- Carregamento.
- Restauração do personagem.
- Restauração do mundo.
- Compatibilidade com futuras alterações.

Confira especialmente se F5 e F9 executam as operações documentadas.

## 7. AUDITORIA DOS DADOS RON

Inspecione os arquivos de `assets/data/` e demais arquivos RON utilizados pelo projeto.

Verifique:

- Se todos os dados são carregados corretamente.
- Se campos obrigatórios são validados.
- Se identificadores duplicados são detectados.
- Se referências inexistentes são identificadas.
- Se valores negativos ou inválidos são rejeitados.
- Se habilidades possuem efeitos compatíveis com suas categorias.
- Se inimigos possuem parâmetros válidos.
- Se itens possuem slots e raridades válidos.
- Se dados incompletos geram erros compreensíveis.

Confira se a validação acontece antes de iniciar a execução do jogo, conforme a documentação.

## 8. AUDITORIA DOS TESTES

Analise os testes existentes em:

- Testes unitários.
- Testes de integração.
- Testes ECS headless.
- Exemplo `autoplay`.

Não considere apenas a quantidade de testes.

Verifique se os comportamentos importantes possuem cobertura adequada.

Procure especialmente:

- Testes que apenas verificam se uma função não falha.
- Testes que não verificam o resultado esperado.
- Casos extremos ausentes.
- Regras importantes sem testes.
- Testes duplicados.
- Testes dependentes da ordem de execução.
- Testes que podem passar mesmo quando a funcionalidade está incorreta.

Avalie se o `autoplay` realmente verifica os resultados das ações ou se apenas injeta entradas e gera screenshots.

Não crie uma segunda compilação completa desnecessária.

## 9. AUDITORIA DO CLAUDE CODE

Inspecione:

- `CLAUDE.md`.
- `.claude/rules/`.
- `.claude/skills/`.
- `.claude/agents/`.
- `.claude/settings.json`.

Verifique:

- Se as instruções estão consistentes entre si.
- Se existem regras contraditórias.
- Se as skills estão adequadas às tarefas descritas.
- Se os agentes possuem responsabilidades claras.
- Se há sobreposição de responsabilidades.
- Se as permissões são excessivas.
- Se os comandos destrutivos estão protegidos adequadamente.
- Se as regras de commits atômicos são suficientes.
- Se o Claude Code está instruído a preservar alterações do usuário.
- Se a documentação está orientando corretamente a execução dos testes.

Não crie agentes ou skills novos.

Apresente sugestões de melhorias, mas não modifique as configurações sem autorização.

## 10. AUDITORIA DA DOCUMENTAÇÃO

Compare a documentação com o comportamento real do código.

Consulte:

- `README.md`.
- `CLAUDE.md`.
- `docs/RESUMO_PROJETO.md`.
- `docs/ARCHITECTURE.md`.
- `docs/GAME_DESIGN.md`.
- `docs/ROADMAP.md`.
- `docs/DECISIONS.md`.
- Todos os documentos específicos dos sistemas.

Identifique:

- Funcionalidades documentadas que não existem.
- Funcionalidades existentes que não estão documentadas.
- Informações desatualizadas.
- Contradições entre documentos.
- Etapas do roadmap marcadas como concluídas sem evidência suficiente.
- Decisões técnicas que precisam de justificativa adicional.

Não reescreva os documentos nesta etapa. Apenas apresente as inconsistências encontradas.

## 11. VALIDAÇÃO TÉCNICA

Consulte primeiro o histórico recente de validações e o estado atual do Git.

Verifique se é possível reutilizar os artefatos de compilação existentes.

Execute, quando necessário e sem duplicar compilações completas:

- `cargo fmt --check`
- `cargo clippy --all-targets`
- `cargo test`
- `cargo check`

Se for necessário executar o jogo, considere o ambiente gráfico disponível e os recursos da máquina.

Não execute comandos desnecessários apenas para repetir resultados já comprovados e ainda válidos.

Registre:

- Comando executado.
- Resultado.
- Quantidade de testes aprovados.
- Avisos.
- Erros.
- Limitações do ambiente.

Não declare uma validação como aprovada se ela não foi executada ou se seu resultado não pôde ser confirmado.

## 12. CLASSIFICAÇÃO DOS PROBLEMAS

Todos os problemas encontrados devem ser classificados:

**CRÍTICO**
- Pode causar corrupção de dados.
- Pode provocar falhas graves.
- Pode comprometer a integridade dos sistemas.
- Pode tornar o projeto instável.

**ALTO**
- Afeta diretamente uma mecânica importante.
- Causa comportamento incorreto.
- Pode gerar problemas de integração ou manutenção.

**MÉDIO**
- Problema localizado.
- Cobertura insuficiente.
- Documentação incorreta.
- Risco de manutenção futura.

**BAIXO**
- Melhoria de organização.
- Pequena inconsistência.
- Ajuste não urgente.

Para cada problema,