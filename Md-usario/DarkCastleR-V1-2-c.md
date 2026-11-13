# DARK CASTLE RPG — MEGA PROMPT V1.2-C
## Reconstrução artística do protagonista — Modelo 3D v003
### Rust + Bevy 0.20.1 | Blender 5.2.4 LTS | Claude Code + MCP

---

# 0. MISSÃO

Você atuará como diretor de arte técnico, modelador 3D, especialista em Blender Python, artista de personagens e revisor de qualidade visual do projeto Dark Castle RPG.

Sua missão é reconstruir substancialmente o protagonista 3D experimental v002, criando um novo modelo v003 com qualidade artística significativamente superior.

**Não quero apenas melhorar o manequim existente. Quero evoluir a abordagem de modelagem.**

O modelo v002 demonstrou que o pipeline funciona, mas apresenta limitações visuais importantes:

- Cabeça sem anatomia facial convincente.
- Cabelo semelhante a uma boina.
- Tronco cilíndrico.
- Braços formados por cilindros.
- Articulações esféricas.
- Mãos sem dedos.
- Pernas retangulares.
- Botas em formato de caixas.
- Casaca semelhante a um cone fechado.
- Meia-capa semelhante a um painel rígido.
- Espada muito fina para leitura em baixa resolução.
- Materiais simples e pouca diferenciação entre superfícies.

Esses problemas não serão considerados resolvidos apenas com subdivisões, bevels ou suavização de normais.

Precisamos reconstruir as formas fundamentais.

**Esta missão é exclusivamente artística e técnica. Não autoriza mudanças no gameplay.**

---

# 1. CONTEXTO DO PROJETO

O Dark Castle RPG é um RPG de ação 2D de fantasia sombria, desenvolvido com:

- Rust 2024.
- Bevy 0.20.1.
- Blender 5.2.4 LTS.
- Python e `bpy`.
- Claude Code.
- MCP for Blender.

A Fase 4 do jogo está concluída.

O último commit de referência informado foi:

`e396c31`

Antes de iniciar, confirme o estado atual do repositório. Não assuma que ele permanece idêntico.

As etapas anteriores produziram:

- V1.1 — Direção artística.
- V1.2 — Contrato de assets.
- V1.2-B — Primeiro modelo 3D real, v002.

A V1.2-B está aprovada como validação técnica.

O modelo v002 NÃO está aprovado como arte final.

A V1.3 permanece bloqueada.

---

# 2. DOCUMENTOS E ARQUIVOS OBRIGATÓRIOS

Antes de criar qualquer geometria, leia:

1. `docs/ART_DIRECTION.md`
2. `docs/ASSET_CONTRACT.md`
3. `art-src/README.md`
4. `art-src/player/references/README.md`
5. Todos os scripts relevantes em `art-src/player/scripts/bpy/`
6. Todos os scripts relevantes em `art-src/player/scripts/post/`
7. A documentação existente de câmera, personagem e animações.
8. O estado atual do Git.

Localize o modelo v002 e seus renders.

A folha de referência do modelo anterior foi informada em:

`art-src/player/comparisons/v1_2b_swordsman_base_v002/model_sheet_v002.png`

A imagem de inspiração foi encontrada anteriormente em:

`art-src/player/inspiration/protagonist_concept_v001.png`

Confirme os caminhos reais antes de utilizá-los.

Não invente arquivos ausentes.

Não sobrescreva o modelo v002.

---

# 3. DIREÇÃO ARTÍSTICA OBRIGATÓRIA

## Conceito B — Espadachim aristocrático sombrio

O personagem deve transmitir:

- Elegância.
- Mistério.
- Periculosidade.
- Disciplina.
- Agilidade.
- Nobreza decadente.
- Fantasia gótica.
- Experiência em combate com espada.

Não deve parecer um cavaleiro pesado, um soldado genérico ou um personagem de ficção científica.

## 3.1 Proporções

Manter como referência aproximadamente sete cabeças de altura.

Criar uma anatomia estilizada e coerente:

- Cabeça proporcional.
- Pescoço definido.
- Ombros naturais.
- Tórax com volume.
- Cintura mais estreita.
- Quadril proporcional.
- Braços com anatomia.
- Cotovelos e punhos definidos.
- Pernas com coxas, joelhos e panturrilhas.
- Pés com formato anatômico.

Não exagerar a musculatura.

O personagem deve parecer ágil, não fisicamente pesado.

## 3.2 Identidade visual

Manter:

- Cabelo curto cinza-ardósia.
- Casaca longa aberta.
- Forro vinho.
- Gola alta.
- Meia-capa assimétrica curta.
- Luvas.
- Botas altas.
- Espada longa, fina e levemente curva.
- Paleta escura com acentos âmbar.

Evitar:

- Capa longa até o chão.
- Capuz cobrindo toda a cabeça.
- Armadura pesada.
- Cabelo longo prateado.
- Ornamentos dourados excessivos.
- Referências diretas a personagens protegidos por direitos autorais.

A referência visual é inspiração, não uma instrução para copiar todos os detalhes.

---

# 4. MUDANÇA FUNDAMENTAL NA MODELAGEM

O v002 foi construído principalmente com primitivas geométricas.

Para o v003, quero uma abordagem de modelagem mais avançada.

Utilize, conforme apropriado:

- Malhas editáveis.
- Geometria procedural com vértices e faces.
- Perfis anatômicos.
- Extrusão de seções.
- Curvas convertidas em malha.
- Modificadores de espessura.
- Bevel controlado.
- Subdivision Surface quando necessário.
- Normais suavizadas.
- Modelagem de tecidos.
- Malhas separadas para roupas e acessórios.

Primitivas podem continuar sendo utilizadas como elementos auxiliares, mas não devem definir a aparência final do corpo.

Não aumente polígonos sem melhorar a silhueta.

Não adicione complexidade que não sobreviva à conversão para sprite.

A prioridade é qualidade visual, não contagem de triângulos.

---

# 5. RECONSTRUÇÃO DA CABEÇA

Criar uma cabeça estilizada com:

- Crânio proporcional.
- Testa.
- Têmporas.
- Maçãs do rosto.
- Mandíbula.
- Queixo.
- Nariz.
- Orelhas.
- Olhos.
- Sobrancelhas.

A anatomia facial não precisa ser fotorealista.

Entretanto, não pode parecer uma esfera com dois pontos.

O rosto deve transmitir personalidade.

Priorizar uma expressão neutra, séria e determinada.

Não utilizar texturas faciais externas sem autorização.

---

# 6. RECONSTRUÇÃO DO CABELO

O cabelo atual parece uma boina.

Isso deve ser corrigido.

Criar cabelo curto cinza-ardósia, formado por volumes e mechas com direções diferentes.

Requisitos:

- Silhueta irregular.
- Mechas sobre a testa.
- Volume natural nas laterais.
- Região traseira coerente.
- Separação visível entre cabelo e rosto.
- Ausência de uma única massa achatada semelhante a um boné.

Usar geometria de mechas estilizadas, não milhares de fios individuais.

Avaliar o cabelo nas vistas frontal, lateral e traseira.

---

# 7. RECONSTRUÇÃO DO CORPO

## Tronco

Substituir o cilindro por uma malha com:

- Peitoral.
- Caixa torácica.
- Abdômen.
- Cintura.
- Transição para o quadril.

## Braços

Modelar:

- Ombros.
- Braços superiores.
- Cotovelos.
- Antebraços.
- Punhos.

Evitar articulações esféricas aparentes.

## Mãos

Criar mãos estilizadas com:

- Palma.
- Polegar.
- Dedos simplificados.
- Luvas.

Os dedos podem ser simplificados para manter a legibilidade, mas as mãos não devem parecer esferas.

## Pernas

Criar:

- Coxas.
- Joelhos.
- Panturrilhas.
- Tornozelos.

Evitar pernas retangulares.

## Botas

Modelar botas com:

- Cano.
- Tornozelo.
- Peito do pé.
- Biqueira.
- Sola.
- Salto discreto.

As botas devem ser reconhecíveis em perfil.

---

# 8. CASACA ARISTOCRÁTICA

A casaca é uma das peças mais importantes do design.

Ela deve parecer uma roupa de verdade.

Requisitos:

- Abertura frontal visível.
- Dois painéis dianteiros independentes.
- Forro vinho aparente.
- Gola alta com volume.
- Ombros ajustados.
- Cintura definida.
- Caudas traseiras com separação.
- Espessura perceptível nas bordas.
- Dobras estilizadas.

Evitar:

- Casaca em forma de cone.
- Superfícies totalmente rígidas.
- Painéis atravessando o corpo.
- Interseções evidentes.
- Aparência de armadura tubular.

O tecido deve parecer pesado, mas flexível.

A silhueta lateral precisa ser particularmente bem resolvida, porque o jogo é 2D.

---

# 9. MEIA-CAPA ASSIMÉTRICA

A meia-capa v002 parece um painel de saia.

Reconstruí-la.

Ela deve:

- Estar presa a um ombro.
- Ter ponto de fixação visível.
- Cobrir parcialmente as costas.
- Ter comprimento curto ou médio.
- Apresentar caimento natural.
- Possuir alguma assimetria.
- Ser visível na vista lateral.
- Não esconder a espada.
- Não cobrir completamente a casaca.

Criar espessura discreta e dobras principais.

Não precisa executar simulação de tecido nesta etapa.

A geometria deve permitir animação futura, mas não é necessário implementar física.

---

# 10. ESPADA

A espada é essencial para identificar o protagonista.

Manter:

- Lâmina longa.
- Curvatura discreta.
- Aparência elegante.
- Guarda simples.
- Cabo proporcional.
- Acentos metálicos.

Melhorar:

- Espessura aparente.
- Perfil da lâmina.
- Ponta.
- Guarda.
- Empunhadura.
- Leitura contra fundo escuro.

A espada não deve parecer uma linha branca desconectada.

Criar uma silhueta que sobreviva à redução para aproximadamente 52 pixels de altura do personagem.

Se necessário, produzir uma variante de renderização com espessura visual ligeiramente exagerada.

Não alterar hitboxes ou alcance do jogo.

---

# 11. MATERIAIS E ILUMINAÇÃO

Criar materiais estilizados diferenciando:

- Pele.
- Cabelo.
- Tecido da casaca.
- Forro vinho.
- Meia-capa.
- Couro.
- Metal.
- Espada.
- Botas.
- Luvas.

Evitar que todas as superfícies tenham o mesmo brilho.

Usar valores coerentes de roughness e metallic.

Não utilizar texturas externas sem autorização.

A iluminação deve funcionar nas vistas:

- Frontal.
- Lateral.
- Traseira.
- Três quartos.

Não deixar a vista traseira significativamente mais escura apenas porque a luz principal favorece o perfil.

Testar o personagem também sobre fundo escuro.

---

# 12. ORGANIZAÇÃO DOS SCRIPTS

Reutilize a infraestrutura existente.

Não destrua nem substitua os scripts da V1.2-B sem necessidade.

Separe responsabilidades:

- Geometria corporal.
- Cabeça e cabelo.
- Roupas.
- Espada.
- Materiais.
- Iluminação.
- Câmeras.
- Renderização.
- Exportação.
- Validação.

Pode criar novos módulos Python quando necessário.

Evite um único script gigantesco.

Preserve a capacidade de reproduzir o modelo v002.

O modelo v003 deve ter um ponto de entrada claramente identificado.

---

# 13. BLENDER + MCP

O MCP já foi validado com sucesso para leitura da cena.

Mantenha:

`BLENDER_MCP_SAFE_MODE=1`

Não desative o modo de segurança.

Utilize o MCP para:

- Inspecionar cenas.
- Consultar objetos.
- Verificar materiais.
- Conferir propriedades.
- Realizar operações permitidas em cenas experimentais.

Para executar scripts completos, pode utilizar:

`blender --background --factory-startup --python ...`

Execute somente scripts previamente revisados.

Não altere a cena aberta do usuário.

Não utilize serviços externos de geração 3D.

---

# 14. ARQUIVOS E VERSIONAMENTO

Criar uma nova versão identificada como v003.

Sugestão de organização:

`art-src/player/renders/v003/`

`art-src/player/comparisons/v1_2c_swordsman_v003/`

Preservar os arquivos anteriores.

Não sobrescrever:

- Modelo v001.
- Modelo v002.
- Renders anteriores.
- Comparações anteriores.

Salvar o modelo em formato `.blend`.

Registrar a versão real do Blender utilizada.

Registrar os parâmetros de geração.

Não criar commits automaticamente.

---

# 15. ENTREGÁVEIS VISUAIS

Gerar obrigatoriamente:

1. Vista frontal ortográfica.
2. Vista lateral ortográfica.
3. Vista traseira ortográfica.
4. Vista em três quartos.
5. Close do rosto.
6. Close do cabelo.
7. Close da casaca.
8. Close da meia-capa.
9. Close das botas.
10. Close da espada.

Criar também uma folha de apresentação do v003.

A folha deve mostrar claramente o personagem inteiro e os principais detalhes.

Não utilizar imagens provisórias do v001 no lugar dos renders reais.

---

# 16. COMPARAÇÃO V002 × V003

Criar uma comparação lado a lado.

Usar:

- Mesma orientação.
- Escala equivalente.
- Fundo equivalente.
- Iluminação comparável.

Avaliar:

- Anatomia.
- Silhueta.
- Cabelo.
- Roupas.
- Mãos.
- Botas.
- Espada.
- Qualidade geral.

Identificar o que realmente melhorou.

Não declarar melhorias apenas com base na contagem de polígonos.

---

# 17. TESTES EM ESCALA DE JOGO

Renderizar o personagem v003 em tamanho adequado ao jogo.

Comparar:

- 1:1.
- Zoom ×1,5.
- Zoom ×2.

Testar:

- Pixel art.
- Alta resolução.
- Renderização estilizada com cores chapadas.

A terceira alternativa é uma experiência adicional, não uma escolha definitiva.

Verificar:

- Legibilidade do rosto.
- Separação entre corpo e casaca.
- Visibilidade da meia-capa.
- Espessura da espada.
- Contraste com cenário escuro.
- Artefatos de transparência.
- Qualidade das bordas.

Não modificar a câmera do jogo.

Não escolher definitivamente D2 ou D3.

---

# 18. PREPARAÇÃO PARA ANIMAÇÃO

Não criar animações ainda.

Entretanto, avaliar se a geometria permite:

- Movimento dos braços.
- Flexão dos joelhos.
- Movimento da cabeça.
- Rotação do tronco.
- Movimento da casaca.
- Movimento da meia-capa.
- Ataques com espada.
- Corrida.
- Esquiva.
- Salto.

Não exigir uma malha única sem justificativa.

Malhas separadas podem ser vinculadas a uma armature.

Avaliar as vantagens e limitações das alternativas.

Entregar uma proposta de rig, sem implementá-lo nesta etapa.

---

# 19. CRITÉRIOS DE ACEITAÇÃO

O v003 somente poderá ser considerado candidato à aprovação se:

1. A cabeça não parecer uma esfera genérica.
2. O cabelo não parecer uma boina.
3. O tronco apresentar anatomia estilizada.
4. Os braços não parecerem tubos articulados por esferas.
5. As pernas tiverem proporções coerentes.
6. As botas tiverem formato reconhecível.
7. A casaca apresentar abertura frontal real.
8. A meia-capa parecer tecido preso ao ombro.
9. A espada tiver uma silhueta legível.
10. A vista lateral funcionar visualmente.
11. Os materiais apresentarem diferenciação.
12. As vistas ortográficas forem consistentes.
13. O modelo puder ser reproduzido pelos scripts.
14. Os arquivos anteriores permanecerem preservados.
15. Nenhum arquivo de gameplay for alterado.

Se algum critério não for atendido, registre a falha explicitamente.

Não declare aprovação artística em meu nome.

---

# 20. VALIDAÇÃO TÉCNICA

Ao concluir:

- Verifique os arquivos gerados.
- Confirme que os PNGs são válidos.
- Confirme as dimensões.
- Confirme o canal alfa.
- Confirme que o `.blend` abre.
- Confirme que os scripts executam no Blender real.
- Verifique erros e avisos.
- Execute `git diff --check`.
- Confira os arquivos modificados.
- Confirme que o gameplay está intacto.
- Confirme que os saves estão intactos.
- Confirme que nenhum commit foi criado.
- Confirme que nenhum push foi realizado.

Não executar:

- `cargo clean`
- `git clean`
- `git reset --hard`

Não modificar arquivos de `Md-usario/`.

Não modificar `target/verify/` ou `target/f4-stage/`.

Não sobrescrever arquivos protegidos.

---

# 21. RELATÓRIO FINAL

Entregar um relatório detalhado contendo:

## A. Estado inicial
Commit, branch e Git status.

## B. Arquitetura de modelagem
Como o v003 foi construído e quais técnicas foram utilizadas.

## C. Reconstrução anatômica
Cabeça, corpo, braços, mãos, pernas e botas.

## D. Roupas
Casaca, gola, forro e meia-capa.

## E. Espada
Geometria, materiais e legibilidade.

## F. Materiais
Paleta, iluminação e superfícies.

## G. Blender e MCP
Versões, scripts executados e limitações.

## H. Renders
Lista completa de imagens e caminhos.

## I. Comparação v002 × v003
Melhorias verificáveis e problemas restantes.

## J. Testes em escala de jogo
Pixel art, alta resolução e terceira alternativa.

## K. Preparação para rig
Recomendação técnica para a próxima etapa.

## L. Arquivos
Criados, modificados e preservados.

## M. Validações
Resultados técnicos e integridade do projeto.

## N. Pendências
Tudo que ainda depende de aprovação.

---

# 22. REGRA FINAL

Esta missão é a V1.2-C.

Não iniciar V1.3.

Não implementar animações.

Não alterar gameplay.

Não alterar câmera do jogo.

Não escolher D2 ou D3 definitivamente.

Não criar commits.

Não fazer push.

Não utilizar geração 3D externa.

**O objetivo é entregar um protagonista v003 visualmente convincente, reproduzível e pronto para avaliação humana — não simplesmente um manequim mais subdividido.**

Se a qualidade desejada não puder ser alcançada com os recursos disponíveis, explique objetivamente a limitação e proponha alternativas, sem apresentar um resultado insuficiente como definitivo.

Ao terminar, aguarde minha avaliação.