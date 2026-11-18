# DARK CASTLE RPG — MEGA PROMPT D21
## Primeiro Chefe Funcional, Arena, Fases, Combate, Recompensas e Persistência

**Projeto:** Dark Castle RPG  
**Engine:** Bevy 0.20.1  
**Linguagem:** Rust 2024  
**Etapa:** D21  
**Objetivo:** Implementar o primeiro chefe completo e jogável  
**Prioridade:** Gameplay, estabilidade e testes reais  
**Arte definitiva:** FUTURO — NÃO EXECUTAR AGORA

---

# 0. MISSÃO

Implementar o primeiro encontro de chefe do Dark Castle RPG, aproveitando os sistemas já existentes de inimigos, combate, salas, progressão, equipamentos, inventário, salvamento e interfaces.

A D20.1 confirmou, segundo seu relatório, que o jogo pode ser executado com janela gráfica real, com 23/23 cenários de autoplay aprovados, 15 capturas e testes integrados das etapas D17–D20.

Agora precisamos de um chefe que ofereça uma experiência completa:

1. Entrada em uma arena.
2. Início controlado do encontro.
3. Bloqueio temporário das saídas.
4. Chefe com HP próprio.
5. Ataques distintos e legíveis.
6. Mudança de comportamento por porcentagem de HP.
7. Reação a ataques físicos e magias.
8. Vitória e derrota do jogador.
9. Recompensa concedida uma única vez.
10. Desbloqueio das saídas após a vitória.
11. Persistência da derrota do chefe no Save V3.
12. Retorno à região sem reaparecimento indevido do chefe.

**O objetivo não é criar apenas um inimigo com mais HP. Queremos um encontro de chefe completo.**

---

# 1. ESTADO INICIAL E SEGURANÇA

Antes de implementar:

- Confirmar branch e HEAD.
- Executar `git status --short`.
- Identificar arquivos modificados e não rastreados.
- Preservar integralmente D19, D20 e D20.1.
- Verificar se existem alterações do proprietário em andamento.
- Não sobrescrever arquivos preexistentes.

Último estado informado:

- Branch: `art/v1.2-protagonist`.
- HEAD: `a9b8f96`.
- Working tree com alterações locais D19/D20/D20.1.

Não presumir que esse estado continua igual.

## Proibições

Não executar comandos Git destrutivos.

Não fazer commit ou push.

Não criar branch.

Não realizar merge ou rebase.

Não apagar evidências ou documentos anteriores.

Não modificar saves reais.

Não alterar arquivos protegidos do proprietário.

---

# 2. AUDITORIA OBRIGATÓRIA

Ler os documentos reais:

- `docs/development/d20/D20_PREPARACAO_CHEFES.md`
- `docs/development/d20_1/D20_1_PREPARACAO_D21.md`
- `docs/development/d20_1/D20_1_DEFEITOS.md`
- `docs/development/d20_1/D20_1_RELATORIO_FINAL.md`

Inspecionar também:

- Sistema de inimigos.
- `AiProfile` e `AiStyle`.
- Dados RON.
- Hitboxes e hurtboxes.
- Dano e resistências.
- Progressão.
- Drops.
- Salas.
- Portas e transições.
- `TransitionLock`.
- Save V3.
- HUD.
- GameState.
- GameSet.
- Sistema de eventos/mensagens.
- Testes ECS.
- Exemplos autoplay e D20.1.

**Não inventar estruturas que já existam.**

Registrar o que pode ser reutilizado e o que realmente precisa ser implementado.

---

# 3. ESCOPO DO PRIMEIRO CHEFE

Criar um chefe provisório chamado:

**The Ashen Sentinel — Sentinela das Cinzas**

Esse nome é provisório e pode ser adaptado ao vocabulário do projeto.

Conceito:

Um antigo guardião de uma fortaleza esquecida, responsável por impedir o avanço do protagonista.

Características desejadas:

- Humanoide ou criatura terrestre.
- Combate principalmente corpo a corpo.
- Um ataque de longo alcance ou de área.
- Duas fases.
- Movimentos claros.
- Intervalos de recuperação.
- Resistência superior aos inimigos comuns.
- Derrota persistente.

Não criar modelo 3D.

Não gerar sprite definitivo.

Usar placeholders compatíveis com a apresentação atual.

---

# 4. ARQUITETURA DE CHEFES

Antes de criar novos componentes, verificar se a arquitetura atual já possui mecanismos equivalentes.

Preferir uma extensão pequena e reutilizável.

Conceitos necessários:

- Identificador estável do chefe.
- Definição de chefe em dados.
- HP máximo.
- HP atual.
- Estado do encontro.
- Fase atual.
- Conjunto de ataques.
- Condições de mudança de fase.
- Recompensas.
- Estado de derrota persistente.

Não criar uma segunda implementação de dano.

Não duplicar sistemas de morte.

Não criar um segundo gerenciador de salas.

Não substituir a IA existente indiscriminadamente.

O chefe deve continuar utilizando o pipeline central de combate sempre que possível.

---

# 5. DADOS RON

Criar ou estender definições orientadas a dados.

Os dados devem permitir configurar:

- ID.
- Nome.
- Sala.
- HP.
- Dano.
- Defesa.
- Velocidade.
- Resistências, quando existentes.
- Ataques.
- Cooldowns.
- Limiares de fase.
- Recompensas.
- Estado persistente.

Exemplo conceitual:

```text
boss_id: ashen_sentinel
display_name: The Ashen Sentinel
phase_threshold: 0.50
```

Não copiar literalmente se o formato do projeto for diferente.

Validar IDs duplicados, ataques inexistentes, limiares inválidos e referências quebradas.

---

# 6. ARENA

Selecionar uma sala existente adequada ou criar uma sala pequena e específica, conforme a arquitetura e o conteúdo atual.

Preferir uma arena:

- Plana.
- Com chão sólido.
- Sem plataformas complexas.
- Com espaço para esquiva.
- Com limites definidos.
- Com entrada e saída.
- Sem sobreposição com inimigos comuns.
- Com câmera funcional.

A D20.1 registrou um problema P3 relacionado à plataforma do Bone Bolt.

Não reproduzir esse problema na arena.

Não construir uma região inteira.

Não modificar áreas existentes sem necessidade.

---

# 7. INÍCIO DO ENCONTRO

O encontro deve começar de forma previsível.

Fluxo desejado:

1. Jogador entra na arena.
2. Sistema verifica se o chefe já foi derrotado.
3. Se não foi, inicia o encontro.
4. Chefe é ativado.
5. Saídas são bloqueadas.
6. Barra de HP aparece.
7. IA inicia o combate.

Evitar ativação repetida.

Evitar spawn duplicado.

Não bloquear permanentemente a sala se ocorrer uma falha.

---

# 8. BLOQUEIO DA ARENA

Reutilizar o sistema de portas e `TransitionLock`, quando adequado.

Regras:

- Saídas bloqueadas durante combate ativo.
- Saídas liberadas após vitória.
- Estado restaurado após derrota do jogador.
- Encontro pode ser reiniciado conforme política definida.
- Save carregado não deve deixar arena travada incorretamente.
- Chefe já derrotado não deve bloquear a arena novamente.

Não introduzir bloqueios permanentes sem recuperação.

---

# 9. MÁQUINA DE ESTADOS DO CHEFE

Considerar:

- Dormant.
- Intro.
- Idle.
- Approach.
- PrepareAttack.
- Attack.
- Recover.
- PhaseTransition.
- Stunned, se suportado.
- Defeated.

Adaptar aos estados reais existentes.

Regras:

- Não atacar enquanto morto.
- Não atacar durante transição de fase.
- Respeitar cooldowns.
- Não executar múltiplos ataques incompatíveis.
- Não entrar em loop infinito.
- Não ultrapassar limites da arena.
- Não continuar perseguindo o jogador fora do encontro.

---

# 10. FASE 1 — 100% A 50% HP

A primeira fase deve ensinar o jogador a reconhecer os ataques.

Implementar pelo menos dois padrões distintos.

## Ataque A — Golpe frontal

- Preparação perceptível.
- Hitbox frontal.
- Dano físico.
- Alcance definido.
- Recuperação após execução.

## Ataque B — Avanço ofensivo

- Preparação.
- Deslocamento limitado.
- Ataque frontal.
- Recuperação.
- Respeito à colisão.

Os dois ataques precisam ser funcionalmente distintos.

Não usar apenas diferenças numéricas de dano.

---

# 11. FASE 2 — ABAIXO DE 50% HP

Ao atingir o limiar configurado:

1. Finalizar ou cancelar a ação atual com segurança.
2. Entrar em transição de fase.
3. Aplicar feedback provisório.
4. Atualizar a fase.
5. Alterar seleção de ataques.
6. Retomar o combate.

A fase 2 deve apresentar:

- Maior pressão ofensiva.
- Pelo menos um novo padrão.
- Cooldowns ainda legíveis.
- Oportunidades reais de reação.

## Ataque C — Onda de cinzas

Criar um ataque à distância ou de área reutilizando projéteis e colisões existentes.

Verificar:

- Direção.
- Alcance.
- Colisão com sólidos.
- Dano.
- Tempo de vida.
- Limpeza ao encerrar o encontro.

Não disparar através de paredes sólidas.

Não criar uma nova infraestrutura de projéteis.

---

# 12. LEGIBILIDADE DO COMBATE

O jogador precisa reconhecer:

- Preparação.
- Ataque ativo.
- Recuperação.
- Mudança de fase.
- Dano recebido pelo chefe.
- Derrota.

Utilizar recursos visuais provisórios já existentes.

Pode empregar:

- Mudanças de cor.
- Pequenos indicadores.
- Texto de estado.
- Feedback de dano.

Não criar arte definitiva.

Não adicionar efeitos gráficos pesados.

---

# 13. BARRA DE HP DO CHEFE

Adicionar interface específica para o encontro.

Mostrar:

- Nome do chefe.
- HP atual.
- HP máximo ou proporção.
- Mudança de fase, quando apropriado.

A barra deve:

- Aparecer ao iniciar o encontro.
- Atualizar ao receber dano.
- Desaparecer após vitória.
- Desaparecer ao sair do encontro.
- Não duplicar.
- Não permanecer após carregar um save fora da arena.

Reutilizar tema e componentes de UI existentes.

---

# 14. DANO E COMBATE

Integrar com:

- Ataque básico.
- Técnicas especiais.
- Magias.
- Hitboxes.
- Hurtboxes.
- Defesa.
- Invulnerabilidade.
- Knockback.
- Recursos HP/MP/Hearts.

Verificar que:

- O chefe recebe dano corretamente.
- O jogador recebe dano corretamente.
- Ataques não causam dano duplicado por erro de evento.
- A fase muda apenas quando necessário.
- A morte acontece uma única vez.
- Projéteis não permanecem após o encontro.

Não modificar fórmulas globais de dano sem justificativa.

---

# 15. DERROTA DO JOGADOR

Se o jogador morrer durante o encontro:

- Encerrar o encontro.
- Limpar projéteis temporários.
- Restaurar o estado da arena.
- Permitir respawn normal.
- Não registrar vitória.
- Não conceder recompensas.
- Não manter a barra de chefe indevidamente.

Ao retornar à arena, o encontro deve funcionar conforme a política escolhida.

Documentar se o HP do chefe reinicia.

Preferência inicial: reiniciar o chefe com HP completo após uma nova tentativa.

---

# 16. VITÓRIA

Ao derrotar o chefe:

1. Registrar a morte.
2. Encerrar ataques.
3. Remover projéteis temporários.
4. Liberar portas.
5. Atualizar interface.
6. Conceder recompensas.
7. Registrar progresso persistente.
8. Impedir duplicação de recompensas.
9. Permitir continuar explorando.

A vitória deve ser idempotente.

Eventos duplicados de morte não podem gerar recompensas extras.

---

# 17. PERSISTÊNCIA NO SAVE V3

Auditar o formato atual antes de alterar.

Verificar se já existe estrutura de progresso adequada para registrar chefes derrotados.

Preferir reutilizar o progresso existente.

Se forem necessários novos campos:

- Avaliar compatibilidade com saves anteriores.
- Utilizar defaults quando apropriado.
- Não alterar versão silenciosamente.
- Documentar qualquer migração necessária.
- Criar testes de compatibilidade.

**Preservar Save V3 sempre que tecnicamente possível.**

Não prometer ausência de migração sem verificar a estrutura real.

Testar:

- Save antes da vitória.
- Save depois da vitória.
- Carregamento após vitória.
- Retorno à arena.
- Chefe não reaparece indevidamente.
- Recompensa não é duplicada.
- Portas permanecem coerentes.

Utilizar saves isolados.

Não tocar em saves reais.

---

# 18. RECOMPENSAS

O chefe deve conceder uma recompensa significativa.

Priorizar mecanismos já existentes:

- EXP.
- Item.
- Equipamento.
- Desbloqueio de progressão.

Escolher uma recompensa compatível com o balanceamento atual.

Não criar um sistema de itens lendários inteiro.

Não conceder recompensas repetidas.

Documentar exatamente o que é recebido.

---

# 19. BALANCEAMENTO

Avaliar os atributos reais do jogador e os inimigos existentes.

O chefe deve ser desafiador, mas derrotável.

Considerar:

- HP do protagonista.
- Dano básico.
- Técnicas.
- Magias.
- Consumíveis.
- Esquiva.
- Guarda.
- Tempo de recuperação.
- Duração estimada do encontro.

Não escolher números arbitrários sem comparar com o gameplay existente.

Registrar a justificativa dos valores iniciais.

Não realizar rebalanceamento global do jogo.

---

# 20. TESTES UNITÁRIOS

Criar testes para:

- Inicialização do encontro.
- Chefe já derrotado.
- Bloqueio da arena.
- Seleção de ataques.
- Cooldowns.
- Preparação.
- Recuperação.
- Mudança de fase.
- Limiar de HP.
- Morte.
- Vitória.
- Derrota do jogador.
- Recompensa única.
- Persistência.
- Carregamento.
- Limpeza de projéteis.
- Ausência de duplicação.

Testar limites exatos de HP.

---

# 21. TESTES DE INTEGRAÇÃO ECS

Criar cenário headless que:

1. Carrega arena.
2. Instancia jogador e chefe.
3. Inicia encontro.
4. Bloqueia saídas.
5. Executa fase 1.
6. Aplica dano ao chefe.
7. Aciona fase 2.
8. Executa ataque da fase 2.
9. Derrota o chefe.
10. Concede recompensa.
11. Libera saídas.
12. Registra progresso.
13. Salva.
14. Carrega.
15. Verifica ausência de respawn do chefe.
16. Verifica ausência de recompensa duplicada.

Criar também cenário de derrota do jogador.

Não declarar integração concluída apenas com testes de funções puras.

---

# 22. TESTE REAL COM JANELA

A D20.1 estabeleceu um precedente importante.

**A D21 deve tentar executar o jogo graficamente.**

Reutilizar:

- `cargo run`
- `cargo run --example autoplay`
- `cargo run --example d20_1_probe`
- Ferramentas de captura existentes.

Adaptar o tooling somente se necessário.

Não transformar o teste visual em uma nova infraestrutura complexa.

## Evidências desejadas

- Arena antes do encontro.
- Chefe em fase 1.
- Ataque frontal.
- Ataque de avanço.
- Transição para fase 2.
- Ataque à distância.
- Barra de HP.
- Vitória.
- Arena desbloqueada.
- Retorno após carregar save.

As capturas devem ser reais.

Não usar imagens geradas por IA.

Não afirmar que testes visuais passaram se não foram executados.

---

# 23. REGRESSÃO D17–D20.1

Verificar:

## D17
- Altares.
- Descanso.
- Salvamento.

## D18
- Consumíveis.
- Inventário.
- HP/MP.

## D19
- Viagem rápida.
- Transições.
- Estado de sala.

## D20
- Bone Sentry.
- Bone Bolt.
- Ash Warden.
- EXP e drops.

## D20.1
- Autoplay.
- Probe.
- Testes gráficos existentes.
- Ausência de novos P0/P1.

Não alterar comportamentos existentes sem necessidade.

---

# 24. QUALIDADE

Executar sequencialmente:

```bash
cargo fmt --check
CARGO_BUILD_JOBS=2 cargo check
CARGO_BUILD_JOBS=2 cargo clippy --all-targets -- -D warnings
CARGO_BUILD_JOBS=2 cargo test
```

Depois executar os exemplos relevantes, conforme viabilidade do ambiente.

Não iniciar builds pesados em paralelo.

Registrar falhas, tempos e limitações.

---

# 25. DOCUMENTAÇÃO

Criar:

`docs/development/d21/`

Arquivos sugeridos:

- `D21_AUDITORIA.md`
- `D21_DESIGN_CHEFE.md`
- `D21_IMPLEMENTACAO.md`
- `D21_ARENA.md`
- `D21_PERSISTENCIA.md`
- `D21_TESTES.md`
- `D21_RELATORIO_FINAL.md`

Criar evidências em:

`docs/development/d21/evidence/`

Atualizar roadmap e changelog conforme convenções reais.

Preservar documentos anteriores.

---

# 26. ARTE DEFINITIVA E MIXAMO — FUTURO

**NÃO EXECUTAR NA D21.**

Preservar:

`Md-usario/assets-user/prota/base/white_mesh.glb`

O modelo continua sendo um protótipo 3D para uso futuro.

Pipeline planejado:

1. Referências visuais aprovadas pelo proprietário.
2. Geração de geometria com Hunyuan3D.
3. Preservação do GLB original.
4. Preparação de cópia no Blender.
5. Conversão para formato compatível com Mixamo.
6. Auto-rigging, quando possível.
7. Seleção e obtenção de animações.
8. Ajustes de rigging no Blender.
9. Renderização em sprites 2D.
10. Integração no Bevy.
11. Testes visuais e aprovação.

Aplicar futuramente ao protagonista, inimigos, NPCs e chefes.

Não abrir Blender.

Não iniciar Hunyuan.

Não baixar animações.

Não modificar o GLB.

Não substituir a arte atual.

---

# 27. CRITÉRIOS DE ACEITAÇÃO

## Chefe

- [ ] Definição orientada a dados.
- [ ] Arena funcional.
- [ ] Início do encontro.
- [ ] Bloqueio de saídas.
- [ ] HP próprio.
- [ ] Barra de HP.
- [ ] Dois ataques na fase 1.
- [ ] Transição de fase por HP.
- [ ] Ataque adicional na fase 2.
- [ ] Cooldowns.
- [ ] Preparação e recuperação.
- [ ] Dano integrado.
- [ ] Vitória.
- [ ] Derrota do jogador.
- [ ] Recompensa única.
- [ ] Derrota persistente.
- [ ] Retorno seguro à arena.

## Testes

- [ ] Unitários.
- [ ] Integração ECS.
- [ ] Persistência.
- [ ] Regressão D17–D20.1.
- [ ] Cargo fmt.
- [ ] Cargo check.
- [ ] Clippy.
- [ ] Cargo test.
- [ ] Execução gráfica ou bloqueio documentado.
- [ ] Evidências reais, quando possível.

## Segurança

- [ ] Sem commit.
- [ ] Sem push.
- [ ] Saves reais preservados.
- [ ] Arte original preservada.
- [ ] GLB preservado.
- [ ] Sem Hunyuan.
- [ ] Sem Blender.
- [ ] Sem Mixamo.

---

# 28. RELATÓRIO FINAL OBRIGATÓRIO

Ao terminar, apresentar:

## A. Estado inicial
Branch, HEAD e working tree.

## B. Auditoria
Arquitetura existente e sistemas reutilizados.

## C. Chefe
Nome, atributos, localização e definição.

## D. Arena
Entrada, bloqueio e liberação.

## E. Combate
Ataques, cooldowns, hitboxes e recuperação.

## F. Fases
Condições e mudanças de comportamento.

## G. Persistência
Save V3, derrota e recompensas.

## H. Testes automatizados
Comandos, resultados e falhas.

## I. Testes gráficos
Cenários, capturas e observações.

## J. Regressões
D17–D20.1.

## K. Segurança
Arquivos protegidos e Git.

## L. Limitações
Problemas conhecidos e riscos.

## M. Próxima etapa
Recomendação técnica para D22.

---

# 29. DIRETRIZ FINAL

A D21 deve transformar o sistema atual de combate em uma experiência de chefe completa.

Não criar apenas um inimigo com HP elevado.

O chefe precisa possuir:

- Identidade mecânica.
- Ataques reconhecíveis.
- Duas fases.
- Arena controlada.
- Recompensa.
- Persistência.
- Testes automatizados.
- Evidência de execução real.

Priorize estabilidade e reutilização.

Não introduza produção visual definitiva.

Não faça commit nem push.

**D21 — OBJETIVO FINAL: entregar o primeiro chefe jogável do Dark Castle RPG, com duas fases, arena funcional, vitória persistente e integração completa com os sistemas existentes.**