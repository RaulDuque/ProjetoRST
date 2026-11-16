# DARK CASTLE RPG — MEGA PROMPT V2.2
## Identidade Visual, Armas Equipadas, NPCs, Mapa e Polimento da Vila das Cinzas

### MISSÃO

Você é o responsável pela implementação técnica da V2.2 do Dark Castle RPG, desenvolvido em Rust com Bevy 0.20.1, utilizando o pipeline artístico existente com Blender.

A V2.1.1 foi concluída, validada e publicada no GitHub.

Esta será a última etapa de desenvolvimento de hoje. Portanto, priorize qualidade, estabilidade, evidências e uma entrega que possa ser retomada facilmente na próxima sessão.

**Objetivo principal:** fazer os equipamentos e personagens do jogo serem representados visualmente de forma coerente com os sistemas que já existem.

Não reinvente os sistemas de combate, inventário, NPCs, lojas, mapa ou Save V3. Reutilize-os.

---

# 1. ESTADO INICIAL DO PROJETO

O checkpoint Git foi publicado.

Estado esperado:

- Branch: `art/v1.2-protagonist`
- HEAD: `98b664064eadb0d76a38cab0d7130cb3f0f19bbb`
- Upstream: `origin/art/v1.2-protagonist`
- Zero commits à frente ou atrás do remoto.

A V2.1.1 possui:

- Save V3 com migração V1/V2.
- Economia persistente.
- Vila das Cinzas.
- NPCs e diálogos.
- Compra e venda.
- Equipamentos e armas de duas mãos.
- Forma espectral.
- Mapa e exploração.
- 24 tutoriais validados.
- 362 testes Rust aprovados.
- 17 testes Python aprovados.
- 125 testes do hook aprovados.

Confirme o estado real antes de começar.

Não suponha que os arquivos permanecem exatamente iguais ao relatório.

---

# 2. REGRAS ABSOLUTAS DE SEGURANÇA

## Git

Não execute:

- `git add`
- `git commit`
- `git push`
- `git reset --hard`
- `git clean`
- `git rebase`
- `git merge`
- `git checkout` para trocar de branch

Não crie worktrees.

Não reescreva histórico.

Não altere o upstream.

A V2.2 deverá permanecer local até nova autorização.

## Arquivos protegidos

Não altere:

- `Md-usario/`
- `saves/`
- `.cargo/config.toml`
- `.claude/settings.json`
- `.claude/hooks/guard.py`
- `.claude/hooks/test_guard.py`

Não sobrescreva backups ou assets publicados sem autorização específica.

O PNG de inspiração que já está no staging deve permanecer intacto.

As 10 remoções pendentes de `Md-usario/` não devem ser incluídas em nenhuma operação Git.

## Tutoriais

Os tutoriais são mantidos oficialmente em:

`docs/tutorias/`

Não escreva em:

`Md-usario/tutorias/`

Ao final, informe quais tutoriais foram criados ou modificados para que eu possa sincronizar as cópias manualmente.

## Compilação

Utilize:

```bash
export CARGO_BUILD_JOBS=2
```

Execute somente um processo Cargo por vez.

Não execute `cargo clean`.

Não apague diretórios de compilação existentes.

---

# 3. AUDITORIA TÉCNICA INICIAL

Antes de implementar, examine os sistemas existentes:

1. Pipeline Blender.
2. Rig do protagonista.
3. Exportação de sprites.
4. Animações.
5. Renderização do personagem.
6. Inventário e equipamentos.
7. Categorias de armas.
8. Aparência visual dos equipamentos.
9. Sistema de NPCs.
10. Sistema de diálogos.
11. Sistema de lojas.
12. Sistema de mapa.
13. Sistema de regiões e salas.
14. Save V3.
15. Testes visuais e rotas reais.

Leia a documentação relevante em `docs/`.

Identifique precisamente:

- Por que a espada inicial aparece permanentemente no personagem.
- Quais sprites contêm a espada incorporada.
- Como o jogo identifica a arma equipada.
- Como as animações atuais funcionam.
- Como os equipamentos visuais são renderizados.
- Quais assets podem ser modificados com segurança.
- Quais assets são protegidos ou já publicados.
- Como evitar regressões no pipeline artístico.

Produza uma estratégia antes de modificar o código.

---

# 4. PRIORIDADE MÁXIMA — CORRIGIR A ESPADA FIXA

## Problema atual

O protagonista possui uma espada desenhada no sprite base.

Mesmo equipando machado, lança, adaga ou arma pesada, a espada original continua aparecendo.

Isso quebra a coerência visual do equipamento.

## Resultado esperado

O personagem deve mostrar somente a arma realmente equipada.

Exemplos:

- Espada equipada: aparece espada.
- Adaga equipada: aparece adaga.
- Machado equipado: aparece machado.
- Lança equipada: aparece lança.
- Arma pesada equipada: aparece arma pesada.
- Sem arma equipada: nenhuma arma aparece na mão.

## Requisitos

1. Separar visualmente o corpo da arma.
2. Preservar as animações existentes.
3. Preservar a identidade visual do protagonista.
4. Evitar duplicação de armas.
5. Evitar armas flutuando.
6. Evitar armas invertidas incorretamente.
7. Manter posicionamento coerente durante movimentos.
8. Respeitar a direção do personagem.
9. Respeitar a arma equipada.
10. Não alterar as estatísticas ou regras de combate.

**Não tente esconder a espada com um retângulo ou uma sobreposição improvisada.**

Corrija a origem do problema no pipeline visual.

## Atenção aos assets protegidos

Se for necessário substituir sprites publicados ou modificar arquivos protegidos, não faça isso diretamente.

Primeiro produza uma versão candidata em caminhos novos e não protegidos.

Valide a candidata isoladamente.

Apresente os arquivos que precisariam ser substituídos e solicite minha autorização.

Continue as demais tarefas independentes enquanto essa autorização estiver pendente.

Não declare a correção da espada concluída se a versão aprovada do jogo continuar exibindo a espada fixa.

---

# 5. ÂNCORAS DE EQUIPAMENTOS

Implemente ou finalize um sistema de âncoras para equipamentos visuais.

## Âncoras mínimas

- Mão direita.
- Mão esquerda.
- Cabeça.
- Corpo.
- Costas.

A posição deve acompanhar a animação.

## Movimentos obrigatórios

Valide:

1. Idle.
2. Caminhada.
3. Corrida.
4. Pulo.
5. Queda.
6. Ataque.
7. Defesa.
8. Esquiva.
9. Dano recebido.
10. Morte.
11. Mudança de direção.
12. Transição entre salas.

Se o pipeline já oferecer âncoras equivalentes, reutilize-as.

Não crie um segundo sistema incompatível.

## Formato dos dados

Prefira dados declarativos e versionáveis.

Se forem necessários novos campos RON, documente:

- Nome.
- Tipo.
- Unidade.
- Valor padrão.
- Compatibilidade com dados antigos.
- Comportamento quando ausente.

Não altere o formato dos saves apenas para armazenar posições visuais deriváveis.

---

# 6. REPRESENTAÇÃO DAS CINCO CATEGORIAS DE ARMAS

As categorias existentes são:

1. Sword.
2. Dagger.
3. Axe.
4. Spear.
5. HeavyWeapon.

Cada categoria deve ter uma silhueta visual distinguível.

## Espadas

- Lâmina proporcional ao personagem.
- Empunhadura consistente.
- Movimento coerente com os ataques.

## Adagas

- Comprimento menor.
- Silhueta própria.
- Sem reaproveitar uma espada apenas reduzindo sua escala indiscriminadamente.

## Machados

- Cabeça de machado visível.
- Centro de massa visual coerente.
- Empunhadura adequada.

## Lanças

- Haste alongada.
- Ponta claramente identificável.
- Posicionamento adequado à animação.

## Armas pesadas

- Silhueta robusta.
- Proporção coerente.
- Representação visual de duas mãos quando necessário.

Não é necessário produzir arte final para todos os itens individuais nesta versão.

**É obrigatório, porém, que as cinco categorias sejam visualmente distinguíveis em uma demonstração funcional.**

---

# 7. REGRA VISUAL DE DUAS MÃOS

A V2.1 já possui regras funcionais para equipamentos de duas mãos.

Agora a representação visual deve respeitá-las.

## Comportamento esperado

- Arma de uma mão pode coexistir com equipamento compatível na outra mão.
- Arma de duas mãos ocupa visualmente as duas mãos quando a animação permitir.
- Escudo incompatível não deve permanecer visível junto de arma de duas mãos.
- O jogo não deve ocultar inconsistências de equipamento.
- Equipar e desequipar deve atualizar a aparência.

## Testes

Crie cenários para:

1. Espada e escudo.
2. Adaga e escudo.
3. Machado de uma mão e escudo, quando permitido pelos dados.
4. Lança de duas mãos.
5. Arma pesada de duas mãos.
6. Tentativa de equipar escudo incompatível.
7. Troca de arma.
8. Desequipar arma.
9. F5/F9 restaurando a aparência correspondente ao equipamento salvo.

Não modifique as regras existentes de compatibilidade sem justificar e testar.

---

# 8. APARÊNCIA DAS ARMADURAS

A V2.0 já implementou equipamentos e modificadores visuais.

Nesta versão, revise a coerência da aparência.

## Verificações

- Armadura corporal.
- Capa.
- Capacete.
- Luvas ou proteção dos braços.
- Proteção das pernas.
- Escudo.
- Acessórios com efeitos visuais existentes.

Não invente slots adicionais.

Preserve os oito slots existentes.

Se um equipamento não possuir arte própria, utilize uma representação provisória consistente e documentada, sem afirmar que possui arte final.

## Ordem de renderização

Verifique:

- Capa atrás do personagem.
- Corpo na camada correta.
- Armas na camada correta.
- Escudo sem cobrir indevidamente o corpo.
- Equipamentos sem atravessar o personagem de forma evidente.
- Correção da ordem ao mudar de direção.

---

# 9. NPCs — IDENTIDADE VISUAL

A Vila das Cinzas já possui cinco NPCs funcionais.

Quero que eles deixem de parecer apenas marcadores genéricos.

## Objetivo

Criar uma identidade visual provisória, mas consistente, para os NPCs existentes.

Cada NPC deve ser reconhecível por:

- Silhueta.
- Cores.
- Vestimenta.
- Papel na cidade.
- Elementos visuais apropriados.

## Requisitos

1. Reutilizar o sistema atual de NPCs.
2. Não criar personagens duplicados.
3. Não modificar diálogos sem necessidade.
4. Não alterar condições de progresso.
5. Não quebrar o sistema de interação com D.
6. Não quebrar a prioridade dos baús.
7. Preservar a loja.
8. Preservar compra e venda.
9. Preservar o Save V3.

## Arte

Se possível, criar sprites provisórios para os cinco NPCs.

Se o pipeline exigir sobrescrever assets protegidos, gere candidatos separados e aguarde autorização.

Não use imagens de terceiros sem licença apropriada.

Não copie diretamente personagens protegidos por direitos autorais.

---

# 10. MAPA — ÍCONES DE NPCS E LOJAS

O mapa já existe e deve continuar funcionando.

Adicione marcadores visuais úteis para:

- Lojas.
- Mercadores.
- NPCs relevantes.
- Ferreiro, somente se houver um NPC ou serviço correspondente realmente implementado.

## Regras

1. Não revelar áreas inexploradas indevidamente.
2. Não revelar NPCs ocultos sem justificativa.
3. Respeitar o sistema atual de descoberta.
4. Evitar marcadores duplicados.
5. Evitar ícones fora das salas.
6. Atualizar os marcadores quando apropriado.
7. Preservar os marcadores existentes.
8. Preservar o funcionamento da tecla M.

Se um NPC estiver numa sala ainda não descoberta, aplique a política de visibilidade já existente no mapa.

## Legenda

Adicione uma legenda simples, se necessário.

Evite poluição visual.

---

# 11. POLIMENTO DA VILA DAS CINZAS

A cidade possui seis salas conectadas ao castelo.

Não expanda a quantidade de salas nesta versão, salvo necessidade técnica comprovada.

Concentre-se na apresentação.

## Melhorias desejadas

- Identidade visual mais clara.
- Diferenciação entre áreas comerciais e residenciais.
- Marcadores discretos de interação.
- Melhor leitura dos NPCs.
- Melhor leitura das lojas.
- Feedback de compra.
- Feedback de venda.
- Feedback de estoque esgotado.
- Melhor contraste dos textos.
- Menos sobreposição entre mensagens.

## Bug visual conhecido

Na V2.1 foi relatado que mensagens de interação podem se sobrepor às mensagens do HUD.

Investigue e corrija sem ocultar informações importantes.

Valide diferentes situações:

- NPC próximo.
- Baú próximo.
- Loja aberta.
- Inventário aberto.
- Mapa aberto.
- Mensagens de compra e venda.
- Transição de sala.

---

# 12. DEMONSTRAÇÃO VISUAL DE EQUIPAMENTOS

Crie ou atualize um exemplo específico para demonstrar as armas e equipamentos.

Sugestão:

`examples/equipment_visual_showcase.rs`

Reutilize `equipment_showcase` se ele já atender ao objetivo.

## Demonstração mínima

Mostrar o protagonista:

1. Sem arma.
2. Com espada.
3. Com adaga.
4. Com machado.
5. Com lança.
6. Com arma pesada.
7. Com escudo.
8. Com armadura.
9. Com capa.
10. Em movimento.
11. Atacando.
12. Mudando de direção.

A demonstração deve facilitar capturas e comparação visual.

Não use somente logs para afirmar que uma arma aparece corretamente.

---

# 13. TESTES AUTOMATIZADOS

Crie testes para:

1. Seleção da aparência pela arma equipada.
2. Remoção visual ao desequipar.
3. Atualização visual ao trocar arma.
4. Compatibilidade de duas mãos.
5. Escudo incompatível.
6. Âncoras válidas.
7. Dados visuais incompletos.
8. Fallback seguro.
9. Mudança de direção.
10. Troca de animação.
11. Carregamento de equipamentos pelo Save V3.
12. Preservação das estatísticas.
13. Preservação dos ataques.
14. Preservação das skills.
15. Preservação das magias.
16. Preservação da forma espectral.
17. NPCs existentes.
18. Interação D.
19. Prioridade dos baús.
20. Marcadores de lojas.
21. Marcadores de NPCs.
22. Visibilidade conforme exploração.
23. HUD sem sobreposição crítica.
24. Compra e venda sem regressão.
25. Persistência do estoque.

Não use testes que dependam de modificar saves reais.

---

# 14. TESTES COM JANELA

Execute as rotas existentes:

- `world_report`
- `world_town_route`
- `world_equipment_route`
- `world_spectral_route`
- `world_secrets_route`
- `world_main_route`
- `world_vertical_slice`
- `autoplay`
- `visual_check`
- `equipment_showcase`

Adicione a demonstração visual nova, se criada.

## Requisitos

- Nenhum reposicionamento artificial nas rotas reais.
- Nenhum teletransporte usado para mascarar falhas.
- Nenhuma injeção artificial de itens nas rotas de progressão real.
- Nenhuma alteração de saves reais.
- Capturas visuais efetivamente inspecionadas.

As demonstrações isoladas podem usar equipamentos de teste, desde que isso seja claramente identificado e não seja confundido com uma rota real de progressão.

## Capturas obrigatórias

Registre imagens de:

1. Protagonista sem arma.
2. Protagonista com espada.
3. Protagonista com adaga.
4. Protagonista com machado.
5. Protagonista com lança.
6. Protagonista com arma pesada.
7. Escudo equipado.
8. Capa e armadura.
9. NPCs da cidade.
10. Loja aberta.
11. Mapa com marcadores.
12. HUD durante interação.

Compare as capturas com o comportamento esperado.

Se algum resultado estiver visualmente incorreto, registre a falha.

---

# 15. PERFORMANCE

A adição de camadas visuais não deve degradar excessivamente o desempenho.

Meça:

- Tempo médio por frame.
- Tempo com equipamentos visíveis.
- Tempo com NPCs na tela.
- Tempo com loja aberta.
- Tempo com mapa aberto.
- Memória máxima.
- Quantidade adicional de entidades ou sprites.
- Quantidade de texturas carregadas.

Compare com a V2.1.1 quando houver medições equivalentes.

Não apresente comparações entre cenários diferentes como se fossem medições equivalentes.

Evite criar e destruir entidades desnecessariamente a cada frame.

Prefira atualizar componentes existentes quando apropriado.

---

# 16. DOCUMENTAÇÃO TÉCNICA

Crie:

`docs/VISUAL_EQUIPMENT_V2_2.md`

Inclua:

1. Objetivo.
2. Arquitetura.
3. Problema da espada fixa.
4. Solução adotada.
5. Sistema de âncoras.
6. Armas visíveis.
7. Escudos.
8. Armaduras.
9. NPCs.
10. Marcadores do mapa.
11. Pipeline artístico.
12. Compatibilidade.
13. Testes.
14. Performance.
15. Limitações.
16. Pendências futuras.

Atualize os documentos existentes que realmente forem afetados.

Não crie documentação redundante.

Registre decisões arquiteturais importantes em `docs/DECISIONS.md`.

---

# 17. TUTORIAIS PARA O PROPRIETÁRIO

Os tutoriais oficiais ficam em:

`docs/tutorias/`

Não escreva em `Md-usario/`.

Atualize os tutoriais existentes quando necessário.

Crie novos tutoriais, se os sistemas forem realmente implementados, para:

- Criar uma arma com aparência própria.
- Configurar a posição de uma arma.
- Criar sprites de NPCs.
- Associar sprites a NPCs.
- Adicionar marcadores no mapa.
- Testar equipamentos visuais.

Use a numeração seguinte à existente, sem sobrescrever arquivos.

Cada tutorial deve seguir o padrão de nove seções já adotado no projeto.

Os exemplos devem usar caminhos reais e formatos aceitos pelo jogo.

Não documente funcionalidades ainda inexistentes como se estivessem implementadas.

No relatório, liste exatamente quais arquivos de `docs/tutorias/` foram criados ou modificados.

Eu sincronizarei manualmente as cópias em `Md-usario/tutorias/`.

---

# 18. REGRESSÃO GERAL

Execute sequencialmente:

```bash
export CARGO_BUILD_JOBS=2

cargo fmt --all -- --check
cargo check -j 2
cargo clippy --all-targets -j 2 -- -D warnings
cargo test -j 2
```

Execute também:

- Testes Python do pipeline artístico.
- Testes do hook de proteção.
- Testes dos tutoriais.
- Testes Save V3.
- Testes de economia.
- Testes de equipamentos.
- Testes de NPCs.
- Testes de exploração.

Se algum teste falhar, investigue a causa.

Não altere testes apenas para esconder uma falha.

---

# 19. AUDITORIA DE SEGURANÇA FINAL

Confirme:

1. Branch original preservada.
2. HEAD original preservado.
3. Staging original preservado.
4. PNG de inspiração intacto.
5. Remoções pendentes de `Md-usario/` preservadas.
6. Arquivos protegidos intactos.
7. Saves reais intactos.
8. Assets publicados intactos, salvo autorização específica posterior.
9. Configuração Cargo intacta.
10. Hooks intactos.
11. Diretórios de compilação preservados.
12. Nenhum commit criado.
13. Nenhum push realizado.

Se algum arquivo protegido precisar mudar para concluir a parte visual, apresente a proposta e aguarde autorização.

---

# 20. RELATÓRIO FINAL DA V2.2

Entregue um relatório com as seguintes seções:

A. Auditoria inicial.

B. Problema da espada fixa.

C. Solução visual implementada.

D. Âncoras de equipamentos.

E. Cinco categorias de armas.

F. Regra visual de duas mãos.

G. Armaduras e escudos.

H. NPCs e arte.

I. Marcadores do mapa.

J. Polimento da Vila das Cinzas.

K. Correções de HUD.

L. Testes Rust.

M. Testes Python.

N. Testes visuais.

O. Capturas inspecionadas.

P. Performance.

Q. Save V3 e compatibilidade.

R. Arquivos criados.

S. Arquivos modificados.

T. Tutoriais novos ou atualizados.

U. Arquivos protegidos e hashes.

V. Estado final do Git.

W. Pendências.

X. Recomendações para a próxima sessão.

Não declare uma funcionalidade concluída sem testes e evidências.

---

# 21. CRITÉRIOS DE ACEITAÇÃO

A V2.2 só estará totalmente concluída quando:

1. A arma visível corresponder à arma equipada.
2. O personagem sem arma não exibir uma arma fixa.
3. As cinco categorias forem visualmente distinguíveis.
4. As armas acompanharem as animações.
5. As regras de duas mãos forem respeitadas visualmente.
6. Os NPCs existentes tiverem representação visual coerente.
7. O mapa identificar lojas e NPCs conforme a exploração.
8. O HUD não apresentar sobreposições críticas.
9. Save V3 continuar funcionando.
10. Todos os testes obrigatórios passarem.
11. As rotas reais continuarem funcionando.
12. Os arquivos protegidos permanecerem intactos.
13. A documentação estiver atualizada.
14. Os tutoriais estiverem atualizados em `docs/tutorias/`.

Se a correção completa das armas depender de autorização para substituir assets protegidos, entregue os candidatos e classifique a V2.2 como parcialmente concluída.

Não improvise uma solução visual inferior apenas para declarar sucesso.

---

# 22. ORDEM DE EXECUÇÃO

Trabalhe nesta ordem:

**Fase A — Auditoria**

Inspecione o código, os assets e o pipeline. Identifique dependências e riscos.

**Fase B — Equipamentos visuais**

Implemente a separação entre corpo e armas em uma versão candidata segura.

**Fase C — Âncoras e animações**

Faça os equipamentos acompanharem o personagem.

**Fase D — Cinco categorias**

Valide espada, adaga, machado, lança e arma pesada.

**Fase E — Armaduras e escudos**

Valide equipamentos e regra de duas mãos.

**Fase F — NPCs e mapa**

Melhore a apresentação dos NPCs e os marcadores.

**Fase G — Polimento**

Corrija HUD e problemas visuais da cidade.

**Fase H — Testes**

Execute testes automatizados e rotas com janela.

**Fase I — Documentação**

Atualize documentação e tutoriais.

**Fase J — Auditoria final**

Confirme integridade, hashes, Git e pendências.

---

# 23. INSTRUÇÃO FINAL

Esta é a última etapa de desenvolvimento de hoje.

Priorize uma entrega estável em vez de implementar recursos demais.

Não comece a V2.3.

Não implemente novos sistemas de combate, novos chefes ou novas regiões nesta etapa.

Não modifique o Save V3 sem necessidade comprovada.

Não faça commits ou push.

Não altere arquivos pessoais.

Não sobrescreva assets protegidos.

Se precisar de autorização para uma mudança protegida, explique exatamente o que precisa ser alterado, quais arquivos serão afetados e como será possível validar a mudança com segurança.

Ao finalizar, entregue o relatório completo da V2.2.

**O objetivo é terminar o dia com um jogo mais consistente visualmente, sem comprometer a base técnica construída até a V2.1.1.**