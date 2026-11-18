# DARK CASTLE RPG — MEGA PROMPT N3.2
# DIREÇÃO ARTÍSTICA ASSISTIDA POR IA, PROTAGONISTA GÓTICO E ADAPTAÇÃO PARA SPRITES 2D

**Projeto:** Dark Castle RPG  
**Etapa:** N3.2 — Protótipo artístico vertical  
**Tecnologia:** Rust 2024 + Bevy 0.20.1 + Blender + ferramentas de pixel art  
**Branch:** `art/v1.2-protagonist`  
**HEAD remoto de referência:** `7f2f4764f1215c19b679b2627d71feabf3d7fc43`  
**Status:** AUTORIZADA PARA EXECUÇÃO  
**N4 e N5:** NÃO AUTORIZADAS

---

# 0. MISSÃO E OBJETIVO

Você atuará como diretor técnico de arte, artista técnico de personagens, engenheiro de pipeline, especialista em pixel art e integrador Bevy.

A missão é criar uma proposta visual convincente para o protagonista do Dark Castle RPG utilizando **geração de imagens como referência artística**, seguida de adaptação técnica ao pipeline existente.

O projeto já demonstrou:

- Composição de corpo, arma e capa.
- Uso de sockets.
- Troca de equipamentos.
- Reprodução de atlas.
- Renderização ortográfica no Blender.
- Conversão automática para sprites.
- Execução de protótipos isolados.

O que ainda NÃO foi demonstrado:

- Direção artística aprovada.
- Protagonista com acabamento gótico convincente.
- Rosto legível na escala escolhida.
- Materiais bem diferenciados.
- Pixel art de qualidade de produção.
- Consistência visual entre poses geradas por IA.
- Conversão de referência artística em animação reutilizável.

A N3.2 deve investigar esses problemas.

**Não basta produzir uma ilustração bonita. O objetivo é verificar se a identidade visual pode sobreviver à transformação em um sprite modular e animável.**

---

# 1. AUDITORIA OBRIGATÓRIA

Antes de modificar arquivos, ler:

- `CLAUDE.md`
- `docs/ARCHITECTURE.md`
- `docs/DECISIONS.md`
- `docs/development/n1/N1_DIRECAO_ARTISTICA.md`
- `docs/development/n1/N1_PERSONAGEM_MODULAR.md`
- `docs/development/n1/N1_EQUIPAMENTOS_VISUAIS.md`
- `docs/development/n1/N1_PIXEL_ART_32_64_BITS.md`
- `docs/development/n1/N1_PIPELINE_3D_PARA_2D.md`
- `docs/development/n1/N1_PROVA_DE_CONCEITO.md`
- `docs/development/n3/N3_RELATORIO_FINAL.md`
- `docs/development/n3_1/N3_1_RELATORIO_FINAL.md`
- `docs/development/n3_1/N3_1_AVALIACAO.md`
- `docs/development/n3_1/N3_1_COMPARACAO_ARTISTICA.md`
- `docs/development/n3_1/N3_1_PIXELORAMA.md`

Inspecionar:

- `art-src/player/blender/swordsman_rig_v007.blend`
- `art-src/n3-proof/`
- `examples/n3_visual_proof.rs`
- Manifestos de animação.
- Sockets.
- Camadas de arma e capa.
- Scripts de exportação.

Confirmar o HEAD e verificar a árvore de trabalho.

Não descartar alterações preexistentes.

---

# 2. REGRAS DE SEGURANÇA

Preservar integralmente:

- `Md-usario/`
- `white_mesh.glb`
- `saves/`
- `assets/animations/player.ron`
- `assets/animations/player_sockets.ron`
- `camera.ron`
- Save v3.
- Código principal de gameplay.
- Controles N2.
- Arte atualmente publicada.

Não executar `git clean`, `git reset --hard` ou force push.

Não substituir o protagonista no jogo principal.

Não alterar os arquivos originais do Blender em lugar.

Criar toda a experimentação em uma área isolada, preferencialmente:

`art-src/n3-2-art-direction/`

Não iniciar N4 ou N5.

---

# 3. PRINCÍPIO CENTRAL: IA COMO REFERÊNCIA, NÃO COMO PROVA AUTOMÁTICA

A geração de imagens deve ser usada para:

1. Explorar conceitos visuais.
2. Estudar silhuetas.
3. Definir armaduras.
4. Definir materiais.
5. Explorar rosto e cabelo.
6. Explorar capas.
7. Estudar proporções.
8. Criar referências de iluminação.
9. Propor uma paleta.
10. Apoiar o redesenho do personagem.

Uma imagem gerada não deve ser considerada automaticamente:

- Sprite final.
- Atlas válido.
- Personagem riggado.
- Animação coerente.
- Equipamento modular.
- Pixel art tecnicamente correta.

É obrigatório distinguir essas categorias nos arquivos e relatórios.

---

# 4. DISPONIBILIDADE REAL DE GERAÇÃO DE IMAGENS

Antes de começar, investigar quais ferramentas de geração de imagens estão realmente disponíveis no ambiente.

Verificar:

- Ferramentas já instaladas.
- APIs já configuradas e autorizadas.
- Recursos locais de GPU.
- Limites de VRAM.
- Capacidade de usar imagens de referência.
- Capacidade de preservar personagens entre gerações.
- Exportação de PNG.
- Resolução.
- Transparência.
- Licenças e condições de uso.

Não presumir que uma API está disponível.

Não procurar nem revelar chaves de API.

Não enviar arquivos privados para serviços externos sem autorização específica.

Não instalar modelos enormes automaticamente.

Não baixar dezenas de gigabytes.

Não iniciar serviços pagos sem autorização.

Não utilizar Hunyuan3D apenas porque está instalado.

Se não existir uma ferramenta de geração de imagens autorizada, registrar o bloqueio e preparar os prompts, especificações e referências necessárias para que o proprietário gere as imagens externamente.

**Nesse caso, interromper a fase que depende das imagens e solicitar os arquivos ao proprietário. Não inventar imagens nem substituí-las silenciosamente por figuras geométricas.**

---

# 5. DEFINIÇÃO DA IDENTIDADE DO PROTAGONISTA

O protagonista deve possuir identidade visual própria.

Direção geral:

**Guerreiro gótico de fantasia sombria, com influência de jogos de ação 2D clássicos, silhueta elegante, armadura medieval ornamentada, capa marcante e aparência de caçador de criaturas sobrenaturais.**

Evitar cópias diretas de personagens existentes.

O design deve combinar:

- Elegância.
- Mistério.
- Perigo.
- Mobilidade.
- Sofisticação.
- Elementos sobrenaturais.
- Armadura funcional.
- Leitura clara em pixel art.

O protagonista não deve parecer um soldado medieval genérico.

Também não deve depender de uma quantidade excessiva de detalhes impossíveis de representar em 100 pixels de altura.

---

# 6. TRÊS PROPOSTAS ARTÍSTICAS

Produzir três propostas conceituais distintas.

## Conceito A — Cavaleiro Gótico

- Armadura escura ornamentada.
- Ombreiras metálicas.
- Casaco longo.
- Capa vermelha profunda.
- Botas reforçadas.
- Silhueta imponente.
- Detalhes metálicos discretos.

## Conceito B — Caçador Aristocrático

- Casaco longo de alfaiataria fantástica.
- Armadura leve sob o tecido.
- Cabelos marcantes.
- Luvas e botas de couro.
- Capa com movimento.
- Silhueta mais fina e ágil.
- Mistura de nobreza e combate.

## Conceito C — Guerreiro Amaldiçoado

- Armadura assimétrica.
- Elementos sobrenaturais discretos.
- Capa danificada.
- Metais envelhecidos.
- Detalhes de energia arcana.
- Silhueta ameaçadora.
- Aparência de alguém marcado por uma maldição.

As três propostas devem compartilhar escala e enquadramento comparáveis.

Não misturar detalhes dos três conceitos automaticamente.

Não escolher uma proposta definitiva em nome do proprietário.

---

# 7. PROMPTS DE GERAÇÃO DE IMAGENS

Preparar prompts específicos para cada conceito.

Usar instruções que favoreçam consistência visual e adaptação para sprites.

## Prompt base — Cavaleiro Gótico

"Original gothic dark fantasy action RPG protagonist, full-body side-view character design, elegant armored swordsman, dark steel armor with restrained antique gold accents, deep crimson cape, long fitted coat, leather boots, articulated gauntlets, readable heroic silhouette, detailed but production-friendly anatomy, dramatic restrained lighting, consistent materials, clean separation of body and equipment, neutral standing pose, isolated plain background, game character concept sheet, original design, no text, no watermark."

## Prompt base — Caçador Aristocrático

"Original aristocratic supernatural hunter for a gothic 2D action RPG, full-body strict side-view, long tailored dark coat, layered leather and light metal protection, expressive hair silhouette, refined boots and gloves, deep burgundy fabric accents, elegant athletic proportions, mysterious expression, readable face and hands, coherent anatomy, production-ready character design reference, isolated neutral background, no text, no watermark."

## Prompt base — Guerreiro Amaldiçoado

"Original cursed gothic warrior for a dark fantasy 2D action RPG, full-body strict side-view, asymmetrical aged armor, weathered cloak, restrained arcane markings, layered dark fabrics, worn steel and leather, powerful readable silhouette, subtle supernatural atmosphere, consistent proportions, neutral character reference pose, clear separation of costume elements, isolated plain background, no text, no watermark."

Esses prompts são pontos de partida.

Adaptar para a ferramenta realmente disponível.

Não usar o nome de personagens protegidos como instrução para copiá-los.

---

# 8. FOLHAS DE REFERÊNCIA

Para cada conceito, tentar produzir:

- Vista lateral direita.
- Vista lateral esquerda.
- Vista frontal.
- Vista traseira.
- Close do rosto.
- Close do torso.
- Close das botas.
- Detalhes da capa.
- Detalhes dos materiais.
- Paleta sugerida.

**A vista lateral é prioritária**, pois o gameplay é 2D lateral.

As outras vistas servem para esclarecer volumes e construção.

Se a ferramenta não mantiver consistência entre vistas, registrar a falha.

Não afirmar que vistas geradas independentemente representam um modelo 3D consistente.

---

# 9. CRITÉRIOS PARA SELECIONAR A REFERÊNCIA

Avaliar cada conceito segundo:

- Silhueta.
- Originalidade.
- Leitura lateral.
- Rosto.
- Separação de materiais.
- Facilidade de animação.
- Compatibilidade com o rig.
- Compatibilidade com armaduras.
- Compatibilidade com capa.
- Viabilidade em aproximadamente 100 texels.
- Complexidade de produção.
- Custo para adicionar equipamentos.

Criar uma matriz comparativa.

A seleção automática pode indicar uma proposta recomendada, mas não equivale à aprovação do proprietário.

Se os conceitos não atenderem ao mínimo técnico, documentar isso.

---

# 10. COMPARAÇÃO COM O MANEQUIM V2.2

A N3.1 mostrou que o manequim possui:

- Casaco.
- Capa.
- Braços.
- Botas.
- Perfil legível.

Mas apresenta problemas:

- Rosto desaparece na redução.
- Materiais pouco diferenciados.
- Ausência de acabamento manual.
- Armadura única.
- Ataque sem antecipação.

Agora comparar as propostas visuais com o manequim existente.

Perguntas:

1. O rig atual suporta a proporção desejada?
2. O casaco precisa ser remodelado?
3. A cabeça precisa aumentar?
4. O rosto precisa ser simplificado?
5. A capa precisa de outra silhueta?
6. A armadura exige malha separada?
7. Os braços continuam animáveis?
8. Os sockets continuam utilizáveis?
9. O personagem mantém os pés no mesmo pivô?
10. A referência funciona quando reduzida?

Não reconstruir o modelo inteiro antes de responder.

---

# 11. ESCOLHER UM GESTO VERTICAL

A N3.2 deve focar em um único gesto como prova artística.

Prioridade:

**Ataque de espada com preparação, impacto e recuperação.**

Produzir pelo menos três poses coerentes:

1. Antecipação.
2. Impacto.
3. Recuperação.

A N3.1 constatou que o clipe publicado não tinha antecipação.

Essa deficiência precisa ser enfrentada.

Não aceitar três imagens independentes com anatomias incompatíveis.

As poses devem preservar:

- Mesmo personagem.
- Mesmo traje.
- Mesmas proporções.
- Mesma arma.
- Mesma mão dominante.
- Mesmo pivô.
- Mesma direção.
- Mesmo enquadramento.

Se a geração de imagens não conseguir manter consistência, usar o Blender como fonte geométrica das poses e a IA apenas como referência estética.

---

# 12. ADAPTAÇÃO PARA PIXEL ART

Não aplicar apenas um filtro e declarar sucesso.

Investigar um processo de adaptação que preserve:

- Silhueta.
- Materiais.
- Contraste.
- Expressão.
- Arma.
- Mãos.
- Casaco.
- Capa.
- Contornos.
- Proporções.

Trabalhar com resolução coerente com a N1:

**Personagem com aproximadamente 80–110 texels de altura.**

A resolução exata deve ser medida.

Produzir pelo menos:

- Referência original.
- Versão reduzida automaticamente.
- Versão adaptada tecnicamente.
- Comparação ampliada com nearest-neighbor.

Se não houver edição artística manual, a versão adaptada deve ser rotulada como automática ou semiautomática.

Não inventar autoria manual.

---

# 13. PALETA E LEGIBILIDADE

Criar uma paleta experimental com famílias de cor para:

- Pele.
- Cabelo.
- Couro.
- Tecido.
- Aço.
- Ouro.
- Capa.
- Efeitos sobrenaturais.

Não usar um único grupo de tons escuros para todos os materiais.

Verificar legibilidade:

- Fundo neutro.
- Fundo escuro.
- Fundo de castelo.
- Escala de trabalho.
- Escala real de apresentação.

O rosto deve ter pelo menos elementos identificáveis de cabeça, orientação e expressão geral.

Não exigir detalhes faciais impossíveis na escala escolhida.

---

# 14. MODULARIDADE VISUAL

A N3.2 deve preservar a arquitetura já validada.

Camadas:

- Corpo.
- Armadura.
- Arma.
- Capa.

Não produzir uma folha completa para cada combinação.

A espada deve permanecer separada do corpo.

A capa deve permanecer separada.

As armaduras podem exigir variantes de corpo, mas devem compartilhar o mesmo contrato de animação quando viável.

Nesta etapa, não é necessário produzir dez equipamentos.

O objetivo é provar a viabilidade de adaptar o conceito artístico ao sistema modular.

---

# 15. INTEGRAÇÃO COM BLENDER

Investigar duas possibilidades:

**A. Adaptar materiais, silhueta e proporções do manequim V2.2 para aproximá-lo da referência.**

**B. Utilizar a referência como guia para redesenhar sprites sem modificar o modelo 3D.**

Comparar esforço e qualidade.

Não modificar o `.blend` original.

Criar variante experimental.

Não executar retopologia completa sem justificativa.

Não importar automaticamente uma imagem gerada como textura do corpo e considerar a integração concluída.

A integração precisa preservar coerência entre poses.

---

# 16. INTEGRAÇÃO COM BEVY

Reutilizar o exemplo:

`examples/n3_visual_proof.rs`

Ou criar um exemplo isolado específico se houver justificativa técnica.

Permitir comparar:

- Manequim V2.2.
- Pixel automático N3.1.
- Conceito adaptado N3.2.

Exibir claramente qual variante está ativa.

Manter:

- Manifesto válido.
- Sockets.
- Pivôs.
- Camadas.
- Espelhamento.
- Troca de arma.
- Capa.
- Escala inteira.

Não modificar `cargo run`.

Não substituir `assets/animations/player.ron`.

---

# 17. PROVA DE CONSISTÊNCIA ENTRE POSES

Este é um dos principais critérios da N3.2.

Para antecipação, impacto e recuperação:

Verificar:

- Altura do personagem.
- Comprimento dos braços.
- Comprimento das pernas.
- Tamanho da cabeça.
- Comprimento do casaco.
- Forma da capa.
- Cor da armadura.
- Localização da mão.
- Empunhadura.
- Pivô dos pés.

Documentar diferenças.

Uma referência gerada por IA pode parecer excelente isoladamente e falhar como animação.

Não ocultar essas inconsistências.

Se a consistência não for suficiente, não promover os sprites.

---

# 18. ARQUIVOS DE REFERÊNCIA E PROVENIÊNCIA

Guardar os arquivos de forma organizada.

Sugestão:

`art-src/n3-2-art-direction/references/`

`art-src/n3-2-art-direction/working/`

`art-src/n3-2-art-direction/generated/`

`docs/development/n3_2/evidence/`

Para cada imagem, registrar:

- Ferramenta.
- Modelo ou versão, quando disponível.
- Prompt.
- Parâmetros conhecidos.
- Data de geração.
- Origem.
- Se usou imagem de referência.
- Se foi editada.
- Se foi reduzida.
- Se foi pintada manualmente.
- Licença ou termos aplicáveis conhecidos.

Não inventar parâmetros que a ferramenta não fornecer.

Não enviar recursos privados a serviços externos sem autorização.

---

# 19. EVIDÊNCIAS OBRIGATÓRIAS

Publicar, conforme efetivamente produzidos:

1. Painel dos três conceitos.
2. Vista lateral do conceito A.
3. Vista lateral do conceito B.
4. Vista lateral do conceito C.
5. Comparação com o manequim V2.2.
6. Comparação de silhuetas.
7. Comparação de paletas.
8. Comparação em aproximadamente 100 texels.
9. Antecipação do ataque.
10. Impacto.
11. Recuperação.
12. Comparação de consistência das três poses.
13. Versão reduzida automaticamente.
14. Versão adaptada para sprite.
15. Captura do protótipo Bevy.
16. Teste em fundo escuro.
17. Teste de arma separada.
18. Teste da capa.
19. Painel antes/depois N3.1 → N3.2.

Se algum material não puder ser produzido, registrar a ausência.

Não criar evidência artificial para aparentar conclusão.

---

# 20. GATES DE QUALIDADE

A N3.2 deve ser avaliada em três níveis.

## Gate A — Referência artística

Existe uma proposta original, legível e coerente?

A proposta pode ser apresentada ao proprietário mesmo sem sprite final.

## Gate B — Adaptação técnica

A referência pode ser convertida em sprites sem perder sua identidade?

Os materiais continuam distinguíveis?

As proporções continuam consistentes?

## Gate C — Integração

O sprite pode ser reproduzido no Bevy?

A arma continua separada?

A capa continua separada?

Os sockets continuam corretos?

**Aprovar Gate A não aprova automaticamente B ou C.**

---

# 21. TESTES

Executar:

`cargo fmt --check`

`cargo check`

`cargo clippy --all-targets --jobs 2 -- -D warnings`

`cargo test --jobs 2`

Executar o exemplo gráfico.

Testar:

- Atlas.
- Manifesto.
- Sockets.
- Dimensões.
- Pivôs.
- Separação de arma.
- Separação de capa.
- Espelhamento.
- Carregamento dos PNG.
- Compatibilidade com o protótipo N3.

Não alterar testes antigos para esconder regressões.

Não declarar teste gráfico se não houve execução.

---

# 22. DOCUMENTAÇÃO

Criar:

`docs/development/n3_2/`

Arquivos mínimos:

1. `N3_2_AUDITORIA.md`
2. `N3_2_FERRAMENTAS_IA.md`
3. `N3_2_DIRECAO_ARTISTICA.md`
4. `N3_2_CONCEITOS.md`
5. `N3_2_PROMPTS.md`
6. `N3_2_COMPARACAO_V22.md`
7. `N3_2_PIXEL_ART.md`
8. `N3_2_CONSISTENCIA.md`
9. `N3_2_MODULARIDADE.md`
10. `N3_2_INTEGRACAO_BEVY.md`
11. `N3_2_TESTES.md`
12. `N3_2_AVALIACAO.md`
13. `N3_2_RELATORIO_FINAL.md`

Documentar resultados reais.

Não preencher conclusões antes dos experimentos.

---

# 23. PUBLICAÇÃO NO GITHUB

A publicação é obrigatória para trabalho concluído.

Ao finalizar a etapa ou encontrar um bloqueio que impeça sua continuação:

1. Revisar a árvore de trabalho.
2. Confirmar a branch.
3. Verificar alterações preexistentes.
4. Conferir arquivos protegidos.
5. Executar os testes aplicáveis.
6. Criar commits organizados.
7. Publicar documentação e evidências reais.
8. Fazer push para `origin/art/v1.2-protagonist`.
9. Confirmar SHA remoto.
10. Informar os commits.

Se houver bloqueio por falta de ferramenta de geração de imagens, publicar somente a auditoria, os prompts e a documentação do bloqueio.

Não fabricar imagens substitutas.

Não realizar force push.

Não fazer merge ou rebase automático.

Não incluir segredos, arquivos privados ou recursos sem autorização.

---

# 24. RELATÓRIO FINAL

Entregar:

**A. Estado inicial** — Branch, HEAD e alterações.

**B. Ferramentas** — Geração de imagens disponível.

**C. Referências** — Conceitos produzidos.

**D. Prompts** — Prompts utilizados.

**E. Direção artística** — Identidade visual.

**F. Comparação V2.2** — Diferenças e viabilidade.

**G. Pixel art** — Processo de adaptação.

**H. Consistência** — Poses e proporções.

**I. Animação** — Antecipação, impacto e recuperação.

**J. Arma** — Separação e sockets.

