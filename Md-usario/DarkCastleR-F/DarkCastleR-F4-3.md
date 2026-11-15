# DARK CASTLE RPG — FASE 4, ETAPA B
## Implementação multiagente das correções A3, A1 e A2

Leia integralmente:

- `Md-usario/DarkCastleR-F4.md`
- `Md-usario/DarkCastleR-F4-2.md`
- `CLAUDE.md`
- `docs/AUDITORIA_E2.md`
- `docs/DECISIONS.md`

Considere também o diagnóstico multiagente que você acabou de produzir.

**Autorizo a implementação das correções A3, A1 e A2, exclusivamente dentro do escopo da Fase 4.**

Não crie commits nesta execução.

## 1. Decisões aprovadas

### Q1 — CommandResolved

Preferência: opção B, emitir o resultado definitivo após a confirmação da ativação.

Antes de implementar, examine todos os produtores e consumidores de `CommandResolved`, incluindo o HUD e o autoplay.

Objetivo obrigatório:

- Uma habilidade rejeitada não pode ser anunciada como executada com sucesso.
- O motivo da rejeição deve permanecer observável.
- Não podem existir mensagens contraditórias para a mesma tentativa.
- O reconhecimento de comandos não pode ser prejudicado.
- Não altere o comportamento D-GAME-003 sem autorização.

Se a opção B exigir uma refatoração desproporcional ou alterar o significado de resultados existentes, interrompa apenas essa decisão, apresente as alternativas e solicite autorização.

Não escolha automaticamente a opção A para contornar o problema.

### Q2 — Pulo e habilidade restrita ao chão

Aprovado.

A validação deve respeitar o estado atual e `usable_in`.

Não executar habilidade nem seus efeitos quando o estado já não for permitido.

Preserve D-GAME-003 como pendência independente.

### Q3 — Quick load

Adiar a correção específica de quick load para a Fase 6.

Isso não elimina a obrigação de impedir que hitboxes de ações interrompidas sobrevivam à morte e ao respawn normal.

### Q4 — Mending Light

Manter a cura instantânea conforme a implementação atual.

Corrigir somente a documentação que descreva incorretamente o comportamento.

Não transformar a habilidade em magia com tempo de conjuração nesta fase.

### Q5 — Projéteis

Projéteis já lançados devem continuar existindo após o dono sofrer dano.

Projéteis ainda não criados por habilidades rejeitadas não devem surgir.

### Q6 — Arquitetura

Autorizo atualizar `.claude/rules/architecture.md` exclusivamente para registrar `SkillStarted`, caso essa mensagem seja implementada.

Não modifique outras regras ou skills sem autorização.

---

## 2. Organização dos agentes

Utilize os agentes existentes:

**game-architect**
- Revisar o plano e as dependências ECS.
- Conferir a ordem dos sistemas.
- Avaliar o fluxo de mensagens.
- Não editar código.

**combat-specialist**
- Implementar as correções de combate autorizadas.
- Trabalhar em um bloco por vez.
- Adicionar testes de regressão.
- Não alterar gameplay fora do escopo.

**code-reviewer**
- Revisar cada bloco depois da implementação.
- Verificar bugs, regressões e alterações não autorizadas.
- Examinar testes para detectar falsos positivos.
- Não editar código.

O agente principal deve coordenar as tarefas, integrar resultados e executar as validações.

Análises independentes podem ocorrer em paralelo.

**Não execute implementações simultâneas em arquivos compartilhados, nem compilações Cargo concorrentes.**

Não lance múltiplos agentes para modificar o mesmo módulo ao mesmo tempo.

---

## 3. Bloco A3 — Dano múltiplo

Comece por A3.

Investigue e corrija `player::systems::receive_damage`.

Requisitos:

1. Verificar a invulnerabilidade no momento de aplicar cada mensagem de dano.
2. Atualizar o estado de proteção após um dano válido.
3. Ignorar os eventos posteriores bloqueados.
4. Não consumir invulnerabilidade com Miss ou dano zero.
5. Não deixar mensagens de dano pendentes após a morte.
6. Impedir que mensagens antigas afetem o personagem após respawn.
7. Preservar fórmulas e durações atuais.

Antes de modificar o código, adicione testes que reproduzam os defeitos existentes, quando viável.

Depois implemente a menor correção possível.

### Testes A3

- Dois danos no mesmo frame.
- Três danos no mesmo frame.
- Invulnerabilidade já ativa.
- Miss seguido de hit.
- Dano zero seguido de hit.
- Hit seguido de Miss.
- Dano fatal seguido de outros eventos.
- Respawn sem reaplicação de mensagens antigas.
- Novo dano após o fim da invulnerabilidade.

Não altere a ordem de prioridade entre diferentes fontes de dano sem autorização.

Após implementar, execute os testes pertinentes.

Solicite revisão do code-reviewer e resolva os achados relevantes antes de avançar.

---

## 4. Bloco A1 — Ativação consistente

Depois de validar A3, implemente A1.

O diagnóstico identificou:

- `SkillActivated` usado como solicitação e como confirmação.
- Consumo de MP e Hearts potencialmente parcial.
- Cooldown iniciado antes da confirmação.
- Efeitos executados por consumidores independentes.
- Validação de estado desatualizada.

### Arquitetura esperada

Considere a separação:

`SkillActivated` → validação e confirmação → `SkillStarted` → execução dos efeitos.

A confirmação deve ocorrer em um único ponto.

Somente uma ativação confirmada pode:

- Consumir MP e Hearts.
- Iniciar cooldown.
- Aplicar lunge.
- Criar hitboxes.
- Criar projéteis.
- Aplicar cura.

Não modifique custos ou regras dos arquivos RON.

Preserve a execução no mesmo frame quando ela for válida.

Verifique explicitamente a ordem dos sistemas para evitar atrasos de um frame.

### Pagamento atômico

A cobrança de MP e Hearts não pode ser parcialmente aplicada.

Uma habilidade que exige ambos os recursos deve ser rejeitada integralmente se qualquer um deles for insuficiente.

Crie um teste ECS com uma habilidade definida somente para teste que tenha ambos os custos.

Não modifique arquivos RON para criar esse cenário.

### Testes A1

- Habilidade válida.
- Estado incompatível.
- Esquiva e magia no mesmo frame.
- Guarda e magia no mesmo frame, conforme as regras existentes.
- Pulo e habilidade restrita ao chão.
- Habilidade não aprendida.
- Cooldown ativo.
- MP insuficiente.
- Hearts insuficientes.
- MP e Hearts insuficientes em combinação.
- Habilidade inexistente.
- Duas ativações conflitantes.
- Nenhum efeito de habilidade rejeitada.
- Resultado correto no HUD e no fluxo de comandos.

Verifique também que D-GAME-003 não foi modificado acidentalmente.

Após implementar, execute testes pertinentes e solicite revisão do code-reviewer.

---

## 5. Bloco A2 — Interrupção de ataques

Somente depois de validar A1, implemente A2.

Corrija o problema das hitboxes vinculadas ao personagem que continuam ativas após dano ou morte.

Considere um sistema `cancel_interrupted_hitboxes`, mas confirme a ordem de execução antes de implementá-lo.

Requisitos:

1. Cancelar hitboxes vinculadas à ação interrompida.
2. Impedir dano ofensivo posterior ao cancelamento.
3. Impedir que hitboxes antigos acompanhem o jogador no respawn.
4. Não remover hitboxes de inimigos por engano.
5. Não remover projéteis já lançados.
6. Não cancelar ações por eventos de dano bloqueados.
7. Não alterar a mecânica de morte ou respawn além do necessário.

### Atenção à ordem ECS

O diagnóstico identificou que `Combat` roda antes de `Resolution`.

Um golpe que acertou no mesmo frame em que o jogador recebeu dano pode ser uma troca simultânea legítima.

Preserve esse comportamento, salvo se houver regra documentada em contrário.

Não crie cancelamentos retroativos de golpes já resolvidos.

### Testes A2

- Ataque interrompido por dano.
- Hitbox removida após interrupção.
- Morte durante Crescent Slash.
- Respawn sem hitbox antiga.
- Dano bloqueado não interrompe a ação.
- Personagem morto não inicia ataques.
- Projétil já lançado permanece ativo.
- Hitboxes de inimigos permanecem corretas.
- Ausência de hitboxes órfãs capazes de causar dano.

Após implementar, execute testes pertinentes e solicite revisão do code-reviewer.

---

## 6. Revisão de cada bloco

Ao concluir cada bloco, o code-reviewer deve analisar:

- Diff completo.
- Regras de gameplay preservadas.
- Ownership e borrowing.
- Ordem dos sistemas ECS.
- Mensagens produzidas e consumidas.
- Casos extremos.
- Testes adicionados.
- Possíveis regressões.

Cada achado deve incluir arquivo, linha, gravidade e cenário reproduzível.

Não faça alterações apenas por preferência estética.

Corrija problemas relevantes antes de iniciar o próximo bloco.

---

## 7. Validação final

Execute sequencialmente:

1. `cargo fmt --check`
2. `cargo clippy --all-targets`
3. `cargo test`
4. `cargo check`
5. Testes ECS específicos de A3, A1 e A2.
6. Autoplay isolado, quando o ambiente permitir.

Se for necessário executar `cargo fmt`, informe os arquivos modificados e confira se pertencem ao escopo.

Respeite as regras de compilação segura do `CLAUDE.md`.

Não use `/tmp` para compilar.

Não crie novos diretórios de build.

Não apague `target/verify/`.

Não execute compilações em paralelo.

Não execute `cargo clean`.

Antes e depois do autoplay, confira a integridade dos saves reais.

O autoplay deve continuar aprovando as 23 verificações obrigatórias, salvo mudança explicitamente autorizada que exija atualização legítima de uma verificação.

Nunca enfraqueça uma asserção para obter PASS.

---

## 8. Documentação

Atualize somente a documentação afetada pelas correções.

Documente:

- Fluxo de solicitação e confirmação das habilidades.
- Pagamento atômico de recursos.
- Cancelamento de hitboxes após interrupção.
- Invulnerabilidade aplicada por evento.
- Preservação de projéteis já lançados.
- Comportamento instantâneo atual de Mending Light.
- Decisões técnicas relevantes em `docs/DECISIONS.md`.

Se necessário, registre a decisão D-014.

Não altere arquivos de `Md-usario/`.

Não marque fases futuras como concluídas.

---

## 9. Segurança e Git

Antes de editar:

- Confirme o HEAD e o branch.
- Confirme que não existem alterações rastreadas inesperadas.
- Identifique os arquivos protegidos.
- Registre a situação inicial dos saves.

Durante a implementação:

- Preserve `Md-usario/`.
- Preserve os saves reais.
- Preserve `target/verify/`.
- Não altere arquivos RON.
- Não altere dependências.
- Não crie worktrees.
- Não faça push.
- Não execute comandos Git destrutivos.

**Não crie commits nesta execução.**

Os commits serão autorizados depois de uma revisão independente dos resultados.

---

## 10. Relatório final obrigatório

Apresente:

### A. Agentes

Quais agentes foram utilizados e o que cada um realmente executou.

### B. A3

Causa-raiz, solução, arquivos, testes e resultado.

### C. A1

Causa-raiz, solução, arquivos, testes e resultado.

### D. A2

Causa-raiz, solução, arquivos, testes e resultado.

### E. Revisões

Achados do code-reviewer e como foram resolvidos.

### F. Testes

Resultados reais de fmt, clippy, test, check e autoplay.

### G. Regressões

Informe se houve mudanças no HUD, comandos, ataques, magias, dano, morte ou respawn.

### H. Arquivos

Lista completa dos arquivos alterados e justificativas.

### I. Segurança

Integridade dos saves, briefings e diretórios protegidos.

### J. Git

Branch, HEAD, arquivos modificados, stage e arquivos não rastreados.

### K. Pendências

Questões que ainda dependem de decisão, sem resolvê-las silenciosamente.

---

## 11. Condição de parada

Se encontrar uma decisão de design não coberta pelas autorizações acima, pare antes de implementar a parte afetada.

Não altere o comportamento D-GAME-003.

Não implemente a Fase 5, 6 ou 7.

Não faça commits.

**Ao terminar a implementação e a validação, pare e aguarde minha revisão e autorização para consolidar os commits da Fase 4.**