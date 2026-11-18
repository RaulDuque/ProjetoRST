# DARK CASTLE RPG — MEGA PROMPT N3.1
# REFINAMENTO ARTÍSTICO, PIXEL ART AUTORAL E VALIDAÇÃO REAL DO PIPELINE 3D → 2D

**Projeto:** Dark Castle RPG  
**Etapa:** N3.1 — Refinamento e validação artística  
**Engine:** Bevy 0.20.1  
**Linguagem:** Rust 2024  
**Branch:** `art/v1.2-protagonist`  
**HEAD publicado de referência:** `963cbb9`  
**Status:** AUTORIZADO PARA EXECUÇÃO  
**N4 e N5:** NÃO AUTORIZADAS

---

# 0. MISSÃO PRINCIPAL

Você atuará como diretor técnico de arte, especialista em Blender, pixel art, animação 2D, integração Bevy e engenharia de ferramentas.

A N3 demonstrou que o contrato técnico de camadas funciona, mas não demonstrou qualidade artística final.

Sua missão agora é responder:

**Conseguimos transformar o protagonista 3D existente em um personagem 2D de pixel art gótica detalhada, com animações convincentes e equipamentos modulares, usando um pipeline reproduzível?**

A resposta deve vir de imagens reais, arquivos editáveis, medições e testes, não apenas de documentação.

O objetivo não é concluir N4 nem substituir o protagonista publicado.

A N3.1 deve entregar material visual suficiente para que o proprietário decida se aprova ou rejeita o estilo.

---

# 1. LEITURA OBRIGATÓRIA

Antes de alterar qualquer arquivo, leia:

- `CLAUDE.md`
- `docs/ARCHITECTURE.md`
- `docs/DECISIONS.md`
- `docs/development/n1/N1_DIRECAO_ARTISTICA.md`
- `docs/development/n1/N1_PIXEL_ART_32_64_BITS.md`
- `docs/development/n1/N1_PIPELINE_3D_PARA_2D.md`
- `docs/development/n1/N1_ANIMACAO_ESQUELETICA_2D.md`
- `docs/development/n1/N1_PERSONAGEM_MODULAR.md`
- `docs/development/n1/N1_EQUIPAMENTOS_VISUAIS.md`
- `docs/development/n1/N1_PROVA_DE_CONCEITO.md`
- `docs/development/n3/N3_RELATORIO_FINAL.md`
- `docs/development/n3/N3_AVALIACAO_VISUAL.md`
- `docs/development/n3/N3_COMPARACAO_PIPELINES.md`
- `docs/development/n3/N3_MODELO_E_RIG.md`
- `docs/development/n3/N3_PIXEL_ART.md`
- `docs/development/n3/N3_ANIMACOES.md`
- `docs/development/n3/N3_METRICAS.md`
- `docs/development/n3/N3_TESTES.md`

Inspecione também:

- `art-src/player/blender/swordsman_rig_v007.blend`
- `art-src/player/scripts/bpy/build_game_sprites_v2_2.py`
- `art-src/player/scripts/post/game_sprites_v2_2.py`
- `art-src/n3-proof/scripts/build_n3_proof.py`
- `examples/n3_visual_proof.rs`
- `tests/n3_visual_proof_tests.rs`

Não assumir que os nomes, caminhos ou parâmetros permanecem iguais sem conferir.

---

# 2. ESTADO INICIAL E PROTEÇÃO

Confirmar:

- Branch atual.
- HEAD local.
- HEAD remoto.
- Working tree.
- Alterações preexistentes.
- Ferramentas instaladas.
- Versões de Blender, Python e bibliotecas.
- Estado dos recursos gerados pela N3.

Preservar integralmente:

- `Md-usario/`
- `Md-usario/DarkCastleR-D/`
- `Md-usario/assets-user/prota/base/white_mesh.glb`
- `saves/`
- `assets/animations/player.ron`
- `assets/animations/player_sockets.ron`
- `camera.ron`
- Save v3.
- Controles da N2.
- Sistemas de combate.
- Sistemas de progressão.

Não executar limpeza destrutiva.

Não substituir arquivos do jogo principal.

A N3.1 trabalha em recursos experimentais isolados.

---

# 3. CORRIGIR O PROBLEMA CENTRAL DA N3

A N3 produziu um personagem formado por elementos geométricos gerados com Pillow.

Isso foi suficiente para validar:

- Troca de arma.
- Troca de armadura.
- Capa.
- Pivôs.
- Sockets.
- Atlas.
- Reprodução de clipes.

Mas não foi suficiente para validar:

- Anatomia convincente.
- Rosto.
- Tecidos.
- Materiais.
- Peso do movimento.
- Acabamento manual.
- Qualidade de pixel art.
- Pipeline híbrido real.

**Não repetir a N3 apenas acrescentando mais detalhes geométricos ao script.**

Nesta etapa, o protagonista 3D existente deve ser usado como referência real para produzir arte visualmente mais elaborada.

O desenho procedural da N3 permanece como baseline técnico, não como candidato artístico principal.

---

# 4. AUDITORIA DO MANEQUIM V2.2

Abrir e inspecionar uma cópia de trabalho do projeto Blender:

`art-src/player/blender/swordsman_rig_v007.blend`

Verificar:

1. Armature.
2. Hierarquia dos ossos.
3. Malhas do corpo.
4. Casaco.
5. Mãos.
6. Pernas.
7. Arma embutida.
8. Materiais.
9. Iluminação.
10. Câmera ortográfica.
11. Ações disponíveis.
12. Poses de idle, walk, jump e attack.
13. Pontos de ancoragem.
14. Exportação atual.

O objetivo é aproveitar o que já existe.

Não remodelar o protagonista inteiro se isso não for necessário.

Não modificar o arquivo-fonte original em lugar.

Se forem necessários ajustes, criar uma variante experimental em `art-src/n3-proof/` ou outra pasta apropriada.

Não utilizar `white_mesh.glb` como substituto automático.

---

# 5. PIXELORAMA — INSTALAÇÃO E VALIDAÇÃO

A N3 não instalou Pixelorama.

Na N3.1, está autorizada uma tentativa de instalação segura e não invasiva do Pixelorama.

Requisitos:

- Obter de fonte oficial.
- Verificar a licença.
- Verificar compatibilidade com Linux.
- Preferir instalação local ou portátil.
- Evitar privilégios administrativos.
- Não executar scripts remotos sem inspeção.
- Não instalar ferramentas pagas.
- Não alterar o ambiente do usuário desnecessariamente.

Após a instalação:

1. Registrar versão.
2. Confirmar abertura.
3. Confirmar importação de PNG.
4. Confirmar edição de pixels.
5. Confirmar exportação.
6. Verificar suporte a arquivos-fonte editáveis.
7. Verificar se há automação por linha de comando e suas limitações.

**Importante:** instalar Pixelorama não comprova que houve acabamento manual.

Se não existir um meio legítimo de realizar edição manual no ambiente, declarar a limitação.

Não simular pinceladas humanas por script e chamá-las de trabalho manual.

Se a instalação falhar, continuar apenas com experimentos honestamente identificados e não aprovar o pipeline híbrido.

---

# 6. SELECIONAR O MESMO GESTO PARA COMPARAÇÃO

A N3 comparou gestos diferentes.

Agora isso precisa ser corrigido.

Selecionar um gesto específico do manequim V2.2.

Prioridade:

**Attack — preparação, impacto e recuperação.**

Se o arquivo Blender não possuir esse gesto em condição utilizável, selecionar idle ou walk e justificar.

Produzir uma referência comum com:

- Mesmo personagem.
- Mesma pose.
- Mesmo enquadramento.
- Mesma orientação.
- Mesma resolução-base.
- Mesmo pivô.
- Mesma iluminação de origem.
- Mesmo tamanho de apresentação.

Não comparar imagens de personagens diferentes como se fossem alternativas equivalentes.

---

# 7. TRÊS RESULTADOS VISUAIS DISTINTOS

Para o mesmo gesto, produzir:

## A. Render bruto do Blender

Exportar quadros ortográficos sem acabamento.

Registrar parâmetros.

Preservar transparência quando apropriado.

Não usar o PNG publicado no jogo como se fosse um render novo, sem identificar sua origem.

## B. Render convertido e artisticamente corrigido

Partir do render A.

Reduzir para resolução de trabalho apropriada.

Corrigir, quando houver capacidade real:

- Contorno.
- Silhueta.
- Escadas de pixel.
- Materiais.
- Reflexos.
- Sombras.
- Ruído.
- Rosto.
- Cabelo ou elmo.
- Casaco.
- Mãos.
- Separação entre membros.
- Legibilidade da arma.

Salvar arquivo editável do Pixelorama.

Se o acabamento não puder ser feito manualmente, identificar a imagem como processamento automático, não como pintura manual.

## C. Quadro desenhado independentemente

Criar uma interpretação em pixel art do mesmo gesto, sem simplesmente aplicar filtro ao render.

Usar o mesmo tamanho, pivô e enquadramento.

Documentar a autoria e o processo.

Se não houver capacidade real para produzir essa versão, registrar como não executada.

**Não inventar evidências.**

A comparação deve deixar claro o que foi efetivamente produzido.

---

# 8. QUALIDADE VISUAL DO PROTAGONISTA

O protagonista deve apresentar identidade gótica reconhecível.

Prioridades:

- Anatomia humana legível.
- Proporção coerente.
- Cabeça distinguível.
- Torso estruturado.
- Braços e mãos reconhecíveis.
- Pernas com volume.
- Botas.
- Casaco ou armadura.
- Separação entre tecidos e metais.
- Contorno controlado.
- Iluminação consistente.

Evitar:

- Membros de cápsula.
- Articulações circulares evidentes.
- Silhueta de boneco de teste.
- Reflexos de render 3D sem correção.
- Contornos borrados.
- Pixels de tamanhos inconsistentes.
- Excesso de detalhe que desaparece na escala do jogo.

A arte não precisa estar pronta para produção em massa.

Precisa demonstrar uma direção visual convincente.

---

# 9. PALETA E MATERIAIS

Definir uma paleta experimental limitada.

Separar cores de:

- Pele.
- Tecido.
- Couro.
- Metal.
- Ouro.
- Sombra.
- Luz.
- Capa.
- Armas.

Não utilizar o mesmo brilho para metal, pele e tecido.

Evitar gradientes excessivamente suaves.

Registrar as cores utilizadas.

O resultado deve permanecer legível sobre fundo escuro.

Produzir uma comparação com pelo menos dois fundos:

- Fundo neutro.
- Fundo gótico escuro.

A silhueta não pode depender exclusivamente de um contorno branco.

---

# 10. REFINAR AS ANIMAÇÕES

A N3 possui:

- Idle: 4 quadros.
- Walk: 6 quadros.
- Jump: 6 quadros.
- Attack: 6 quadros.

Não é obrigatório aumentar todos os clipes.

Priorizar qualidade e coerência.

## Idle

Melhorar respiração, postura e movimento secundário.

Evitar tremulação do pivô.

## Walk

Corrigir transferência de peso.

Observar:

- Contato do pé.
- Passagem.
- Elevação.
- Apoio.
- Oscilação do quadril.
- Movimento dos braços.

Evitar pés deslizando.

## Jump

Corrigir o problema identificado na N3: a capa atravessa o corpo.

Separar visualmente:

- Preparação.
- Impulso.
- Subida.
- Ápice.
- Queda.
- Recuperação.

A animação pode usar os estados disponíveis no protótipo sem alterar a física real do jogo.

## Attack

Melhorar:

- Antecipação.
- Aceleração.
- Silhueta no impacto.
- Recuperação.
- Arma seguindo a mão.
- Ordem de desenho.
- Movimento da capa.

Preservar o contrato de sockets.

Não alterar o timing de combate do jogo principal.

---

# 11. CAPA — CORREÇÃO OBRIGATÓRIA

A N3 registrou que a capa cruza o corpo durante o salto.

Investigar a causa:

- Socket incorreto.
- Pivô.
- Rotação.
- Ordem de desenho.
- Silhueta da capa.
- Quadro incompatível.
- Espelhamento.

Corrigir no protótipo isolado.

Testar a capa durante:

- Idle.
- Walk.
- Jump.
- Attack.

Testar também:

- Capa ligada.
- Capa desligada.
- Personagem espelhado.
- Armadura leve.
- Armadura pesada.

Não esconder o defeito simplesmente desativando a capa durante todo o salto.

Se a solução exigir uma variante específica de capa, documentar o custo.

---

# 12. ARMADURAS E ARMAS

Manter o objetivo da N3:

- Duas armaduras.
- Duas armas.
- Uma capa.

Não criar dezenas de itens.

Usar o mesmo gesto refinado para verificar se as duas armaduras mantêm:

- Pivô.
- Alinhamento.
- Proporção.
- Silhueta.
- Posição da mão.
- Compatibilidade com a capa.

A arma deve continuar independente do corpo.

Verificar que a versão artística não introduz uma espada embutida no atlas do protagonista.

Se uma armadura exigir variante específica do corpo, registrar essa necessidade.

Não criar uma folha por combinação arma-armadura.

---

# 13. RESOLUÇÃO E LEGIBILIDADE

Manter o experimento:

**640×360 → 1280×720 em escala inteira 2×.**

Comparar:

1. Personagem em tamanho de apresentação próximo de 80–110 texels.
2. Enquadramento equivalente ao jogo atual.
3. Fundo neutro.
4. Fundo escuro.
5. Armadura leve.
6. Armadura pesada.

Registrar:

- Altura em texels.
- Altura exibida em pixels.
- Dimensões do atlas.
- Escala aplicada.
- Filtro.
- Área visível de cenário.

Não confundir altura do sprite com altura do personagem no mundo.

Não modificar a câmera do jogo principal.

Não alterar `camera.ron`.

---

# 14. COMPARAÇÃO DE CUSTO DE PRODUÇÃO

Registrar tempos reais de:

- Preparação do Blender.
- Render.
- Conversão.
- Correção de pixels.
- Ajuste de sockets.
- Montagem de atlas.
- Importação no Bevy.
- Correção de animação.
- Troca de equipamento.

Separar:

**Tempo de máquina** e **tempo de trabalho humano**.

Não inventar tempo de artista.

Se não houver artista humano trabalhando, registrar explicitamente.

Medir o tempo necessário para atualizar uma arma ou armadura depois de uma alteração no clipe.

O objetivo é estimar a viabilidade de produzir muitos equipamentos no futuro.

---

# 15. PROTÓTIPO EXECUTÁVEL

Reutilizar:

`examples/n3_visual_proof.rs`

Preferencialmente estender a prova sem quebrar sua execução anterior.

Adicionar modo de comparação N3.1, se necessário.

Permitir:

- Alternar render bruto e pixel art.
- Alternar armaduras.
- Alternar armas.
- Alternar capa.
- Alternar clipes.
- Pausar.
- Avançar quadros.
- Espelhar.
- Alternar enquadramento.

Mostrar claramente qual imagem está sendo exibida.

Não substituir o personagem do jogo principal.

Não iniciar N4.

---

# 16. EVIDÊNCIAS VISUAIS

Criar:

`docs/development/n3_1/evidence/`

Entregar, quando realmente produzidos:

1. Render Blender bruto.
2. Render convertido automaticamente.
3. Render com acabamento manual.
4. Quadro desenhado independentemente.
5. Comparação do mesmo gesto.
6. Comparação de silhuetas.
7. Armadura leve.
8. Armadura pesada.
9. Espada.
10. Lança.
11. Capa durante jump.
12. Capa durante attack.
13. Walk em sequência.
14. Jump em sequência.
15. Attack em sequência.
16. Enquadramento 640×360.
17. Enquadramento equivalente ao jogo atual.
18. Painel antes/depois da N3.

Para cada evidência, registrar sua origem.

Não apresentar filtro como pintura manual.

Não gerar arquivos vazios apenas para completar a lista.

Se possível, fornecer GIF ou vídeo da animação.

As evidências devem ser suficientes para avaliação visual sem exigir que o proprietário execute o jogo.

---

# 17. ARQUIVOS-FONTE EDITÁVEIS

Preservar:

- Projeto Blender experimental.
- Arquivos Pixelorama, quando utilizados.
- Scripts de exportação.
- Paleta.
- Atlas.
- Manifesto.
- Tabela de sockets.
- Configurações de render.

Os arquivos-fonte precisam ser suficientes para reproduzir o resultado.

Não incluir caches e temporários desnecessários.

Não sobrescrever a fonte V2.2.

Não substituir o pipeline antigo antes da aprovação.

---

# 18. TESTES AUTOMATIZADOS

Manter os testes da N3.

Adicionar verificações para:

- Quatro clipes válidos.
- Pivôs consistentes.
- Sockets existentes.
- Armas separadas do corpo.
- Dimensões dos atlas.
- Referências de arquivos.
- Sincronização de frames.
- Troca de armadura.
- Troca de arma.
- Espelhamento.
- Ordem de desenho.
- Compatibilidade com o manifesto.
- Ausência de regressões da N2.

Testes automatizados não substituem avaliação artística.

---

# 19. VALIDAÇÃO GRÁFICA

Executar o protótipo, quando possível.

Conferir:

- Animações em movimento.
- Troca de equipamento.
- Pivôs.
- Capa.
- Silhueta.
- Pixel perfect.
- Enquadramento.
- Ausência de texturas ausentes.
- Ausência de artefatos visuais.

Produzir evidências reais.

Se a execução gráfica falhar, registrar a limitação.

Não declarar validação concluída com base apenas em compilação.

---

# 20. GATES TÉCNICOS

Executar:

`cargo fmt --check`

`cargo check`

`cargo clippy --all-targets --jobs 2 -- -D warnings`

`cargo test --jobs 2`

Executar o exemplo N3/N3.1.

Registrar resultados.

Não remover testes.

Não mascarar warnings.

Não introduzir dependências sem justificativa.

---

# 21. DOCUMENTAÇÃO

Criar:

`docs/development/n3_1/`

Documentos:

1. `N3_1_AUDITORIA.md`
2. `N3_1_BLENDER.md`
3. `N3_1_PIXELORAMA.md`
4. `N3_1_COMPARACAO_ARTISTICA.md`
5. `N3_1_ANIMACOES.md`
6. `N3_1_EQUIPAMENTOS.md`
7. `N3_1_CAPA.md`
8. `N3_1_RESOLUCAO.md`
9. `N3_1_METRICAS.md`
10. `N3_1_TESTES.md`
11. `N3_1_AVALIACAO.md`
12. `N3_1_RELATORIO_FINAL.md`

Não declarar aprovação artística em nome do proprietário.

---

# 22. CRITÉRIOS DE ACEITAÇÃO

A N3.1 deverá responder:

- [ ] O render do Blender foi realmente utilizado?
- [ ] O mesmo gesto foi comparado?
- [ ] O Pixelorama foi instalado e validado?
- [ ] Houve acabamento manual real?
- [ ] Existe arquivo-fonte editável?
- [ ] A silhueta melhorou?
- [ ] O rosto está mais legível?
- [ ] O tecido se distingue do metal?
- [ ] O walk possui melhor transferência de peso?
- [ ] O jump está mais legível?
- [ ] A capa não atravessa o corpo?
- [ ] A arma acompanha a mão?
- [ ] As armaduras continuam alinhadas?
- [ ] Os equipamentos continuam modulares?
- [ ] A resolução mantém pixels consistentes?
- [ ] O protótipo abre?
- [ ] Os testes passam?
- [ ] As evidências foram publicadas?
- [ ] Os arquivos originais foram preservados?

Cada item deve receber:

- Concluído.
- Parcial.
- Não concluído.
- Não verificado.

Não marcar como concluído sem evidência.

---

# 23. PUBLICAÇÃO OBRIGATÓRIA NO GITHUB

Ao finalizar:

1. Conferir branch.
2. Conferir alterações preexistentes.
3. Revisar o diff.
4. Conferir arquivos protegidos.
5. Executar testes.
6. Criar commits organizados.
7. Publicar código, fontes, documentação e evidências.
8. Fazer push para `origin/art/v1.2-protagonist`.
9. Confirmar SHA remoto.
10. Informar os hashes dos commits.

Não executar force push.

Não fazer merge.

Não fazer rebase automaticamente.

Não incluir arquivos pessoais fora do escopo.

Se o remoto tiver avançado, interromper e pedir orientação.

Se a publicação falhar, não declarar sucesso.

O relatório deve distinguir o HEAD inicial do HEAD publicado.

---

# 24. RELATÓRIO FINAL OBRIGATÓRIO

Apresentar:

**A. Estado inicial** — Branch, HEAD e árvore.

**B. Ferramentas** — Versões e instalação.

**C. Blender** — Fonte e render.

**D. Pixelorama** — Uso real e limitações.

**E. Comparação** — Mesmo gesto, três caminhos.

**F. Direção artística** — Silhueta, materiais e acabamento.

**G. Animações** — Idle, walk, jump e attack.

**H. Capa** — Correção do cruzamento.

**I. Armas** — Sincronização e modularidade.

**J. Armaduras** — Pivôs e diferenças.

**K. Resolução** — Escala e legibilidade.

**L. Protótipo** — Comandos de execução.

**M. Evidências** — Arquivos e imagens.

**N. Métricas** — Tempo, tamanho e memória.

**O. Testes** — Resultados reais.

**P. Critérios** — Checklist com evidências.

**Q. Limitações** — Tudo que ficou pendente.

**R. Arquivos** — Criados e modificados.

**S. Segurança** — Integridade dos originais.

**T. GitHub** — Commits e SHA remoto.

**U. Recomendação** — Pipeline a seguir, rejeitar ou continuar experimentando.

**V. Aprovação pendente** — O que o proprietário precisa avaliar.

---

# 25. REGRA DE ENCERRAMENTO

A N3.1 não é uma corrida para marcar todas as tarefas como concluídas.

Se o pipeline não produzir qualidade suficiente, registrar isso.

Se o Pixelorama não puder ser utilizado, registrar isso.

Se a comparação manual não for possível, não inventar um resultado.

Se a capa continuar atravessando o corpo, mostrar a falha.

Se a arte continuar com aparência de manequim ou desenho geométrico, não recomendar promoção.

**O objetivo é tomar uma decisão artística informada, não fabricar uma aprovação.**

Concluir a execução técnica, publicar os resultados no GitHub e aguardar a avaliação do proprietário.

**Não iniciar N4 ou N5. Não substituir o protagonista do jogo principal.**