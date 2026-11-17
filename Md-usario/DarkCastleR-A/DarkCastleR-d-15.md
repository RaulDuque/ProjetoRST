# DARK CASTLE RPG — D15
## Protagonista-base, referências individuais e sistema de equipamentos modulares

Você é o responsável pela arquitetura de personagens e equipamentos 3D do Dark Castle RPG.

Temos Hunyuan3D-2mini instalado e funcional localmente em `~/AI-3D/`, com RTX 4050 de 6 GB, geração de geometria, exportação GLB e integração experimental com Blender 5.2.4 e Blender MCP.

## OBJETIVO

Construir um pipeline de criação de personagens e equipamentos independentes.

**Não queremos gerar o protagonista já vestido com toda a armadura.**

Queremos:

1. Um protagonista-base sem equipamentos removíveis.
2. Referências visuais separadas para cada equipamento.
3. Modelos 3D independentes de armas, armaduras, capas, calças, botas e acessórios de cabeça.
4. Um sistema de encaixes compatível com o mesmo personagem-base.
5. Possibilidade de trocar equipamentos sem recriar o protagonista.
6. Renderização posterior de sprites 2D para Bevy.

## FASE 1 — ESTRUTURA DE PASTAS

Inspecione a raiz do repositório antes de criar qualquer coisa.

Crie ou complete a seguinte estrutura, preservando diretórios e arquivos existentes:

```text
assets-user/
├── protagonista/
│   ├── base/
│   ├── frente/
│   ├── costas/
│   ├── lateral/
│   └── poses/
├── equip/
│   ├── arma/
│   ├── cabeça/
│   ├── calça/
│   ├── capa/
│   ├── corpo/
│   └── sapatos/
└── prota/
    ├── trans-aeria/
    ├── trans-golem/
    ├── trans-lobisomem/
    └── trans-fantasma/
```

Respeite os nomes existentes, incluindo acentos.

Não renomeie diretórios automaticamente.

Use `.gitkeep` quando necessário.

`assets-user/` é uma biblioteca de referências e insumos de arte, não a pasta de assets publicados do Bevy.

Não altere `assets/` nesta fase.

## FASE 2 — PROTAGONISTA SEM EQUIPAMENTO

O protagonista deve possuir um corpo-base reutilizável.

Características:

- Humanoide masculino adulto.
- Proporções consistentes entre frente, costas e lateral.
- Posição neutra adequada para rigging.
- Braços ligeiramente afastados do tronco.
- Mãos visíveis.
- Pés separados.
- Roupas básicas simples, ajustadas ao corpo.
- Sem armadura.
- Sem espada.
- Sem escudo.
- Sem capa.
- Sem botas de combate.
- Sem acessórios removíveis.

Preserve a identidade visual aprovada do protagonista do projeto.

**Não altere cabelo, rosto ou identidade canônica sem autorização.**

Se as referências atuais apresentarem diferenças de identidade, registre o conflito e solicite decisão antes de produzir uma nova versão oficial.

Não use anatomia explícita. O corpo-base deve vestir roupas neutras.

### Referências necessárias

Preparar especificações para:

- Frente.
- Costas.
- Lado esquerdo.
- Lado direito.
- Pose neutra.
- Pose de rigging.

Todas as vistas devem representar exatamente o mesmo personagem.

Não invente vistas como se fossem verificadas. Se não existirem imagens adequadas, registre quais precisam ser produzidas.

## FASE 3 — PADRÃO DOS EQUIPAMENTOS

Cada equipamento deve ser um asset independente.

Categorias:

### Arma
- Espadas.
- Espadas pesadas.
- Adagas.
- Lanças.
- Cajados.
- Outras armas futuras.

### Cabeça
- Capacetes.
- Capuzes.
- Coroas.
- Máscaras.
- Chapéus.

### Corpo
- Armaduras.
- Casacos.
- Peitorais.
- Túnicas.
- Roupas especiais.

### Calça
- Calças.
- Proteções de pernas.
- Armaduras inferiores.

### Capa
- Capas.
- Mantos.
- Elementos de tecido independentes.

### Sapatos
- Botas.
- Sapatos.
- Proteções de pés.

Não gere todos esses equipamentos agora. Primeiro estabeleça o padrão.

## FASE 4 — CONTRATO DE CADA EQUIPAMENTO

Defina uma estrutura reutilizável por item, por exemplo:

```text
equip/arma/espada_inicial/
├── referencia/
├── modelo/
├── blender/
├── renders/
└── metadata.json
```

O `metadata.json` deve permitir registrar:

- ID único.
- Nome.
- Categoria.
- Descrição.
- Imagem de referência.
- Arquivo GLB.
- Arquivo Blender.
- Versão.
- Escala.
- Orientação.
- Ponto de encaixe.
- Ossos necessários.
- Material.
- Estado de validação.
- Problemas conhecidos.

Não marque itens como prontos antes de verificar seus encaixes.

## FASE 5 — SISTEMA DE ENCAIXES

Estude o rig existente do protagonista.

Defina sockets ou vínculos para:

- Mão direita.
- Mão esquerda.
- Cabeça.
- Tronco.
- Quadril.
- Pernas.
- Pés.
- Costas/capa.

Diferencie:

**Equipamentos rígidos:** podem acompanhar um osso ou socket.

**Equipamentos deformáveis:** precisam de pesos de skinning ou solução equivalente.

Uma armadura não deve ser considerada modular apenas porque seu GLB foi importado.

Ela precisa acompanhar corretamente as animações do personagem.

## FASE 6 — HUNYUAN3D

Utilize a instalação existente:

`~/AI-3D/`

Não reinstale PyTorch, CUDA ou Hunyuan.

Não baixe novos checkpoints sem necessidade.

Use inicialmente:

`tencent/Hunyuan3D-2mini`

Geração somente de geometria.

Para cada peça:

1. Selecionar referência.
2. Verificar fundo e silhueta.
3. Gerar malha.
4. Exportar GLB.
5. Inspecionar no Blender.
6. Corrigir orientação.
7. Ajustar escala.
8. Identificar problemas geométricos.
9. Avaliar necessidade de retopologia.
10. Testar encaixe no protagonista.

Nunca assuma que uma malha gerada por IA está pronta para animação.

## FASE 7 — PRIMEIRO TESTE

Realize um piloto limitado:

**Personagem-base + uma espada independente.**

Não produza armadura completa ainda.

Objetivos do piloto:

- Verificar se existe uma referência aprovada do protagonista-base.
- Gerar ou reutilizar a malha-base, quando apropriado.
- Criar uma espada independente a partir de referência autorizada.
- Importar os dois modelos no Blender.
- Posicionar a espada na mão.
- Verificar escala e orientação.
- Demonstrar a espada equipada e desequipada.
- Salvar uma cena experimental independente.

Se não houver referência aprovada do personagem-base ou da espada, não improvise uma identidade final. Prepare as especificações e registre a pendência.

## FASE 8 — VALIDAÇÃO VISUAL

Produza, se os modelos estiverem disponíveis:

- Protagonista sem espada.
- Protagonista com espada.
- Espada isolada.
- Vista frontal.
- Vista lateral.

Compare a identidade visual com o padrão aprovado D12-A/D7-B.

Não substitua sprites oficiais.

Não modifique o renderer padrão.

Não altere a aparência do jogo principal.

## FASE 9 — TRANSFORMAÇÕES

Preserve as pastas:

- `trans-aeria`
- `trans-golem`
- `trans-lobisomem`
- `trans-fantasma`

Elas serão utilizadas futuramente para formas alternativas do protagonista.

Nesta fase, apenas documente o propósito de cada pasta.

Não implemente transformações ainda.

## FASE 10 — DOCUMENTAÇÃO

Crie documentação dentro da estrutura experimental, sem alterar arquivos de regras existentes sem autorização.

Explique:

1. Como adicionar um equipamento.
2. Como preparar a referência.
3. Como gerar a malha no Hunyuan.
4. Como importar no Blender.
5. Como padronizar escala.
6. Como definir sockets.
7. Como testar o encaixe.
8. Como validar a animação.
9. Como renderizar sprites futuramente.
10. Como preservar a identidade do protagonista.

## REGRAS DE SEGURANÇA

- Não alterar código Rust.
- Não alterar `assets/` do jogo.
- Não alterar Save V3.
- Não alterar combate ou movimentação.
- Não alterar arquivos existentes em `Md-usario/`.
- Não substituir a arte aprovada.
- Não modificar o rig original diretamente.
- Não instalar novamente o Hunyuan.
- Não fazer commit.
- Não fazer push.
- Não executar limpeza destrutiva.

## ENTREGA FINAL

Apresente:

- Árvore de pastas criada.
- Arquivos novos.
- Arquivos existentes preservados.
- Referências encontradas.
- Identidade visual utilizada.
- Estado do protagonista-base.
- Estado da espada independente.
- Resultado do teste no Blender.
- Problemas de encaixe.
- Limitações do Hunyuan3D-2mini.
- Próximos equipamentos recomendados.

Se não conseguir concluir alguma etapa, informe exatamente o motivo.

**Critério principal de sucesso: provar que podemos manter um único protagonista e equipá-lo com peças independentes, sem recriar o personagem inteiro.**