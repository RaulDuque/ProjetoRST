# DARK CASTLE RPG — MEGA PROMPT N3.7
# CONSOLIDAÇÃO DA CORRIDA, SINCRONIZAÇÃO DE PASSADA E PIPELINE DE ANIMAÇÕES

**Projeto:** Dark Castle RPG  
**Etapa:** N3.7 — Animation Consolidation & Motion Synchronization  
**Branch:** `art/v1.2-protagonist`  
**HEAD remoto inicial de referência:** `d4e5e046c3ae1e10c1cb5fd9f3f2945867e343c4`  
**Engine:** Bevy 0.20.1  
**Linguagem:** Rust 2024  
**Direção artística N3.5:** aprovada como referência  
**Corrida N3.6:** identidade e consistência dos quadros aprovadas visualmente  
**N4 e N5:** ainda bloqueadas

---

# 0. MISSÃO

Você é o responsável pela execução técnica e artística experimental da N3.7.

O proprietário finalmente aprovou a direção visual do protagonista e a sequência de corrida produzida na N3.6.

Essa aprovação é um marco importante.

**A partir de agora, não devemos continuar reinventando a aparência do protagonista.**

O padrão visual aprovado é:

- Personagem da ficha original.
- Cabelo longo branco-prateado.
- Rosto masculino sério.
- Camisa cinza-escura.
- Braceletes.
- Calça escura.
- Botas altas.
- Anatomia humana estilizada.
- Modelado detalhado.
- Mechas naturais.
- Silhueta coerente.
- Movimento corporal convincente.

A N3.5 estabelece a referência visual do personagem parado.

A N3.6 estabelece a referência visual da corrida.

A missão é consolidar essas conquistas e resolver os problemas necessários para permitir futuras animações de qualidade equivalente.

## Objetivos principais

1. Preservar integralmente a arte aprovada.
2. Investigar a sincronização da corrida com a velocidade real do jogo.
3. Corrigir ou documentar o deslizamento dos pés.
4. Avaliar contraste em cenários escuros.
5. Investigar a transição entre idle e corrida.
6. Preparar um pipeline reutilizável de extração e validação de animações.
7. Definir requisitos para salto, ataque e idle.
8. Validar tudo em um exemplo isolado no Bevy.
9. Produzir evidências reais.
10. Executar testes e publicar no GitHub.

**Não iniciar N4 ou N5.**

---

# 1. REGRAS INEGOCIÁVEIS

Não criar outro protagonista.

Não retornar ao manequim geométrico.

Não substituir a N3.5 por uma versão simplificada.

Não alterar arbitrariamente os seis quadros aprovados da N3.6.

Não alterar a câmera principal.

Não modificar a física de movimentação do jogo principal.

Não modificar o sistema de combate.

Não inventar quadros de animação.

Não usar interpolação de IA sem autorização.

Não substituir animação real por movimento procedural de blocos.

Não afirmar que houve pintura manual se ela não ocorreu.

Não confundir aprovação artística da corrida com aprovação para produção.

Não modificar arquivos em `Md-usario/`.

---

# 2. AUDITORIA INICIAL

Antes de modificar qualquer arquivo:

1. Confirmar branch atual.
2. Confirmar HEAD local.
3. Consultar HEAD remoto.
4. Inspecionar `git status`.
5. Identificar alterações preexistentes.
6. Ler `CLAUDE.md`.
7. Ler os documentos arquiteturais relevantes.
8. Ler a documentação N3.5.
9. Ler a documentação N3.6.
10. Inspecionar os scripts, manifestos, sprites e exemplos existentes.

Documentos especialmente importantes:

- `docs/development/n3_5/N3_5_RELATORIO_FINAL.md`
- `docs/development/n3_5/N3_5_AVALIACAO.md`
- `docs/development/n3_5/N3_5_CAMERA.md`
- `docs/development/n3_6/N3_6_RELATORIO_FINAL.md`
- `docs/development/n3_6/N3_6_ANALISE_CORRIDA.md`
- `docs/development/n3_6/N3_6_ALINHAMENTO.md`
- `docs/development/n3_6/N3_6_AVALIACAO.md`
- `docs/development/n3_6/N3_6_FUNDO_ESCURO.md`
- `docs/development/n3_6/N3_6_BEVY.md`

Não presumir que arquivos ou APIs possuem determinado nome sem verificar.

Se o HEAD mudou, reconciliar as alterações antes de prosseguir.

---

# 3. BASE ARTÍSTICA APROVADA

O proprietário aprovou visualmente os seguintes resultados:

## N3.5

- Fidelidade à ficha original.
- Cabelo prateado.
- Rosto em perfil.
- Roupas detalhadas.
- Método de redução que preserva modelado.

## N3.6

- Corrida de seis quadros.
- Alternância coerente dos membros.
- Movimento natural do cabelo.
- Consistência do corpo.
- Apresentação em câmera experimental 360.

Os seis quadros da corrida são derivados dos quadros do vídeo:

48, 50, 52, 54, 56 e 58.

A temporização original é de 12 FPS.

O ciclo dura 0,5 segundo.

O canvas é 112×128.

O pivô é `(56, 118)`.

A altura artística de referência é 106 texels.

A densidade experimental é 2 texels por unidade.

**Preservar essa versão como baseline imutável para comparação.**

Se forem criadas alternativas, utilizar novos arquivos.

---

# 4. PRIMEIRO PROBLEMA — VELOCIDADE DA CORRIDA

A N3.6 identificou uma incompatibilidade entre a corrida visual e a movimentação do jogo.

A velocidade de corrida do jogo foi documentada como:

`PLAYER_RUN_SPEED = 310 unidades/s`

A passada extraída do vídeo sugere uma velocidade visual aproximada de:

78 a 170 unidades/s.

Esses valores foram estimados a partir do movimento dos pés e da abertura da passada.

Não são uma calibração definitiva.

Se a animação for simplesmente reproduzida a 12 FPS enquanto o personagem se desloca a 310 unidades/s, poderá ocorrer deslizamento visual dos pés.

A N3.7 deve investigar esse problema.

**Não alterar `PLAYER_RUN_SPEED` para esconder a incompatibilidade.**

---

# 5. ESTUDO DE SINCRONIZAÇÃO

Criar um experimento que compare:

## Cenário A — Reprodução original

- Corrida a 12 FPS.
- Velocidade horizontal simulada de 310 unidades/s.
- Sem alteração da arte.

## Cenário B — Animação acelerada

- Aumentar a velocidade de reprodução.
- Manter a velocidade horizontal.
- Medir deslizamento e naturalidade.

## Cenário C — Velocidade visual reduzida

- Manter 12 FPS.
- Usar velocidade horizontal compatível com a passada estimada.
- Apenas no exemplo isolado.

## Cenário D — Ajuste moderado

- Combinar uma aceleração limitada do clipe com uma velocidade de deslocamento experimental.
- Evitar movimento artificialmente rápido.

## Cenário E — Adaptação artística futura

Avaliar se seria necessário criar uma corrida com passada maior.

Não gerar novos quadros automaticamente nesta etapa.

Registrar o custo artístico dessa alternativa.

---

# 6. MEDIÇÃO DO DESLIZAMENTO DOS PÉS

Não avaliar somente pela aparência.

Criar uma análise quantitativa aproximada.

Identificar em cada quadro:

- Pé de apoio.
- Pé livre.
- Posição da sola.
- Fase de contato.
- Fase de impulsão.
- Fase aérea.
- Deslocamento horizontal do pé.
- Deslocamento horizontal do corpo.

Utilizar coordenadas consistentes.

Estimar a velocidade de deslocamento necessária para que o pé de apoio permaneça aproximadamente estável no mundo.

Reconhecer limitações:

- O vídeo pode conter interpolação.
- A animação foi produzida correndo no lugar.
- Nem todos os quadros possuem contato.
- A redução introduz arredondamento.
- O movimento não foi criado originalmente com root motion.

Não apresentar as medições como biomecânica exata.

---

# 7. EXPERIMENTO VISUAL COMPARATIVO

Produzir uma comparação simultânea ou sequencial de:

- Corrida original a 12 FPS.
- Corrida acelerada.
- Corrida com deslocamento reduzido.
- Melhor alternativa experimental.

Usar o mesmo fundo.

Usar a mesma câmera.

Usar a mesma escala.

Usar os mesmos quadros.

Exibir:

- FPS efetivo.
- Velocidade horizontal.
- Fator de reprodução.
- Posição do personagem.
- Fase do ciclo.
- Estimativa de deslizamento.

Não permitir que a câmera acompanhe cada versão de forma diferente e prejudique a comparação.

---

# 8. NÃO ALTERAR A FÍSICA DO JOGO

A N3.7 não está autorizada a modificar:

- Velocidade máxima.
- Aceleração.
- Atrito.
- Gravidade.
- Pulo.
- Esquiva.
- Colisão.
- Estados do jogador.
- Sistema de input.

O problema deve ser estudado no sistema de apresentação visual.

Se a melhor solução exigir alterar a movimentação, registrar a proposta e aguardar autorização.

---

# 9. SEGUNDO PROBLEMA — FUNDO ESCURO

A N3.6 mostrou que:

- O cabelo continua legível.
- A pele continua legível.
- A camisa continua legível.
- A calça perde contraste.
- As botas perdem contraste.
- O problema é maior nas pernas traseiras.

Foi testada uma borda de luz fria discreta.

A N3.7 deve aprofundar essa avaliação.

Não aplicar um contorno branco em todo o personagem.

Não clarear toda a calça.

Não modificar a paleta aprovada sem comparação.

---

# 10. TESTES DE CONTRASTE

Comparar:

## Versão A

Arte original da N3.6.

## Versão B

Borda de luz fria discreta de 1 texel nas regiões escuras expostas.

## Versão C

Pequeno ajuste localizado de luminosidade nas botas.

## Versão D

Sem alteração no sprite, utilizando iluminação ou contraste do cenário experimental.

Todas as versões devem ser claramente identificadas.

Não sobrescrever os quadros aprovados.

Testar em:

- Fundo claro.
- Fundo cinza.
- Fundo quase preto.
- Fundo representativo do castelo.

Avaliar também a animação em movimento, não apenas quadros estáticos.

---

# 11. TERCEIRO PROBLEMA — TRANSIÇÃO IDLE → RUN

A N3.5 possui um candidato parado.

A N3.6 possui uma corrida.

Ainda não existe garantia de que a troca entre os dois seja visualmente natural.

Criar uma prova de transição no exemplo isolado.

Sequência:

1. Idle.
2. Início da corrida.
3. Corrida contínua.
4. Interrupção.
5. Retorno ao idle.

Não inventar quadros de antecipação.

Se não houver quadros adequados, usar troca direta e registrar o salto visual.

Investigar:

- Posição do quadril.
- Altura da cabeça.
- Volume do cabelo.
- Posição dos pés.
- Alinhamento do torso.
- Diferença de paleta.
- Diferença de acabamento.
- Mudança brusca de postura.

O objetivo é descobrir o que falta para uma transição convincente.

Não esconder defeitos com efeitos visuais.

---

# 12. ALINHAMENTO ENTRE N3.5 E N3.6

A N3.5 utiliza:

- Canvas 80×128.
- Pivô `(40, 118)`.

A N3.6 utiliza:

- Canvas 112×128.
- Pivô `(56, 118)`.

Ambos representam o mesmo personagem.

O sistema de apresentação deve respeitar o pivô lógico de cada fonte.

Não alinhar os sprites pelo canto superior esquerdo.

Não centralizar cada quadro com base na bounding box opaca.

Não alterar o pivô quadro a quadro para eliminar o balanço natural.

Testar a transição com guias de:

- Chão.
- Quadril.
- Cabeça.
- Centro corporal.

---

# 13. CÂMERA EXPERIMENTAL

Manter:

- Câmera 360 no exemplo isolado.
- Câmera 405 como comparação.
- `camera.ron` inalterado.

A câmera 360 continua sendo a referência preferida para avaliar fidelidade dos pixels.

Mas não está aprovada para produção.

Investigar o impacto do enquadramento em:

- Visibilidade do personagem.
- Área útil de exploração.
- Plataformas.
- Inimigos.
- Projéteis.
- HUD.
- Distância de leitura.
- Cenários internos.

Não alterar os sistemas correspondentes.

Documentar os riscos.

---

# 14. QUARTO PROBLEMA — PIPELINE REUTILIZÁVEL

A N3.6 extraiu quadros do vídeo utilizando GStreamer e processamento de imagem.

Esse processo funcionou para a corrida.

Agora precisamos transformá-lo em um pipeline experimental reutilizável.

O pipeline deve ser capaz de:

1. Receber um vídeo de referência.
2. Ler metadados.
3. Identificar FPS.
4. Identificar resolução.
5. Extrair quadros.
6. Preservar a fonte original.
7. Aplicar remoção de fundo quando apropriado.
8. Registrar a procedência de cada quadro.
9. Aplicar redução consistente.
10. Gerar PNGs transparentes.
11. Calcular bounding boxes.
12. Aplicar pivô lógico.
13. Produzir manifesto experimental.
14. Gerar comparação visual.
15. Gerar animação de prova.
16. Validar dimensões.
17. Registrar métricas.
18. Produzir evidências.

Não criar uma infraestrutura excessivamente complexa.

Priorizar ferramentas simples e reproduzíveis.

---

# 15. PROVENIÊNCIA OBRIGATÓRIA

Cada quadro deve registrar:

- Arquivo original.
- Hash da fonte.
- Índice do quadro.
- Timestamp.
- Método de extração.
- Método de remoção do fundo.
- Escala.
- Canvas.
- Pivô.
- Ajustes aplicados.
- Indicação de edição manual.
- Indicação de geração por IA.
- Versão do pipeline.

Não atribuir autoria manual a transformações automáticas.

Não afirmar que quadros foram gerados quando foram extraídos.

Não misturar fontes sem documentar.

---

# 16. PRESERVAÇÃO DA QUALIDADE ARTÍSTICA

O pipeline não deve destruir:

- Detalhes do rosto.
- Mechas do cabelo.
- Sombras da camisa.
- Dobras da calça.
- Volume das botas.
- Contornos originais.

Não impor paleta de 16 cores indiscriminadamente.

Não criar blocos geométricos sobre a imagem.

Não aplicar filtros que produzam halos fortes.

Não aceitar transparência com bordas verdes.

Não alterar cores sem registrar a transformação.

Comparar sempre com o padrão aprovado da N3.5 e da N3.6.

---

# 17. CONTRATO DE ANIMAÇÃO

Propor um contrato experimental reutilizável para animações futuras.

Campos possíveis:

- Identificador.
- Nome.
- Fonte.
- Quantidade de quadros.
- Duração total.
- Duração por quadro.
- Loop.
- Canvas.
- Pivô.
- Direção.
- Densidade.
- Escala.
- Bounding boxes.
- Eventos visuais.
- Proveniência.
- Status de aprovação.

Inspecionar o formato RON existente antes de propor mudanças.

Preferir reutilizar o contrato atual.

Não criar um novo sistema de animação paralelo sem necessidade.

Não modificar manifestos de produção.

---

# 18. PREPARAÇÃO DO SALTO

O salto será uma etapa futura.

A N3.6 identificou que as referências existentes não oferecem um ciclo lateral completo e consistente.

Preparar uma especificação para produzir uma referência melhor.

A referência ideal deve conter:

- Vista lateral direita.
- Mesmo personagem.
- Mesma roupa.
- Câmera fixa.
- Fundo uniforme.
- Escala constante.
- Salto normal, sem acrobacias.
- Preparação.
- Impulsão.
- Subida.
- Ápice.
- Queda.
- Aterrissagem.
- Recuperação.

O personagem deve permanecer totalmente dentro do enquadramento.

Não cortar cabelo, mãos ou botas.

Não adicionar objetos ou peças inexistentes.

**Não gerar o salto nesta etapa.**

---

# 19. PREPARAÇÃO DO ATAQUE

A N3.6 identificou que as referências existentes de ataque não possuem antecipação suficientemente consistente.

Preparar uma especificação de vídeo para ataque desarmado.

Exigir:

- Perfil lateral direito.
- Mesmo personagem.
- Mesma escala.
- Mesmo figurino.
- Guarda inicial.
- Antecipação.
- Execução.
- Impacto.
- Recuperação.
- Retorno à guarda.

Não incorporar efeitos de impacto permanentemente ao corpo.

Separar conceitualmente:

- Corpo.
- Mãos.
- Arma futura.
- Efeitos visuais.

Não implementar hitboxes.

Não modificar o combate.

Não gerar o ataque nesta etapa.

---

# 20. PREPARAÇÃO DO IDLE

A N3.5 continua sendo a referência principal do personagem parado.

Criar uma especificação para vídeo de idle lateral.

O vídeo deve preservar:

- Mesmo rosto.
- Mesmo cabelo.
- Mesma roupa.
- Mesma escala.
- Mesma orientação.
- Mesmo enquadramento.

Movimentos permitidos:

- Respiração sutil.
- Pequena movimentação dos ombros.
- Movimento discreto do cabelo.
- Transferência mínima de peso.

Não transformar o idle em uma pose de combate.

Não adicionar equipamentos.

Não gerar o idle nesta etapa.

---

# 21. BRIEFINGS PARA O PROPRIETÁRIO

Produzir três briefings independentes:

1. Vídeo de salto.
2. Vídeo de ataque.
3. Vídeo de idle.

Cada briefing deve conter:

- Descrição visual.
- Referência do protagonista.
- Orientação.
- Sequência temporal.
- Restrições de câmera.
- Restrições de anatomia.
- Restrições de cabelo.
- Restrições de roupa.
- Restrições de fundo.
- Duração sugerida.
- FPS sugerido.
- Critérios de aceitação.
- Defeitos que invalidam o vídeo.

Os briefings devem ser utilizáveis em uma ferramenta de geração de vídeo.

Não afirmar que foram executados.

Não enviar imagens ou arquivos a serviços externos sem autorização.

---

# 22. MODULARIDADE FUTURA

A N3.7 não implementará equipamentos modulares.

Mas deve documentar os requisitos necessários para N4.

Investigar:

- Separação do cabelo.
- Sobreposição da capa.
- Posição das mãos.
- Sockets de armas.
- Posição dos ombros.
- Posição da cabeça.
- Mudanças de orientação.
- Sincronização das camadas.
- Variação de largura da corrida.

Não separar artificialmente o corpo aprovado se isso danificar a arte.

Não criar sockets inventados.

Se forem estudadas posições de mãos, classificá-las como estimativas experimentais.

---

# 23. EXEMPLO ISOLADO NO BEVY

Reutilizar:

`examples/n3_visual_proof.rs`

Adicionar um modo experimental N3.7.

Funcionalidades desejadas:

- Corrida N3.6 original.
- Comparação de velocidades.
- Comparação de FPS.
- Idle N3.5.
- Transição idle/run.
- Fundo claro/escuro.
- Contorno experimental.
- Câmera 360/405.
- Pausa.
- Avanço quadro a quadro.
- Exibição de pivô.
- Exibição de velocidade.
- Exibição de métricas.

Reutilizar controles existentes quando possível.

Evitar conflitos.

Documentar as teclas novas.

Não quebrar as fontes N3 a N3.6.

---

# 24. EVIDÊNCIAS

Criar:

`docs/development/n3_7/evidence/`

Produzir, quando aplicável:

- `run_baseline.png`
- `run_speed_comparison.png`
- `foot_contact_analysis.png`
- `foot_sliding_metrics.png`
- `run_12fps.gif`
- `run_speed_variant.gif`
- `idle_run_transition.gif`
- `idle_run_alignment.png`
- `dark_original.png`
- `dark_rim_light.png`
- `dark_comparison.png`
- `camera_405.png`
- `camera_360.png`
- `pipeline_output.png`
- `bevy_n37.png`

Não criar arquivos fictícios.

GIFs devem conter movimento real.

Capturas devem corresponder à versão testada.

Registrar claramente quais imagens foram modificadas experimentalmente.

---

# 25. GATES DA N3.7

## Gate A — Preservação artística

- [ ] Identidade N3.5 preservada.
- [ ] Corrida N3.6 preservada.
- [ ] Rosto reconhecível.
- [ ] Cabelo consistente.
- [ ] Roupas consistentes.
- [ ] Nenhuma simplificação indevida.

## Gate B — Sincronização

- [ ] Deslizamento dos pés medido.
- [ ] Velocidade do jogo comparada.
- [ ] Alternativas testadas.
- [ ] Impacto visual documentado.
- [ ] Recomendação fundamentada.

## Gate C — Contraste

- [ ] Fundo claro testado.
- [ ] Fundo escuro testado.
- [ ] Contorno experimental testado.
- [ ] Arte original preservada.
- [ ] Comparação publicada.

## Gate D — Transição

- [ ] Idle e corrida alinhados.
- [ ] Troca de estado testada.
- [ ] Diferenças de proporção documentadas.
- [ ] Problemas de continuidade identificados.

## Gate E — Pipeline

- [ ] Extração reproduzível.
- [ ] Proveniência registrada.
- [ ] Escala consistente.
- [ ] Pivô consistente.
- [ ] Manifestos válidos.
- [ ] Testes automatizados.

## Gate F — Segurança

- [ ] Jogo principal inalterado.
- [ ] Câmera principal inalterada.
- [ ] N3.5 preservada.
- [ ] N3.6 preservada.
- [ ] Arquivos pessoais preservados.
- [ ] Testes aprovados.
- [ ] GitHub publicado.

**A aprovação artística final continua sendo do proprietário.**

---

# 26. TESTES

Executar:

`cargo fmt --check`

`cargo check`

`cargo clippy --all-targets --jobs 2 -- -D warnings`

`cargo test --jobs 2`

Executar o exemplo visual.

Adicionar testes para:

- Preservação dos seis quadros N3.6.
- Manifestos.
- Pivôs.
- Dimensões.
- Durações.
- Loop.
- Proveniência.
- Escala.
- Integridade dos recursos.
- Compatibilidade com N3.5.
- Compatibilidade com N3.6.

Não modificar testes antigos para esconder regressões.

Registrar quantidade real de testes.

---

# 27. DOCUMENTAÇÃO

Criar:

`docs/development/n3_7/`

Arquivos mínimos:

1. `N3_7_AUDITORIA.md`
2. `N3_7_BASELINE.md`
3. `N3_7_VELOCIDADE.md`
4. `N3_7_DESLIZAMENTO.md`
5. `N3_7_SINCRONIZACAO.md`
6. `N3_7_CONTRASTE.md`
7. `N3_7_TRANSICAO.md`
8. `N3_7_CAMERA.md`
9. `N3_7_PIPELINE.md`
10. `N3_7_PROVENIENCIA.md`
11. `N3_7_CONTRATO_ANIMACAO.md`
12. `N3_7_BRIEFING_SALTO.md`
13. `N3_7_BRIEFING_ATAQUE.md`
14. `N3_7_BRIEFING_IDLE.md`
15. `N3_7_BEVY.md`
16. `N3_7_TESTES.md`
17. `N3_7_AVALIACAO.md`
18. `N3_7_RELATORIO_FINAL.md`

Não criar documentos vazios.

Registrar resultados reais e limitações.

---

# 28. ARQUIVOS PROTEGIDOS

Não alterar:

- `Md-usario/`
- `white_mesh.glb`
- Blender V2.2 original.
- `assets/data/camera.ron`
- Save v3.
- Sistema de input.
- Sistema de combate.
- Sistema de magia.
- Física do jogador.
- Protagonista publicado.
- Manifestos de produção.
- Fontes artísticas N3.5.
- Quadros aprovados N3.6.
- Evidências anteriores.

Não iniciar N4 ou N5.

Não executar Git destrutivo.

Não fazer force push.

Não fazer merge.

Não fazer rebase automático.

---

# 29. PUBLICAÇÃO OBRIGATÓRIA

Publicar resultados reais na branch:

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
12. Confirmar que os originais não mudaram.

Sugestões de commits:

`feat(visual): add n3.7 run synchronization experiments`

`docs(n3): record n3.7 animation pipeline and evidence`

`docs(n3): record published n3.7 commits`

Não criar commits vazios.

Se houver impedimento técnico, publicar somente resultados efetivamente produzidos.

---

# 30. RELATÓRIO FINAL

Responder obrigatoriamente:

**A. Estado inicial:** branch, HEAD e working tree.

**B. Baseline:** arquivos artísticos preservados.

**C. Velocidade:** análise de 310 unidades/s.

**D. Passada:** medição dos pés.

**E. Deslizamento:** resultados.

**F. Alternativas:** comparação.

**G. Recomendação:** solução preferida.

**H. Contraste:** fundo escuro.

**I. Borda de luz:** resultado experimental.

**J. Transição:** idle para corrida.

**K. Alinhamento:** pivôs e chão.

**L. Câmera:** 360 versus 405.

**M. Pipeline:** funcionamento.

**N. Proveniência:** rastreabilidade.

**O. Contrato:** animações futuras.

**P. Salto:** briefing.

**Q. Ataque:** briefing.

**R. Idle:** briefing.

**S. Modularidade:** riscos.

**T. Bevy:** prova isolada.

**U. Evidências:** arquivos produzidos.

**V. Testes:** resultados.

**W. Segurança:** integridade dos originais.

**X. GitHub:** commits e SHA remoto.

**Y. Limitações:** problemas não resolvidos.

**Z. Próxima decisão:** gates que dependem do proprietário.

---

# 31. REGRA FINAL

O protagonista e a corrida finalmente atingiram uma direção artística aprovada.

**Não sacrificar essa qualidade para acelerar o desenvolvimento.**

Não criar outro personagem.

Não redesenhar os seis quadros aprovados.

Não modificar a física do jogo para compensar o vídeo.

Não aplicar alterações visuais permanentes sem comparação.

Não substituir os sprites de produção.

Não iniciar N4 ou N5.

A N3.7 deve entregar:

1. Uma análise confiável da sincronização da corrida.
2. Uma prova visual comparativa.
3. Um estudo de contraste em fundo escuro.
4. Uma transição experimental entre idle e corrida.
5. Um pipeline documentado e reproduzível.
6. Três briefings para as próximas animações.
7. Testes e evidências.
8. Commits publicados no GitHub.

**Execute a N3.7 até o limite tecnicamente justificável, publique os resultados reais e aguarde a avaliação do proprietário antes de promover qualquer recurso ao jogo principal.**