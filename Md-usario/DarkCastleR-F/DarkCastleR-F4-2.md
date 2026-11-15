# DARK CASTLE RPG — RETOMADA DA FASE 4
## Verificação do arquivo restaurado e diagnóstico multiagente

Leia e execute as instruções de `Md-usario/DarkCastleR-F4.md`, respeitando os limites adicionais deste pedido.

### 1. Situação atual

O usuário utilizou `Ctrl + Z` manualmente no editor para desfazer a colagem acidental que havia corrompido `src/combat/attacks.rs`.

O arquivo aparentemente voltou ao código Rust original.

**Não restaure nem sobrescreva o arquivo automaticamente.**

Primeiro confirme que a alteração acidental realmente desapareceu.

Execute:

```bash
git status --short
git diff -- src/combat/attacks.rs
git diff --check
```

Compare também o conteúdo atual com:

```bash
git show HEAD:src/combat/attacks.rs
```

Verifique:

- Se as aproximadamente 229 linhas de texto coladas desapareceram.
- Se os imports originais foram recuperados.
- Se não existem alterações inesperadas no arquivo.
- Se o arquivo corresponde exatamente à versão do HEAD.
- Se os únicos arquivos não rastreados são os briefings esperados em `Md-usario/`.

Se `attacks.rs` ainda estiver modificado, apresente o diff e interrompa. Não execute `git restore`, não sobrescreva o arquivo e não descarte alterações.

Se estiver idêntico ao HEAD, prossiga.

### 2. Retomar a Fase 4

A Fase 4 ainda não teve seu diagnóstico concluído nem sua implementação iniciada.

Retome exclusivamente a **Etapa A — Diagnóstico técnico de A1, A2 e A3** descrita em `Md-usario/DarkCastleR-F4.md`.

Leia as instruções de `CLAUDE.md`, as regras aplicáveis de `.claude/rules/` e a documentação relevante.

Não altere código nesta execução.

### 3. Utilizar os agentes especializados

Quero aproveitar os agentes existentes para realizar uma análise mais profunda e independente.

Utilize os agentes disponíveis, se estiverem corretamente configurados.

**Agente `game-architect` — Arquitetura e execução ECS**

Responsabilidades:

- Mapear o pipeline de combate.
- Identificar os schedules e `GameSet` envolvidos.
- Verificar a ordem de produção e consumo de mensagens.
- Identificar dependências entre A1, A2 e A3.
- Analisar riscos de alterar a ordem dos sistemas.
- Propor a menor intervenção arquitetural possível.

Somente leitura. Não editar arquivos.

**Agente `combat-specialist` — Investigação das falhas**

Responsabilidades:

- Investigar a causa-raiz de A1.
- Investigar cancelamento de ações, hitboxes e magias em A2.
- Investigar processamento de múltiplos danos e invulnerabilidade em A3.
- Identificar os pontos exatos que precisam ser corrigidos.
- Propor testes de regressão para cada defeito.
- Consultar os dados RON antes de propor alterações que afetem habilidades.

Nesta etapa, somente leitura. Não implementar.

**Agente `code-reviewer` — Revisão independente**

Responsabilidades:

- Revisar as conclusões dos outros agentes.
- Procurar efeitos colaterais e regressões.
- Identificar riscos de ECS, ownership e borrowing.
- Verificar se alguma proposta altera regras de gameplay.
- Avaliar se os testes propostos realmente reproduzem os defeitos.
- Apontar soluções excessivamente complexas.

Somente leitura.

Os agentes podem realizar análises independentes em paralelo, desde que não editem arquivos nem iniciem compilações concorrentes.

Se algum agente não estiver disponível, informe isso e realize a análise correspondente diretamente, sem inventar resultados de agentes que não foram executados.

### 4. Consolidar as conclusões

Depois das análises, produza um relatório unificado.

Para cada problema A1, A2 e A3, apresente:

1. Causa-raiz.
2. Arquivo e função envolvidos.
3. Evidência concreta no código.
4. Cenário de reprodução.
5. Consequência no gameplay.
6. Correção recomendada.
7. Arquivos que precisarão mudar.
8. Testes necessários.
9. Riscos e possíveis regressões.
10. Dependências de outras correções.

Se os agentes discordarem, apresente as divergências e justifique a recomendação final.

### 5. Planejamento da implementação

Monte um plano de execução para a próxima autorização.

Separe a implementação em blocos:

- Bloco A1 — Ativação consistente das habilidades.
- Bloco A2 — Cancelamento correto de ações interrompidas.
- Bloco A3 — Invulnerabilidade e múltiplos danos no mesmo frame.

Identifique quais blocos podem ser implementados independentemente e quais dependem de alterações anteriores.

Sugira a divisão de responsabilidades entre os agentes, mas não permita edições simultâneas dos mesmos arquivos.

Não execute compilações paralelas.

### 6. Restrições obrigatórias

- Não modifique `Md-usario/`.
- Não altere os arquivos RON.
- Não modifique os saves reais.
- Não apague `target/verify/`.
- Não crie worktrees.
- Não execute `cargo clean`.
- Não altere dependências.
- Não faça push.
- Não crie commits.
- Não implemente A1, A2 ou A3 nesta execução.
- Não altere skills, agentes ou regras do Claude Code.
- Não inicie as Fases 5, 6 ou 7.

### 7. Entrega final

Apresente:

1. Resultado da verificação de `src/combat/attacks.rs`.
2. Estado atual do Git.
3. Agentes realmente utilizados.
4. Diagnóstico de A1, A2 e A3.
5. Divergências identificadas.
6. Plano de implementação recomendado.
7. Testes de regressão planejados.
8. Questões que dependem de decisão do usuário.

**Ao terminar, pare e aguarde minha autorização para iniciar a implementação da Fase 4.**