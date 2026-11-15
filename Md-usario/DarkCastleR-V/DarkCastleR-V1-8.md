# DARK CASTLE RPG — MEGA PROMPT V1.8
## Exploração Avançada, Segredos, Baús, Relíquias, Mecanismos e Expansão do Castelo

### MISSÃO OFICIAL — EXECUÇÃO AUTORIZADA

Você é o arquiteto principal e responsável técnico pela continuidade do Dark Castle RPG.

O projeto utiliza Rust 2024, Bevy 0.20.1, Blender 5.2.4 LTS e uma arquitetura modular de plugins, com dados configuráveis em RON.

A visão do projeto é construir um Action RPG Metroidvania original, inspirado na profundidade de exploração de Castlevania: Symphony of the Night, com castelo gigantesco, cidade, NPCs, comércio, armas, armaduras, magias, transformações, chefes, relíquias, passagens secretas e exploração não linear.

Não copie mapas, personagens, arte, música, nomes ou história de Castlevania.

A V1.7 foi aprovada tecnicamente, com ressalvas documentadas.

**Você está autorizado a executar a V1.8, dentro do escopo e das restrições deste documento.**

Não inicie a V1.9.

---

# 1. OBJETIVO DA V1.8

A V1.8 deverá transformar a fundação de mundo da V1.7 em uma experiência de exploração mais rica.

Os objetivos são:

1. Criar paredes quebráveis.
2. Criar passagens secretas persistentes.
3. Implementar baús e recompensas.
4. Integrar itens coletados ao inventário existente.
5. Expandir o sistema de relíquias.
6. Implementar mecanismos e portas especiais.
7. Criar atalhos permanentes.
8. Melhorar o mapa.
9. Expandir o castelo com uma pequena região original.
10. Garantir progressão sem softlocks.
11. Melhorar a apresentação das transições.
12. Fortalecer os testes de save e câmera.

Não implemente ainda transformações completas, NPCs, comércio ou dezenas de armas.

A prioridade é **exploração real, persistência e qualidade de implementação**.

---

# 2. ESTADO CONHECIDO DO PROJETO

O relatório da V1.7 informou:

- Branch `art/v1.2-protagonist`.
- HEAD `204f965`.
- Um PNG de conceito previamente staged.
- Alterações não commitadas acumuladas desde V1.2-D.
- 238 testes Rust passando.
- 17 testes Python do pipeline artístico.
- Autoplay 23/23.
- Visual Check aprovado.
- Rota principal aprovada com 14 transições.
- Zero reposicionamentos artificiais.
- Mapa do protótipo explorado em 100%.
- Save V2 implementado.
- Migração tipada V1 → V2.
- Mundo integrado ao jogo principal.
- Nenhum commit ou push.

Confirme tudo no código.

Não presuma que o estado permaneceu inalterado.

---

# 3. REGRAS PERMANENTES DE SEGURANÇA

Antes de modificar arquivos, registre:

- Branch e HEAD.
- `git status --porcelain=v1 -uall`.
- Arquivos staged.
- Arquivos modificados.
- Arquivos não rastreados.
- Arquivos ignorados relevantes.
- Hashes dos arquivos protegidos.
- Estado dos saves.
- Estado dos assets publicados.
- Estado dos backups.

Preserve integralmente:

- `Md-usario/`.
- `saves/`.
- `.cargo/config.toml`.
- `target/verify/`.
- `target/f4-stage/`.
- Arquivos Blender históricos.
- Backups artísticos.
- Assets publicados pelo pipeline seguro.
- PNG previamente staged.
- Todas as alterações não commitadas.

É proibido:

- `git clean`.
- `git reset --hard`.
- `cargo clean`.
- `git commit`.
- `git push`.
- `git add`.
- Merge.
- Rebase.
- Troca de branch.
- Criação de worktree.
- Exclusão de backups.
- Desabilitar hooks.
- Alterar configurações de segurança.
- Instalar dependências sem autorização.

Não apague pastas de iteração ignoradas.

Não faça staging.

Não crie commits.

---

# 4. REGRA CRÍTICA — COMPILAÇÃO E MEMÓRIA

Na V1.7 houve um travamento por falta de memória durante o linking simultâneo de binários de teste do Bevy.

A partir de agora, esta regra é obrigatória.

Configure os comandos da sessão para utilizar no máximo dois jobs:

```bash
export CARGO_BUILD_JOBS=2
```

Quando apropriado, utilize `-j 2`.

Requisitos:

1. Execute apenas um comando Cargo por vez.
2. Nunca rode múltiplas compilações simultaneamente.
3. Não utilize quatro ou mais jobs.
4. Não apague `target/`.
5. Não execute `cargo clean`.
6. Não altere `.cargo/config.toml`.
7. Monitore a memória antes de operações pesadas.
8. Registre eventuais falhas de memória.

Se dois jobs ainda provocarem pressão de memória, reduza para um job.

Não aumente o paralelismo para acelerar a execução.

---

# 5. AUDITORIA INICIAL

Leia os documentos existentes:

- `CLAUDE.md`.
- `docs/GAME_VISION_MASTER.md`.
- `docs/WORLD_ARCHITECTURE.md`.
- `docs/CASTLE_WORLD_DESIGN.md`.
- `docs/ROOM_SYSTEM.md`.
- `docs/MAP_EXPLORATION_SYSTEM.md`.
- `docs/WORLD_PERSISTENCE.md`.
- `docs/WORLD_INTEGRATION_V1_7.md`.
- `docs/WORLD_CONTENT_PIPELINE.md`.
- `docs/INVENTORY_EQUIPMENT.md`.
- `docs/TRANSFORMATION_SYSTEM.md`.
- `docs/DECISIONS.md`.
- `docs/ROADMAP.md`.

Inspecione:

- `src/world/`.
- `src/player/`.
- `src/combat/`.
- `src/enemies/`.
- `src/inventory/`, se existir.
- `src/save/`.
- `src/ui/`.
- `assets/world/`.
- `examples/world_main_route.rs`.

Confirme os caminhos reais.

Identifique o que já existe antes de criar novos módulos.

Não duplique sistemas.

---

# 6. ARQUITETURA DE INTERAÇÕES DO MUNDO

Antes de implementar paredes, baús e mecanismos, avalie a necessidade de um contrato reutilizável para objetos interativos.

Ele deverá permitir:

- Identificador estável.
- Sala proprietária.
- Tipo de interação.
- Estado.
- Requisitos.
- Resultado.
- Persistência.
- Representação visual.
- Validação.

Tipos possíveis:

- BreakableWall.
- HiddenPassage.
- Chest.
- Lever.
- PressurePlate.
- LockedDoor.
- RelicPickup.
- Shortcut.
- Switch.
- SecretTrigger.

Esses nomes são conceituais.

Adapte ao código real.

Não crie uma hierarquia excessivamente complexa.

Prefira dados tipados e componentes pequenos.

Não use strings livres para representar regras de gameplay quando enums ou tipos estruturados forem adequados.

---

# 7. PAREDES QUEBRÁVEIS

Implemente paredes que possam ser destruídas por ataques existentes.

## Requisitos

Cada parede deverá possuir:

- ID estável.
- Sala.
- Geometria.
- Estado intacto/quebrado.
- Condição de destruição.
- Representação visual.
- Colisão.
- Estado persistente.

A parede deverá reagir ao sistema real de ataques.

Não crie um segundo sistema de dano.

Não altere hitboxes do jogador.

Não altere dano das armas.

Não altere alcance.

Não altere cooldown.

## 7.1 Condições

Planeje suporte a:

- Qualquer ataque válido.
- Tipo específico de dano.
- Elemento específico.
- Habilidade de exploração.
- Relíquia necessária.

Na V1.8, implemente apenas as condições necessárias ao protótipo.

## 7.2 Persistência

Depois de destruída:

- A parede permanece destruída ao sair e voltar.
- Permanece destruída após save/load.
- A conexão correspondente fica disponível.
- A colisão não reaparece indevidamente.
- A porcentagem de exploração é atualizada corretamente.

Não permita que um efeito visual isolado destrua a parede sem um ataque válido.

---

# 8. PASSAGENS SECRETAS

Expanda o sistema de segredos da V1.7.

Implemente pelo menos dois métodos distintos de descoberta:

1. Parede quebrável.
2. Mecanismo ou interação oculta.

O segredo não deve aparecer no mapa antes da descoberta.

Depois de descoberto:

- A sala ou conexão é revelada.
- O estado é persistido.
- A passagem permanece acessível.
- O mapa reflete a descoberta.

Não revele todos os segredos por proximidade.

Não use aleatoriedade para decidir se a passagem existe.

---

# 9. BAÚS E RECOMPENSAS

Implemente um sistema reutilizável de baús.

## 9.1 Tipos

Planeje:

- Common.
- Rare.
- Secret.
- BossReward.

Implemente somente os tipos necessários à V1.8.

## 9.2 Estados

Um baú deverá possuir:

- ID.
- Sala.
- Tipo.
- Estado fechado/aberto.
- Recompensa.
- Requisitos.
- Persistência.

## 9.3 Recompensas

Integre ao inventário existente.

Exemplos:

- Moedas, se o sistema já existir.
- Consumíveis.
- Equipamentos existentes.
- Materiais.
- Relíquias.

Não invente uma economia completa nesta fase.

Não crie centenas de itens.

Use alguns itens de teste bem definidos.

## 9.4 Coleta única

Depois de aberto:

- Não pode conceder a recompensa novamente.
- Permanece aberto ao retornar.
- Permanece aberto após load.
- Não duplica itens em transições.
- Não duplica itens quando a interação é repetida no mesmo frame.

A concessão da recompensa e a atualização do estado do baú devem ser uma operação lógica coerente.

Teste interrupções e falhas de persistência.

---

# 10. RELÍQUIAS DE EXPLORAÇÃO

A V1.7 integrou a Relíquia do Véu como progressão permanente.

Preserve esse sistema.

Expanda a arquitetura para suportar outras relíquias.

Propostas:

- Relíquia do Véu.
- Selo das Profundezas.
- Emblema Astral.
- Chave dos Guardiões.

Os nomes são provisórios.

Não implemente todas.

## 10.1 Requisitos

Uma relíquia poderá:

- Desbloquear uma passagem.
- Ativar um mecanismo.
- Permitir acesso a uma região.
- Conceder uma capacidade futura.
- Interagir com transformações futuras.

Não implemente ainda a Forma Espectral completa.

Não altere a física do jogador.

Não adicione voo ou atravessar paredes como movimento livre nesta fase.

---

# 11. MECANISMOS DO CASTELO

Implemente mecanismos interativos.

Priorize:

- Alavanca.
- Interruptor.
- Porta controlada.
- Atalho permanente.

## 11.1 Alavancas

Uma alavanca deverá:

- Ter ID estável.
- Possuir estado.
- Acionar um alvo válido.
- Produzir feedback visual.
- Persistir quando apropriado.

## 11.2 Portas especiais

Uma porta poderá exigir:

- Relíquia.
- Chave.
- Mecanismo ativado.
- Flag de progressão.

Não altere a lógica de combate.

## 11.3 Atalhos

Um atalho deverá permitir retornar mais rapidamente a uma região anterior.

Teste ida e volta.

Não crie conexões inválidas.

Não crie atalhos que ignorem requisitos obrigatórios da progressão principal.

---

# 12. EXPANSÃO DO CASTELO

Crie uma região original pequena, conectada ao mundo atual.

Nome provisório:

**Galerias Esquecidas.**

A região deverá conter aproximadamente 8 a 12 salas.

Sugestão de composição:

1. Entrada das galerias.
2. Corredor principal.
3. Sala vertical.
4. Sala com parede quebrável.
5. Sala secreta.
6. Sala com baú.
7. Sala com mecanismo.
8. Sala com porta especial.
9. Sala de recompensa.
10. Conexão de retorno.

A quantidade final pode variar conforme a geometria e a qualidade da rota.

Não crie salas apenas para alcançar um número.

## 12.1 Design do mapa

A região deve possuir:

- Pelo menos um ciclo.
- Um caminho principal.
- Um caminho opcional.
- Um segredo.
- Um atalho.
- Uma barreira de progressão.
- Uma recompensa.
- Uma conexão com a região anterior.

Evite um corredor linear.

## 12.2 Navegabilidade

Todas as salas obrigatórias devem ser percorríveis com as habilidades disponíveis.

Não altere:

- Gravidade.
- Velocidade.
- Altura do salto.
- Collider do jogador.

Ajuste a geometria das salas.

---

# 13. MAPA APRIMORADO

Expanda o mapa integrado à tecla M.

Adicione, quando apropriado:

- Sala atual.
- Salas visitadas.
- Salas descobertas.
- Conexões conhecidas.
- Salas de recompensa descobertas.
- Passagens secretas descobertas.
- Portas bloqueadas conhecidas.
- Atalhos desbloqueados.
- Marcadores de exploração.

Não revele conteúdo oculto antes da descoberta.

## 13.1 Porcentagem de exploração

Mantenha uma fórmula consistente.

Defina claramente:

- O que conta como sala visitada.
- O que conta como segredo.
- Como novas regiões entram no denominador.
- Como regiões opcionais afetam o percentual.

Não altere silenciosamente o significado da porcentagem existente.

Se a expansão exigir mudança de contrato, documente e teste.

---

# 14. SAVE V2 E COMPATIBILIDADE

A V1.8 deverá preservar a compatibilidade com Save V1 e Save V2.

Não modifique saves reais durante os testes.

Antes de alterar o formato, verifique se os novos estados podem ser representados pelos campos existentes.

Prefira IDs e flags persistentes já previstos.

Não crie Save V3 sem necessidade.

Se uma mudança de schema for indispensável, apresente a proposta e solicite autorização antes de implementá-la.

Teste:

- Parede quebrada.
- Baú aberto.
- Relíquia coletada.
- Mecanismo ativado.
- Porta desbloqueada.
- Atalho aberto.
- Segredo descoberto.
- Mudança de sala.
- Quick load.
- Retorno ao jogo após reinício.

## 14.1 Fixture histórica

A V1.7 utilizou uma fixture V1 construída manualmente.

Mantenha o teste existente.

Documente que a compatibilidade com um save histórico real ainda não foi comprovada.

Não acesse ou copie saves reais sem autorização.

---

# 15. TESTES DE CÂMERA

A V1.7 validou visualmente as entradas pelas quatro direções.

Agora implemente testes automatizados para:

1. Entrada pela esquerda.
2. Entrada pela direita.
3. Entrada pelo alto.
4. Entrada por baixo.
5. Sala pequena.
6. Sala grande.
7. Sala vertical.
8. Quick load em outra sala.
9. Início do jogo.
10. Transição repetida.

Verifique que a câmera não exibe áreas inválidas por erro de inicialização.

Preserve o zoom.

Não modifique o sistema global de escala.

---

# 16. TRANSIÇÕES VISUAIS

Atualmente as transições entre salas utilizam corte seco.

Implemente uma transição visual discreta, se isso puder ser feito sem alterar a física ou o ciclo de vida.

Uma opção é:

- Fade-out curto.
- Troca segura de sala.
- Fade-in curto.

A duração deve ser pequena e não prejudicar o ritmo de exploração.

Não permita que o fade esconda erros de carregamento.

Não altere a posição do jogador para sincronizar o efeito.

Não bloqueie indefinidamente o input.

Se houver risco arquitetural, mantenha o corte seco e documente a proposta.

---

# 17. TESTES DE PROGRESSÃO

Crie um validador que comprove que a região pode ser concluída.

Cenários:

- Jogador sem relíquia.
- Jogador com relíquia.
- Segredo não descoberto.
- Segredo descoberto.
- Mecanismo desligado.
- Mecanismo ligado.
- Atalho fechado.
- Atalho aberto.

Verifique:

- Caminho principal alcançável.
- Recompensa opcional alcançável.
- Retorno possível.
- Ausência de bloqueios irreversíveis.
- Ausência de dependências circulares impossíveis.

O validador deve distinguir progressão obrigatória de conteúdo opcional.

---

# 18. TESTE DE ROTA REAL

Crie um exemplo automatizado para percorrer a região.

Sugestão:

`examples/world_secrets_route.rs`

A rota deve utilizar:

- Movimento real.
- Salto real.
- Ataques reais.
- Interações reais.
- Transições legítimas.

Não reposicione diretamente o jogador durante a rota.

Não utilize teleporte de teste.

O contador esperado é:

**Zero reposicionamentos artificiais.**

A demonstração deverá:

1. Entrar nas Galerias Esquecidas.
2. Explorar o corredor.
3. Destruir uma parede.
4. Encontrar uma sala secreta.
5. Abrir um baú.
6. Ativar um mecanismo.
7. Abrir uma porta.
8. Desbloquear um atalho.
9. Retornar à região anterior.
10. Confirmar persistência após save/load.

Se a rota não puder ser percorrida, corrija a geometria ou a integração.

Não altere a física.

---

# 19. TESTES DE COMBATE E OBJETOS

Valide:

- Ataque atinge parede quebrável.
- Ataque não atravessa obstáculos indevidamente.
- Hitbox não destrói objetos em outra sala.
- VFX não causa dano.
- Projétil não concede recompensa.
- Ataque interrompido não duplica destruição.
- Parede já destruída não gera evento repetido.
- Baú não abre duas vezes.
- Mecanismo não dispara duas vezes indevidamente.
- Porta não troca de sala duas vezes no mesmo frame.

Não altere as fórmulas de dano.

---

# 20. TESTES DE PERSISTÊNCIA

Crie testes determinísticos para:

1. Abrir baú e salvar.
2. Carregar e verificar baú aberto.
3. Quebrar parede e salvar.
4. Carregar e verificar parede quebrada.
5. Ativar alavanca e salvar.
6. Carregar e verificar mecanismo ativo.
7. Desbloquear atalho e salvar.
8. Carregar e verificar conexão aberta.
9. Coletar relíquia e salvar.
10. Carregar e verificar requisito desbloqueado.
11. Voltar à sala anterior.
12. Reiniciar a sessão.
13. Carregar fixture V1.
14. Carregar fixture V2.
15. Rejeitar IDs inválidos com segurança.

Não grave em `saves/`.

Use diretórios temporários de teste.

---

# 21. TESTES DE STRESS

Execute:

- 100 transições.
- 100 interações repetidas.
- 100 aberturas e fechamentos de mapa.
- 50 ciclos de save/load em arquivos temporários.
- 50 retornos a salas com baús.
- 50 retornos a salas com paredes destruídas.
- 50 ativações repetidas de mecanismos.

Verifique:

- Ausência de duplicação.
- Ausência de vazamento de entidades.
- Estado persistente coerente.
- Ausência de hitboxes antigas.
- Ausência de recompensas duplicadas.

Não considere ausência de crash como prova suficiente.

---

# 22. VALIDAÇÃO VISUAL

Execute o jogo real com janela.

Produza capturas de:

- Parede intacta.
- Parede destruída.
- Passagem revelada.
- Baú fechado.
- Baú aberto.
- Item recebido.
- Alavanca desligada.
- Alavanca ligada.
- Porta fechada.
- Porta aberta.
- Mapa antes da descoberta.
- Mapa depois da descoberta.
- Atalho desbloqueado.

Produza uma demonstração curta, se as ferramentas existentes permitirem.

Identifique claramente capturas reais do Bevy e comparações offline.

Não afirme ter inspecionado imagens que não foram abertas.

---

# 23. PERFORMANCE

Compare a V1.8 com a V1.7.

Registre:

- Quantidade de salas.
- Entidades por sala.
- Tempo de transição.
- Tempo de carregamento.
- Memória.
- Tempo de save/load.
- Custo de atualização do mapa.
- Custo de interação.
- Quantidade de objetos persistentes.

Não carregue o castelo inteiro em entidades ativas.

Não faça leitura de arquivos por frame.

Não gere novas texturas em runtime sem necessidade.

---

# 24. TESTES E COMANDOS

Execute, sequencialmente:

```bash
export CARGO_BUILD_JOBS=2

cargo fmt --all -- --check
cargo check -j 2
cargo clippy --all-targets -j 2 -- -D warnings
cargo test -j 2
```

Execute também:

- Testes Python do pipeline artístico.
- Autoplay.
- Visual Check.
- World Report.
- World Main Route.
- World Secrets Route.
- Testes de Save V1/V2.
- Testes de câmera.
- Testes de progressão.
- Testes de interações.

Se houver pressão de memória, use um job.

Não execute compilações paralelas.

Não apague caches.

---

# 25. DOCUMENTAÇÃO OBRIGATÓRIA

Crie:

`docs/WORLD_EXPLORATION_V1_8.md`

Documente:

1. Estado inicial.
2. Arquitetura de interações.
3. Paredes quebráveis.
4. Segredos.
5. Baús.
6. Recompensas.
7. Relíquias.
8. Mecanismos.
9. Portas.
10. Atalhos.
11. Região nova.
12. Mapa.
13. Save/load.
14. Câmera.
15. Transições visuais.
16. Testes de progressão.
17. Testes de rota.
18. Stress.
19. Performance.
20. Riscos e pendências.

Atualize somente os documentos afetados.

Registre decisões em `docs/DECISIONS.md`.

Não reescreva documentos inteiros sem necessidade.

---

# 26. CRITÉRIOS DE ACEITAÇÃO

A V1.8 só estará concluída quando:

- [ ] Estado inicial auditado.
- [ ] Arquivos protegidos preservados.
- [ ] Regra de compilação respeitada.
- [ ] Interações reutilizáveis implementadas.
- [ ] Paredes quebráveis funcionais.
- [ ] Passagens secretas persistentes.
- [ ] Baús funcionais.
- [ ] Recompensas sem duplicação.
- [ ] Integração com inventário.
- [ ] Relíquias integradas.
- [ ] Mecanismos funcionais.
- [ ] Portas especiais funcionais.
- [ ] Atalhos persistentes.
- [ ] Região nova conectada.
- [ ] Mapa atualizado.
- [ ] Segredos não revelados antecipadamente.
- [ ] Save V1 compatível.
- [ ] Save V2 compatível.
- [ ] Câmera testada nas quatro direções.
- [ ] Transições visuais implementadas ou impedimento documentado.
- [ ] Rota real sem reposicionamentos artificiais.
- [ ] Testes de progressão passando.
- [ ] Testes de persistência passando.
- [ ] Testes de stress passando.
- [ ] Autoplay passando.
- [ ] Visual Check passando.
- [ ] Gameplay preservado.
- [ ] Animações preservadas.
- [ ] VFX preservados.
- [ ] Git preservado.
- [ ] Nenhum commit ou push.
- [ ] V1.9 não iniciada.

Não marque como concluído um item apenas porque a arquitetura está planejada.

---

# 27. RELATÓRIO FINAL

Entregue:

**A. Estado inicial e Git**

**B. Auditoria da V1.7**

**C. Arquitetura de interações**

**D. Paredes quebráveis**

**E. Passagens secretas**

**F. Baús e recompensas**

**G. Relíquias**

**H. Mecanismos**

**I. Portas e atalhos**

**J. Nova região**

**K. Mapa**

**L. Save V1/V2**

**M. Câmera**

**N. Transições visuais**

**O. Progressão**

**P. Rota real**

**Q. Testes de combate e objetos**

**R. Persistência**

**S. Stress**

**T. Validação visual**

**U. Performance**

**V. Arquivos criados e modificados**

**W. Estado final do Git**

**X. Pendências e riscos**

**Y. Próximos passos recomendados**

Inclua:

- Comandos executados.
- Resultados reais.
- Quantidade de testes.
- Caminhos de capturas.
- Caminhos de demonstrações.
- Limitações.
- Arquivos alterados.
- Hashes relevantes.

Diferencie o que foi implementado, testado, inspecionado visualmente e apenas planejado.

---

# 28. CONDIÇÕES DE PARADA

Interrompa a operação afetada se:

- Houver risco de perda de saves.
- Uma migração de schema for indispensável.
- Uma alteração exigir reescrever gameplay.
- O Git apresentar risco de perda de trabalho.
- Um objeto persistente puder duplicar recompensas.
- Uma transição puder corromper o mundo.
- Uma dependência nova for necessária.
- A memória ficar insuficiente.
- Uma solução exigir alterar a física do jogador.
- Um arquivo protegido precisar ser modificado.

Documente o problema.

Não aplique soluções destrutivas.

Continue somente tarefas independentes e seguras.

---

# 29. INSTRUÇÃO FINAL

Você está autorizado a executar a V1.8.

Ordem obrigatória:

1. Auditoria.
2. Segurança.
3. Arquitetura de interações.
4. Paredes e segredos.
5. Baús e recompensas.
6. Relíquias.
7. Mecanismos.
8. Portas e atalhos.
9. Expansão do castelo.
10. Mapa.
11. Save/load.
12. Câmera e transições.
13. Testes.
14. Validação visual.
15. Documentação.
16. Relatório final.

Não implemente a V1.9.

Não implemente transformações completas.

Não implemente NPCs ou comércio.

Não produza dezenas de armas.

Não altere o gameplay fundamental.

Não faça staging.

Não crie commits.

Não faça push.

**A V1.8 deve terminar com uma nova região realmente explorável, contendo segredos, paredes quebráveis, baús, mecanismos, relíquias e atalhos persistentes, integrada ao castelo existente e compatível com os saves atuais.**

Ao concluir, apresente o relatório completo e aguarde minha avaliação.