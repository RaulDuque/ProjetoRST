# DARK CASTLE RPG — MEGA PROMPT V1.7
## Integração do Castelo ao Jogo Principal, Save V2, Transições Seguras e Exploração Real

### MISSÃO OFICIAL — EXECUÇÃO AUTORIZADA

Você é o arquiteto técnico principal e responsável pela implementação da V1.7 do Dark Castle RPG.

O projeto utiliza Rust 2024, Bevy 0.20.1, dados RON e uma arquitetura modular baseada em plugins.

A visão definitiva do projeto é construir um Action RPG Metroidvania original, inspirado na profundidade de exploração de Castlevania: Symphony of the Night, com:

- Um castelo gigantesco e labiríntico.
- Regiões interconectadas.
- Centenas de salas no objetivo de longo prazo.
- Segredos, atalhos, relíquias e barreiras de progressão.
- Transformações e habilidades de exploração.
- Armas, armaduras e equipamentos visuais.
- Magias, técnicas e inimigos.
- NPCs, cidades, comércio e missões.
- Persistência completa do mundo.

Não copie mapas, personagens, nomes, música ou arte de Castlevania.

A V1.6 estabeleceu a arquitetura inicial do mundo e um protótipo de nove salas.

**Sua missão agora é integrar esse sistema ao jogo principal de forma segura, persistente, verificável e realmente jogável.**

Você está autorizado a implementar a V1.7 dentro das restrições deste documento.

Não inicie a V1.8.

---

# 1. ESTADO CONHECIDO DO PROJETO

O relatório da V1.6 informou:

- Branch: `art/v1.2-protagonist`.
- HEAD: `204f965`.
- Um PNG de conceito previamente staged.
- Alterações não commitadas acumuladas desde V1.2-D.
- 204 testes Rust passando.
- 17 testes Python do pipeline artístico passando.
- Autoplay 23/23.
- Visual Check 30/30.
- Nenhum commit ou push realizado.

A V1.6 implementou:

- Grafo de salas com ciclos.
- Requisitos tipados de acesso.
- Validação de progressão.
- Mapa de exploração.
- Transições entre salas.
- Descoberta de segredos.
- Persistência experimental de exploração.
- Ferramenta de relatório do mundo.
- Protótipo de nove salas na região Portões da Vigília.

O jogo principal ainda inicia na sala única `entry_hall`.

O protótipo está separado do carregamento normal.

Confirme todos esses fatos no código real antes de implementar.

Não trate o relatório como substituto da auditoria.

---

# 2. REGRAS ABSOLUTAS DE SEGURANÇA

Antes de qualquer alteração:

1. Registre branch e HEAD.
2. Registre `git status --porcelain=v1 -uall`.
3. Registre os arquivos staged.
4. Registre os arquivos modificados.
5. Registre os arquivos não rastreados.
6. Identifique os arquivos ignorados relevantes.
7. Registre hashes dos arquivos protegidos.
8. Verifique a integridade dos saves.
9. Verifique os assets publicados.
10. Confirme a linha de base dos testes.

É proibido:

- Executar `git clean`.
- Executar `git reset --hard`.
- Executar `cargo clean`.
- Criar commits.
- Executar push.
- Fazer merge ou rebase.
- Trocar de branch.
- Criar worktrees.
- Apagar backups.
- Desabilitar hooks.
- Alterar configurações de segurança.
- Instalar dependências sem autorização.
- Executar staging.
- Usar `git add .` ou `git add -A`.

Preserve:

- `Md-usario/`.
- Saves reais e respectivos backups.
- `.cargo/config.toml`.
- `target/verify/`.
- `target/f4-stage/`.
- Modelos Blender anteriores.
- Backups artísticos.
- Assets publicados.
- Alterações não commitadas.
- O PNG previamente staged.

Não modifique o índice Git.

Não apague arquivos de iteração ignorados.

---

# 3. REGRAS DE PRESERVAÇÃO DO GAMEPLAY

A V1.7 autoriza mudanças controladas na integração de mundo, transições e persistência.

Não autoriza alterar:

- Fórmulas de dano.
- Hitboxes de ataques.
- Hurtboxes do jogador.
- Dano dos inimigos.
- Gravidade.
- Velocidade de movimento.
- Altura do salto.
- Esquiva.
- Defesa.
- Cooldowns.
- Custos de habilidades.
- Comandos de técnicas.
- Magias existentes.
- Atributos.
- Experiência.
- Progressão.
- Regras de equipamentos.
- IA de combate, salvo limpeza de ciclo de vida nas transições.
- Duração de animações.
- Câmera global.
- Pipeline artístico.

A integração deve preservar as mecânicas atuais.

Mudanças em entidades temporárias são permitidas somente para garantir que elas não sobrevivam incorretamente a uma transição de sala.

---

# 4. AUDITORIA INICIAL OBRIGATÓRIA

Leia e confira:

- `CLAUDE.md`.
- `docs/GAME_VISION_MASTER.md`.
- `docs/WORLD_ARCHITECTURE.md`.
- `docs/CASTLE_WORLD_DESIGN.md`.
- `docs/ROOM_SYSTEM.md`.
- `docs/MAP_EXPLORATION_SYSTEM.md`.
- `docs/WORLD_PERSISTENCE.md`.
- `docs/WORLD_CONTENT_PIPELINE.md`.
- `docs/V1_6_VERTICAL_SLICE.md`.
- `docs/ARCHITECTURE.md`.
- `docs/DECISIONS.md`.
- `docs/ROADMAP.md`.

Inspecione o código real:

- `src/world/`.
- `src/player/`.
- `src/enemies/`.
- `src/combat/`.
- `src/magic/`.
- `src/save/`.
- `src/ui/`.
- `src/app/`, se existir.
- `assets/world/`.
- `assets/data/`, se existir.
- `examples/world_vertical_slice.rs`.

Confirme os contratos existentes antes de alterar qualquer módulo.

Produza uma matriz:

| Subsistema | Estado V1.6 | Mudança V1.7 | Risco | Teste de regressão |
|---|---|---|---|---|
| Carregamento inicial | | | | |
| Grafo | | | | |
| Transições | | | | |
| Spawn | | | | |
| Despawn | | | | |
| Câmera | | | | |
| Combate | | | | |
| Inimigos | | | | |
| Projéteis | | | | |
| Hitboxes | | | | |
| Mapa | | | | |
| Segredos | | | | |
| Relíquias | | | | |
| Save | | | | |
| Quick load | | | | |
| UI | | | | |

Não avance para migração de save antes de entender o formato atual e seus pontos de escrita.

---

# 5. OBJETIVOS DE ACEITAÇÃO DA V1.7

A V1.7 deverá entregar:

1. Mundo de múltiplas salas integrado ao jogo principal.
2. Transições reais pelas portas.
3. Entrada correta em cada sala.
4. Limpeza segura das entidades da sala anterior.
5. Persistência de exploração.
6. Persistência de segredos.
7. Persistência de desbloqueios.
8. Save V2 com compatibilidade retroativa.
9. Quick load entre salas.
10. Mapa acessível durante o jogo.
11. Rota jogável sem reposicionamentos de teste.
12. Testes automatizados de progressão.
13. Regressão zero no gameplay existente.

Não implemente sistemas de NPCs, comércio ou transformações completas nesta fase.

---

# 6. INTEGRAÇÃO DO MUNDO AO JOGO PRINCIPAL

Atualmente o jogo principal inicia em `entry_hall`.

A V1.7 deverá integrar o novo sistema de mundo sem quebrar o carregamento anterior.

## 6.1 Estratégia de compatibilidade

Primeiro determine como o jogo principal carrega:

- A sala inicial.
- O jogador.
- A câmera.
- Os inimigos.
- As plataformas.
- Os objetos.
- Os recursos de gameplay.

Não substitua imediatamente a implementação anterior.

Crie uma camada de integração que permita utilizar o grafo de salas com o mesmo jogador e os mesmos sistemas.

Evite manter duas arquiteturas independentes de mundo.

## 6.2 Sala inicial

Defina uma sala inicial válida no novo grafo.

A região Portões da Vigília poderá servir como ponto de entrada de desenvolvimento.

Preserve `entry_hall` como referência de compatibilidade ou fixture de testes, conforme a arquitetura real.

Não apague a sala antiga.

Não deixe saves antigos apontando para salas inexistentes.

## 6.3 Ativação

O jogo principal deve inicializar:

- Catálogo de salas.
- Grafo validado.
- Estado de exploração.
- Sala atual.
- Entidades da sala atual.
- Mapa.
- Controlador de transição.

Não carregue todas as salas simultaneamente.

---

# 7. CICLO DE VIDA DAS SALAS

Defina um ciclo de vida explícito para a sala ativa.

Etapas conceituais:

```text
Solicitação de transição
          ↓
Validar destino e requisitos
          ↓
Bloquear transições concorrentes
          ↓
Registrar estado persistente da origem
          ↓
Encerrar entidades e efeitos temporários
          ↓
Descarregar entidades da origem
          ↓
Instanciar sala de destino
          ↓
Posicionar jogador na entrada correta
          ↓
Atualizar câmera
          ↓
Atualizar mapa e exploração
          ↓
Liberar controle normal
```

A implementação pode ter outra organização, desde que preserve essas garantias.

## 7.1 Identidade das entidades

Diferencie claramente:

- Entidades persistentes.
- Entidades pertencentes à sala.
- Entidades temporárias de combate.
- Entidades visuais.
- Entidades de interface.

Não destrua o jogador durante uma transição normal.

Não destrua a UI global.

Não duplique o protagonista.

Não mantenha inimigos antigos.

## 7.2 Transições concorrentes

Proteja contra:

- Duas portas acionadas no mesmo frame.
- Entrada repetida na mesma porta.
- Transição durante morte.
- Transição durante carregamento.
- Transição durante quick load.
- Transição enquanto outra transição ainda está pendente.

Defina o comportamento de forma determinística.

Não permita duas salas ativas por acidente.

---

# 8. CORREÇÃO CRÍTICA — HITBOXES REMANESCENTES

A V1.6 identificou que hitboxes temporárias de inimigos podem continuar ativas por aproximadamente 0,15 segundo após uma troca de sala.

**Essa falha precisa ser corrigida antes de considerar a V1.7 concluída.**

## 8.1 Auditoria

Localize todas as entidades temporárias relevantes:

- Hitboxes de inimigos.
- Hitboxes do jogador.
- Projéteis.
- Magias.
- Efeitos temporários.
- Indicadores de dano.
- Eventos de respawn.
- Ataques em andamento.

Identifique seus proprietários e vínculos com a sala.

## 8.2 Correção

Implemente limpeza por ciclo de vida, proprietário ou geração da sala, conforme a arquitetura existente.

Uma hitbox criada em uma sala não pode causar dano depois que essa sala for descarregada.

Evite depender exclusivamente do timer de expiração.

Não altere a duração normal das hitboxes durante o combate.

Não altere dano ou colisão dentro da sala.

## 8.3 Testes

Teste:

1. Transição durante ataque inimigo.
2. Transição com projétil ativo.
3. Transição com magia ativa.
4. Transição com hitbox do jogador ativa.
5. Transição com inimigo em respawn pendente.
6. Entrada imediata na sala seguinte.
7. Retorno à sala anterior.
8. Transições repetidas.
9. Morte durante transição.
10. Quick load com entidades temporárias.

**Critério obrigatório: nenhuma entidade temporária da sala anterior pode atingir o jogador ou inimigos da sala seguinte.**

---

# 9. PORTAS E PONTOS DE ENTRADA

Cada conexão precisa possuir entradas verificáveis.

Valide:

- ID da porta de origem.
- ID da sala de destino.
- ID da porta de destino.
- Direção.
- Posição de entrada.
- Requisitos de acesso.
- Tipo de conexão.

Tipos possíveis:

- Bidirecional.
- Mão única.
- Atalho.
- Passagem secreta.
- Portão bloqueado.
- Conexão vertical.

Não presuma que toda conexão possui retorno direto.

## 9.1 Spawn seguro

Ao entrar em uma sala:

- O jogador não pode nascer dentro de parede.
- Não pode nascer dentro de inimigo.
- Não pode nascer fora dos limites.
- Não pode nascer em queda infinita por erro.
- Não pode ativar imediatamente a porta de retorno sem intenção.

Use pontos de entrada explícitos e validados.

Não altere o tamanho do collider do jogador para resolver problemas de portas.

---

# 10. EXPLORAÇÃO REAL SEM TELEPORTES DE TESTE

Na V1.6, a rota automatizada utilizou oito reposicionamentos de teste:

- Seis posicionamentos próximos às portas.
- Dois posicionamentos dentro de salas.

Esses atalhos foram úteis para testar as transições, mas não provam a navegabilidade completa do mundo.

Na V1.7, crie uma rota que possa ser percorrida com as mecânicas reais.

## 10.1 Requisitos

A rota deverá permitir:

1. Entrar na região.
2. Percorrer corredores.
3. Subir o poço vertical.
4. Encontrar a parede falsa.
5. Descobrir o esconderijo.
6. Obter a Relíquia do Véu.
7. Abrir a passagem bloqueada.
8. Utilizar o atalho.
9. Chegar ao relicário.
10. Retornar a uma sala anterior.

Não use teleporte de teste para completar a rota principal.

Não aumente a altura do salto.

Não aumente a velocidade.

Não altere a gravidade.

Se o poço não puder ser percorrido com as habilidades atuais, ajuste a geometria provisória do protótipo.

Não introduza salto duplo sem autorização.

## 10.2 Teste de navegabilidade

Crie um teste automatizado que utilize entradas reais de movimento, salto e interação.

É permitido utilizar um controlador de teste para pressionar botões.

Não é permitido posicionar diretamente o Transform do jogador durante a rota, exceto no spawn inicial e nas transições legítimas do sistema.

Registre a rota com:

- Sala.
- Posição.
- Ação.
- Tempo.
- Resultado.

Inclua contadores de reposicionamentos artificiais.

O resultado esperado para a rota principal é:

**0 reposicionamentos artificiais.**

Se o teste falhar, registre o trecho exato.

Não declare a rota completamente jogável enquanto depender de atalhos.

---

# 11. SAVE V2 — COMPATIBILIDADE RETROATIVA

Esta é a segunda área crítica da V1.7.

O formato de save deverá evoluir para armazenar o estado do mundo.

Entretanto, os saves reais existentes são protegidos.

## 11.1 Auditoria

Identifique:

- Formato atual.
- Versão atual.
- Campos existentes.
- Serialização.
- Desserialização.
- Validação.
- Backups.
- Quick save.
- Quick load.
- Tratamento de arquivos corrompidos.
- Política de escrita atômica, se existente.

Leia `docs/WORLD_PERSISTENCE.md`.

Não presuma que o contrato proposto na V1.6 já está implementado.

## 11.2 Novo formato

O Save V2 deverá representar:

- Versão do schema.
- Identidade da sala atual.
- Região atual, se necessária.
- Posição do jogador.
- Estado persistente do jogador.
- Inventário.
- Equipamentos.
- Atributos.
- Progressão.
- Mapa visitado.
- Segredos encontrados.
- Relíquias obtidas.
- Portas e atalhos desbloqueados.
- Flags persistentes do mundo.

Não duplique informações que podem ser derivadas com segurança.

Use IDs estáveis.

Não salve Entity IDs do Bevy como identificadores persistentes.

## 11.3 Migração V1 → V2

Implemente migração explícita.

A migração deve:

1. Reconhecer saves antigos.
2. Preservar os dados existentes.
3. Preencher os novos campos com valores seguros.
4. Resolver a sala inicial antiga.
5. Validar a posição.
6. Preservar inventário e progressão.
7. Não descartar campos silenciosamente.
8. Não modificar o arquivo de origem durante a leitura.

A migração não pode depender de uma substituição textual frágil.

Use tipos e validação adequados.

## 11.4 Política de gravação

Não sobrescreva automaticamente um save V1 só porque ele foi carregado.

A conversão em memória deve ser separada da persistência.

Uma gravação explícita poderá gerar V2 somente após validação completa.

Preserve um backup recuperável do save anterior.

Se a estratégia de backup atual for insuficiente, implemente uma alternativa segura sem alterar os saves reais durante os testes.

## 11.5 Compatibilidade futura

Documente como versões posteriores poderão migrar V2 para V3.

Não implemente V3 agora.

---

# 12. SAVE/LOAD ENTRE SALAS

O quick_load precisa restaurar corretamente o mundo, não apenas o jogador.

Cenário obrigatório:

1. Jogador está na sala A.
2. Vai para a sala B.
3. Descobre um segredo.
4. Obtém uma relíquia.
5. Salva.
6. Vai para a sala C.
7. Carrega o save.
8. Retorna à sala B.
9. Recupera a posição correta.
10. Mantém segredo e relíquia.
11. Não duplica entidades.

A restauração deve ocorrer em uma ordem segura.

Não aplique posição do jogador antes de garantir que a sala correta está carregada.

Não permita que um evento atrasado da sala anterior altere o mundo restaurado.

Teste também:

- Load após morte.
- Load durante habilidade.
- Load durante transição.
- Load com projétil ativo.
- Load de sala secreta.
- Load de save V1.
- Load de arquivo inválido.
- Load de sala removida do catálogo.
- Load de posição fora dos limites.

Defina recuperação segura para dados inválidos.

Não invente uma posição arbitrária silenciosamente.

---

# 13. PERSISTÊNCIA DE EXPLORAÇÃO

O estado de exploração deverá registrar:

- Salas visitadas.
- Segredos descobertos.
- Relíquias obtidas.
- Flags de progressão.
- Portas desbloqueadas.

A porcentagem de exploração deve continuar sendo calculada, não armazenada como valor definitivo.

Não revele salas secretas antes da descoberta.

Não permita que o quick_load misture o estado atual com o estado salvo.

O carregamento deve substituir o estado persistente de maneira coerente.

---

# 14. RELÍQUIAS E ITENS DE PROGRESSÃO

A V1.6 criou a Relíquia do Véu, que concede a habilidade de exploração `VeilStep`.

Ela ainda não representa uma transformação física.

Mantenha essa distinção.

## 14.1 Integração

Ao coletar a relíquia:

- Registrar aquisição.
- Atualizar os requisitos de acesso.
- Atualizar a exploração.
- Evitar coleta duplicada.
- Persistir a aquisição no Save V2.
- Restaurar após load.

Não implemente a Forma Espectral completa nesta fase.

## 14.2 Inventário

O relatório da V1.6 informou que os itens coletados no protótipo ainda não entram no inventário.

Audite o sistema atual.

Defina uma separação clara entre:

- Item de inventário.
- Relíquia de progressão.
- Habilidade desbloqueada.
- Flag do mundo.

Se a Relíquia do Véu for um item de progressão permanente, ela não precisa necessariamente ocupar um slot de inventário comum.

Documente a decisão.

Não crie dois registros independentes que possam divergir.

---

# 15. MAPA NO JOGO PRINCIPAL

O mapa da V1.6 já possui um painel experimental.

Integre-o ao fluxo normal.

O jogador deverá conseguir:

- Abrir o mapa.
- Fechar o mapa.
- Identificar a sala atual.
- Visualizar salas visitadas.
- Visualizar conexões conhecidas.
- Identificar segredos descobertos.
- Consultar porcentagem de exploração.

Respeite os controles atuais.

Não substitua teclas existentes sem autorização.

Se for necessário definir uma nova tecla, escolha uma opção livre e documente.

O mapa não pode revelar automaticamente todo o castelo.

Não deve expor conteúdo secreto por meio de nomes, marcadores ou conexões antes da descoberta.

---

# 16. CÂMERA E LIMITES DE SALA

Preserve:

- Zoom atual.
- Escala.
- Filtro visual.
- Resolução lógica.
- Comportamento de câmera já aprovado.

Adapte apenas os limites da câmera à sala ativa, se necessário.

Teste:

- Sala pequena.
- Sala grande.
- Sala vertical.
- Sala horizontal.
- Entrada pela esquerda.
- Entrada pela direita.
- Entrada pelo alto.
- Entrada por baixo.

Não permita mostrar regiões descarregadas por erro.

Não introduza mudanças de zoom para resolver problemas de geometria.

---

# 17. INTERAÇÕES E SEGREDOS

A V1.7 deve integrar as interações já prototipadas.

Inclua:

- Parede falsa.
- Relíquia.
- Grade bloqueada.
- Atalho.
- Recompensa.

Não implemente ainda um sistema universal de missões ou diálogos.

## 17.1 Parede falsa

A descoberta deve ser persistente.

A passagem deve funcionar depois de descoberta.

Ao recarregar, não deve voltar ao estado anterior indevidamente.

## 17.2 Atalhos

Um atalho aberto deve continuar aberto após save/load.

Conexões de mão única devem continuar respeitando sua direção.

## 17.3 Recompensas

Não permita coletar a mesma recompensa permanente repetidamente após transições ou load.

---

# 18. TESTES DE GRAFO E PROGRESSÃO

Preserve os testes da V1.6.

Acrescente:

- Mundo carregado no jogo principal.
- Sala inicial válida.
- Conexões recíprocas corretas.
- Conexões de mão única.
- Requisitos de relíquias.
- Segredos persistentes.
- Portas desbloqueadas.
- Progressão principal alcançável.
- Ausência de softlocks evitáveis.
- Catálogo de salas válido.

O validador deve continuar diferenciando bloqueios intencionais de erros.

Não altere testes antigos apenas para eliminar falhas.

---

# 19. TESTES DE REGRESSÃO DO GAMEPLAY

Compare a V1.7 com a linha de base da V1.6.

Verifique:

- Movimento.
- Salto.
- Esquiva.
- Ataque básico.
- Crescent Slash.
- Rising Edge.
- Ember Bolt.
- Mending Light.
- Dano.
- Morte.
- Revive.
- Inimigos.
- Respawn.
- Equipamentos.
- Inventário.
- Save/load.
- Animações.
- VFX.

Não modifique valores de gameplay para fazer o mundo funcionar.

Inclua um teste determinístico que compare o comportamento de combate antes e depois da integração, quando tecnicamente possível.

---

# 20. TESTES DE STRESS

Execute cenários repetitivos:

1. Cem transições de sala.
2. Cem aberturas e fechamentos do mapa.
3. Transições rápidas em sequência.
4. Save/load repetido.
5. Entrada e saída de sala com inimigos.
6. Entrada e saída com projéteis.
7. Morte e revive em salas diferentes.
8. Descoberta de segredos seguida de load.
9. Coleta seguida de retorno.
10. Troca de sala durante efeitos visuais.

Monitore:

- Entidades ativas.
- Recursos.
- Eventos pendentes.
- Estado da sala.
- Estado do mapa.
- Memória, quando possível.

Não declare ausência de vazamento apenas porque o jogo não travou.

---

# 21. TESTES AUTOMATIZADOS E COMANDOS

Execute as verificações pertinentes:

```bash
cargo fmt --all -- --check
cargo check
cargo clippy --all-targets -- -D warnings
cargo test
```

Execute também:

- Testes Python do guarda de publicação.
- Autoplay.
- Visual Check.
- World Report.
- Vertical Slice.
- Testes de Save V1.
- Testes de Save V2.
- Testes de migração.
- Testes de transição.
- Testes de limpeza de entidades.

Respeite `CLAUDE.md`.

Não use `--all-features` se ativar recursos incompatíveis com o ambiente.

Não execute compilações simultâneas.

Não apague caches.

Não use saves reais como fixtures destrutivas.

---

# 22. VALIDAÇÃO VISUAL REAL

Execute o jogo principal com janela.

Confirme:

- Entrada no castelo.
- Movimento entre salas.
- Transição vertical.
- Mapa.
- Parede secreta.
- Relíquia.
- Grade.
- Atalho.
- Save/load.
- Inimigos.
- Animações.
- VFX.

Produza capturas reais do Bevy.

Crie uma demonstração curta mostrando:

```text
Iniciar jogo
    ↓
Entrar no castelo
    ↓
Explorar salas
    ↓
Abrir mapa
    ↓
Descobrir passagem secreta
    ↓
Coletar relíquia
    ↓
Desbloquear grade
    ↓
Salvar
    ↓
Mudar de sala
    ↓
Carregar
    ↓
Retornar ao estado salvo
```

A demonstração não pode utilizar reposicionamentos artificiais para simular a navegação.

Se houver impossibilidade de percorrer algum trecho, registre o problema e não declare sucesso visual completo.

---

# 23. PERFORMANCE E ESCALABILIDADE

A arquitetura deverá continuar adequada a um castelo futuro de centenas de salas.

Meça:

- Entidades por sala.
- Tempo de transição.
- Custo de carregamento.
- Memória.
- Atualizações do mapa.
- Tempo de validação do grafo.
- Tempo de save/load.

Não carregue todas as salas em entidades ativas.

Não reconstrua o mapa inteiro a cada frame.

Não faça leituras de arquivos desnecessárias durante o gameplay.

Evite complexidade proporcional ao total de salas em sistemas executados por frame.

---

# 24. DOCUMENTAÇÃO OBRIGATÓRIA

Crie:

`docs/WORLD_INTEGRATION_V1_7.md`

Inclua:

1. Estado inicial.
2. Arquitetura integrada.
3. Ciclo de vida de salas.
4. Transições.
5. Limpeza de entidades.
6. Hitboxes.
7. Spawn.
8. Save V2.
9. Migração V1.
10. Quick load.
11. Exploração.
12. Relíquias.
13. Mapa.
14. Segredos.
15. Rota sem atalhos.
16. Testes.
17. Performance.
18. Riscos.
19. Pendências.
20. Próximas etapas.

Atualize os documentos diretamente afetados:

- `docs/WORLD_ARCHITECTURE.md`.
- `docs/WORLD_PERSISTENCE.md`.
- `docs/ROOM_SYSTEM.md`.
- `docs/MAP_EXPLORATION_SYSTEM.md`.
- `docs/ARCHITECTURE.md`.
- `docs/DECISIONS.md`.
- `docs/ROADMAP.md`.

Não reescreva documentos inteiros sem necessidade.

Registre decisões novas com identificadores consistentes.

---

# 25. CRITÉRIOS DE ACEITAÇÃO

A V1.7 somente estará concluída quando:

- [ ] Git inicial auditado.
- [ ] Arquivos protegidos preservados.
- [ ] Mundo integrado ao jogo principal.
- [ ] Sala inicial válida.
- [ ] Transições funcionais.
- [ ] Entradas corretas.
- [ ] Sem entidades duplicadas.
- [ ] Sem hitboxes remanescentes.
- [ ] Sem projéteis antigos.
- [ ] Sem respawns antigos indevidos.
- [ ] Save V2 implementado.
- [ ] Migração V1 → V2 validada.
- [ ] Saves reais preservados.
- [ ] Quick load entre salas funcionando.
- [ ] Exploração persistente.
- [ ] Segredos persistentes.
- [ ] Relíquias persistentes.
- [ ] Portas desbloqueadas persistentes.
- [ ] Mapa integrado.
- [ ] Segredos não revelados antecipadamente.
- [ ] Rota principal sem reposicionamentos artificiais.
- [ ] Navegação vertical real validada.
- [ ] Testes do grafo passando.
- [ ] Testes de regressão passando.
- [ ] Autoplay passando.
- [ ] Visual Check passando.
- [ ] Testes de stress passando.
- [ ] Gameplay preservado.
- [ ] Câmera preservada.
- [ ] Animações preservadas.
- [ ] VFX preservados.
- [ ] Nenhum commit realizado.
- [ ] Nenhum push realizado.
- [ ] V1.8 não iniciada.

Se qualquer critério estiver pendente, declare a V1.7 parcialmente concluída e explique exatamente o motivo.

---

# 26. RELATÓRIO FINAL OBRIGATÓRIO

Entregue:

**A. Estado inicial e Git**

**B. Auditoria da V1.6**

**C. Arquitetura integrada**

**D. Carregamento inicial**

**E. Ciclo de vida das salas**

**F. Portas e transições**

**G. Hitboxes e limpeza de entidades**

**H. Navegação real**

**I. Save V2**

**J. Migração V1 → V2**

**K. Quick load**

**L. Exploração persistente**

**M. Relíquias e segredos**

**N. Mapa**

**O. Câmera**

**P. Testes de regressão**

**Q. Testes de stress**

**R. Validação visual**

**S. Performance**

**T. Arquivos criados e modificados**

**U. Estado final do Git**

**V. Pendências e riscos**

**W. Próximos passos recomendados**

Para cada resultado, diferencie:

- Implementado.
- Testado automaticamente.
- Validado visualmente.
- Parcial.
- Não implementado.

Inclua comandos executados e resultados.

Não declare testes que não foram executados.

---

# 27. CONDIÇÕES DE PARADA

Interrompa a operação afetada se:

- Houver risco de perda de saves.
- A migração não preservar os dados antigos.
- O Git apresentar risco de perda de trabalho.
- Uma transição exigir modificar regras fundamentais do combate.
- O mundo precisar de uma mudança estrutural incompatível com a arquitetura aprovada.
- Um teste revelar corrupção de estado.
- A publicação exigir sobrescrever arquivos protegidos.
- Uma dependência nova for indispensável.
- O protótipo não puder ser percorrido sem alterar a física.

Nesses casos:

1. Preserve o estado.
2. Documente o problema.
3. Não aplique uma solução destrutiva.
4. Continue somente tarefas independentes e seguras.
5. Solicite autorização para a mudança necessária.

---

# 28. INSTRUÇÃO FINAL

Você está autorizado a implementar a V1.7 dentro deste escopo.

Execute nesta ordem:

1. Auditoria e linha de base.
2. Integração inicial do mundo.
3. Ciclo de vida de salas.
4. Correção das hitboxes.
5. Portas e entradas.
6. Navegação real sem atalhos.
7. Save V2 e migração.
8. Quick load entre salas.
9. Exploração e mapa.
10. Testes de regressão.
11. Testes de stress.
12. Validação visual.
13. Documentação.
14. Relatório final.

**Não faça commits, push ou staging. Não altere o gameplay, a câmera global, os saves reais nem o pipeline artístico. Não inicie a V1.8.**

A V1.7 deve terminar com o jogador explorando o castelo dentro do jogo principal, utilizando o movimento real, descobrindo segredos e preservando o progresso entre sessões.

Ao concluir, apresente o relatório completo e aguarde minha avaliação.