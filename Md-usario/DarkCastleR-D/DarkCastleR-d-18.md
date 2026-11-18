# DARK CASTLE RPG — MEGA PROMPT D18
## Sistema de Consumíveis Funcionais, Inventário, Combate e Persistência

**Projeto:** Dark Castle RPG  
**Engine:** Bevy 0.20.1  
**Linguagem:** Rust 2024  
**Prioridade:** Gameplay  
**Produção visual definitiva:** FUTURO — NÃO EXECUTAR AGORA

---

# 0. MISSÃO

Implementar um sistema completo, seguro e testável de utilização de itens consumíveis durante o gameplay.

O jogo já possui, conforme o relatório D17:

- Movimentação funcional.
- Combate básico.
- Magias e técnicas.
- Atributos e progressão.
- Inventário e equipamentos.
- Portas e transições de salas.
- Interações.
- NPCs e comércio.
- Salvamento Save V3.
- Altares de salvamento.
- Mapa do mundo.

Entretanto, o uso de consumíveis durante a exploração foi classificado como parcial.

**Não presuma que esteja totalmente ausente.**

A primeira tarefa é descobrir o que já existe e completar a integração.

O objetivo final é permitir que o jogador obtenha, armazene, utilize e salve consumíveis de maneira consistente.

---

# 1. SEGURANÇA DO REPOSITÓRIO

Antes de editar:

1. Identifique a branch atual.
2. Identifique o HEAD.
3. Execute `git status --short`.
4. Examine alterações preexistentes.
5. Verifique o resultado da D17.
6. Preserve arquivos modificados pelo proprietário.

O relatório anterior informou:

- Branch: `art/v1.2-protagonist`
- HEAD: `1115f0d`

Esses valores são históricos e devem ser confirmados novamente.

Não execute comandos Git destrutivos.

Não utilize `git add .`.

Não faça commit.

Não faça push.

Não altere arquivos fora do escopo.

---

# 2. DIRETÓRIOS PROTEGIDOS

Não modificar:

- `Md-usario/`
- `Md-usario/assets-user/`
- `art-src/visual-prototypes/`
- `.claude/`
- `.cargo/`
- `saves/` existentes
- Arquivos Blender originais
- Referências de arte
- Modelos Hunyuan
- Experimentos D9–D16

Preservar especificamente:

`Md-usario/assets-user/prota/base/white_mesh.glb`

Não iniciar Hunyuan.

Não executar Blender MCP.

Não gerar novas imagens.

Não substituir sprites.

Não modificar a aparência oficial do protagonista.

---

# 3. AUDITORIA DOS CONSUMÍVEIS EXISTENTES

Investigue:

- Definições de itens.
- Tipos e categorias.
- Inventário.
- Quantidades.
- Empilhamento.
- Coleta.
- Drops.
- Comércio.
- Uso de itens.
- HP e MP.
- Mensagens/eventos.
- UI.
- Persistência.
- Testes.

Localize as implementações reais.

Verifique se já existem poções ou outros consumíveis definidos em RON.

Identifique se existe uma operação de uso de item que não está conectada ao gameplay.

Verifique se algum sistema já aplica efeitos de consumíveis.

Não crie estruturas duplicadas.

Produza um diagnóstico antes de editar.

---

# 4. CONTRATO FUNCIONAL

O sistema deve permitir:

1. Possuir consumíveis no inventário.
2. Consultar suas quantidades.
3. Selecionar um consumível.
4. Verificar se ele pode ser utilizado.
5. Aplicar seu efeito.
6. Consumir uma unidade.
7. Atualizar os recursos do personagem.
8. Atualizar a interface.
9. Persistir o resultado no salvamento.
10. Restaurar o estado ao carregar.

O consumo deve ser **atômico do ponto de vista da lógica do jogo**.

Um item não pode desaparecer sem aplicar seu efeito quando o uso deveria ser válido.

Um item não pode aplicar o efeito duas vezes por um único comando.

---

# 5. TIPOS INICIAIS

Implemente inicialmente, reutilizando definições existentes quando disponíveis:

## Poção de vida

Recupera HP.

Regras:

- Não ultrapassar HP máximo.
- Não funcionar com quantidade zero.
- Não consumir quando o uso for inválido.
- Não ressuscitar automaticamente um personagem morto.
- Exibir feedback adequado.

## Poção de mana

Recupera MP.

Regras:

- Não ultrapassar MP máximo.
- Não funcionar com quantidade zero.
- Respeitar estados inválidos.
- Atualizar HUD.

## Consumível misto

Se a arquitetura existente suportar naturalmente, permitir um item que recupere HP e MP.

Não implementar um sistema genérico excessivamente complexo apenas para esse terceiro exemplo.

## Outros efeitos

Deixar pontos de extensão para:

- Cura de status negativos.
- Buff temporário.
- Resistência elemental.
- Recuperação de Hearts.
- Itens especiais.

Esses efeitos são futuros, salvo se já estiverem implementados.

---

# 6. DEFINIÇÕES ORIENTADAS A DADOS

Sempre que possível, os consumíveis devem ser definidos nos dados do jogo.

Exemplo conceitual, não um esquema obrigatório:

```text
id: potion_minor
name: Poção Menor
category: Consumable
effect: RestoreHp
amount: 30
stack_limit: 99
```

Adapte ao formato real do projeto.

Não introduza uma segunda fonte de verdade.

Valide:

- IDs duplicados.
- Efeitos desconhecidos.
- Valores inválidos.
- Quantidades negativas.
- Limites de empilhamento.
- Referências inexistentes.

Evite colocar os valores das poções diretamente no código de interface.

---

# 7. INTEGRAÇÃO COM O INVENTÁRIO

O inventário deve continuar sendo responsável pelas quantidades.

O sistema de efeitos não deve manter um contador independente.

Verifique:

- Item existente.
- Quantidade disponível.
- Condições de uso.
- Remoção de uma unidade.
- Atualização da UI.
- Persistência.

Não permita quantidades negativas.

Não permita duplicação por entradas repetidas.

Não permita que o jogador use um item que já foi removido.

Se o inventário já possuir operações transacionais ou equivalentes, reutilize-as.

---

# 8. REGRAS DE UTILIZAÇÃO

Defina explicitamente os estados em que consumíveis podem ser usados.

Considere:

- Idle.
- Walking.
- Running.
- Jumping.
- Falling.
- Attacking.
- Casting.
- Guarding.
- Rolling.
- TakingDamage.
- Dead.
- StatusScreen.
- Transição de sala.

Não decida arbitrariamente que todos os estados são permitidos.

Analise a máquina de estados existente.

Para a primeira versão, prefira uma política simples e consistente.

Se o uso durante ataques, esquivas ou conjuração provocar conflitos, bloqueie nesses estados e documente.

Evite introduzir animações novas nesta fase.

---

# 9. EFEITOS E RECURSOS

A aplicação de consumíveis deve respeitar os atributos derivados existentes.

Exemplo:

HP atual = 70  
HP máximo = 100  
Poção recupera = 50

Resultado esperado:

HP final = 100.

Não ultrapassar o máximo.

Se o recurso já estiver cheio, prefira impedir o consumo de um item cuja única função seria recuperar esse recurso.

Para itens mistos, avalie todos os efeitos antes de decidir se existe benefício.

Não consumir o item se nenhum efeito puder ser aplicado e essa for a política definida.

Documente essa decisão.

---

# 10. INTEGRAÇÃO COM COMBATE

Verifique a interação com:

- Invulnerabilidade.
- Dano recebido.
- Morte.
- Knockback.
- Ataques.
- Técnicas.
- Conjuração.
- Cooldowns.
- Estados do personagem.

Evite conflitos de ordem de execução.

Um consumível não pode permitir que o personagem execute ações proibidas pela máquina de estados.

Não reescreva o sistema de combate.

Não altere fórmulas de dano.

Não modifique o parser de comandos especiais sem necessidade.

---

# 11. CONTROLES E INTERFACE

Investigue primeiro os controles existentes.

Não sobrescreva teclas já utilizadas.

O projeto historicamente utiliza:

- Setas para movimento.
- Z para pular.
- X para atacar.
- C para magia.
- A para guardar.
- S para esquiva.
- D para interação.
- Tab para status.
- M para mapa.
- F5/F9 para salvar/carregar.

Confirme os controles atuais.

Implemente uma maneira simples de usar consumíveis.

Preferência:

**Selecionar o item no inventário ou menu existente e acionar Usar.**

Se já houver interface de inventário funcional, integre nela.

Não crie um segundo menu de inventário.

Não introduza uma nova tecla global sem verificar conflitos.

A interface deve indicar:

- Nome do item.
- Quantidade.
- Efeito.
- Disponibilidade.
- Resultado do uso.
- Motivo de falha.

Utilize elementos visuais provisórios existentes.

---

# 12. FEEDBACK

O jogador deve receber feedback para:

- Item utilizado.
- HP recuperado.
- MP recuperado.
- Recurso cheio.
- Quantidade insuficiente.
- Estado inválido.
- Item não utilizável.
- Item inexistente.

Reutilize o sistema atual de mensagens ou HUD.

Evite criar um novo sistema de notificações.

Não gere sprites ou efeitos definitivos.

---

# 13. INTEGRAÇÃO COM COLETA E COMÉRCIO

Verifique se itens consumíveis podem ser obtidos por:

- Drops.
- Recompensas.
- Baús.
- Comércio.
- Dados iniciais do jogador.

Escolha o caminho mais simples já existente para permitir um teste completo.

O jogador deve conseguir obter um consumível sem editar manualmente o inventário em tempo de execução.

Não implemente um novo sistema de comércio ou drops se o atual já funcionar.

Não altere o balanceamento global.

---

# 14. SALVAMENTO E COMPATIBILIDADE

O relatório D17 informou que Save V3 permanece em uso.

Verifique o formato real.

A implementação deve preservar:

- Inventário.
- Quantidades.
- Equipamentos.
- HP.
- MP.
- Progressão.
- Mundo.
- Altares descobertos.
- Último altar utilizado.

**Não alterar a versão do save se o esquema existente já representar corretamente os consumíveis.**

Não modificar saves reais do proprietário.

Utilize fixtures ou diretórios temporários controlados para testes.

Teste o ciclo:

1. Carregar jogo.
2. Obter poção.
3. Salvar.
4. Usar poção.
5. Salvar novamente.
6. Encerrar.
7. Reabrir.
8. Verificar HP/MP.
9. Verificar quantidade.
10. Confirmar ausência de duplicação.

Não faça migração de save sem necessidade comprovada.

---

# 15. TESTES UNITÁRIOS

Criar testes para:

- Uso válido.
- Uso sem quantidade.
- HP cheio.
- MP cheio.
- Cura parcial.
- Cura limitada ao máximo.
- Personagem morto.
- Estado inválido.
- Item desconhecido.
- Item não consumível.
- Quantidade exatamente um.
- Uso repetido.
- Item com múltiplos efeitos, se implementado.

Não testar apenas a função de recuperação de HP.

Teste também a redução de quantidade e a validação de uso.

---

# 16. TESTES DE INTEGRAÇÃO

Criar ou ampliar testes headless para validar:

- Inventário → ação de uso.
- Ação → efeito.
- Efeito → atributos.
- Atributos → HUD ou estado de apresentação.
- Inventário → persistência.
- Save → Load.
- Coleta ou comércio → inventário → uso.

Verifique o fluxo real de mensagens e GameSets.

Não considere integração concluída apenas com testes de funções puras.

---

# 17. TESTE MANUAL

Prepare um roteiro reproduzível.

Exemplo:

1. Executar `cargo run`.
2. Entrar no jogo.
3. Obter uma poção.
4. Receber dano.
5. Abrir inventário.
6. Selecionar poção.
7. Utilizar.
8. Verificar HP.
9. Verificar quantidade.
10. Salvar.
11. Carregar.
12. Confirmar persistência.

Adapte o roteiro à interface e aos controles efetivamente implementados.

Não invente atalhos.

Se a interface não puder ser testada visualmente, registre a limitação.

---

# 18. VERIFICAÇÕES DE QUALIDADE

Execute sequencialmente:

```bash
cargo fmt --check
CARGO_BUILD_JOBS=2 cargo check
CARGO_BUILD_JOBS=2 cargo clippy --all-targets -- -D warnings
CARGO_BUILD_JOBS=2 cargo test
```

Não execute vários processos Cargo pesados simultaneamente.

Se um comando falhar, registre a falha.

Não declare testes aprovados sem evidência.

Execute testes de regressão em:

- Combate.
- Magias.
- Inventário.
- Equipamentos.
- Progressão.
- Salvamento.
- Altares D17.
- Transições de salas.

---

# 19. DOCUMENTAÇÃO

Crie documentação específica em:

`docs/development/d18/`

Ou utilize a convenção real do repositório, caso exista outra.

Arquivos sugeridos:

```text
D18_AUDITORIA.md
D18_DESIGN.md
D18_IMPLEMENTACAO.md
D18_TESTES.md
D18_RELATORIO_FINAL.md
```

Documente:

- Estado inicial.
- Sistemas reutilizados.
- Decisões.
- Arquivos modificados.
- Dados de consumíveis.
- Política de estados.
- Integração.
- Persistência.
- Testes.
- Limitações.
- Próximas etapas.

Não reescreva documentação histórica desnecessariamente.

---

# 20. PLANEJAMENTO D19

Após concluir a D18, recomende a próxima entrega.

Preferência inicial:

**D19 — Viagem rápida entre altares descobertos.**

A D17 já implementou:

- Descoberta de altares.
- Último altar utilizado.
- Persistência.
- Marcação no mapa.

A D19 poderá aproveitar essas informações para permitir deslocamento entre pontos de salvamento já descobertos.

Mas não implemente viagem rápida nesta D18.

---

# 21. PRODUÇÃO VISUAL FUTURA — PRESERVAR

A produção visual definitiva permanece:

**FUTURO — NÃO EXECUTAR AGORA.**

O pipeline acordado é:

1. Definir personagem, inimigo, chefe, NPC ou equipamento.
2. Gerar imagem de referência no ChatGPT.
3. Aprovar visualmente.
4. Salvar em `Md-usario/assets-user/`.
5. Separar manualmente imagens e equipamentos.
6. Gerar molde 3D no Hunyuan3D local.
7. Preservar GLB original.
8. Importar no Blender.
9. Ajustar geometria, escala, materiais e rigging.
10. Criar animações.
11. Renderizar sprites 2D.
12. Testar no Bevy.
13. Integrar somente após aprovação.

O protótipo branco-prateado está preservado em:

`Md-usario/assets-user/prota/base/white_mesh.glb`

Não utilizar esse arquivo na D18.

Não iniciar Hunyuan.

Não modificar Blender MCP.

Não produzir arte definitiva de inimigos, chefes ou NPCs.

---

# 22. CRITÉRIOS DE ACEITAÇÃO

## Funcionalidade

- [ ] Consumíveis existentes auditados.
- [ ] Uso funcional implementado.
- [ ] HP recuperado corretamente.
- [ ] MP recuperado corretamente.
- [ ] Quantidade decrementada.
- [ ] Uso inválido bloqueado.
- [ ] Sem duplicação.
- [ ] Feedback implementado.
- [ ] Interface integrada.
- [ ] Item obtido por caminho jogável existente.

## Persistência

- [ ] Inventário salvo.
- [ ] Quantidades restauradas.
- [ ] HP/MP restaurados.
- [ ] Save V3 preservado quando possível.
- [ ] Altares D17 sem regressão.

## Qualidade

- [ ] Formatação aprovada.
- [ ] Compilação aprovada.
- [ ] Clippy aprovado.
- [ ] Testes aprovados.
- [ ] Teste de integração aprovado.
- [ ] Teste manual executado ou limitação documentada.

## Segurança

- [ ] Git preservado.
- [ ] Sem commit.
- [ ] Sem push.
- [ ] Arte preservada.
- [ ] Hunyuan não utilizado.
- [ ] GLB original preservado.

Não marque itens como concluídos sem evidência.

---

# 23. RELATÓRIO FINAL

Ao terminar, responda com:

## A. Estado inicial
Branch, HEAD e estado Git.

## B. Auditoria
O que já existia e o que faltava.

## C. Arquitetura
Como o sistema de consumíveis foi integrado.

## D. Implementação
Arquivos, dados, mensagens e regras.

## E. Interface
Como o jogador utiliza consumíveis.

## F. Persistência
Como o Save V3 foi preservado.

## G. Testes
Comandos, resultados e evidências.

## H. Teste manual
Passo a passo para reproduzir.

## I. Limitações
O que não foi implementado.

## J. Segurança
Confirmação de preservação dos arquivos e ausência de commit/push.

## K. Próxima etapa
Recomendação fundamentada para D19.

---

# 24. DIRETRIZ FINAL

O Dark Castle RPG deve evoluir como um jogo integrado, não como um conjunto de demonstrações técnicas.

A D18 deve transformar os consumíveis em uma mecânica real do gameplay.

**Não reescreva sistemas existentes.**

**Não implemente viagem rápida ainda.**

**Não retome a produção visual.**

**Não faça commit nem push.**

O sucesso da D18 exige que o jogador consiga obter, utilizar e salvar consumíveis durante uma sessão real de jogo, com testes que comprovem o funcionamento.

Execute a auditoria, implemente o escopo necessário, valide e entregue o relatório completo.