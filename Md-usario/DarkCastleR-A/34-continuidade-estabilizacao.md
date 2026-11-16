# PROMPT MESTRE — CONTINUIDADE, ESTABILIZAÇÃO E PREPARAÇÃO PARA EXPANSÃO DO DARK CASTLE RPG

## 1. PAPEL E RESPONSABILIDADE

Você está trabalhando no projeto **Dark Castle RPG**, um RPG de ação 2D / Metroidvania desenvolvido em Rust 2024 e Bevy 0.20.1.

Atue como engenheiro de software sênior, arquiteto de software e responsável pela qualidade técnica do projeto.

Sua missão é continuar o desenvolvimento a partir do estado REAL do repositório, consolidar os resultados da auditoria E2, concluir as correções de estabilização E3 e preparar uma fundação confiável para futuras funcionalidades.

**Não reconstrua o projeto. Não recomece etapas já concluídas. Não substitua sistemas existentes sem necessidade.**

O projeto já possui aproximadamente 7.300 linhas de Rust, arquitetura modular, sistemas ECS, testes automatizados e funcionalidades jogáveis.

A prioridade atual é estabilidade, segurança, integridade dos dados e correção de bugs.

Não implemente novos chefes, mapas, magias, armas, inimigos ou mecânicas até a estabilização ser concluída.

---

## 2. DOCUMENTOS OBRIGATÓRIOS

Antes de qualquer alteração, leia e compreenda:

### Instruções e configuração

- `CLAUDE.md`
- `.cargo/config.toml`
- `.claude/settings.json`
- `.claude/rules/`
- `.claude/skills/`
- `.claude/agents/`

### Documentação técnica

- `docs/ARCHITECTURE.md`
- `docs/GAME_DESIGN.md`
- `docs/ROADMAP.md`
- `docs/DECISIONS.md`
- `docs/RESUMO_PROJETO.md`
- `docs/AUDITORIA_E2.md`
- Todos os documentos `docs/*_SYSTEM.md` relevantes.

### Briefings originais

- `Md-usario/promptMaster.md`
- `Md-usario/E2-Auditoria-Tecnica.md`
- `Md-usario/E3-Estabilizacao-correcao.md`

Os briefings originais são propriedade do usuário e devem permanecer intactos.

**Nunca modifique, mova, renomeie, sobrescreva ou exclua arquivos de `Md-usario/`.**

Caso algum briefing esteja truncado ou incompleto, registre a limitação. Não invente instruções ausentes.

O relatório `docs/AUDITORIA_E2.md` contém problemas identificados no código. Confirme cada problema antes de corrigir.

---

## 3. ESTADO CONHECIDO DO PROJETO

Último estado informado:

- Branch: `master`.
- Commit: `9dcdfcc5759860b834c3bc45343dfde2e776ec76`.
- Auditoria E2 executada.
- E3 Fase 0 executada.
- E3 Fase 1 implementada, ainda sem commit.
- Fases 2 a 7 pendentes.
- 85 testes aprovados na auditoria anterior.
- Autoplay executado com sete screenshots.
- Cinco arquivos modificados e quatro caminhos não rastreados no último `git status`.

Esses dados são históricos, não garantias sobre o estado atual.

**Verifique novamente o Git antes de agir.**

Não presuma que a árvore de trabalho está limpa.

### Alterações conhecidas da Fase 1

- `.cargo/config.toml`: `build.jobs = 4`.
- `CLAUDE.md`: política de compilação segura.
- `.claude/rules/commits.md`: orientação de compilação para validação de commits.
- `.claude/skills/atomic-commit/SKILL.md`: orientações de compilação.
- `.claude/skills/test-and-validate/SKILL.md`: orientações de validação.
- `docs/DECISIONS.md`: decisão D-011.

### Arquivos novos conhecidos

- `docs/AUDITORIA_E2.md`.
- `.cargo/config.toml`.
- Briefings E2 e E3 em `Md-usario/`.

Não adicione automaticamente os briefings ao Git.

---

## 4. REGRAS GERAIS INEGOCIÁVEIS

1. Preserve o código existente e as alterações do usuário.
2. Não altere versões de Rust, Bevy ou dependências.
3. Não adicione dependências sem justificativa e autorização.
4. Não reescreva módulos inteiros quando uma correção localizada for suficiente.
5. Não introduza abstrações sem necessidade.
6. Não execute operações destrutivas.
7. Não use `git reset --hard`, `git clean`, `git push --force` ou comandos equivalentes.
8. Não apague arquivos, branches, worktrees, caches ou saves sem autorização.
9. Não utilize `git add .` ou `git add -A` indiscriminadamente.
10. Não modifique `Md-usario/`.
11. Não altere mecânicas de gameplay além do necessário para corrigir defeitos confirmados e previamente autorizados.
12. Não declare correções concluídas sem testes.
13. Não esconda erros com supressões de lint ou tratamento fictício.
14. Não crie novos agentes ou skills nesta etapa.
15. Não execute várias compilações pesadas simultaneamente.
16. Não utilize `/tmp` como diretório de build do Bevy.
17. Não execute `cargo clean`.
18. Não altere configurações de Git remotas.
19. Não faça push sem autorização.
20. Não execute fases posteriores sem autorização explícita.

Quando houver dúvida entre preservar dados e concluir uma tarefa, priorize a preservação dos dados.

---

# PARTE I — CONSOLIDAÇÃO DA E2 E DA FASE 1 DA E3

## 5. FASE A — INSPEÇÃO DO ESTADO ATUAL

Esta é a única fase autorizada a iniciar imediatamente.

### 5.1. Inspeção do Git

Execute verificações somente leitura:

```bash
git status --short
git status --branch
git branch --show-current
git rev-parse HEAD
git log -8 --oneline
git diff --check
git diff --stat
git diff --cached --stat
git worktree list
```

Inspecione também arquivos não rastreados, sem alterá-los.

Confirme:

- Branch atual.
- Hash do HEAD.
- Existência de alterações staged e unstaged.
- Arquivos novos.
- Existência de worktrees.
- Diferenças em relação ao estado anteriormente informado.

Não modifique o Git durante a inspeção.

### 5.2. Revisão da Fase 1

Inspecione integralmente:

- `.cargo/config.toml`
- `CLAUDE.md`
- `.claude/rules/commits.md`
- `.claude/skills/atomic-commit/SKILL.md`
- `.claude/skills/test-and-validate/SKILL.md`
- `docs/DECISIONS.md`

Confirme que:

1. O Cargo utiliza quatro jobs por padrão.
2. As configurações existentes foram preservadas.
3. O diretório de compilação padrão é o `target/` da raiz.
4. O projeto não cria novos diretórios de build desnecessários.
5. O Claude Code possui instruções claras para evitar travamentos.
6. Os comandos de validação respeitam os limites de memória.
7. As instruções não se contradizem.
8. O perfil `dist` não é usado em validações rotineiras.
9. Nenhum arquivo de gameplay foi alterado.
10. Nenhum arquivo do usuário foi sobrescrito.

### 5.3. Revisão dos arquivos novos

O comando `git diff --check` normalmente não examina arquivos não rastreados.

Verifique separadamente:

- `.cargo/config.toml`.
- `docs/AUDITORIA_E2.md`.

Não use `git add -N` sem necessidade. Caso seja útil para a inspeção, explique previamente seu efeito sobre o índice.

Verifique sintaxe, formatação, integridade e conteúdo.

### 5.4. Política de worktrees

Minha preferência para o projeto é:

- Não utilizar worktrees para validação rotineira.
- Não exigir recompilação isolada de todos os commits.
- Não criar worktrees em `/tmp`.
- Não executar builds simultâneos compartilhando artefatos.
- Permitir worktree dentro de `target/` apenas como exceção, mediante necessidade técnica comprovada e autorização específica.

Analise se as instruções atuais refletem essa política.

Se for necessária alteração, apresente a proposta, mas não a implemente nesta fase.

### 5.5. Resultado da Fase A

Entregue um relatório com:

1. Estado do Git.
2. Alterações encontradas.
3. Validação das configurações.
4. Problemas ou contradições.
5. Arquivos que precisam de ajustes.
6. Proposta de divisão em commits.
7. Riscos restantes.

**Pare ao concluir a Fase A. Não faça commits automaticamente.**

---

# PARTE II — CONSOLIDAÇÃO DOS COMMITS

## 6. FASE B — COMMITS ATÔMICOS

Executar somente após autorização.

Objetivo: registrar o trabalho concluído sem misturar responsabilidades.

Proposta inicial:

### Commit 1 — Auditoria

`docs: add stage 2 technical audit`

Conteúdo:

- `docs/AUDITORIA_E2.md`.

### Commit 2 — Configuração de compilação

`build: limit cargo parallel jobs`

Conteúdo:

- `.cargo/config.toml`.
- Registro D-011, se for coerente com o escopo.

### Commit 3 — Política de compilação segura

`docs(claude): enforce safe build workflow`

Conteúdo:

- `CLAUDE.md`.
- `.claude/rules/commits.md`.
- Skills modificadas.

Antes de cada commit:

- Confira o diff.
- Verifique os arquivos selecionados.
- Execute somente as validações pertinentes.
- Preserve alterações não relacionadas.
- Não inclua `Md-usario/`.
- Não inclua `target/` ou `saves/`.

Depois de cada commit, registre o hash.

Se não for possível separar os commits sem modificar alterações preexistentes, pare e explique.

---

# PARTE III — ESTABILIZAÇÃO E3

## 7. FASE 2 — SEGURANÇA DO CLAUDE CODE

Executar somente após autorização.

### Objetivo

Reduzir o risco de operações destrutivas ou alterações não autorizadas.

### Verificações

Analise:

- `.claude/settings.json`.
- Permissões de Bash.
- Permissões de leitura e escrita.
- Operações Git.
- Scripts e comandos indiretos.
- Skills que executam comandos.
- Proteção de `Md-usario/`.
- Proteção de `saves/`.
- Permissões concedidas a agentes.

### Problemas conhecidos

A auditoria identificou que:

- `rm -rf` é bloqueado parcialmente.
- Variantes de remoção podem escapar.
- Comandos Git podem sobrescrever arquivos usando opções como `--output`.
- `cargo fmt` pode modificar arquivos fora do escopo.
- Algumas permissões são excessivamente amplas.
- Os briefings não possuem proteção suficiente.
- Saves reais podem ser sobrescritos por testes.

### Resultado esperado

Criar uma política de menor privilégio compatível com a versão instalada do Claude Code.

Não presuma que regras de permissão baseadas em padrões bloqueiam todas as variantes de comandos.

Verifique o funcionamento real das restrições, sem executar operações destrutivas.

Priorize:

1. Proteção dos arquivos do usuário.
2. Proteção dos saves.
3. Proteção do Git.
4. Restrição de comandos potencialmente destrutivos.
5. Preservação dos comandos necessários ao desenvolvimento.

Não torne o ambiente inutilizável por excesso de bloqueios.

Documente as limitações das proteções implementadas.

---

## 8. FASE 3 — AUTOPLAY E TESTES DE INTEGRAÇÃO

Executar somente após autorização.

### Problema

O autoplay atual:

- Pode sobrescrever `saves/slot1.ron`.
- Produz screenshots sem validar os resultados.
- Pode terminar com sucesso mesmo quando ações falham.
- Não garante que golpes acertaram.
- Pode tratar resultados inesperados como sucesso.

### Objetivos

1. Isolar completamente os saves de teste.
2. Garantir que o autoplay nunca sobrescreva saves reais.
3. Transformar o autoplay em um teste que detecta falhas.
4. Validar os resultados das ações executadas.
5. Produzir logs claros.
6. Preservar screenshots para inspeção.

### Requisitos

Implemente um mecanismo configurável para o diretório de saves.

A solução deve funcionar no Linux e permitir futura compatibilidade com Windows.

Não migre automaticamente saves existentes.

Não sobrescreva arquivos reais.

O autoplay deve usar um diretório isolado, preferencialmente dentro de `target/`, sem compartilhar caminhos com os saves do jogador.

### Asserções mínimas

O autoplay deve verificar:

- Jogador criado.
- Sala inicial carregada.
- Ataque básico executado.
- Crescent Slash ativado e acertando o alvo esperado.
- Ember Bolt ativado e causando dano.
- Mending Light executada e restaurando HP quando aplicável.
- Rising Edge bloqueado quando não aprendido.
- Nenhum ataque básico indevido após comando rejeitado.
- Save de teste criado e decodificado.
- Ausência de falhas inesperadas.

Se uma asserção falhar, o processo deve terminar com erro.

Não considere screenshot isolada como prova de funcionamento.

### Skill de validação

Revise `/test-and-validate`.

Não trate `timeout` como prova de que o jogo iniciou.

Diferencie:

- Compilação concluída.
- Aplicação iniciada.
- Janela criada.
- Teste executado.
- Asserções aprovadas.

---

## 9. FASE 4 — CORREÇÕES DE COMBATE

Executar somente após autorização.

### 9.1. Problema A1 — Skill executa mesmo quando o início falha

Arquivos relacionados:

- `src/skills/systems.rs`.
- `src/combat/attacks.rs`.
- `src/magic/casting.rs`.

Objetivo:

Garantir que efeitos de habilidades só aconteçam após a confirmação de que a ação foi iniciada com sucesso.

Regras:

- Validar estado antes de cobrar recursos.
- Não consumir MP ou Hearts quando a ativação falhar.
- Não iniciar cooldown quando a ativação falhar.
- Não criar hitboxes ou projéteis quando a ativação falhar.
- Não aplicar movimento especial quando a ativação falhar.
- Evitar mensagens de sucesso falsas.

Considere uma mensagem de confirmação como `SkillStarted`, caso seja a menor solução coerente com a arquitetura existente.

Crie testes ECS que reproduzam a ativação de skill durante esquiva e outros estados incompatíveis.

### 9.2. Problema A2 — Dano não interrompe ações

Objetivo:

Garantir que o jogador não continue causando dano por uma técnica interrompida indevidamente.

Verifique:

- Hitboxes vinculadas ao jogador.
- Entrada em `Hurt`.
- Entrada em `Dead`.
- Remoção ou cancelamento de ataques.
- Conjuração de magias.
- Interrupção de habilidades.

A política definitiva de conjuração será aprovada separadamente.

Não transforme automaticamente todas as magias em conjurações longas.

### 9.3. Problema A3 — Dano múltiplo durante invulnerabilidade

Objetivo:

Garantir que a invulnerabilidade seja aplicada corretamente mesmo quando vários eventos de dano chegam no mesmo frame.

Verifique:

- Ordem de aplicação de dano.
- Estado de invulnerabilidade.
- Dano por contato.
- Dano de hitboxes.
- Dano simultâneo.
- Knockback.
- Morte.

Crie testes que produzam dois ou mais danos no mesmo frame.

O comportamento esperado é que a invulnerabilidade impeça danos adicionais após o primeiro acerto válido, respeitando as regras de gameplay aprovadas.

### 9.4. Comandos e fallback

A auditoria identificou problemas envolvendo comandos rejeitados e ataques básicos indevidos.

Investigue:

- Comando não aprendido.
- Comando em cooldown.
- Recursos insuficientes.
- Comando incompleto.
- Comando inválido.
- Mudança de orientação durante a sequência.
- Guarda e esquiva durante a execução.

Não implemente automaticamente uma nova política de fallback sem aprovação.

Apresente uma tabela de comportamentos atuais e propostos.

---

## 10. FASE 5 — VALIDAÇÃO DOS DADOS RON

Executar somente após autorização.

### Objetivo

Garantir que dados inválidos sejam rejeitados antes da inicialização do jogo.

### Verificações obrigatórias

- IDs duplicados.
- Referências inexistentes.
- HP máximo igual ou inferior a zero.
- Velocidades negativas.
- Tamanhos inválidos.
- Cooldowns negativos.
- Tempos de respawn inválidos.
- Chances fora do intervalo permitido.
- Fórmulas com limites inconsistentes.
- Valores NaN ou infinitos.
- Requisitos de habilidades inválidos.
- Referências de drops inexistentes.
- Slots de equipamento incompatíveis.
- Categorias de habilidades incompatíveis com seus efeitos.
- Configurações inválidas de inimigos.
- Configurações inválidas de salas.

### Arquitetura

Centralize a validação no fluxo de carregamento de `GameData`, respeitando a organização existente.

Use erros descritivos que indiquem:

- Arquivo.
- Identificador.
- Campo inválido.
- Motivo da rejeição.

Não use `panic!` para erros normais de configuração.

Crie testes para cada categoria importante de erro.

---

## 11. FASE 6 — SAVE/LOAD E INTEGRIDADE DO MUNDO

Executar somente após autorização.

### Problemas conhecidos

- F9 não recarrega completamente a sala.
- Entidades antigas permanecem após carregar.
- Projéteis e hitboxes podem continuar ativos.
- Inimigos podem permanecer em estados incompatíveis com o save.
- Escrita sem garantias suficientes de persistência.
- Backup pode ser sobrescrito quando o principal está corrompido.
- Recuperação de backup não está integrada adequadamente.

### Objetivos

1. Garantir restauração consistente do jogador.
2. Garantir restauração consistente do mundo.
3. Limpar entidades transitórias quando necessário.
4. Restaurar corretamente inimigos e progresso.
5. Garantir consistência de atributos derivados.
6. Melhorar a segurança de escrita.
7. Preservar backups válidos.
8. Tratar saves corrompidos sem destruir dados.
9. Preparar a arquitetura para futuras transições entre salas.

### Requisitos

Não migre automaticamente os saves existentes para outro diretório.

Não sobrescreva saves reais durante testes.

Não introduza uma segunda implementação independente de carregamento de salas.

Se for necessário um fluxo centralizado de carregamento, integre-o ao módulo `world` existente.

Crie testes de:

- Round-trip de save/load.
- Corrupção de arquivo.
- Backup válido.
- Backup inválido.
- Interrupção de escrita, quando simulável.
- Carregamento de sala.
- Restauração de inimigos.
- Limpeza de projéteis e hitboxes.
- Restauração de atributos e equipamentos.

Documente claramente quais garantias de escrita são efetivamente oferecidas em cada plataforma.

Não prometa proteção absoluta contra corrupção.

---

## 12. FASE 7 — VALIDAÇÃO FINAL DA ESTABILIZAÇÃO

Executar somente após autorização.

### Validação técnica

Execute com segurança:

```bash
cargo fmt --check
cargo clippy --all-targets
cargo test
cargo check
```

Execute o autoplay corrigido, se o ambiente gráfico estiver disponível.

Respeite a política de compilação segura.

### Revisão independente

Utilize o agente `code-reviewer` em modo somente leitura, caso esteja disponível.

A revisão deve verificar:

- Correções implementadas.
- Possíveis regressões.
- Testes de regressão.
- Integridade do save.
- Segurança do ambiente.
- Consistência da documentação.
- Ausência de alterações fora do escopo.

### Documentação

Atualize, quando autorizado:

- `docs/ARCHITECTURE.md`.
- `docs/ROADMAP.md`.
- `docs/DECISIONS.md`.
- `docs/RESUMO_PROJETO.md`.
- Documentos específicos dos sistemas.

Não marque uma funcionalidade como concluída sem evidência.

### Critérios de aprovação

A estabilização só poderá ser considerada concluída quando:

1. Problemas críticos estiverem resolvidos.
2. Problemas altos estiverem resolvidos ou formalmente aceitos como risco.
3. Testes de regressão relevantes estiverem passando.
4. Autoplay estiver isolado dos saves reais.
5. Dados inválidos forem rejeitados adequadamente.
6. Save/load tiver comportamento consistente.
7. Configurações do Claude Code estiverem revisadas.
8. Documentação estiver sincronizada.
9. Não houver alterações indevidas.
10. O Git estiver em estado conhecido e verificável.

---

# PARTE IV — DECISÕES DE GAME DESIGN PENDENTES

## 13. DECISÕES QUE EXIGEM APROVAÇÃO

Não tome decisões definitivas sobre os seguintes pontos sem consultar o usuário.

### D-GAME-001 — Worktrees

Preferência provisória:

Não usar worktrees rotineiramente. Permitir exceções controladas somente quando necessárias.

### D-GAME-002 — Mending Light

Opções:

- Cura instantânea.
- Cura com conjuração interrompível.
- Comportamento configurável por habilidade.

**Proposta:** permitir que cada habilidade tenha sua própria política de conjuração, sem obrigar todas as magias a funcionar da mesma maneira.

Não implemente até aprovação.

### D-GAME-003 — Fallback de comandos

Definir quando um comando rejeitado pode ou não resultar em ataque básico.

Proposta inicial:

- Ataque sem comando reconhecido: ataque básico.
- Comando reconhecido, mas não aprendido: não executar ataque básico.
- Comando reconhecido em cooldown: não executar ataque básico.
- Comando reconhecido sem recurso: não executar ataque básico.
- Comando reconhecido em estado inválido: não executar ataque básico.

Casos incompletos e inválidos precisam de análise específica.

### D-GAME-004 — Orientação durante comandos

Definir se a orientação será fixada no início da sequência, no momento da ativação ou se o buffer manterá entradas físicas independentes da orientação.

Apresente consequências para comandos com frente e trás.

### D-GAME-005 — Localização dos saves

Manter temporariamente a pasta atual para compatibilidade.

Planejar posteriormente uma política multiplataforma para Linux e Windows.

Não migrar arquivos existentes sem autorização.

---

# PARTE V — ORGANIZAÇÃO E ENTREGA

## 14. RELATÓRIOS DE CADA FASE

Ao concluir uma fase autorizada, apresente:

### Identificação

- Nome da fase.
- Branch.
- HEAD inicial.
- HEAD final.
- Data.
- Arquivos alterados.

### Trabalho realizado

- Problemas corrigidos.
- Implementações realizadas.
- Decisões técnicas.
- Alterações de arquitetura.

### Testes

Para cada teste:

- Comando executado.
- Resultado.
- Quantidade de testes.
- Falhas.
- Limitações.

### Segurança

Confirme:

- `Md-usario/` preservado.
- Saves reais preservados.
- Nenhuma operação destrutiva.
- Nenhuma compilação pesada simultânea.
- Nenhuma alteração não autorizada.

### Git

- Commits realizados, se autorizados.
- Arquivos staged.
- Arquivos unstaged.
- Arquivos não rastreados.
- Estado final da árvore de trabalho.

### Pendências

Classifique:

- CRÍTICO.
- ALTO.
- MÉDIO.
- BAIXO.

Não esconda problemas restantes.

---

## 15. POLÍTICA DE COMMITS

Cada correção deve ser pequena, testável e independente.

Exemplos futuros:

```text
fix(skills): prevent effects when activation fails
test(skills): cover activation during dodge
fix(combat): prevent multiple damage during invulnerability
fix(combat): cancel attacks when player is hurt
fix(data): reject invalid ron definitions
fix(save): restore room state on load
test(save): cover backup recovery
test(app): assert autoplay outcomes
```

Esses são exemplos, não autorização para commitar.

Antes de qualquer commit:

- Verifique o Git.
- Inspecione o diff.
- Execute testes pertinentes.
- Confirme o escopo.
- Preserve arquivos do usuário.

Não crie commits automaticamente se a fase não estiver autorizada.

---

## 16. PROIBIÇÃO DE EXPANSÃO PREMATURA

Durante E3, não implementar:

- Chefes.
- Novos inimigos.
- Novas magias.
- Novas armas.
- Novos mapas.
- Transições entre salas como funcionalidade de exploração.
- Menus completos.
- Sistema de missões.
- Multiplayer.
- Novos agentes ou skills.
- Refatorações amplas.
- Atualizações de engine.

Exceção: pequenas adaptações estruturais estritamente necessárias para corrigir problemas confirmados da estabilização.

---

## 17. EXECUÇÃO IMEDIATA

**Sua autorização atual é exclusivamente para a FASE A — INSPEÇÃO DO ESTADO ATUAL.**

Execute agora:

1. Leia os documentos obrigatórios.
2. Inspecione o Git.
3. Revise os seis arquivos da Fase 1.
4. Verifique os arquivos não rastreados.
5. Confirme a segurança da configuração Cargo.
6. Identifique contradições e alterações indevidas.
7. Apresente uma proposta de commits atômicos.
8. Registre as decisões ainda pendentes.
9. Entregue um relatório objetivo e verificável.

**Não altere arquivos nesta execução.**

**Não faça commits.**

**Não execute as Fases B, 2, 3, 4, 5, 6 ou 7.**

Ao terminar, pare e aguarde minha autorização.

O objetivo é transformar o Dark Castle RPG em uma base estável, testada, segura e preparada para crescimento, sem comprometer o trabalho já realizado.