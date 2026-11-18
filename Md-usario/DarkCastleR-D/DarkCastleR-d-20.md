# DARK CASTLE RPG — MEGA PROMPT D20
## Variedade de Inimigos, Inteligência Artificial e Preparação para Chefes

**Projeto:** Dark Castle RPG  
**Tecnologia:** Rust 2024 + Bevy 0.20.1  
**Prioridade:** Gameplay e conteúdo funcional  
**Arte definitiva:** FUTURO — NÃO EXECUTAR AGORA

---

# 0. MISSÃO

Evoluir o sistema de inimigos do Dark Castle RPG, criando maior variedade de comportamentos e desafios durante a exploração.

O projeto já possui, conforme os relatórios anteriores:

- Movimentação e combate.
- Técnicas e magias.
- Progressão e experiência.
- Inventário e equipamentos.
- Consumíveis.
- NPCs e comércio.
- Salas e transições.
- Altares de salvamento.
- Viagem rápida.
- Save V3.

A D20 deve aproveitar essa estrutura para fortalecer a experiência de combate contra inimigos comuns.

**Não iniciar a implementação de um chefe completo nesta etapa.**

O objetivo é construir uma base confiável para que a D21 possa introduzir chefes com comportamentos mais complexos.

---

# 1. SEGURANÇA E GIT

Antes de qualquer alteração:

1. Confirmar branch.
2. Confirmar HEAD.
3. Executar `git status --short`.
4. Identificar mudanças preexistentes.
5. Examinar arquivos afetados pela D19.
6. Preservar trabalho não commitado.

O último estado informado foi:

- Branch: `art/v1.2-protagonist`
- HEAD: `a9b8f96`

Não presumir que esses valores permanecem atuais.

Não executar comandos destrutivos.

Não criar branch automaticamente.

Não fazer commit.

Não fazer push.

Não sobrescrever alterações do proprietário.

---

# 2. DIRETÓRIOS PROTEGIDOS

Preservar:

- `Md-usario/`.
- `Md-usario/assets-user/`.
- `art-src/visual-prototypes/`.
- Arquivos Blender.
- Modelos Hunyuan.
- Saves reais.
- Arte aprovada.
- Experimentos anteriores.

Preservar especificamente:

`Md-usario/assets-user/prota/base/white_mesh.glb`

Não iniciar Hunyuan.

Não utilizar Blender MCP.

Não gerar sprites definitivos.

Não modificar a identidade visual do protagonista.

---

# 3. FASE A — AUDITORIA DOS INIMIGOS

Antes de implementar, examine:

- Definições de inimigos.
- Dados RON.
- Spawn.
- IA.
- Estados.
- Patrulha.
- Perseguição.
- Ataques.
- Distância de detecção.
- Distância de ataque.
- Dano.
- Hitboxes e hurtboxes.
- Recuo.
- Invulnerabilidade.
- Morte.
- EXP.
- Drops.
- Respawn.
- Persistência.
- Testes.

Verifique quais comportamentos já existem e quais estão incompletos.

Não recrie sistemas existentes.

Documente o diagnóstico com evidências do código.

---

# 4. FASE B — DEFINIR ARQUÉTIPOS

Queremos inimigos com comportamentos distintos.

Considere estes arquétipos:

## A. Patrulheiro corpo a corpo

- Patrulha uma área.
- Detecta o jogador.
- Aproxima-se.
- Ataca em curta distância.
- Respeita cooldown.
- Retorna à patrulha quando perde o alvo.

## B. Inimigo agressivo

- Detecta a uma distância maior.
- Persegue com mais insistência.
- Executa ataques rápidos.
- Possui janelas claras de recuperação.
- Não ataca continuamente sem intervalo.

## C. Inimigo defensivo

- Mantém distância adequada.
- Pode bloquear ou recuar.
- Aguarda oportunidades.
- Ataca quando o jogador se aproxima.

## D. Inimigo à distância

- Mantém distância.
- Utiliza projéteis, se a arquitetura suportar.
- Respeita linha de visão quando aplicável.
- Possui cooldown.
- Não dispara através de paredes sólidas.

Esses são arquétipos candidatos.

**Não implementar todos obrigatoriamente.**

Selecione dois comportamentos distintos que ainda não estejam adequadamente representados, conforme a auditoria.

A D20 deve entregar uma melhoria concreta e controlada.

---

# 5. FASE C — IA ORIENTADA A DADOS

Preferir definições configuráveis.

Parâmetros possíveis:

- Velocidade.
- HP.
- Dano.
- Distância de detecção.
- Distância de perseguição.
- Distância de ataque.
- Tempo de preparação.
- Cooldown.
- Tempo de recuperação.
- Comportamento.
- EXP.
- Drops.

Não codificar cada inimigo como um sistema Rust inteiramente independente.

Reutilizar componentes e estados existentes.

Evitar uma arquitetura excessivamente genérica.

O objetivo é permitir adicionar inimigos por dados sempre que a mecânica existente suportar.

---

# 6. FASE D — MÁQUINA DE ESTADOS

Auditar os estados atuais.

Considerar:

- Idle.
- Patrol.
- Alert.
- Chase.
- PrepareAttack.
- Attack.
- Recover.
- Retreat.
- Stunned.
- Dead.

Não substituir a máquina de estados atual sem justificativa.

Adicionar somente os estados necessários.

## Regras

- Inimigo morto não ataca.
- Inimigo atordoado não executa ações incompatíveis.
- Ataque respeita cooldown.
- Transições são determinísticas.
- Estados não entram em loops infinitos.
- Mudança de sala não deixa IA antiga ativa.
- Inimigos não atravessam paredes por decisão da IA.

---

# 7. FASE E — COMBATE LEGÍVEL

Os ataques devem oferecer oportunidade de reação ao jogador.

Para cada comportamento implementado, avaliar:

1. Detecção.
2. Preparação do ataque.
3. Execução.
4. Hitbox ativa.
5. Recuperação.
6. Retorno ao comportamento normal.

Não é necessário criar animações definitivas.

Utilizar placeholders existentes para comunicar os estados.

Se possível, utilizar alterações simples de cor ou indicadores temporários já suportados pela apresentação.

Não criar um novo pipeline visual.

---

# 8. FASE F — COMBATE E FÍSICA

Garantir integração com:

- Colisão.
- Gravidade.
- Plataformas.
- Hitboxes.
- Hurtboxes.
- Dano.
- Knockback.
- Invulnerabilidade.
- Magias.
- Técnicas especiais.
- Morte.

Investigar especialmente:

- Inimigo atravessando parede.
- Inimigo caindo de plataforma indevidamente.
- Ataque atingindo fora do alcance.
- Ataque repetido no mesmo frame.
- Inimigo atacando depois de morto.
- Inimigo perseguindo através de salas.
- Projétil atravessando geometria sólida.

Não alterar a física global sem necessidade comprovada.

---

# 9. FASE G — RECOMPENSAS

Todo inimigo novo ou alterado deve respeitar os sistemas existentes de:

- EXP.
- Drops.
- Inventário.
- Progressão.
- Morte.
- Respawn.

Não duplicar concessão de EXP.

Não gerar drops repetidos por múltiplos eventos de morte.

Não alterar a economia do jogo indiscriminadamente.

Reutilizar as tabelas de loot existentes.

---

# 10. FASE H — INTEGRAÇÃO COM O MUNDO

Escolher salas existentes adequadas para testar os arquétipos.

Não criar uma região inteira.

Não remover inimigos existentes sem justificativa.

Verificar:

- Posições de spawn.
- Colisão.
- Distância entre inimigos.
- Dificuldade.
- Acessibilidade.
- Transições de sala.
- Respawn.
- Limites de perseguição.

O jogador deve conseguir enfrentar os novos comportamentos durante uma sessão normal.

---

# 11. FASE I — PREPARAÇÃO PARA CHEFES

Não implementar um chefe completo.

Preparar um diagnóstico técnico para a D21.

Verificar se a arquitetura suporta futuramente:

- Identificador de chefe.
- HP elevado.
- Múltiplos ataques.
- Seleção de ataques.
- Fases.
- Mudanças de comportamento por HP.
- Arena.
- Morte persistente.
- Recompensa única.
- Portas bloqueadas durante combate.
- Barra de HP de chefe.
- Save após vitória.

Identificar o que já existe e o que falta.

**Não adicionar sistemas de chefe apenas por antecipação.**

Produzir uma proposta de arquitetura com mudanças mínimas para a D21.

---

# 12. FASE J — TESTES UNITÁRIOS

Criar testes para:

- Detecção.
- Perda do alvo.
- Patrulha.
- Perseguição.
- Alcance de ataque.
- Cooldown.
- Preparação.
- Recuperação.
- Estado inválido.
- Morte.
- Dano.
- EXP.
- Drops.
- Mudança de sala.

Testar os novos comportamentos implementados.

Não criar testes fictícios que apenas verificam constantes.

---

# 13. FASE K — TESTES HEADLESS

Criar ou ampliar testes ECS para verificar:

1. Spawn de inimigo.
2. Aquisição de alvo.
3. Mudança de estado.
4. Aproximação.
5. Preparação de ataque.
6. Execução de ataque.
7. Aplicação de dano.
8. Cooldown.
9. Morte.
10. EXP.
11. Drop, quando aplicável.
12. Remoção ou respawn.
13. Troca de sala.

Testar ao menos dois arquétipos com diferenças reais de comportamento.

Verificar que o sistema não cria entidades duplicadas.

---

# 14. FASE L — REGRESSÕES

Validar especialmente:

## D17

- Altares.
- Descanso.
- Cura.
- Salvamento.

## D18

- Consumíveis.
- Inventário.
- HP/MP.
- Persistência.

## D19

- Viagem rápida.
- Troca de salas.
- Descoberta de altares.
- Retorno de controles.
- Ausência de entidades antigas.

## Gameplay geral

- Ataque básico.
- Técnicas.
- Magias.
- Morte.
- Respawn.
- EXP.
- Equipamentos.

Não quebrar o ciclo jogável para adicionar variedade de inimigos.

---

# 15. FASE M — QUALIDADE

Executar sequencialmente:

```bash
cargo fmt --check
CARGO_BUILD_JOBS=2 cargo check
CARGO_BUILD_JOBS=2 cargo clippy --all-targets -- -D warnings
CARGO_BUILD_JOBS=2 cargo test
```

Se possível, executar o jogo.

Testar visualmente os novos inimigos.

Diferenciar:

- Testes unitários.
- Testes de integração.
- Testes headless.
- Testes manuais.

Não declarar sucesso de teste manual sem execução real.

---

# 16. FASE N — DOCUMENTAÇÃO

Criar documentação da etapa em:

`docs/development/d20/`

Respeitar a convenção atual do repositório.

Arquivos sugeridos:

- `D20_AUDITORIA.md`
- `D20_DESIGN.md`
- `D20_IMPLEMENTACAO.md`
- `D20_TESTES.md`
- `D20_RELATORIO_FINAL.md`
- `D20_PREPARACAO_CHEFES.md`

Documentar:

- Sistemas reutilizados.
- Arquétipos selecionados.
- Regras de IA.
- Estados.
- Parâmetros RON.
- Arquivos alterados.
- Testes.
- Limitações.
- Preparação para D21.

---

# 17. ARTE DEFINITIVA — FUTURO

**NÃO EXECUTAR NA D20.**

Preservar o pipeline aprovado:

1. ChatGPT gera imagens de referência.
2. Proprietário aprova.
3. Referências são salvas em `Md-usario/assets-user/`.
4. Imagens são separadas manualmente quando necessário.
5. Hunyuan3D gera geometria.
6. GLBs originais são preservados.
7. Blender prepara modelos e rigging.
8. Animações são produzidas.
9. Sprites 2D são renderizados.
10. Assets são testados no Bevy.
11. Integração ocorre após aprovação.

Esse pipeline será aplicado futuramente a:

- Protagonista.
- Equipamentos.
- Inimigos comuns.
- Chefes.
- NPCs.
- Transformações.

Não executar nenhuma dessas etapas agora.

---

# 18. CRITÉRIOS DE ACEITAÇÃO

## Auditoria

- [ ] Estado real do repositório registrado.
- [ ] Sistemas de IA existentes analisados.
- [ ] Lacunas identificadas.
- [ ] Dois arquétipos selecionados.

## Implementação

- [ ] Dois comportamentos distintos funcionais.
- [ ] Estados coerentes.
- [ ] Ataques com cooldown.
- [ ] Colisão respeitada.
- [ ] Dano integrado.
- [ ] Morte integrada.
- [ ] EXP e drops preservados.
- [ ] Spawn em salas existentes.
- [ ] Sem duplicação de entidades.

## Qualidade

- [ ] Testes unitários.
- [ ] Testes headless.
- [ ] Regressão D17.
- [ ] Regressão D18.
- [ ] Regressão D19.
- [ ] Cargo fmt.
- [ ] Cargo check.
- [ ] Clippy sem warnings.
- [ ] Cargo test.
- [ ] Teste manual ou limitação documentada.

## Segurança

- [ ] Sem commit.
- [ ] Sem push.
- [ ] Sem alterações destrutivas.
- [ ] Arte preservada.
- [ ] Hunyuan não utilizado.
- [ ] Blender não utilizado.
- [ ] GLB original preservado.

---

# 19. RELATÓRIO FINAL

Ao terminar, responder:

## A. Estado inicial
Branch, HEAD e Git.

## B. Auditoria
O que já existia e o que faltava.

## C. Arquétipos
Quais inimigos foram selecionados e por quê.

## D. Implementação
Arquivos, componentes, sistemas e dados.

## E. Comportamento
Como cada inimigo age.

## F. Integração
Combate, física, EXP, drops e salas.

## G. Testes
Resultados e evidências.

## H. Teste manual
Como encontrar e enfrentar os inimigos.

## I. Segurança
Arquivos preservados e ausência de commit/push.

## J. Preparação D21
Arquitetura necessária para o primeiro chefe.

## K. Limitações
Pendências e riscos.

---

# 20. DIRETRIZ FINAL

O objetivo da D20 não é apenas aumentar a quantidade de inimigos.

É criar **variedade real de comportamento**.

O jogador deve perceber diferenças entre os adversários e precisar adaptar suas decisões de combate.

Priorize:

1. Reutilização.
2. IA consistente.
3. Ataques legíveis.
4. Integração com combate.
5. Testes.
6. Estabilidade.
7. Documentação.

Não implementar chefe completo.

Não iniciar produção visual.

Não fazer commit nem push.

**D20 — OBJETIVO FINAL: entregar pelo menos dois comportamentos de inimigos distintos e testados, preparando o Dark Castle RPG para a implementação de chefes na D21.**