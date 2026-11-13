# DARK CASTLE RPG — FASE 4
## Estabilização crítica do combate: A1, A2 e A3

Você está trabalhando no projeto Dark Castle RPG, desenvolvido em Rust 2024 com Bevy 0.20.1.

As Fases 1, 2 e 3 foram concluídas. A Fase 3 consolidou o autoplay com isolamento de saves, 23 verificações obrigatórias aprovadas, 86 testes Rust e 125 testes do hook de segurança.

**Autorizo exclusivamente a Fase 4, dedicada às correções A1, A2 e A3 da auditoria técnica.**

Não implemente funcionalidades novas, não altere balanceamento e não inicie as Fases 5, 6 ou 7.

---

# 1. Objetivo

Corrigir três defeitos críticos:

**A1 — Ativação inconsistente de habilidades**

Uma habilidade não pode consumir recursos, iniciar cooldown, movimentar o personagem ou produzir efeitos se a transição necessária para executá-la falhar.

**A2 — Ações ofensivas persistem após interrupção**

Quando o jogador sofre uma interrupção válida ou morre, ataques e efeitos ofensivos associados à ação interrompida não podem continuar causando dano indevidamente.

**A3 — Dano múltiplo ignora invulnerabilidade**

Múltiplos eventos de dano recebidos no mesmo frame devem respeitar a invulnerabilidade e não podem descontar HP repetidamente quando essa proteção já foi ativada.

---

# 2. Inspeção obrigatória antes de editar

Leia integralmente:

- `CLAUDE.md`
- `docs/AUDITORIA_E2.md`
- `docs/ARCHITECTURE.md`
- `docs/COMBAT_SYSTEM.md`
- `docs/CHARACTER_ATTRIBUTES.md`
- `docs/DECISIONS.md`
- `src/skills/systems.rs`
- `src/skills/execution.rs`
- `src/skills/definitions.rs`
- `src/combat/command_input.rs`
- `src/combat/attacks.rs`
- `src/combat/hitboxes.rs`
- `src/combat/damage.rs`
- `src/magic/casting.rs`
- `src/player/state.rs`
- `src/player/systems.rs`
- `src/player/components.rs`
- `src/app/plugins.rs`
- `src/core/events.rs`

Leia também os testes relevantes e os arquivos RON de habilidades.

Não assuma que a ordem dos sistemas é a ordem visual do código. Confirme explicitamente os schedules, conjuntos de sistemas, dependências `.before()` e `.after()` e pontos em que as mensagens são produzidas e consumidas.

Se houver outros módulos envolvidos, inspecione-os antes de alterar a implementação.

---

# 3. Etapa A — Diagnóstico sem alterações

Antes de modificar o código, produza um diagnóstico objetivo de A1, A2 e A3.

Para cada problema, informe:

1. Causa-raiz.
2. Arquivos e funções envolvidos.
3. Ordem de execução relevante.
4. Como reproduzir.
5. Comportamento esperado.
6. Riscos da correção.
7. Testes de regressão necessários.

Investigue especialmente:

- Se `SkillActivated` representa uma solicitação ou uma ativação já confirmada.
- Se vários sistemas consumidores executam efeitos independentemente do sucesso de `begin_skill`.
- Se o reconhecimento do comando e a ativação utilizam estados diferentes no mesmo frame.
- Se `start_action` pode falhar depois de a habilidade ter sido validada.
- Se os custos de MP e Hearts são realmente atômicos.
- Se hitboxes ativos sobrevivem a interrupções.
- Se projéteis já lançados devem continuar existindo após o jogador sofrer dano.
- Se o estado `Dead` impede novas ativações.
- Se `Invulnerable` é verificado novamente quando o dano é efetivamente aplicado.

**Não comece a implementação antes de concluir esse diagnóstico.**

---

# 4. Etapa B — Correção A1

## Problema confirmado

Em `src/skills/systems.rs`, `begin_skill` atualmente tenta descontar recursos e iniciar cooldown antes de verificar o sucesso de `start_action(PlayerState::Casting, ...)`.

Além disso, outros sistemas recebem `SkillActivated` para executar ataques e magias.

## Comportamento obrigatório

Uma habilidade somente deve produzir seus efeitos quando sua ativação tiver sido confirmada.

A operação deve ser consistente:

1. Validar existência da habilidade.
2. Validar estado atual.
3. Validar aprendizado, cooldown e recursos, conforme as responsabilidades existentes.
4. Confirmar que a transição de estado pode ocorrer.
5. Aplicar custos e cooldown de forma consistente.
6. Permitir os efeitos somente após a ativação bem-sucedida.

Não basta adicionar um `return` depois de `start_action` se outros sistemas já receberam a mesma mensagem e ainda puderem executar os efeitos.

Avalie a necessidade de separar uma solicitação de ativação de uma ativação confirmada, mas não crie eventos ou abstrações extras sem necessidade.

A solução deve respeitar a arquitetura existente e evitar que o reconhecimento de comandos, os sistemas de habilidades e os efeitos tenham interpretações divergentes da mesma ativação.

## Testes obrigatórios

Cubra pelo menos:

- Ativação válida.
- Estado que rejeita a transição.
- Recursos insuficientes.
- Cooldown ativo.
- Habilidade não aprendida.
- Habilidade inexistente.
- Duas solicitações conflitantes no mesmo frame.
- MP e Hearts não podem ser parcialmente consumidos.
- Rejeição não pode iniciar cooldown.
- Rejeição não pode criar hitbox ou projétil.
- Rejeição não pode aplicar cura.
- Rejeição não pode alterar velocidade por efeito da habilidade.

Use os dados reais das habilidades quando adequado.

---

# 5. Etapa C — Correção A2

## Problema

Ações ofensivas podem continuar ativas depois que o jogador sofre dano ou morre.

## Comportamento obrigatório

Quando uma ação é efetivamente interrompida:

- O estado do jogador deve refletir a interrupção.
- Hitboxes vinculados à ação interrompida não podem continuar acertando.
- Uma magia ainda não efetivada não pode ser concluída depois da interrupção.
- A morte deve impedir novas ações.
- O respawn não deve reativar ataques antigos.

Diferencie cuidadosamente:

- Ataques ainda em preparação.
- Hitboxes ativos de golpes corpo a corpo.
- Magias ainda em execução.
- Projéteis que já foram efetivamente lançados.
- Efeitos instantâneos que já aconteceram.

**Não presuma que um projétil já lançado deve desaparecer quando o personagem sofre dano.** Consulte a documentação e preserve o comportamento atual quando não houver uma regra aprovada para modificá-lo.

Da mesma forma, não altere o momento em que Mending Light aplica cura sem identificar se isso é um defeito de implementação ou uma decisão de design ainda pendente.

Se alguma decisão de gameplay for indispensável, registre-a e solicite autorização, em vez de inventar uma regra.

## Testes obrigatórios

- Ataque interrompido por dano válido.
- Hitbox ofensivo não continua acertando depois da interrupção.
- Morte durante uma ação ofensiva.
- Nenhuma nova ação pode iniciar durante `Dead`.
- Respawn não restaura ataques antigos.
- Um dano bloqueado por invulnerabilidade não interrompe indevidamente a ação.
- Não existem entidades ou efeitos órfãos capazes de causar dano indevido.

Verifique também a ordem entre aplicação de dano, cancelamento de ações, detecção de colisões e criação de efeitos.

---

# 6. Etapa D — Correção A3

## Problema

Vários eventos `DamageDealt` podem ser produzidos antes de `receive_damage` atualizar a invulnerabilidade.

O jogador pode receber múltiplos descontos de HP no mesmo frame.

## Comportamento obrigatório

- Um primeiro dano válido pode aplicar HP, knockback e invulnerabilidade.
- Eventos posteriores devem respeitar a invulnerabilidade ativada.
- Um `Miss` não deve consumir invulnerabilidade.
- Um golpe que cause zero dano não deve consumir invulnerabilidade, salvo regra já documentada.
- Um personagem morto não deve receber novos descontos.
- A lógica deve funcionar independentemente da quantidade de eventos na fila.

Não resolva o problema apenas verificando a invulnerabilidade na geração do evento.

A aplicação do dano precisa verificar o estado atual, inclusive alterações provocadas por eventos anteriores do mesmo frame.

## Testes obrigatórios

- Dois hits válidos no mesmo frame.
- Três ou mais hits no mesmo frame.
- `Miss` seguido de hit válido.
- Hit de dano zero seguido de hit válido.
- Hit válido seguido de `Miss`.
- Invulnerabilidade já ativa antes do frame.
- Dano fatal seguido de outros eventos.
- Novo dano depois que a invulnerabilidade expirar.

Não altere os valores de duração da invulnerabilidade nem os cálculos de dano sem necessidade.

---

# 7. Regras reais das habilidades

Preserve os dados existentes.

**Crescent Slash**
- 0 MP e 0 Hearts.
- Estados permitidos: `Grounded`, `Attacking`.
- Ataque físico com avanço.

**Rising Edge**
- Custo de 2 Hearts.
- Estados permitidos: `Grounded`, `Airborne`, `Attacking`.
- Desbloqueio por requisitos a partir do nível 3.

**Ember Bolt**
- Custo de 8 MP.
- Estados permitidos: `Grounded`, `Airborne`.
- Projétil de fogo.

**Mending Light**
- Custo de 15 MP.
- Estado permitido: `Grounded`.
- Cura de 30 HP.

Esses valores vêm dos arquivos RON existentes.

Não altere os custos, cooldowns, danos, animações, requisitos ou estados permitidos para fazer os testes passarem.

---

# 8. Decisões que NÃO estão autorizadas

A Fase 3 identificou outras pendências:

**D-GAME-003:** um comando especial rejeitado pode disparar um ataque básico.

**Mending Light:** o autoplay ainda não exercita uma cura real porque o personagem está com HP cheio.

**M17:** a física depende do intervalo entre frames.

Essas pendências não fazem parte das correções obrigatórias A1–A3.

Não modifique o comportamento de D-GAME-003 sem uma decisão explícita.

Não refatore a física nesta fase.

Pode adicionar testes pertinentes ao combate e melhorar diagnósticos, mas não transforme essas pendências em mudanças de gameplay não autorizadas.

---

# 9. Estratégia de implementação

Trabalhe em três blocos independentes:

**Bloco 1 — A1**

Corrija a ativação de habilidades e adicione testes.

**Bloco 2 — A2**

Corrija interrupção e cancelamento de ações ofensivas e adicione testes.

**Bloco 3 — A3**

Corrija dano múltiplo e adicione testes.

Após cada bloco:

1. Confira o diff.
2. Execute testes pertinentes.
3. Verifique regressões.
4. Informe alterações realizadas.
5. Preserve as correções anteriores.

Evite refatorações amplas e mudanças de arquitetura não justificadas.

Se a solução de um bloco depender de uma alteração estrutural relevante, explique a necessidade antes de implementá-la.

---

# 10. Validação final

Depois das correções, execute sequencialmente, sem compilações paralelas:

- `cargo fmt --check`
- `cargo check`
- `cargo clippy --all-targets`
- `cargo test`
- Testes específicos de regressão A1, A2 e A3.
- `cargo run --example autoplay`, somente com o isolamento de saves ativo.

O autoplay não substitui os testes específicos dos defeitos corrigidos.

Antes de executá-lo, confirme os checksums dos saves reais, memória disponível e ausência de outras compilações.

Depois, confirme novamente os checksums.

Se alguma verificação falhar, não declare a fase concluída.

Não ajuste os testes para esconder um defeito real.

---

# 11. Restrições de ambiente e Git

- Use exclusivamente o `target/` da raiz.
- Respeite o limite de quatro jobs do Cargo.
- Nunca compile em `/tmp`.
- Não execute `cargo clean`.
- Não apague `target/verify/`.
- Não crie worktrees.
- Não instale dependências sem autorização.
- Não modifique `Md-usario/`.
- Não modifique os saves reais.
- Não altere arquivos RON de balanceamento.
- Não faça push.
- Não execute as Fases 5, 6 ou 7.
- Não crie commits nesta execução.

Preserve todas as alterações anteriores.

Se encontrar alterações preexistentes inesperadas, pare e informe antes de prosseguir.

---

# 12. Relatório final obrigatório

Ao terminar, apresente:

## A. Diagnóstico

Causa-raiz de A1, A2 e A3, incluindo funções e arquivos.

## B. Implementação

Arquivos modificados e justificativa de cada alteração.

## C. Testes

Quantidade de testes anteriores, testes novos, total aprovado e testes que falharam.

## D. Comportamento

Demonstre que:

- Habilidades rejeitadas não produzem efeitos.
- Custos e cooldowns são consistentes.
- Ações interrompidas não deixam hitboxes ofensivos indevidos.
- A morte impede ataques posteriores.
- Múltiplos danos respeitam invulnerabilidade.

## E. Autoplay

Informe código de saída, verificações aprovadas, eventuais regressões e integridade dos saves.

## F. Segurança

Confirme que `Md-usario/`, saves reais e `target/verify/` permaneceram intactos.

## G. Git

Informe branch, HEAD, arquivos modificados, arquivos não rastreados e estado do stage.

## H. Pendências

Liste qualquer questão de design ou defeito fora do escopo, sem implementá-lo.

---

# 13. Critério de encerramento

A Fase 4 só poderá ser considerada pronta para consolidação quando A1, A2 e A3 estiverem corrigidos e cobertos por testes que demonstrem os comportamentos esperados.

**Não crie commits automaticamente.**

Quando concluir a implementação e as validações, pare e aguarde minha autorização para revisar e consolidar os commits da Fase 4.