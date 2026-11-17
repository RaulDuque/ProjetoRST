# DARK CASTLE RPG — MEGA PROMPT D17
## Retomada do Desenvolvimento, Auditoria Técnica e Evolução do Gameplay

**Projeto:** Dark Castle RPG  
**Linguagem:** Rust 2024  
**Engine:** Bevy 0.20.1  
**Gênero:** Action RPG / Metroidvania 2D  
**Referência de gameplay:** Castlevania: Symphony of the Night  
**Responsável pela implementação:** Claude Code  
**Direção técnica e planejamento:** ChatGPT + proprietário

---

# 0. MISSÃO PRINCIPAL

A partir desta etapa, voltaremos a priorizar o desenvolvimento funcional do Dark Castle RPG.

As etapas D11–D16 concentraram esforços em pesquisa visual, personagens, Blender, equipamentos modulares e geração 3D.

Esses experimentos produziram resultados importantes, mas **não queremos continuar investindo na aparência final enquanto os sistemas fundamentais do jogo estiverem incompletos**.

Sua missão na D17 é:

1. Inspecionar o estado real do repositório.
2. Auditar a arquitetura e os sistemas implementados.
3. Comparar código, documentação, testes e roadmap.
4. Identificar lacunas e problemas de integração.
5. Selecionar uma etapa funcional de alto valor.
6. Implementá-la com escopo controlado.
7. Validar seu funcionamento por testes e, quando possível, execução real.
8. Documentar o resultado.
9. Preparar recomendações para a D18.

**Não transforme esta tarefa em outra etapa de produção visual.**

O objetivo é aproximar o projeto de um jogo completo e jogável.

---

# 1. CONTEXTO DO PROJETO

O Dark Castle RPG é um Metroidvania 2D com:

- Exploração de ambientes góticos.
- Movimentação lateral.
- Combate físico.
- Magias.
- Técnicas especiais.
- Comandos executados por sequências de botões.
- Atributos.
- Experiência e níveis.
- Equipamentos.
- Inventário.
- Inimigos.
- Chefes.
- NPCs.
- Progressão de mundo.
- Salvamento e carregamento.
- Futuras transformações do protagonista.

A arquitetura utiliza módulos e plugins Bevy.

Entre os sistemas documentados historicamente estão:

```text
src/
├── app/
├── core/
├── input/
├── player/
├── combat/
├── skills/
├── magic/
├── enemies/
├── inventory/
├── world/
├── save/
└── ui/
```

Essas informações representam o contexto histórico.

**O repositório atual é a fonte de verdade.**

Não presuma que arquivos, APIs ou sistemas continuem exatamente iguais.

---

# 2. REGRAS ABSOLUTAS DE SEGURANÇA

Antes de qualquer alteração:

1. Identifique a raiz do repositório.
2. Identifique a branch atual.
3. Identifique o HEAD.
4. Execute `git status --short`.
5. Registre alterações não commitadas.
6. Registre arquivos não rastreados relevantes.
7. Verifique diferenças existentes.
8. Não sobrescreva trabalho anterior.

É proibido executar:

```bash
git reset --hard
git clean -fd
git checkout -- .
git restore .
git add .
```

Não utilize comandos equivalentes com efeito destrutivo.

**Não faça commit nem push.**

Não crie branches automaticamente.

Não faça rebase, merge ou cherry-pick sem autorização.

Não modifique arquivos fora do escopo funcional selecionado.

Se houver alterações preexistentes nos arquivos necessários, examine-as antes de editar.

Se não conseguir preservar essas alterações com segurança, interrompa a edição e explique o bloqueio.

---

# 3. DIRETÓRIOS PROTEGIDOS

Preserve:

- `Md-usario/`
- `assets-user/`, se existir em outro local
- `art-src/visual-prototypes/`
- `.claude/`
- `.cargo/`
- `saves/`
- Arquivos Blender originais
- Resultados Hunyuan
- Referências visuais
- Relatórios históricos
- Assets de experimentos D11–D16

Não reorganize esses diretórios.

Não renomeie arquivos.

Não mova referências.

Não exclua versões antigas.

Não atualize a arte principal.

A pasta `assets/` do Bevy só poderá receber alterações diretamente necessárias à funcionalidade escolhida, como dados RON ou mapas de teste, preservando os recursos existentes.

---

# 4. DECISÃO ESTRATÉGICA — ARTE DEFINITIVA ADIADA

## Estado: FUTURO — NÃO EXECUTAR

O proprietário decidiu interromper temporariamente a geração de modelos 3D.

O foco agora será construir o jogo.

Não executar nesta fase:

- Geração de imagens com IA.
- Geração de modelos Hunyuan3D.
- Retopologia do protagonista.
- Rigging definitivo.
- Texturização definitiva.
- Criação de sprites finais.
- Remodelagem de inimigos.
- Remodelagem de chefes.
- Remodelagem de NPCs.
- Reformulação visual do protagonista.
- Substituição de placeholders.
- Alterações estéticas sem necessidade funcional.

O visual atual deverá continuar funcionando.

Use placeholders existentes sempre que possível.

Se algum sistema precisar de um novo elemento visual, utilize representação provisória simples, claramente identificada como placeholder.

---

# 5. PRESERVAR O PROTAGONISTA EXPERIMENTAL

Existe um arquivo de referência experimental:

```text
Md-usario/assets-user/prota/base/white_mesh.glb
```

Esse arquivo representa o primeiro protótipo 3D do protagonista de cabelo branco-prateado.

**Não modificar esse arquivo.**

Não mover.

Não converter.

Não otimizar.

Não abrir automaticamente no Blender.

Não substituir o protagonista atual do jogo por ele.

Não iniciar Hunyuan3D.

Não instalar dependências de IA.

A identidade visual branco-prateada está reservada para a futura produção definitiva, sujeita à aprovação visual e técnica.

---

# 6. FASE A — AUDITORIA INICIAL DO REPOSITÓRIO

Execute uma inspeção abrangente, mas não destrutiva.

Identifique:

- Branch.
- HEAD.
- Estado Git.
- Arquivos modificados.
- Arquivos não rastreados.
- Estrutura atual.
- Dependências.
- Versão do Rust.
- Versão do Bevy.
- Features do projeto.
- Exemplos executáveis.
- Testes existentes.
- Documentação.
- Roadmap.

Consulte os documentos disponíveis, especialmente:

```text
README.md
Cargo.toml
docs/ARCHITECTURE.md
docs/GAME_DESIGN.md
docs/ROADMAP.md
docs/COMBAT_SYSTEM.md
docs/INPUT_SYSTEM.md
docs/SKILL_SYSTEM.md
docs/MAGIC_SYSTEM.md
docs/ENEMY_SYSTEM.md
docs/EQUIPMENT_SYSTEM.md
docs/SAVE_SYSTEM.md
docs/PROGRESSION_SYSTEM.md
docs/DECISIONS.md
```

Se algum arquivo não existir, registre a ausência.

Não invente seu conteúdo.

Consulte também os relatórios de etapas anteriores quando forem relevantes.

---

# 7. FASE B — AUDITORIA DOS SISTEMAS

Analise cada sistema considerando:

1. O que está implementado.
2. O que está parcialmente implementado.
3. O que está apenas documentado.
4. O que está ausente.
5. Quais testes existem.
6. Quais integrações funcionam.
7. Quais problemas podem comprometer o gameplay.

## 7.1 Movimentação

Verifique:

- Andar.
- Correr.
- Pular.
- Altura variável do pulo.
- Coyote time.
- Queda.
- Gravidade.
- Colisão.
- Plataformas atravessáveis.
- Agachamento.
- Guarda.
- Esquiva.
- Invulnerabilidade.
- Recuo.
- Morte.
- Respawn.
- Máquina de estados.
- Câmera.

Investigue situações em que o jogador pode ficar preso ou executar ações incompatíveis.

## 7.2 Combate

Verifique:

- Ataque básico.
- Hitboxes.
- Hurtboxes.
- Dano físico.
- Dano mágico.
- Defesa.
- Resistências.
- Críticos.
- Precisão.
- Invulnerabilidade.
- Knockback.
- Morte de inimigos.
- Combos.
- Efeitos de status.

Não presuma que combos ou efeitos de status já existam.

## 7.3 Comandos especiais

Verifique:

- Buffer.
- Orientação relativa.
- Diagonais.
- Parser.
- Prioridades.
- Desempate.
- Cooldowns.
- Custos.
- Cancelamento.
- Feedback.
- Estados inválidos.
- Sequências parcialmente reconhecidas.

## 7.4 Técnicas

Verifique:

- Definições por dados.
- Desbloqueio.
- Execução.
- Custos.
- Cooldowns.
- Técnicas físicas.
- Técnicas defensivas.
- Técnicas de movimento.
- Técnicas por categoria de arma.

## 7.5 Magias

Verifique:

- Elementos.
- MP.
- Conjuração.
- Projéteis.
- Colisão.
- Cura.
- Interrupção.
- Efeitos em área.
- Efeitos contínuos.
- Magias de suporte.

## 7.6 Progressão

Verifique:

- Experiência.
- Níveis.
- Atributos.
- Pontos distribuíveis.
- Atributos derivados.
- Recursos.
- Desbloqueios.
- Persistência.

## 7.7 Inventário e equipamentos

Verifique:

- Definições de itens.
- Categorias.
- Raridade.
- Quantidades.
- Slots.
- Equipar.
- Desequipar.
- Modificadores.
- Atributos derivados.
- Persistência.
- UI.

## 7.8 Inimigos

Verifique:

- Spawn.
- Patrulha.
- Perseguição.
- Ataque.
- Distância de detecção.
- Estados de IA.
- Dano.
- Morte.
- EXP.
- Drops.
- Respawn.
- Comportamentos especiais.

## 7.9 Mundo

Verifique:

- Salas.
- Geometria.
- Colisão.
- Plataformas.
- Portas.
- Transições.
- Persistência de progresso.
- Interações.
- Segredos.
- Baús.
- Pontos de salvamento.
- Limites da câmera.

## 7.10 Interface

Verifique:

- HUD.
- HP.
- MP.
- Hearts.
- EXP.
- Nível.
- Cooldowns.
- Feedback de comandos.
- Inventário.
- Equipamentos.
- Status.
- Mensagens.
- Menus.

## 7.11 Salvamento

Verifique:

- Formato atual.
- Compatibilidade com Save V3, se ainda aplicável.
- Serialização.
- Carregamento.
- Validação.
- Recuperação de erros.
- Estado do jogador.
- Estado do mundo.
- Inventário.
- Equipamentos.
- Progressão.

Não quebre a compatibilidade de saves existentes.

---

# 8. FASE C — MAPA DE DEPENDÊNCIAS DO GAMEPLAY

Construa um mapa técnico mostrando como os sistemas se conectam.

Exemplo conceitual:

```text
Input
  ↓
Player / Commands
  ↓
Combat / Skills / Magic
  ↓
Damage / Enemy AI
  ↓
Death / EXP / Drops
  ↓
Inventory / Progression
  ↓
Save / World Progress
  ↓
UI / Feedback
```

Verifique se os eventos realmente conectam esses sistemas.

Identifique fluxos interrompidos.

Exemplos:

- Inimigo morre, mas não concede EXP.
- Item existe nos dados, mas não pode ser coletado.
- Equipamento altera atributos, mas o HUD não atualiza.
- Porta existe, mas não troca de sala.
- Save grava dados, mas não restaura determinado estado.
- Inimigo ataca, mas a hitbox não funciona.

Esses exemplos são hipóteses de auditoria, não defeitos confirmados.

Não declare um problema sem evidência.

---

# 9. FASE D — DEFINIR O CICLO JOGÁVEL MÍNIMO

Nosso objetivo de médio prazo é permitir que o jogador:

1. Inicie o jogo.
2. Controle o protagonista.
3. Explore uma sala.
4. Encontre um inimigo.
5. Entre em combate.
6. Derrote o inimigo.
7. Receba EXP ou recompensa.
8. Colete um item.
9. Equipe ou utilize esse item.
10. Encontre uma passagem.
11. Acesse outra sala.
12. Encontre um ponto de salvamento.
13. Salve o progresso.
14. Encerre o jogo.
15. Retorne e continue de onde parou.

Esse é o **ciclo vertical de gameplay** que queremos construir progressivamente.

Não implemente os quinze passos de uma vez.

Determine quais já funcionam e quais impedem completar esse ciclo.

---

# 10. FASE E — PRIORIZAÇÃO

Após a auditoria, produza uma matriz de prioridades.

Avalie:

- Impacto no gameplay.
- Dependências.
- Complexidade.
- Risco de regressão.
- Testabilidade.
- Valor para o próximo marco jogável.

Considere esta ordem inicial, ajustando conforme evidências:

### P0 — Estabilidade e bloqueadores

Erros que impedem compilar, iniciar, movimentar, combater ou salvar.

### P1 — Exploração funcional

Interações, portas, transições, progressão de salas e persistência.

### P2 — Recompensas e equipamentos

Coleta, inventário, equipar, desequipar e atributos.

### P3 — Combate mais completo

Combos, variedade de ataques, efeitos de status e feedback.

### P4 — Inimigos e chefes

IA, comportamentos especiais, fases, recompensas e integração ao mundo.

### P5 — NPCs e progressão narrativa

Diálogos, comerciantes, missões e eventos.

### P6 — Polimento

Balanceamento, experiência de usuário e qualidade geral.

### FUTURO — Arte definitiva

Somente após autorização.

**A ordem não é uma autorização para implementar todas as categorias.**

---

# 11. FASE F — ESCOLHER UMA ENTREGA REAL PARA D17

Selecione uma única entrega funcional principal, baseada no diagnóstico.

Minha preferência inicial é:

**Interações com o mundo + portas/transições de sala**, caso ainda estejam incompletas.

Motivo:

Um Metroidvania precisa permitir exploração progressiva, não apenas combate em uma sala isolada.

Entretanto, se a auditoria demonstrar que outro bloqueador é mais importante, priorize esse bloqueador e justifique.

A entrega escolhida deve ser pequena o suficiente para:

- Implementar.
- Testar.
- Integrar.
- Documentar.
- Revisar.

Não tente completar todo o roadmap na D17.

---

# 12. ESCOPO SUGERIDO — INTERAÇÕES E TRANSIÇÕES

**Executar esta seção somente se ela for selecionada após a auditoria.**

## 12.1 Sistema de interação

Permitir que o jogador interaja com entidades próximas.

Possíveis entidades:

- Portas.
- Alavancas.
- Baús.
- NPCs.
- Pontos de salvamento.

Nesta fase, implemente apenas os tipos necessários à entrega selecionada.

O sistema deve:

- Detectar entidades interagíveis.
- Verificar distância.
- Exigir estado válido.
- Evitar múltiplas ativações.
- Oferecer feedback provisório.
- Respeitar a arquitetura de eventos existente.

Não crie uma segunda arquitetura de mensagens se já existir uma solução adequada.

## 12.2 Portas

Uma porta deve possuir:

- Identificador.
- Sala de origem.
- Sala de destino.
- Ponto de chegada.
- Condições de acesso, se aplicáveis.
- Estado de bloqueio, quando necessário.

## 12.3 Transição

Ao atravessar uma porta:

1. Validar a interação.
2. Identificar o destino.
3. Preparar a troca.
4. Descarregar entidades específicas da sala anterior.
5. Carregar a sala de destino.
6. Posicionar o jogador.
7. Atualizar geometria e colisão.
8. Atualizar limites da câmera.
9. Preservar estado persistente.
10. Reabilitar controles.

Evitar:

- Teleporte para dentro de paredes.
- Spawn dentro do chão.
- Portas em loop.
- Entidades duplicadas.
- Inimigos antigos permanecendo indevidamente.
- Câmera presa na sala anterior.
- Perda de estado do jogador.

## 12.4 Dados

Prefira definições em RON ou no formato já utilizado pelo projeto.

Não codifique cada porta diretamente no sistema Rust.

Valide destinos inexistentes e identificadores duplicados.

## 12.5 Testes

Crie testes para:

- Porta válida.
- Porta bloqueada.
- Destino inexistente.
- Interação fora do alcance.
- Estado inválido do jogador.
- Troca de sala.
- Posição de chegada.
- Colisão após transição.
- Câmera.
- Persistência do progresso.

Use fixtures ou salas de teste isoladas quando necessário.

---

# 13. ARQUITETURA E COMPATIBILIDADE

Preserve a separação entre:

- Regras puras.
- Sistemas Bevy.
- Dados.
- Eventos.
- Apresentação.

Evite lógica de negócio acoplada diretamente ao HUD.

Evite módulos alterando componentes de outros subsistemas sem necessidade.

Reutilize:

- GameState.
- GameSet.
- Recursos existentes.
- Mensagens/eventos existentes.
- Carregador RON.
- Sistemas de validação.

Se a arquitetura atual diferir da documentação histórica, siga o código real e registre a divergência.

Não atualize o Bevy nesta etapa.

Não atualize dependências sem necessidade comprovada.

---

# 14. TESTES E QUALIDADE

Execute, quando aplicável:

```bash
cargo fmt --check
cargo check
cargo clippy --all-targets
cargo test
```

Para reduzir pressão de memória:

```bash
CARGO_BUILD_JOBS=2 cargo check
CARGO_BUILD_JOBS=2 cargo test
```

Não execute múltiplos processos Cargo pesados simultaneamente.

Se houver bibliotecas de sistema ausentes, identifique o problema.

Não instale pacotes com sudo automaticamente.

Não declare sucesso de compilação sem execução real.

## Testes adicionais

Verifique:

- Execução headless, se disponível.
- Exemplo autoplay, se aplicável.
- Inicialização do jogo.
- Funcionamento da funcionalidade nova.
- Regressões em movimentação.
- Regressões em combate.
- Regressões no save.
- Regressões na UI.

Diferencie claramente testes automatizados de testes visuais ou manuais.

Se não puder abrir a janela do jogo, registre a limitação.

---

# 15. PROTEÇÃO DE PERFORMANCE

O computador possui aproximadamente:

- RTX 4050 Laptop 6 GB.
- 14,6 GB de RAM.
- Intel Core i5-13420H.

Durante a D17:

- Não iniciar Hunyuan3D.
- Não carregar checkpoints de IA.
- Não executar Blender em renderização pesada.
- Não executar múltiplos builds simultaneamente.
- Não iniciar testes de estresse desnecessários.
- Não consumir toda a RAM com tarefas paralelas.

Se o Cursor ou o agente estiverem sob pressão de memória, reduza a concorrência.

Não finalize processos do usuário sem autorização.

---

# 16. DOCUMENTAÇÃO DA D17

Crie um diretório de documentação específico para esta etapa, preferencialmente em:

```text
docs/development/d17/
```

Se já houver convenção de localização para relatórios de desenvolvimento, respeite-a.

Documentos desejados:

```text
D17_AUDITORIA.md
D17_PRIORIDADES.md
D17_IMPLEMENTACAO.md
D17_TESTES.md
D17_RELATORIO_FINAL.md
D17_ROADMAP_FUTURO.md
```

Evite duplicação desnecessária.

## Auditoria

Documentar:

- Estado inicial.
- HEAD.
- Branch.
- Sistemas existentes.
- Lacunas.
- Problemas confirmados.
- Evidências.
- Riscos.

## Prioridades

Documentar:

- Bloqueadores.
- Próximas entregas.
- Dependências.
- Justificativa da escolha.

## Implementação

Documentar:

- Escopo escolhido.
- Arquivos alterados.
- Estruturas criadas.
- Eventos.
- Dados.
- Integração.
- Limitações.

## Testes

Documentar:

- Comandos executados.
- Resultados.
- Falhas.
- Testes não executados.
- Evidências.

## Relatório final

Documentar o resultado completo e os próximos passos.

---

# 17. PLANO FUTURO — PRODUÇÃO VISUAL DEFINITIVA

**ESTADO: FUTURO / ADIADO / NÃO EXECUTAR NA D17**

Esta seção deve permanecer registrada no planejamento do projeto.

O proprietário estabeleceu um fluxo de criação visual que deverá ser reutilizado quando os principais sistemas do jogo estiverem funcionais.

## 17.1 Fluxo aprovado de trabalho

### Passo 1 — Definição do personagem ou equipamento

Antes de gerar qualquer imagem, definir:

- ID.
- Nome.
- Categoria.
- Função no jogo.
- Proporções.
- Estilo.
- Equipamentos.
- Animações necessárias.
- Variações.
- Referências visuais.

### Passo 2 — Geração da imagem pelo ChatGPT

O ChatGPT será utilizado para gerar imagens de referência.

Exemplos:

- Protagonista.
- Inimigos comuns.
- Chefes.
- NPCs.
- Espadas.
- Escudos.
- Armaduras.
- Capas.
- Botas.
- Capacetes.
- Transformações.
- Criaturas especiais.

As imagens deverão representar os elementos de maneira individual sempre que possível.

### Passo 3 — Aprovação humana

O proprietário analisará a imagem.

Poderá solicitar:

- Alterações no rosto.
- Alterações no cabelo.
- Alterações na roupa.
- Mudanças nas cores.
- Ajustes de proporção.
- Remoção de acessórios.
- Novas vistas.

Somente imagens aprovadas poderão avançar para modelagem.

### Passo 4 — Armazenamento no projeto

Salvar referências aprovadas em:

```text
Md-usario/assets-user/
```

Preservar a organização existente.

Categorias previstas:

```text
protagonista/
equip/
prota/
inimigos/
chefes/
npcs/
```

As três últimas são propostas futuras: não criá-las automaticamente agora.

Cada referência deverá possuir identificação e versão.

### Passo 5 — Preparação manual da imagem

O proprietário poderá separar manualmente as imagens geradas pelo ChatGPT.

Exemplos:

- Separar espada.
- Separar capa.
- Separar armadura.
- Separar botas.
- Remover fundo.
- Ajustar enquadramento.
- Preparar referência isolada.

Esse processo manual faz parte do fluxo escolhido.

Não exigir automação completa.

### Passo 6 — Geração 3D local

Utilizar futuramente a instalação existente:

```text
~/AI-3D/
```

Ferramenta:

Hunyuan3D-2mini.

Hardware:

RTX 4050 Laptop 6 GB.

O modelo já foi testado com sucesso para geração de geometria.

Não presumir que todos os modelos ou texturas funcionarão com essa VRAM.

### Passo 7 — Preservação do GLB

Salvar o modelo gerado sem sobrescrever o original.

Registrar:

- Imagem de entrada.
- Modelo utilizado.
- Seed.
- Parâmetros.
- Data.
- Versão.
- Arquivo GLB.
- Problemas conhecidos.

### Passo 8 — Blender e Blender MCP

Importar o GLB no Blender.

Verificar:

- Escala.
- Orientação.
- Topologia.
- Normais.
- Componentes.
- Materiais.
- Proporções.
- Rigging.
- Skinning.
- Encaixes.

Corrigir o que for necessário.

Não presumir que o modelo Hunyuan já esteja pronto para animação.

### Passo 9 — Animações

Preparar as animações exigidas pelo jogo.

Para inimigos comuns:

- Idle.
- Walk.
- Attack.
- Hit.
- Death.

Para chefes:

- Idle.
- Locomoção.
- Ataques.
- Técnicas especiais.
- Transições de fase.
- Hit.
- Death.

Para NPCs:

- Idle.
- Walk, quando aplicável.
- Interação.
- Gestos.
- Animações específicas.

Essas listas são referências iniciais e deverão ser adaptadas a cada entidade.

### Passo 10 — Exportação para sprites

Renderizar as animações em sprites 2D.

Preparar:

- Frames.
- Atlas.
- Pivot.
- Escala.
- Transparência.
- Metadados.
- Sincronização de animações.
- Compatibilidade com Bevy.

### Passo 11 — Integração experimental

Testar os novos sprites em laboratório isolado.

Validar:

- Colisão.
- Hitboxes.
- Hurtboxes.
- Animações.
- Direção.
- Escala.
- Performance.
- Legibilidade visual.

### Passo 12 — Aprovação final

Somente substituir placeholders quando:

- O modelo estiver aprovado.
- As animações funcionarem.
- Os testes passarem.
- A identidade visual estiver consistente.
- O proprietário autorizar a integração.

## 17.2 Categorias de produção futura

### Protagonista

Manter o protótipo branco-prateado e desenvolver futuramente a versão final modular.

### Equipamentos

Gerar referências individuais para armas, armaduras, capas, calças, botas e acessórios.

### Inimigos comuns

Criar identidade própria para cada tipo de inimigo.

### Chefes

Criar modelos exclusivos, silhuetas marcantes e animações de fases.

### NPCs

Criar comerciantes, personagens de missões, habitantes e outros personagens interativos.

### Transformações

Preservar as ideias existentes de transformações do protagonista.

## 17.3 Critério para retomar a arte

Não retomar automaticamente após a D17.

A produção definitiva deverá depender de:

1. Gameplay suficientemente estável.
2. Sistemas essenciais integrados.
3. Conteúdo funcional com placeholders.
4. Roadmap de arte aprovado.
5. Autorização expressa do proprietário.

**Não executar nenhum passo desta seção durante a D17.**

---

# 18. ROADMAP DE DESENVOLVIMENTO FUTURO

Após a auditoria, proponha uma sequência de etapas futuras.

Exemplo inicial:

| Etapa | Foco sugerido |
|---|---|
| D17 | Auditoria + entrega funcional prioritária |
| D18 | Próximo bloqueador do ciclo jogável |
| D19 | Inventário, coleta e equipamentos |
| D20 | Combate e variedade de inimigos |
| D21 | Chefes e comportamentos especiais |
| D22 | NPCs e interações |
| D23 | Progressão do mundo e exploração |
| D24 | Integração de conteúdo |
| D25 | Testes, balanceamento e estabilidade |
| Futuro | Produção visual definitiva |

**Essa tabela é apenas uma hipótese de planejamento.**

Reorganize as etapas de acordo com o estado real do código.

Não invente entregas concluídas.

Não marque sistemas como prontos apenas porque existem documentos.

Não altere o roadmap principal indiscriminadamente.

---

# 19. CRITÉRIOS DE ACEITAÇÃO DA D17

## Auditoria

- [ ] Repositório inspecionado.
- [ ] Branch e HEAD registrados.
- [ ] Estado Git preservado.
- [ ] Arquitetura examinada.
- [ ] Roadmap comparado com o código.
- [ ] Sistemas classificados.
- [ ] Bloqueadores identificados.
- [ ] Prioridades justificadas.

## Implementação

- [ ] Uma entrega funcional principal selecionada.
- [ ] Escopo definido.
- [ ] Código implementado.
- [ ] Dados validados.
- [ ] Integração concluída.
- [ ] Testes criados.
- [ ] Regressões verificadas.

## Qualidade

- [ ] `cargo fmt --check`.
- [ ] `cargo check`.
- [ ] `cargo clippy --all-targets`.
- [ ] `cargo test`.
- [ ] Teste funcional, quando possível.

## Preservação

- [ ] Nenhum arquivo visual original sobrescrito.
- [ ] Hunyuan não iniciado.
- [ ] Blender MCP preservado.
- [ ] Protagonista GLB preservado.
- [ ] Save existente preservado.
- [ ] Sem commit.
- [ ] Sem push.

## Documentação

- [ ] Auditoria registrada.
- [ ] Implementação documentada.
- [ ] Testes registrados.
- [ ] Próximas prioridades identificadas.
- [ ] Produção visual futura documentada.

Use os estados:

- Concluído.
- Parcial.
- Bloqueado.
- Não iniciado.

Não marque como concluído algo que não foi testado.

---

# 20. LIMITES DE EXECUÇÃO

Esta tarefa autoriza a implementação de **uma entrega funcional principal**.

Não autoriza:

- Refatoração geral do projeto.
- Reescrita da arquitetura.
- Atualização de Bevy.
- Migração do formato de save.
- Implementação simultânea de vários sistemas grandes.
- Produção visual definitiva.
- Alterações no Hunyuan.
- Mudanças no Blender MCP.
- Commits.
- Push.

Se a auditoria encontrar múltiplos problemas, priorize um bloqueador principal e documente os demais.

Se a solução exigir uma alteração arquitetural de alto risco, não a execute automaticamente: apresente a proposta e aguarde autorização.

---

# 21. RELATÓRIO FINAL OBRIGATÓRIO

Ao terminar, responda diretamente no terminal com:

## A. Estado inicial

- Branch.
- HEAD.
- Estado Git.
- Ambiente Rust/Bevy.

## B. Resultado da auditoria

- Sistemas completos.
- Sistemas parciais.
- Sistemas ausentes.
- Problemas confirmados.
- Riscos.

## C. Prioridades

- P0.
- P1.
- P2.
- Próximas etapas.

## D. Entrega escolhida

- Nome.
- Justificativa.
- Escopo.
- Critérios de aceite.

## E. Implementação

- Arquivos criados.
- Arquivos modificados.
- Sistemas integrados.
- Eventos.
- Dados.

## F. Testes

- Comandos executados.
- Resultados.
- Falhas.
- Limitações.

## G. Gameplay

Explique como testar manualmente a funcionalidade.

Inclua controles, passos e comportamento esperado.

## H. Segurança

Confirme:

- Sem alterações destrutivas.
- Sem commit.
- Sem push.
- Arte preservada.
- Hunyuan não utilizado.
- Arquivo `white_mesh.glb` preservado.

## I. Próxima etapa

Recomende uma D18 com base nos resultados reais.

## J. Arte futura

Confirme que o pipeline visual definitivo foi documentado como:

**FUTURO — NÃO EXECUTAR AGORA.**

---

# 22. INSTRUÇÃO FINAL

Você não está trabalhando em uma demonstração técnica isolada.

Está desenvolvendo um RPG completo.

Cada nova funcionalidade deve aproximar o projeto de uma experiência jogável e integrada.

Priorize:

1. Funcionamento.
2. Estabilidade.
3. Arquitetura.
4. Integração.
5. Testes.
6. Conteúdo.
7. Documentação.

A aparência definitiva será tratada posteriormente.

**Execute a auditoria, escolha uma entrega funcional de alto valor, implemente dentro do escopo seguro, valide e apresente o relatório completo.**

Não faça commit nem push.

Não inicie Hunyuan.

Não altere o protagonista experimental.

**D17 — OBJETIVO FINAL: avançar concretamente o gameplay do Dark Castle RPG, preservando todo o trabalho realizado até agora.**