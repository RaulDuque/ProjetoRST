# DARK CASTLE RPG — MEGA PROMPT V1.2-D
## Refinamento artístico do protagonista — Modelo 3D v004
### Rust 2024 + Bevy 0.20.1 | Blender 5.2.4 LTS | Claude Code + MCP

---

# 0. MISSÃO PRINCIPAL

Você atuará como diretor de arte técnico, modelador 3D, especialista em Blender Python, artista de personagens, especialista em renderização estilizada e revisor técnico do projeto **Dark Castle RPG**.

Sua missão é realizar a **V1.2-D — Refinamento Artístico do Protagonista**, evoluindo o modelo 3D v003 para uma nova versão v004.

O v003 já representa uma evolução substancial em relação ao v002 e está aprovado como base técnica e artística provisória.

**Não quero uma reconstrução completa. Quero refinamentos localizados, consistentes e visualmente verificáveis.**

A prioridade é corrigir:

1. Rosto sem personalidade suficiente.
2. Cabelo excessivamente arredondado e brilhante.
3. Ombros e braços com aparência rígida.
4. Pernas muito cilíndricas.
5. Botas pouco detalhadas.
6. Colete frontal semelhante a um macacão.
7. Casaca e meia-capa ainda rígidas.
8. Materiais com brilho excessivamente plástico.

A nova versão deverá manter o conceito B: **Espadachim Aristocrático Sombrio**.

Não declare o modelo aprovado como arte final.

Não inicie a V1.3.

---

# 1. CONTEXTO E ESTADO DO PROJETO

O projeto utiliza:

- Rust 2024.
- Bevy 0.20.1.
- Blender 5.2.4 LTS.
- Python e `bpy`.
- Claude Code.
- MCP for Blender.

Histórico:

- Fase 4: concluída.
- V1.1: direção artística.
- V1.2: contrato de assets.
- V1.2-B: primeiro modelo Blender real, v002.
- V1.2-C: reconstrução procedural, v003.
- V1.2-D: refinamento localizado, v004 — missão atual.

O último commit de referência informado foi `e396c31`, na branch `master`.

Confirme o estado real antes de começar.

Não assuma que os arquivos continuam exatamente iguais.

A Fase 4 não deve ser reaberta.

---

# 2. LEITURA E AUDITORIA INICIAL

Antes de modificar qualquer arquivo:

1. Execute `git status --short`.
2. Confirme branch e HEAD.
3. Identifique arquivos rastreados modificados e arquivos não rastreados.
4. Identifique os arquivos de arte existentes.
5. Leia `docs/ART_DIRECTION.md`.
6. Leia `docs/ASSET_CONTRACT.md`.
7. Leia `art-src/README.md`.
8. Leia a documentação das referências.
9. Inspecione os scripts do v003.
10. Confirme a versão real do Blender.
11. Verifique a disponibilidade do MCP.
12. Confirme os caminhos de entrada e saída.

Arquivos principais informados:

`art-src/player/scripts/bpy/build_swordsman_v003.py`

`art-src/player/scripts/bpy/swordsman_v003/`

`art-src/player/comparisons/v1_2c_swordsman_v003/model_sheet_v003.png`

`art-src/player/comparisons/v1_2c_swordsman_v003/model_sheet_v003_dark.png`

`art-src/player/comparisons/v1_2c_swordsman_v003/compare_v002_v003.png`

`art-src/player/renders/v003/`

Localize o `.blend` original do v003.

Não invente caminhos ausentes.

Registre hashes dos arquivos que devem permanecer inalterados e faça a conferência novamente no final.

---

# 3. PRESERVAÇÃO OBRIGATÓRIA

Não modificar:

- Código Rust de gameplay.
- RON de gameplay.
- Sistema de combate.
- Sistema de habilidades.
- Sistema de magia.
- Sistema de câmera.
- Sistema de movimentação.
- Sistemas de física.
- Saves.
- Testes existentes.
- Arquivos de `Md-usario/`.
- `target/verify/`.
- `target/f4-stage/`.
- Configurações protegidas de Cargo.

Não executar:

- `cargo clean`
- `git clean`
- `git reset --hard`
- `git push`

Não criar commits.

Não fazer push.

Não instalar dependências sem autorização.

Não utilizar serviços externos de geração 3D.

Não sobrescrever os modelos v001, v002 ou v003.

Não alterar arquivos existentes fora do escopo artístico autorizado.

Preserve a capacidade de reproduzir o v003.

---

# 4. DIREÇÃO ARTÍSTICA

Manter:

- Silhueta esguia.
- Aproximadamente sete cabeças de altura.
- Elegância aristocrática.
- Fantasia gótica.
- Cabelo curto cinza-ardósia.
- Expressão séria.
- Casaca longa aberta.
- Forro vinho.
- Gola alta.
- Meia-capa assimétrica curta.
- Luvas.
- Botas altas.
- Espada longa com curvatura discreta.
- Acentos âmbar.

Evitar:

- Armadura pesada.
- Capa até o chão.
- Excesso de ornamentos.
- Cabelo longo prateado.
- Visual de soldado genérico.
- Aparência infantil.
- Visual excessivamente cartunesco.
- Materiais com aspecto de plástico.
- Cópia de personagens existentes.

O personagem deve transmitir a impressão de alguém experiente, reservado, disciplinado e perigoso.

Não mudar sua identidade visual sem justificar e solicitar aprovação.

---

# 5. PRIORIDADE MÁXIMA — REFINAMENTO DO ROSTO

## 5.1 Problemas observados no v003

Nas imagens apresentadas, o rosto tem anatomia básica, mas ainda parece excessivamente macio.

Problemas:

- Olhos muito estreitos e pouco expressivos.
- Sobrancelhas uniformes.
- Nariz com definição insuficiente.
- Boca pouco visível.
- Maçãs do rosto suaves demais.
- Mandíbula pouco marcante.
- Queixo sem personalidade.
- Transição muito lisa entre as regiões faciais.

O objetivo não é criar um rosto fotorealista.

O objetivo é criar um rosto estilizado convincente, com personalidade.

## 5.2 Correções desejadas

Refinar:

**Olhos**
- Pálpebras superiores e inferiores.
- Formato levemente angular.
- Profundidade das órbitas.
- Iris âmbar sem emissão exagerada.
- Direção coerente do olhar.

**Sobrancelhas**
- Curvatura natural.
- Inclinação que transmita seriedade.
- Separação da testa sem parecer peças coladas.

**Nariz**
- Ponte nasal.
- Ponta.
- Asas.
- Narinas simplificadas.
- Silhueta lateral.

**Boca**
- Linha labial discreta.
- Lábio superior e inferior.
- Cantos da boca.
- Expressão neutra e determinada.

**Mandíbula e queixo**
- Definição maior.
- Transição natural para o pescoço.
- Silhueta lateral mais forte.

**Maçãs do rosto**
- Volumes discretos.
- Planos faciais mais definidos.

## 5.3 Limite da abordagem procedural

Antes de modificar a cabeça, avalie se a geometria paramétrica existente permite essas melhorias.

Se permitir, implemente refinamentos localizados.

Se não permitir, não force dezenas de parâmetros artificiais que deixem o código frágil.

Nesse caso:

1. Explique a limitação.
2. Identifique quais partes podem continuar procedurais.
3. Proponha uma abordagem híbrida.
4. Registre o que exigiria escultura manual.

Não substitua silenciosamente o rosto por um modelo externo.

Não utilize geração 3D de terceiros.

---

# 6. CABELO

## Problemas atuais

O cabelo melhorou bastante em relação ao v002, mas ainda apresenta:

- Mechas grossas.
- Pontas arredondadas.
- Brilho exagerado.
- Distribuição relativamente uniforme.
- Aparência de cabelo plástico.

## Refinamento

Criar:

- Mechas de espessuras diferentes.
- Pontas mais finas.
- Sobreposição natural.
- Direções menos uniformes.
- Pequenas assimetrias.
- Melhor implantação na testa.
- Volume lateral e traseiro coerente.

Reduzir o brilho especular.

Evitar milhares de fios individuais.

A silhueta do cabelo deve continuar reconhecível quando reduzida para sprite.

---

# 7. OMBROS, BRAÇOS E MÃOS

Os ombros ainda apresentam uma aparência rígida.

Refinar:

- Transição entre pescoço e ombros.
- Inclinação natural das clavículas.
- Volume do deltoide.
- Proporções do braço.
- Cotovelos.
- Antebraços.
- Punhos.

Diferenciar melhor tecido e peças rígidas.

Não transformar os braços em armaduras pesadas.

As mãos devem manter:

- Palma.
- Dedos simplificados.
- Polegar.
- Empunhadura da espada.

Verificar se a mão segura a espada corretamente, sem atravessamentos evidentes.

Não criar rig nesta etapa.

---

# 8. PERNAS E BOTAS

## Pernas

Os volumes atuais ainda parecem tubos.

Refinar:

- Transição do quadril para as coxas.
- Joelhos.
- Panturrilhas.
- Tornozelos.
- Proporção entre coxa e canela.
- Silhueta lateral.

Não exagerar detalhes anatômicos que ficarão escondidos pela roupa.

## Botas

Adicionar detalhes moderados:

- Cano estruturado.
- Costuras ou divisões principais.
- Dobras próximas ao tornozelo.
- Biqueira mais natural.
- Sola com espessura.
- Salto discreto.
- Separação visual entre couro e sola.

Evitar ornamentos excessivos.

As botas devem continuar legíveis na escala do jogo.

---

# 9. COLETE E REGIÃO FRONTAL

O v003 apresenta uma região abaixo do colete que lembra um macacão.

Corrigir a separação visual entre:

- Colete.
- Cinto.
- Cintura.
- Calça.
- Painéis da casaca.

Adicionar uma transição de vestuário coerente.

O cinto deve parecer uma peça funcional.

O colete deve terminar em um ponto visualmente convincente.

Não alterar a paleta principal sem necessidade.

---

# 10. CASACA

A casaca v003 é uma boa base.

Não reconstruí-la completamente.

Refinar:

- Caimento.
- Dobras.
- Espessura.
- Separação dos painéis.
- Assimetria discreta.
- Transição entre cintura e caudas.
- Bordas do tecido.
- Forro vinho.

Evitar superfícies excessivamente lisas.

Verificar as vistas frontal, lateral e traseira.

Não permitir que o tecido atravesse as pernas em repouso.

A geometria deve continuar adequada para uma futura armature.

---

# 11. MEIA-CAPA ASSIMÉTRICA

A meia-capa v003 já está presa ao ombro, mas ainda apresenta um formato muito regular.

Refinar:

- Ponto de fixação.
- Distribuição do peso.
- Dobras principais.
- Curvatura do tecido.
- Borda inferior.
- Assimetria.
- Sobreposição sobre a casaca.

A meia-capa deve parecer um tecido que repousa sobre o ombro e as costas.

Não deve parecer uma placa rígida.

Não executar simulação física.

Não criar animações.

Preservar espaço para futura deformação por ossos.

---

# 12. MATERIAIS E ILUMINAÇÃO

## Problemas atuais

Algumas superfícies parecem muito brilhantes:

- Cabelo.
- Casaca.
- Ombreiras.
- Botas.
- Partes das luvas.

## Objetivo

Diferenciar claramente:

- Lã.
- Couro.
- Cetim.
- Metal.
- Pele.
- Cabelo.

Ajustar:

- Roughness.
- Metallic.
- Intensidade especular.
- Subsurface quando necessário.
- Cores e contraste.

Não remover todos os reflexos.

O metal deve continuar parecendo metal.

O tecido deve continuar parecendo tecido.

Testar sobre:

- Fundo neutro.
- Fundo escuro semelhante ao ambiente do jogo.

Não usar texturas externas sem autorização.

Manter a iluminação consistente entre as vistas.

---

# 13. ESPADA

A espada do v003 está satisfatória como base.

Não reconstruí-la sem necessidade.

Preservar:

- Curvatura discreta.
- Guarda.
- Cabo.
- Pomo.
- Materiais metálicos.
- Comprimento.

Manter duas variantes experimentais:

**Variante normal:** largura atual.

**Variante ampliada:** largura visual 1,7×.

Não modificar alcance, hitbox ou dano.

Comparar as variantes na escala real do jogo.

Não escolher uma delas definitivamente sem aprovação.

---

# 14. IMPLEMENTAÇÃO DA V004

Criar uma nova versão independente.

Sugestão:

`art-src/player/scripts/bpy/build_swordsman_v004.py`

`art-src/player/scripts/bpy/swordsman_v004/`

Reutilizar os módulos do v003 quando apropriado.

Evitar duplicação desnecessária.

Entretanto, garantir que futuras alterações no v004 não modifiquem acidentalmente a reprodução do v003.

Preferir módulos novos ou parâmetros versionados para alterações que afetem o resultado anterior.

Não modificar os scripts do v003 apenas por conveniência.

A v004 deve ser reproduzível a partir de uma execução limpa do Blender.

Registrar os parâmetros utilizados.

---

# 15. BLENDER E MCP

Utilizar Blender 5.2.4 LTS.

Manter:

`BLENDER_MCP_SAFE_MODE=1`

O MCP pode ser usado para:

- Inspeção.
- Consultas.
- Verificação de propriedades.
- Conferência da cena aberta.

Para scripts completos, utilizar o Blender em processo separado:

`blender --background --factory-startup --python ...`

Não executar código desconhecido.

Não alterar a cena pessoal aberta no Blender.

Não desativar o modo seguro.

Não utilizar:

- Meshy.
- Tripo.
- Hunyuan3D.
- Hyper3D.
- World Labs.
- Outros serviços externos de geração 3D.

---

# 16. ENTREGÁVEIS DA V004

Criar:

`art-src/player/renders/v004/`

`art-src/player/comparisons/v1_2d_swordsman_v004/`

Gerar:

1. Vista frontal.
2. Vista lateral.
3. Vista traseira.
4. Vista três quartos.
5. Close do rosto.
6. Close do cabelo.
7. Close do tronco e colete.
8. Close da casaca.
9. Close da meia-capa.
10. Close das botas.
11. Close da espada.

Criar também:

- `model_sheet_v004.png`
- `model_sheet_v004_dark.png`
- `compare_v003_v004.png`

As comparações devem usar condições equivalentes de câmera, escala, fundo e iluminação.

Não reaproveitar renders antigos fingindo que são do v004.

---

# 17. COMPARAÇÃO EM ESCALA DE JOGO

Testar o v004 em aproximadamente 52 pixels de altura, conforme o contrato atual.

Comparar:

**A — Pixel art sombreada**

**B — Alta resolução**

**C — Cores chapadas**

Para cada alternativa, avaliar:

- Legibilidade.
- Silhueta.
- Contraste.
- Cabelo.
- Casaca.
- Meia-capa.
- Espada.
- Transparência.
- Contornos.

Comparar nas escalas:

- 1:1.
- ×1,5.
- ×2.

A câmera 1:1 continua sendo a configuração provisória do jogo.

Não modificar a câmera real.

Não declarar vencedor definitivo.

Testar a espada normal e a espada 1,7×.

Se o resultado em 52 pixels for praticamente idêntico ao v003, registre isso: refinamentos que só aparecem em closes não devem ser apresentados como melhorias de gameplay.

---

# 18. PREPARAÇÃO PARA RIG

Não implementar o rig.

Verificar se as alterações continuam permitindo:

- Flexão de braços.
- Flexão de joelhos.
- Rotação de tronco.
- Rotação de cabeça.
- Movimento da casaca.
- Movimento da meia-capa.
- Ataques de espada.
- Corrida.
- Salto.
- Esquiva.

Identificar pontos onde a geometria poderá atravessar outras malhas.

Documentar as correções necessárias antes do rig.

Não exigir uma única malha para todo o corpo.

---

# 19. CRITÉRIOS DE ACEITAÇÃO

A v004 deve apresentar melhorias verificáveis em relação à v003.

Critérios:

1. Rosto mais expressivo.
2. Mandíbula e nariz mais definidos.
3. Olhos e sobrancelhas mais naturais.
4. Cabelo menos plástico.
5. Mechas com silhueta melhor.
6. Ombros menos rígidos.
7. Braços com volumes mais naturais.
8. Pernas menos cilíndricas.
9. Botas mais reconhecíveis.
10. Colete separado visualmente da calça.
11. Casaca com caimento melhor.
12. Meia-capa com aparência de tecido.
13. Materiais diferenciados.
14. Espada preservada e legível.
15. Vista lateral adequada ao jogo.
16. Scripts reproduzíveis.
17. V003 preservado.
18. Gameplay intacto.
19. Saves intactos.
20. Nenhum commit ou push.

Classifique cada critério como:

- Atendido.
- Parcialmente atendido.
- Não atendido.

Apresente evidências visuais ou técnicas para cada conclusão relevante.

Não declare um critério atendido apenas porque o script contém uma função com o nome correspondente.

---

# 20. VALIDAÇÕES TÉCNICAS

Ao terminar:

1. Execute o script da v004 no Blender real.
2. Confirme que terminou sem erros.
3. Reabra o `.blend`.
4. Verifique os objetos.
5. Verifique os materiais.
6. Verifique as câmeras.
7. Confirme as dimensões dos PNGs.
8. Confirme o alfa dos renders.
9. Confirme a reprodutibilidade do build.
10. Confira os hashes dos arquivos protegidos.
11. Confira os hashes dos modelos anteriores.
12. Execute `git diff --check`.
13. Verifique o estado do Git.
14. Confirme que não houve commit.
15. Confirme que não houve push.

Não é necessário executar toda a suíte Rust se nenhum arquivo do jogo tiver sido alterado.

Não modificar arquivos de gameplay para facilitar os testes artísticos.

---

# 21. RELATÓRIO FINAL

Entregue um relatório organizado nas seguintes seções:

## A. Estado inicial
Branch, HEAD, status e arquivos protegidos.

## B. Estratégia
Como o v003 foi refinado sem reconstrução completa.

## C. Rosto
Alterações e limitações.

## D. Cabelo
Geometria, silhueta e materiais.

## E. Corpo
Ombros, braços, pernas e botas.

## F. Vestuário
Colete, casaca e meia-capa.

## G. Materiais
Rugosidade, reflexos e iluminação.

## H. Espada
Variantes e legibilidade.

## I. Modelo v004
Objetos, triângulos, materiais e dimensões.

## J. Comparação v003 × v004
Melhorias realmente observáveis.

## K. Testes de acabamento
Pixel art, alta resolução e cores chapadas.

## L. Preparação para rig
Problemas e recomendações.

## M. Arquivos
Criados, modificados e preservados.

## N. Validações
Resultados, hashes e Git.

## O. Critérios de aceitação
Atendidos, parciais e não atendidos.

## P. Pendências
Decisões que dependem do usuário.

---

# 22. REGRAS FINAIS

Não iniciar a V1.3.

Não implementar animações.

Não implementar rig.

Não alterar gameplay.

Não alterar câmera do jogo.

Não modificar saves.

Não criar commits.

Não fazer push.

Não utilizar geração 3D externa.

Não declarar aprovação artística em nome do usuário.

**A V1.2-D será considerada tecnicamente concluída quando o v004 estiver gerado, validado, documentado e comparado visualmente com o v003. A aprovação artística continuará dependendo da avaliação humana.**

Se algum refinamento não puder ser realizado com qualidade suficiente, explique a limitação e apresente uma alternativa, sem mascarar o problema.

Ao terminar, apresente o relatório e aguarde minha avaliação.