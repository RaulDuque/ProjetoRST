# DARK CASTLE RPG — MEGA PROMPT D19
## Viagem Rápida, Altares Descobertos, Persistência e Integração com o Mundo

**Projeto:** Dark Castle RPG  
**Tecnologia:** Rust 2024 + Bevy 0.20.1  
**Objetivo:** Expandir a exploração e conectar os altares de salvamento existentes  
**Prioridade:** Gameplay, estabilidade e integração  
**Arte definitiva:** FUTURO — NÃO EXECUTAR AGORA

---

# 0. MISSÃO PRINCIPAL

Implementar um sistema de viagem rápida entre altares de salvamento previamente descobertos pelo jogador.

A D17 introduziu pontos de salvamento funcionais, descoberta persistente e marcações no mapa.

A D18 introduziu o uso de consumíveis durante o gameplay.

Agora queremos transformar os altares em uma rede de deslocamento entre regiões exploradas, sem quebrar combate, progressão, salvamento ou transições de salas.

## Resultado desejado

O jogador deverá conseguir:

1. Encontrar um altar.
2. Interagir com ele.
3. Descobri-lo permanentemente no progresso salvo.
4. Descobrir outro altar em uma região diferente.
5. Abrir uma interface de viagem rápida em um altar.
6. Visualizar destinos descobertos.
7. Selecionar um destino válido.
8. Confirmar a viagem.
9. Ser transportado para a sala correta.
10. Aparecer em uma posição segura.
11. Continuar jogando normalmente.
12. Salvar e carregar sem perder as descobertas.

**Não implementar um sistema paralelo de teleporte. Reutilizar a arquitetura de exploração e transições já existente.**

---

# 1. CONTEXTO CONFIRMADO PELOS RELATÓRIOS

## D17

Foi reportada a implementação de:

- `ObjectKind::SavePoint`.
- Mensagem `SavePointActivated`.
- Descoberta de altares.
- `discovered_save_points`.
- `last_save_point`.
- Cura de HP/MP ao descansar.
- Salvamento pelo altar.
- Marcação ciano no mapa.
- Altar `vg_approach_save` em Gate Approach.
- Persistência sem alteração de versão do Save V3.

## D18

Foi reportada a implementação de:

- `inventory/consume.rs`.
- `UseItemRequest`.
- `ItemUsed`.
- Uso de consumíveis.
- Interface de inventário pela tecla I.
- Poções de HP e MP.
- Persistência das quantidades.
- Testes unitários e de integração.

## Sistemas anteriormente reportados

- Portas e transições de sala V1.7.
- Interações V1.8.
- Mapa do mundo.
- NPCs e comércio.
- Progressão e equipamentos.
- Combate.
- Save V3.

**Esses relatórios são contexto histórico, não substituem a inspeção do código atual.**

---

# 2. REGRAS ABSOLUTAS DE SEGURANÇA

Antes de modificar qualquer arquivo:

- Identificar a raiz do repositório.
- Confirmar branch e HEAD.
- Executar `git status --short`.
- Examinar alterações existentes.
- Identificar arquivos modificados pelas etapas D17 e D18.
- Preservar alterações não commitadas.
- Verificar se existem conflitos ou edições simultâneas.

O último estado informado foi:

- Branch `art/v1.2-protagonist`.
- HEAD `1115f0d`.
- Working tree com alterações não commitadas.

Não assumir que esse estado permanece igual.

## Proibições

Não executar:

- `git reset --hard`
- `git clean`
- `git checkout -- .`
- `git restore .`
- `git add .`

Não executar operações destrutivas equivalentes.

**Não fazer commit nem push.**

Não criar branch automaticamente.

Não sobrescrever mudanças preexistentes.

Não modificar arquivos protegidos.

---

# 3. ARQUIVOS E SISTEMAS PROTEGIDOS

Preservar:

- `Md-usario/`.
- `Md-usario/assets-user/`.
- `Md-usario/assets-user/prota/base/white_mesh.glb`.
- `art-src/visual-prototypes/`.
- `.claude/`.
- `.cargo/`.
- Saves reais do usuário.
- Experimentos D9–D16.
- Arquivos Blender.
- Modelos Hunyuan.
- Arte existente.
- Protagonista atual.
- Combate e magias existentes.

Não iniciar Hunyuan.

Não abrir Blender MCP.

Não gerar imagens.

Não modificar a aparência final do jogo.

---

# 4. FASE A — AUDITORIA DA EXPLORAÇÃO

Antes de implementar, inspecione:

- Estrutura de salas.
- IDs de salas.
- Definições RON.
- Sistema de portas.
- Sistema de transições.
- Recursos de mundo.
- Sistema de interações.
- Altares.
- Mapa.
- Save V3.
- GameState.
- PlayerState.
- GameSet.
- Mensagens/eventos.
- HUD e interfaces existentes.

Localize especificamente:

- `ObjectKind::SavePoint`.
- `SavePointActivated`.
- `discovered_save_points`.
- `last_save_point`.
- `vg_approach_save`.

Determine se a arquitetura atual já permite teletransportar o jogador para uma sala arbitrária.

Se existir uma função ou mecanismo de transição adequado, reutilize-o.

Se não existir, proponha uma extensão pequena e segura.

Não copie o carregamento de salas para um segundo sistema.

---

# 5. FASE B — DEFINIR O CONTRATO DE VIAGEM

Uma viagem rápida possui:

- Altar de origem.
- Altar de destino.
- Sala de destino.
- Posição segura de chegada.
- Identificador persistente.
- Nome exibido ao jogador.
- Condições de desbloqueio.
- Estado da viagem.

O sistema deve permitir viajar apenas para destinos válidos e descobertos.

## Regras fundamentais

1. O jogador precisa estar interagindo com um altar válido.
2. O altar de origem precisa estar ativo e acessível.
3. O destino precisa estar descoberto.
4. O destino precisa existir nos dados.
5. A sala de destino precisa existir.
6. A posição de chegada precisa ser válida.
7. O jogador não pode estar morto.
8. A viagem não pode ocorrer durante uma transição anterior.
9. Não podem ocorrer duas viagens simultâneas.
10. O sistema não pode duplicar entidades.

Não permitir teleporte livre por qualquer ponto do mapa.

Não permitir viagem para altares ainda não descobertos.

---

# 6. FASE C — DADOS DOS ALTARES

Analise o esquema atual de `ObjectKind::SavePoint`.

Verifique se já existem informações suficientes para identificar:

- ID.
- Sala.
- Posição.
- Nome.
- Estado de descoberta.
- Ponto de chegada.

Evite duplicar dados já presentes na definição de objetos.

Se necessário, estenda o formato de maneira retrocompatível.

Exemplo conceitual:

```text
SavePoint:
  id: vg_approach_save
  display_name: Gate Approach
  room_id: ...
  arrival_position: ...
  fast_travel_enabled: true
```

Não copie esse exemplo literalmente se o projeto utiliza outra estrutura RON.

## Validação

Detectar:

- IDs duplicados.
- Altar sem sala.
- Sala inexistente.
- Posição inválida.
- Destino inexistente.
- Referências inconsistentes.
- Altar desativado configurado como destino.

A validação deve ocorrer preferencialmente no carregamento de dados.

---

# 7. FASE D — MAIS DE UM ALTAR

Uma rede de viagem rápida precisa de pelo menos dois destinos para ser testada.

O relatório D17 identificou o altar:

`vg_approach_save`

Inspecione as salas existentes e determine uma localização adequada para um segundo altar.

## Critérios

- Sala existente.
- Região acessível.
- Posição segura.
- Sem sobreposição com portas.
- Sem sobreposição com inimigos.
- Sem interferência na geometria.
- Sem alterar progressão de forma indevida.

Adicione um segundo altar apenas se necessário.

Utilize um marcador visual provisório compatível com o altar existente.

Não crie uma nova região inteira apenas para testar viagem rápida.

Não altere a arte gótica principal.

---

# 8. FASE E — INTERFACE DE VIAGEM

O jogador deverá acessar a viagem rápida pela interação com um altar.

## Fluxo recomendado

Ao pressionar D próximo ao altar:

1. Validar a interação.
2. Ativar o descanso e a descoberta conforme a política existente.
3. Exibir opções relacionadas ao altar.
4. Permitir selecionar viagem rápida.
5. Mostrar os destinos descobertos.
6. Permitir selecionar destino.
7. Confirmar a viagem.
8. Executar a transição.

**Atenção:** a D17 já associou D ao descanso e salvamento.

Não remova silenciosamente esse comportamento.

Se for necessário introduzir um menu, preserve o descanso e o salvamento de forma explícita e intuitiva.

Documente a mudança de interação.

## Interface mínima

Mostrar:

- Nome do altar atual.
- Destinos descobertos.
- Destino selecionado.
- Informação da região.
- Opção de confirmar.
- Opção de cancelar.

Preferir UI Bevy já existente.

Não criar uma segunda infraestrutura de menus.

Não gerar ícones definitivos.

---

# 9. FASE F — NAVEGAÇÃO E CONTROLES

Antes de escolher teclas, audite os controles atuais.

Historicamente:

- Setas: movimento.
- Z: pulo.
- X: ataque.
- C: magia.
- A: guarda.
- S: esquiva.
- D: interação.
- I: inventário.
- M: mapa.
- Tab: status.
- Enter: confirmar em interfaces existentes.
- F5: salvar.
- F9: carregar.

Esses controles precisam ser confirmados.

## Política sugerida

- Setas: navegar pelos destinos.
- Enter: confirmar.
- Esc: cancelar.

Se houver conflito com a arquitetura atual, adaptar.

Enquanto a interface estiver aberta:

- Bloquear movimentação.
- Bloquear ataques.
- Bloquear magias.
- Bloquear uso indevido de consumíveis.
- Impedir abertura de interfaces conflitantes.
- Permitir cancelamento seguro.

Ao fechar:

- Restaurar controles.
- Restaurar estado adequado.
- Não deixar o personagem preso em estado de menu.

---

# 10. FASE G — TRANSIÇÃO ENTRE SALAS

Esta é a parte mais importante da D19.

**Não implementar viagem rápida apenas alterando a posição do Transform do jogador.**

Uma viagem entre salas precisa respeitar o ciclo de vida do mundo.

Reutilize a infraestrutura de transições já existente.

## Sequência esperada

1. Receber solicitação de viagem.
2. Validar origem e destino.
3. Verificar estado do jogador.
4. Impedir novas solicitações.
5. Encerrar a interface.
6. Iniciar a transição.
7. Descarregar entidades específicas da sala anterior.
8. Carregar a sala de destino.
9. Atualizar geometria.
10. Atualizar colisões.
11. Posicionar o protagonista no ponto de chegada.
12. Atualizar câmera e limites.
13. Restaurar controles.
14. Liberar novas interações.
15. Confirmar a conclusão da viagem.

A sequência concreta deve respeitar os GameSets existentes.

## Evitar

- Teleporte dentro de paredes.
- Queda infinita.
- Câmera na sala antiga.
- Inimigos duplicados.
- NPCs duplicados.
- Portas duplicadas.
- HUD desatualizado.
- Eventos pendentes de salas anteriores.
- Teleporte repetido por tecla pressionada.
- Personagem preso após falha.

---

# 11. FASE H — POLÍTICA DE SALVAMENTO

Defina explicitamente a relação entre:

- Descanso.
- Descoberta de altar.
- Viagem rápida.
- Último altar utilizado.
- Save V3.

Não presuma que viajar deva salvar automaticamente.

Minha preferência inicial:

**Descansar salva; viajar apenas desloca o jogador.**

Contudo, a D17 pode ter associado descanso e interação de maneira específica.

Inspecione essa implementação antes de decidir.

## Ponto importante

`last_save_point` deve continuar representando o último altar relevante para a política de salvamento ou respawn existente.

Não atualize esse campo simplesmente porque o jogador chegou a outro altar por viagem rápida, sem avaliar as consequências.

Diferencie:

- Último altar descoberto.
- Último altar em que descansou.
- Altar atual.
- Destino de viagem.
- Posição salva.

Se não forem necessários novos campos persistentes, não os crie.

---

# 12. FASE I — SAVE V3

Preservar compatibilidade.

Verifique se `discovered_save_points` já contém tudo o que é necessário para reconstruir a lista de destinos.

Se sim, não alterar a versão do save.

## Testes de persistência

1. Descobrir altar A.
2. Descobrir altar B.
3. Salvar.
4. Encerrar.
5. Carregar.
6. Confirmar A e B descobertos.
7. Viajar de A para B.
8. Confirmar sala correta.
9. Confirmar inventário preservado.
10. Confirmar HP/MP preservados conforme política.
11. Confirmar equipamentos preservados.
12. Confirmar ausência de duplicação.

Utilizar saves temporários de teste.

Não sobrescrever saves reais.

---

# 13. FASE J — INTEGRAÇÃO COM MAPA

A D17 já introduziu marcações ciano para altares descobertos.

Reutilize essa informação.

Se possível, indicar no mapa:

- Altar atual.
- Altares descobertos.
- Destino selecionado.

Não implementar um sistema completo de viagem diretamente pelo mapa nesta etapa.

O mapa pode continuar sendo informativo.

A interação com o altar deve permanecer como mecanismo principal de viagem.

Não revelar automaticamente salas inexploradas.

---

# 14. FASE K — COMBATE E ESTADOS

Verificar que a viagem rápida não pode ocorrer:

- Durante a morte.
- Durante knockback.
- Durante ataque.
- Durante conjuração.
- Durante esquiva.
- Durante transição de sala.
- Durante diálogo incompatível.
- Durante outra viagem.
- Com interface em estado inconsistente.

Não interromper arbitrariamente um combate ativo para teletransportar o jogador.

Se o altar puder ser alcançado durante combate, defina uma política simples.

Preferência:

**Bloquear a viagem quando houver uma condição de combate ativo confiavelmente detectável.**

Não inventar um novo detector global de combate sem necessidade.

Se não existir um estado confiável, documentar a limitação e garantir pelo menos a validação dos estados do personagem.

---

# 15. FASE L — FALHAS E RECUPERAÇÃO

A viagem deve falhar de forma segura.

Cenários:

- Destino removido dos dados.
- Sala inexistente.
- Altar desconhecido.
- Estado inválido.
- Solicitação duplicada.
- Transição já ativa.
- Posição de chegada inválida.

## Regra

Se a validação falhar antes da transição, manter o jogador na sala de origem e exibir feedback.

Se houver falha durante a transição, utilizar o mecanismo de recuperação já existente, se disponível.

Não criar uma recuperação falsa que declare sucesso após uma falha.

Não deixar o jogo em estado sem controles.

---

# 16. FASE M — TESTES UNITÁRIOS

Criar testes para:

- Origem válida.
- Destino descoberto.
- Destino não descoberto.
- Destino inexistente.
- Origem inexistente.
- Mesmo altar como destino.
- Jogador morto.
- Estado inválido.
- Solicitação duplicada.
- Viagem durante transição.
- Cancelamento.
- Lista de destinos.
- Ordenação determinística da lista.
- Ausência de duplicatas.
- IDs inconsistentes.

Não depender exclusivamente de testes visuais.

---

# 17. FASE N — TESTES HEADLESS

Criar teste integrado que:

1. Instancia mundo de teste.
2. Carrega sala A.
3. Descobre altar A.
4. Descobre altar B.
5. Solicita viagem A → B.
6. Processa os sistemas Bevy relevantes.
7. Confirma sala B.
8. Confirma posição de chegada.
9. Confirma colisão válida.
10. Confirma atualização da câmera, quando testável.
11. Confirma jogador único.
12. Confirma ausência de entidades duplicadas.
13. Confirma inventário intacto.
14. Confirma progressão intacta.
15. Confirma retorno B → A.

Criar também testes negativos.

Um teste que apenas verifica a função de validação não é suficiente para declarar a viagem integrada.

---

# 18. FASE O — TESTE MANUAL

Fornecer um roteiro real de teste.

Exemplo:

1. Executar `cargo run`.
2. Encontrar altar A.
3. Interagir com D.
4. Descobrir altar A.
5. Explorar até altar B.
6. Descobrir altar B.
7. Interagir novamente.
8. Abrir viagem rápida.
9. Selecionar altar A.
10. Confirmar.
11. Verificar transição.
12. Verificar posição e câmera.
13. Voltar ao altar B.
14. Salvar.
15. Carregar.
16. Confirmar destinos persistentes.

Adaptar às salas e controles reais.

Não inventar localizações.

Registrar se o teste manual não puder ser executado.

---

# 19. FASE P — REGRESSÕES

Verificar especialmente:

## D17

- Descanso.
- Cura HP/MP.
- Descoberta.
- Salvamento.
- Mapa.
- F5 livre.
- F9.
- Último altar.

## D18

- Inventário I.
- Poções HP/MP.
- Quantidades.
- Enter para usar.
- Persistência.
- Comércio.

## Outros sistemas

- Portas normais.
- Transições.
- NPCs.
- Combate.
- Magias.
- Câmera.
- Progressão.
- Equipamentos.
- Respawn.

Não corrigir sistemas não relacionados sem necessidade.

---

# 20. FASE Q — QUALIDADE

Executar sequencialmente:

```bash
cargo fmt --check
CARGO_BUILD_JOBS=2 cargo check
CARGO_BUILD_JOBS=2 cargo clippy --all-targets -- -D warnings
CARGO_BUILD_JOBS=2 cargo test
```

Não executar processos Cargo pesados simultaneamente.

Não instalar dependências automaticamente.

Se possível, executar o jogo e realizar o teste visual.

Não afirmar que o teste visual passou quando apenas o headless foi executado.

---

# 21. FASE R — DOCUMENTAÇÃO

Criar:

`docs/development/d19/`

Arquivos sugeridos:

```text
D19_AUDITORIA.md
D19_DESIGN.md
D19_IMPLEMENTACAO.md
D19_TESTES.md
D19_RELATORIO_FINAL.md
```

Documentar:

- Estado inicial do Git.
- Arquitetura encontrada.
- Sistema de viagem.
- Dados dos altares.
- Interface.
- Regras de descoberta.
- Política de salvamento.
- Tratamento de erros.
- Testes.
- Limitações.
- Próximos passos.

Atualizar roadmap ou changelog apenas nos trechos necessários, preservando alterações preexistentes.

---

# 22. PRODUÇÃO VISUAL DEFINITIVA — FUTURO

**NÃO EXECUTAR NA D19.**

Preservar o pipeline aprovado:

1. ChatGPT gera imagens individuais de referência.
2. Proprietário aprova.
3. Imagens são salvas em `Md-usario/assets-user/`.
4. Proprietário pode separar manualmente personagens e equipamentos.
5. Hunyuan3D local gera moldes 3D.
6. GLBs originais são preservados.
7. Blender MCP auxilia no ajuste das malhas.
8. Rigging e animações são preparados.
9. Sprites 2D são renderizados.
10. Assets são testados no Bevy.
11. Integração ocorre somente após aprovação.

Aplicável futuramente a:

- Protagonista branco-prateado.
- Equipamentos.
- Inimigos comuns.
- Chefes.
- NPCs.
- Transformações.
- Criaturas especiais.

Preservar:

`Md-usario/assets-user/prota/base/white_mesh.glb`

Não iniciar Hunyuan ou Blender nesta etapa.

---

# 23. CRITÉRIOS DE ACEITAÇÃO

## Funcionalidade

- [ ] Altares existentes auditados.
- [ ] Pelo menos dois altares utilizáveis para teste.
- [ ] Destinos descobertos listados.
- [ ] Destinos desconhecidos bloqueados.
- [ ] Interface funcional.
- [ ] Confirmação de viagem.
- [ ] Cancelamento.
- [ ] Transição entre salas.
- [ ] Posição segura.
- [ ] Câmera correta.
- [ ] Controles restaurados.
- [ ] Sem entidades duplicadas.

## Persistência

- [ ] Descobertas preservadas.
- [ ] Save V3 compatível.
- [ ] Último altar coerente.
- [ ] Inventário preservado.
- [ ] HP/MP preservados conforme política.
- [ ] Equipamentos preservados.
- [ ] F5/F9 sem regressão.

## Testes

- [ ] Unitários.
- [ ] Integração headless.
- [ ] Regressão D17.
- [ ] Regressão D18.
- [ ] Cargo fmt.
- [ ] Cargo check.
- [ ] Clippy sem warnings.
- [ ] Cargo test.
- [ ] Teste manual ou limitação registrada.

## Segurança

- [ ] Sem alterações destrutivas.
- [ ] Sem commit.
- [ ] Sem push.
- [ ] Arte preservada.
- [ ] Hunyuan não utilizado.
- [ ] Blender não utilizado.
- [ ] GLB original preservado.

---

# 24. RELATÓRIO FINAL OBRIGATÓRIO

Ao concluir, responder:

## A. Estado inicial
Branch, HEAD, Git e ambiente.

## B. Auditoria
Sistemas existentes reutilizados.

## C. Arquitetura
Como a viagem rápida se conecta à exploração.

## D. Implementação
Arquivos, eventos, dados e interface.

## E. Regras
Descoberta, validação, estados e salvamento.

## F. Testes
Comandos, resultados e evidências.

## G. Teste manual
Passo a passo para o proprietário.

## H. Persistência
Compatibilidade com Save V3.

## I. Segurança
Arquivos protegidos, Git e arte.

## J. Limitações
O que permaneceu incompleto.

## K. Próxima etapa
Recomendação fundamentada para D20.

---

# 25. DIRETRIZ FINAL

A D19 deve fortalecer a exploração do Dark Castle RPG.

Não queremos apenas um botão de teleporte.

Queremos um sistema integrado ao mundo, ao mapa, às salas, aos altares e ao progresso do jogador.

**Reutilize a infraestrutura existente.**

**Preserve o Save V3.**

**Não quebre a D17 nem a D18.**

**Não retome a produção visual.**

**Não faça commit nem push.**

Execute a auditoria, implemente a viagem rápida, valide os testes e entregue o relatório completo.

**D19 — OBJETIVO FINAL: permitir viagens seguras entre altares descobertos, aproximando o Dark Castle RPG de um Metroidvania completo.**