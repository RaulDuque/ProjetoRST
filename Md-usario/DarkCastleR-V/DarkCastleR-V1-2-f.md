# DARK CASTLE RPG — MEGA PROMPT V1.2-F
## Estabilização do Rig v006, Correção de Deformações e Validação para Animações

### 0. PAPEL E OBJETIVO

Você é o responsável técnico pela produção artística 3D do **Dark Castle RPG**, desenvolvido em Rust + Bevy, utilizando Blender 5.2.4 LTS, Python `bpy` e, quando disponível, o MCP `mcp-for-blender`.

Estamos concluindo a fase experimental de construção do protagonista, seguindo o **Conceito B — Espadachim Aristocrático Sombrio**.

O modelo v004 foi criado e refinado. Na V1.2-E foi produzido o rig experimental v005, com:

- 42 ossos, sendo 35 de deformação.
- 38.718 vértices com pesos normalizados.
- No máximo quatro influências por vértice.
- IK nas pernas e na mão da espada.
- Espada rigidamente vinculada à mão direita.
- Cadeias de ossos para casaca e meia-capa.
- 13 poses obrigatórias avaliadas.
- Testes de caminhada, ataque e esquiva.
- Arquivos `.blend` reabertos e validados.
- Nenhuma alteração no gameplay ou nos saves.

**O v005 está aprovado como base técnica experimental, mas não como rig definitivo.**

Sua missão agora é produzir o **v006**, corrigindo os problemas de deformação identificados, sem reconstruir desnecessariamente o personagem.

A V1.2-F é uma fase de estabilização, não de criação de novas mecânicas, nem de implementação de animações no jogo.

---

### 1. INSPEÇÃO OBRIGATÓRIA ANTES DE MODIFICAR ARQUIVOS

Antes de executar qualquer alteração:

1. Identifique a branch e o HEAD reais.
2. Execute `git status --short` e registre os arquivos staged, modificados e não rastreados.
3. Confira o histórico recente, sem presumir que o HEAD ainda seja `204f965`.
4. Localize e leia:
   - `art-src/player/RIG_SPECIFICATION.md`
   - `art-src/player/RIG_VALIDATION_V005.md`
   - `art-src/README.md`
   - `docs/ART_DIRECTION.md`
   - `docs/ASSET_CONTRACT.md`
5. Inspecione o pacote `art-src/player/scripts/bpy/swordsman_rig_v005/`.
6. Identifique os scripts de construção e validação do v005.
7. Localize os arquivos `.blend` do v004 e v005.
8. Inspecione as folhas e os relatórios de validação da V1.2-E.
9. Verifique as dependências Python utilizadas pelos scripts.
10. Registre os hashes dos arquivos que devem permanecer intactos.

Não suponha que os caminhos, nomes ou estados descritos neste briefing estejam atualizados. Verifique tudo no repositório real.

**Preserve qualquer alteração preexistente do usuário. Não misture seu trabalho com alterações que já estavam staged.**

Se algum arquivo necessário estiver ausente ou inconsistente, investigue antes de continuar.

---

### 2. RESTRIÇÕES ABSOLUTAS

É proibido:

- Alterar Rust ou código de gameplay.
- Alterar arquivos RON de habilidades, inimigos, combate ou câmera.
- Alterar a câmera do jogo.
- Alterar hitboxes, dano, física ou cooldowns.
- Modificar saves ou seus backups.
- Modificar arquivos de `Md-usario/`.
- Sobrescrever os modelos v001, v002, v003, v004 ou v005.
- Alterar os scripts originais do v005 para implementar o v006.
- Executar `git clean`, `git reset --hard` ou `cargo clean`.
- Apagar arquivos experimentais anteriores.
- Alterar `.cargo/config.toml`.
- Modificar `target/verify/` ou `target/f4-stage/`.
- Instalar dependências sem autorização.
- Fazer commits ou push.
- Desabilitar o safe mode do MCP.
- Utilizar serviços externos de geração 3D.
- Enviar imagens, modelos ou outros assets a serviços externos sem autorização.

A Fase 4 do gameplay está encerrada e não deve ser reaberta.

Todo o trabalho desta missão deve permanecer isolado no pipeline artístico.

---

### 3. VERIFICAÇÃO DO BLENDER E MCP

Verifique:

- Versão real do Blender.
- Disponibilidade do EEVEE.
- Existência e integridade dos arquivos `.blend`.
- Estado do complemento MCP.
- Disponibilidade das ferramentas do Blender na sessão.
- Funcionamento do pipeline `blender --background --factory-startup`.

Se o Blender estiver aberto e o MCP conectado, utilize as ferramentas de leitura para inspecionar o ambiente.

Se o MCP estiver offline, **não bloqueie o trabalho**. Utilize o Blender em processo separado pelo terminal.

Não altere a cena interativa do usuário.

Não desabilite o safe mode.

Não tente iniciar ou modificar serviços privilegiados sem necessidade.

Registre no relatório qual método foi utilizado.

---

### 4. ESTRATÉGIA DE VERSIONAMENTO DO MODELO

Crie uma versão experimental independente:

`art-src/player/blender/swordsman_rig_v006.blend`

Se houver arquivo de testes separado, utilize:

`art-src/player/blender/swordsman_rig_v006_anim_tests.blend`

Crie novos scripts específicos do v006, seguindo a organização existente no projeto.

Sugestão, caso seja compatível com a estrutura atual:

`art-src/player/scripts/bpy/build_swordsman_rig_v006.py`

`art-src/player/scripts/bpy/validate_rig_v006.py`

`art-src/player/scripts/bpy/swordsman_rig_v006/`

Reutilize funções e módulos estáveis do v005 quando apropriado, sem modificá-los.

O build deve:

1. Abrir o v005 como fonte, sem sobrescrevê-lo.
2. Validar a integridade da geometria e da armature de origem.
3. Aplicar as correções de maneira reproduzível.
4. Salvar o resultado exclusivamente como v006.
5. Recusar sobrescrita de arquivos de versões anteriores.
6. Gerar um relatório estruturado com as modificações.

**Não aumente indiscriminadamente a complexidade do rig.** Cada novo osso, restrição ou grupo de pesos deve ter uma justificativa técnica.

---

### 5. CORREÇÃO PRIORITÁRIA — MEIA-CAPA

O v005 apresenta atravessamentos da meia-capa contra o braço esquerdo, a casaca e o tronco.

Esses problemas aparecem principalmente em:

- Jump Fall.
- Crescent Slash.
- Dodge.
- Hurt.
- Movimento de ataque, quadros 2 e 3.
- Movimento de esquiva, quadro 5.
- Teste de estresse T11.

Investigue primeiro as causas reais, utilizando o relatório de validação e inspeção visual.

Avalie:

1. Pesos de `upper_arm.L` na região superior da capa.
2. Influência das cadeias atuais de ossos.
3. Distribuição dos pesos ao longo das dobras.
4. Necessidade de corrigir os pivôs.
5. Possíveis interferências entre capa, ombreira e manga.
6. Movimento da capa quando o braço esquerdo sobe ou recua.
7. Comportamento quando o tronco gira.

Implemente a menor correção que resolva os atravessamentos relevantes.

Não substitua automaticamente a capa por simulação física.

Não faça a capa acompanhar integralmente o braço se isso provocar interseção com o tronco.

Evite deformações que façam o tecido parecer rígido ou esticado artificialmente.

**Resultado esperado:** meia-capa estável, com movimento coerente e sem falhas Altas nas poses e testes definidos.

---

### 6. CORREÇÃO PRIORITÁRIA — CASACA

O v005 apresenta problemas na casaca durante movimentos amplos.

As situações principais são:

- Death.
- Agachamento profundo T10.
- Movimentos extremos das pernas.
- Transições rápidas de esquiva.

A casaca atual utiliza quatro cadeias de dois ossos.

Avalie a proposta de utilizar três ossos por painel, mas não a implemente sem comparar tecnicamente com a estrutura atual.

Investigue:

1. Distribuição dos pesos nas barras.
2. Deformação das caudas dianteiras.
3. Deformação das caudas traseiras.
4. Interseção com coxas e joelhos.
5. Comportamento da fenda traseira.
6. Preservação do forro vinho.
7. Continuidade visual entre cintura e caudas.
8. Necessidade de corrigir a amplitude dos controles.

Se a inclusão de ossos melhorar efetivamente os resultados, implemente-a.

Se uma correção de pesos ou restrições resolver o problema sem novos ossos, prefira a solução mais simples.

Não altere a silhueta aprovada do modelo sem justificativa.

**Resultado esperado:** casaca com deformação natural e sem atravessamentos Altos durante os testes obrigatórios e de estresse.

---

### 7. CORREÇÃO — GOLA E MOVIMENTO DA CABEÇA

O v005 apresenta interferências entre cabeça e gola nas rotações mais amplas.

Investigue especialmente:

- Cabeça olhando para cima em aproximadamente 40°.
- Rotação lateral de aproximadamente 60°.
- Movimentos combinados de pescoço e tronco.

Avalie a criação de um osso `collar_back`, ou solução equivalente, caso seja necessária.

A gola deve:

- Continuar alta.
- Permanecer aberta na frente.
- Preservar o forro vinho.
- Não atravessar o rosto ou pescoço.
- Não acompanhar a cabeça de forma completamente rígida.
- Manter a silhueta aristocrática.

Compare o comportamento antes e depois.

Não modifique a geometria da cabeça nesta missão.

---

### 8. CORREÇÃO — TORNOZELO E BOTAS

O teste Death identificou perda aproximada de 7,7% de volume no tornozelo.

Investigue:

1. Pesos entre canela e pé.
2. Localização dos pivôs.
3. Deformação da bota.
4. Possíveis influências indevidas de outros ossos.
5. Preservação da sola, salto e biqueira.

Corrija o colapso visual, sem adicionar geometria desnecessária.

Registre a medida antes e depois utilizando o mesmo procedimento de avaliação.

Não considere automaticamente qualquer alteração percentual como melhoria: a deformação também precisa parecer correta nos renders.

---

### 9. OMBROS E BRAÇOS

O modelo v004 ainda apresenta ombros relativamente arredondados, e o v005 tem sobreposições entre mangas, casaca e ombreiras.

Nesta fase, **não reconstrua a anatomia**.

Verifique apenas os problemas diretamente relacionados ao rig:

- Ombreira acompanhando corretamente o braço.
- Manga sem atravessar a casaca.
- Deformação coerente ao levantar os braços.
- Continuidade visual na articulação do ombro.
- Compatibilidade com os movimentos de espada e defesa.

Se a correção exigir remodelagem estrutural extensa, documente a necessidade e não amplie o escopo automaticamente.

---

### 10. ESPADA E MÃO DIREITA

Preserve o vínculo rígido da espada ao osso `sword`, filho da mão direita.

Verifique novamente:

- Posição do cabo.
- Orientação da lâmina.
- Ausência de deriva durante movimentos.
- Compatibilidade com Basic Attack.
- Compatibilidade com Crescent Slash.
- Compatibilidade com Rising Edge.
- Compatibilidade com Guard.
- Compatibilidade com Dodge.

Preserve as variantes:

- Espada normal.
- Espada com largura ×1,7.

Não altere comprimento, alcance ou mecânicas do jogo.

A escolha definitiva da variante continua pendente.

---

### 11. BATERIA DE TESTES OBRIGATÓRIA

Repita as 13 poses do v005:

| ID | Pose |
|---|---|
| P01 | Neutral |
| P02 | Walk |
| P03 | Run |
| P04 | Jump Rise |
| P05 | Jump Fall |
| P06 | Basic Attack |
| P07 | Crescent Slash |
| P08 | Rising Edge |
| P09 | Dodge |
| P10 | Guard |
| P11 | Hurt |
| P12 | Magic Cast |
| P13 | Death |

Repita também os testes de estresse:

- T10 — Agachamento profundo.
- T11 — Braço elevado com tronco torcido.

Execute os testes de movimento:

- Caminhada.
- Ataque.
- Esquiva.

Preserve as durações já utilizadas nos testes anteriores para permitir comparação direta.

O objetivo é eliminar problemas Críticos e Altos.

Problemas Médios somente poderão permanecer se forem visualmente aceitáveis, documentados e acompanhados de justificativa.

Não declare uma falha corrigida apenas porque o script terminou sem erros.

---

### 12. COMPARAÇÃO CONTROLADA v005 × v006

Gere comparações com:

- Mesmas câmeras.
- Mesma escala.
- Mesma iluminação.
- Mesmos materiais, salvo correção justificada.
- Mesmas poses.
- Mesmo enquadramento.
- Mesmo fundo.

As comparações devem demonstrar claramente o antes e depois.

Priorize:

1. Meia-capa durante Jump Fall.
2. Meia-capa durante Crescent Slash.
3. Casaca durante Dodge.
4. Casaca durante Death.
5. Gola durante rotação da cabeça.
6. Tornozelo durante flexão.
7. T10 antes/depois.
8. T11 antes/depois.

Não esconda interseções com ângulos favoráveis de câmera.

Inclua vistas laterais e 3/4.

Quando necessário, gere closes para inspeção.

---

### 13. RENDERS E FOLHAS DE VALIDAÇÃO

Crie uma pasta específica, seguindo a organização existente:

`art-src/player/comparisons/v1_2f_rig_v006/`

Produza, no mínimo:

- `rig_validation_sheet_v006.png`
- `rig_structure_v006.png`
- `deformation_tests_v006.png`
- `v005_vs_v006_neutral.png`
- `v005_vs_v006_problem_poses.png`
- `stress_tests_v006.png`
- `animation_tests_v006.png`
- `game_scale_v006.png`

Os nomes podem ser adaptados se o pipeline exigir, mas documente os nomes reais.

Corrija o enquadramento das vistas 3/4 que ficaram pequenas na V1.2-E.

Garanta que os PNGs tenham dimensões válidas e sejam visualmente inspecionáveis.

Não declare validação visual com base apenas em hashes ou na existência dos arquivos.

---

### 14. TESTES NA ESCALA REAL DO JOGO

Preserve o contrato atual:

- Personagem com aproximadamente 52 px de altura.
- Célula de movimento 64×64.
- Pivô (32, 63).
- Célula de combate 128×96.
- Pivô (40, 95).

Compare:

**Acabamento A:** pixel art sombreada.

**Acabamento B:** alta resolução.

**Acabamento C:** cores chapadas.

Compare também:

- Espada normal.
- Espada ×1,7.
- Escala de câmera 1:1.
- Zoom experimental ×1,5.
- Zoom experimental ×2.

Essas comparações devem ser externas ao jogo.

Não altere a câmera de Bevy.

Não escolha definitivamente D2 ou D3.

Avalie especialmente se as correções do rig continuam legíveis na escala real do gameplay.

---

### 15. FPS E SINCRONIZAÇÃO DE ANIMAÇÕES

Prepare um estudo técnico, sem implementar animações definitivas.

Compare a viabilidade de:

- 12 fps.
- 24 fps.

Considere as durações reais das ações existentes, especialmente:

- Ataque básico: 0,28 s.
- Dodge: 0,28 s.
- Crescent Slash: 0,40 s.
- Rising Edge: 0,45 s.

Considere que os efeitos de gameplay são aplicados no início das ações.

**Não altere esse comportamento.**

Investigue como construir animações visualmente convincentes mantendo a sincronização atual.

Se houver necessidade futura de antecipação visual, apresente alternativas sem modificar gameplay.

Não implemente combos, novos ataques, novas habilidades ou novos estados.

---

### 16. VALIDAÇÕES AUTOMÁTICAS

Crie ou adapte verificações para confirmar:

1. Arquivo `.blend` reabre corretamente.
2. Armature presente e hierarquia válida.
3. Ausência de ciclos.
4. Ossos obrigatórios presentes.
5. Pesos normalizados.
6. Limite de influências respeitado.
7. Ausência de vértices deformáveis sem pesos válidos.
8. Espada acompanhando a mão.
9. Pivôs consistentes.
10. Geometria preservada fora das regiões corrigidas.
11. Poses reproduzíveis.
12. PNGs válidos.
13. Transparência preservada.
14. Contrato de pivôs respeitado.
15. Nenhuma regressão grave em relação ao v005.
16. Nenhum arquivo protegido alterado.

Use o mesmo método de medição antes/depois sempre que possível.

Se a detecção automática de colisões apresentar falsos positivos, documente a limitação e complemente com inspeção visual.

Não invente resultados de testes.

---

### 17. PILLOW E PIPELINE DE IMAGENS

O ambiente apresentou um problema com `Pillow` e `Image.resize(NEAREST)`.

O pipeline já possui `post/resample.py` como alternativa.

Nesta missão:

1. Verifique se o problema ainda existe.
2. Preserve o comportamento determinístico.
3. Não reinstale pacotes sem autorização.
4. Não altere o Python do sistema.
5. Não remova o desvio existente sem demonstrar equivalência.
6. Registre a versão real do Python e do Pillow.
7. Compare saídas antes/depois quando houver mudanças.

Evite modificar os scripts de comparação antigos se não for necessário.

---

### 18. VERSIONAMENTO E BACKUP DOS ARQUIVOS BLEND

Os arquivos `.blend` anteriores estão ignorados pelo Git.

Isso representa um risco de perda do trabalho artístico caso o projeto seja copiado, restaurado ou clonado em outra máquina.

Faça uma auditoria somente de leitura:

- Quais `.blend` estão ignorados?
- Quais versões existem?
- Quais são seus tamanhos?
- Quais dependências externas utilizam?
- Quais scripts permitem reconstruí-los?
- Quais arquivos são indispensáveis para preservar o rig e as correções?

Apresente uma proposta para:

- Versionamento dos modelos.
- Uso eventual de Git LFS.
- Backup separado.
- Política de retenção dos arquivos `.blend1`.
- Reprodução do pipeline em outra máquina.

**Não altere `.gitignore`, não configure Git LFS e não faça commits sem minha autorização.**

---

### 19. DOCUMENTAÇÃO

Crie:

`art-src/player/RIG_VALIDATION_V006.md`

Atualize apenas as seções pertinentes da documentação artística existente, caso necessário.

O relatório deve conter:

- Objetivo da V1.2-F.
- Estado inicial.
- Arquivos inspecionados.
- Estratégia de correção.
- Ossos adicionados ou alterados.
- Mudanças nos pesos.
- Resultados antes/depois.
- Problemas corrigidos.
- Problemas restantes.
- Testes automáticos.
- Testes visuais.
- Comparação na escala de jogo.
- Riscos para animação.
- Recomendações para V1.3.
- Arquivos criados e modificados.
- Estado final do Git.

Diferencie claramente:

**CORRIGIDO**, **PARCIALMENTE CORRIGIDO**, **NÃO CORRIGIDO** e **NÃO TESTADO**.

Não declare uma correção concluída se a evidência for insuficiente.

---

### 20. CRITÉRIOS DE ACEITAÇÃO

A V1.2-F só poderá ser considerada concluída quando:

- [ ] O v005 permanecer intacto.
- [ ] O v006 existir como arquivo independente.
- [ ] O v006 reabrir em outro processo Blender.
- [ ] A hierarquia do rig continuar válida.
- [ ] Os pesos permanecerem normalizados.
- [ ] A espada permanecer rigidamente vinculada à mão.
- [ ] As 13 poses obrigatórias forem reavaliadas.
- [ ] T10 e T11 forem reavaliados.
- [ ] Os três testes de movimento forem repetidos.
- [ ] Nenhuma falha Crítica permanecer.
- [ ] Nenhuma falha Alta permanecer nos testes definidos.
- [ ] As deformações Médias restantes forem justificadas.
- [ ] A casaca funcionar visualmente nas poses principais.
- [ ] A meia-capa funcionar visualmente nas poses principais.
- [ ] A gola permitir os movimentos definidos.
- [ ] O tornozelo não apresentar colapso visual relevante.
- [ ] Os renders de comparação forem gerados e inspecionados.
- [ ] Os testes de escala de jogo forem concluídos.
- [ ] O pipeline permanecer reproduzível.
- [ ] Os arquivos anteriores forem preservados.
- [ ] Gameplay e saves permanecerem intactos.
- [ ] Nenhum commit ou push for realizado.

Se algum critério falhar, não declare a V1.2-F integralmente aprovada.

Registre as pendências e explique a causa.

---

### 21. RELATÓRIO FINAL OBRIGATÓRIO

Ao terminar, apresente o relatório nesta estrutura:

**A. Estado inicial**

Branch, HEAD, status do Git e arquivos preexistentes.

**B. Blender e MCP**

Versão, disponibilidade e método utilizado.

**C. Modelo v006**

Estrutura, quantidade de ossos, pesos e diferenças em relação ao v005.

**D. Meia-capa**

Problemas identificados, correções e resultados.

**E. Casaca**

Problemas identificados, correções e resultados.

**F. Gola**

Movimentos testados e limitações.

**G. Tornozelos e botas**

Resultados antes/depois.

**H. Ombros e braços**

Compatibilidade com movimentos.

**I. Espada**

Integridade do vínculo e variantes.

**J. Poses**

Tabela completa das 13 poses, com classificação anterior e nova.

**K. Testes de estresse**

T10 e T11, com evidências.

**L. Testes de movimento**

Caminhada, ataque e esquiva.

**M. Renders**

Arquivos gerados e onde encontrá-los.

**N. Escala de jogo**

A, B, C, espada normal e larga, câmera 1:1, ×1,5 e ×2.

**O. FPS e sincronização**

Estudo de 12 e 24 fps e compatibilidade com o gameplay.

**P. Validações automáticas**

Resultados e falhas.

**Q. Arquivos**

Criados, modificados e preservados.

**R. Git e backup**

Estado final, arquivos `.blend` ignorados e recomendações.

**S. Critérios de aceitação**

Atendidos, parciais e não atendidos.

**T. Pendências e recomendações**

Decisões que dependem do usuário.

---

### 22. REGRA FINAL — NÃO INICIAR V1.3

A V1.2-F termina na estabilização e validação do rig.

Não implemente o controlador de animações no Bevy.

Não integre sprites ao jogo.

Não modifique o gameplay.

Não escolha D2 ou D3 automaticamente.

Não altere a taxa definitiva de quadros.

Não faça commits ou push.

**Depois de gerar o v006 e apresentar os resultados, aguarde minha avaliação e autorização explícita antes de iniciar a V1.3.**

O objetivo é construir uma base artística sólida e reproduzível, evitando que problemas estruturais do rig sejam multiplicados quando começarmos a produzir as animações definitivas do Dark Castle RPG.