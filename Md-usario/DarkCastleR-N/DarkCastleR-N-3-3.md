# DARK CASTLE RPG — MEGA PROMPT N3.3
# PIXEL ART NATIVA, PERSONAGEM CONSISTENTE E ANIMAÇÃO MODULAR

**Projeto:** Dark Castle RPG  
**Etapa:** N3.3 — Prova artística nativa  
**Branch autorizada:** `art/v1.2-protagonist`  
**HEAD remoto inicial esperado:** `fb9ac29487b590f0430bf95c4f406bc4aa4dd91e`  
**Tecnologias:** Rust 2024, Bevy 0.20.1, Blender, Pixelorama e ferramentas existentes  
**Status:** Execução autorizada; aprovação artística pendente  
**N4 e N5:** BLOQUEADAS

---

# 0. ORDEM PRINCIPAL

Você é responsável pela execução técnica da N3.3 do Dark Castle RPG.

Sua missão é produzir uma prova visual de um protagonista gótico em pixel art, desenhado para sua resolução final e tecnicamente compatível com o sistema modular existente.

Esta etapa NÃO deve repetir os erros das N3, N3.1 e N3.2.

A N3 demonstrou composição técnica de corpo, arma e capa, mas utilizou geometria simplificada.

A N3.1 demonstrou renderização do manequim Blender V2.2, mas não acabamento artístico. A redução automática perdeu o rosto e misturou materiais.

A N3.2 demonstrou que a geração de imagens consegue criar ilustrações góticas grandes, mas falha quando essas imagens são reduzidas para aproximadamente 100 pixels. As poses geradas independentemente também apresentaram inconsistências de anatomia, câmera, roupas e empunhadura.

**A N3.3 deve resolver o problema pela origem: criar uma arte projetada para 100 pixels, mantendo a geometria e a animação controladas.**

O resultado esperado não é um jogo completo, nem uma coleção de ilustrações. É uma prova visual pequena, verificável e reproduzível.

---

# 1. REGRAS ABSOLUTAS

1. Não iniciar N4.
2. Não iniciar N5.
3. Não substituir o protagonista do jogo principal.
4. Não modificar a câmera principal.
5. Não alterar o save v3.
6. Não alterar os controles da N2.
7. Não modificar arquivos pessoais.
8. Não sobrescrever o Blender V2.2 original.
9. Não promover automaticamente a arte.
10. Não chamar redução automática de pintura manual.
11. Não chamar imagem gerada por IA de sprite modular.
12. Não criar um novo personagem geométrico de cápsulas como substituto artístico.
13. Não inventar testes, evidências ou medições.
14. Não gerar dezenas de quadros antes de validar um quadro.
15. Publicar o trabalho concluído no GitHub.

O proprietário ainda não aprovou nenhum estilo.

---

# 2. AUDITORIA INICIAL

Ler os documentos existentes, incluindo:

- `CLAUDE.md`
- `docs/ARCHITECTURE.md`
- `docs/DECISIONS.md`
- `docs/development/n1/N1_PIXEL_ART_32_64_BITS.md`
- `docs/development/n1/N1_PIPELINE_3D_PARA_2D.md`
- `docs/development/n1/N1_PERSONAGEM_MODULAR.md`
- `docs/development/n1/N1_EQUIPAMENTOS_VISUAIS.md`
- `docs/development/n3/N3_RELATORIO_FINAL.md`
- `docs/development/n3_1/N3_1_RELATORIO_FINAL.md`
- `docs/development/n3_2/N3_2_RELATORIO_FINAL.md`
- `docs/development/n3_2/N3_2_AVALIACAO.md`
- `docs/development/n3_2/N3_2_CONSISTENCIA.md`
- `docs/development/n3_2/N3_2_MODULARIDADE.md`

Inspecionar os códigos e arquivos de arte correspondentes.

Verificar a existência dos caminhos antes de utilizá-los.

Confirmar:

- Branch atual.
- HEAD local e remoto.
- Estado da árvore.
- Alterações preexistentes.
- Versão do Blender.
- Disponibilidade do Pixelorama.
- Scripts existentes.
- Manifestos de animação.
- Sistema de sockets.
- Exemplo visual N3.
- Referências geradas na N3.2.

Não iniciar modificações antes de concluir a auditoria.

---

# 3. PROTEÇÃO DOS ARQUIVOS

Preservar integralmente:

`Md-usario/`

`Md-usario/assets-user/prota/base/white_mesh.glb`

`art-src/player/blender/swordsman_rig_v007.blend`

`assets/animations/player.ron`

`assets/animations/player_sockets.ron`

`camera.ron`

`saves/`

Preservar também todos os recursos do protagonista publicado e os arquivos das N3, N3.1 e N3.2.

O arquivo `white_mesh.glb` possuía 3.387.148 bytes na última auditoria. Conferir sua integridade sem reescrevê-lo.

Não executar operações Git destrutivas.

Criar uma área isolada para a N3.3:

`art-src/n3-3-native-pixel/`

Criar evidências em:

`docs/development/n3_3/evidence/`

---

# 4. NOVA ESTRATÉGIA ARTÍSTICA

A N3.2 tentou o caminho:

Ilustração grande → redução → quantização → sprite.

Esse caminho falhou.

A N3.3 deve testar:

Referência artística → simplificação consciente → desenho nativo → animação controlada → atlas → Bevy.

A diferença é fundamental.

Não queremos conservar todos os detalhes da ilustração.

Queremos conservar sua identidade visual.

O sprite final deve ser projetado considerando:

- Poucos pixels.
- Contraste.
- Silhueta.
- Separação de materiais.
- Leitura lateral.
- Movimento.
- Modularidade.

A IA pode orientar o desenho, mas não deve controlar independentemente cada quadro de animação.

---

# 5. REFERÊNCIA VISUAL INICIAL

Reutilizar as três propostas da N3.2:

- Cavaleiro gótico de capa vermelha.
- Caçador aristocrático de casaco.
- Guerreiro amaldiçoado de capuz.

Não gerar novamente dezenas de conceitos.

Para a prova técnica, utilizar provisoriamente o **caçador aristocrático** como referência principal, pois sua silhueta de casaco é a mais próxima da estrutura existente do manequim V2.2.

Essa escolha é experimental.

Não significa aprovação do conceito B pelo proprietário.

Preservar os outros dois conceitos para comparação futura.

---

# 6. DESENHAR PARA 100 PIXELS

Criar um estudo de personagem com altura opaca próxima de 100 pixels.

Faixa inicial:

**96 a 112 texels de altura.**

A resolução final deve ser medida, não presumida.

Usar canvas com espaço para:

- Ataques.
- Arma.
- Capa.
- Movimento.
- Pivô.
- Margens transparentes.

Não confundir tamanho do canvas com altura do personagem.

Não aumentar artificialmente a resolução apenas para esconder problemas de desenho.

O sprite precisa ser legível em tamanho nativo.

---

# 7. ESTUDO DE SILHUETA

Antes de pintar detalhes, criar pelo menos três variações de silhueta do mesmo caçador:

A. Casaco longo e estreito.

B. Casaco médio com ombreiras.

C. Casaco longo com capa curta.

As três devem manter:

- Mesma anatomia-base.
- Mesmo tamanho de cabeça.
- Mesmo comprimento das pernas.
- Mesmo pivô.
- Mesma orientação lateral.

Mostrar as silhuetas em preto sólido.

Testar em fundo claro e escuro.

Não gastar tempo com ornamentos antes de selecionar uma silhueta tecnicamente legível.

Registrar qual variação funciona melhor e por quê.

A escolha continua provisória.

---

# 8. ANATOMIA E PROPORÇÕES

Definir uma única estrutura corporal.

Documentar:

- Altura total.
- Altura da cabeça.
- Posição dos ombros.
- Posição dos cotovelos.
- Posição das mãos.
- Quadril.
- Joelhos.
- Tornozelos.
- Posição dos pés.
- Pivô no chão.

A mesma estrutura deve ser usada em todas as poses.

Não permitir que o tamanho da cabeça varie entre quadros.

Não permitir que os braços mudem de comprimento.

Não permitir que o casaco cresça ou encolha arbitrariamente.

Se utilizar Blender, as poses devem derivar do mesmo rig.

Se utilizar desenho 2D, manter guias de proporção e pontos de referência comuns.

---

# 9. ROSTO LEGÍVEL

A N3.1 e a N3.2 perderam o rosto na redução.

Corrigir isso com desenho específico para pixel art.

Não tentar conservar o rosto detalhado da ilustração original.

Criar uma representação simplificada com:

- Formato de cabeça.
- Testa.
- Nariz.
- Queixo.
- Cabelo.
- Região dos olhos.
- Contraste entre pele e cabelo.

O rosto deve indicar claramente a direção do olhar.

Não é necessário desenhar olhos realistas.

É necessário evitar que a cabeça pareça uma mancha sem identidade.

Criar uma comparação:

- Rosto original em alta resolução.
- Rosto reduzido automaticamente.
- Rosto redesenhado em pixels nativos.

Mostrar o resultado ampliado com nearest-neighbor.

---

# 10. DIFERENCIAÇÃO DE MATERIAIS

A N3.1 falhou em separar tecido e metal.

Criar famílias de cores distintas para:

- Pele.
- Cabelo.
- Tecido.
- Couro.
- Aço.
- Ouro.
- Capa.

Utilizar diferenças de:

- Valor.
- Saturação.
- Temperatura.
- Forma dos reflexos.
- Padrão de sombreamento.

O metal deve ter reflexos mais concentrados.

O tecido deve ter sombras mais suaves e agrupadas.

O couro deve ser distinguível dos dois.

Não depender apenas da cor para diferenciar os materiais.

Evitar dithering excessivo.

Evitar granulado semelhante ao da redução N3.2.

Não copiar a filigrana das ilustrações grandes pixel por pixel.

---

# 11. PALETA CONTROLADA

Criar uma paleta pequena, mas suficiente para diferenciar materiais.

Registrar os valores RGB ou RGBA.

A quantidade de cores deve ser justificada pelo resultado visual.

Não impor um limite arbitrário que destrua a leitura do personagem.

Evitar centenas de cores quase iguais.

Criar comparação com:

- Paleta automática N3.2.
- Paleta nativa N3.3.

A comparação deve demonstrar se houve melhoria real.

---

# 12. PRIMEIRO MARCO: UM ÚNICO QUADRO

Antes de criar animações, produzir um quadro estático de perfil.

Requisitos:

- Altura próxima de 100 texels.
- Silhueta reconhecível.
- Rosto legível.
- Casaco.
- Botas.
- Braços.
- Mãos.
- Materiais diferenciados.
- Capa separada.
- Arma separada.
- Pivô documentado.

Salvar o arquivo-fonte editável.

Produzir comparação com N3.1 e N3.2.

**Não avançar para dezenas de quadros se este primeiro quadro ainda parecer um manequim geométrico ou uma ilustração granulada.**

Se o primeiro quadro não alcançar qualidade suficiente, registrar a falha e concentrar os esforços em corrigir o desenho.

---

# 13. FERRAMENTAS DE PINTURA

Verificar Pixelorama 1.2.4 ou versão efetivamente instalada.

Confirmar se abre em modo gráfico.

Confirmar se consegue:

- Importar PNG.
- Editar pixels.
- Trabalhar com camadas.
- Salvar projeto.
- Exportar PNG.

Preferir produzir arquivos-fonte editáveis.

Não afirmar pintura manual se ela não ocorreu.

Se o ambiente não permitir edição manual, não substituir essa exigência por um script de discos ou cápsulas.

É permitido criar ferramentas auxiliares para:

- Montar atlas.
- Validar paleta.
- Exportar camadas.
- Verificar transparência.
- Medir pivôs.
- Testar consistência.

Essas ferramentas não substituem o julgamento artístico.

Se a qualidade depender de intervenção humana indisponível, interromper o gate artístico e apresentar o quadro e as instruções de edição ao proprietário.

---

# 14. BLENDER COMO FONTE DE MOVIMENTO

Utilizar o manequim V2.2 como referência de movimento e proporção quando tecnicamente apropriado.

Não copiar automaticamente a aparência lisa do render.

O Blender deve controlar:

- Poses.
- Orientação.
- Comprimento dos membros.
- Movimento dos braços.
- Movimento das pernas.
- Empunhadura.
- Pivôs.

A pixel art deve controlar:

- Contorno final.
- Rosto.
- Materiais.
- Detalhes.
- Silhueta visual.
- Correção de pixels.

Criar uma cópia experimental do `.blend` se forem necessários ajustes.

Nunca salvar alterações no original.

---

# 15. ATAQUE COM ANTECIPAÇÃO

A N3.1 registrou que o ataque do manequim não tinha antecipação.

A N3.2 tentou gerar antecipação, impacto e recuperação por IA, mas produziu poses inconsistentes.

A N3.3 deve criar um único gesto controlado.

Fases:

1. Postura inicial.
2. Antecipação.
3. Início do golpe.
4. Impacto.
5. Seguimento.
6. Recuperação.

Não é obrigatório usar exatamente seis quadros.

O número final deve ser escolhido conforme o movimento.

A antecipação precisa ser visualmente identificável.

O impacto deve apresentar silhueta forte.

A recuperação deve retornar a uma postura coerente.

A espada deve acompanhar a mão.

---

# 16. CONSISTÊNCIA DE ANIMAÇÃO

Todos os quadros devem compartilhar:

- Mesmo personagem.
- Mesmo traje.
- Mesmo tamanho de cabeça.
- Mesmo comprimento de membros.
- Mesmo casaco.
- Mesma direção de câmera.
- Mesmo pivô.
- Mesma paleta.
- Mesma fonte de iluminação.

Criar uma folha comparativa de poses.

Adicionar guias visuais de:

- Cabeça.
- Ombro.
- Quadril.
- Pés.
- Mão dominante.

Diferenças naturais decorrentes da pose são permitidas.

Mudanças arbitrárias de anatomia não são.

Não utilizar três imagens independentes geradas por IA como se fossem quadros de uma animação.

---

# 17. ARMA INDEPENDENTE

A espada não deve estar incorporada ao corpo.

Criar:

- Sprite de espada.
- Ponto de empunhadura.
- Ângulo.
- Socket da mão.
- Ordem de desenho.

Testar a espada durante:

- Preparação.
- Impacto.
- Recuperação.

Verificar:

- Alinhamento da mão.
- Rotação.
- Espelhamento.
- Ausência de espada duplicada.
- Ausência de saltos abruptos.

Preservar `weapon_in_body: false` no contrato experimental, quando aplicável.

Não alterar o manifesto do protagonista publicado.

---

# 18. CAPA INDEPENDENTE

A capa deve ser uma camada separada.

Criar um desenho que funcione em aproximadamente 100 pixels.

Testar:

- Posição atrás do corpo.
- Movimento durante o ataque.
- Alinhamento com o ombro.
- Espelhamento.
- Ausência de cruzamento incorreto.

Não utilizar a capa embutida nas ilustrações N3.2 como se fosse uma camada pronta.

Se for necessário redesenhá-la, fazer isso na resolução final.

A capa deve preservar a silhueta gótica sem esconder completamente o corpo.

---

# 19. ARMADURA E MODULARIDADE

Nesta etapa, produzir uma armadura-base.

Não é obrigatório criar duas armaduras completas.

Entretanto, demonstrar que a estrutura escolhida não impede futuras variantes.

Documentar:

- Quais partes pertencem ao corpo.
- Quais partes pertencem à armadura.
- Quais partes precisam ser redesenhadas por pose.
- Quais partes podem ser compartilhadas.
- Quais sockets são reutilizáveis.

Se possível, produzir um pequeno teste de substituição de ombreira ou torso.

Não iniciar a produção modular completa da N4.

---

# 20. ESCALA E CÂMERA

Manter o experimento isolado de:

**640×360 → 1280×720, escala inteira 2×.**

Testar também a aparência no enquadramento equivalente ao jogo atual.

Não modificar `camera.ron`.

Não alterar `cargo run`.

Comparar:

- Pixel nativo.
- Ampliação nearest.
- Fundo neutro.
- Fundo escuro.
- Personagem com capa.
- Personagem sem capa.
- Personagem com espada.
- Personagem sem espada.

Registrar dimensões reais.

---

# 21. INTEGRAÇÃO COM BEVY

Reutilizar:

`examples/n3_visual_proof.rs`

Ou criar um exemplo experimental específico quando houver justificativa.

Permitir alternar entre:

- N3 geométrico.
- N3.1 manequim reduzido.
- N3.2 conceito reduzido.
- N3.3 sprite nativo.

Identificar cada variante na interface.

Não apresentar a N3.3 como arte aprovada.

A integração deve demonstrar:

- Carregamento de atlas.
- Pivô.
- Socket.
- Arma separada.
- Capa separada.
- Espelhamento.
- Reprodução do ataque.

Não alterar o personagem do jogo principal.

---

# 22. EVIDÊNCIAS OBRIGATÓRIAS

Criar:

`docs/development/n3_3/evidence/`

Entregar, conforme realmente produzidos:

1. `silhouette_study.png`
2. `native_idle.png`
3. `face_comparison.png`
4. `material_comparison.png`
5. `palette_comparison.png`
6. `n3_n31_n32_n33.png`
7. `attack_anticipation.png`
8. `attack_impact.png`
9. `attack_recovery.png`
10. `attack_contact_sheet.png`
11. `weapon_separated.png`
12. `cape_separated.png`
13. `socket_debug.png`
14. `native_100px.png`
15. `nearest_2x.png`
16. `dark_background.png`
17. `bevy_n33.png`
18. `animation_sequence.png`

Não criar arquivos fictícios para completar a lista.

Quando possível, produzir GIF ou vídeo do ataque.

A sequência precisa mostrar movimento, não apenas poses desconectadas.

---

# 23. MÉTRICAS

Registrar:

- Altura opaca do personagem.
- Largura por quadro.
- Dimensões do atlas.
- Quantidade de cores.
- Tamanho dos PNG.
- Memória estimada das texturas.
- Tempo de render.
- Tempo de adaptação.
- Tempo de montagem.
- Tempo de edição manual, quando houver.
- Tempo de integração no Bevy.

Separar tempo de máquina de tempo humano.

Não inventar horas de artista.

Comparar o custo com N3, N3.1 e N3.2.

---

# 24. TESTES

Executar:

`cargo fmt --check`

`cargo check`

`cargo clippy --all-targets --jobs 2 -- -D warnings`

`cargo test --jobs 2`

Executar o exemplo gráfico.

Adicionar testes para:

- Manifesto.
- Atlas.
- Sockets.
- Pivôs.
- Arma separada.
- Capa separada.
- Dimensões.
- Espelhamento.
- Consistência de quadros.
- Carregamento de recursos.

Não alterar testes antigos para ocultar regressões.

Registrar resultados reais.

---

# 25. GATES DE ACEITAÇÃO

## Gate A — Arte nativa

Verificar:

- [ ] Sprite desenhado para aproximadamente 100 px.
- [ ] Silhueta legível.
- [ ] Rosto legível.
- [ ] Tecido distinguível de metal.
- [ ] Paleta coerente.
- [ ] Ausência de granulado automático.
- [ ] Arquivo-fonte editável.
- [ ] Processo de autoria documentado.

## Gate B — Consistência

Verificar:

- [ ] Mesma anatomia entre poses.
- [ ] Mesmo traje.
- [ ] Mesma câmera.
- [ ] Mesmo pivô.
- [ ] Antecipação identificável.
- [ ] Impacto legível.
- [ ] Recuperação coerente.
- [ ] Ausência de mudanças arbitrárias.

## Gate C — Modularidade

Verificar:

- [ ] Espada separada.
- [ ] Capa separada.
- [ ] Socket funcional.
- [ ] Espelhamento correto.
- [ ] Atlas válido.
- [ ] Integração Bevy.
- [ ] Sem alteração do jogo principal.

Classificar cada item como:

- Concluído.
- Parcial.
- Não concluído.
- Não verificado.

Não aprovar um gate com base apenas em testes automatizados.

---

# 26. DOCUMENTAÇÃO

Criar:

`docs/development/n3_3/`

Arquivos:

1. `N3_3_AUDITORIA.md`
2. `N3_3_DIRECAO_ARTISTICA.md`
3. `N3_3_SILHUETA.md`
4. `N3_3_PIXEL_ART_NATIVA.md`
5. `N3_3_ANATOMIA.md`
6. `N3_3_PALETA.md`
7. `N3_3_ANIMACAO.md`
8. `N3_3_ARMA.md`
9. `N3_3_CAPA.md`
10. `N3_3_MODULARIDADE.md`
11. `N3_3_INTEGRACAO_BEVY.md`
12. `N3_3_METRICAS.md`
13. `N3_3_TESTES.md`
14. `N3_3_AVALIACAO.md`
15. `N3_3_RELATORIO_FINAL.md`

A documentação deve conter imagens, medições e conclusões verificáveis.

Não escrever que o estilo foi aprovado.

---

# 27. PUBLICAÇÃO OBRIGATÓRIA

Ao finalizar o trabalho autorizado:

1. Conferir a branch.
2. Conferir HEAD local e remoto.
3. Revisar alterações.
4. Preservar arquivos protegidos.
5. Executar testes.
6. Criar commits organizados.
7. Publicar arte, scripts, código, documentação e evidências.
8. Fazer push para `origin/art/v1.2-protagonist`.
9. Confirmar HEAD remoto.
10. Informar SHA completo.

Não executar force push.

Não fazer merge.

Não fazer rebase automático.

Não alterar arquivos pessoais.

Se o remoto tiver avançado inesperadamente, interromper e solicitar orientação.

Se a publicação falhar, não declarar sucesso.

---

# 28. RELATÓRIO FINAL

Apresentar:

**A. Estado inicial** — Branch, HEAD e working tree.

**B. Referência** — Conceito utilizado.

**C. Silhueta** — Estudos realizados.

**D. Pixel art** — Processo real.

**E. Rosto** — Legibilidade.

**F. Materiais** — Tecido, metal e couro.

**G. Paleta** — Cores utilizadas.

**H. Anatomia** — Proporções e consistência.

**I. Blender** — Uso do rig.

**J. Ataque** — Antecipação, impacto e recuperação.

**K. Arma** — Sprite e socket.

**L. Capa** — Camada e movimento.

**M. Modularidade** — Limites.

**N. Bevy** — Integração.

**O. Resolução** — Escala e enquadramento.

**P. Evidências** — Arquivos.

**Q. Métricas** — Tempo e memória.

**R. Testes** — Resultados.

**S. Segurança** — Integridade dos originais.

**T. GitHub** — Commits e SHA remoto.

**U. Recomendação** — Prosseguir, corrigir ou rejeitar.

**V. Aprovação pendente** — Decisão do proprietário.

---

# 29. REGRA FINAL

A N3.3 não precisa entregar um protagonista definitivo.

Precisa entregar **uma prova convincente de que o protagonista pode ser criado em pixel art nativa, animado com consistência e composto por camadas independentes**.

Se não for possível alcançar essa qualidade com as ferramentas disponíveis, registrar o bloqueio.

Não mascarar problemas com mais scripts.

Não substituir pintura artística por filtros.

Não declarar uma animação válida se as poses forem inconsistentes.

Não promover a arte sem aprovação.

**Finalize a prova técnica, publique os resultados reais no GitHub e aguarde a avaliação do proprietário.**

**N4 e N5 permanecem bloqueadas.**