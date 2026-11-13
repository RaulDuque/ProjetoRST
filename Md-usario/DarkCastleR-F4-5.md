# DARK CASTLE RPG
# SUPER PROMPT MASTER — CONSOLIDAÇÃO DA FASE 4 + PLANEJAMENTO VISUAL V1

## PAPEL

Você é o coordenador principal de engenharia, arquitetura, qualidade e direção artística do Dark Castle RPG.

O projeto é um RPG de ação 2D desenvolvido em Rust + Bevy, inspirado na atmosfera de Castlevania: Symphony of the Night, mas com identidade própria.

Este super prompt reúne duas missões:

**MISSÃO A — Finalizar a Fase 4:** validar e consolidar as correções A1, A2 e A3 em commits atômicos.

**MISSÃO B — Planejar a evolução visual V1:** estudar a modernização do protagonista, suas animações e a integração Blender → Bevy, usando múltiplos agentes quando disponíveis.

As missões devem ser executadas sequencialmente.

É proibido iniciar a Missão B antes de concluir com sucesso a Missão A.

---

# PARTE 0 — REGRAS GERAIS

## 0.1. Leitura obrigatória

Antes de executar qualquer ação:

1. Leia `CLAUDE.md`.
2. Leia as regras de `.claude/rules/`.
3. Leia as instruções relevantes das skills existentes.
4. Leia `docs/ARCHITECTURE.md`.
5. Leia `docs/ROADMAP.md`.
6. Leia `docs/DECISIONS.md`.
7. Leia `docs/COMBAT_SYSTEM.md`.
8. Leia `docs/MAGIC_SYSTEM.md`.
9. Leia `docs/SKILL_SYSTEM.md`, se existir.
10. Leia `docs/CHARACTER_ATTRIBUTES.md`.
11. Leia `docs/INPUT_SYSTEM.md`.
12. Leia `Md-usario/DarkCastleR-F4.md`.
13. Leia `Md-usario/DarkCastleR-F4-2.md`.
14. Leia `Md-usario/DarkCastleR-F4-3.md`.
15. Leia `Md-usario/DarkCastleR-F4-4.md`, se existir.
16. Leia `Md-usario/DarkCastleR-V1.md`, se existir.

Se algum arquivo não existir, registre a ausência. Não invente seu conteúdo.

O arquivo V1, quando existir, deve ser tratado como especificação detalhada complementar da Missão B.

## 0.2. Proteções absolutas

Preserve integralmente:

- `Md-usario/`
- `saves/`
- `target/verify/`
- `target/` e seu cache de compilação
- Arquivos de configuração não relacionados ao escopo
- Assets e dados RON existentes

Não execute:

- `git clean`
- `git reset --hard`
- `cargo clean`
- `git checkout -- .`
- `git restore` para descartar alterações
- `git add .`
- `git add -A`
- `git push`

Não crie worktrees ou clones sem autorização.

Não crie outro diretório de compilação.

Não compile em `/tmp`.

Não execute múltiplas compilações Cargo simultaneamente.

Não modifique o jogo durante a Missão B.

## 0.3. Estado inicial

Antes de qualquer alteração, registre:

- Branch atual.
- HEAD completo.
- `git status`.
- `git diff --stat`.
- `git diff --cached`.
- Arquivos não rastreados.
- Lista dos arquivos modificados.
- Disponibilidade de espaço e memória para validação.
- Existência dos arquivos protegidos.

Compare com o relatório da Fase 4 — Etapa C.

O relatório anterior indicou:

- Branch `master`.
- HEAD `df6727a`.
- 18 arquivos modificados.
- Índice vazio.
- 123 testes aprovados.
- Clippy sem avisos.
- Autoplay anterior com 23/23 verificações.
- Cinco commits propostos.

Esses valores são históricos. Confirme o estado real antes de agir.

Se encontrar alterações inesperadas, interrompa.

---

# PARTE 1 — ORGANIZAÇÃO MULTIAGENTE

## 1.1. Descoberta

Verifique os agentes e skills realmente disponíveis.

Não invente agentes.

Não declare que um agente foi utilizado se ele não foi executado.

Use especialistas existentes quando suas atribuições forem compatíveis.

Se algum agente não existir, o coordenador pode realizar a análise correspondente.

## 1.2. Especialistas da Missão A

### Code Reviewer

Revisar o plano dos commits e identificar riscos.

### Game Architect

Verificar dependências entre módulos, ECS e ordem dos sistemas.

### Test Specialist

Avaliar a consistência dos testes e a estratégia de validação dos commits intermediários.

Se não existir agente específico, o coordenador assume a função.

## 1.3. Especialistas da Missão B

### Game Architect

Analisar a arquitetura existente do protagonista.

### Visual Director

Definir propostas originais de identidade visual.

### Animation Specialist

Planejar animações, transições e prioridades.

### Rendering Specialist

Avaliar integração com Bevy, sprites, atlases, materiais e desempenho.

### Code Reviewer

Revisar criticamente a proposta arquitetural.

## 1.4. Paralelismo

Agentes de leitura podem trabalhar em paralelo.

Não permitir edição concorrente dos mesmos arquivos.

Somente o coordenador controla operações Git.

Somente uma compilação Cargo por vez.

Os especialistas da Missão B só podem iniciar após a aprovação do portão de segurança da Missão A.

---

# PARTE 2 — MISSÃO A
# CONSOLIDAÇÃO DA FASE 4

## 2.1. Objetivo

Consolidar as correções:

- A1 — Ativação de habilidades sem efeitos indevidos após rejeição.
- A2 — Cancelamento de ataques e hitboxes após interrupção.
- A3 — Respeito à invulnerabilidade ao processar múltiplos danos no mesmo frame.

A implementação e a auditoria já foram realizadas.

Não reimplemente essas correções.

Não altere gameplay, balanceamento ou arquitetura fora do escopo.

## 2.2. Revisão inicial

Confirme:

1. Separação `SkillActivated` / `SkillStarted`.
2. Pagamento atômico de MP e Hearts.
3. Resultado único de `CommandResolved` por tentativa.
4. Cancelamento de hitboxes interrompidas.
5. Preservação de projéteis já lançados.
6. Tratamento correto de danos fatais e mensagens restantes.
7. Preservação de D-GAME-003.
8. Consistência dos 37 testes novos.
9. Coerência da documentação.
10. Integridade da skill `/combat-system`.

Não faça outra grande auditoria se o estado estiver idêntico ao relatório da Etapa C.

## 2.3. Commits autorizados

Os commits planejados são:

### Commit 1

`fix(player): respect invulnerability for each damage event`

Escopo:

- `src/player/systems.rs`
- Testes A3 correspondentes em `tests/app_smoke_tests.rs`

### Commit 2

`fix(skills): confirm activation before executing effects`

Escopo:

- `src/core/events.rs`
- `src/core/mod.rs`
- `src/combat/command_input.rs`
- `src/skills/systems.rs`
- `src/skills/mod.rs`
- `src/combat/attacks.rs`
- `src/magic/casting.rs`
- Testes A1 correspondentes

### Commit 3

`fix(combat): cancel interrupted melee hitboxes`

Escopo:

- `src/combat/hitboxes.rs`
- `src/combat/mod.rs`
- Testes A2 correspondentes

### Commit 4

`docs(combat): document combat stabilization`

Escopo:

- `docs/ARCHITECTURE.md`
- `docs/COMBAT_SYSTEM.md`
- `docs/MAGIC_SYSTEM.md`
- `docs/INPUT_SYSTEM.md`
- `docs/DECISIONS.md`

### Commit 5

`docs(claude): sync combat flow in rules and skill`

Escopo:

- `.claude/rules/architecture.md`
- `.claude/skills/combat-system/SKILL.md`

Não incluir outros arquivos sem justificativa e autorização.

## 2.4. Validação dos estados intermediários

O arquivo `tests/app_smoke_tests.rs` contém testes dos três blocos.

É necessário comprovar a consistência dos estados:

- S1 = HEAD + A3.
- S2 = S1 + A1.
- S3 = S2 + A2.

Não presuma que a árvore final compilada prova que S1 e S2 compilam.

Investigue uma estratégia que:

1. Preserve integralmente a árvore de trabalho final.
2. Não modifique saves.
3. Não apague caches.
4. Não exija worktrees.
5. Não exija outro target.
6. Não utilize comandos destrutivos.
7. Permita verificar exatamente os conteúdos dos commits intermediários.

### Limite de autorização

Não substitua temporariamente os arquivos da árvore de trabalho para executar os testes sem minha aprovação específica.

Não crie commits antes de comprovar os estados intermediários.

Se não houver uma solução segura que respeite essas restrições, apresente:

- Estratégia recomendada.
- Comandos pretendidos.
- Riscos.
- Plano de recuperação.
- Arquivos envolvidos.
- Garantias de integridade.

**Nesse caso, interrompa a execução e aguarde autorização.**

Não trate a ausência de autorização como permissão implícita.

## 2.5. Execução dos commits

Somente se a validação anterior for concluída com sucesso e dentro das permissões:

1. Confira o conteúdo de cada commit.
2. Prepare exclusivamente os arquivos e trechos autorizados.
3. Confirme que os testes acompanham a correção correspondente.
4. Verifique que o índice representa exatamente o estado validado.
5. Crie o commit.
6. Registre o hash completo.
7. Repita para o próximo bloco.

Se qualquer etapa falhar, pare.

Não faça push.

## 2.6. Validação final

Confirme:

- `cargo fmt --check`.
- `cargo clippy --all-targets`.
- `cargo test`.
- `cargo check`.
- `git diff --check`.
- Autoplay isolado, se necessário e seguro.
- Integridade dos arquivos protegidos.
- Estado final do Git.

Não declare uma validação executada se apenas reutilizou resultados anteriores.

## 2.7. Portão de segurança

A Missão A somente estará concluída se:

1. Todos os commits autorizados forem criados corretamente.
2. Os estados intermediários forem validados.
3. Os testes finais passarem.
4. Nenhum arquivo protegido for modificado.
5. Não houver alterações inesperadas.
6. Não houver push.
7. A árvore de trabalho estiver no estado esperado.

Se qualquer condição falhar, não inicie a Missão B.

---

# PARTE 3 — MISSÃO B
# PLANEJAMENTO VISUAL DO PROTAGONISTA

## 3.1. Condição de início

Somente iniciar após a conclusão integral da Missão A.

Esta missão é exclusivamente de análise.

Não modificar código.

Não modificar assets.

Não criar animações.

Não criar novos sistemas ECS.

Não criar documentação no repositório.

Não criar commits.

O resultado deve ser apresentado no relatório final.

## 3.2. Objetivo visual

Planejar um protagonista original para Dark Castle RPG, com estilo:

**2D moderno, fantasia sombria, estética gótica e efeitos visuais cinematográficos.**

O jogo continuará sendo 2D.

O Blender será uma ferramenta de produção artística, não um substituto do Bevy.

O Bevy continuará responsável pela renderização, animações, câmera e integração com gameplay.

## 3.3. Auditoria do personagem atual

Investigue:

1. Spawn do jogador.
2. Componentes ECS.
3. Sprite atual.
4. Atualização visual.
5. Direção e orientação.
6. Movimento.
7. Salto e queda.
8. Ataque básico.
9. Crescent Slash.
10. Rising Edge.
11. Esquiva.
12. Guarda.
13. Dano e knockback.
14. Morte.
15. Respawn.
16. Magias.
17. Câmera.
18. Assets existentes.

Para cada item, apresentar:

- Arquivo e função.
- Implementação atual.
- Limitação visual.
- Ponto de integração futuro.
- Risco de regressão.

Distinguir claramente o que existe do que está apenas planejado.

## 3.4. Direção artística

Desenvolver três conceitos originais:

### Conceito A — Guerreiro gótico

Armadura parcial, tecidos pesados, aparência de combatente experiente.

### Conceito B — Espadachim aristocrático sombrio

Silhueta elegante, roupas detalhadas, espada refinada e movimento fluido.

### Conceito C — Caçador sobrenatural

Equipamentos de caça, detalhes místicos, aparência funcional e misteriosa.

Para cada conceito:

- Silhueta.
- Proporções.
- Paleta.
- Vestimenta.
- Arma.
- Animações características.
- Legibilidade.
- Complexidade de produção.
- Compatibilidade com Blender.
- Vantagens e desvantagens.

Recomende um conceito, mas não o torne definitivo sem aprovação.

Não copiar diretamente personagens ou sprites de jogos existentes.

## 3.5. Matriz completa de animações

Planejar as seguintes animações candidatas.

### Locomoção

- Idle
- Walk
- Run
- Turn
- JumpStart
- JumpRise
- JumpApex
- Fall
- Land

### Combate físico

- BasicAttack
- AttackRecovery
- CrescentSlash
- RisingEdge
- ComboTransition, se fizer sentido

### Defesa

- GuardStart
- GuardLoop
- GuardEnd
- Dodge
- Hurt
- Knockback

### Magia

- CastStart
- CastRelease
- CastRecovery
- ElementalCast
- SupportCast

### Estados especiais

- Death
- Dead
- Respawn
- Interact, se existir ou estiver documentado

### Animações secundárias

- Cabelo
- Capa
- Tecido
- Arma
- Rastros de golpe
- Respiração
- Partículas

Para cada animação, apresentar:

1. Prioridade.
2. Estado ou evento real correspondente.
3. Condição de início.
4. Condição de término.
5. Possibilidade de interrupção.
6. Loop ou execução única.
7. Direção visual.
8. Frames preliminares.
9. FPS preliminar.
10. Dependências.
11. Risco de dessincronização.
12. Testes futuros.

Não criar estados de gameplay apenas para satisfazer animações.

## 3.6. Arquitetura de animação Bevy

Princípio:

**O gameplay determina o acontecimento; a camada visual o representa.**

O sistema visual não pode decidir:

- Dano.
- Acerto.
- Morte.
- Custos.
- Cooldown.
- Validação de habilidades.
- Colisões.
- Criação de hitboxes.

Investigue a arquitetura real antes de recomendar novos componentes.

Avalie, se necessário:

- `AnimationController`
- `AnimationClipId`
- `AnimationPlayback`
- `VisualFacing`
- `AnimationTransition`
- `VisualEffectRequest`

Esses nomes são sugestões, não exigências.

Explique:

- Como o estado lógico seleciona a animação.
- Como interrupções são representadas.
- Como evitar ataques visuais após morte.
- Como sincronizar ataques e animações.
- Como manter o timing real do combate.
- Como preservar hitboxes.
- Como evitar atrasos de frame.
- Como lidar com assets ausentes.
- Como testar o sistema sem interface gráfica.
- Como separar entidade lógica e entidade visual.

Entregue um diagrama textual do fluxo ECS.

## 3.7. Pipeline Blender → Bevy

Comparar três abordagens:

### A. Pixel art manual

Sprites desenhados diretamente em 2D.

### B. Blender renderizado para 2D

Modelo 3D animado e exportado como quadros 2D.

### C. Pipeline híbrido

Blender para modelagem e animação-base, seguido de acabamento artístico em 2D.

Recomendar a abordagem mais adequada.

Para o pipeline escolhido, detalhar:

1. Criação do modelo ou sprite.
2. Rigging, se necessário.
3. Animação.
4. Câmera ortográfica, se usar Blender.
5. Iluminação de renderização.
6. Exportação PNG transparente.
7. Montagem de spritesheets.
8. Atlas e metadados.
9. Pivô consistente.
10. Escala.
11. Transparência.
12. Padding.
13. Importação no Bevy.
14. Seleção de animação.
15. Testes visuais.
16. Controle de versões dos assets.

Separar arquivos-fonte artísticos dos arquivos usados pelo jogo.

Não criar arquivos nesta execução.

## 3.8. Renderização e desempenho

Investigue a versão real do Bevy instalada.

Não use APIs de versões diferentes.

Planeje:

- Resolução lógica.
- Escala dos sprites.
- Filtros de textura.
- Texture atlases.
- Materiais.
- Transparência.
- Partículas.
- Brilho.
- Efeitos de ataque.
- Qualidade gráfica configurável.
- Compatibilidade Windows e Linux.
- Consumo de memória.
- Tempo de carregamento.
- FPS e frame time.

Não invente métricas.

Defina uma metodologia para medir desempenho futuramente.

## 3.9. Primeiro protótipo

Planejar uma vertical slice com:

1. Idle.
2. Movimento horizontal.
3. Salto.
4. Queda.
5. Ataque básico.
6. Crescent Slash.
7. Esquiva.
8. Hurt.
9. Morte.

Avaliar a inclusão de Rising Edge, Ember Bolt e Mending Light.

Preferir uma sala existente.

Não implementar novos mapas.

Critérios futuros:

- Animações correspondem ao gameplay.
- Interrupções funcionam visualmente.
- Não há ataque visual após morte.
- Hitboxes inalteradas.
- Física inalterada.
- MP e Hearts inalterados.
- Save/load compatível.
- Autoplay preservado.
- Fallback visual para assets ausentes.
- Desempenho aceitável após medição real.

## 3.10. Roadmap V1

Proponha etapas detalhadas:

- V1.1 — Direção artística.
- V1.2 — Contrato de assets.
- V1.3 — Controlador de animação.
- V1.4 — Idle e movimento.
- V1.5 — Salto e queda.
- V1.6 — Ataque básico.
- V1.7 — Técnicas físicas.
- V1.8 — Defesa, dano e morte.
- V1.9 — Magias.
- V1.10 — Animações secundárias.
- V1.11 — Otimização.
- V1.12 — Revisão final.

Para cada etapa, informar:

- Objetivo.
- Dependências.
- Arquivos prováveis.
- Responsável.
- Critérios de aceitação.
- Testes.
- Riscos.
- Sugestão de commit futuro.

## 3.11. Planejamento de agentes e skills

Avaliar a criação futura de:

- `visual-director`
- `animation-specialist`
- `rendering-specialist`
- `/art-pipeline`
- `/character-animation`
- `/visual-review`

Para cada proposta:

- Responsabilidade.
- Ferramentas.
- Limites.
- Arquivos permitidos.
- Arquivos protegidos.
- Critérios de entrega.
- Relação com agentes existentes.

Não criar esses agentes ou skills nesta execução.

## 3.12. Revisão cruzada

Após os especialistas concluírem:

1. O Game Architect verifica a compatibilidade com o gameplay.
2. O Rendering Specialist verifica a viabilidade no Bevy.
3. O Animation Specialist verifica transições e sincronização.
4. O Code Reviewer identifica riscos e inconsistências.
5. O coordenador consolida as divergências.

Não ocultar recomendações conflitantes.

---

# PARTE 4 — RELATÓRIO FINAL

Entregar um relatório único com duas partes.

## A. Consolidação da Fase 4

- Estado inicial.
- Estratégia de validação.
- Testes realizados.
- Commits criados.
- Hash de cada commit.
- Estado final do Git.
- Integridade dos arquivos protegidos.
- Pendências conhecidas.

## B. Planejamento visual V1

- Agentes utilizados.
- Auditoria do protagonista.
- Três direções artísticas.
- Recomendação.
- Matriz de animações.
- Arquitetura Bevy.
- Pipeline Blender.
- Estratégia de renderização.
- Primeiro protótipo.
- Roadmap detalhado.
- Testes futuros.
- Riscos.
- Decisões que exigem aprovação.

## C. Resumo executivo

Apresente:

1. O que foi concluído.
2. O que foi apenas planejado.
3. O que permanece pendente.
4. Quais decisões precisam da minha autorização.
5. Qual deve ser o próximo mega prompt.

---

# REGRA FINAL

Não misture implementação de combate com planejamento visual.

Não inicie a parte visual se a consolidação da Fase 4 falhar.

Não altere código ou assets durante o planejamento visual.

Não faça push.

Não inicie a Fase 5.

Não implemente a V1.

**Execute somente as ações expressamente autorizadas e pare diante de qualquer risco não coberto por estas instruções.**