# DARK CASTLE RPG — MEGA PROMPT D22
## Validação Real do Primeiro Chefe, Telégrafos de Ataque, Recompensas e Polimento de Combate

**Projeto:** Dark Castle RPG  
**Tecnologia:** Rust 2024 + Bevy 0.20.1  
**Etapa:** D22  
**Prioridade:** Qualidade do gameplay e confiabilidade  
**Arte definitiva:** FUTURO — NÃO EXECUTAR AGORA

---

# 0. MISSÃO PRINCIPAL

A D21 implementou o primeiro chefe, The Ashen Sentinel, localizado na Ashen Court.

Segundo o relatório, já existem:

- Chefe orientado a dados.
- Arena com bloqueio de saída.
- Barra de HP.
- FrontalSlash.
- Lunge.
- AshWave.
- Duas fases.
- Recompensa `sentinel_brand`.
- Persistência em `defeated_bosses`.
- Integração com Save V3.
- Testes automatizados.
- Probe gráfico.

Entretanto, o probe da D21 utilizou dano injetado para derrotar rapidamente o chefe.

**A D22 deve validar e melhorar o combate real, sem mascarar falhas por meio de dano artificial.**

## Objetivos

1. Testar uma luta completa usando o pipeline real de ataques do jogador.
2. Confirmar que os três ataques do chefe realmente acontecem.
3. Melhorar a leitura visual da preparação e recuperação dos ataques.
4. Verificar colisões, esquiva, guarda e dano.
5. Confirmar que a segunda fase é jogável.
6. Validar a derrota do jogador e a possibilidade de tentar novamente.
7. Melhorar o feedback da vitória e da recompensa.
8. Garantir que a arena não fique bloqueada indevidamente.
9. Produzir evidências gráficas reais.
10. Preservar os sistemas e a arte existentes.

Não implementar um segundo chefe nesta etapa.

---

# 1. SEGURANÇA E GIT

Antes de modificar qualquer arquivo:

- Confirmar branch.
- Confirmar HEAD.
- Executar `git status --short`.
- Identificar alterações preexistentes.
- Preservar D19, D20, D20.1 e D21.
- Verificar alterações do proprietário.

Último estado informado:

`art/v1.2-protagonist @ a9b8f96`

O working tree estava dirty.

Não presumir que continua igual.

**Não fazer commit nem push.**

Não executar operações destrutivas de Git.

Não sobrescrever alterações não relacionadas.

---

# 2. AUDITORIA OBRIGATÓRIA

Ler os documentos reais de:

- `docs/development/d20/`
- `docs/development/d20_1/`
- `docs/development/d21/`

Inspecionar:

- `BossDefinition`.
- Sistema de encontros.
- Estados de IA.
- FrontalSlash.
- Lunge.
- AshWave.
- Hitboxes.
- Projéteis.
- Barra de HP.
- `TransitionLock`.
- `defeated_bosses`.
- Recompensas.
- Save V3.
- Testes de chefe.
- `d21_boss_probe`.

Identificar exatamente o que já funciona e quais lacunas permanecem.

Não recriar mecanismos existentes.

---

# 3. FASE A — LUTA REAL SEM DANO INJETADO

Criar um cenário de validação em que o chefe receba dano pelo pipeline normal de combate.

O teste deverá:

1. Entrar na arena.
2. Iniciar o encontro.
3. Esperar a preparação de um ataque.
4. Observar sua execução.
5. Permitir recuperação.
6. Atacar o chefe com ações reais do jogador.
7. Reduzir seu HP gradualmente.
8. Ativar a segunda fase.
9. Observar AshWave.
10. Continuar o combate.
11. Derrotar o chefe.
12. Confirmar vitória e recompensa.

Pode utilizar entrada automatizada ou um driver de combate, desde que o dano passe pelos sistemas normais de ataque, hitbox e resolução.

**Não utilizar injeção direta de HP como prova de combate real.**

Testes unitários que modificam HP diretamente continuam permitidos para verificar limites de fase, mas devem ser identificados como testes de lógica.

Se não for possível automatizar uma luta completa, não falsificar o resultado: fornecer um teste manual reproduzível.

---

# 4. FASE B — TELÉGRAFOS DE ATAQUE

Melhorar a legibilidade dos três ataques.

## FrontalSlash

- Preparação perceptível.
- Direção identificável.
- Hitbox ativa somente na janela correta.
- Recuperação após o golpe.

## Lunge

- Indicação clara de avanço.
- Movimento respeitando colisões.
- Alcance limitado.
- Recuperação que permita contra-ataque.

## AshWave

- Preparação distinta.
- Projétil visível.
- Direção coerente.
- Colisão com sólidos.
- Janela de reação.
- Recuperação.

Utilizar placeholders, cores e indicadores simples.

Não criar sprites definitivos.

Não iniciar produção visual.

---

# 5. FASE C — SEGUNDA FASE

Validar o comportamento abaixo de 50% de HP.

Confirmar:

- Transição ocorre uma única vez.
- `phase_lock` impede ataques incompatíveis.
- Barra de HP continua correta.
- AshWave entra na seleção de ataques.
- O chefe não fica preso em transição.
- O jogador consegue reagir.
- O combate pode terminar normalmente.

Produzir evidência de AshWave durante a luta, não apenas da existência de sua definição.

---

# 6. FASE D — DEFESA DO JOGADOR

Testar:

- Esquiva contra FrontalSlash.
- Esquiva contra Lunge.
- Interação da guarda com ataques físicos.
- Colisão do projétil AshWave.
- Invulnerabilidade.
- Knockback.
- Morte do jogador.

Não alterar globalmente as regras de defesa sem necessidade.

Se algum ataque for intencionalmente impossível de bloquear, documentar essa regra e garantir feedback adequado.

---

# 7. FASE E — DERROTA E NOVA TENTATIVA

Testar o ciclo:

1. Iniciar encontro.
2. Receber dano.
3. Morrer.
4. Executar respawn.
5. Retornar à arena.
6. Iniciar uma nova tentativa.

Verificar:

- Chefe volta com HP completo.
- Barra de HP não fica duplicada.
- Projéteis antigos são removidos.
- Bloqueios são restaurados corretamente.
- O jogador não fica preso.
- Nenhuma recompensa é concedida após derrota.

---

# 8. FASE F — VITÓRIA E RECOMPENSA

Auditar a entrega de `sentinel_brand`.

Verificar:

- Recompensa concedida uma vez.
- Item aparece no inventário.
- Feedback de obtenção é visível.
- EXP é concedida corretamente.
- Portas são liberadas.
- Chefe deixa de atacar.
- Projéteis são removidos.
- Barra de HP desaparece.

Se o sistema atual já possui notificações adequadas, reutilizá-las.

Não implementar obrigatoriamente um novo sistema de baús.

Um baú de recompensa pode ficar como proposta futura caso exija arquitetura adicional.

---

# 9. FASE G — PERSISTÊNCIA

Validar Save V3 com saves isolados.

Cenários:

- Salvar antes da luta.
- Carregar antes da luta.
- Derrotar o chefe.
- Salvar após vitória.
- Carregar após vitória.
- Retornar à arena.
- Confirmar chefe ausente.
- Confirmar recompensa única.
- Confirmar portas desbloqueadas.
- Confirmar inventário e progressão.

Não modificar saves reais.

Não alterar a versão do save sem necessidade comprovada.

---

# 10. FASE H — ARENA E ACESSIBILIDADE

A D21 informou que a entrada exige um salto no Reliquary.

Verificar visualmente:

- Se a porta é identificável.
- Se o salto é possível com os controles atuais.
- Se o jogador consegue alcançar a entrada sem conhecimento oculto.
- Se a câmera acompanha a chegada.
- Se a posição inicial da arena é segura.
- Se o jogador não nasce dentro do chefe.

Corrigir somente problemas funcionais comprovados.

Não reconstruir a região.

---

# 11. FASE I — TESTES AUTOMATIZADOS

Ampliar os testes existentes para verificar:

- Três ataques executáveis.
- Janelas de preparação.
- Janelas de dano.
- Recuperação.
- Mudança de fase.
- Ausência de ataques duplicados.
- Colisão do projétil.
- Morte do jogador.
- Reinício do encontro.
- Vitória.
- Recompensa única.
- Persistência.
- Desbloqueio da arena.

Reutilizar `boss_encounter_tests` e a infraestrutura ECS existente.

---

# 12. FASE J — TESTE GRÁFICO REAL

Executar o jogo em janela gráfica, se o ambiente permitir.

Reutilizar os exemplos de D20.1 e D21.

Capturar evidências de:

1. Entrada da arena.
2. Início do encontro.
3. FrontalSlash.
4. Lunge.
5. Mudança de fase.
6. AshWave em execução.
7. HP do chefe diminuindo por combate normal.
8. Vitória.
9. Recompensa.
10. Arena após a vitória.

Não criar screenshots artificiais.

Não chamar teste headless de teste gráfico.

Não considerar dano injetado equivalente a uma luta real.

Se a execução gráfica falhar, documentar o motivo.

---

# 13. FASE K — REGRESSÕES

Executar testes das etapas anteriores:

- D17 — Altares.
- D18 — Consumíveis.
- D19 — Viagem rápida.
- D20 — Inimigos.
- D20.1 — Autoplay e probe.
- D21 — Chefe e persistência.

Garantir que as alterações da D22 não quebrem as rotas existentes.

---

# 14. FASE L — QUALIDADE

Executar sequencialmente:

```bash
cargo fmt --check
CARGO_BUILD_JOBS=2 cargo check
CARGO_BUILD_JOBS=2 cargo clippy --all-targets -- -D warnings
CARGO_BUILD_JOBS=2 cargo test
```

Executar os probes relevantes.

Registrar os resultados reais.

Não rodar vários processos Cargo pesados simultaneamente.

---

# 15. FASE M — DOCUMENTAÇÃO

Criar:

`docs/development/d22/`

Documentos sugeridos:

- `D22_AUDITORIA.md`
- `D22_COMBATE_REAL.md`
- `D22_TELEGRAFOS.md`
- `D22_TESTES.md`
- `D22_DEFEITOS.md`
- `D22_RELATORIO_FINAL.md`

E:

`docs/development/d22/evidence/`

Registrar:

- Comandos executados.
- Cenários testados.
- Capturas.
- Defeitos.
- Correções.
- Limitações.
- Evidências de combate sem dano injetado.

---

# 16. ARTE E ANIMAÇÕES — FUTURO

**NÃO EXECUTAR AGORA.**

Preservar:

`Md-usario/assets-user/prota/base/white_mesh.glb`

O pipeline futuro permanece:

1. ChatGPT — referências visuais.
2. Hunyuan3D — geometria.
3. Blender — preparação e conversão.
4. Mixamo — rigging e animações humanoides quando compatível.
5. Blender — adaptação e renderização de sprites.
6. Bevy — integração e validação.

Não abrir Blender.

Não iniciar Hunyuan.

Não baixar animações do Mixamo.

Não alterar o GLB.

Não substituir os placeholders atuais.

---

# 17. CRITÉRIOS DE ACEITAÇÃO

- [ ] Luta completa testada sem dano injetado.
- [ ] FrontalSlash observado.
- [ ] Lunge observado.
- [ ] AshWave observado.
- [ ] Mudança de fase funcional.
- [ ] Telégrafos legíveis.
- [ ] Defesa e esquiva verificadas.
- [ ] Derrota do jogador testada.
- [ ] Nova tentativa funcional.
- [ ] Vitória funcional.
- [ ] Recompensa única.
- [ ] Persistência Save V3.
- [ ] Arena desbloqueada após vitória.
- [ ] Testes automatizados aprovados.
- [ ] Teste gráfico realizado ou limitação registrada.
- [ ] Evidências reais produzidas.
- [ ] Sem commit/push.
- [ ] Arte e GLB preservados.

**Não marcar como concluído um item que não foi efetivamente verificado.**

---

# 18. RELATÓRIO FINAL

Entregar:

## A. Estado inicial
Branch, HEAD e alterações existentes.

## B. Auditoria
O que já funcionava.

## C. Combate real
Como foi validado sem dano artificial.

## D. Ataques
FrontalSlash, Lunge e AshWave.

## E. Fases
Transição e comportamento.

## F. Defesa
Guarda, esquiva, dano e invulnerabilidade.

## G. Derrota
Morte e nova tentativa.

## H. Vitória
Recompensa e desbloqueio.

## I. Persistência
Save V3.

## J. Testes
Unitários, integração e execução gráfica.

## K. Evidências
Arquivos, cenários e resultados.

## L. Segurança
Git, saves e arte.

## M. Limitações
Pendências e defeitos.

## N. Próxima etapa
Recomendação fundamentada para D23.

---

# 19. DIRETRIZ FINAL

A D22 deve transformar o primeiro chefe de uma implementação tecnicamente funcional em um encontro efetivamente validado.

**Não criar novos chefes.**

**Não esconder problemas com dano injetado.**

**Não confundir teste automatizado com avaliação humana da dificuldade.**

**Não iniciar produção visual definitiva.**

**Não fazer commit nem push.**

Entregar um encontro verificável, estável, legível e preparado para expansão futura.

**D22 — OBJETIVO FINAL: validar uma luta completa contra The Ashen Sentinel, com ataques reais, duas fases, recompensa persistente e evidências gráficas confiáveis.**