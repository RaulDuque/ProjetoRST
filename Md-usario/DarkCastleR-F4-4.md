# DARK CASTLE RPG — FASE 4, ETAPA C
## Auditoria final antes dos commits

Leia `Md-usario/DarkCastleR-F4.md`, `DarkCastleR-F4-2.md`, `DarkCastleR-F4-3.md` e o relatório mais recente da implementação.

A Fase 4 foi implementada, mas ainda não foi commitada.

**Autorizo exclusivamente uma auditoria final, correções de defeitos comprovados dentro do escopo A1/A2/A3 e a preparação de um plano de commits.**

Não crie commits nesta execução.

## 1. Revisão independente

Utilize `code-reviewer` para revisar o diff completo, em modo somente leitura.

Utilize `game-architect` para verificar a ordem dos sistemas ECS, o fluxo de mensagens e a consistência da arquitetura.

O agente principal deve consolidar as conclusões. Use `combat-specialist` somente se alguma correção comprovada for necessária, e nunca em paralelo com outra edição dos mesmos arquivos.

Confira especialmente:

- `SkillActivated` versus `SkillStarted`.
- Se cada tentativa produz exatamente um `CommandResolved`.
- Se o pagamento de MP e Hearts é realmente atômico.
- Se a validação usa o estado correto no frame.
- Se as hitboxes são removidas antes de poderem acertar novamente.
- Se eventos de dano não permanecem pendentes após a morte.
- Se projéteis já lançados continuam existindo.
- Se D-GAME-003 permaneceu inalterado.
- Se a ordem entre `tick_cooldowns` e `begin_skill` introduz algum comportamento de borda.
- Se as mudanças em `src/combat/attacks.rs` são exclusivamente as necessárias para A1.

Classifique cada achado por gravidade e apresente evidência em arquivo e linha.

## 2. Auditoria dos 36 testes novos

Examine especialmente as 1.287 linhas adicionadas a `tests/app_smoke_tests.rs`.

Verifique:

1. Se os testes comprovam os defeitos reais.
2. Se as asserções são suficientemente fortes.
3. Se existem testes duplicados.
4. Se algum teste depende de ordem acidental entre sistemas.
5. Se há testes que passam sem executar o cenário anunciado.
6. Se a regressão do respawn realmente está coberta.
7. Se há dependência indevida de timing ou regeneração.
8. Se os testes de A1, A2 e A3 podem ser separados em commits atômicos.

Não reduza cobertura apenas para diminuir o tamanho do arquivo.

Não reescreva testes funcionais por preferência estética.

## 3. Documentação e skill

A skill `.claude/skills/combat-system/SKILL.md` ainda descreve o fluxo antigo.

Autorizo exclusivamente a correção dessa descrição para refletir:

`SkillActivated` → `begin_skill` → `SkillStarted` → executores de efeitos.

Não altere outras instruções da skill.

Confira também se `docs/ARCHITECTURE.md`, `docs/COMBAT_SYSTEM.md`, `docs/MAGIC_SYSTEM.md` e `docs/DECISIONS.md` estão coerentes com o código.

## 4. Planejamento dos commits

Prepare um plano de commits atômicos, preferencialmente:

1. `fix(player): respect invulnerability for each damage event`
2. `fix(skills): confirm activation before executing effects`
3. `fix(combat): cancel interrupted melee hitboxes`
4. `docs(combat): document combat stabilization`

Os testes devem acompanhar a correção correspondente.

Como `tests/app_smoke_tests.rs` contém testes dos três blocos, proponha uma estratégia segura para construir as versões intermediárias no índice Git.

**Não presuma que testar a árvore de trabalho completa comprova que cada commit isolado compila.**

Se for possível verificar as versões intermediárias sem criar worktrees, sem outro `target/` e sem comandos destrutivos, descreva como.

Se não for possível demonstrar essa independência com segurança, informe a limitação e proponha uma alternativa antes de commitar.

Não use `git add .`, `git add -A`, `git reset --hard`, `git clean`, `git checkout -- .` ou `git restore` para descartar mudanças.

Não altere o índice Git nesta etapa.

## 5. Validação

Se houver alterações funcionais durante esta auditoria, execute as verificações pertinentes novamente.

Se houver apenas a atualização documental da skill, valide o diff e a consistência da documentação sem recompilar desnecessariamente.

Preserve:

- `Md-usario/`
- `saves/`
- `target/verify/`
- O `target/` existente

Não crie worktrees, não faça push e não inicie a Fase 5.

## 6. Entrega

Apresente:

1. Resultado da revisão de código.
2. Resultado da auditoria dos testes.
3. Problemas encontrados e eventuais correções.
4. Confirmação da documentação e skill.
5. Plano exato dos commits, com arquivos e trechos.
6. Estratégia para validar os commits intermediários.
7. Estado final do Git.
8. Integridade dos arquivos protegidos.

Ao terminar, pare e aguarde minha autorização para criar os commits.