# DARK CASTLE RPG — V1.2-B
## Primeira modelagem 3D real do protagonista com Blender + MCP

### 1. Contexto

Estamos desenvolvendo o Dark Castle RPG em Rust 2024 + Bevy 0.20.1.

A Fase 4 está concluída e não deve ser modificada.

As etapas V1.1 e V1.2 produziram:

- `docs/ART_DIRECTION.md`
- `docs/ASSET_CONTRACT.md`
- Scripts de Blender em `art-src/player/scripts/bpy/`
- Scripts de pós-processamento em `art-src/player/scripts/post/`
- Comparações visuais provisórias

O Blender 5.2.4 LTS está instalado e funcionando.

A comunicação entre Claude Code e Blender pelo MCP foi confirmada com sucesso, incluindo a leitura real da cena e de seus objetos.

O complemento MCP permite executar Python dentro do Blender, mas essa capacidade ainda não foi testada.

### 2. Objetivo

**Autorizo iniciar a V1.2-B.**

Quero gerar o primeiro modelo 3D experimental do protagonista do Dark Castle RPG, validar os scripts existentes e produzir imagens reais para avaliação artística.

Esta autorização é limitada ao pipeline artístico.

Não autoriza alterações no gameplay nem o início da V1.3.

### 3. Referência visual

A referência escolhida está prevista em:

`art-src/player/references/inspiration/protagonist_concept_v001.png`

Antes de começar:

1. Confirme se o arquivo realmente existe.
2. Inspecione a imagem.
3. Leia `docs/ART_DIRECTION.md`.
4. Compare a referência com o conceito B aprovado.
5. Identifique diferenças relevantes.
6. Preserve a identidade do espadachim aristocrático sombrio.

O conceito B tem prioridade sobre detalhes conflitantes da imagem.

Características principais:

- Silhueta esguia.
- Proporção aproximada de sete cabeças.
- Cabelo curto cinza-ardósia.
- Casaca longa aberta, com forro vinho.
- Gola alta.
- Meia-capa assimétrica curta.
- Botas e luvas.
- Espada longa e fina, levemente curva.
- Paleta escura com acentos âmbar.

Não copie personagens conhecidos nem introduza elementos que descaracterizem a direção artística.

Se a referência estiver ausente, informe a ausência. Não invente que a inspecionou. Nesse caso, use apenas a documentação existente e identifique o modelo como provisório.

### 4. Preparação e segurança

Antes de modificar qualquer coisa:

1. Verifique `git status`.
2. Confirme o commit atual.
3. Identifique os arquivos existentes do pipeline artístico.
4. Inspecione os scripts antes de executá-los.
5. Confirme a versão real do Blender.
6. Confirme que o MCP continua conectado.
7. Preserve a cena atualmente aberta, caso contenha trabalho do usuário.

Não sobrescreva arquivos `.blend` existentes.

Não apague objetos de cenas do usuário.

Para o primeiro teste, utilize uma cena experimental isolada.

Não execute comandos destrutivos de Git.

### 5. Testar Python pelo MCP

Execute uma operação Python mínima, segura e reversível, preferencialmente apenas de leitura.

Confirme que o Python realmente é executado dentro do Blender.

Se falhar, investigue e corrija a integração antes de iniciar a modelagem.

Não utilize serviços externos de geração 3D.

### 6. Executar os scripts existentes

Analise os scripts em:

`art-src/player/scripts/bpy/`

Utilize os scripts já criados como base.

Não reescreva todo o pipeline sem necessidade.

Execute-os no Blender real.

Corrija incompatibilidades com Blender 5.2.4.

Registre erros encontrados e respectivas correções.

Não considere o teste anterior com `bpy` falso como validação suficiente.

### 7. Gerar o primeiro modelo

Construa um modelo 3D experimental do protagonista.

O modelo deve conter, dentro das possibilidades dos scripts:

- Cabeça e cabelo.
- Tronco.
- Braços e mãos.
- Pernas e botas.
- Casaca.
- Gola alta.
- Meia-capa assimétrica.
- Luvas.
- Espada.
- Materiais e cores.
- Iluminação de apresentação.

Priorize proporções, silhueta e legibilidade.

Não é necessário produzir ainda um personagem final pronto para animação.

Não declare o modelo como definitivo.

### 8. Produzir vistas de avaliação

Renderize:

1. Vista frontal.
2. Vista lateral.
3. Vista traseira.
4. Vista em perspectiva de três quartos.
5. Detalhes da roupa e da espada.

Use iluminação consistente e fundo neutro.

Mantenha as proporções iguais entre as vistas ortográficas.

Salve os resultados em uma pasta experimental versionada por nome, sem sobrescrever renders anteriores.

### 9. Comparar acabamentos

Utilize as renderizações reais para comparar:

**A — Pixel art detalhada**

**B — Alta resolução**

As duas versões devem representar o mesmo personagem e a mesma pose.

Avalie:

- Legibilidade.
- Silhueta.
- Detalhes.
- Espada.
- Roupas.
- Transparência.
- Contornos.
- Artefatos.
- Necessidade de correção manual.
- Tamanho dos arquivos.

Não declarar vencedor definitivo.

### 10. Comparar câmeras

Produza comparações externas ao jogo:

- Escala 1:1.
- Escala ×1,5.
- Escala ×2.

Avalie a visibilidade do personagem, inimigos, plataformas e projéteis.

Não modifique a câmera real do Bevy.

### 11. Revisar o contrato de assets

Depois dos testes reais, compare os resultados com:

`docs/ASSET_CONTRACT.md`

Identifique possíveis ajustes em:

- Dimensões.
- Quadros.
- Atlases.
- Padding.
- Pivôs.
- Transparência.
- Filtros.
- Nomenclatura.
- Estimativas de memória.

Não altere decisões ainda pendentes de aprovação.

### 12. Restrições absolutas

Não modificar:

- Código Rust do gameplay.
- Arquivos RON de gameplay.
- Física.
- Combate.
- Hitboxes.
- Câmera do jogo.
- Saves.
- Arquivos de `Md-usario/`.
- Artefatos protegidos de testes anteriores.

Não executar:

- `cargo clean`
- `git clean`
- `git reset --hard`
- `git push`

Não criar commits sem autorização.

Não instalar dependências sem autorização.

Não utilizar Meshy, Tripo, Hunyuan3D, Hyper3D, WorldLabs ou outros serviços externos de geração 3D.

### 13. Validação

Ao concluir:

1. Confirme que os scripts foram executados no Blender real.
2. Confirme que o modelo foi gerado.
3. Liste os arquivos produzidos.
4. Verifique as dimensões dos renders.
5. Verifique a transparência.
6. Verifique os materiais.
7. Confirme que os arquivos protegidos permanecem intactos.
8. Execute `git diff --check`.
9. Apresente o estado final do Git.

### 14. Relatório final

Apresente:

1. Resultado da execução Python pelo MCP.
2. Scripts executados e corrigidos.
3. Modelo 3D produzido.
4. Vistas renderizadas.
5. Comparação entre pixel art e alta resolução.
6. Comparação entre as três escalas de câmera.
7. Problemas encontrados.
8. Limitações do modelo.
9. Ajustes recomendados.
10. Arquivos criados e modificados.
11. Estado do Git.
12. Pendências para aprovação.

**Não inicie V1.3 automaticamente.**

Ao concluir, aguarde minha avaliação e autorização.