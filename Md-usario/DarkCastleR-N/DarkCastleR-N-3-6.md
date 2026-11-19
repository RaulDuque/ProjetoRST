# DARK CASTLE RPG — MEGA PROMPT N3.6
# CONSOLIDAÇÃO ARTÍSTICA, REFERÊNCIAS DE MOVIMENTO E PRIMEIRA PROVA DE ANIMAÇÃO

**Projeto:** Dark Castle RPG  
**Etapa:** N3.6 — Art Direction & Animation Feasibility  
**Branch:** `art/v1.2-protagonist`  
**HEAD remoto de referência:** `1d628f28caafa19d0ab97d58cf246f77fff8eaeb`  
**Tecnologias:** Rust 2024, Bevy 0.20.1 e ferramentas de processamento gráfico disponíveis  
**Direção artística:** aprovada como referência visual, não como recurso definitivo  
**N4 e N5:** BLOQUEADAS

---

# 0. MISSÃO

Você atuará como diretor de arte técnico, especialista em animação 2D, pixel art, processamento de imagens e integração com Bevy.

O proprietário publicou novas referências do protagonista original, incluindo sequências de movimento.

Essas imagens representam o nível de qualidade visual desejado para o Dark Castle RPG.

**Não crie outro protagonista.**

Não retorne ao estilo geométrico da N3 ou N3.3.

Não substitua o personagem por um manequim do Blender.

Não redesenhe a identidade visual.

A missão da N3.6 é:

1. Auditar as novas referências.
2. Consolidar a identidade visual.
3. Verificar a consistência dos movimentos.
4. Estudar como transformar as referências em sprites utilizáveis.
5. Criar uma primeira prova de animação isolada, se tecnicamente viável.
6. Preservar a qualidade artística alcançada na N3.5.
7. Documentar limitações reais.
8. Publicar resultados verificáveis no GitHub.

**A N3.6 não é autorização para iniciar a N4.**

---

# 1. ESTADO ATUAL DO PROJETO

A N3.5 demonstrou que:

- A referência reduzida possui 106 pixels de altura.
- O canvas utilizado é 80×128.
- A qualidade visual depende principalmente do método de redução.
- A reconstrução em blocos prejudica o modelado original.
- A câmera 360, com densidade 2, permite apresentar 1 texel por pixel de tela.
- A câmera atual de 405 reduz os detalhes visíveis.
- O candidato N3.5 preserva melhor o cabelo, o rosto e as roupas.
- O personagem ainda não foi promovido à produção.
- Não foram criadas animações na N3.5.

O proprietário avaliou as evidências e gostou especialmente de:

- `docs/development/n3_5/evidence/dark_background.png`
- `docs/development/n3_5/evidence/bevy_n35_360.png`
- `docs/development/n3_5/evidence/final_candidate.png`
- `docs/development/n3_5/evidence/palette_test.png`

Essas evidências devem ser preservadas.

Não sobrescrevê-las.

---

# 2. AUDITORIA INICIAL OBRIGATÓRIA

Antes de qualquer alteração:

- Confirmar branch.
- Confirmar HEAD local e remoto.
- Inspecionar `git status`.
- Verificar alterações preexistentes.
- Ler `CLAUDE.md`.
- Ler a documentação arquitetural relevante.
- Ler os relatórios N3 a N3.5.
- Inspecionar os manifestos e scripts da prova visual.
- Identificar os arquivos de produção que devem permanecer intocados.

Não presumir que o HEAD remoto continua igual ao informado neste prompt.

Se houver novos commits, analisar as diferenças antes de continuar.

Não apagar arquivos preexistentes.

---

# 3. NOVAS REFERÊNCIAS PUBLICADAS

O proprietário informou que os arquivos foram publicados em:

`Md-usario/assets-user/prota/base/`

Arquivos identificados no GitHub:

`image.png`

`Ficha_Modular_do_Protagonista_Gotico.png`

`Gemini_Generated_Image_15q6g515q6g515q6.jpg`

`Gemini_Generated_Image_gm471hgm471hgm47.jpg`

`Gemini_Generated_Image_p5njkap5njkap5nj.jpg`

`Gemini_Generated_Image_xwfavvxwfavvxwfa.jpg`

`al.jpg`

`side.png`

`trimmed_fd4c76cf-0416-4387-b451-da4cc90a2061.mp4`

`white_mesh.glb`

Todos devem permanecer intactos.

**Não modificar, converter por cima, renomear ou excluir arquivos em `Md-usario/`.**

Criar derivados em diretório próprio da N3.6.

---

# 4. INSPEÇÃO VISUAL REAL

Não basta listar os arquivos.

Abrir e analisar visualmente as imagens.

Examinar o vídeo, se houver ferramentas disponíveis.

Registrar para cada arquivo:

- Nome.
- Dimensões.
- Formato.
- Quantidade aparente de poses.
- Direção do personagem.
- Tipo de movimento.
- Presença de fundo.
- Possíveis elementos de interface.
- Consistência do personagem.
- Qualidade das bordas.
- Problemas de recorte.
- Possibilidade de extrair quadros.
- Limitações para uso como sprite.

Se não conseguir abrir algum arquivo, registrar a falha.

Não inventar conteúdo.

---

# 5. IDENTIDADE VISUAL OBRIGATÓRIA

O protagonista deve preservar:

- Cabelo longo branco-prateado.
- Rosto masculino sério.
- Perfil elegante.
- Anatomia humana estilizada.
- Camisa cinza-escura.
- Mangas dobradas.
- Braceletes.
- Calça escura.
- Cinto.
- Botas altas.
- Proporções coerentes.

O personagem não deve mudar de aparência entre:

- Idle.
- Walk.
- Run.
- Jump.
- Attack.
- Fall.
- Landing.

A identidade original é mais importante que a quantidade de animações.

---

# 6. CLASSIFICAÇÃO DAS REFERÊNCIAS

Inspecionar e classificar cada folha.

As imagens compartilhadas pelo proprietário incluem referências visuais de:

## Corrida

Poses com:

- Braços alternados.
- Joelhos elevados.
- Pernas estendidas.
- Inclinação do torso.
- Cabelo acompanhando o deslocamento.

## Salto e acrobacias

Poses de:

- Impulsão.
- Subida.
- Movimento aéreo.
- Rotação.
- Queda.
- Aterrissagem.

## Ataque desarmado

Poses de:

- Guarda.
- Antecipação.
- Extensão do braço.
- Impacto.
- Recuperação.

## Idle e movimentação

Poses de:

- Repouso.
- Variação sutil.
- Preparação para movimento.
- Corrida.
- Salto.

Essas categorias são observações preliminares das imagens apresentadas na conversa.

Confirmar a correspondência com os nomes dos arquivos após inspecioná-los.

Não atribuir automaticamente um nome de animação a um arquivo sem verificar seu conteúdo.

---

# 7. DIFERENÇA ENTRE REFERÊNCIA E SPRITE

As novas imagens são folhas ilustradas.

Elas não devem ser tratadas automaticamente como spritesheets prontas.

Antes de extrair quadros, verificar:

- Se as poses são realmente do mesmo personagem.
- Se a anatomia permanece consistente.
- Se o tamanho da cabeça muda.
- Se o cabelo mantém a mesma identidade.
- Se o rosto muda de proporção.
- Se braços e pernas mantêm o mesmo comprimento.
- Se o tamanho das botas varia.
- Se a direção é consistente.
- Se há poses duplicadas.
- Se há poses incompatíveis com a sequência.
- Se o fundo pode ser removido sem danificar a arte.

Uma folha visualmente bonita não garante uma animação funcional.

---

# 8. AUDITORIA DA CONSISTÊNCIA ANATÔMICA

Criar uma comparação de poses usando guias.

Medir:

- Altura da cabeça.
- Comprimento do torso.
- Comprimento dos braços.
- Comprimento das pernas.
- Largura dos ombros.
- Tamanho das mãos.
- Tamanho dos pés.
- Volume do cabelo.
- Posição do quadril.

Não comparar somente caixas delimitadoras.

A largura de uma pose pode mudar legitimamente durante um movimento.

Separar mudanças naturais da animação de inconsistências anatômicas.

Registrar diferenças significativas.

---

# 9. AUDITORIA DO CABELO

O cabelo é um elemento de identidade fundamental.

Verificar:

- Volume.
- Comprimento.
- Mechas.
- Direção.
- Movimento secundário.
- Inércia.
- Continuidade entre quadros.
- Sobreposição com torso e braços.

Não deixar o cabelo mudar de comprimento entre quadros.

Não permitir que ele se transforme em outra forma sem continuidade.

Não simplificá-lo para uma massa branca chapada.

O cabelo deve continuar reconhecível na resolução final.

---

# 10. PRIMEIRA PROVA: CORRIDA

A primeira animação experimental deve ser a corrida.

Motivos:

- Existem várias poses de corrida nas referências.
- O movimento permite avaliar proporções.
- A alternância dos membros é facilmente observável.
- O cabelo oferece um teste de movimento secundário.
- A animação pode ser reproduzida isoladamente no Bevy.

**Não tentar implementar todas as animações simultaneamente.**

A prioridade é criar um clipe de corrida coerente.

---

# 11. SELEÇÃO DOS QUADROS

Inspecionar todas as poses candidatas.

Selecionar somente quadros que possam formar um ciclo coerente.

Não impor quantidade fixa de quadros antes da análise.

Criar uma tabela contendo:

- Identificador.
- Arquivo de origem.
- Posição na folha.
- Tipo de pose.
- Pé de apoio.
- Direção dos braços.
- Inclinação do corpo.
- Direção do cabelo.
- Qualidade visual.
- Problemas.
- Decisão de uso.

Descartar poses incompatíveis.

Não inventar quadros intermediários e apresentá-los como extraídos.

---

# 12. EXTRAÇÃO DOS QUADROS

Criar um processo reproduzível de extração.

Requisitos:

- Preservar o arquivo original.
- Evitar cortes no cabelo.
- Evitar cortes nas botas.
- Remover fundo com cuidado.
- Preservar contornos.
- Manter transparência correta.
- Registrar coordenadas de recorte.
- Documentar transformações.
- Preservar a resolução intermediária.

Se o fundo não puder ser removido sem destruir pixels importantes, registrar o impedimento.

Não utilizar um recorte defeituoso apenas para concluir a tarefa.

---

# 13. NÃO DESTRUIR A ARTE NA REDUÇÃO

A N3.5 mostrou que o método de redução é decisivo.

Portanto:

Não utilizar automaticamente a mesma quantização de 16 cores da N3.1.

Não substituir sombras por grandes áreas chapadas.

Não aplicar contorno preto indiscriminadamente.

Não criar um novo desenho geométrico sobre as poses.

Comparar métodos de redução preservando:

- Volume do rosto.
- Mechas do cabelo.
- Dobras da camisa.
- Volume das pernas.
- Formato das botas.
- Separação dos materiais.

Registrar qual método foi utilizado.

Não declarar pintura manual quando houve apenas processamento automático.

---

# 14. RESOLUÇÃO E CANVAS

A base N3.5 utiliza:

- Canvas 80×128.
- Figura de aproximadamente 106 px.
- Densidade 2.
- Pivô documentado.

Mas poses de corrida podem ser mais largas que o personagem parado.

Portanto:

**Não forçar todas as poses a caber em 80 pixels de largura se isso exigir deformar o personagem.**

Investigar um canvas comum maior para os quadros animados.

Manter:

- Mesma escala artística.
- Mesma altura corporal de referência.
- Mesmo sistema de coordenadas.
- Mesmo pivô lógico.
- Mesmo alinhamento do personagem.

A largura do canvas pode ser maior.

Não aumentar ou diminuir arbitrariamente cada quadro para preencher seu canvas.

Documentar as dimensões escolhidas.

---

# 15. PIVÔ E ALINHAMENTO

Definir um pivô lógico consistente para o clipe.

Registrar:

- Origem do personagem.
- Posição do quadril.
- Linha do chão.
- Deslocamento dos pés.
- Centro corporal.
- Bounding box de cada quadro.

A corrida deve poder ser reproduzida:

1. Parada no lugar.
2. Com deslocamento horizontal controlado pelo jogo.

Não confundir movimento da câmera com deslocamento do personagem.

Não criar vibração artificial por alinhamento incorreto.

---

# 16. CICLO DE CORRIDA

O ciclo deve demonstrar:

- Alternância coerente das pernas.
- Alternância coerente dos braços.
- Contato e recuperação.
- Movimento natural do torso.
- Movimento secundário do cabelo.
- Transição suave entre último e primeiro quadro.

Não utilizar todas as poses só porque estão disponíveis.

Não misturar corrida com salto.

Não misturar corrida com ataque.

Não inserir poses acrobáticas num ciclo normal de corrida.

Se a folha não permitir um ciclo fechado convincente, produzir apenas uma sequência demonstrativa e rotulá-la como não cíclica.

---

# 17. VELOCIDADE E TEMPORIZAÇÃO

Testar diferentes durações de quadro.

Não escolher FPS apenas por conveniência.

Criar comparações de reprodução, por exemplo:

- 8 FPS.
- 10 FPS.
- 12 FPS.

Esses valores são hipóteses, não requisitos definitivos.

Avaliar:

- Peso do movimento.
- Velocidade percebida.
- Continuidade.
- Movimento do cabelo.
- Legibilidade dos pés.
- Repetição do ciclo.

Escolher a temporização mais convincente.

---

# 18. PROVA ISOLADA NO BEVY

Reutilizar:

`examples/n3_visual_proof.rs`

Sempre que for tecnicamente apropriado.

Não criar uma segunda arquitetura paralela desnecessária.

Adicionar uma fonte experimental claramente identificada:

`n3.6-run-proof`

O exemplo deve permitir:

- Visualizar o personagem-base N3.5.
- Visualizar a animação experimental.
- Pausar.
- Avançar quadro a quadro.
- Comparar câmera 405.
- Comparar câmera 360.
- Exibir pivô.
- Exibir bounding box.
- Exibir número do quadro.
- Exibir FPS.
- Exibir resolução.
- Exibir estado de aprovação.

Não alterar o jogo principal.

---

# 19. CÂMERA

Manter `camera.ron` inalterado.

Usar o exemplo isolado para comparar:

## Câmera 405

Enquadramento atual do jogo.

## Câmera 360

Enquadramento experimental com correspondência inteira dos pixels, conforme a N3.5.

Verificar se a animação mantém qualidade nos dois cenários.

Não declarar que a câmera 360 está aprovada para produção.

Essa decisão continua pendente.

---

# 20. TESTE EM FUNDO ESCURO

A N3.5 apresentou perda de contraste nas pernas e botas.

A N3.6 deve testar a animação em:

- Fundo claro.
- Fundo cinza.
- Fundo quase preto.
- Fundo representativo do castelo, se houver recurso disponível.

Não adicionar um contorno branco grosseiro.

Se necessário, testar uma borda de luz discreta.

Preservar a aparência original.

Comparar com e sem a correção.

Não aplicar a correção automaticamente à produção.

---

# 21. MODULARIDADE FUTURA

A N3.6 deve considerar que futuramente existirão:

- Casaca.
- Armadura.
- Capa.
- Espada.
- Escudo.
- Elmo.
- Outros equipamentos.

Mas não implementar o sistema modular completo.

Registrar riscos relacionados a:

- Mudança de largura das poses.
- Oclusão do cabelo.
- Posição das mãos.
- Movimento do torso.
- Sincronização de equipamentos.
- Camadas visuais.
- Sockets.
- Mudanças de orientação.

Não desenhar a espada permanentemente dentro do corpo-base.

Não incorporar uma capa ao sprite-base.

Não iniciar N4.

---

# 22. ESTUDO DE SALTO

Após a prova de corrida, analisar as referências de salto.

Não é obrigatório criar o clipe nesta etapa.

Classificar as poses em:

- Preparação.
- Impulsão.
- Subida.
- Ápice.
- Queda.
- Aterrissagem.
- Recuperação.

Separar saltos normais de movimentos acrobáticos.

Não transformar automaticamente um salto com rotação em salto padrão do jogo.

Registrar quais poses são reutilizáveis.

---

# 23. ESTUDO DE ATAQUE

Analisar as referências de ataque desarmado.

Identificar:

- Guarda.
- Antecipação.
- Extensão.
- Impacto.
- Recuperação.
- Retorno ao idle.

Verificar consistência de:

- Punhos.
- Cotovelos.
- Ombros.
- Torso.
- Pernas.
- Cabelo.
- Direção.

Não integrar hitboxes novas.

Não modificar o sistema de combate.

Não substituir os comandos especiais existentes.

Essa etapa é estudo artístico, não implementação de combate.

---

# 24. ESTUDO DE IDLE

Analisar as poses de repouso.

Verificar:

- Consistência facial.
- Posição dos braços.
- Posição dos pés.
- Movimento do cabelo.
- Respiração.
- Mudanças indesejadas de proporção.

Não produzir animação de idle se isso comprometer a prova principal de corrida.

---

# 25. ARQUIVOS DE TRABALHO

Criar:

`art-src/n3-6-animation-proof/`

Estrutura sugerida:

- `references/`
- `analysis/`
- `source/`
- `frames/`
- `generated/`
- `scripts/`
- `manifests/`

Não duplicar os arquivos originais grandes sem necessidade.

Se for possível referenciá-los diretamente, fazê-lo.

Registrar origem e hashes.

Preservar fontes editáveis.

---

# 26. EVIDÊNCIAS

Criar:

`docs/development/n3_6/evidence/`

Produzir, conforme disponibilidade real:

- `references_overview.png`
- `character_identity.png`
- `run_pose_analysis.png`
- `run_anatomy_comparison.png`
- `run_hair_comparison.png`
- `run_selected_frames.png`
- `run_alignment.png`
- `run_cycle.png`
- `run_405.png`
- `run_360.png`
- `run_dark_background.png`
- `jump_pose_analysis.png`
- `attack_pose_analysis.png`
- `idle_pose_analysis.png`
- `bevy_n36.png`

Também produzir um GIF ou vídeo da corrida, se houver ferramenta adequada.

O arquivo animado deve mostrar a sequência real, não uma montagem estática apresentada como animação.

Não criar evidências fictícias.

---

# 27. GATES DA N3.6

## Gate A — Identidade

- [ ] O protagonista mantém o cabelo original.
- [ ] O rosto permanece reconhecível.
- [ ] As proporções são consistentes.
- [ ] As roupas preservam o estilo.
- [ ] A animação não troca a identidade do personagem.

## Gate B — Consistência dos quadros

- [ ] Quadros possuem procedência documentada.
- [ ] Não há deformações arbitrárias.
- [ ] O cabelo mantém continuidade.
- [ ] As pernas alternam coerentemente.
- [ ] O pivô é consistente.
- [ ] O ciclo não apresenta saltos visuais graves.

## Gate C — Qualidade de apresentação

- [ ] Resolução documentada.
- [ ] Escala documentada.
- [ ] Transparência correta.
- [ ] Fundo escuro testado.
- [ ] Câmera 405 testada.
- [ ] Câmera 360 testada.

## Gate D — Integração isolada

- [ ] Animação carrega no Bevy.
- [ ] É possível pausar.
- [ ] É possível avançar quadros.
- [ ] Não altera o jogo principal.
- [ ] Não quebra provas anteriores.

**Nenhum gate artístico pode ser aprovado automaticamente pelo agente.**

---

# 28. CRITÉRIOS DE INTERRUPÇÃO

Interromper a produção do clipe caso:

- As poses não tenham consistência anatômica.
- O rosto mude de identidade.
- O cabelo varie de comprimento sem justificativa.
- A remoção do fundo destrua a arte.
- A redução elimine detalhes fundamentais.
- Os quadros não possam ser alinhados coerentemente.
- A animação dependa de inventar imagens que não existem.
- As ferramentas disponíveis não permitam produzir um resultado confiável.

Nesse caso, publicar a auditoria e as evidências reais.

Não substituir a corrida por um boneco procedural.

Não declarar sucesso artístico.

---

# 29. TESTES

Executar:

`cargo fmt --check`

`cargo check`

`cargo clippy --all-targets --jobs 2 -- -D warnings`

`cargo test --jobs 2`

Executar o exemplo visual.

Adicionar testes para:

- Dimensões.
- Transparência.
- Quantidade de quadros.
- Manifestos.
- Pivôs.
- Integridade dos arquivos.
- Sequência de reprodução.
- Compatibilidade com a N3.5.

Não modificar testes antigos para esconder regressões.

Registrar resultados reais.

---

# 30. DOCUMENTAÇÃO

Criar:

`docs/development/n3_6/`

Documentos:

1. `N3_6_AUDITORIA.md`
2. `N3_6_REFERENCIAS.md`
3. `N3_6_IDENTIDADE.md`
4. `N3_6_ANALISE_CORRIDA.md`
5. `N3_6_QUADROS.md`
6. `N3_6_ALINHAMENTO.md`
7. `N3_6_RESOLUCAO.md`
8. `N3_6_CABELO.md`
9. `N3_6_FUNDO_ESCURO.md`
10. `N3_6_ESTUDO_SALTO.md`
11. `N3_6_ESTUDO_ATAQUE.md`
12. `N3_6_ESTUDO_IDLE.md`
13. `N3_6_BEVY.md`
14. `N3_6_TESTES.md`
15. `N3_6_AVALIACAO.md`
16. `N3_6_RELATORIO_FINAL.md`

Não criar documentos vazios.

Cada documento deve conter observações, medidas ou resultados reais.

---

# 31. ARQUIVOS PROTEGIDOS

Não alterar:

- `Md-usario/`
- `white_mesh.glb`
- Blender V2.2 original.
- `assets/data/camera.ron`
- Save v3.
- Controles N2.
- Protagonista publicado.
- Manifestos de produção.
- Sistema de combate.
- Sistema de magia.
- N3 a N3.5.

Não iniciar N4 ou N5.

Não executar Git destrutivo.

Não fazer force push.

Não fazer merge.

Não fazer rebase automático.

---

# 32. PUBLICAÇÃO NO GITHUB

Publicar os resultados reais na branch:

`origin/art/v1.2-protagonist`

Procedimento:

1. Confirmar branch.
2. Confirmar HEAD.
3. Inspecionar working tree.
4. Preservar alterações preexistentes.
5. Executar testes.
6. Conferir arquivos modificados.
7. Revisar evidências.
8. Criar commits organizados.
9. Fazer push.
10. Confirmar HEAD remoto.
11. Informar SHA completo.
12. Confirmar integridade dos originais.

Sugestões de commits, se corresponderem ao trabalho realizado:

`feat(visual): add n3.6 animation feasibility proof`

`docs(n3): record animation reference analysis and evidence`

`docs(n3): record published n3.6 commits`

Não criar commits vazios para cumprir uma quantidade.

Se houver impedimento, publicar apenas o trabalho efetivamente realizado.

---

# 33. RELATÓRIO FINAL

Responder:

**A. Estado inicial:** branch, HEAD e Git.

**B. Referências:** arquivos encontrados.

**C. Inspeção:** conteúdo visual real.

**D. Identidade:** consistência do protagonista.

**E. Corrida:** poses disponíveis.

**F. Quadros:** seleção e procedência.

**G. Anatomia:** diferenças identificadas.

**H. Cabelo:** continuidade.

**I. Extração:** método e limitações.

**J. Redução:** método e resultados.

**K. Resolução:** canvas e figura.

**L. Pivô:** alinhamento.

**M. Temporização:** FPS e ciclo.

**N. Bevy:** prova isolada.

**O. Câmera:** 405 versus 360.

**P. Fundo escuro:** legibilidade.

**Q. Salto:** estudo.

**R. Ataque:** estudo.

**S. Idle:** estudo.

**T. Modularidade:** riscos futuros.

**U. Testes:** resultados.

**V. Evidências:** arquivos produzidos.

**W. Segurança:** arquivos preservados.

**X. GitHub:** commits e HEAD remoto.

**Y. Limitações:** pendências reais.

**Z. Recomendação:** próxima etapa e gates pendentes.

---

# 34. REGRA FINAL

O proprietário já definiu a identidade do protagonista e forneceu novas referências de movimento.

A N3.6 deve utilizar esse material, não substituí-lo.

**Qualidade artística e consistência são mais importantes que quantidade de quadros.**

Não converter automaticamente uma folha ilustrada em uma animação supostamente pronta.

Não deformar o personagem para encaixá-lo num canvas.

Não inventar poses ausentes.

Não alterar a câmera principal.

Não alterar o jogo principal.

Não iniciar N4 ou N5.

**Execute a auditoria, tente a primeira prova real de corrida somente se as referências permitirem, publique resultados verificáveis e aguarde a avaliação artística.**