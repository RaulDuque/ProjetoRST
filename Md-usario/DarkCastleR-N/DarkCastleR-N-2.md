# DARK CASTLE RPG — N2
# SISTEMA DEFINITIVO DE CONTROLES HÍBRIDOS, REMAPEAMENTO, PERSISTÊNCIA E COMPATIBILIDADE COM COMBATE

**Projeto:** Dark Castle RPG  
**Etapa:** N2 — Controles  
**Engine:** Bevy 0.20.1  
**Linguagem:** Rust 2024  
**Plataformas:** Windows 11 e Linux  
**Status:** AUTORIZADA PARA IMPLEMENTAÇÃO  
**Etapa anterior:** N1 concluída, decisões aprovadas e registradas em D-036.

---

# 0. MISSÃO E REGRAS INEGOCIÁVEIS

Você atuará como engenheiro principal de gameplay, arquitetura Rust/Bevy, sistemas de entrada, UX e testes.

Sua missão é implementar um sistema de controles robusto, configurável e persistente, preservando integralmente os sistemas existentes do Dark Castle RPG.

**A N2 não consiste em substituir teclas fixas por outras teclas fixas.**

A entrega precisa permitir que o jogador personalize os controles por meio de uma interface funcional dentro do jogo.

Ao finalizar, o jogador deverá conseguir:

- Jogar com teclado e mouse.
- Utilizar WASD.
- Pular tanto com W quanto com Espaço.
- Atacar com o botão esquerdo do mouse.
- Usar uma tecla alternativa para atacar.
- Interagir com E.
- Personalizar ações.
- Detectar conflitos.
- Salvar as preferências.
- Restaurar os controles padrão.
- Ativar o preset clássico.
- Executar técnicas especiais independentemente das teclas físicas escolhidas.
- Navegar pelos menus sem movimentar o personagem.
- Reiniciar o jogo e recuperar as preferências.

Não iniciar N3.

Não modificar arte, sprites, modelos 3D, câmera ou resolução.

Não alterar Save v3.

Não criar commits nem executar push.

---

# 1. LEITURA OBRIGATÓRIA DO PROJETO

Antes de escrever código, leia:

- `CLAUDE.md`
- `docs/ARCHITECTURE.md`
- `docs/DECISIONS.md`
- `docs/development/n1/N1_CONTROLES.md`
- `docs/development/n1/N1_ARQUITETURA_TECNICA.md`
- `docs/development/n1/N1_ROADMAP.md`
- `docs/development/n1/N1_VISAO_DO_JOGO.md`
- `controles.md`

Leia também a documentação existente sobre:

- Sistema de entrada.
- Parser de comandos.
- Combate.
- Estados do personagem.
- Interface.
- Inventário.
- Salvamento.
- Exemplos automatizados.
- Testes headless.

Inspecione os arquivos reais antes de assumir nomes, funções ou componentes.

Verifique:

- Branch atual.
- HEAD.
- Arquivos modificados.
- Arquivos não rastreados.
- Arquivos apagados.
- Diferenças anteriores à N2.

A reorganização de `Md-usario/DarkCastleR-A/` para `Md-usario/DarkCastleR-D/` já foi explicada pelo commit `212094c`.

Não restaurar nem limpar essas pastas.

Preservar todos os arquivos existentes.

---

# 2. AUDITORIA DO SISTEMA DE ENTRADA ATUAL

Localize e documente:

1. `InputBindings`.
2. Leitura de teclado.
3. Construção de `PlayerInput`.
4. Movimentação horizontal.
5. Pulo.
6. Agachamento.
7. Ataque.
8. Guarda.
9. Esquiva.
10. Magia.
11. Interação.
12. Transformação.
13. Inventário.
14. Mapa.
15. Tela de status.
16. Salvamento manual.
17. Carregamento.
18. Respawn.
19. Comandos especiais.
20. Sistemas que leem teclas diretamente.

Produza uma matriz contendo:

- Ação.
- Tecla atual.
- Sistema responsável.
- Tipo de entrada.
- Estado em que funciona.
- Risco de conflito.
- Testes existentes.

Identifique qualquer uso direto de `KeyCode` ou `MouseButton` fora do módulo de entrada.

**Não substitua automaticamente todas as leituras diretas.**

Primeiro classifique quais representam controles do jogador, quais são ferramentas de desenvolvimento e quais pertencem a testes.

---

# 3. CONTROLES PADRÃO APROVADOS

Implementar o preset:

`HybridDefault`

Mapeamentos obrigatórios:

| Ação | Entrada |
|---|---|
| Esquerda | A |
| Direita | D |
| Pular | W |
| Pular alternativa | Espaço |
| Agachar | S |
| Ataque básico | Mouse esquerdo |
| Ataque alternativo | Tecla configurável |
| Interagir | E |
| Confirmar diálogo | Enter, quando aplicável |

Os controles restantes devem ser configuráveis.

Para ações sem tecla nova explicitamente aprovada, preservar o comportamento atual quando não houver conflito.

Não inventar um novo esquema completo sem necessidade.

Se alguma ação atual perder sua tecla devido ao novo padrão, escolher uma alternativa provisória coerente, documentar e permitir remapeamento.

Não utilizar a mesma tecla para duas ações incompatíveis no mesmo contexto.

O preset híbrido será o padrão para instalações sem configuração anterior.

---

# 4. PRESET CLÁSSICO

Criar um segundo preset:

`LegacyClassic`

Ele deve reproduzir o esquema anterior do projeto, incluindo:

- Setas para movimentação.
- Z para pulo.
- X para ataque.
- C para magia.
- Demais teclas antigas.

Não reconstruir o preset pela memória.

Extrair os valores exatos do código e de `controles.md`.

O preset clássico deve permanecer utilizável.

A troca de preset não pode alterar o save do personagem.

A interface deve permitir selecionar:

- Híbrido.
- Clássico.
- Personalizado.

Ao editar um preset, não modificar a definição imutável desse preset. Criar uma configuração personalizada derivada dele.

---

# 5. ARQUITETURA DE AÇÕES ABSTRATAS

Separar claramente:

**Dispositivo físico → Binding → Ação lógica → Sistema de gameplay.**

Exemplo:

`KeyW → Jump → PlayerInput.jump`

`Space → Jump → PlayerInput.jump`

`MouseLeft → BasicAttack → PlayerInput.attack`

A lógica de combate não deve precisar saber se o jogador apertou W, Espaço, X ou mouse.

Criar ou adaptar uma representação tipada das ações.

Evitar strings livres para ações internas.

A arquitetura deve permitir:

- Uma ação com múltiplos bindings.
- Teclado.
- Mouse.
- Estados de pressionamento.
- Estados de soltura.
- Pulsos de ativação.
- Entradas mantidas.
- Combinações futuras.
- Contextos de entrada.

Não adicionar uma biblioteca externa se o Bevy e o código existente forem suficientes.

Se recomendar uma dependência nova, justificar compatibilidade com Bevy 0.20.1 e impacto no projeto antes de incluí-la.

---

# 6. SEMÂNTICA CORRETA DE INPUT

Diferenciar:

- `pressed`: entrada mantida.
- `just_pressed`: início de pressionamento.
- `just_released`: fim de pressionamento.

Definir essas semânticas no nível da ação lógica.

Quando uma ação possuir duas teclas, o comportamento deve ser consistente.

Exemplo:

W está pressionado e o jogador aperta Espaço.

Isso não pode produzir um segundo pulo terrestre artificial apenas porque a segunda tecla gerou um novo evento.

Da mesma forma, se mouse e tecla de ataque forem pressionados no mesmo frame, a ação não pode disparar dois ataques básicos.

O sistema deve consolidar os bindings antes de enviar a ação ao gameplay.

Verificar o comportamento ao soltar uma das teclas enquanto outra permanece pressionada.

Não gerar falsos eventos de soltura.

---

# 7. MOVIMENTAÇÃO E DIREÇÃO

O novo esquema utiliza:

- A: esquerda.
- D: direita.
- S: baixo.
- W: pulo.

Atenção: W é pulo, não direção para cima por padrão.

O projeto já possui um sistema de comandos direcionais com notações como:

- Frente.
- Trás.
- Baixo.
- Cima.
- Diagonais.

Isso cria uma questão técnica importante.

O parser não pode presumir que a tecla de pulo é automaticamente equivalente a uma direção para cima sem avaliar o comportamento existente.

Audite os comandos atuais e proponha uma solução coerente para:

- Direção para cima.
- Direção para baixo.
- Diagonais.
- Técnicas aéreas.
- Comandos que exigem neutro.
- Movimentação durante comandos.

**Regra:** preservar a capacidade de executar todos os comandos já existentes.

Se for necessário um binding independente para direção de comando "cima", implemente-o de maneira configurável e documentada, sem alterar silenciosamente a semântica do pulo.

Não remover comandos existentes.

Não enfraquecer o parser.

---

# 8. COMPATIBILIDADE COM COMANDOS ESPECIAIS

O sistema existente utiliza buffer temporal e direções relativas à orientação do personagem.

Preservar:

- Frente e trás relativos à face.
- Baixo.
- Cima.
- Diagonais.
- Neutro.
- Tolerância temporal.
- Prioridades.
- Desempates.
- Cancelamento pela guarda.
- Cooldowns.
- Custos.
- Estados inválidos.
- Feedback de comando.

Exemplo:

O jogador está olhando para a direita.

Executa:

Baixo → Frente → Ataque.

No preset híbrido, a sequência deve utilizar os bindings configurados.

No preset clássico, deve funcionar com os controles antigos.

Depois de remapear as teclas, o resultado deve continuar idêntico.

Não reescrever `command_parser.rs` sem necessidade demonstrada.

Preferir adaptar a camada de entrada.

---

# 9. INTERAÇÃO COM E

A tecla D atualmente possui função de interação.

No novo padrão:

- D movimenta para direita.
- E interage.

Auditar todas as interações:

- NPCs.
- Diálogos.
- Baús.
- Altares.
- Portas.
- Alavancas.
- Comerciantes.
- Objetos.
- Menus contextuais.

Verificar se algum sistema ainda lê D diretamente.

Garantir que pressionar D não interaja acidentalmente.

Garantir que pressionar E não movimente o personagem.

Preservar Enter para confirmação de diálogo quando apropriado.

---

# 10. INTERFACE DE CONFIGURAÇÃO

Implementar uma tela de controles dentro do jogo.

Ela deve apresentar:

- Nome da ação.
- Tecla principal.
- Tecla alternativa.
- Botão do mouse, quando aplicável.
- Opção de remapear.
- Preset ativo.
- Opção de restaurar.
- Opção de cancelar alterações.
- Opção de aplicar.

A interface precisa ser legível no estilo atual do Dark Castle RPG.

Não criar uma identidade visual definitiva nova.

Reutilizar o tema de UI existente.

## Captura de tecla

Fluxo esperado:

1. Jogador seleciona uma ação.
2. Seleciona um slot de binding.
3. Interface informa que está aguardando uma entrada.
4. Jogador pressiona tecla ou botão de mouse.
5. Sistema identifica o binding.
6. Sistema verifica conflitos.
7. Jogador confirma ou cancela.
8. Configuração é aplicada.

Durante a captura:

- Não movimentar o personagem.
- Não atacar.
- Não lançar magia.
- Não abrir outros menus.
- Não interagir.
- Não executar comandos especiais.

Escape deve permitir cancelar a captura, com comportamento documentado.

---

# 11. DETECÇÃO E RESOLUÇÃO DE CONFLITOS

Não basta detectar que duas ações utilizam a mesma tecla.

É necessário considerar o contexto.

Exemplo:

Enter pode confirmar um diálogo e também confirmar uma seleção de menu.

Isso não é necessariamente um conflito.

Por outro lado:

D não pode simultaneamente mover para direita e interagir durante o gameplay.

Implementar validação que identifique:

- Conflitos no mesmo contexto.
- Duplicações dentro da mesma ação.
- Bindings vazios.
- Ações essenciais sem entrada.
- Teclas reservadas, se existirem.
- Conflitos entre mouse e teclado quando aplicáveis.

O sistema deve oferecer resolução clara.

Opções aceitáveis:

- Rejeitar binding incompatível.
- Permitir troca entre ações.
- Exigir confirmação explícita.

Escolha uma política consistente e documente.

Não deixar o jogador sem maneira de abrir o menu de controles ou restaurar os padrões.

---

# 12. CONTEXTOS DE ENTRADA

Planejar contextos como:

- Gameplay.
- Diálogo.
- Inventário.
- Mapa.
- Status.
- Configurações.
- Captura de binding.
- Menu de pausa.

Adaptar aos estados reais existentes.

Não criar uma segunda máquina de estados paralela se o projeto já possui estados adequados.

Regras:

Durante configuração, o personagem não deve executar ações de gameplay.

Durante diálogo, ataque e magia não devem ser disparados pela confirmação.

Durante inventário, entradas destinadas à interface não devem acionar comandos especiais.

Ao fechar um menu, entradas ainda mantidas não devem gerar ações inesperadas.

Se o jogo não possuir um sistema global de pausa apropriado, implementar apenas o bloqueio de gameplay necessário, documentando a diferença.

Não afirmar que o mundo está pausado se inimigos e física continuarem avançando.

---

# 13. PERSISTÊNCIA DAS CONFIGURAÇÕES

Criar um arquivo local próprio para preferências de controles.

Não salvar bindings no Save v3.

Requisitos:

- Carregar na inicialização.
- Criar padrão quando não existir.
- Salvar após aplicação.
- Manter configurações entre sessões.
- Suportar restauração.
- Detectar arquivo inválido.
- Não travar o jogo por configuração corrompida.
- Evitar perda de configurações em escrita interrompida.

Utilizar um formato compatível com as dependências já presentes, preferencialmente RON se apropriado.

Definir um caminho estável e apropriado para configurações por usuário em Windows e Linux.

Não assumir que a pasta do executável é gravável.

Não gravar em `assets/`.

Não gravar em `Md-usario/`.

Não alterar saves existentes.

Se a configuração estiver inválida, carregar um preset seguro e informar o problema sem sobrescrever silenciosamente o arquivo corrompido.

---

# 14. CONTROLES DE DESENVOLVIMENTO

O projeto possui comandos e exemplos de desenvolvimento.

Auditar:

- F5.
- F9.
- Respawn.
- Ferramentas de teste.
- Exemplos automatizados.
- Atalhos de debug.
- Controles de probes.

Diferenciar controles de jogador de comandos internos de desenvolvimento.

Evitar que o remapeamento quebre ferramentas automatizadas.

Quando possível, permitir injetar ações lógicas nos testes, sem simular teclas físicas.

Não reescrever os testes existentes apenas para fazê-los passar.

---

# 15. GAMEPAD — PREPARAÇÃO SEM IMPLEMENTAÇÃO

Não implementar suporte completo a gamepad na N2.

A arquitetura deve permitir que um dispositivo futuro produza as mesmas ações lógicas.

Não incluir campos fictícios sem função real apenas para alegar compatibilidade.

Documentar o ponto de extensão.

Não instalar drivers.

Não comprar controles.

Não alterar o escopo para suporte multiplataforma a gamepad.

---

# 16. TESTES UNITÁRIOS

Criar testes para:

1. Preset híbrido.
2. Preset clássico.
3. Múltiplos bindings.
4. W e Espaço para pulo.
5. Mouse para ataque.
6. Ataque por tecla alternativa.
7. Pressionamentos simultâneos.
8. Soltura parcial de bindings.
9. Conflitos.
10. Troca de preset.
11. Restaurar padrão.
12. Serialização.
13. Desserialização.
14. Configuração ausente.
15. Configuração inválida.
16. Binding duplicado.
17. Ação essencial sem binding.
18. Remapeamento de interação.
19. Bloqueio durante captura.
20. Independência entre entrada física e ação lógica.

Testar a semântica de `just_pressed` e `just_released`.

---

# 17. TESTES DE INTEGRAÇÃO

Verificar:

- Movimento real.
- Pulo.
- Agachamento.
- Ataque.
- Guarda.
- Esquiva.
- Magia.
- Interação.
- Diálogo.
- Inventário.
- Mapa.
- Status.
- Salvamento.
- Carregamento.
- Respawn.
- Transformação.
- Comandos especiais.

Executar testes no preset híbrido e no clássico.

Testar remapeamento para teclas diferentes das padrão.

Verificar que o parser recebe o mesmo comando lógico.

Testar que as preferências não alteram o Save v3.

---

# 18. VALIDAÇÃO GRÁFICA

Se o ambiente permitir execução gráfica:

1. Abrir o jogo.
2. Entrar na tela de controles.
3. Conferir legibilidade.
4. Remapear uma tecla.
5. Aplicar.
6. Voltar ao jogo.
7. Testar a ação.
8. Reiniciar.
9. Confirmar persistência.
10. Restaurar padrão.

Produzir capturas ou evidências equivalentes quando disponíveis.

Não declarar validação gráfica concluída apenas com testes headless.

Se a execução gráfica não estiver disponível, registrar explicitamente o que ficou sem validação.

---

# 19. GATES DE QUALIDADE

Executar, conforme as regras do repositório:

`cargo fmt --check`

`cargo check`

`cargo clippy --all-targets -- -D warnings`

`cargo test`

Se o ambiente exigir limitação de paralelismo, utilizar `CARGO_BUILD_JOBS=2`.

Não ocultar warnings.

Não remover testes.

Não usar `#[allow(...)]` indiscriminadamente.

Não modificar dependências sem justificativa.

Registrar resultados reais.

---

# 20. DOCUMENTAÇÃO N2

Criar:

`docs/development/n2/`

Documentos mínimos:

1. `N2_AUDITORIA_INPUT.md`
2. `N2_ARQUITETURA.md`
3. `N2_PRESETS.md`
4. `N2_REMAPEAMENTO.md`
5. `N2_CONTEXTOS.md`
6. `N2_PERSISTENCIA.md`
7. `N2_COMANDOS_ESPECIAIS.md`
8. `N2_TESTES.md`
9. `N2_RELATORIO_FINAL.md`

Atualizar `controles.md` para refletir os controles efetivamente implementados.

Atualizar `docs/DECISIONS.md` apenas quando houver uma decisão nova, sem duplicar a D-036.

Não criar documentação afirmando funcionalidades que ainda não existem.

---

# 21. SEGURANÇA DO REPOSITÓRIO

Preservar integralmente:

- `Md-usario/`
- `Md-usario/DarkCastleR-D/`
- `Md-usario/assets-user/`
- `white_mesh.glb`
- `saves/`
- `assets/`
- `art-src/`
- Documentação N1.
- Sistemas de combate.
- Sistemas de equipamentos.
- Progressão.
- Relíquias.
- Grafo de salas.

Não executar:

- `git reset --hard`
- `git clean -fd`
- `git checkout -- .`
- Exclusões em massa.
- Commit.
- Push.
- Merge.
- Rebase.

Não restaurar arquivos de commits anteriores sem autorização.

Não sobrescrever alterações preexistentes.

Não executar Hunyuan3D.

Não iniciar Blender ou Pixelorama para produção de arte.

---

# 22. CRITÉRIOS DE ACEITAÇÃO

A N2 somente estará concluída quando:

- [ ] O preset híbrido funcionar.
- [ ] O preset clássico funcionar.
- [ ] A/D movimentarem.
- [ ] W e Espaço pularem.
- [ ] S agachar.
- [ ] Mouse esquerdo atacar.
- [ ] Ataque alternativo funcionar.
- [ ] E interagir.
- [ ] Todos os controles essenciais forem remapeáveis.
- [ ] A tela de configuração funcionar.
- [ ] Conflitos forem tratados.
- [ ] Configurações forem persistidas.
- [ ] Restaurar padrões funcionar.
- [ ] Os menus bloquearem entradas indevidas.
- [ ] Os comandos especiais continuarem funcionando.
- [ ] O Save v3 permanecer compatível.
- [ ] Os testes automatizados passarem.
- [ ] A validação gráfica for executada, quando possível.
- [ ] A documentação corresponder ao código.
- [ ] Os arquivos protegidos permanecerem intactos.
- [ ] Nenhum commit ou push for executado.
- [ ] A N3 não tiver começado.

Não marcar um item como concluído sem evidência.

---

# 23. RELATÓRIO FINAL OBRIGATÓRIO

Apresentar:

**A. Estado inicial:** branch, HEAD e working tree.

**B. Auditoria:** como os controles funcionavam antes.

**C. Arquitetura:** como funcionam agora.

**D. Preset híbrido:** mapa completo.

**E. Preset clássico:** mapa completo.

**F. Remapeamento:** interface e fluxo.

**G. Conflitos:** política adotada.

**H. Contextos:** bloqueio de gameplay e menus.

**I. Persistência:** formato, caminho e recuperação.

**J. Comandos especiais:** testes de compatibilidade.

**K. Testes unitários:** resultados.

**L. Testes de integração:** resultados.

**M. Validação gráfica:** evidências.

**N. Qualidade:** fmt, check, clippy, test.

**O. Arquivos alterados:** lista completa.

**P. Documentação:** arquivos criados.

**Q. Segurança:** arquivos preservados.

**R. Pendências:** limitações reais.

**S. Decisões:** o que ainda precisa de aprovação.

**T. Estado final:** Git e confirmação de ausência de commit/push.

---

# 24. INSTRUÇÃO FINAL

A