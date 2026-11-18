# DARK CASTLE RPG — MEGA PROMPT D20.1
## Validação Real do Gameplay, Testes Visuais, Regressão e Preparação para D21

**Projeto:** Dark Castle RPG  
**Tecnologia:** Rust 2024 + Bevy 0.20.1  
**Etapa:** D20.1 — Validação funcional e visual  
**Objetivo:** Verificar o jogo real antes da implementação do primeiro chefe  
**Prioridade:** Estabilidade, evidências e correção de bloqueadores  
**Arte definitiva:** FUTURO — NÃO EXECUTAR AGORA

---

# 0. MISSÃO PRINCIPAL

A D17, D18, D19 e D20 adicionaram sistemas importantes ao Dark Castle RPG.

Entretanto, os relatórios anteriores apresentaram principalmente resultados de:

- `cargo fmt`
- `cargo check`
- `cargo clippy`
- `cargo test`
- Testes ECS/headless

Esses testes são necessários, mas não comprovam que o jogo funciona corretamente quando executado com janela gráfica.

**A D20.1 existe para verificar o jogo real.**

Sua missão é:

1. Auditar o estado atual do repositório.
2. Verificar a disponibilidade de ambiente gráfico.
3. Compilar e iniciar o jogo.
4. Inspecionar o comportamento visual e funcional.
5. Exercitar os sistemas das etapas D17–D20.
6. Produzir evidências verificáveis.
7. Identificar defeitos e classificá-los.
8. Corrigir apenas bloqueadores críticos dentro de um escopo seguro.
9. Executar testes de regressão.
10. Entregar relatório técnico para orientar a D21.

**Não declarar que o jogo foi testado visualmente apenas porque `cargo test` passou.**

---

# 1. ESTADO HISTÓRICO DO PROJETO

O último relatório D20 informou:

- Branch: `art/v1.2-protagonist`.
- HEAD: `a9b8f96`.
- D19 ainda estava local no início da D20.
- Nenhum commit ou push foi realizado pela execução D20.

Essas informações são históricas.

Antes de qualquer ação, confirme o estado real do repositório.

O proprietário informou anteriormente que enviou ao GitHub as alterações até D18.

Não presuma que D19 ou D20 já foram commitadas.

---

# 2. REGRAS DE SEGURANÇA

Antes de editar ou executar processos:

1. Confirmar a raiz do repositório.
2. Executar `git branch --show-current`.
3. Executar `git rev-parse HEAD`.
4. Executar `git status --short`.
5. Inspecionar alterações não commitadas.
6. Identificar arquivos novos das etapas D19 e D20.
7. Preservar todo trabalho preexistente.

## Proibido

- `git reset --hard`
- `git clean -fd`
- `git checkout -- .`
- `git restore .`
- `git add .`
- Commit.
- Push.
- Merge.
- Rebase.
- Cherry-pick.
- Troca automática de branch.

Não sobrescrever arquivos modificados pelo proprietário.

Não limpar diretórios de saída indiscriminadamente.

Não apagar logs, capturas ou artefatos anteriores.

---

# 3. DIRETÓRIOS PROTEGIDOS

Não modificar:

- `Md-usario/`
- `Md-usario/assets-user/`
- `art-src/visual-prototypes/`
- Arquivos Blender originais
- Modelos Hunyuan
- Referências de arte
- Saves reais do usuário
- Experimentos D9–D16
- Configurações de IA não relacionadas
- Assets visuais aprovados

Preservar especialmente:

`Md-usario/assets-user/prota/base/white_mesh.glb`

Não iniciar Hunyuan3D.

Não utilizar Blender MCP.

Não alterar sprites definitivos.

Não integrar o GLB ao jogo nesta etapa.

---

# 4. FASE A — AUDITORIA DO AMBIENTE

Verificar:

- Sistema operacional.
- Versão do Rust.
- Versão do Cargo.
- Versão do Bevy no projeto.
- Bibliotecas nativas necessárias.
- Disponibilidade de servidor gráfico.
- Variáveis `DISPLAY` e `WAYLAND_DISPLAY`, quando aplicáveis.
- Disponibilidade de GPU.
- Drivers gráficos.
- Possibilidade de abrir uma janela Bevy.
- Existência de ambiente de execução headless.

O ambiente histórico era Ubuntu com GPU RTX 4050 Laptop.

Não assumir que a execução atual utiliza o mesmo ambiente.

## Importante

Não instalar pacotes automaticamente.

Não executar `sudo`.

Não modificar drivers.

Não atualizar CUDA.

Não alterar versões do Rust ou Bevy.

Se o ambiente gráfico estiver indisponível, documentar exatamente o motivo.

---

# 5. FASE B — AUDITORIA DOS TESTES EXISTENTES

Identificar:

- Testes unitários.
- Testes de integração.
- Testes ECS/headless.
- Exemplo autoplay.
- Ferramentas de captura.
- Scripts de execução.
- Logs.
- Testes de regressão.

Verificar se o projeto possui mecanismos para:

- Controlar o personagem automaticamente.
- Simular entradas.
- Capturar screenshots.
- Avançar frames.
- Detectar falhas.
- Inspecionar estados.
- Encerrar a execução com segurança.

**Reutilizar mecanismos existentes antes de criar ferramentas novas.**

Não instalar ferramentas de automação desnecessariamente.

---

# 6. FASE C — EXECUTAR O JOGO REAL

Tentar executar:

```bash
cargo run
```

Se houver risco de consumo excessivo de memória:

```bash
CARGO_BUILD_JOBS=2 cargo run
```

Utilizar o comando de execução apropriado ao ambiente real.

## Critérios

- A janela deve abrir.
- O jogo deve carregar dados.
- O personagem deve aparecer.
- A câmera deve funcionar.
- A interface deve aparecer.
- O jogo não deve encerrar inesperadamente.

Registrar:

- Comando executado.
- Horário.
- Duração aproximada.
- Código de saída, quando aplicável.
- Logs relevantes.
- Resultado observado.

Se o processo permanecer aberto como esperado, isso não deve ser interpretado como travamento.

Encerrar somente o processo iniciado para o teste.

Não finalizar processos preexistentes do usuário.

---

# 7. FASE D — EVIDÊNCIAS VISUAIS

Se a janela gráfica funcionar, produzir capturas reais.

Salvar preferencialmente em:

`docs/development/d20_1/evidence/`

Ou seguir a convenção existente.

Capturas desejadas:

1. Tela inicial do jogo.
2. Protagonista em uma sala.
3. HUD com HP/MP.
4. Inimigo Bone Sentry.
5. Inimigo Bone Bolt.
6. Inimigo Ash Warden.
7. Inventário aberto.
8. Consumível selecionado.
9. Altar de salvamento.
10. Menu de viagem rápida.
11. Mapa com altares.
12. Sala após transição.

Não criar imagens artificiais para representar screenshots.

Não usar imagens geradas por IA como evidência.

Não reutilizar screenshots antigas como se fossem novas.

Cada captura deve corresponder a uma execução real identificável.

## Se não houver captura

Registrar:

- Motivo.
- Etapa bloqueada.
- Alternativa utilizada.
- Limitações.

---

# 8. FASE E — TESTAR MOVIMENTAÇÃO

Verificar visualmente:

- Andar.
- Correr.
- Pular.
- Queda.
- Colisão.
- Plataformas atravessáveis.
- Agachar.
- Guarda.
- Esquiva.
- Câmera.
- Orientação do personagem.

Observar:

- Tremores.
- Teleporte inesperado.
- Travamento em paredes.
- Queda através do chão.
- Câmera fora dos limites.
- Desalinhamento visual.
- Estados presos.

Registrar resultados individuais.

---

# 9. FASE F — TESTAR COMBATE

Verificar:

- Ataque básico.
- Hitbox.
- Dano.
- Knockback.
- Invulnerabilidade.
- Morte de inimigo.
- EXP.
- Drops.
- Técnicas especiais.
- Magias.
- Projéteis.
- Feedback de dano.

Não é necessário testar todas as combinações possíveis.

Priorizar fluxos representativos.

Verificar que o personagem não consegue executar ações incompatíveis.

---

# 10. FASE G — TESTAR D17

## Altares de salvamento

Localizar o altar em Gate Approach.

Referência histórica:

`vg_approach_save`

A posição aproximada informada foi x 340–380.

Confirmar posição atual nos dados.

Verificar:

1. Aproximação.
2. Interação com D.
3. Descoberta.
4. Recuperação de HP.
5. Recuperação de MP.
6. Salvamento.
7. Marcação no mapa.
8. Reutilização.
9. Carregamento.

## Proteção dos saves

Não utilizar F5/F9 sobre um slot real do proprietário durante testes automatizados.

Utilizar save temporário ou configuração isolada suportada pelo projeto.

Se não for possível isolar o save com segurança, não executar ações que possam sobrescrevê-lo.

Registrar a limitação.

---

# 11. FASE H — TESTAR D18

## Consumíveis

Verificar:

- Inventário pela tecla I.
- Lista de consumíveis.
- Seleção.
- Uso com Enter.
- Recuperação de HP.
- Recuperação de MP.
- Quantidade decrementada.
- Feedback.
- Persistência.

Fluxo:

1. Obter ou possuir poção.
2. Reduzir HP ou MP.
3. Abrir inventário.
4. Selecionar consumível.
5. Utilizar.
6. Confirmar recuperação.
7. Confirmar redução da quantidade.
8. Salvar em slot isolado.
9. Carregar.
10. Confirmar persistência.

Não alterar os dados iniciais apenas para facilitar o teste sem necessidade.

---

# 12. FASE I — TESTAR D19

## Viagem rápida

Verificar:

- Altar Approach.
- Altar Causeway.
- Descoberta.
- Lista de destinos.
- Menu FastTravel.
- Seleção.
- Confirmação.
- Cancelamento.
- Transição.
- Câmera.
- Posição segura.
- Controles restaurados.

Referências históricas:

- Approach: x ~360.
- Causeway: x ~1940.

Confirmar os valores reais nos arquivos de mapa.

## Teste essencial

Viajar de Causeway para Approach.

Depois, retornar quando possível.

Verificar se a viagem:

- Não cura automaticamente.
- Não salva automaticamente.
- Não altera indevidamente `last_save_point`.
- Não duplica inimigos.
- Não duplica objetos.
- Não deixa a câmera presa.
- Não permite viagem para altar desconhecido.

---

# 13. FASE J — TESTAR D20

## Bone Sentry

Verificar:

- Patrulha.
- Detecção.
- Aproximação.
- Ataque corpo a corpo.
- Cooldown.
- Morte.
- EXP.

## Bone Bolt

Localização histórica:

Shaft, ledge próximo de y 409.

Verificar:

- Detecção.
- Distância.
- Disparo.
- Direção do projétil.
- Colisão com paredes.
- Dano no jogador.
- Recuo.
- Morte.
- EXP.

## Ash Warden

Localização histórica:

Upper Gallery, x ~1600.

Verificar:

- Manutenção de distância.
- Recuo.
- Defesa.
- Ataque curto.
- Cooldown.
- Colisão.
- Morte.
- EXP.

## Questões visuais

Registrar:

- Inimigo preso.
- Inimigo atravessando parede.
- Ataque invisível.
- Projétil desalinhado.
- Dano fora do alcance.
- Movimento irregular.
- Inimigo inativo.
- IA em loop.
- Spawn inacessível.

Não considerar o comportamento visual correto apenas por passar em testes unitários.

---

# 14. FASE K — CLASSIFICAR PROBLEMAS

Criar uma tabela de defeitos.

Campos:

- ID.
- Sistema.
- Localização.
- Descrição.
- Passos para reproduzir.
- Resultado esperado.
- Resultado observado.
- Evidência.
- Severidade.
- Status.

## Severidade

### P0 — Bloqueador

Impede iniciar ou continuar o jogo, causa corrupção de save ou travamento crítico.

### P1 — Alto

Quebra um sistema essencial ou impede concluir um fluxo principal.

### P2 — Médio

Falha funcional localizada com alternativa disponível.

### P3 — Baixo

Problema visual ou de usabilidade sem impacto crítico.

Não classificar todos os problemas como P0.

---

# 15. FASE L — POLÍTICA DE CORREÇÕES

A D20.1 é prioritariamente uma etapa de validação.

Pode corrigir:

- Falha que impede inicialização.
- Crash reproduzível.
- Erro grave de transição.
- Erro que impede testar o jogo.
- Problema crítico de input.
- Regressão diretamente causada por D17–D20.

Somente quando:

1. O defeito foi reproduzido.
2. A causa foi identificada.
3. A correção é pequena e localizada.
4. Existe teste de regressão.
5. Não há risco relevante aos dados do usuário.

Não corrigir automaticamente:

- Problemas arquiteturais amplos.
- Sistemas de chefe.
- Balanceamento geral.
- Animações definitivas.
- Remodelagem visual.
- Mudanças de engine.
- Reformulação de UI.

Para alterações grandes ou arriscadas, documentar e solicitar autorização.

---

# 16. FASE M — TESTES AUTOMATIZADOS

Executar:

```bash
cargo fmt --check
CARGO_BUILD_JOBS=2 cargo check
CARGO_BUILD_JOBS=2 cargo clippy --all-targets -- -D warnings
CARGO_BUILD_JOBS=2 cargo test
```

Verificar especificamente os testes relacionados a:

- Save points.
- Consumíveis.
- Fast travel.
- Enemy archetypes.
- Transições.
- Combate.
- Persistência.

Se existir autoplay, tentar utilizá-lo de forma segura.

Não executar múltiplos builds pesados simultaneamente.

---

# 17. FASE N — AUTOMAÇÃO VISUAL

Se o projeto já possui exemplo autoplay ou captura de screenshots, investigar a possibilidade de ampliar a cobertura.

Preferir:

- Entrada simulada.
- Execução determinística.
- Capturas reais.
- Estado conhecido.
- Tempo limitado.
- Logs.
- Encerramento controlado.

Não criar uma infraestrutura extensa de automação gráfica nesta etapa.

Não instalar dependências pesadas sem necessidade.

Não afirmar que um autoplay cobre todos os sistemas se ele apenas movimenta o personagem.

## Distinção obrigatória

Registrar separadamente:

- Teste de compilação.
- Teste unitário.
- Teste headless.
- Teste com janela.
- Teste de interação automatizada.
- Teste manual humano.

---

# 18. FASE O — DESEMPENHO

Observar, quando possível:

- Tempo de inicialização.
- Travamentos.
- Consumo de RAM.
- Consumo de CPU.
- Uso de GPU.
- Quedas perceptíveis de FPS.
- Problemas ao trocar de sala.

Não realizar benchmarks extensos.

Não iniciar Hunyuan ou Blender durante os testes.

Não executar várias instâncias do jogo simultaneamente.

Se coletar FPS, identificar a ferramenta e o método utilizados.

Não inventar métricas.

---

# 19. FASE P — PREPARAÇÃO PARA D21

Após a validação, revisar a preparação de chefes produzida na D20.

Referência:

`docs/development/d20/D20_PREPARACAO_CHEFES.md`

Inspecionar esse arquivo real.

Identificar:

- Infraestrutura reutilizável.
- Estados de IA.
- Ataques.
- Fases.
- Arena.
- Persistência de derrota.
- Recompensas.
- Barra de HP.
- Save.
- Testes necessários.

Não implementar o chefe nesta etapa.

Produzir uma recomendação para a D21 baseada nos resultados.

---

# 20. FASE Q — MIXAMO E ARTE FUTURA

**ESTADO: FUTURO — NÃO EXECUTAR AGORA**

O proprietário identificou o Adobe Mixamo como possível fonte de rigging automático e animações para personagens humanoides.

Site:

https://www.mixamo.com/

Registrar o Mixamo como ferramenta candidata no pipeline futuro.

## Pipeline planejado

1. ChatGPT gera a imagem de referência.
2. Proprietário aprova a aparência.
3. Imagem é armazenada em `Md-usario/assets-user/`.
4. Hunyuan3D local gera o modelo 3D.
5. GLB original é preservado.
6. Blender prepara uma cópia da malha.
7. Modelo é convertido para formato aceito pelo Mixamo, quando necessário.
8. Mixamo realiza auto-rigging, quando compatível.
9. Animações são selecionadas e obtidas.
10. Blender importa e adapta as animações.
11. Rigging e deformações são verificados.
12. Animações são renderizadas em sprites 2D.
13. Sprites são integrados ao Bevy.
14. Resultado é testado e aprovado.

## Restrições

- Não iniciar Hunyuan.
- Não modificar o GLB original.
- Não executar Blender.
- Não baixar animações nesta etapa.
- Não automatizar Mixamo.
- Não substituir sprites atuais.
- Verificar licença e condições de uso antes da distribuição.
- Não presumir que o auto-rigging funcionará sem ajustes.

O arquivo experimental permanece em:

`Md-usario/assets-user/prota/base/white_mesh.glb`

**Não utilizar esse arquivo durante a D20.1.**

---

# 21. DOCUMENTAÇÃO

Criar:

`docs/development/d20_1/`

Arquivos sugeridos:

- `D20_1_AMBIENTE.md`
- `D20_1_TESTES_VISUAIS.md`
- `D20_1_REGRESSAO.md`
- `D20_1_DEFEITOS.md`
- `D20_1_PREPARACAO_D21.md`
- `D20_1_RELATORIO_FINAL.md`

E diretório:

`docs/development/d20_1/evidence/`

Para cada evidência, registrar:

- Nome do arquivo.
- Data.
- Comando.
- Cenário.
- Resultado.
- Limitação.

Não incluir screenshots artificiais.

Não afirmar que um teste visual passou sem observação real.

---

# 22. CRITÉRIOS DE ACEITAÇÃO

## Ambiente

- [ ] Estado Git registrado.
- [ ] Ambiente gráfico verificado.
- [ ] Jogo iniciado ou bloqueio documentado.

## Gameplay

- [ ] Movimentação verificada.
- [ ] Combate verificado.
- [ ] D17 verificada.
- [ ] D18 verificada.
- [ ] D19 verificada.
- [ ] D20 verificada.

## Evidências

- [ ] Logs reais.
- [ ] Capturas reais, quando possível.
- [ ] Resultados por cenário.
- [ ] Defeitos classificados.
- [ ] Limitações identificadas.

## Qualidade

- [ ] Cargo fmt.
- [ ] Cargo check.
- [ ] Clippy.
- [ ] Cargo test.
- [ ] Regressões.

## Segurança

- [ ] Sem commit.
- [ ] Sem push.
- [ ] Saves reais preservados.
- [ ] Arte preservada.
- [ ] Hunyuan não utilizado.
- [ ] Blender não utilizado.
- [ ] GLB original preservado.

**Não marque como concluído um cenário que não foi realmente executado.**

---

# 23. RELATÓRIO FINAL OBRIGATÓRIO

Responder com:

## A. Estado inicial
Branch, HEAD e Git.

## B. Ambiente
Sistema operacional, GPU, janela gráfica e limitações.

## C. Execução real
Comando, resultado, duração e logs.

## D. Testes visuais
Cenários executados, resultados e capturas.

## E. D17
Altares e salvamento.

## F. D18
Consumíveis.

## G. D19
Viagem rápida.

## H. D20
Inimigos e IA.

## I. Defeitos
IDs, severidade, reprodução e evidências.

## J. Correções
Arquivos alterados, testes e riscos.

## K. Segurança
Git, saves, arte e GLB preservados.

## L. D21
Recomendação técnica para o primeiro chefe.

## M. Arte futura
Confirmar Mixamo/Hunyuan/Blender como pipeline futuro não executado.

---

# 24. DIRETRIZ FINAL

Esta etapa não é sobre adicionar conteúdo.

É sobre descobrir se o jogo que construímos funciona de verdade.

**Não confundir compilação com gameplay validado.**

**Não confundir testes headless com testes visuais.**

**Não criar screenshots artificiais.**

**Não modificar o protagonista 3D.**

**Não iniciar produção visual.**

**Não fazer commit nem push.**

Execute o máximo de testes reais que o ambiente permitir, documente evidências, corrija apenas bloqueadores críticos e prepare a D21.

**D20.1 — OBJETIVO FINAL: obter uma avaliação confiável do estado jogável do Dark Castle RPG antes de desenvolver o primeiro chefe.**