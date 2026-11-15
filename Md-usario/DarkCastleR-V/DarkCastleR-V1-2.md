# DARK CASTLE RPG — MEGA PROMPT V1.1 + V1.2
## Direção artística, referências visuais, Blender e contrato de assets

### PAPEL

Você é o coordenador técnico e artístico do Dark Castle RPG, desenvolvido em Rust 2024 + Bevy 0.20.1.

Nesta execução, sua missão é estabelecer a direção visual original do protagonista e preparar o pipeline profissional de produção artística com Blender.

Você será responsável por coordenar a análise e, dentro dos limites autorizados, produzir scripts Python para Blender, um modelo-base experimental e renderizações comparativas.

**Esta fase não autoriza implementar o controlador de animações no jogo.**

---

# 1. DECISÕES JÁ APROVADAS PELO USUÁRIO

As decisões abaixo são obrigatórias.

## D1 — Conceito do protagonista

**Conceito B: Espadachim aristocrático sombrio.**

Características principais:

- Protagonista masculino de fantasia sombria.
- Silhueta esguia e elegante.
- Proporções aproximadamente de sete cabeças.
- Casaca longa, preferencialmente aberta.
- Capa curta ou meia-capa.
- Gola alta.
- Botas altas.
- Luvas.
- Detalhes metálicos discretos.
- Espada longa e fina, levemente curva.
- Aparência aristocrática, misteriosa e combativa.
- Movimentação visual compatível com combate rápido.
- Design original, sem copiar personagens existentes.

Paleta inicial sugerida:

- Preto azulado.
- Prata fria.
- Vinho escuro.
- Pequenos detalhes em âmbar.

A paleta poderá ser refinada durante a análise.

## D2 — Acabamento visual

**Ainda não existe escolha definitiva entre pixel art detalhada e arte 2D em alta resolução.**

Produzir duas alternativas comparáveis:

A. Pixel art detalhada, com contornos legíveis e controle de pixels.

B. Arte 2D de resolução maior, com acabamento ilustrado ou estilizado e transparências suaves.

As duas alternativas devem representar o mesmo personagem, com a mesma silhueta, roupas, espada, pose e enquadramento.

Não comparar personagens diferentes.

Não escolher o vencedor apenas por preferência pessoal.

## D3 — Câmera

**Preservar inicialmente a câmera 1:1.**

Estudar também zoom ×2.

Não modificar a câmera real do jogo nesta execução.

Preparar comparações visuais da mesma cena e do mesmo personagem nas duas escalas.

Avaliar:

- Legibilidade do personagem.
- Área visível da sala.
- Visibilidade de inimigos.
- Antecipação de ataques.
- Espaço para saltos.
- Visibilidade de plataformas.
- Clareza dos efeitos.
- Impacto sobre exploração e combate.

A decisão definitiva sobre zoom depende de aprovação do usuário.

---

# 2. LEITURA E ESTADO INICIAL

Antes de alterar qualquer arquivo:

1. Leia `CLAUDE.md`.
2. Leia `.claude/rules/`.
3. Leia `docs/ARCHITECTURE.md`.
4. Leia `docs/ROADMAP.md`.
5. Leia `docs/COMBAT_SYSTEM.md`.
6. Leia `docs/MAGIC_SYSTEM.md`.
7. Leia `docs/DECISIONS.md`.
8. Leia `docs/CHARACTER_ATTRIBUTES.md`.
9. Leia `Md-usario/DarkCastleR-V1.md`.
10. Inspecione `src/player/animation.rs`, `src/player/components.rs`, `src/player/state.rs` e `src/world/camera.rs`.
11. Inspecione a estrutura real de assets.
12. Verifique a instalação do Blender e sua versão.
13. Registre o estado do Git.

Confirme que a Fase 4 permanece consolidada.

Não reabra a Fase 4.

Não execute alterações de gameplay.

Não modifique os arquivos em `Md-usario/`.

---

# 3. PROTEÇÕES

Preservar:

- `saves/`.
- `target/verify/`.
- `target/f4-stage/`.
- Todos os commits da Fase 4.
- Arquivos RON de habilidades.
- Física, hitboxes e hurtboxes.
- Sistema de dano.
- Sistema de cooldowns.
- Custos de MP e Hearts.
- Lógica de câmera existente.
- Testes de gameplay.

Proibido:

- `cargo clean`.
- `git clean`.
- `git reset --hard`.
- `git push`.
- Alterar Rust do jogo.
- Alterar arquivos de balanceamento.
- Criar commits sem autorização.
- Criar worktrees ou clones.
- Modificar os briefings do usuário.

Os protótipos artísticos devem ficar isolados do runtime do jogo.

---

# 4. AGENTES

Verifique quais agentes realmente existem.

Utilize, quando apropriado:

### game-architect

Avaliar a integração futura com Bevy.

### code-reviewer

Revisar scripts, organização de arquivos e riscos técnicos.

### visual-director

Se existir, coordenar o design artístico.

### animation-specialist

Se existir, avaliar rigging, poses e animações futuras.

### rendering-specialist

Se existir, avaliar exportação e renderização.

Não invente agentes.

Se os especialistas artísticos não existirem, realize a análise diretamente.

Agentes independentes podem trabalhar em paralelo, mas não devem editar os mesmos arquivos simultaneamente.

---

# 5. REFERÊNCIAS VISUAIS

O projeto poderá utilizar imagens de referência fornecidas pelo usuário.

As imagens servem para orientar:

- Silhueta.
- Roupas.
- Cores.
- Materiais.
- Proporções.
- Cabelo.
- Espada.
- Estilo artístico.
- Iluminação.
- Acabamento.

Não copiar literalmente personagens protegidos por direitos autorais.

Não reproduzir sprites ou modelos de outros jogos.

Não utilizar uma imagem como textura final sem autorização e licença compatível.

## Se houver imagens fornecidas

Analise cada uma.

Identifique:

1. Elementos visuais relevantes.
2. Elementos que devem ser adaptados.
3. Possíveis dificuldades de modelagem.
4. Elementos que precisam de interpretação artística.
5. Riscos de semelhança excessiva com personagens existentes.

Se as referências forem incompatíveis entre si, explique as diferenças e recomende uma composição original.

## Se não houver imagens

Não invente que recebeu referências.

Utilize as diretrizes aprovadas para propor um design inicial.

Antes de consolidar a direção artística, solicite aprovação do usuário.

---

# 6. V1.1 — DIREÇÃO ARTÍSTICA

## 6.1. Design do protagonista

Produza uma especificação visual detalhada.

Defina:

- Altura proporcional.
- Silhueta frontal e lateral.
- Cabeça e rosto.
- Cabelo.
- Casaca.
- Capa.
- Luvas.
- Calças.
- Botas.
- Espada.
- Materiais.
- Cores.
- Ornamentos.
- Distribuição dos detalhes.
- Legibilidade em escala reduzida.

Priorize uma silhueta reconhecível.

Evite excesso de acessórios pequenos que desapareçam na resolução do jogo.

## 6.2. Folha de referência

Planeje uma folha de personagem contendo:

- Vista frontal.
- Vista lateral.
- Vista traseira.
- Pose neutra.
- Pose com espada.
- Paleta.
- Detalhes principais da roupa e da arma.

Se houver recursos para produzir imagens conceituais, crie apenas propostas experimentais, sem tratá-las como arte definitiva.

Não afirme que um modelo está aprovado sem confirmação do usuário.

---

# 7. BLENDER — PIPELINE TÉCNICO

O Claude será responsável por escrever e executar os scripts Python necessários para Blender, caso a ferramenta esteja instalada e acessível.

Utilizar `bpy` e APIs compatíveis com a versão real instalada.

Não presumir que Blender está disponível.

## 7.1. Estrutura de produção

Propor uma organização semelhante a:

art-src/
  player/
    blender/
    scripts/
    references/
    renders/
    comparisons/

Essa estrutura é uma proposta.

Antes de criá-la, confira convenções existentes e possíveis arquivos conflitantes.

## 7.2. Modelo-base experimental

Criar um modelo 3D estilizado original do espadachim sombrio, com:

- Corpo proporcional.
- Cabeça.
- Cabelo.
- Casaca.
- Capa curta.
- Botas.
- Luvas.
- Espada.
- Materiais básicos.
- Pose neutra.

A primeira versão deve priorizar silhueta, proporção e legibilidade, não detalhamento excessivo.

Não tentar construir um modelo hiper-realista.

Não criar dependência de serviços pagos.

Não utilizar modelos externos sem verificar licenças.

## 7.3. Scripts Blender

Separar responsabilidades:

- Criação de geometria.
- Materiais.
- Configuração de câmera.
- Iluminação.
- Renderização.
- Exportação de quadros.
- Geração de prévias.

Os scripts devem ser reproduzíveis.

Evitar operações destrutivas fora da pasta experimental.

Registrar a versão do Blender.

## 7.4. Rigging

Avaliar a criação de um rig simples para:

- Tronco.
- Cabeça.
- Braços.
- Pernas.
- Espada.
- Capa.

Nesta fase, um rig funcional completo não é obrigatório.

Se o rigging aumentar significativamente o escopo, limitar-se à avaliação técnica e registrar a implementação como tarefa futura.

---

# 8. COMPARAÇÃO DE ACABAMENTOS

Criar duas prévias do mesmo modelo.

## Alternativa A — Pixel art detalhada

Estudar:

- Renderização em baixa resolução.
- Paleta reduzida.
- Silhueta.
- Contornos.
- Limpeza de pixels.
- Filtro nearest.
- Consistência entre quadros.
- Legibilidade em movimento.

## Alternativa B — 2D em alta resolução

Estudar:

- Renderização com maior quantidade de detalhes.
- Bordas suaves.
- Texturas estilizadas.
- Transparência.
- Filtro linear.
- Escala na tela.
- Legibilidade.
- Custo de memória.

## Comparação obrigatória

Avaliar as duas alternativas considerando:

1. Qualidade visual.
2. Leitura em movimento.
3. Compatibilidade com o cenário gótico.
4. Complexidade de produção.
5. Facilidade de criar novas animações.
6. Consistência dos sprites.
7. Consumo de memória estimado, sem inventar medições.
8. Manutenção futura.
9. Possibilidade de trocar armas e roupas.
10. Compatibilidade com Bevy.

Não declarar um vencedor definitivo sem aprovação.

---

# 9. TESTES DE ESCALA E CÂMERA

Comparar:

- Câmera 1:1.
- Câmera com zoom ×2.

Utilizar a mesma cena, o mesmo personagem e o mesmo enquadramento de referência.

Se possível, basear a composição nas dimensões reais da sala `entry_hall`.

Não inventar um cenário que represente falsamente o mapa existente.

Se não houver meios de reproduzir fielmente a sala, utilizar uma composição experimental claramente identificada.

Não alterar `src/world/camera.rs`.

Entregar imagens ou capturas que permitam comparar as duas escalas.

---

# 10. V1.2 — CONTRATO DE ASSETS

Definir um contrato técnico preliminar para os sprites futuros.

## 10.1. Convenções

Especificar:

- Estrutura de diretórios.
- Nomenclatura.
- Formato PNG.
- Transparência.
- Espaço de cor.
- Resolução dos quadros.
- Pivô.
- Escala.
- Orientação.
- Espelhamento horizontal.
- Padding.
- Organização de atlases.
- Identificadores de animação.
- Metadados.
- Versionamento.

Não congelar a resolução final enquanto a comparação artística estiver pendente.

## 10.2. Atlases

Comparar:

- Atlas único.
- Atlas por grupo de animações.
- Atlases separados conforme necessidade.

Avaliar desperdício de espaço e facilidade de manutenção.

Não impor uma grade única para todas as animações sem justificar.

## 10.3. Manifesto RON

Planejar um manifesto de animações para o protagonista.

O manifesto poderá descrever:

- Identificador.
- Arquivo da textura.
- Quadros.
- FPS.
- Modo de reprodução.
- Pivô.
- Direção.
- Fallback.

Não alterar os RON existentes.

Não integrar o manifesto ao jogo nesta fase.

## 10.4. Fonte única de verdade

A documentação de V1.2 deverá ser a referência principal para formatos, pivôs, nomes e organização dos assets.

Evitar regras contraditórias entre documentos.

---

# 11. TESTES E VALIDAÇÃO

## 11.1. Scripts Blender

Verificar:

- Execução sem erros.
- Criação dos objetos esperados.
- Materiais.
- Câmera.
- Exportação.
- Transparência.
- Dimensões dos renders.
- Reprodutibilidade.

## 11.2. Prévia visual

Verificar:

- Silhueta.
- Legibilidade.
- Proporções.
- Coerência das roupas.
- Espada visível.
- Consistência de escala.
- Diferenças reais entre os dois acabamentos.

## 11.3. Contrato de assets

Verificar:

- Nomes consistentes.
- Ausência de colisões de nomes.
- Formatos definidos.
- Pivôs coerentes.
- Metadados suficientes.
- Compatibilidade planejada com Bevy.

## 11.4. Integridade do projeto

Confirmar:

- Nenhum Rust alterado.
- Nenhum RON de gameplay alterado.
- Nenhum save alterado.
- Nenhum arquivo protegido alterado.
- Nenhum commit criado.
- Nenhum push executado.

---

# 12. CRITÉRIOS DE CONCLUSÃO

## V1.1

Considerar a etapa pronta para aprovação quando houver:

1. Design original documentado.
2. Silhueta definida.
3. Paleta preliminar.
4. Especificação de roupas e espada.
5. Comparação visual entre pixel art e alta resolução.
6. Comparação entre câmera 1:1 e zoom ×2.
7. Modelo-base experimental, se Blender estiver disponível.
8. Pendências artísticas explicitadas.

Não considerar V1.1 definitivamente aprovada sem a escolha do usuário.

## V1.2

Considerar a etapa pronta para aprovação quando houver:

1. Estrutura de assets proposta.
2. Formatos e convenções.
3. Estratégia de atlas.
4. Contrato de pivô e alinhamento.
5. Manifesto RON proposto.
6. Pipeline de exportação reproduzível.
7. Dependências identificadas.
8. Itens condicionais claramente marcados.

Não congelar decisões dependentes do acabamento visual ainda não aprovado.

---

# 13. RELATÓRIO FINAL

Entregar:

## A. Estado inicial
Branch, HEAD e integridade.

## B. Agentes
Agentes existentes e realmente utilizados.

## C. Direção artística
Descrição do protagonista original.

## D. Referências
Imagens recebidas e como foram interpretadas.

## E. Blender
Versão, scripts, modelo experimental e limitações.

## F. Comparação artística
Pixel art versus alta resolução.

## G. Câmera
1:1 versus zoom ×2.

## H. Contrato de assets
Estrutura, formatos, pivôs, atlases e manifesto.

## I. Arquivos
Lista exata de arquivos criados e modificados.

## J. Testes
Validações realmente executadas.

## K. Pendências
Decisões que exigem minha aprovação.

## L. Próximo passo
Proposta de V1.3 após aprovação.

---

# 14. CONDIÇÃO FINAL

Não implemente animações no Bevy.

Não altere gameplay.

Não modifique o combate.

Não faça commits.

Não faça push.

Não declare que um protótipo artístico está aprovado sem minha confirmação.

Se Blender não estiver instalado, informe a limitação e apresente um plano executável sem inventar resultados.

**Sua missão é produzir uma base artística e técnica verificável para o protagonista, comparar dois acabamentos e preparar o contrato de assets.**

Ao concluir, aguarde minha aprovação.