# DARK CASTLE RPG — MEGA PROMPT N3.5
# FIDELIDADE VISUAL À FICHA ORIGINAL, RESOLUÇÃO E PROVA ARTÍSTICA

**Projeto:** Dark Castle RPG  
**Etapa:** N3.5 — Fidelity & Resolution Proof  
**Branch:** `art/v1.2-protagonist`  
**HEAD remoto inicial esperado:** `c1eb46c7e31cdb58926ab6dcdd8f04c13109f868`  
**Tecnologias:** Rust 2024, Bevy 0.20.1, Blender e ferramentas de imagem disponíveis  
**Status:** AUTORIZADA PARA EXECUÇÃO  
**Gate A da N3.4:** REPROVADO ARTISTICAMENTE  
**N4 e N5:** BLOQUEADAS

---

# 0. MISSÃO

Você é o responsável pela execução técnica e artística experimental da N3.5.

O proprietário avaliou visualmente as evidências da N3.4 e rejeitou o resultado como estilo final.

A N3.4 melhorou a identidade do personagem em relação à N3.3, mas ainda não alcançou o nível de detalhe, proporção, acabamento e expressividade das referências originais.

**O proprietário identificou explicitamente três referências visuais desejadas:**

1. **Ficha (perfil)** — rosto e cabelo na vista lateral.
2. **Ficha lateral** — corpo completo, anatomia e roupas.
3. **Redução** — interpretação reduzida que preserva melhor o acabamento visual.

Essas três referências são o alvo artístico.

Não substituir essas referências por:

- N3.3.
- N3.4.
- Conceito B da N3.2.
- Outro personagem gerado por IA.
- Um novo manequim.
- Uma silhueta genérica.

A missão é investigar como reproduzir o visual desejado na escala efetiva do jogo, preservando a identidade do protagonista.

---

# 1. PRINCÍPIO FUNDAMENTAL

**Não confundir três conceitos:**

- Resolução da imagem-fonte.
- Resolução do sprite exportado.
- Tamanho efetivo do personagem na tela.

A N3.4 mostrou que um sprite com 106 texels de altura pode ser apresentado com aproximadamente 94 pixels de tela na câmera atual.

Portanto, aumentar a resolução da arte não garante que mais detalhes sejam visíveis no jogo.

A N3.5 deve investigar conjuntamente:

1. Resolução artística.
2. Densidade de texels por unidade.
3. Escala de apresentação.
4. Amostragem de textura.
5. Enquadramento da câmera.
6. Legibilidade em movimento.

Nenhuma dessas variáveis deve ser alterada silenciosamente.

---

# 2. AUDITORIA INICIAL

Antes de modificar arquivos:

Ler:

- `CLAUDE.md`
- `docs/ARCHITECTURE.md`
- `docs/DECISIONS.md`
- `docs/development/n1/N1_PIXEL_ART_32_64_BITS.md`
- `docs/development/n1/N1_PIPELINE_3D_PARA_2D.md`
- `docs/development/n1/N1_PERSONAGEM_MODULAR.md`
- `docs/development/n3/N3_RELATORIO_FINAL.md`
- `docs/development/n3_1/N3_1_RELATORIO_FINAL.md`
- `docs/development/n3_2/N3_2_RELATORIO_FINAL.md`
- `docs/development/n3_3/N3_3_RELATORIO_FINAL.md`
- `docs/development/n3_4/N3_4_RELATORIO_FINAL.md`
- `docs/development/n3_4/N3_4_RESOLUCAO.md`
- `docs/development/n3_4/N3_4_AVALIACAO.md`

Inspecionar os arquivos realmente existentes.

Confirmar branch, HEAD e estado do Git.

Preservar alterações preexistentes.

---

# 3. REFERÊNCIAS OBRIGATÓRIAS

A referência principal continua sendo:

`Md-usario/assets-user/prota/base/image.png`

A ficha modular:

`Md-usario/assets-user/prota/base/Ficha_Modular_do_Protagonista_Gotico.png`

Verificar também se existe:

`Md-usario/assets-user/prota/base/Ficha_de_Modelagem_do_Protagonista_Gotico.png`

O proprietário compartilhou na conversa duas folhas de comparação contendo os recortes denominados:

- `ficha (perfil)`
- `ficha lateral`
- `reducao`

Essas folhas podem não estar presentes no repositório.

**Não afirmar que elas estão disponíveis localmente sem verificar.**

Se estiverem disponíveis, utilizá-las.

Se não estiverem, reconstruir os recortes de perfil e corpo a partir das fichas originais, documentando a procedência.

A imagem chamada `reducao` é uma referência de acabamento, mas sua resolução real ainda precisa ser verificada. Se o arquivo original dessa redução não estiver acessível, solicitar ao proprietário o arquivo antes de afirmar equivalência exata.

Não confundir uma reconstrução aproximada com o arquivo original.

---

# 4. DIREÇÃO ARTÍSTICA FIXA

O protagonista deve continuar sendo:

**Guerreiro gótico aristocrático, de cabelos longos branco-prateados, rosto masculino sério, camisa cinza-escura, calça escura, braceletes e botas altas.**

Preservar:

- Proporções humanas relativamente realistas.
- Perfil elegante.
- Nariz definido.
- Queixo e mandíbula reconhecíveis.
- Cabelo longo com mechas.
- Caimento natural do cabelo.
- Camisa com dobras.
- Manga dobrada.
- Cordão na gola.
- Cinto com fivela.
- Calça com volume.
- Botas com cano, aba e detalhes.

Não alterar o personagem para torná-lo mais fácil de desenhar.

Não colocar armadura, capa ou espada no corpo-base para esconder problemas.

---

# 5. POR QUE A N3.4 NÃO FOI APROVADA

A N3.4 tem:

- Canvas 80×128.
- Altura opaca 106 px.
- Pivô `(40, 118)`.
- Paleta de 19 cores.
- Rosto de aproximadamente 7×12 px.
- Fonte editável em grade de texto.
- Integração isolada no Bevy.

Mas o resultado visual apresenta:

- Coroa do cabelo lisa.
- Mechas pouco naturais.
- Rosto com pouca expressão.
- Perda de identidade em 1×.
- Calça excessivamente reta.
- Botas pouco volumosas.
- Antebraço sem modelagem suficiente.
- Diferença perceptível em relação ao acabamento da referência reduzida.

Não tratar essas falhas como detalhes secundários.

O objetivo da N3.5 é investigar se elas decorrem principalmente:

A. Da resolução.

B. Da técnica de desenho.

C. Da redução.

D. Da escala de apresentação.

E. De uma combinação desses fatores.

---

# 6. PRIMEIRO EXPERIMENTO — MEDIR AS REFERÊNCIAS

Antes de desenhar qualquer sprite novo, produzir uma análise das imagens.

Medir:

- Altura da figura.
- Largura da figura.
- Altura da cabeça.
- Largura do rosto.
- Comprimento do cabelo.
- Largura dos ombros.
- Comprimento dos braços.
- Comprimento das pernas.
- Altura das botas.
- Proporção entre cabeça e corpo.

Separar medidas reais de estimativas.

Comparar:

1. Imagem-base original.
2. Ficha lateral.
3. Redução de referência, se disponível.
4. N3.4.

Criar uma folha com guias de proporção.

Não usar redimensionamentos diferentes para favorecer artificialmente uma das versões.

---

# 7. SEGUNDO EXPERIMENTO — RESOLUÇÕES CANDIDATAS

Investigar pelo menos três hipóteses.

## Hipótese A — 80×128

Manter o contrato N3.4.

Figura com aproximadamente 106 px de altura.

Vantagem:

- Compatibilidade imediata com a prova anterior.

Risco:

- Rosto pequeno.
- Poucos pixels para mechas.
- Dificuldade em reproduzir dobras.

## Hipótese B — 96×160

Aumentar a área disponível.

A altura opaca poderá ser maior, conforme a composição.

Vantagem:

- Mais espaço para rosto e roupas.

Risco:

- Se o tamanho na tela continuar o mesmo, será necessário reduzir mais texels.

## Hipótese C — Resolução derivada da referência

Medir a resolução efetiva necessária para conservar a aparência da imagem `reducao`.

Não escolher números arbitrários.

Não presumir que a referência tem 100 px.

Registrar a resolução medida ou, caso não haja arquivo original, a melhor estimativa e sua incerteza.

---

# 8. TERCEIRO EXPERIMENTO — APRESENTAÇÃO NO JOGO

A N3.4 documentou:

- Câmera de 405 unidades.
- Janela de 720 px.
- Densidade 2 texels por unidade.
- Figura de 106 texels.
- Altura apresentada próxima de 94 pixels.

A N3.5 deve testar, em exemplo isolado:

**Cenário 1:** câmera equivalente a 405.

**Cenário 2:** câmera equivalente a 360.

**Cenário 3:** uma alternativa de escala/densidade, caso seja tecnicamente justificável.

Não modificar `assets/data/camera.ron`.

Não modificar a câmera do jogo principal.

Manter o personagem com dimensões de mundo comparáveis nos experimentos.

Não confundir aumento do personagem na tela com melhoria do desenho.

---

# 9. MATRIZ DE RESOLUÇÃO E ESCALA

Criar uma matriz visual comparando:

| Arte | Câmera equivalente | Objetivo |
|---|---|---|
| 80×128 | 405 | Contrato atual |
| 80×128 | 360 | Correspondência inteira |
| 96×160 | 405 | Mais detalhes na fonte |
| 96×160 | 360 | Mais detalhes com enquadramento alternativo |
| Resolução derivada | 405 | Preservação da referência |
| Resolução derivada | 360 | Preservação com escala alternativa |

Se uma combinação não for tecnicamente válida, explicar por quê.

Não criar capturas que apresentem tamanhos físicos incompatíveis sem rotulá-los.

Documentar:

- Texels por unidade.
- Pixels por texel na tela.
- Altura visível.
- Perda por amostragem.
- Resultado artístico.

---

# 10. QUARTO EXPERIMENTO — FIDELIDADE DO ROSTO

O rosto é o principal critério.

Criar uma comparação específica entre:

- Ficha original.
- Redução de referência.
- N3.4.
- Candidato N3.5.

Avaliar:

- Testa.
- Sobrancelha.
- Olho.
- Nariz.
- Boca.
- Queixo.
- Mandíbula.
- Linha do pescoço.
- Transição cabelo/pele.
- Expressão séria.

O rosto precisa manter a identidade visual, não apenas possuir pixels que representem partes anatômicas.

Não considerar um close ampliado como prova suficiente.

O rosto também deve ser avaliado na escala real de apresentação.

---

# 11. QUINTO EXPERIMENTO — CABELO

A coroa do cabelo da N3.4 parece excessivamente lisa.

A referência desejada possui:

- Mechas longas.
- Direção de fluxo.
- Volume natural.
- Separação de grupos.
- Reflexos branco-prateados.
- Sombras frias.
- Pontas irregulares.
- Movimento sugerido pela forma.

Não desenhar o cabelo como uma massa branca uniforme.

Não usar ruído aleatório.

Não aumentar o número de cores sem necessidade.

Priorizar grupos de pixels que descrevam mechas reais.

---

# 12. SEXTO EXPERIMENTO — ROUPAS

Comparar as formas da N3.4 com a ficha lateral.

Corrigir, se a resolução permitir:

- Volume do torso.
- Manga dobrada.
- Cordão da camisa.
- Cinto.
- Bracelete.
- Antebraço.
- Dobras diagonais da calça.
- Joelho.
- Panturrilha.
- Cano da bota.
- Aba da bota.
- Fivelas.
- Sola.

Não transformar cada peça em um retângulo de cor.

Não reproduzir filigranas desnecessárias.

O objetivo é preservar a anatomia e a leitura dos materiais.

---

# 13. PROCESSO ARTÍSTICO

A N3.4 utilizou correções em grade de texto.

Essa técnica pode ser mantida como formato editável auxiliar.

Entretanto, ela não deve ser a única estratégia artística se continuar produzindo acabamento inferior à referência.

Investigar ferramentas gráficas realmente disponíveis.

Se for possível operar uma interface de pintura, utilizá-la.

Se não for possível, registrar a limitação.

Scripts podem auxiliar em:

- Recortes.
- Alinhamento.
- Medição.
- Montagem de painéis.
- Exportação.
- Testes.
- Comparação de paletas.

Scripts não devem ser usados para criar automaticamente um novo boneco geométrico.

**Não confundir quantidade de pixels alterados com qualidade artística.**

---

# 14. USO DE IA E REFERÊNCIAS

A geração de imagens só poderá ser usada para apoiar uma transformação visual fiel.

Se houver ferramenta autorizada capaz de editar uma imagem existente, priorizar a referência original em vez de gerar outro personagem.

Não alterar:

- Cor do cabelo.
- Comprimento do cabelo.
- Rosto.
- Anatomia.
- Roupa-base.
- Orientação.

Não gerar animações.

Não gerar novas armaduras.

Não gerar novos conceitos.

Não enviar recursos privados para serviços externos sem autorização.

Se a ferramenta não conseguir preservar a identidade, não utilizar seu resultado como sprite final.

---

# 15. CRIAR UM ÚNICO CANDIDATO ARTÍSTICO

Depois dos experimentos, escolher uma hipótese técnica para produzir um único candidato N3.5.

Esse candidato deve:

- Seguir a ficha lateral.
- Preservar o rosto.
- Preservar o cabelo.
- Ter anatomia coerente.
- Ter roupas legíveis.
- Ter botas reconhecíveis.
- Funcionar em fundo escuro.
- Funcionar na escala de jogo.
- Possuir transparência correta.
- Ter fonte editável.
- Ter pivô documentado.

Se o candidato exigir mudança de resolução, não sobrescrever a N3.4.

Criar arquivos novos.

Não promover automaticamente o resultado.

---

# 16. NÃO PRODUZIR ANIMAÇÕES

A N3.5 não deve produzir:

- Walk.
- Jump.
- Attack.
- Cast.
- Dodge.
- Guard.
- Death.

Essas animações continuam fora do escopo.

Não adicionar quadros apenas para demonstrar atividade.

O objetivo é aprovar um único personagem estático.

---

# 17. MODULARIDADE

Manter a base sem equipamentos removíveis adicionais.

Preservar a possibilidade futura de:

- Casaca.
- Armadura.
- Capa.
- Espada.
- Escudo.
- Elmo.

Não implementar N4.

Calça e botas permanecem visualmente na base conforme `image.png`, até decisão específica sobre sua remoção.

Não redesenhar as pernas para criar slots nesta etapa.

---

# 18. INTEGRAÇÃO BEVY

Criar uma nova fonte experimental:

`n3.5-fidelity`

Ou nomenclatura equivalente.

Reutilizar o exemplo visual isolado.

Permitir comparação com N3.4.

Exibir claramente:

- Resolução da arte.
- Altura opaca.
- Densidade.
- Escala.
- Câmera equivalente.
- Estado de aprovação.

Não alterar o jogo principal.

Não modificar `camera.ron`.

Não substituir o protagonista publicado.

---

# 19. EVIDÊNCIAS OBRIGATÓRIAS

Criar:

`docs/development/n3_5/evidence/`

Arquivos sugeridos:

1. `reference_sources.png`
2. `reference_measurements.png`
3. `resolution_80x128.png`
4. `resolution_96x160.png`
5. `resolution_reference.png`
6. `camera_405.png`
7. `camera_360.png`
8. `resolution_matrix.png`
9. `face_comparison.png`
10. `hair_comparison.png`
11. `clothing_comparison.png`
12. `n34_vs_n35.png`
13. `native_scale.png`
14. `dark_background.png`
15. `bevy_n35.png`
16. `final_candidate.png`

Publicar apenas evidências realmente produzidas.

Se uma hipótese não for implementada, documentar a razão.

Não gerar uma imagem vazia apenas para completar a lista.

---

# 20. CRITÉRIOS ARTÍSTICOS

Avaliar:

## Identidade

O sprite lembra claramente o protagonista da ficha?

## Rosto

A expressão séria e o perfil sobrevivem na escala de jogo?

## Cabelo

O cabelo parece longo e prateado, com mechas naturais?

## Corpo

As proporções continuam próximas da ficha?

## Roupa

A camisa, a calça e as botas possuem volume?

## Silhueta

O personagem é reconhecível sem detalhes internos?

## Apresentação

A câmera e a densidade preservam os pixels relevantes?

## Fidelidade

O candidato N3.5 se aproxima mais da referência `reducao` do que a N3.4?

**O Gate A permanece pendente de aprovação do proprietário.**

Não atribuir aprovação automática com base em métricas.

---

# 21. TESTES

Executar:

`cargo fmt --check`

`cargo check`

`cargo clippy --all-targets --jobs 2 -- -D warnings`

`cargo test --jobs 2`

Executar o exemplo visual.

Testar:

- Carregamento de PNG.
- Dimensões.
- Pivô.
- Transparência.
- Escala.
- Manifestos.
- Compatibilidade com a prova anterior.
- Ausência de regressões.

Não modificar testes existentes para ocultar falhas.

Registrar resultados reais.

---

# 22. DOCUMENTAÇÃO

Criar:

`docs/development/n3_5/`

Arquivos mínimos:

1. `N3_5_AUDITORIA.md`
2. `N3_5_REFERENCIAS.md`
3. `N3_5_MEDICOES.md`
4. `N3_5_RESOLUCOES.md`
5. `N3_5_CAMERA.md`
6. `N3_5_ROSTO.md`
7. `N3_5_CABELO.md`
8. `N3_5_ROUPAS.md`
9. `N3_5_PROCESSO_ARTISTICO.md`
10. `N3_5_INTEGRACAO_BEVY.md`
11. `N3_5_TESTES.md`
12. `N3_5_AVALIACAO.md`
13. `N3_5_RELATORIO_FINAL.md`

Documentar as decisões, hipóteses, limitações e evidências.

Não preencher relatórios antes dos testes.

---

# 23. SEGURANÇA

Preservar:

- `Md-usario/`
- `white_mesh.glb`
- Blender V2.2 original.
- `assets/data/camera.ron`
- Save v3.
- Controles N2.
- Protagonista publicado.
- N3, N3.1, N3.2, N3.3 e N3.4.

Não alterar arquivos originais.

Não executar Git destrutivo.

Não realizar merge.

Não realizar rebase automático.

Não usar force push.

---

# 24. PUBLICAÇÃO OBRIGATÓRIA

Publicar resultados reais na branch:

`art/v1.2-protagonist`

Fluxo:

1. Verificar branch e HEAD.
2. Inspecionar working tree.
3. Conferir arquivos protegidos.
4. Executar testes.
5. Revisar evidências.
6. Criar commits organizados.
7. Fazer push.
8. Confirmar HEAD remoto.
9. Informar SHA completo.
10. Confirmar que o jogo principal não mudou.

Se houver bloqueio artístico, publicar a auditoria e a análise sem inventar um sprite.

Não commitar arquivos pessoais.

Não declarar sucesso se o push falhar.

---

# 25. RELATÓRIO FINAL

Apresentar:

**A. Estado inicial:** branch, HEAD e working tree.

**B. Referências:** imagens utilizadas e procedência.

**C. Medições:** proporções e resolução.

**D. Hipóteses:** alternativas investigadas.

**E. Resolução:** comparação.

**F. Câmera:** 405 versus 360.

**G. Rosto:** fidelidade e legibilidade.

**H. Cabelo:** volume e mechas.

**I. Roupa:** camisa, calça e botas.

**J. Anatomia:** proporções.

**K. Arte:** processo e autoria.

**L. Bevy:** integração isolada.

**M. Evidências:** arquivos publicados.

**N. Testes:** resultados.

**O. Segurança:** integridade dos originais.

**P. GitHub:** commits e SHA remoto.

**Q. Limitações:** problemas não resolvidos.

**R. Recomendação:** resolução e escala preferidas.

**S. Gate A:** aprovação artística pendente.

**T. Próxima decisão:** o que o proprietário precisa avaliar.

---

# 26. REGRA FINAL

A N3.5 não é uma etapa de programação de sistemas novos.

É uma investigação de fidelidade artística e apresentação.

O objetivo é responder:

**Qual combinação de resolução, desenho e escala permite reproduzir o protagonista original com a aparência desejada?**

Não confundir um render ampliado com um sprite aprovado.

Não aceitar um novo manequim de blocos.

Não inventar outra identidade visual.

Não iniciar animações.

Não alterar a câmera principal.

Não iniciar N4 ou N5.

**Execute a N3.5, publique os resultados verificáveis no GitHub e aguarde a avaliação artística do proprietário.**