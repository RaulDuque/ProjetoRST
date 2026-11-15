# DARK CASTLE RPG — MEGA PROMPT V1.2-E
## Rig experimental, preparação para animações, deformação de roupas e validação do protagonista v004

**Projeto:** Dark Castle RPG  
**Engine:** Bevy 0.20.1  
**Linguagem:** Rust 2024  
**Software artístico:** Blender 5.2.4 LTS  
**Integração:** Claude Code + MCP for Blender  
**Modelo de origem:** Protagonista v004  
**Etapa:** V1.2-E — Preparação para animação  
**Status:** Execução experimental autorizada, sem integração ao gameplay.

---

# 1. SUA FUNÇÃO

Você é o responsável técnico pela preparação artística e pela infraestrutura de animação 3D do protagonista do Dark Castle RPG.

Atue como:

- Technical Artist especializado em Blender.
- Desenvolvedor Python especializado na API `bpy`.
- Especialista em rigging, skinning e deformação de personagens.
- Animador técnico de jogos 2D e 3D.
- Especialista em pipelines 3D → sprites 2D.
- Auditor de compatibilidade e integridade de arquivos.
- Engenheiro de automação e validação de assets.

Seu trabalho é preparar o modelo v004 para animações futuras, preservando sua identidade visual e a integridade do projeto.

**Não trate esta etapa como produção de arte final.**

O objetivo é obter um modelo tecnicamente utilizável, reproduzível, testável e suficientemente estável para iniciar a produção das animações.

Não avance automaticamente para a V1.3.

---

# 2. CONTEXTO DO PROJETO

O Dark Castle RPG é um RPG de ação 2D com atmosfera gótica.

O protagonista utiliza o conceito B:

**Espadachim aristocrático sombrio.**

Características estabelecidas:

- Corpo humanoide esguio.
- Silhueta elegante.
- Casaca longa e escura, aberta na frente.
- Forro e detalhes vinho.
- Gola alta.
- Meia-capa assimétrica.
- Cabelo curto cinza-ardósia.
- Olhos âmbar.
- Botas escuras.
- Luvas.
- Sabre levemente curvo.
- Ornamentos metálicos discretos.
- Aparência séria e misteriosa.

O personagem será utilizado em um jogo 2D de visão lateral.

O Blender será usado para criar e animar o modelo 3D, permitindo renderizar sprites 2D posteriormente.

O modelo precisa funcionar especialmente bem na vista lateral ortográfica.

---

# 3. ESTADO ATUAL

Já foram produzidos:

- v001: experimentos iniciais.
- v002: primeiro modelo construído com primitivas.
- v003: reconstrução procedural com malhas personalizadas.
- v004: refinamento de rosto, cabelo, corpo, roupas, materiais e silhueta.

O v004 foi gerado no Blender 5.2.4 LTS.

Informações reportadas:

- 132 objetos.
- 26 materiais.
- 13 câmeras.
- Altura aproximada de 1,811 m.
- Sola posicionada em Z = 0.
- Aproximadamente 404.096 triângulos avaliados após modificadores.
- Scripts procedurais reproduzíveis.
- Vistas ortográficas e comparações geradas.
- Sem rig.
- Sem animações definitivas.

O modelo v004 foi **aprovado provisoriamente como base para rigging**.

Isso não significa aprovação como arte final.

A versão v004 original deve permanecer preservada.

---

# 4. DECISÕES DO PROPRIETÁRIO

As decisões atuais são:

| Código | Decisão | Estado |
|---|---|---|
| D1 | Conceito B — espadachim aristocrático sombrio | Aprovado |
| D2 | Acabamento visual | Em avaliação |
| D3 | Zoom da câmera | Em avaliação |
| D4 | Modelo-base v004 | Aprovado provisoriamente |
| D5 | Rosto | Manter provisoriamente |
| D6 | Espada | Comparar normal e 1,7× |
| D7 | Rig experimental | Autorizado nesta etapa |
| D8 | Integração ao Bevy | Não autorizada |
| D9 | Commits e push | Não autorizados |

Para D2:

Priorizar a investigação de:

- B: alta resolução.
- C: cores chapadas.

Manter A, pixel art, como alternativa documentada.

Não declarar uma escolha definitiva.

Para D3:

- Preservar a câmera atual do jogo em 1:1.
- Manter testes visuais em ×1,5 e ×2.
- Não alterar a câmera do gameplay.

Não transforme recomendações em decisões finais sem autorização.

---

# 5. REGRAS DE SEGURANÇA

Antes de qualquer alteração, inspecione o repositório.

Execute verificações equivalentes a:

```bash
git status --short
git branch --show-current
git rev-parse HEAD
git diff --check
```

Registre:

- Branch atual.
- HEAD atual.
- Arquivos modificados.
- Arquivos staged.
- Arquivos untracked.
- Arquivos ignorados relevantes.
- Estado inicial das pastas artísticas.
- Estado dos arquivos protegidos.

Não assuma que a branch ainda é `master`.

O último relatório informou:

```text
art/v1.2-protagonist
HEAD inicial anterior: 204f965
```

Essas informações são históricas e precisam ser verificadas novamente.

## 5.1. Proibições absolutas

Não execute:

```bash
git clean
git reset --hard
cargo clean
git push
```

Não faça commits.

Não altere branches.

Não faça checkout que possa sobrescrever alterações.

Não apague versões antigas do modelo.

Não sobrescreva arquivos de arte anteriores.

Não modifique:

- Código Rust do gameplay.
- Sistemas ECS de combate.
- Física.
- Skills.
- Magias.
- Hitboxes.
- Dano.
- Cooldowns.
- Animações já existentes no jogo.
- Arquivos RON de gameplay.
- Saves.
- `Md-usario/`.
- `.cargo/config.toml`.
- `target/verify/`.
- `target/f4-stage/`.

Preserve especialmente:

```text
saves/slot1.ron
saves/slot1.ron.bak
```

Não altere nem exclua arquivos do usuário que já estavam modificados antes da execução.

Não execute instalações com `sudo`.

Não instale dependências adicionais sem autorização.

Não utilize serviços externos de geração 3D.

Não envie referências, modelos ou outros assets para terceiros.

Não desative o safe mode do MCP.

Se alguma tarefa exigir violar essas restrições, interrompa a tarefa e solicite autorização.

---

# 6. FASE E0 — AUDITORIA DO AMBIENTE

Antes de iniciar o rigging, verifique o ambiente real.

## 6.1. Blender

Confirme:

- Versão instalada.
- Executável utilizado.
- Disponibilidade do Python interno.
- Disponibilidade do EEVEE.
- Compatibilidade dos scripts existentes.
- Capacidade de abrir o v004.
- Capacidade de salvar um arquivo experimental em caminho novo.
- Capacidade de renderizar uma cena de teste.

A versão esperada é:

```text
Blender 5.2.4 LTS
```

Não considere essa versão confirmada sem consultar o ambiente.

## 6.2. MCP

O Claude Code anteriormente apresentou:

```text
blender — Connected — 10 tools
```

Isso confirma que o servidor estava registrado e carregado naquela sessão, mas não garante que o Blender esteja conectado agora.

Verifique:

1. Se o MCP está disponível nesta sessão.
2. Se o Blender está aberto.
3. Se o complemento está ativo.
4. Se a porta local está acessível.
5. Se as ferramentas de leitura funcionam.
6. Se a cena atual pode ser consultada.
7. Se o safe mode permanece habilitado.

Use inicialmente somente consultas de leitura.

O complemento anteriormente utilizou:

```text
127.0.0.1:9876
```

Se estiver offline, registre a situação.

Não tente desabilitar proteções para forçar a conexão.

**O MCP não é obrigatório para a execução do pipeline.**

Se necessário, utilize:

```bash
blender --background --factory-startup --python ...
```

A execução em processo separado é preferível para builds automatizados e evita modificar a cena interativa do usuário.

Se o Blender estiver aberto com uma cena do usuário, preserve-a integralmente.

## 6.3. Pillow

O relatório anterior identificou falha em `Image.resize(NEAREST)` no Pillow do sistema.

Existe um desvio implementado em:

```text
art-src/player/scripts/post/resample.py
```

Verifique:

- Se a falha continua ocorrendo.
- Se o desvio continua necessário.
- Se o resultado permanece determinístico.
- Se imagens antigas continuam sendo geradas sem regressões.

Não reinstale Pillow automaticamente.

Não substitua a solução existente sem evidências.

---

# 7. FASE E1 — INSPEÇÃO DO MODELO V004

Localize os arquivos reais do v004.

Os caminhos esperados incluem:

```text
art-src/player/scripts/bpy/build_swordsman_v004.py
art-src/player/scripts/bpy/swordsman_v004/
art-src/player/blender/
art-src/player/renders/v004/
art-src/player/comparisons/v1_2d_swordsman_v004/
```

Confirme os caminhos reais antes de utilizá-los.

Abra uma cópia experimental do modelo.

Inspecione:

- Hierarquia dos objetos.
- Malhas.
- Materiais.
- Modificadores.
- Transformações.
- Origens.
- Normais.
- Topologia.
- Grupos de vértices existentes.
- Objetos rígidos.
- Objetos deformáveis.
- Peças de roupa.
- Espada.
- Cabelo.
- Mãos.
- Pés.
- Gola.
- Meia-capa.

Produza um inventário técnico.

## 7.1. Classificação dos objetos

Classifique cada objeto como:

**A — Deformação por armature**

Exemplos:

- Tronco.
- Braços.
- Pernas.
- Mangas.
- Casaca.
- Meia-capa.

**B — Vinculação rígida a osso**

Exemplos:

- Espada.
- Fivela.
- Fechos.
- Partes rígidas de armadura.

**C — Deformação especial**

Exemplos:

- Caudas da casaca.
- Meia-capa.
- Algumas partes do cabelo.

**D — Apenas renderização ou controle**

Exemplos:

- Câmeras.
- Luzes.
- Objetos auxiliares.
- Controladores de teste.

Documente a classificação.

---

# 8. FASE E2 — CRIAÇÃO DA VERSÃO EXPERIMENTAL

Crie uma nova versão derivada:

```text
v005_rig_experimental
```

O v005 deve ser uma derivação técnica do v004.

Não é necessário redesenhar o personagem.

O foco é a preparação para animação.

Preserve integralmente:

- v001.
- v002.
- v003.
- v004.

O novo arquivo deverá utilizar um nome exclusivo, por exemplo:

```text
art-src/player/blender/swordsman_rig_v005.blend
```

Se esse caminho já existir, não sobrescreva sem investigar.

Registre claramente a relação:

```text
v004 → v005 rig experimental
```

Não altere os scripts do v004 para transformá-los silenciosamente em scripts do v005.

Prefira módulos novos ou uma camada de preparação separada.

---

# 9. FASE E3 — ARQUITETURA DO RIG

Crie uma armature humanoide adequada a animações de ação 2D.

A estrutura deve permitir:

- Idle.
- Caminhada.
- Corrida.
- Salto.
- Queda.
- Aterrissagem.
- Ataque básico.
- Ataque aéreo.
- Esquiva.
- Defesa.
- Recebimento de dano.
- Magias.
- Morte.
- Respawn.

Nesta etapa, crie somente o rig e as poses necessárias para validá-lo.

Não produza ainda a biblioteca completa de animações.

## 9.1. Estrutura mínima de ossos

Utilize uma nomenclatura consistente.

Sugestão:

```text
ROOT
└── pelvis
    ├── spine_01
    │   └── spine_02
    │       └── chest
    │           ├── neck
    │           │   └── head
    │           ├── clavicle.L
    │           │   └── upper_arm.L
    │           │       └── forearm.L
    │           │           └── hand.L
    │           └── clavicle.R
    │               └── upper_arm.R
    │                   └── forearm.R
    │                       └── hand.R
    ├── thigh.L
    │   └── shin.L
    │       └── foot.L
    │           └── toe.L
    └── thigh.R
        └── shin.R
            └── foot.R
                └── toe.R
```

A hierarquia acima é conceitual.

A implementação pode usar ossos adicionais e uma separação entre ossos de deformação e controladores.

Se isso melhorar a manutenção, use:

```text
DEF_
CTRL_
MCH_
```

Não crie uma estrutura excessivamente complexa sem necessidade.

## 9.2. Controles

Prepare controles para:

- Movimento global.
- Quadril.
- Tronco.
- Cabeça.
- Mãos.
- Pés.
- Espada.
- Casaca.
- Meia-capa.

## 9.3. IK e FK

Avalie:

- IK para pernas.
- IK para a mão que segura a espada.
- FK para movimentos de ataque.
- Alternância IK/FK, se necessária.

Não implemente mecanismos sofisticados apenas por conveniência.

Priorize estabilidade e reprodutibilidade.

---

# 10. FASE E4 — SKINNING E PESOS

Vincule as malhas deformáveis à armature.

Use pesos adequados para:

- Ombros.
- Cotovelos.
- Punhos.
- Quadril.
- Joelhos.
- Tornozelos.
- Pescoço.
- Tronco.

Não aceite automaticamente pesos gerados pelo Blender sem inspeção.

## 10.1. Critérios técnicos

Verifique:

- Vértices sem influência.
- Pesos não normalizados.
- Influências excessivas.
- Deformações inesperadas.
- Malhas que permanecem paradas.
- Objetos vinculados ao osso errado.
- Distorções de proporção.

Evite que a animação altere permanentemente a escala ou as proporções do personagem.

## 10.2. Ombros

O v004 apresenta interseções entre mangas, ombreiras e casaca.

Teste:

- Braço abaixado.
- Braço horizontal.
- Braço elevado.
- Braço cruzando o peito.
- Braço estendido para ataque.

Corrija problemas de deformação sem alterar desnecessariamente o design.

Se alguma interseção for inevitável por causa da geometria, registre o problema e proponha a menor correção possível.

---

# 11. FASE E5 — CASACA E MEIA-CAPA

Esta é uma das prioridades da etapa.

O personagem possui roupas longas que podem atravessar as pernas durante as animações.

## 11.1. Casaca

Prepare:

- Parte superior.
- Painéis dianteiros.
- Caudas traseiras.
- Fenda traseira.
- Forro.
- Bordas.

Crie ossos auxiliares quando necessário.

Sugestão:

```text
coat_front.L
coat_front.R
coat_back.L
coat_back.R
```

Cada cadeia pode conter mais de um osso, conforme a geometria real.

## 11.2. Meia-capa

A meia-capa deve acompanhar o ombro, mas possuir movimento secundário independente.

Prepare uma cadeia curta de ossos.

Teste:

- Braço abaixado.
- Braço elevado.
- Rotação do tronco.
- Corrida.
- Salto.
- Esquiva.

Não permita que a meia-capa atravesse visivelmente o tronco ou o braço em poses comuns.

## 11.3. Física de tecido

Não introduza simulação física em tempo real no jogo.

Nesta etapa, prefira:

- Ossos auxiliares.
- Poses controladas.
- Animação manual ou procedural.
- Simulação offline somente se realmente necessária.

O resultado final será renderizado como sprite.

Portanto, não precisamos transportar a simulação de tecido para Bevy.

---

# 12. FASE E6 — ESPADA

A espada deve acompanhar corretamente a mão.

Prepare:

- Ponto de fixação.
- Orientação local.
- Transformação de repouso.
- Controle para ataques.
- Controle para posicionamento defensivo.

A espada não deve atravessar a mão de maneira visível.

Teste duas variantes:

**Variante N — normal**

Lâmina original do v004.

**Variante W — larga**

Lâmina com largura 1,7×.

Mantenha:

- Mesmo comprimento.
- Mesmo ponto de fixação.
- Mesma pose.
- Mesma câmera.
- Mesma iluminação.

Não escolha uma variante definitivamente.

A comparação deve considerar especialmente a legibilidade na escala de jogo.

---

# 13. FASE E7 — CABEÇA, CABELO E GOLA

O rosto do v004 está aprovado provisoriamente.

Não faça uma nova reconstrução facial nesta etapa.

## 13.1. Cabeça

Vincule corretamente:

- Crânio.
- Rosto.
- Orelhas.
- Olhos.
- Sobrancelhas.
- Cabelo.

Se algum elemento precisar de movimento independente, documente a justificativa.

## 13.2. Cabelo

O relatório anterior identificou aproximadamente 44 mechas.

Avalie se é melhor:

- Manter as mechas separadas.
- Agrupá-las por região.
- Consolidá-las em malhas maiores.

Não destrua a geometria original.

O cabelo deve acompanhar a cabeça sem deslocamentos estranhos.

Movimento secundário do cabelo é opcional nesta etapa.

## 13.3. Gola

Teste:

- Cabeça neutra.
- Olhar para cima.
- Olhar para baixo.
- Rotação lateral moderada.

Identifique atravessamentos com a gola.

Não sacrifique a identidade visual da gola alta sem necessidade.

---

# 14. FASE E8 — POSES EXPERIMENTAIS

Crie uma pequena biblioteca de poses de validação.

Não confunda poses estáticas com animações definitivas.

## 14.1. Poses obrigatórias

**P01 — Neutral**

Postura neutra do personagem.

**P02 — Walk Contact**

Uma perna à frente, outra atrás, braços compensando.

**P03 — Run Extreme**

Passada ampla, tronco inclinado, casaca deslocada.

**P04 — Jump Rise**

Pernas recolhidas ou estendidas conforme o movimento, tronco acompanhando a subida.

**P05 — Jump Fall**

Postura de queda controlada.

**P06 — Basic Attack**

Golpe de espada com rotação do tronco e extensão do braço.

**P07 — Crescent Slash**

Pose de ataque com trajetória ampla de espada.

**P08 — Rising Edge**

Pose de golpe ascendente.

**P09 — Dodge**

Pose de esquiva com inclinação e deslocamento corporal.

**P10 — Guard**

Espada posicionada para defesa.

**P11 — Hurt**

Reação de impacto.

**P12 — Magic Cast**

Pose de conjuração com braço e mão preparados.

**P13 — Death**

Pose de queda ou colapso.

## 14.2. Restrições

As poses não devem:

- Criar novos ataques.
- Alterar o alcance das habilidades.
- Alterar hitboxes.
- Alterar dano.
- Alterar duração de ações.
- Alterar custos de MP ou Hearts.
- Alterar cooldowns.
- Modificar a lógica do gameplay.

São exclusivamente testes artísticos.

As habilidades existentes incluem:

- Crescent Slash.
- Rising Edge.
- Ember Bolt.
- Mending Light.

Respeite suas identidades visuais.

Não invente habilidades adicionais.

---

# 15. FASE E9 — TESTES DE DEFORMAÇÃO

Para cada pose, execute verificações técnicas.

## 15.1. Inspeção

Verifique:

- Interseção entre roupas e corpo.
- Interseção entre braços e tronco.
- Interseção entre pernas e casaca.
- Interseção entre meia-capa e ombro.
- Interseção entre cabelo e gola.
- Espada atravessando a mão.
- Pés deformados.
- Articulações colapsadas.
- Malhas desconectadas.
- Normais invertidas.
- Objetos deslocados.

## 15.2. Classificação

Classifique cada problema como:

- Crítico.
- Alto.
- Médio.
- Baixo.
- Cosmético.

Problemas críticos:

- Perda de parte do corpo.
- Deformação grave.
- Espada desconectada.
- Rig quebrado.
- Malha desaparecendo.
- Arquivo impossível de reabrir.

Problemas altos:

- Casaca atravessando grande parte da perna.
- Meia-capa atravessando o tronco.
- Braço visivelmente desconectado.
- Pé deformado durante a corrida.

Problemas cosméticos podem ser documentados para refinamento posterior.

Não declare o rig aprovado se existirem problemas críticos.

---

# 16. FASE E10 — RENDERS DE VALIDAÇÃO

Gere imagens para inspeção.

Utilize o Blender real.

Prefira EEVEE.

Mantenha iluminação consistente.

## 16.1. Folha principal

Crie:

```text
rig_validation_sheet_v005.png
```

Inclua:

- Personagem neutro.
- Caminhada.
- Corrida.
- Salto.
- Ataque.
- Defesa.
- Esquiva.
- Magia.
- Morte.

Use fundo escuro neutro.

## 16.2. Folha técnica

Crie:

```text
rig_structure_v005.png
```

Mostre:

- Armature.
- Principais ossos.
- Controladores.
- Hierarquia relevante.
- Ossos da casaca.
- Ossos da meia-capa.
- Fixação da espada.

## 16.3. Folha de deformações

Crie:

```text
deformation_tests_v005.png
```

Mostre os testes mais importantes.

Inclua aproximações das áreas problemáticas.

## 16.4. Comparação artística

Crie:

```text
v004_vs_v005_neutral.png
```

A comparação deve utilizar:

- Mesma pose.
- Mesma escala.
- Mesma câmera.
- Mesma iluminação.
- Mesmo fundo.

O objetivo é demonstrar que o rig não alterou desnecessariamente a aparência do personagem.

---

# 17. FASE E11 — TESTES NA ESCALA REAL DO JOGO

Renderize o personagem na escala definida pelo contrato atual.

O relatório anterior informou aproximadamente:

```text
Altura: 52 px
Pivô: sola na linha 63
```

Confirme esses valores no contrato real.

Não assuma que permanecem válidos sem verificar.

Teste:

- Idle.
- Caminhada.
- Corrida.
- Ataque.
- Esquiva.

Compare:

- A: pixel art.
- B: alta resolução.
- C: cores chapadas.

Priorize B e C para análise.

Mantenha A como alternativa.

## 17.1. Câmera

Compare visualmente:

- 1:1.
- ×1,5.
- ×2.

Não modifique a câmera real do jogo.

Registre:

- Legibilidade da silhueta.
- Visibilidade da espada.
- Leitura da meia-capa.
- Clareza dos movimentos.
- Risco de o personagem ficar pequeno demais.
- Risco de a câmera mostrar pouca área do cenário.

Não declare D2 ou D3 encerradas.

---

# 18. FASE E12 — TESTE DE ANIMAÇÃO TEMPORÁRIA

Depois de validar as poses, crie um teste curto de movimento.

O objetivo não é produzir animação final.

Crie somente:

**Teste A — Caminhada**

Um ciclo simples que permita observar:

- Pernas.
- Braços.
- Quadril.
- Casaca.
- Meia-capa.

**Teste B — Ataque**

Um movimento curto de espada para verificar:

- Ombro.
- Cotovelo.
- Punho.
- Espada.
- Tronco.
- Casaca.

**Teste C — Esquiva**

Um movimento de inclinação ou deslocamento para verificar:

- Centro de massa.
- Pernas.
- Casaca.
- Meia-capa.

Use arquivos de teste separados.

Não modifique os tempos reais do gameplay.

Não considere esses testes como animações definitivas.

Não produza spritesheets finais nesta etapa.

Se o rig ainda apresentar problemas graves, não force os testes de movimento. Corrija primeiro os bloqueadores e documente o que não puder ser resolvido.

---

# 19. FASE E13 — AUTOMAÇÃO PYTHON

Organize os scripts de forma modular.

Sugestão de estrutura:

```text
art-src/player/
├── blender/
│   ├── swordsman_base_v004.blend
│   └── swordsman_rig_v005.blend
│
├── scripts/
│   ├── bpy/
│   │   ├── build_swordsman_v004.py
│   │   ├── build_swordsman_rig_v005.py
│   │   └── swordsman_rig_v005/
│   │       ├── __init__.py
│   │       ├── config.py
│   │       ├── skeleton.py
│   │       ├── controls.py
│   │       ├── skinning.py
│   │       ├── garments.py
│   │       ├── sword.py
│   │       ├── poses.py
│   │       ├── animation_tests.py
│   │       ├── rendering.py
│   │       └── validation.py
│   └── post/
│
├── renders/
│   └── v005/
│
└── comparisons/
    └── v1_2e_rig_v005/
```

A estrutura é uma proposta.

Antes de criar arquivos, confira a organização existente e reutilize o que fizer sentido.

Não duplique código desnecessariamente.

Não altere os scripts antigos apenas para adequá-los a essa proposta.

## 19.1. Reprodutibilidade

O pipeline deve permitir:

1. Abrir a versão-base correta.
2. Preparar as malhas.
3. Criar a armature.
4. Criar controles.
5. Aplicar skinning.
6. Vincular espada e roupas.
7. Criar poses de teste.
8. Validar o rig.
9. Salvar o `.blend`.
10. Renderizar as folhas.

A execução deve ser determinística sempre que tecnicamente possível.

Registre parâmetros e versões.

Se alguma operação exigir intervenção manual, documente exatamente o procedimento e não afirme que o processo é 100% automatizado.

---

# 20. FASE E14 — TESTES AUTOMATIZADOS

Crie verificações adequadas à API do Blender.

## 20.1. Estrutura

Verifique:

- Armature existente.
- Ossos obrigatórios existentes.
- Nomes sem duplicações inesperadas.
- Hierarquia válida.
- Ausência de ciclos.
- Objetos corretamente vinculados.

## 20.2. Pesos

Verifique:

- Vértices deformáveis com influência.
- Pesos válidos.
- Ausência de pesos negativos.
- Influências normalizadas conforme o método adotado.
- Grupos correspondentes a ossos existentes.

## 20.3. Espada

Verifique:

- Espada vinculada à mão correta.
- Transformação preservada.
- Ausência de deslocamento inesperado.
- Variantes reproduzíveis.

## 20.4. Render

Verifique:

- PNGs válidos.
- Dimensões corretas.
- Canal alfa.
- Transparência.
- Pivô.
- Escala.
- Arquivos gerados nos caminhos previstos.

## 20.5. Reabertura

Salve o `.blend`.

Encerre o processo.

Abra o arquivo novamente em outro processo.

Verifique:

- Armature.
- Objetos.
- Materiais.
- Poses.
- Ações de teste.
- Câmeras.
- Luzes.
- Integridade geral.

Não declare sucesso apenas porque o script terminou com código 0.

---

# 21. FASE E15 — COMPATIBILIDADE COM O GAMEPLAY

A arquitetura de gameplay existente não deve ser modificada.

O sistema atual utiliza fases de execução como:

```text
Input
Commands
Actions
Physics
Combat
Resolution
Progression
Presentation
```

As futuras animações serão integradas na camada de apresentação.

O rig do Blender não deve determinar:

- Dano.
- Alcance.
- Hitboxes.
- Velocidade de movimento.
- Física.
- Tempo de ataque.
- Tempo de esquiva.
- Cooldown.
- Custos de habilidades.

A lógica de gameplay continua sendo a autoridade.

As animações deverão futuramente acompanhar o estado do personagem, não redefini-lo.

## 21.1. Tempos existentes

Considere os tempos documentados do jogo:

- Ataque básico: aproximadamente 0,28 s.
- Esquiva: aproximadamente 0,28 s.
- Crescent Slash: Casting de aproximadamente 0,4 s.
- Rising Edge: Casting de aproximadamente 0,45 s.
- Ember Bolt: preparação de aproximadamente 0,3 s.
- Mending Light: preparação de aproximadamente 0,6 s.

Esses valores devem ser confirmados no código e na documentação.

Não os altere.

O sistema atual aplica determinados efeitos no frame inicial da ação.

Isso é uma característica de gameplay existente, não uma autorização para alterar o momento do impacto.

Quando produzirmos animações definitivas, será necessário sincronizar a apresentação com esse comportamento ou solicitar aprovação explícita para uma mudança de gameplay.

Nesta etapa, apenas documente a compatibilidade.

---

# 22. FASE E16 — DOCUMENTAÇÃO

Atualize somente a documentação pertinente à produção artística.

Documente:

- Estrutura do rig.
- Hierarquia dos ossos.
- Convenções de nomes.
- Vinculação dos objetos.
- Controles.
- Pesos.
- Roupas.
- Espada.
- Poses.
- Testes.
- Problemas conhecidos.
- Limitações.
- Próximos passos.

Se necessário, crie um documento específico:

```text
art-src/player/RIG_SPECIFICATION.md
```

E um relatório:

```text
art-src/player/RIG_VALIDATION_V005.md
```

Verifique antes se já existem documentos equivalentes.

Não substitua documentação anterior.

Não modifique arquivos em `Md-usario/`.

---

# 23. FASE E17 — CRITÉRIOS DE ACEITAÇÃO

A etapa somente poderá ser considerada tecnicamente concluída quando:

- [ ] O v004 original estiver preservado.
- [ ] O v005 experimental tiver sido criado.
- [ ] O `.blend` puder ser reaberto.
- [ ] A armature existir e possuir hierarquia válida.
- [ ] Os principais ossos estiverem funcionais.
- [ ] Braços e pernas puderem ser posicionados.
- [ ] A espada acompanhar corretamente a mão.
- [ ] A casaca tiver controles de deformação.
- [ ] A meia-capa tiver controles de deformação.
- [ ] O cabelo acompanhar a cabeça.
- [ ] A gola tiver sido testada.
- [ ] As poses obrigatórias tiverem sido avaliadas.
- [ ] Os problemas críticos tiverem sido corrigidos ou claramente declarados como bloqueadores.
- [ ] As folhas de validação tiverem sido geradas.
- [ ] O personagem continuar reconhecível como conceito B.
- [ ] A escala de jogo tiver sido testada.
- [ ] Os acabamentos A, B e C continuarem disponíveis para comparação.
- [ ] As variantes da espada continuarem disponíveis.
- [ ] Os scripts forem reproduzíveis ou suas limitações estiverem documentadas.
- [ ] O gameplay estiver intacto.
- [ ] Os saves estiverem intactos.
- [ ] As versões anteriores estiverem intactas.
- [ ] Nenhum commit tiver sido criado.
- [ ] Nenhum push tiver sido realizado.

Não marque critérios como concluídos sem evidências.

---

# 24. FASE E18 — VALIDAÇÃO FINAL DO GIT

Ao terminar, execute verificações equivalentes a:

```bash
git status --short
git diff --check
git diff --stat
git rev-parse HEAD
git branch --show-current
```

Compare com o estado inicial.

Verifique os hashes dos arquivos protegidos.

Diferencie:

- Alterações que já existiam.
- Arquivos criados nesta etapa.
- Arquivos modificados nesta etapa.
- Arquivos ignorados pelo Git.
- Arquivos staged anteriormente.
- Eventuais problemas encontrados.

Não adicione arquivos ao stage automaticamente.

Não crie commits.

Não faça push.

Não execute comandos destrutivos para tentar limpar o repositório.

---

# 25. RELATÓRIO FINAL OBRIGATÓRIO

Ao terminar, apresente um relatório completo com as seguintes seções.

## A. Estado inicial

- Branch.
- HEAD.
- Status.
- Arquivos protegidos.
- Ambiente Blender.
- MCP.
- Pillow.

## B. Modelo utilizado

- Origem do v004.
- Caminhos reais.
- Arquivos preservados.
- Estratégia de derivação.

## C. Rig criado

- Quantidade de ossos.
- Hierarquia.
- Controles.
- IK/FK.
- Organização.

## D. Skinning

- Malhas vinculadas.
- Estratégia de pesos.
- Problemas corrigidos.
- Limitações.

## E. Casaca

- Ossos.
- Deformações.
- Testes.
- Problemas restantes.

## F. Meia-capa

- Ossos.
- Deformações.
- Testes.
- Problemas restantes.

## G. Espada

- Fixação.
- Variante normal.
- Variante larga.
- Testes.

## H. Cabeça e cabelo

- Vinculação.
- Gola.
- Problemas encontrados.

## I. Poses

Apresente uma tabela com:

| Pose | Resultado | Problemas |
|---|---|---|
| Neutral | | |
| Walk | | |
| Run | | |
| Jump Rise | | |
| Jump Fall | | |
| Basic Attack | | |
| Crescent Slash | | |
| Rising Edge | | |
| Dodge | | |
| Guard | | |
| Hurt | | |
| Magic Cast | | |
| Death | | |

## J. Testes de animação

- Caminhada.
- Ataque.
- Esquiva.
- Problemas de deformação.
- Resultados.

## K. Renders

Informe os caminhos de todas as folhas e imagens principais.

## L. Escala do jogo

- Altura em pixels.
- Pivô.
- Acabamentos.
- Câmeras.
- Legibilidade.

## M. Validações automáticas

- Testes executados.
- Testes aprovados.
- Testes reprovados.
- Problemas conhecidos.

## N. Arquivos

- Criados.
- Modificados.
- Preservados.
- Ignorados.

## O. Git

- Branch final.
- HEAD final.
- Estado do stage.
- Estado da árvore.
- Confirmação de ausência de commits e push.

## P. Critérios de aceitação

Classifique cada critério como:

- Atendido.
- Parcialmente atendido.
- Não atendido.
- Bloqueado.

## Q. Próximas decisões

Informe o que ainda depende do proprietário.

Não escolha automaticamente:

- D2.
- D3.
- Variante definitiva da espada.
- Mudanças de gameplay.
- Integração ao Bevy.
- Início da V1.3.

---

# 26. ORDEM DE EXECUÇÃO

Siga esta sequência:

1. Auditar o repositório e o ambiente.
2. Verificar Blender e MCP.
3. Inspecionar o v004.
4. Criar uma derivação experimental preservando o original.
5. Construir o rig.
6. Aplicar skinning.
7. Preparar casaca e meia-capa.
8. Vincular espada e cabelo.
9. Criar poses.
10. Validar deformações.
11. Corrigir problemas críticos.
12. Executar testes curtos de movimento.
13. Gerar renders e folhas.
14. Comparar a escala de jogo.
15. Executar validações automáticas.
16. Reabrir o arquivo salvo.
17. Atualizar documentação pertinente.
18. Auditar a integridade do projeto e o Git.
19. Apresentar o relatório final.
20. Aguardar aprovação.

**Não interrompa o trabalho apenas para pedir autorização a cada etapa técnica já abrangida por este prompt.**

A autorização cobre a criação e os testes dos arquivos experimentais da V1.2-E.

Entretanto, interrompa e solicite autorização se encontrar uma situação que exija:

- Alterar gameplay.
- Modificar arquivos protegidos.
- Sobrescrever modelos anteriores.
- Instalar dependências.
- Desabilitar o safe mode.
- Utilizar serviços externos.
- Fazer commits.
- Fazer push.
- Mudar de branch.
- Executar operações destrutivas.

Se uma etapa técnica não puder ser concluída, documente o bloqueio e continue somente com atividades independentes e seguras.

---

# 27. RESULTADO ESPERADO

Ao final da V1.2-E, quero ter:

**1. Um protagonista v005 experimental com rig funcional.**

O modelo deve continuar fiel ao conceito B.

**2. Uma armature organizada.**

Com controles suficientes para produzir animações de ação.

**3. Roupas preparadas para movimento.**

Principalmente casaca e meia-capa.

**4. Espada corretamente vinculada.**

Com as duas variantes preservadas.

**5. Poses de validação.**

Incluindo caminhada, corrida, salto, ataque, defesa, esquiva, magia e morte.

**6. Testes curtos de animação.**

Apenas para verificar o comportamento do rig.

**7. Evidências visuais.**

Folhas comparativas, estrutura de ossos e deformações.

**8. Scripts reproduzíveis.**

Sempre que tecnicamente possível.

**9. Documentação técnica.**

Com arquitetura, limitações e instruções para manutenção.

**10. Projeto preservado.**

Sem alterações no gameplay, saves, modelos antigos ou arquivos protegidos.

---

# INSTRUÇÃO FINAL

Execute a **V1.2-E — Rig experimental e preparação para animação do protagonista v004**.

Priorize a qualidade técnica, a estabilidade e a legibilidade do personagem em movimento.

Não desperdice tempo refinando detalhes faciais que não serão perceptíveis na escala real do jogo.

Concentre o esforço em:

**Rig, articulações, casaca, meia-capa, espada, deformações e poses.**

Use o Blender real para validar o resultado.

Mantenha o MCP em safe mode.

Preserve as versões anteriores.

Não altere gameplay.

Não faça commits.

Não faça push.

Ao concluir, apresente o relatório completo e aguarde minha avaliação antes de iniciar a V1.3.