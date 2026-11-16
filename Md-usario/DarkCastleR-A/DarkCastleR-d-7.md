# DARK CASTLE RPG — MEGA PROMPT FASE D7
## Reconstrução visual do protagonista D2 por código — Blender como referência, não como aparência final

**Projeto:** Dark Castle RPG  
**Engine:** Rust 2024 / Bevy 0.20.1  
**Fase:** D7 — Pixel Art Programática e Personagem Modular  
**Direção artística aprovada:** D2 detalhado  
**Prioridade:** aparência do protagonista, modularidade dos equipamentos e corrida  
**Ferramentas:** Claude Code, Python, Pillow, Blender como apoio e ferramentas existentes  
**Escopo:** laboratório isolado, sem alterar o jogo principal

---

# 1. MISSÃO PRINCIPAL

Você será responsável por investigar e construir uma nova abordagem para a produção artística do protagonista do Dark Castle RPG.

O proprietário identificou um problema persistente:

**O acabamento gerado pelo Blender nas fases D5 e D6 continua parecido com o personagem antigo, mesmo quando são adicionadas novas armaduras e filtros.**

Isso não atende à direção visual desejada.

A decisão é investigar um pipeline diferente:

**Claude Code escreve o código responsável pela construção e animação visual do personagem. O Blender pode fornecer referências de anatomia, rigging, movimento e posicionamento, mas não deve determinar o acabamento final.**

O protagonista deve continuar sendo o D2 detalhado.

Não redesenhe sua identidade.

Não utilize o estilo geométrico D3 como substituto.

Não utilize a referência do Espadachim das Sombras, que foi reservada exclusivamente para um futuro inimigo.

A meta da D7 é demonstrar que conseguimos produzir, de maneira programática e reproduzível, sprites 2D visualmente próximos do D2, com equipamentos independentes.

Não produza uma grande quantidade de animações antes de comprovar a qualidade de uma única pose.

---

# 2. AUDITORIA INICIAL DO PROJETO

Antes de criar arquivos:

1. Leia `CLAUDE.md`.
2. Leia as regras de desenvolvimento e arte.
3. Confirme branch, HEAD e upstream.
4. Execute `git status --short`.
5. Identifique arquivos locais modificados e não rastreados.
6. Confira as pastas D1 a D6.
7. Verifique quais ferramentas estão instaladas.
8. Preserve integralmente o trabalho anterior.

A branch esperada é:

`art/v1.2-protagonist`

Não presuma o hash atual.

Não altere:

- `src/`
- `assets/`
- `saves/`
- `Md-usario/`
- `.cargo/`
- `.claude/`
- Arquivos Blender originais.
- Protótipos D1–D6.

Não faça commit nem push.

Não utilize `git reset --hard`, `git clean`, force push ou staging indiscriminado.

Todo trabalho novo deve ficar em:

`art-src/visual-prototypes/d7_pixel_programatico/`

---

# 3. ESTUDAR O PROBLEMA REAL DAS FASES ANTERIORES

Leia os documentos e inspecione as imagens.

Referências:

`art-src/visual-prototypes/d2_protagonista_B/`

`art-src/visual-prototypes/d3_protagonista_B_refinado/`

`art-src/visual-prototypes/d4_modular_d2/`

`art-src/visual-prototypes/d5_corrida_modular/`

`art-src/visual-prototypes/d6_fidelidade_d2/`

Priorize os seguintes estudos:

- Identidade visual D2.
- Paleta D2.
- Proporções e silhueta.
- Pose lateral.
- Cabelo e rosto.
- Casaco.
- Capa.
- Espada e bainha.
- Armaduras.
- GIFs de corrida.
- Problemas de acabamento da D5.
- Problemas de acabamento da D6.
- Arquitetura de camadas.
- Regras de sockets e pivôs.

Leia especificamente:

- `d5_corrida_modular/COMPARACAO_ACABAMENTO.md`
- `d6_fidelidade_d2/IDENTIDADE_D2.md`
- `d6_fidelidade_d2/COMPARACAO_VISUAL.md`
- `d6_fidelidade_d2/LIMITACOES.md`
- `d6_fidelidade_d2/ARQUITETURA_EQUIPAMENTOS.md`

Confirme o que os arquivos realmente demonstram.

Não confunda uma composição de imagens estáticas com um pipeline modular funcional.

---

# 4. MUDANÇA FUNDAMENTAL DE ESTRATÉGIA

Nas fases anteriores, o fluxo principal era:

Blender → Render EEVEE → Toon/Outline → PNG → Composição.

O resultado manteve a aparência do modelo antigo.

Agora investigue:

**Referência D2 → Definição de formas 2D → Desenho por código → Camadas independentes → Composição → Animação.**

O Blender poderá ser utilizado para:

- Referência de poses.
- Referência de anatomia.
- Referência de articulações.
- Cálculo de movimentos.
- Transformações de ossos.
- Posicionamento de equipamentos.
- Exportação de dados auxiliares.

Mas o personagem final não deve ser simplesmente um render EEVEE recolorido.

Não use downscale do modelo antigo como solução principal.

Não use filtros de borda como substitutos de desenho.

**Queremos que a aparência final seja definida pela arte 2D, não pelos materiais do Blender v007.**

---

# 5. INVESTIGAÇÃO TÉCNICA — DESENHO 2D POR CÓDIGO

Investigue a viabilidade de criar sprites por código Python.

Ferramentas possíveis:

- Pillow.
- Desenho de polígonos.
- Máscaras.
- Paletas indexadas.
- Composição RGBA.
- Transformações de camadas.
- Desenho por coordenadas.
- Curvas e formas rasterizadas.
- Sprites de referência.
- Dados declarativos de poses.
- Scripts de animação.
- Ferramentas de geração visual disponíveis no ambiente.

Não instale dependências desnecessárias.

Não altere o ambiente global do projeto sem necessidade.

Avalie pelo menos três estratégias:

### Estratégia A — Pixel art por primitivas

Construir o personagem com formas, polígonos e detalhes definidos em código.

Vantagens potenciais:

- Reprodutibilidade.
- Camadas independentes.
- Controle de pixels.
- Facilidade de variação.

Riscos:

- Aparência geométrica.
- Falta de detalhes.
- Semelhança com a D3 rejeitada.

### Estratégia B — Pixel art por sprites e máscaras

Utilizar partes 2D desenhadas como fontes, com máscaras e transformações controladas por código.

Vantagens potenciais:

- Maior fidelidade artística.
- Peças independentes.
- Melhor controle de texturas.

Riscos:

- Necessidade de arte-fonte consistente.
- Deformações.
- Trabalho de ajuste.

### Estratégia C — Híbrido 2D programático

Combinar:

- Anatomia e movimento do Blender.
- Silhuetas 2D próprias.
- Máscaras.
- Sprites por camada.
- Paletas.
- Detalhes desenhados por código.
- Correções artísticas pontuais.

Essa estratégia não deve simplesmente reproduzir o render antigo.

Compare as três antes de escolher a arquitetura.

---

# 6. PRIMEIRO MARCO — UMA POSE D2 CONVINCENTE

**Não comece pela corrida.**

Primeiro crie uma pose lateral estática do protagonista.

A pose deve preservar:

- Cabelo curto e escuro.
- Rosto legível.
- Proporção aristocrática.
- Casaco gótico.
- Capa vinho.
- Detalhes metálicos dourados.
- Botas.
- Luvas.
- Espada curva.
- Bainha.
- Silhueta característica D2.

Crie três candidatos:

- `D7-A`: desenho programático por primitivas.
- `D7-B`: desenho baseado em partes 2D e máscaras.
- `D7-C`: híbrido programático com dados auxiliares do Blender.

Cada candidato deve ser gerado por um script reproduzível.

**Não apresente uma imagem de conceito gerada externamente como se tivesse sido desenhada pelo script.**

Se usar uma imagem de referência, documente sua origem e como foi utilizada.

O objetivo é avaliar a qualidade real que o código consegue produzir.

---

# 7. CRITÉRIO DE FIDELIDADE D2

Compare cada candidato com a referência D2.

Avalie separadamente:

1. Anatomia.
2. Silhueta.
3. Cabeça.
4. Cabelo.
5. Rosto.
6. Casaco.
7. Capa.
8. Espada.
9. Bainha.
10. Botas.
11. Paleta.
12. Contraste.
13. Acabamento.
14. Leitura em escala real.

Classifique:

- Aprovável para avaliação do proprietário.
- Promissor, mas incompleto.
- Apenas prova técnica.
- Rejeitado visualmente.

Não escolha automaticamente o candidato mais fácil de programar.

A fidelidade visual tem prioridade.

Se nenhuma estratégia alcançar uma aparência convincente, pare a expansão e apresente o diagnóstico.

Não esconda o problema criando mais animações.

---

# 8. RESOLUÇÃO E ESCALA

Consulte o contrato real de sprites do projeto.

Referências conhecidas:

- Corpo do jogador: aproximadamente 26 × 46 unidades de colisão.
- Silhueta visual: aproximadamente 52 unidades de altura.
- Câmera V2.3: altura de visão 405.
- Densidade anterior: 2 texels por unidade.

Confirme esses valores no código e nos dados atuais.

Não confunda:

- Resolução do sprite.
- Dimensão da célula.
- Tamanho da silhueta.
- Tamanho da hurtbox.
- Escala na tela.

Produza as comparações na escala real do jogo.

Não aumente o personagem apenas para melhorar a aparência da prancha.

Se 52 pixels de altura não forem suficientes para determinados detalhes, avalie uma resolução interna maior com escala final controlada.

Documente o impacto na legibilidade e no pipeline.

---

# 9. ARQUITETURA DE CAMADAS INDEPENDENTES

Depois de escolher o candidato visual mais promissor, demonstre a separação de camadas.

Categorias:

- Corpo-base.
- Cabelo.
- Rosto.
- Camisa.
- Casaco.
- Peitoral.
- Ombreiras.
- Braçadeiras.
- Luvas.
- Calças.
- Botas.
- Capa.
- Chapéu.
- Elmo.
- Óculos.
- Espada.
- Escudo.
- Bainha.

Não é necessário finalizar todas essas peças na D7.

Mas a arquitetura deve permitir sua inclusão futura.

Cada camada deve possuir:

- Identificador.
- Dimensões.
- Origem.
- Pivô.
- Ordem de desenho.
- Máscara opcional.
- Paleta.
- Regras de visibilidade.
- Âncora ou articulação.
- Associação com animação, quando necessária.

Não altere os oito slots funcionais atuais do jogo.

Slots visuais são uma camada de apresentação.

Não modifique Save V3.

---

# 10. EQUIPAMENTOS — PROVA DE CONCEITO

Crie dois conjuntos sobre o mesmo corpo-base.

## Conjunto A — Aristocrata viajante

- Camisa.
- Casaco gótico.
- Calças.
- Botas.
- Luvas.
- Capa vinho.
- Espada.
- Bainha.

## Conjunto B — Cavaleiro equipado

- Peitoral.
- Ombreiras.
- Braçadeiras.
- Luvas pesadas.
- Proteção de pernas.
- Botas reforçadas.
- Elmo.
- Capa.
- Espada.
- Escudo.

O conjunto B precisa ter uma silhueta diferente.

Não basta recolorir o conjunto A.

Não use cubos de Blender como armadura final.

Se a qualidade artística ainda não estiver pronta, marque a peça como protótipo.

---

# 11. TESTE DE TROCA INDIVIDUAL

Demonstre que as peças podem ser trocadas independentemente.

Testes:

1. Trocar somente as botas.
2. Trocar somente as luvas.
3. Trocar somente o casaco.
4. Trocar somente o peitoral.
5. Trocar somente a capa.
6. Trocar somente a espada.
7. Trocar somente o escudo.
8. Trocar somente o elmo.

A composição deve preservar os demais elementos.

Não gere oito personagens completos independentes.

Registre os problemas de oclusão.

Exemplo:

Um elmo fechado pode ocultar cabelo.

Um casaco pode ocultar parte da calça.

A armadura pode exigir peças dianteiras e traseiras.

---

# 12. SEGUNDO MARCO — ANIMAÇÃO PILOTO

Somente depois de produzir uma pose estática convincente, crie um piloto curto de movimento.

Prioridade:

**Idle → corrida.**

Use o contrato real de animação.

Se a corrida atual possuir dez quadros, utilize essa contagem como referência inicial.

O objetivo é testar:

- Consistência anatômica.
- Movimento das pernas.
- Movimento dos braços.
- Movimento do casaco.
- Movimento da capa.
- Estabilidade da cabeça.
- Estabilidade dos equipamentos.
- Alinhamento dos pés.
- Continuidade do ciclo.

Não produza os demais clips.

Não produza ataques, magias ou transformações nesta fase.

Se a qualidade cair drasticamente durante a animação, registre o problema e não declare o pipeline pronto.

---

# 13. ANIMAÇÃO MODULAR

O corpo-base deve possuir uma única animação.

Os equipamentos devem acompanhar o movimento por:

- Articulações.
- Pivôs.
- Transformações.
- Sprites por peça.
- Máscaras.
- Deformações controladas.

Não crie um personagem inteiro por loadout.

Se uma peça precisar de animação própria, isso é permitido.

Exemplo:

A capa pode utilizar quadros próprios sincronizados ao movimento do corpo.

Mas não deve ser necessário recriar a animação completa do protagonista para trocar a capa.

---

# 14. COMPARAÇÃO NO CENÁRIO V2.3

Utilize a captura real da causeway.

Mostre:

- D2 original.
- D5.
- D6.
- D7-A.
- D7-B.
- D7-C.
- Candidato selecionado com roupa leve.
- Candidato selecionado com armadura pesada.

Crie comparações:

- Escala real.
- Ampliação nearest-neighbor.
- Fundo escuro.
- Fundo claro.
- Cenário V2.3.

Mantenha:

- Mesma escala.
- Mesmo chão.
- Mesma orientação.
- Mesmo enquadramento.

Não modifique o cenário aprovado.

---

# 15. TESTES AUTOMÁTICOS

Crie testes para:

- Dimensões.
- Transparência.
- Paleta.
- Quantidade de quadros.
- Pivôs.
- Alinhamento dos pés.
- Composição de camadas.
- Independência dos equipamentos.
- Ordem de desenho.
- Máscaras.
- Ausência de fundo magenta.
- Integridade dos PNGs.
- Reprodução determinística.

Se os scripts gerarem imagens a partir de dados e código, teste a regeneração em diretório temporário.

Compare os resultados por hash quando apropriado.

**Os testes automáticos não aprovam a qualidade artística.**

---

# 16. PROIBIÇÃO DE FALSO PROGRESSO

Evite repetir o problema das fases anteriores.

Não considere avanço artístico:

- Apenas aumentar o número de arquivos.
- Criar mais GIFs com aparência antiga.
- Adicionar filtros toon.
- Aplicar contornos automáticos.
- Reduzir resolução.
- Recolorir o mesmo modelo.
- Produzir armaduras geométricas sem acabamento.
- Criar composições inteiras e chamá-las de equipamentos independentes.

O sucesso precisa ser visualmente demonstrável.

Se o código produzir um personagem tecnicamente modular, mas artisticamente inferior ao D2, classifique como prova técnica parcial.

---

# 17. PONTO DE CONTROLE OBRIGATÓRIO

A execução deverá ter dois marcos.

## Marco A — Identidade estática

Entregue:

- Três candidatos.
- Comparação D2.
- Escala real.
- Código-fonte.
- Avaliação de fidelidade.

Se nenhum candidato for visualmente convincente, **pare aqui**.

Não inicie a animação.

Explique qual trabalho artístico adicional seria necessário.

## Marco B — Modularidade e movimento

Execute somente se o Marco A produzir pelo menos um candidato tecnicamente promissor e visualmente próximo do D2.

Entregue:

- Camadas independentes.
- Dois loadouts.
- Troca de peças.
- Idle.
- Corrida piloto.
- Comparações na causeway.
- Testes.

Mesmo que avance ao Marco B, não publique no jogo.

---

# 18. ESTRUTURA DE ARQUIVOS

Crie:

```text
art-src/visual-prototypes/d7_pixel_programatico/
├── README.md
├── AUDITORIA_PIPELINE.md
├── ESTUDO_ESTRATEGIAS.md
├── IDENTIDADE_D2.md
├── ARQUITETURA_CAMADAS.md
├── COMPARACAO_VISUAL.md
├── LIMITACOES.md
├── DECISAO.md
├── referencias/
├── estrategias/
│   ├── A_primitivas/
│   ├── B_sprites_masks/
│   └── C_hibrido/
├── personagem/
├── equipamentos/
├── animacoes/
├── loadouts/
├── previews/
├── comparacoes/
├── scripts/
└── testes/
```

Evite gerar centenas de arquivos redundantes.

Priorize artefatos que permitam avaliar o resultado.

---

# 19. ENTREGAS VISUAIS

Arquivos desejados:

1. `01_D2_D5_D6_comparacao.png`
2. `02_D7_tres_estrategias.png`
3. `03_D7_identidade_escala_real.png`
4. `04_D7_detalhes_personagem.png`
5. `05_D7_camadas.png`
6. `06_D7_loadouts.png`
7. `07_D7_troca_equipamentos.png`
8. `08_D7_causeway.png`
9. `09_D7_problemas.png`
10. `10_D7_resultado_final.png`

GIFs, somente se o Marco B for alcançado:

- Idle.
- Corrida leve.
- Corrida pesada.
- Troca dinâmica.

Se algum arquivo não for produzido, documente o motivo.

Não gere arquivos vazios apenas para preencher a estrutura.

---

# 20. CRITÉRIOS DE ACEITAÇÃO

## Marco A

- [ ] Identidade D2 reconhecível.
- [ ] Silhueta convincente.
- [ ] Cabelo e rosto legíveis.
- [ ] Casaco detalhado.
- [ ] Capa coerente.
- [ ] Espada e bainha consistentes.
- [ ] Paleta compatível.
- [ ] Escala real respeitada.
- [ ] Imagens geradas por scripts reproduzíveis.
- [ ] Comparação honesta com o D2.

## Marco B

- [ ] Corpo-base compartilhado.
- [ ] Equipamentos independentes.
- [ ] Dois loadouts.
- [ ] Troca individual.
- [ ] Idle consistente.
- [ ] Corrida consistente.
- [ ] Pivôs estáveis.
- [ ] Sem clipping grave.
- [ ] Exportação reproduzível.
- [ ] Sem alterações no jogo principal.

A aprovação artística final pertence ao proprietário.

---

# 21. RELATÓRIO FINAL

Apresente:

A. Estado inicial do Git.

B. Auditoria D2/D5/D6.

C. Causa principal da aparência antiga.

D. Estratégias programáticas estudadas.

E. Implementação D7-A.

F. Implementação D7-B.

G. Implementação D7-C.

H. Comparação visual dos candidatos.

I. Avaliação de fidelidade D2.

J. Resultado do Marco A.

K. Arquitetura de camadas, se aplicável.

L. Equipamentos e loadouts, se aplicável.

M. Animações piloto, se aplicável.

N. Comparação na causeway.

O. Testes executados.

P. Problemas artísticos encontrados.

Q. Limitações do pipeline.

R. Arquivos prioritários para avaliação.

S. Estado final do Git.

T. Recomendação para a D8.

Não faça commit.

Não faça push.

Não altere o jogo principal.

Não declare arte aprovada sem a decisão do proprietário.

---

# 22. DIRETRIZ FINAL

A D7 deve responder a uma pergunta:

**Conseguimos criar por código uma aparência próxima do protagonista D2, mantendo equipamentos independentes, sem depender do acabamento visual antigo do Blender?**

Se a resposta for sim, demonstre com imagens e scripts reproduzíveis.

Se a resposta for parcialmente, identifique exatamente o que falta.

Se a resposta for não, apresente uma alternativa viável baseada em arte-fonte 2D e automação, sem insistir em gerar mais variações geométricas.

A prioridade é a qualidade visual do protagonista.

**Blender como apoio. Código como ferramenta de produção. D2 como identidade. Modularidade como requisito. Nenhuma integração antes da aprovação.**