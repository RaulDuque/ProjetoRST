# DARK CASTLE RPG — MEGA PROMPT D23
## Playtest Humano, Balanceamento, Legibilidade e Acesso à Arena

**Projeto:** Dark Castle RPG  
**Tecnologia:** Rust 2024 + Bevy 0.20.1  
**Etapa:** D23  
**Prioridade:** Experiência real do jogador  
**Arte definitiva:** FUTURO — NÃO EXECUTAR AGORA

---

# 0. MISSÃO PRINCIPAL

A D22 confirmou que The Ashen Sentinel pode ser derrotado por ataques reais do jogador.

Entretanto, o teste automatizado utiliza cura artificial do protagonista para garantir a conclusão da luta.

Isso deixa uma questão essencial:

**Um jogador humano consegue chegar à arena, compreender os ataques e derrotar o chefe utilizando os recursos normais do jogo?**

A D23 deve preparar e verificar essa experiência.

Objetivos:

1. Validar o acesso à Ashen Court pela escada do Reliquary.
2. Executar o jogo em janela gráfica real.
3. Testar o chefe sem cura artificial.
4. Avaliar a legibilidade dos ataques.
5. Verificar guarda, esquiva e recuperação.
6. Avaliar o balanceamento com os recursos normais.
7. Verificar morte, respawn e nova tentativa.
8. Identificar problemas de câmera e enquadramento.
9. Preparar um roteiro de playtest humano para o proprietário.
10. Corrigir somente problemas comprovados e dentro do escopo.

**Não criar um segundo chefe.**

---

# 1. SEGURANÇA E ESTADO INICIAL

Antes de qualquer alteração:

- Confirmar branch e HEAD.
- Executar `git status --short`.
- Inspecionar alterações existentes.
- Preservar D19–D22.
- Preservar arquivos do proprietário.
- Registrar o estado inicial.

Último estado informado:

- Branch `art/v1.2-protagonist`.
- HEAD `a9b8f96`.
- Working tree com alterações locais.

Não assumir que esse estado permanece igual.

Proibido:

- Commit.
- Push.
- Merge.
- Rebase.
- Reset destrutivo.
- Limpeza indiscriminada.
- Sobrescrita de saves reais.

---

# 2. AUDITORIA

Ler os documentos reais em:

- `docs/development/d20_1/`
- `docs/development/d21/`
- `docs/development/d22/`

Inspecionar:

- Acesso ao Reliquary.
- Escada one-way.
- Porta superior.
- Ashen Court.
- Câmera.
- Spawn do chefe.
- IA e ataques.
- Indicadores de preparação.
- Flash branco.
- Barra de HP.
- Invulnerabilidade.
- Esquiva.
- Guarda.
- Consumíveis.
- Progressão.
- Save V3.
- Scripts e probes existentes.

Registrar as limitações atuais antes de implementar mudanças.

---

# 3. FASE A — ACESSO À ARENA

A D22 não filmou a subida da escada do Reliquary.

Verificar o percurso completo:

1. Entrar no Reliquary.
2. Localizar a escada.
3. Subir os cinco degraus.
4. Alcançar a porta superior.
5. Entrar na Ashen Court.
6. Confirmar posição segura.
7. Confirmar câmera.
8. Confirmar início do encontro.

A rota deve ser executável com os controles normais.

Não utilizar teleporte direto como única evidência de acesso.

Se a automação não conseguir subir, diferenciar falha do driver de falha da geometria.

Registrar capturas da subida e da chegada.

---

# 4. FASE B — COMBATE SEM CURA ARTIFICIAL

Criar um cenário de teste em que:

- O jogador utilize HP inicial normal.
- MP e Hearts respeitem os valores normais.
- Não exista cura injetada por script.
- O chefe mantenha os atributos definidos.
- O dano passe pelo pipeline normal.
- Os ataques sejam acionados por entradas válidas.

Não escrever diretamente no HP do chefe.

Não restaurar HP do jogador durante a luta, exceto por mecânicas legítimas, como consumíveis ou magias de cura.

Se o driver automatizado morrer, registrar a derrota.

**Uma derrota automatizada não significa necessariamente que o chefe esteja desbalanceado.**

O objetivo é medir o comportamento, não forçar vitória.

Registrar:

- Duração.
- Dano causado.
- Dano recebido.
- Ataques executados.
- Esquivas.
- Guardas.
- Consumíveis utilizados.
- Fase alcançada.
- Resultado final.

Somente apresentar métricas realmente coletadas.

---

# 5. FASE C — LEGIBILIDADE DOS ATAQUES

Inspecionar:

## FrontalSlash

- Preparação laranja.
- Direção do ataque.
- Duração do aviso.
- Hitbox.
- Recuperação.

## Lunge

- Preparação amarela.
- Direção do avanço.
- Distância.
- Recuperação.

## AshWave

- Preparação.
- Projétil ciano.
- Trajetória.
- Colisão.
- Tempo de reação.

Verificar especificamente se o flash branco de dano ou transição encobre os indicadores.

Prioridade visual recomendada:

1. Estado crítico de combate.
2. Preparação de ataque.
3. Feedback de dano.
4. Efeitos decorativos.

Se houver conflito visual comprovado, aplicar uma correção localizada.

Não criar sprites definitivos.

---

# 6. FASE D — ESQUIVA E GUARDA

A D22 observou que:

- Guarda frontal reduz HP, mas mantém o estado Guarding.
- Esquivas bem posicionadas evitam golpes.
- Esquivas tardias podem receber dano.
- Os i-frames duram aproximadamente 0,28 s.

Verificar se esse comportamento é coerente com as regras atuais.

Não alterar globalmente a esquiva apenas para facilitar o chefe.

Testar:

- Guarda frontal.
- Guarda na direção errada.
- Esquiva antecipada.
- Esquiva durante o aviso.
- Esquiva tardia.
- Ataque durante recuperação.
- Dano após invulnerabilidade.

Documentar as janelas reais e os resultados.

---

# 7. FASE E — BALANCEAMENTO

Auditar os valores existentes:

- HP do jogador.
- HP do chefe.
- Dano básico.
- Defesa.
- Magias.
- Técnicas.
- Consumíveis.
- Cooldowns.
- Dano dos três ataques.
- Tempo de preparação.
- Tempo de recuperação.

Não alterar números arbitrariamente.

Avaliar:

- Quantos golpes são necessários para vencer.
- Quantos golpes o jogador suporta.
- Quanto tempo dura a luta.
- Se existe oportunidade de contra-ataque.
- Se a fase 2 é mais difícil, mas compreensível.
- Se o jogador consegue recuperar o controle após receber dano.

Preparar recomendações fundamentadas.

**Não fazer rebalanceamento amplo sem dados suficientes.**

---

# 8. FASE F — CÂMERA

Verificar:

- Entrada na arena.
- Enquadramento inicial.
- Visibilidade do chefe.
- Visibilidade dos projéteis.
- Barra de HP.
- Limites da arena.
- Aproximação das paredes.
- Vitória.

A D22 informou que o chefe ficou fora do quadro em uma captura inicial.

Determinar se isso é apenas consequência da posição do jogador ou um problema real de apresentação.

Se necessário, ajustar o enquadramento somente durante o encontro, reutilizando a câmera existente.

Não substituir o sistema global de câmera.

---

# 9. FASE G — DERROTA E RETENTATIVA

Testar:

1. Entrar na arena.
2. Receber dano até morrer.
3. Confirmar fim do encontro.
4. Executar respawn.
5. Retornar ao Reliquary.
6. Subir novamente.
7. Reentrar na arena.
8. Confirmar HP completo do chefe.
9. Confirmar ausência de recompensa duplicada.
10. Confirmar funcionamento das portas.

Não utilizar saves reais.

---

# 10. FASE H — PLAYTEST HUMANO

Preparar um roteiro curto para o proprietário executar.

O roteiro deve informar:

- Como iniciar o jogo.
- Como chegar ao Reliquary.
- Como subir a escada.
- Como entrar na Ashen Court.
- Como atacar.
- Como defender.
- Como esquivar.
- Como utilizar consumíveis.
- Como reiniciar após morrer.
- Como verificar a recompensa.

Criar um formulário simples de observações, preferencialmente em Markdown, com perguntas sobre:

- Clareza dos ataques.
- Dificuldade.
- Tempo de reação.
- Controles.
- Câmera.
- Diversão.
- Frustração.
- Bugs.

Não marcar esse playtest como realizado sem participação humana.

**Se o proprietário ainda não testou, registrar status PENDENTE DE PLAYTEST HUMANO.**

---

# 11. FASE I — EVIDÊNCIAS

Criar:

`docs/development/d23/evidence/`

Capturas desejadas:

1. Reliquary — início da escada.
2. Subida.
3. Porta superior.
4. Entrada da arena.
5. FrontalSlash.
6. Lunge.
7. AshWave.
8. Flash de dano.
9. Fase 2.
10. Derrota ou vitória sem cura artificial.

Capturas devem ser reais.

Se possível, produzir vídeo curto da luta.

Não gerar imagens artificiais como evidência.

---

# 12. FASE J — TESTES AUTOMATIZADOS

Reutilizar a infraestrutura existente.

Verificar:

- Acesso à arena.
- Início do encontro.
- Ataques.
- Defesa.
- Esquiva.
- Morte.
- Respawn.
- Persistência.
- Recompensa única.
- Câmera, quando testável.
- Ausência de regressões.

Manter testes que utilizam injeção de HP apenas para verificar lógica isolada, identificando-os corretamente.

O novo teste de combate sem cura deve permanecer separado.

---

# 13. FASE K — QUALIDADE

Executar sequencialmente:

```bash
cargo fmt --check
CARGO_BUILD_JOBS=2 cargo check
CARGO_BUILD_JOBS=2 cargo clippy --all-targets -- -D warnings
CARGO_BUILD_JOBS=2 cargo test
```

Executar os probes gráficos relevantes.

Registrar resultados reais.

Não iniciar builds pesados simultaneamente.

---

# 14. FASE L — DOCUMENTAÇÃO

Criar:

`docs/development/d23/`

Arquivos sugeridos:

- `D23_AUDITORIA.md`
- `D23_ACESSO_ARENA.md`
- `D23_BALANCEAMENTO.md`
- `D23_LEGIBILIDADE.md`
- `D23_PLAYTEST_HUMANO.md`
- `D23_TESTES.md`
- `D23_RELATORIO_FINAL.md`

Atualizar roadmap e changelog somente quando necessário.

---

# 15. ARTE DEFINITIVA — FUTURO

**NÃO EXECUTAR NA D23.**

Preservar:

`Md-usario/assets-user/prota/base/white_mesh.glb`

Pipeline futuro:

1. Referências visuais aprovadas.
2. Hunyuan3D.
3. Blender.
4. Mixamo, quando compatível.
5. Ajustes de animação.
6. Renderização de sprites 2D.
7. Integração Bevy.

Não modificar o GLB.

Não iniciar Hunyuan.

Não abrir Blender.

Não baixar animações do Mixamo.

Não substituir placeholders.

---

# 16. CRITÉRIOS DE ACEITAÇÃO

- [ ] Estado Git auditado.
- [ ] Rota Reliquary → Ashen Court verificada.
- [ ] Jogo executado graficamente.
- [ ] Luta sem cura artificial executada.
- [ ] Resultado da luta registrado, inclusive se houver derrota.
- [ ] Três ataques observados.
- [ ] Flash branco avaliado.
- [ ] Guarda testada.
- [ ] Esquiva testada.
- [ ] Câmera avaliada.
- [ ] Morte e nova tentativa testadas.
- [ ] Balanceamento documentado.
- [ ] Roteiro humano entregue.
- [ ] Playtest humano marcado como pendente até ocorrer.
- [ ] Testes automatizados aprovados.
- [ ] Evidências reais produzidas.
- [ ] Sem commit/push.
- [ ] Saves e arte preservados.

---

# 17. RELATÓRIO FINAL OBRIGATÓRIO

Entregar:

## A. Estado inicial
Branch, HEAD e Git.

## B. Auditoria
Sistemas e documentos examinados.

## C. Acesso
Resultado da subida do Reliquary.

## D. Combate
Resultado sem cura artificial.

## E. Ataques
Legibilidade e evidências.

## F. Defesa
Guarda, esquiva e invulnerabilidade.

## G. Balanceamento
Métricas e recomendações.

## H. Câmera
Problemas e correções.

## I. Derrota
Respawn e nova tentativa.

## J. Testes
Comandos e resultados.

## K. Evidências
Capturas e vídeos.

## L. Playtest humano
Roteiro e status real.

## M. Segurança
Git, saves, GLB e arte.

## N. Próxima etapa
Recomendação para D24.

---

# 18. DIRETRIZ FINAL

A D23 deve responder se o primeiro chefe está pronto para ser avaliado por uma pessoa.

Não basta que o código funcione.

O combate precisa ser compreensível, acessível e tecnicamente estável.

**Não declarar vitória humana com base em automação.**

**Não alterar a dificuldade para garantir que o script vença.**

**Não iniciar um segundo chefe.**

**Não iniciar produção visual definitiva.**

**Não fazer commit nem push.**

**D23 — OBJETIVO FINAL: validar a experiência real do primeiro chefe, preparar o playtest humano e identificar ajustes de balanceamento antes de ampliar o conteúdo.**