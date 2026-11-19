# DARK CASTLE RPG — MEGA PROMPT N3.4
# RECONSTRUÇÃO ARTÍSTICA FIEL DO PROTAGONISTA ORIGINAL
# PRIMEIRO A IDENTIDADE, DEPOIS A ANIMAÇÃO

**Projeto:** Dark Castle RPG  
**Etapa:** N3.4 — Reconstrução artística do protagonista-base  
**Branch:** `art/v1.2-protagonist`  
**HEAD remoto inicial de referência:** `8a39ab1a0d4401e97a75fdacb105eea784ef1ed0`  
**Estado artístico:** NÃO APROVADO  
**N4 e N5:** BLOQUEADAS

---

# 0. MISSÃO

Você atuará como artista técnico, diretor de arte, especialista em pixel art, Blender e integração Bevy.

O objetivo é reproduzir fielmente a identidade visual do protagonista que o proprietário já definiu.

**Não invente outro protagonista.**

A imagem-base original já existe no repositório:

`Md-usario/assets-user/prota/base/image.png`

A ficha modular também já existe:

`Md-usario/assets-user/prota/base/Ficha_Modular_do_Protagonista_Gotico.png`

Há ainda uma malha 3D:

`Md-usario/assets-user/prota/base/white_mesh.glb`

Esses três arquivos devem ser analisados antes de qualquer decisão artística.

O proprietário também forneceu uma referência visual do protagonista equipado em pixel art. Caso ela não esteja disponível no repositório, não finja tê-la encontrado. Use somente os arquivos efetivamente acessíveis e solicite a referência adicional se ela for indispensável.

**A missão da N3.4 é criar uma prova visual do personagem-base original, não uma interpretação genérica de um guerreiro gótico.**

---

# 1. LIÇÕES DAS ETAPAS ANTERIORES

## N3

O pipeline técnico funcionou, mas o protagonista era uma construção geométrica.

## N3.1

O Blender produziu renders do manequim V2.2, mas o rosto desapareceu na redução e os materiais não ficaram suficientemente diferenciados.

## N3.2

A IA produziu conceitos góticos grandes, mas a redução para aproximadamente 100 pixels destruiu detalhes.

As poses geradas independentemente não formaram uma animação consistente.

## N3.3

Foi produzido um quadro nativo de 106 pixels, em canvas 80×128, com sola em `(40, 118)`.

A espada e a capa ficaram separadas.

Porém:

- O corpo continuou parecendo um gabarito de blocos.
- O rosto não ficou legível.
- O casaco parecia um trapézio.
- Não houve pintura no Pixelorama.
- Não houve ataque.
- O Gate A foi reprovado.
- O Gate B não foi tentado.
- O Gate C ficou parcial.

**A N3.4 não deve repetir essas falhas.**

---

# 2. AUDITORIA OBRIGATÓRIA DOS ARQUIVOS ORIGINAIS

Antes de modificar qualquer coisa, abrir e inspecionar visualmente:

`Md-usario/assets-user/prota/base/image.png`

`Md-usario/assets-user/prota/base/Ficha_Modular_do_Protagonista_Gotico.png`

Inspecionar tecnicamente:

`Md-usario/assets-user/prota/base/white_mesh.glb`

Não basta listar os nomes.

Não basta confirmar dimensões.

Não basta calcular hashes.

É obrigatório visualizar as imagens e descrever o que realmente aparece nelas.

Registrar:

- Dimensões.
- Orientação.
- Proporções.
- Cabelo.
- Rosto.
- Roupa-base.
- Botas.
- Luvas.
- Cores.
- Silhueta.
- Vista lateral disponível.
- Diferenças entre base e personagem equipado.

Se o ambiente não permitir visualizar `image.png`, interromper a etapa artística.

Não substituir a referência por uma descrição inventada.

---

# 3. HIERARQUIA DAS REFERÊNCIAS

Estabelecer a seguinte prioridade:

**Prioridade 1 — Identidade do protagonista**

`image.png`

É a referência principal do personagem sem equipamentos removíveis.

**Prioridade 2 — Equipamentos e combinações**

`Ficha_Modular_do_Protagonista_Gotico.png`

Define a aparência e a intenção modular das peças.

**Prioridade 3 — Referências adicionais**

Imagens do protagonista equipado, se realmente disponíveis.

**Prioridade 4 — Estrutura técnica existente**

N3, N3.1 e N3.3.

Esses experimentos fornecem soluções técnicas, mas não substituem a direção artística.

Não usar o caçador de cabelos escuros da N3.2 como identidade do protagonista.

---

# 4. IDENTIDADE VISUAL OBRIGATÓRIA

A direção já definida pelo proprietário é:

**Guerreiro gótico aristocrático de cabelos longos branco-prateados, aparência séria, proporções elegantes e equipamentos medievais de fantasia sombria.**

A base deve respeitar a referência original.

Características importantes:

- Cabelos longos branco-prateados.
- Rosto masculino estilizado, sério e reconhecível.
- Anatomia humana coerente.
- Roupa-base escura.
- Calças escuras.
- Botas.
- Braceletes ou luvas, conforme referência.
- Silhueta elegante.
- Identidade visual consistente.

Não transformar o personagem em:

- Guerreiro genérico.
- Cavaleiro de cabelos curtos.
- Personagem de capuz permanente.
- Manequim de blocos.
- Boneco de cápsulas.
- Cópia direta de personagem de outra franquia.

A identidade facial e o cabelo são prioritários.

---

# 5. SEPARAR BASE DE EQUIPAMENTOS

A N3.4 deve distinguir:

## Personagem-base

O que permanece quando os equipamentos removíveis são retirados.

## Equipamentos

Peças que podem ser colocadas ou retiradas.

A ficha modular apresenta categorias como:

- Casaca.
- Armadura.
- Calça.
- Botas.
- Capa.
- Capuz ou elmo.
- Espada.
- Escudo opcional.

Não assumir que toda peça ilustrada já possui geometria ou sprite modular funcional.

Não desenhar a armadura completa sobre o corpo-base e depois afirmar que a armadura é removível.

Nesta etapa, o foco é o personagem-base.

A implementação completa de equipamentos permanece fora do escopo.

---

# 6. AVALIAÇÃO DA RESOLUÇÃO

A N3.3 utilizou:

- Canvas 80×128.
- Altura opaca 106 px.
- Sola `(40, 118)`.

Esses valores são o contrato inicial do experimento.

Entretanto, a identidade original inclui cabelo longo, rosto e proporções relativamente realistas.

Antes de desenhar, verificar se essa identidade é representável com qualidade suficiente nessa resolução.

Criar uma análise comparativa entre:

- 80×128.
- Uma resolução experimental intermediária, se necessária.
- A resolução equivalente de exibição no jogo.

Não alterar silenciosamente a resolução do contrato N3.3.

Se a identidade não couber adequadamente em 80×128, documentar o motivo e apresentar a alternativa como proposta, não como decisão aprovada.

Não aumentar a imagem apenas para aparentar mais detalhes.

---

# 7. PRIMEIRO OBJETIVO: ROSTO E CABELO

O principal problema da N3.3 foi a ausência de um rosto convincente.

A N3.4 deve começar por um estudo do rosto em perfil.

Prioridades:

1. Formato da testa.
2. Nariz.
3. Queixo.
4. Região dos olhos.
5. Linha da mandíbula.
6. Separação entre pele e cabelo.
7. Mechas branco-prateadas.
8. Volume do cabelo.
9. Direção do olhar.
10. Expressão séria.

O rosto não precisa ter realismo fotográfico.

Precisa ser reconhecível como uma adaptação do personagem original.

Não criar apenas um retângulo de cabelo com um ponto de pele.

Não substituir o rosto por capuz.

Não esconder o problema atrás de uma armadura.

---

# 8. CABELO BRANCO-PRATEADO

O cabelo é uma das características mais importantes do protagonista.

Ele deve:

- Ser longo.
- Possuir volume.
- Ter mechas reconhecíveis.
- Usar sombras frias.
- Ser distinguível da pele.
- Manter o perfil da referência.
- Funcionar em fundo claro e escuro.

Evitar branco puro em toda a massa.

Utilizar agrupamentos de pixels para sugerir mechas.

Não usar ruído aleatório para simular cabelo.

Não deixar o cabelo parecer um capacete.

---

# 9. CORPO E ROUPA-BASE

Depois do rosto, adaptar o corpo.

Respeitar as proporções da imagem original.

Criar:

- Ombros.
- Torso.
- Braços.
- Mãos.
- Quadril.
- Pernas.
- Botas.

Evitar formas retangulares excessivas.

A roupa deve apresentar:

- Dobras.
- Costuras principais.
- Separação entre torso e calça.
- Volume dos joelhos.
- Botas com formato reconhecível.
- Silhueta anatômica coerente.

Não adicionar filigrana em excesso.

O objetivo é uma base visualmente convincente, não uma reprodução impossível de todos os detalhes da ilustração.

---

# 10. PROCESSO DE CRIAÇÃO ARTÍSTICA

Priorizar edição visual real.

Investigar:

- Pixelorama.
- Aseprite, caso esteja legitimamente disponível.
- Krita.
- GIMP.
- Blender como apoio.
- Outras ferramentas gráficas existentes.

Não instalar programas indiscriminadamente.

Não utilizar ferramentas pagas sem autorização.

Se o Pixelorama abrir somente em modo headless, registrar isso.

Não afirmar que houve pintura manual quando não houve.

Scripts podem auxiliar na exportação, mas não devem criar automaticamente um personagem de blocos e apresentá-lo como arte final.

Se nenhuma ferramenta disponível permitir a criação de arte com qualidade suficiente, parar e informar o bloqueio.

---

# 11. USO DE IA

A IA pode auxiliar na criação de uma referência mais próxima da resolução final.

Entretanto:

- Não gerar outro personagem.
- Não mudar a cor do cabelo.
- Não alterar o rosto arbitrariamente.
- Não substituir a roupa-base.
- Não adicionar equipamentos permanentes.
- Não gerar poses independentes para formar animação.

Se utilizar geração de imagens, fornecer `image.png` como referência visual real, desde que o serviço esteja disponível e o envio seja autorizado.

Não enviar arquivos privados a serviços externos sem autorização.

Se a ferramenta não aceitar referências, não fingir fidelidade.

Registrar as limitações.

---

# 12. UM ÚNICO QUADRO DE QUALIDADE

O principal entregável artístico é:

**Uma pose estática lateral do protagonista-base.**

Requisitos:

- Cabelo branco-prateado longo.
- Rosto legível.
- Proporções coerentes.
- Roupa-base fiel.
- Botas legíveis.
- Silhueta convincente.
- Materiais diferenciados.
- Transparência correta.
- Pivô documentado.

Não criar o ataque antes de validar esse quadro.

Não produzir dezenas de sprites de baixa qualidade.

Não substituir qualidade por quantidade.

---

# 13. COMPARAÇÃO OBRIGATÓRIA

Criar uma folha comparativa contendo:

1. `image.png` original.
2. Recorte da vista lateral da ficha modular, se existente.
3. Sprite N3.3.
4. Nova proposta N3.4.
5. Close do rosto.
6. Silhueta.
7. Exibição na escala do jogo.

A comparação deve permitir responder:

**O novo sprite ainda parece o protagonista original?**

Se a resposta não for convincente, o Gate A permanece reprovado.

---

# 14. PRESERVAÇÃO DO PIPELINE

Não modificar:

- `camera.ron`.
- Save v3.
- Controles N2.
- Protagonista publicado.
- Manifestos de produção.
- Rig Blender original.
- `white_mesh.glb`.
- Arquivos pessoais.

Não alterar o jogo principal.

Criar experimentos em:

`art-src/n3-4-original-protagonist/`

Criar documentação em:

`docs/development/n3_4/`

A N3.4 pode reutilizar o exemplo visual isolado N3.

---

# 15. EQUIPAMENTOS

A N3.4 não precisa implementar o sistema modular completo.

Entretanto, deve preservar a possibilidade de equipamentos futuros.

A base deve ser desenhada sem incorporar permanentemente:

- Casaca removível.
- Armadura removível.
- Capa.
- Espada.
- Escudo.
- Elmo.

Quando necessário, utilizar camadas de referência para verificar encaixe.

Não declarar que uma peça está implementada se existe apenas como ilustração.

---

# 16. PROVA VISUAL NO BEVY

Se o quadro-base estiver pronto, carregá-lo no exemplo isolado.

Exibir:

- Personagem-base.
- Fundo neutro.
- Fundo escuro.
- Escala nativa.
- Ampliação nearest-neighbor.
- Comparação com N3.3.

Não alterar o jogo principal.

Não adicionar uma espada à base apenas para deixar a imagem mais bonita.

A prova principal é o protagonista sem equipamentos.

---

# 17. GATES

## Gate A — Fidelidade artística

- [ ] `image.png` foi visualmente inspecionado.
- [ ] Identidade preservada.
- [ ] Cabelo branco-prateado longo.
- [ ] Rosto reconhecível.
- [ ] Corpo proporcional.
- [ ] Roupa-base coerente.
- [ ] Sprite não parece manequim.
- [ ] Arte legível na escala do jogo.
- [ ] Comparação visual publicada.

**Esse gate exige avaliação do proprietário.**

## Gate B — Preparação técnica

- [ ] Canvas documentado.
- [ ] Pivô documentado.
- [ ] Transparência correta.
- [ ] Arquivo-fonte editável.
- [ ] Paleta documentada.
- [ ] Ausência de elementos removíveis incorporados indevidamente.

## Gate C — Integração isolada

- [ ] Sprite carregado no Bevy.
- [ ] Escala correta.
- [ ] Pivô correto.
- [ ] Sem regressões.
- [ ] Jogo principal intacto.

Aprovação técnica não equivale à aprovação artística.

---

# 18. EVIDÊNCIAS

Produzir, quando efetivamente disponíveis:

- `original_reference.png`
- `reference_analysis.png`
- `face_study.png`
- `hair_study.png`
- `body_base.png`
- `body_comparison.png`
- `silhouette_comparison.png`
- `native_scale.png`
- `nearest_preview.png`
- `dark_background.png`
- `bevy_n34.png`

Não fabricar evidências.

Não declarar edição manual se a arte foi gerada por script.

Não publicar imagens de comparação enganadoras.

---

# 19. TESTES

Executar, quando houver alterações aplicáveis:

`cargo fmt --check`

`cargo check`

`cargo clippy --all-targets --jobs 2 -- -D warnings`

`cargo test --jobs 2`

Executar o exemplo gráfico, caso tenha sido alterado.

Registrar:

- Comandos.
- Resultados.
- Limitações.
- Testes não executados.

Não modificar testes antigos para ocultar regressões.

---

# 20. DOCUMENTAÇÃO

Criar:

`docs/development/n3_4/`

Arquivos mínimos:

- `N3_4_AUDITORIA.md`
- `N3_4_REFERENCIAS_ORIGINAIS.md`
- `N3_4_IDENTIDADE_VISUAL.md`
- `N3_4_ROSTO_E_CABELO.md`
- `N3_4_PERSONAGEM_BASE.md`
- `N3_4_RESOLUCAO.md`
- `N3_4_PROCESSO_ARTISTICO.md`
- `N3_4_COMPARACAO.md`
- `N3_4_INTEGRACAO.md`
- `N3_4_TESTES.md`
- `N3_4_AVALIACAO.md`
- `N3_4_RELATORIO_FINAL.md`

Os documentos devem registrar o trabalho real, não o trabalho desejado.

---

# 21. PUBLICAÇÃO OBRIGATÓRIA NO GITHUB

A publicação dos resultados reais é obrigatória.

Antes de commitar:

1. Verificar branch e HEAD.
2. Conferir alterações preexistentes.
3. Verificar arquivos protegidos.
4. Executar testes aplicáveis.
5. Revisar os arquivos gerados.
6. Excluir temporários desnecessários.

Criar commits organizados.

Fazer push para:

`origin/art/v1.2-protagonist`

Confirmar o SHA remoto.

Não fazer force push.

Não fazer merge.

Não fazer rebase automático.

Não modificar `Md-usario/`.

Não adicionar arquivos privados ou segredos.

Se o trabalho artístico estiver bloqueado, publicar somente a auditoria, a documentação e as evidências reais disponíveis.

Não declarar que um sprite foi produzido se não foi.

---

# 22. RELATÓRIO FINAL

Informar:

**A. Estado inicial:** branch, HEAD e árvore de trabalho.

**B. Arquivos originais:** quais foram abertos e analisados.

**C. Identidade:** características observadas.

**D. Fidelidade:** comparação com `image.png`.

**E. Rosto:** resultado.

**F. Cabelo:** resultado.

**G. Corpo:** proporções.

**H. Roupa-base:** fidelidade.

**I. Resolução:** medidas.

**J. Processo artístico:** ferramentas e autoria real.

**K. Modularidade:** o que está separado e o que não está.

**L. Bevy:** integração.

**M. Evidências:** arquivos publicados.

**N. Testes:** resultados.

**O. Segurança:** integridade dos originais.

**P. GitHub:** commits e SHA remoto.

**Q. Limitações:** impedimentos.

**R. Recomendação:** aprovar para avaliação, corrigir ou interromper.

**S. Aprovação pendente:** decisão do proprietário.

---

# 23. REGRA FINAL DE EXECUÇÃO

Esta etapa deve corrigir a direção artística, não aumentar a quantidade de scripts.

**O protagonista original é a fonte da verdade visual.**

Se conseguir produzir uma adaptação fiel e convincente, publique a prova e aguarde avaliação.

Se não conseguir, explique exatamente por quê.

Não invente um novo personagem.

Não apresente blocos como arte final.

Não esconda a ausência de pintura.

Não comece animações antes de validar a base.

Não inicie N4 ou N5.

**Execute a N3.4, publique os resultados verificáveis no GitHub e aguarde a aprovação artística do proprietário.**