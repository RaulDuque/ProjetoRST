# DARK CASTLE RPG — MEGA PROMPT V1.2-G
## Correção da Ombreira, Testes de Animação e Preparação do Pipeline de Sprites

### 0. MISSÃO

Você é o responsável técnico pelo pipeline artístico do Dark Castle RPG, desenvolvido em Rust 2024 + Bevy 0.20.1, com Blender 5.2.4 LTS e Python `bpy`.

Estamos desenvolvendo o protagonista do jogo: **Conceito B — Espadachim Aristocrático Sombrio**.

As etapas anteriores produziram:

- v004: modelo 3D procedural refinado.
- v005: primeiro rig experimental.
- v006: rig estabilizado, com 43 ossos, 36 de deformação e pesos normalizados.
- Testes de poses, estresse e movimento.
- Pipeline de renders e comparações de acabamento.
- Contrato de assets para futura integração com Bevy.

O relatório da V1.2-F indica que o v006 eliminou os problemas Médios e Altos nos testes definidos.

Restaram três questões:

1. Instabilidade preexistente do modificador Solidify da ombreira, com deslocamentos de até 3,5 mm em 217 vértices.
2. Aparência rígida da casaca no agachamento profundo T10.
3. Perda de volume de aproximadamente 6,2% no tornozelo durante a pose Death, sem colapso visual observado.

**Decisão do projeto: aceitar o v006 como base técnica provisória.**

A V1.2-G deve corrigir a ombreira e validar o comportamento do personagem em animações contínuas.

Não reconstrua o protagonista e não faça refinamentos cosméticos extensos.

Não inicie a integração com Bevy.

---

## 1. ESTADO INICIAL E AUDITORIA

Antes de qualquer alteração:

1. Verifique branch, HEAD e status do Git.
2. Identifique arquivos staged, modificados e não rastreados.
3. Preserve as alterações preexistentes.
4. Leia a documentação artística e de rig.
5. Inspecione os scripts e relatórios do v006.
6. Confirme a existência e integridade dos modelos anteriores.
7. Verifique as durações reais das ações no código Rust, somente leitura.
8. Registre os hashes dos arquivos protegidos.

Arquivos prioritários:

- `art-src/player/RIG_SPECIFICATION.md`
- `art-src/player/RIG_VALIDATION_V005.md`
- `art-src/player/RIG_VALIDATION_V006.md`
- `docs/ART_DIRECTION.md`
- `docs/ASSET_CONTRACT.md`
- `art-src/README.md`

Inspecione também:

- Scripts de construção do rig v006.
- Scripts de validação.
- Scripts de render.
- Scripts de comparação.
- Arquivos `.blend` do v004, v005 e v006.

Não presuma que o HEAD permanece em `204f965`.

Se houver divergências, documente o estado real.

---

## 2. RESTRIÇÕES ABSOLUTAS

Não é permitido:

- Modificar Rust.
- Modificar RON de gameplay.
- Alterar a câmera do jogo.
- Alterar física, dano, hitboxes ou cooldowns.
- Alterar a duração das ações.
- Modificar saves.
- Modificar `Md-usario/`.
- Sobrescrever modelos anteriores.
- Alterar scripts antigos para produzir o novo resultado.
- Executar `git clean`, `git reset --hard` ou `cargo clean`.
- Modificar `.cargo/config.toml`.
- Alterar `target/verify/` ou `target/f4-stage/`.
- Instalar dependências sem autorização.
- Desabilitar o safe mode do MCP.
- Usar serviços externos de geração 3D.
- Enviar assets a terceiros.
- Criar commits.
- Executar push.
- Iniciar a V1.3 automaticamente.

Todo o trabalho deve ficar isolado no pipeline artístico.

---

## 3. BLENDER E MCP

Verifique:

- Blender instalado e versão real.
- Funcionamento do EEVEE.
- Disponibilidade do MCP.
- Estado do complemento.
- Existência de conexão em `127.0.0.1:9876`.
- Integridade do pipeline em background.

Se o MCP estiver conectado, utilize suas ferramentas para inspeção.

Se estiver offline, continue usando:

`blender --background --factory-startup`

Não modifique a cena interativa do usuário.

Não desabilite o safe mode.

Não dependa de serviços externos.

---

## 4. ESTRATÉGIA DE VERSIONAMENTO

O v006 deve permanecer intacto.

Crie uma versão experimental independente, chamada v007, **somente se for necessário salvar alterações persistentes no modelo ou no rig**.

Sugestões de caminhos, sujeitos à estrutura real do projeto:

`art-src/player/blender/swordsman_rig_v007.blend`

`art-src/player/scripts/bpy/build_swordsman_rig_v007.py`

`art-src/player/scripts/bpy/validate_rig_v007.py`

Se a ombreira puder ser corrigida sem modificar o rig, preserve os 43 ossos e a hierarquia existente.

Não incremente a versão apenas por conveniência.

Os testes de animação devem utilizar uma cópia independente do modelo, nunca modificar o arquivo-fonte v006.

Registre os hashes antes e depois.

---

## 5. PRIORIDADE MÁXIMA — INSTABILIDADE DA OMBREIRA

O relatório V1.2-F identificou um problema preexistente:

- Modificador Solidify instável.
- 217 vértices afetados.
- Deslocamento máximo de aproximadamente 3,5 mm.
- Possibilidade de saltos de geometria ou sombreamento entre quadros.

Sua primeira tarefa é reproduzir o problema de maneira controlada.

### 5.1 Diagnóstico

Investigue:

1. Ordem dos modificadores.
2. Normais da malha.
3. Geometria não manifold.
4. Faces degeneradas.
5. Espessura do Solidify.
6. Interferência entre Armature, Solidify e Subdivision.
7. Pesos dos vértices.
8. Transformações não aplicadas.
9. Mudanças de topologia.
10. Dependências da avaliação do Blender.

Compare a mesma pose e o mesmo frame sob condições equivalentes.

Determine se o salto ocorre na geometria avaliada, nas normais, na iluminação ou na atualização do dependency graph.

Não conclua que a causa é o Solidify apenas porque o problema aparece depois dele.

### 5.2 Correção

Prefira a solução menos invasiva.

Considere:

- Corrigir normais.
- Corrigir faces degeneradas.
- Ajustar a ordem dos modificadores.
- Corrigir a espessura.
- Converter a ombreira em uma malha de espessura explícita.
- Ajustar pesos.
- Separar partes rígidas, se necessário.

Não remodele o ombro inteiro sem necessidade.

Não modifique a silhueta geral do personagem.

Não introduza novos ossos sem justificativa.

### 5.3 Critério de aceitação

O problema deve ser testado:

- Em repouso.
- Durante caminhada.
- Durante corrida.
- Durante ataque.
- Durante esquiva.
- Com o braço elevado.
- Com rotação do tronco.

A ombreira não deve apresentar saltos descontínuos de geometria ou sombreado causados pela avaliação do modificador.

Compare os resultados com o v006 usando medições e renders.

---

## 6. ANIMAÇÕES TÉCNICAS EXPERIMENTAIS

Após corrigir ou diagnosticar completamente a ombreira, produza três animações de teste.

Estas animações são experimentais.

Não representam a animação artística definitiva.

### 6.1 WALK — Caminhada

Crie um ciclo de caminhada com:

- Movimento alternado de pernas.
- Movimento coordenado dos braços.
- Transferência de peso.
- Leve movimento vertical do tronco.
- Movimento secundário da casaca.
- Movimento secundário da meia-capa.
- Espada acompanhando a mão.
- Pés sem deslizamento excessivo.

O ciclo deve começar e terminar em poses compatíveis para repetição.

Avalie:

- Continuidade entre último e primeiro quadro.
- Interseções da casaca.
- Interseções da capa.
- Deformação dos tornozelos.
- Estabilidade da ombreira.
- Leitura da silhueta lateral.

### 6.2 BASIC ATTACK — Ataque básico

Utilize a duração real do jogo:

**0,28 segundo.**

Crie uma animação de teste com aproximadamente sete quadros de amostragem a 24 fps, ajustando a representação temporal para corresponder à duração exata da ação.

Importante:

**A hitbox do jogo começa no início da ação.**

Portanto:

- O primeiro quadro deve apresentar uma pose ofensiva coerente com o impacto imediato.
- Não coloque uma longa preparação antes do golpe.
- O movimento posterior pode mostrar continuidade, arrasto, recuperação e movimento da casaca.
- A espada deve permanecer presa à mão.
- A animação deve ser interrompível sem deixar peças em posições inválidas.

Não altere a hitbox nem a duração do gameplay.

Não crie combos ou novos ataques.

### 6.3 DODGE — Esquiva

Utilize a duração real do jogo:

**0,28 segundo.**

Produza uma animação técnica de esquiva com:

- Deslocamento corporal coerente.
- Flexão das pernas.
- Inclinação do tronco.
- Movimento secundário da casaca.
- Movimento secundário da capa.
- Recuperação para uma pose válida.
- Estabilidade da espada e da ombreira.

Não introduza deslocamento adicional no gameplay.

Se o movimento no Blender incluir translação da raiz, documente como ela será removida ou normalizada na exportação de sprites, para que o Bevy continue controlando o deslocamento real.

Não altere invulnerabilidade, colisão ou física.

---

## 7. QUALIDADE DAS ANIMAÇÕES

Verifique:

- Movimento contínuo.
- Ausência de saltos entre quadros.
- Ausência de alterações inesperadas de escala.
- Ausência de objetos desconectados.
- Ausência de atravessamentos graves.
- Ausência de inversões de normais.
- Ausência de deformações bruscas.
- Continuidade de materiais.
- Estabilidade da espada.
- Estabilidade da ombreira.
- Continuidade do cabelo.
- Comportamento da gola.

Não aceite uma animação apenas porque os keyframes existem.

Inspecione visualmente os quadros intermediários.

Aplique os mesmos critérios de classificação utilizados nos relatórios anteriores:

- OK.
- Cosmético.
- Baixo.
- Médio.
- Alto.
- Crítico.

Registre a classificação máxima de cada animação e os frames afetados.

---

## 8. TESTE DE 12 FPS × 24 FPS

Produza duas versões de apresentação das animações.

**Versão A — 12 fps**

Objetivo:

- Avaliar estilo mais econômico.
- Verificar legibilidade dos movimentos.
- Identificar perda de poses importantes.
- Avaliar tamanho dos spritesheets.

**Versão B — 24 fps**

Objetivo:

- Avaliar fluidez.
- Preservar movimentos rápidos.
- Avaliar combate e esquiva.
- Verificar continuidade do tecido.

### Diretriz provisória

Adote **24 fps como taxa de produção e amostragem de referência**.

A versão de 12 fps deve ser uma alternativa derivada para comparação.

Não estabeleça uma taxa definitiva de reprodução no Bevy.

Não force os clipes de 0,28 segundo a terem duração diferente apenas para caberem em um número inteiro de quadros.

Documente a regra de amostragem, os timestamps e a duração efetiva do clipe.

Evite duplicar o último quadro em animações cíclicas se ele for idêntico ao primeiro.

---

## 9. TESTE DE ACABAMENTO A/B/C

Para as três animações, compare:

**A — Pixel art sombreada**

- Filtro nearest.
- Alfa binário.
- Verificação de pixels isolados.
- Verificação de contorno.
- Verificação de legibilidade da espada.

**B — Alta resolução**

- Preservação de detalhes.
- Bordas suaves.
- Tratamento correto do alfa.
- Avaliação de borramento na escala 1:1.

**C — Cores chapadas**

- Paleta consistente.
- Contraste entre casaca, capa e fundo.
- Silhueta legível.
- Espada claramente visível.

Mantenha as três alternativas.

Não declare um acabamento vencedor sem evidência visual suficiente.

---

## 10. TESTE DE CÂMERA

Compare externamente:

- 1:1.
- ×1,5.
- ×2.

Não altere a câmera do jogo.

Use as mesmas condições de enquadramento e escala dos estudos anteriores.

Avalie:

- Visibilidade da espada.
- Movimento da casaca.
- Movimento da meia-capa.
- Legibilidade da esquiva.
- Legibilidade do ataque.
- Espaço visível ao redor do personagem.
- Impacto na leitura do cenário.

Documente se alguma opção compromete a antecipação de ameaças ou a visibilidade das plataformas.

As decisões D2 e D3 continuam pendentes.

---

## 11. TESTES DA CASACA E DO TORNOZELO

Não amplie esta missão para uma reconstrução dessas peças.

### Casaca

Avalie:

- Caminhada.
- Ataque.
- Esquiva.
- Agachamento profundo T10.

Se a casaca permanecer rígida apenas em T10 e isso não comprometer as três animações, registre como limitação aceita provisoriamente.

Se o problema aparecer nas animações reais, proponha uma correção localizada.

### Tornozelo

Avalie:

- Transferência de peso.
- Contato dos pés com o chão.
- Flexão durante caminhada.
- Flexão durante esquiva.
- Pose Death.

Preserve a geometria da sola, salto e biqueira.

A perda de volume de 6,2% é uma referência do v006, não uma autorização para piorar a deformação.

Se não houver colapso visível, não sacrifique a estabilidade da bota para melhorar apenas uma métrica.

---

## 12. EXPORTAÇÃO EXPERIMENTAL DE SPRITES

Prepare um teste do pipeline de exportação.

Não integre os sprites ao jogo.

Utilize o contrato existente:

- PNG RGBA.
- Espaço de cor sRGB.
- Alfa reto.
- Célula de movimento 64×64.
- Pivô de movimento (32, 63).
- Célula de combate 128×96.
- Pivô de combate (40, 95).

A animação Walk deve utilizar a célula de movimento.

Basic Attack e Dodge devem utilizar células adequadas ao enquadramento, respeitando o contrato. Se o contrato classificar Dodge de maneira diferente, siga o documento real e registre a escolha.

Garanta:

1. Personagem corretamente ancorado.
2. Ausência de cortes na espada.
3. Ausência de cortes na casaca.
4. Transparência correta.
5. Quadros em sequência.
6. Padding consistente.
7. Nomes determinísticos.
8. Ausência de quadros duplicados acidentais.
9. Reprodução temporal correta.
10. Metadados suficientes para futura integração.

Se o personagem ou a espada excederem a célula prevista, **não corte silenciosamente a imagem**.

Registre a incompatibilidade e proponha uma solução sem modificar o contrato automaticamente.

---

## 13. SPRITESHEETS E PRÉVIAS

Crie uma pasta específica para esta etapa:

`art-src/player/comparisons/v1_2g_animation_tests/`

Sugestões de entregáveis:

- `shoulder_before_after.png`
- `walk_contact_sheet.png`
- `attack_contact_sheet.png`
- `dodge_contact_sheet.png`
- `walk_12_vs_24.png`
- `attack_12_vs_24.png`
- `dodge_12_vs_24.png`
- `finish_abc_animation_comparison.png`
- `game_scale_animation_comparison.png`
- `animation_validation_report.json`

Gere também spritesheets experimentais para as três animações, separados por acabamento quando necessário.

Se possível, produza prévias animadas em GIF ou vídeo local, sem serviços externos.

Essas prévias devem ser geradas exclusivamente a partir dos renders locais.

Não altere os arquivos de comparação das versões anteriores.

---

## 14. VALIDAÇÕES AUTOMÁTICAS

Implemente ou adapte verificações para:

1. Integridade do modelo de origem.
2. Integridade da armature.
3. Pesos normalizados.
4. Espada presa à mão.
5. Continuidade da ombreira.
6. Continuidade de deformação entre frames.
7. Ausência de deslocamentos anormais.
8. Continuidade do ciclo Walk.
9. Duração correta de Basic Attack.
10. Duração correta de Dodge.
11. Ausência de cortes nos sprites.
12. Dimensões corretas.
13. Pivôs corretos.
14. Transparência válida.
15. Consistência de nomenclatura.
16. Compatibilidade com o contrato de assets.
17. Reprodutibilidade dos scripts.
18. Preservação dos modelos anteriores.
19. Preservação do gameplay.
20. Preservação dos saves.

Não declare que um teste passou se ele não foi executado.

Diferencie verificações automáticas de avaliações visuais.

---

## 15. REPRODUTIBILIDADE

Execute o pipeline pelo menos duas vezes quando viável.

Compare:

- Estrutura do modelo.
- Poses.
- Keyframes.
- Metadados.
- Sprites processados.
- Spritesheets.
- Relatórios.

Registre diferenças.

Se os renders EEVEE apresentarem ruído de amostragem, não afirme identidade byte a byte dos masters sem confirmação.

O pipeline de pós-processamento deve produzir resultados determinísticos quando as entradas forem idênticas.

Não altere o Python do sistema.

Não instale pacotes sem autorização.

Preserve o fallback existente de Pillow se ainda for necessário.

---

## 16. BACKUP E VERSIONAMENTO

Faça uma auditoria dos arquivos `.blend`.

Identifique:

- Arquivos indispensáveis.
- Tamanho de cada arquivo.
- Arquivos ignorados pelo Git.
- Scripts necessários para reconstrução.
- Dependências externas.
- Arquivos intermediários descartáveis.
- Arquivos que representam marcos importantes.

Prepare uma proposta de versionamento para:

- Modelo-base aprovado.
- Rig aprovado.
- Animações-fonte.
- Sprites exportados.
- Scripts de geração.

Considere Git LFS para os arquivos `.blend` aprovados.

Entretanto:

**Não configure Git LFS, não modifique `.gitignore`, não apague backups e não faça commits sem autorização.**

Se possível, informe os comandos necessários para criar um backup externo verificável, mas não execute cópias fora do projeto sem autorização.

---

## 17. DOCUMENTAÇÃO

Crie um relatório específico:

`art-src/player/ANIMATION_VALIDATION_V1_2G.md`

Inclua:

- Estado inicial.
- Blender e MCP.
- Problema da ombreira.
- Causa identificada.
- Correção implementada.
- Arquivos do modelo.
- Animações criadas.
- Durações.
- FPS de produção.
- FPS de exportação.
- Testes de deformação.
- Comparação A/B/C.
- Comparação de câmera.
- Testes de spritesheets.
- Resultados automáticos.
- Resultados visuais.
- Problemas restantes.
- Riscos para V1.3.
- Arquivos criados e modificados.
- Estado final do Git.

Atualize apenas as seções estritamente necessárias da documentação existente.

Não declare D2 ou D3 como decididas.

---

## 18. CRITÉRIOS DE ACEITAÇÃO

A V1.2-G será considerada concluída tecnicamente quando:

- [ ] O v006 estiver preservado.
- [ ] A instabilidade da ombreira tiver sido corrigida ou completamente diagnosticada.
- [ ] Não houver saltos visuais graves entre frames.
- [ ] Walk estiver funcionando em ciclo.
- [ ] Basic Attack respeitar 0,28 segundo.
- [ ] Dodge respeitar 0,28 segundo.
- [ ] A espada acompanhar corretamente a mão.
- [ ] A casaca funcionar nos movimentos principais.
- [ ] A meia-capa funcionar nos movimentos principais.
- [ ] O tornozelo não apresentar colapso visual.
- [ ] Os testes de 12 e 24 fps forem executados.
- [ ] Os acabamentos A/B/C forem comparados.
- [ ] As escalas 1:1, ×1,5 e ×2 forem comparadas.
- [ ] Os spritesheets experimentais forem gerados.
- [ ] Os pivôs e dimensões forem validados.
- [ ] O pipeline for reproduzível.
- [ ] O gameplay permanecer intacto.
- [ ] Os saves permanecerem intactos.
- [ ] Nenhum modelo anterior for sobrescrito.
- [ ] Nenhum commit ou push for realizado.

Se algum critério não for atendido, registre como pendência.

Não declare aprovação integral com falhas importantes sem justificativa.

---

## 19. RELATÓRIO FINAL

Apresente o relatório nesta ordem:

**A. Estado inicial**

Branch, HEAD, alterações preexistentes.

**B. Ambiente**

Blender, EEVEE, MCP e dependências.

**C. Ombreira**

Causa, correção e evidências.

**D. Modelo utilizado**

v006 preservado e eventual v007.

**E. Walk**

Duração, frames, continuidade e problemas.

**F. Basic Attack**

Duração, sincronização e deformações.

**G. Dodge**

Duração, movimento e deformações.

**H. FPS**

Comparação 12 × 24.

**I. Acabamentos**

A, B e C.

**J. Câmera**

1:1, ×1,5 e ×2.

**K. Casaca, capa e tornozelo**

Problemas restantes.

**L. Exportação**

Spritesheets, pivôs e dimensões.

**M. Validações**

Resultados automáticos e visuais.

**N. Reprodutibilidade**

Resultados das execuções repetidas.

**O. Arquivos**

Criados, modificados e preservados.

**P. Git e backup**

Estado final e proposta de versionamento.

**Q. Critérios de aceitação**

Atendidos, parciais e não atendidos.

**R. Recomendações para V1.3**

O que já pode ser integrado e o que ainda precisa de aprovação.

---

## 20. REGRA FINAL

Esta missão termina com a correção da ombreira e a validação das três animações experimentais.

Não implemente o controlador de animações no Bevy.

Não altere Rust ou RON de gameplay.

Não modifique a câmera do jogo.

Não escolha D2 ou D3 automaticamente.

Não gere toda a biblioteca de animações.

Não faça commits.

Não faça push.

**Apresente os resultados e aguarde minha autorização explícita para iniciar a V1.3.**

A prioridade é garantir que o personagem possa ser animado e exportado com qualidade consistente, sem multiplicar defeitos técnicos durante a produção artística.