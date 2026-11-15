# DARK CASTLE RPG — MEGA PROMPT V1.9
## Sistema de Transformações, Forma Espectral, Movimento Especial e Progressão Metroidvania

### MISSÃO OFICIAL — IMPLEMENTAÇÃO AUTORIZADA

Você é o arquiteto técnico principal e responsável pela implementação da V1.9 do Dark Castle RPG.

O projeto é um Action RPG Metroidvania 2D original, desenvolvido em Rust 2024 com Bevy 0.20.1, dados RON e pipeline artístico baseado em Blender 5.2.4 LTS.

A visão de longo prazo inclui:

- Um castelo gigantesco e interconectado.
- Transformações que modificam a exploração.
- Diversos tipos de armas e equipamentos.
- Magias e técnicas especiais.
- Relíquias e habilidades permanentes.
- Regiões secretas.
- Chefes.
- Cidade e NPCs.
- Comércio.
- Missões.
- Progressão não linear.

A inspiração de design é a profundidade de exploração de Castlevania: Symphony of the Night, mas todo o conteúdo deverá ser original.

**Sua missão é implementar a primeira transformação jogável e criar uma arquitetura reutilizável para futuras transformações.**

Você está autorizado a executar a V1.9 dentro das restrições deste briefing.

Não inicie a V2.0.

---

# 1. ESTADO CONHECIDO

A V1.8 foi aprovada tecnicamente, com ressalvas.

O relatório informou:

- Branch `art/v1.2-protagonist`.
- HEAD `204f965`.
- 268 testes Rust passando.
- 17 testes Python passando.
- Autoplay 23/23.
- Visual Check aprovado.
- Mundo com 20 salas.
- Duas regiões conectadas.
- Três segredos.
- Quatro baús nas Galerias Esquecidas.
- Sistema de paredes quebráveis.
- Alavancas.
- Portas especiais.
- Atalhos persistentes.
- Save V2.
- Mapa integrado.
- Rota real com 37 transições.
- Zero reposicionamentos artificiais.

A V1.8 também introduziu:

- Interação pela tecla D.
- Mensagem `ProjectileBlocked`.
- Relíquia Emblema Astral.
- Fade de entrada.
- Objetivo na Ponte de Retorno.

Confirme todos esses fatos no repositório.

Não confunda resultados relatados com evidências verificadas.

---

# 2. REGRAS ABSOLUTAS DE SEGURANÇA

Antes de modificar qualquer arquivo:

1. Registre branch e HEAD.
2. Registre o status completo do Git.
3. Identifique os arquivos staged.
4. Identifique alterações não commitadas.
5. Identifique arquivos ignorados importantes.
6. Registre hashes dos arquivos protegidos.
7. Confirme a integridade dos saves.
8. Confirme a integridade dos assets publicados.
9. Confirme a linha de base dos testes.
10. Leia as regras atuais do projeto.

Preserve integralmente:

- `Md-usario/`, incluindo a reorganização em `DarkCastleR-F/` e `DarkCastleR-V/`.
- `saves/`.
- `.cargo/config.toml`.
- `target/verify/`.
- `target/f4-stage/`.
- Modelos Blender anteriores.
- Backups artísticos.
- Assets publicados.
- O PNG previamente staged.
- Todas as alterações não commitadas.

É proibido:

- Executar `git clean`.
- Executar `git reset --hard`.
- Executar `cargo clean`.
- Criar commits.
- Fazer push.
- Fazer staging.
- Trocar de branch.
- Criar worktrees.
- Fazer merge ou rebase.
- Apagar backups.
- Desabilitar hooks.
- Instalar dependências sem autorização.
- Alterar arquivos protegidos sem autorização.

Não modifique o índice Git.

---

# 3. REGRA PERMANENTE DE MEMÓRIA

A V1.7 sofreu um travamento por falta de memória durante compilações paralelas do Bevy.

Utilize no máximo dois jobs.

```bash
export CARGO_BUILD_JOBS=2
```

Execute apenas um comando Cargo por vez.

Se houver pressão de memória, reduza para um job.

Não execute múltiplos processos de linking simultaneamente.

Não apague caches.

Não modifique `.cargo/config.toml`.

Registre falhas de memória, caso ocorram.

---

# 4. AUDITORIA OBRIGATÓRIA

Leia:

- `CLAUDE.md`.
- `docs/GAME_VISION_MASTER.md`.
- `docs/TRANSFORMATION_SYSTEM.md`.
- `docs/CHARACTER_ATTRIBUTES.md`.
- `docs/COMBAT_SYSTEM.md`.
- `docs/INPUT_COMMANDS.md`.
- `docs/SKILLS_SYSTEM.md`.
- `docs/MAGIC_SYSTEM.md`.
- `docs/WORLD_ARCHITECTURE.md`.
- `docs/ROOM_SYSTEM.md`.
- `docs/MAP_EXPLORATION_SYSTEM.md`.
- `docs/WORLD_PERSISTENCE.md`.
- `docs/WORLD_EXPLORATION_V1_8.md`.
- `docs/INVENTORY_EQUIPMENT.md`.
- `docs/DECISIONS.md`.
- `docs/ROADMAP.md`.

Inspecione:

- `src/player/`.
- `src/input/`.
- `src/combat/`.
- `src/skills/`.
- `src/magic/`.
- `src/world/`.
- `src/save/`.
- `src/ui/`.
- `assets/world/`.
- `assets/animations/player/`, se existir.
- `art-src/player/`, se existir.

Confirme os caminhos reais.

Não crie sistemas duplicados.

Produza uma matriz de impacto antes de implementar.

---

# 5. OBJETIVO CENTRAL

Implementar a primeira transformação:

**Forma Espectral — Spectral Form**

O protagonista poderá converter temporariamente seu corpo em uma manifestação espectral.

Essa forma deverá permitir atravessar barreiras específicas, mas não paredes comuns.

A transformação será desbloqueada pela Relíquia do Véu, já existente no projeto.

A mecânica deverá ser:

- Jogável.
- Visualmente identificável.
- Controlável.
- Reversível.
- Persistente quanto ao desbloqueio.
- Compatível com as salas existentes.
- Compatível com Save V2.
- Segura contra softlocks.
- Extensível a futuras formas.

---

# 6. ARQUITETURA DE TRANSFORMAÇÕES

Crie um sistema modular de transformações.

Avalie uma estrutura conceitual:

```text
TransformationSystem
├── TransformationDefinition
├── TransformationState
├── TransformationController
├── MovementProfile
├── CollisionProfile
├── TransformationVisual
├── TransformationRequirements
└── TransformationEvents
```

Adapte os nomes à arquitetura real.

Evite abstrações desnecessárias.

## 6.1 Definições em dados

Cada transformação deverá possuir:

- ID estável.
- Nome.
- Descrição.
- Requisito de desbloqueio.
- Perfil de movimento.
- Perfil de colisão.
- Habilidades permitidas.
- Habilidades bloqueadas.
- Custo de ativação.
- Custo de manutenção, quando aplicável.
- Regras de cancelamento.
- Representação visual.
- Regras de interação.
- Regras de persistência.

Prefira dados RON validados.

Não permita que dados externos executem lógica arbitrária.

## 6.2 Separação de responsabilidades

O sistema de transformação deverá decidir o estado ativo.

O sistema de movimento deverá aplicar o perfil.

O sistema de colisão deverá aplicar regras específicas.

O sistema visual deverá representar a forma.

O sistema de save deverá persistir somente os dados necessários.

Não concentre tudo em um único sistema gigantesco.

---

# 7. PERFIS DE MOVIMENTO

Implemente a base de perfis de movimento.

O perfil humano deve continuar exatamente como está.

O perfil espectral poderá ter propriedades específicas.

Exemplo conceitual:

```text
Human
  movimento normal
  salto normal
  gravidade normal
  colisão normal

Spectral
  deslocamento controlado
  gravidade conforme design aprovado
  atravessa barreiras espectrais
  não atravessa paredes sólidas
  restrições de combate
```

**Não altere os valores globais do movimento humano.**

Não modifique a altura do salto para facilitar os testes.

Não altere o collider humano permanentemente.

Não permita que o perfil espectral afete o personagem depois da reversão.

---

# 8. DEFINIÇÃO DA FORMA ESPECTRAL

A Forma Espectral será a primeira transformação implementada.

## 8.1 Identidade

Nome: Forma Espectral.

Nome técnico sugerido: `spectral_form`.

Relíquia necessária: Relíquia do Véu.

Aparência: manifestação translúcida, com contorno luminoso e partículas discretas.

## 8.2 Movimento

Minha proposta inicial é:

- Movimento horizontal controlável.
- Movimento vertical limitado.
- Sem voo livre ilimitado.
- Sem ataque convencional.
- Sem uso de equipamentos durante a forma.
- Sem atravessar paredes comuns.
- Travessia de barreiras espectrais específicas.

Avalie a implementação mais segura para o controlador existente.

Documente os valores escolhidos.

Não copie valores de outro jogo.

## 8.3 Duração

Proponho uma forma temporária.

Ela poderá consumir MP continuamente enquanto ativa.

Implemente:

- Custo inicial configurável.
- Consumo por segundo configurável.
- Cancelamento voluntário.
- Cancelamento por falta de MP.
- Retorno à forma humana.

Escolha valores provisórios coerentes com os recursos atuais.

Não altere a regeneração global de MP.

Não crie recursos novos sem necessidade.

---

# 9. ATIVAÇÃO E CONTROLES

A transformação deve possuir um comando próprio.

Audite os controles existentes antes de escolher a tecla.

Controles conhecidos:

- Setas: movimento.
- Shift: corrida.
- Z: salto.
- X: ataque.
- C: magia.
- A: defesa/cancelar.
- S: esquiva.
- D: interação.
- M: mapa.
- Tab: status.
- F5/F9: save/load.

Não reutilize teclas que causem conflitos.

Sugestão inicial: tecla T para alternar a transformação.

Confirme se T está livre.

## 9.1 Ativação

A transformação só poderá iniciar quando:

- O jogador estiver vivo.
- A relíquia estiver desbloqueada.
- Não houver transição de sala em andamento.
- Não houver quick load em andamento.
- O estado atual permitir a ativação.
- Os recursos necessários estiverem disponíveis.

## 9.2 Cancelamento

Permita retornar à forma humana com o mesmo comando.

Não permita ativação repetida em todos os frames enquanto a tecla estiver pressionada.

Não permita consumo duplicado de MP.

---

# 10. COLISÃO ESPECTRAL

Esta é uma das partes mais críticas.

**Não implemente a transformação simplesmente desabilitando todas as colisões.**

Crie uma distinção entre:

- Paredes sólidas.
- Plataformas.
- Portas.
- Barreiras espectrais.
- Objetos interativos.
- Inimigos.
- Projéteis.

A Forma Espectral só poderá atravessar barreiras autorizadas.

## 10.1 Requisitos

Uma barreira espectral deverá:

- Possuir ID estável.
- Ter geometria.
- Possuir regra de colisão.
- Identificar a forma permitida.
- Ser representada visualmente.
- Ser validada nos dados da sala.

## 10.2 Segurança

Teste:

- Entrada na barreira.
- Travessia completa.
- Cancelamento dentro da barreira.
- MP acabando dentro da barreira.
- Morte durante a travessia.
- Quick load dentro da barreira.
- Troca de sala durante a travessia.

O personagem nunca poderá retornar à forma humana preso dentro de uma parede.

Defina uma política determinística de recuperação.

Não use teleporte arbitrário para esconder falhas.

Uma solução aceitável é impedir a reversão enquanto não existir uma posição humana segura, desde que exista recuperação garantida.

Se a forma ficar sem MP dentro da barreira, não permita que o jogador fique preso.

---

# 11. COMBATE DURANTE A TRANSFORMAÇÃO

Na primeira versão, a Forma Espectral não terá ataques convencionais.

Bloqueie:

- Ataque básico.
- Técnicas físicas.
- Magias convencionais.
- Esquiva humana.
- Defesa humana.

Preserve os comandos existentes para a forma humana.

Não altere:

- Dano.
- Cooldowns.
- Fórmulas.
- Hitboxes.
- Hurtboxes.
- IA inimiga.

Se o jogador estiver executando um ataque, defina em quais estados a transformação poderá começar.

Não interrompa ações de combate de maneira inconsistente.

---

# 12. DANO E MORTE

Defina claramente se a Forma Espectral pode receber dano.

Minha proposta:

- O personagem continua vulnerável.
- A transformação não concede invulnerabilidade.
- O dano recebido continua usando as regras existentes.
- A morte cancela a transformação.

Não altere resistências globais.

Não crie imunidade automática.

Teste dano durante travessia de barreira.

Garanta que a morte não deixe o personagem em um estado de colisão inválido.

---

# 13. ANIMAÇÕES E APARÊNCIA

A transformação precisa ser visualmente distinta.

O protagonista atual possui um pipeline artístico cuidadosamente estabilizado.

Preserve:

- Modelos Blender existentes.
- Ativos publicados.
- Atla​ses atuais.
- Pivôs.
- Durações.
- Animações humanas.
- VFX existentes.

## 13.1 Estratégia visual

Para a V1.9, priorize uma representação segura que reutilize a silhueta existente com:

- Transparência.
- Tonalidade espectral.
- Contorno.
- Partículas.
- Efeito de entrada.
- Efeito de saída.

Não modifique os PNGs publicados diretamente.

Não publique novos assets fora do guarda de publicação.

Se forem necessários novos assets, produza-os em caminhos de trabalho isolados e respeite o pipeline de publicação.

## 13.2 Transições

Implemente feedback visual para:

- Ativação.
- Forma ativa.
- MP baixo.
- Reversão.
- Cancelamento por morte.

Não deixe o personagem visualmente humano enquanto a física já está espectral sem uma regra explícita de sincronização.

---

# 14. HUD DA TRANSFORMAÇÃO

Adicione feedback discreto à interface.

O jogador deverá visualizar:

- Forma atual.
- Disponibilidade da transformação.
- MP consumido.
- MP restante.
- Indicação quando a forma não puder ser ativada.

Não cubra o personagem.

Não aumente o problema existente da barra de dicas cobrindo o chão.

Se possível, aproveite a V1.9 para reorganizar a dica de interação sem modificar a lógica de HUD já aprovada.

---

# 15. BARREIRAS ESPECTRAIS NO CASTELO

Adicione algumas barreiras específicas às regiões existentes.

Não reestruture o castelo inteiro.

Proponho:

- Uma barreira tutorial.
- Uma passagem opcional.
- Uma passagem para recompensa.
- Uma rota de retorno.

A arquitetura deve permitir novas barreiras em outras regiões futuramente.

Não coloque barreiras espectrais em todas as salas.

A transformação precisa abrir possibilidades de exploração, não substituir todas as chaves e mecanismos.

---

# 16. PEQUENA EXPANSÃO DE CONTEÚDO

A V1.9 poderá acrescentar aproximadamente 3 a 5 salas, se necessário para demonstrar a transformação.

Priorize reutilizar as 20 salas existentes.

A expansão deverá demonstrar:

1. Obtenção da Relíquia do Véu.
2. Ativação da Forma Espectral.
3. Travessia de uma barreira.
4. Descoberta de uma sala opcional.
5. Obtenção de uma recompensa.
6. Retorno à forma humana.
7. Volta à rota principal.

Não crie uma região gigantesca nesta fase.

---

# 17. PROGRESSÃO METROIDVANIA

Integre a transformação ao grafo de progressão.

Diferencie:

- Relíquia obtida.
- Transformação desbloqueada.
- Transformação ativa.
- Barreira atravessável.
- Sala descoberta.

O validador deverá entender que uma barreira espectral exige uma capacidade específica.

Não trate a forma ativa como uma flag permanente de progressão.

A relíquia representa a capacidade desbloqueada.

A forma ativa representa o estado momentâneo.

Teste:

- Jogador sem relíquia.
- Jogador com relíquia.
- Forma humana.
- Forma espectral.
- MP insuficiente.
- Retorno à forma humana.
- Atalho já desbloqueado.

Não permita que o validador considere uma sala alcançável apenas porque a relíquia existe, se o jogador não puder efetivamente atravessar a barreira.

---

# 18. SAVE V2 E PERSISTÊNCIA

Preserve o Save V2.

Não implemente Save V3 sem autorização.

Avalie o que deve ser salvo:

- Transformação desbloqueada.
- Relíquia obtida.
- Estado de exploração.
- Posição.
- Sala atual.
- MP.

Minha recomendação inicial é restaurar o jogador na forma humana após load, desde que isso seja seguro.

Se o save ocorrer em uma posição exclusiva da Forma Espectral, a restauração humana não poderá colocá-lo dentro de uma barreira.

Defina uma política segura.

Possíveis abordagens:

- Impedir save em posição insegura.
- Restaurar a forma ativa se o schema permitir.
- Restaurar em um ponto seguro explicitamente registrado.

Não escolha uma solução que perca progresso silenciosamente.

Não modifique saves reais durante testes.

Teste migração V1 e compatibilidade V2.

---

# 19. SOFTLOCKS E RECUPERAÇÃO

A transformação não poderá prender o jogador.

Cenários obrigatórios:

1. MP termina dentro da barreira.
2. Jogador cancela dentro da barreira.
3. Jogador morre dentro da barreira.
4. Jogador salva dentro da barreira.
5. Jogador carrega dentro da barreira.
6. Jogador tenta transformar durante transição.
7. Jogador atravessa uma barreira e não tem MP para voltar.
8. Jogador entra em uma sala sem saída humana.
9. Jogador perde controle durante ativação.
10. Jogador recebe dano durante reversão.

Para cada cenário, documente o comportamento esperado e implemente teste.

Não use reposicionamento arbitrário para mascarar o problema.

---

# 20. PREPARAÇÃO PARA FUTURAS TRANSFORMAÇÕES

A arquitetura deverá suportar, futuramente:

- Forma voadora.
- Forma terrestre veloz.
- Forma pequena para passagens estreitas.
- Forma aquática.
- Forma de sombra.

Essas são possibilidades de design, não funcionalidades autorizadas agora.

Não implemente todas.

Não crie assets para todas.

Não crie dezenas de estados vazios.

A V1.9 deve demonstrar extensibilidade real com uma única transformação bem implementada.

---

# 21. TESTES AUTOMATIZADOS

Crie testes cobrindo:

- Ativação sem relíquia.
- Ativação com relíquia.
- MP insuficiente.
- Consumo inicial.
- Consumo contínuo.
- Cancelamento.
- Reversão.
- Transformação durante ataque.
- Transformação durante morte.
- Transformação durante transição.
- Forma humana preservada.
- Perfil espectral aplicado.
- Perfil humano restaurado.
- Colisão com parede normal.
- Travessia de barreira espectral.
- Cancelamento dentro da barreira.
- MP acabando dentro da barreira.
- Dano durante a transformação.
- Quick load.
- Save/load.
- Exploração.
- Progressão.
- Ausência de entidades duplicadas.

Não enfraqueça testes antigos.

---

# 22. TESTE DE ROTA REAL

Crie um exemplo de rota automatizada.

Sugestão:

`examples/world_spectral_route.rs`

A rota deverá:

1. Iniciar no castelo.
2. Obter a Relíquia do Véu.
3. Ativar a transformação.
4. Atravessar uma barreira espectral.
5. Retornar à forma humana.
6. Explorar uma passagem opcional.
7. Obter uma recompensa.
8. Salvar.
9. Mudar de sala.
10. Carregar.
11. Confirmar o estado restaurado.
12. Voltar à rota principal.

Use comandos reais.

Não use teleporte de teste.

Não altere diretamente o Transform do jogador durante a rota, exceto por spawn e transições legítimas.

O contador de reposicionamentos artificiais deverá ser zero.

---

# 23. VALIDAÇÃO VISUAL

Execute o jogo real com janela.

Produza capturas mostrando:

- Protagonista humano.
- Ativação da transformação.
- Forma Espectral.
- Barreira espectral.
- Travessia.
- Reversão.
- HUD.
- MP diminuindo.
- Falha de ativação sem relíquia.
- Passagem secreta.
- Recompensa.
- Save/load.

Inspecione pessoalmente as capturas.

Não declare validação visual se não abrir os arquivos.

Se possível, produza uma demonstração curta do fluxo.

---

# 24. PERFORMANCE

Meça:

- Custo de ativação.
- Custo da reversão.
- Custo de colisão especial.
- Quantidade de entidades adicionais.
- Consumo de memória.
- Tempo de transição.
- Tempo de save/load.

Não faça consultas globais caras em todos os frames sem necessidade.

Não carregue recursos de outras transformações inexistentes.

---

# 25. REGRESSÃO DO JOGO

Preserve:

- Movimento humano.
- Salto.
- Corrida.
- Ataque básico.
- Crescent Slash.
- Rising Edge.
- Ember Bolt.
- Mending Light.
- Defesa.
- Esquiva.
- Dano.
- Morte.
- Revive.
- Inventário.
- Equipamentos.
- Mapa.
- Baús.
- Paredes quebráveis.
- Alavancas.
- Atalhos.
- Relíquias existentes.
- Save V1/V2.
- Câmera.
- Animações.
- VFX.

Não altere a fórmula de dano nem os timings aprovados.

---

# 26. COMANDOS DE VALIDAÇÃO

Utilize no máximo dois jobs.

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
- World Spectral Route.
- Testes de save.
- Testes de colisão.
- Testes de progressão.

Execute tudo sequencialmente.

Se necessário, use um job.

Não apague caches.

---

# 27. DOCUMENTAÇÃO

Crie:

`docs/TRANSFORMATIONS_V1_9.md`

Documente:

1. Auditoria.
2. Arquitetura.
3. Definições de transformação.
4. Perfis de movimento.
5. Perfis de colisão.
6. Forma Espectral.
7. Ativação.
8. Cancelamento.
9. Consumo de MP.
10. Combate.
11. Dano.
12. Morte.
13. Animações.
14. HUD.
15. Barreiras.
16. Progressão.
17. Save/load.
18. Softlocks.
19. Testes.
20. Performance.
21. Riscos.
22. Futuras transformações.

Atualize os documentos existentes afetados.

Registre decisões novas em `docs/DECISIONS.md`.

Não reescreva documentação inteira sem necessidade.

---

# 28. CRITÉRIOS DE ACEITAÇÃO

A V1.9 só estará concluída quando:

- [ ] Git auditado.
- [ ] Arquivos protegidos preservados.
- [ ] Compilações limitadas a dois jobs.
- [ ] Arquitetura de transformações implementada.
- [ ] Forma Espectral jogável.
- [ ] Relíquia do Véu integrada.
- [ ] Ativação funcional.
- [ ] Cancelamento funcional.
- [ ] Consumo de MP correto.
- [ ] Movimento humano preservado.
- [ ] Perfil espectral aplicado.
- [ ] Perfil humano restaurado.
- [ ] Colisão normal preservada.
- [ ] Barreiras espectrais atravessáveis.
- [ ] Paredes comuns continuam sólidas.
- [ ] Combate restrito corretamente.
- [ ] Dano funcionando.
- [ ] Morte segura.
- [ ] Reversão segura.
- [ ] Sem softlocks.
- [ ] Progressão integrada.
- [ ] Mapa integrado.
- [ ] Save V2 preservado.
- [ ] Migração V1 preservada.
- [ ] Rota real sem reposicionamentos.
- [ ] Testes automatizados passando.
- [ ] Validação visual realizada.
- [ ] Performance medida.
- [ ] Regressão zero.
- [ ] Documentação atualizada.
- [ ] Nenhum commit.
- [ ] Nenhum push.
- [ ] V2.0 não iniciada.

Se algum item estiver pendente, declare conclusão parcial.

---

# 29. RELATÓRIO FINAL

Entregue:

**A. Estado inicial e Git**

**B. Auditoria da V1.8**

**C. Arquitetura de transformações**

**D. Perfis de movimento**

**E. Perfis de colisão**

**F. Forma Espectral**

**G. Ativação e controles**

**H. Consumo de MP**

**I. Combate**

**J. Dano e morte**

**K. Animações e VFX**

**L. HUD**

**M. Barreiras espectrais**

**N. Expansão do castelo**

**O. Progressão**

**P. Save V2**

**Q. Segurança contra softlocks**

**R. Testes**

**S. Rota real**

**T. Validação visual**

**U. Performance**

**V. Arquivos criados e modificados**

**W. Estado final do Git**

**X. Pendências e riscos**

**Y. Próximas etapas recomendadas**

Inclua os comandos executados e resultados reais.

Diferencie:

- Implementado.
- Testado.
- Validado visualmente.
- Parcial.
- Planejado.

Não declare sucesso sem evidência.

---

# 30. CONDIÇÕES DE PARADA

Interrompa a operação afetada se:

- A transformação exigir alterar a física humana.
- O Save V2 precisar de mudança incompatível.
- Um softlock não puder ser resolvido com segurança.
- A colisão especial comprometer paredes comuns.
- Houver risco de perda de saves.
- Houver risco de perda de trabalho Git.
- O pipeline artístico exigir publicação insegura.
- Uma dependência nova for indispensável.
- A memória ficar insuficiente.
- Uma correção exigir reescrever o combate.

Documente o impedimento.

Continue apenas tarefas independentes e seguras.

Solicite autorização antes de mudanças fora do escopo.

---

# 31. INSTRUÇÃO FINAL

Você está autorizado a implementar a V1.9.

Execute nesta ordem:

1. Auditoria.
2. Arquitetura.
3. Perfis de movimento.
4. Perfis de colisão.
5. Forma Espectral.
6. Ativação e cancelamento.
7. MP.
8. Restrições de combate.
9. Segurança de reversão.
10. Animações e VFX.
11. HUD.
12. Barreiras.
13. Progressão.
14. Save/load.
15. Testes.
16. Rota real.
17. Validação visual.
18. Documentação.
19. Relatório final.

**Não implemente outras transformações completas.**

Não inicie a V2.0.

Não modifique saves reais.

Não altere o movimento humano.

Não modifique o combate fundamental.

Não publique assets sem o pipeline seguro.

Não faça staging.

Não crie commits.

Não faça push.

A V1.9 deve terminar com o protagonista adquirindo a Relíquia do Véu, ativando a Forma Espectral, atravessando barreiras específicas, explorando novos caminhos e retornando com segurança à forma humana.

Ao concluir, apresente o relatório completo e aguarde minha avaliação.