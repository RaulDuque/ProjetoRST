# DARK CASTLE RPG — MEGA PROMPT V2.1
## Cidade, NPCs, Diálogos, Comércio, Economia, Lojas e Tutoriais de Criação de Conteúdo

### MISSÃO OFICIAL — IMPLEMENTAÇÃO AUTORIZADA

Você é o arquiteto técnico principal e responsável pela implementação da V2.1 do Dark Castle RPG.

O projeto é um Action RPG Metroidvania 2D original, desenvolvido com Rust 2024, Bevy 0.20.1, arquivos RON e pipeline artístico baseado em Blender 5.2.4 LTS.

O jogo deverá oferecer um castelo gigantesco, regiões interconectadas, cidade, NPCs, comerciantes, equipamentos, armas, armaduras, magias, transformações, chefes, missões e exploração não linear.

A inspiração de design é a profundidade de Castlevania: Symphony of the Night, mas o projeto deverá manter identidade original.

A V2.0 foi aprovada funcionalmente, com ressalvas relacionadas à aparência dos equipamentos, regras de armas de duas mãos, distribuição de itens e compatibilidade histórica de saves.

**Você está autorizado a implementar a V2.1 dentro das restrições deste documento.**

Não inicie a V2.2.

---

# 1. OBJETIVO PRINCIPAL

A V2.1 deverá implementar uma primeira área habitada e um ciclo econômico funcional.

O jogador deverá conseguir:

1. Chegar à cidade por uma conexão legítima do mundo.
2. Explorar suas ruas.
3. Encontrar NPCs.
4. Conversar com personagens.
5. Entrar em uma loja.
6. Consultar mercadorias.
7. Comprar equipamentos.
8. Vender itens permitidos.
9. Equipar itens adquiridos.
10. Ver o dinheiro disponível.
11. Salvar e carregar sem perder compras.
12. Retornar ao castelo.
13. Encontrar NPCs e lojas novamente.
14. Utilizar os novos sistemas sem comprometer combate e exploração.

Além disso, a V2.1 deverá criar uma biblioteca de tutoriais simples dentro de `Md-usario/tutorias/`.

Essa biblioteca será uma entrega obrigatória, não uma tarefa opcional.

---

# 2. ESTADO CONHECIDO DO PROJETO

O relatório final da V2.0 informou:

- Branch `art/v1.2-protagonist`.
- HEAD `204f965`.
- 333 testes Rust passando.
- 17 testes Python do pipeline artístico.
- Autoplay 23/23.
- Mundo com 22 salas e três regiões.
- Forma Espectral funcional.
- Save V2 preservado.
- Inventário acessível pela tecla I.
- Oito slots de equipamento.
- Cinco categorias de armas.
- 27 equipamentos cadastrados.
- Dois slots de acessórios.
- Slot Offhand para escudos.
- Capas ocupando o slot Body.
- Equipamentos com modificadores reais.
- Sistema de comparação.
- Baús persistentes.
- Rota de equipamentos com nove transições e zero reposicionamentos artificiais.
- Nenhum commit, push ou staging.

O relatório também identificou pendências:

1. A espada inicial continua desenhada no corpo do protagonista mesmo com outra arma equipada.
2. Armas pesadas e lanças ainda podem ser equipadas com escudos.
3. Inimigos não possuem drops de equipamentos.
4. A compatibilidade com saves históricos reais não foi comprovada.
5. Capas competem com armaduras pelo slot Body.
6. Pontos de ancoragem visuais ainda não são exportados pelo pipeline artístico.

Confirme o estado real do repositório.

Não considere uma afirmação do relatório como prova independente.

---

# 3. REGRAS ABSOLUTAS DE SEGURANÇA

Antes de qualquer modificação:

- Registre branch e HEAD.
- Registre `git status --porcelain=v1 -uall`.
- Identifique o índice staged.
- Identifique arquivos modificados.
- Identifique arquivos não rastreados.
- Identifique arquivos ignorados importantes.
- Registre hashes dos arquivos protegidos.
- Confirme os saves.
- Confirme os assets publicados.
- Confirme o estado da documentação.
- Execute testes de linha de base.

Preserve integralmente:

- `Md-usario/`, exceto os novos tutoriais explicitamente autorizados.
- `saves/`.
- `.cargo/config.toml`.
- `target/verify/`.
- `target/f4-stage/`.
- Modelos Blender históricos.
- Backups artísticos.
- Assets publicados.
- PNG previamente staged.
- Alterações não commitadas.
- Reorganização existente de `Md-usario/`.

É proibido:

- `git clean`.
- `git reset --hard`.
- `cargo clean`.
- `git add`.
- `git commit`.
- `git push`.
- Merge.
- Rebase.
- Troca de branch.
- Criação de worktree.
- Exclusão de backups.
- Alteração de saves reais.
- Instalação de dependências sem autorização.
- Publicação de assets fora do pipeline seguro.
- Alteração do índice Git.

Não modifique arquivos protegidos para facilitar testes.

---

# 4. REGRA PERMANENTE DE COMPILAÇÃO

A V1.7 sofreu travamento por falta de memória durante o linking de binários Bevy.

Utilize no máximo dois jobs.

```bash
export CARGO_BUILD_JOBS=2
```

Execute somente um comando Cargo por vez.

Se houver pressão de memória, reduza para um job.

Não execute compilações paralelas.

Não apague caches.

Não modifique `.cargo/config.toml`.

Não execute comandos de teste em paralelo apenas para acelerar a entrega.

---

# 5. AUDITORIA OBRIGATÓRIA

Leia os documentos existentes relevantes:

- `CLAUDE.md`.
- `docs/GAME_VISION_MASTER.md`.
- `docs/NPC_COMMERCE_SYSTEM.md`.
- `docs/QUEST_FACTION_SYSTEM.md`.
- `docs/CASTLE_WORLD_DESIGN.md`.
- `docs/WORLD_ARCHITECTURE.md`.
- `docs/ROOM_SYSTEM.md`.
- `docs/WORLD_PERSISTENCE.md`.
- `docs/WORLD_CONTENT_PIPELINE.md`.
- `docs/EQUIPMENT_EXPANSION_V2_0.md`.
- `docs/EQUIPMENT_VISUAL_SYSTEM.md`.
- `docs/WEAPON_EXPANSION.md`.
- `docs/EQUIPMENT_SYSTEM.md`.
- `docs/CHARACTER_ATTRIBUTES.md`.
- `docs/COMBAT_SYSTEM.md`.
- `docs/TRANSFORMATIONS_V1_9.md`.
- `docs/DECISIONS.md`.
- `docs/ROADMAP.md`.

Confirme os caminhos reais.

Não invente documentos ausentes.

Inspecione os módulos reais de:

- Mundo.
- Salas.
- Inventário.
- Equipamentos.
- Itens.
- Save.
- Input.
- UI.
- Progressão.
- Interações.
- Combate.
- Assets.
- Pipeline artístico.

Produza uma matriz de implementação antes de alterar o código.

---

# 6. ARQUITETURA DE NPCs

Crie um sistema reutilizável de personagens não jogáveis.

Cada NPC deverá possuir, conforme necessário:

- ID estável.
- Nome.
- Sala.
- Posição.
- Tipo.
- Aparência.
- Diálogo.
- Estado.
- Requisitos.
- Serviços disponíveis.
- Regras de interação.
- Persistência.

Tipos iniciais:

- Morador.
- Comerciante.
- Ferreiro.
- Personagem informativo.
- Personagem relacionado a missões futuras.

Não implemente sistemas complexos de missões nesta fase.

Não crie uma IA social completa.

Os NPCs iniciais poderão ser estáticos ou utilizar animações simples.

Evite introduzir pathfinding desnecessário.

---

# 7. INTERAÇÃO COM NPCs

Reutilize a tecla D, já utilizada para interagir.

O jogador deverá conseguir:

1. Aproximar-se do NPC.
2. Visualizar uma indicação de interação.
3. Pressionar D.
4. Abrir o diálogo.
5. Avançar as falas.
6. Escolher opções, quando disponíveis.
7. Encerrar a conversa.

Não permita que o mesmo comando abra um baú e converse com um NPC simultaneamente.

Implemente seleção determinística de alvo quando vários objetos estiverem próximos.

Defina prioridade e distância máxima de interação.

Não permita interagir com NPCs mortos, inexistentes ou pertencentes a outra sala.

---

# 8. SISTEMA DE DIÁLOGOS

Implemente um sistema de diálogos orientado a dados.

Prefira arquivos RON.

Cada diálogo poderá possuir:

- ID.
- NPC.
- Texto.
- Próximo diálogo.
- Opções.
- Condições.
- Ações permitidas.
- Encerramento.

## 8.1 Diálogos simples

O NPC poderá apresentar uma sequência de falas.

## 8.2 Escolhas

O jogador poderá selecionar opções.

Exemplos:

- Conversar.
- Comprar.
- Vender.
- Perguntar sobre a cidade.
- Sair.

## 8.3 Condições

Prepare suporte a:

- Relíquia obtida.
- Região descoberta.
- Flag de progressão.
- Item possuído.
- Evento concluído.

Implemente apenas as condições necessárias.

Não crie uma linguagem de scripts arbitrária.

Não permita executar comandos de sistema por meio dos arquivos de diálogo.

---

# 9. INTERFACE DE DIÁLOGO

A interface deverá apresentar:

- Nome do NPC.
- Texto.
- Opções.
- Seleção atual.
- Indicação de avanço.
- Indicação de encerramento.

Requisitos:

- Texto legível.
- Layout responsivo às resoluções suportadas.
- Sem sobreposição indevida ao HUD.
- Sem bloquear o mapa permanentemente.
- Sem permitir ações de combate durante o diálogo.

Ao fechar, o controle deve retornar ao jogador corretamente.

Não deixe o personagem preso em estado de conversa.

---

# 10. PRIMEIRA CIDADE

Crie uma pequena região original.

Nome provisório:

**Vila das Cinzas.**

A cidade deverá possuir aproximadamente seis a oito salas, conforme a arquitetura atual.

Proposta:

1. Estrada de acesso.
2. Praça central.
3. Rua dos mercadores.
4. Loja de equipamentos.
5. Oficina do ferreiro.
6. Casa de um morador.
7. Passagem de retorno ao castelo.
8. Pequeno espaço opcional.

Não crie salas apenas para atingir uma quantidade.

A cidade deverá ser navegável com o movimento humano atual.

Não altere a física do jogador.

Não use teletransportes artificiais.

---

# 11. CONEXÃO COM O CASTELO

A cidade deverá estar conectada ao grafo do mundo.

Requisitos:

- Conexão legítima.
- Portas válidas.
- Spawn seguro.
- Câmera correta.
- Retorno possível.
- Mapa integrado.
- Save/load entre cidade e castelo.
- Progressão principal preservada.

Não substitua a região inicial do jogo.

Não altere silenciosamente a sala inicial.

Não torne a cidade obrigatória para completar a progressão já aprovada, salvo justificativa explícita.

---

# 12. NPCs INICIAIS

Crie um conjunto pequeno de personagens originais.

Sugestões:

**Mercador de equipamentos**

Compra e vende itens permitidos.

**Ferreiro**

Oferece diálogo e poderá futuramente prestar serviços de aprimoramento.

Não implemente upgrade de equipamentos ainda.

**Morador**

Apresenta informações sobre a cidade e o castelo.

**Guardião da estrada**

Explica caminhos e perigos.

**Personagem misterioso**

Apresenta uma pista sobre uma região secreta.

Não implemente missões completas apenas para justificar esse personagem.

Meta: quatro a seis NPCs funcionais.

---

# 13. SISTEMA DE ECONOMIA

Audite se o jogo já possui moeda ou saldo persistente.

Se existir, reutilize.

Se não existir, projete um modelo simples.

A moeda poderá ter um nome original.

Requisitos:

- Saldo não negativo.
- Limites seguros.
- Valores inteiros.
- Compra.
- Venda.
- Recompensa.
- Persistência.
- Interface.

Não utilize números de ponto flutuante para representar dinheiro.

Não permita overflow ou underflow.

Não crie múltiplas moedas nesta fase.

---

# 14. SAVE E ECONOMIA — DECISÃO CRÍTICA

O Save V2 foi preservado nas versões anteriores.

Antes de implementar dinheiro persistente, verifique se o schema atual possui um campo compatível.

**Se o Save V2 não suportar saldo persistente, não improvise uma solução.**

Não reutilize campos de inventário com significados incompatíveis.

Não grave dinheiro em arquivos paralelos sem contrato de consistência.

Não crie Save V3 automaticamente.

Nesse caso:

1. Documente a necessidade.
2. Proponha uma migração tipada.
3. Explique compatibilidade V1/V2.
4. Explique backups.
5. Explique rollback.
6. Solicite autorização para a mudança de schema.
7. Continue tarefas independentes e seguras.

Uma loja sem persistência econômica correta não deverá ser declarada concluída.

---

# 15. SISTEMA DE LOJAS

Implemente uma arquitetura reutilizável para estabelecimentos comerciais.

Cada loja deverá possuir:

- ID.
- Nome.
- NPC responsável.
- Estoque.
- Preços.
- Política de compra.
- Política de venda.
- Requisitos.
- Estado persistente, quando necessário.

Não duplique o inventário do jogador.

Não crie itens diretamente no sistema de UI.

Reutilize o catálogo existente.

---

# 16. ESTOQUE DOS COMERCIANTES

Planeje dois tipos de estoque:

- Ilimitado.
- Limitado.

Na V2.1, implemente o necessário para demonstrar ambos, se houver persistência segura.

Um item limitado vendido ao jogador deverá ter o estoque atualizado.

O estoque não poderá reaparecer após save/load indevidamente.

Não permita estoque negativo.

Não permita comprar um item esgotado.

Não permita que um mesmo item seja comprado duas vezes quando restar apenas uma unidade.

---

# 17. COMPRA DE ITENS

O fluxo deverá ser:

1. Jogador abre a loja.
2. Seleciona Comprar.
3. Visualiza itens.
4. Seleciona um item.
5. Visualiza preço e atributos.
6. Confirma a compra.
7. O sistema valida dinheiro e estoque.
8. O dinheiro é descontado.
9. O item entra no inventário.
10. O estoque é atualizado.
11. O HUD apresenta o resultado.

A operação deverá ser atomicamente coerente no estado do jogo.

Se qualquer validação falhar, nada deverá ser alterado.

Teste compra repetida no mesmo frame.

---

# 18. VENDA DE ITENS

O jogador poderá vender itens permitidos.

Valide:

- Item pertence ao jogador.
- Quantidade disponível.
- Item pode ser vendido.
- Preço válido.
- Saldo não ultrapassa limite.
- Item não é uma relíquia de progressão.
- Item não é essencial à progressão.
- Equipamento equipado segue política explícita.

Minha recomendação inicial:

**Não permitir vender itens equipados diretamente.**

O jogador deverá removê-los antes.

Não permita vender o equipamento inicial se isso deixar o jogador sem uma opção mínima de combate, salvo regra de recuperação definida.

Não permita vender relíquias permanentes.

---

# 19. PREÇOS E BALANCEAMENTO

Crie uma política simples e documentada.

Exemplos:

- Preço de compra definido no catálogo ou na loja.
- Valor de venda definido por item.
- Relação de preços consistente.
- Itens raros mais caros quando apropriado.

Não gere preços aleatórios a cada abertura da loja.

Não permita ciclos de comprar e vender que produzam dinheiro infinito.

Teste:

- Comprar e vender o mesmo item.
- Vender itens repetidos.
- Comprar estoque limitado.
- Saldo insuficiente.
- Estoque esgotado.
- Preço zero.
- Preço inválido.
- Overflow.

---

# 20. INTERFACE DE LOJA

Implemente uma interface jogável.

Deverá mostrar:

- Nome da loja.
- Nome do comerciante.
- Saldo do jogador.
- Opções Comprar e Vender.
- Lista de itens.
- Categoria.
- Raridade.
- Preço.
- Quantidade.
- Comparação com equipamento atual.
- Confirmação.
- Mensagens de erro.

Reutilize as funções reais de comparação implementadas na V2.0.

Não duplique a lógica de atributos.

Não permita que o jogador ataque enquanto a loja estiver aberta.

Não deixe o jogo preso ao fechar a interface.

---

# 21. INTEGRAÇÃO COM EQUIPAMENTOS

Os comerciantes deverão utilizar os 27 equipamentos existentes, conforme apropriado.

Não recrie itens equivalentes.

Distribua um pequeno conjunto entre:

- Loja.
- Baús.
- Recompensas.
- Conteúdo opcional.

Não coloque todo o catálogo na primeira loja.

Preserve a progressão.

Não venda equipamentos que trivializem o castelo inicial sem justificativa.

---

# 22. REGRAS DE ARMAS DE DUAS MÃOS

A V2.0 deixou uma pendência:

Armas pesadas e lanças podem ser equipadas junto com escudos.

Audite as categorias e os slots reais.

Defina uma propriedade explícita, como `handedness`, se puder ser adicionada sem alterar o Save V2.

Comportamentos possíveis:

- OneHanded.
- TwoHanded.

Uma arma de duas mãos deverá bloquear o uso simultâneo de equipamento incompatível na mão secundária.

A política deve ser determinística.

Não remova nem perca o escudo automaticamente.

Minha recomendação:

- Recusar equipar arma de duas mãos enquanto houver escudo equipado.
- Recusar equipar escudo enquanto houver arma de duas mãos equipada.
- Mostrar a explicação na interface.

Teste ambas as direções.

Se isso exigir mudança incompatível de schema, documente e solicite autorização.

---

# 23. APARÊNCIA VISUAL PENDENTE DA V2.0

O relatório da V2.0 informou que:

- A capa aparece.
- O tronco pode mudar de tonalidade.
- O rastro do ataque muda.
- A espada inicial continua desenhada no sprite mesmo com outra arma equipada.
- O pipeline não exporta pontos de ancoragem por quadro.

Esta pendência não deve ser esquecida.

Na V2.1:

1. Audite o pipeline artístico.
2. Documente o que falta para exportar âncoras.
3. Avalie a possibilidade de adicionar metadados de mão e cabeça.
4. Preserve os assets publicados.
5. Não sobrescreva sprites existentes.
6. Não produza um redesenho completo do protagonista fora do escopo.

Se houver uma correção pequena e segura, implemente-a.

Se exigir uma nova etapa artística significativa, registre uma tarefa futura específica.

Não declare aparência completa sem resolver a espada embutida.

---

# 24. DROPS DE EQUIPAMENTOS

A V2.0 não implementou drops.

Avalie uma extensão mínima e determinística.

Não altere o consumo de números aleatórios das rotas existentes.

Se possível, crie uma tabela de loot isolada por inimigo.

Requisitos:

- IDs válidos.
- Quantidades válidas.
- Probabilidades válidas.
- Recompensa concedida uma vez por morte.
- Ausência de duplicação.
- Compatibilidade com respawn.
- Compatibilidade com save/load.

Não implemente um sistema complexo de loot procedural.

Se houver risco de regressão, mantenha como pendência documentada.

---

# 25. DIÁLOGOS E PROGRESSÃO

Alguns NPCs poderão alterar falas conforme o progresso do jogador.

Exemplos:

- Antes de encontrar a Relíquia do Véu.
- Depois de obter a Relíquia do Véu.
- Depois de descobrir as Galerias Esquecidas.
- Depois de visitar o Claustro Velado.

Não implemente missões completas.

Não crie flags redundantes.

Reutilize os estados de progressão existentes.

---

# 26. MAPA DA CIDADE

Integre a cidade ao mapa.

Requisitos:

- Salas visitadas.
- Salas não descobertas.
- Conexões.
- Localização atual.
- Lojas descobertas.
- NPCs importantes descobertos, quando apropriado.
- Porcentagem de exploração.

Não revele automaticamente a cidade inteira.

Não altere silenciosamente a fórmula de exploração.

---

# 27. PERFORMANCE

Meça:

- Custo de carregar NPCs.
- Custo de abrir diálogo.
- Custo de abrir loja.
- Custo de listar mercadorias.
- Custo de compra.
- Custo de venda.
- Custo de save/load.
- Quantidade de entidades por sala.
- Memória máxima.

Não atualize todos os NPCs do mundo a cada frame.

Carregue somente os NPCs relevantes à sala atual.

Não faça leitura de RON a cada interação.

---

# 28. TUTORIAIS — ENTREGA OBRIGATÓRIA

Crie a pasta:

`Md-usario/tutorias/`

**A grafia `tutorias` é intencional. Não substitua por `tutoriais` sem autorização.**

Essa pasta será uma biblioteca de instruções para o proprietário criar conteúdo no jogo sem precisar conhecer profundamente Rust, Bevy ou RON.

A documentação deverá ser escrita em português brasileiro.

Utilize linguagem extremamente simples.

Imagine que o leitor nunca criou uma sala, item ou baú.

Não use explicações vagas.

Não diga apenas "edite o arquivo RON".

Mostre exatamente o que editar.

---

# 29. ESTRUTURA DOS TUTORIAIS

Crie, no mínimo:

```text
Md-usario/
└── tutorias/
    ├── 00_LEIA_PRIMEIRO.md
    ├── 01_CRIAR_SALAS.md
    ├── 02_EDITAR_SALAS.md
    ├── 03_CRIAR_REGIOES_E_FASES.md
    ├── 04_CONECTAR_SALAS_E_PORTAS.md
    ├── 05_CRIAR_ITENS.md
    ├── 06_CRIAR_ARMAS.md
    ├── 07_CRIAR_ARMADURAS_E_ACESSORIOS.md
    ├── 08_CRIAR_BAUS.md
    ├── 09_CRIAR_SALAS_SECRETAS.md
    ├── 10_CRIAR_PAREDES_QUEBRAVEIS.md
    ├── 11_CRIAR_ALAVANCAS_E_MECANISMOS.md
    ├── 12_CRIAR_RELIQUIAS.md
    ├── 13_CRIAR_BARREIRAS_ESPECTRAIS.md
    ├── 14_ADICIONAR_INIMIGOS.md
    ├── 15_EDITAR_MAPA_E_EXPLORACAO.md
    ├── 16_TESTAR_CONTEUDO.md
    ├── 17_ERROS_COMUNS.md
    ├── 18_CRIAR_NPCS.md
    ├── 19_CRIAR_DIALOGOS.md
    ├── 20_CRIAR_LOJAS.md
    ├── 21_CADASTRAR_MERCADORIAS.md
    ├── 22_CONFIGURAR_PRECOS.md
    └── 23_CRIAR_CIDADE.md
```

Um arquivo por assunto.

Não junte todos os tutoriais em um único documento.

---

# 30. MODELO OBRIGATÓRIO DE CADA TUTORIAL

Cada tutorial deverá seguir a estrutura:

## 1. O que vamos fazer?

Explique o objetivo em linguagem simples.

## 2. O que você precisa saber antes?

Explique os conceitos mínimos.

## 3. Quais arquivos vamos utilizar?

Mostre os caminhos reais.

## 4. Passo a passo

Use etapas numeradas.

Explique cada operação.

## 5. Exemplo completo

Mostre um exemplo válido e copiável.

Não utilize campos inventados.

## 6. Como testar?

Mostre os comandos.

Explique o resultado esperado.

## 7. Como editar depois?

Ensine a modificar ou remover o conteúdo.

## 8. Problemas comuns

Mostre erros frequentes e soluções.

## 9. Como desfazer com segurança?

Explique como reverter somente as alterações feitas no tutorial.

Não recomende `git clean` nem `git reset --hard`.

---

# 31. EXEMPLOS REAIS DOS TUTORIAIS

Os exemplos deverão utilizar o código real do projeto.

Para criar uma sala, o tutorial deverá explicar:

- Como escolher um ID.
- Onde cadastrar a sala.
- Como definir largura e altura.
- Como criar plataformas.
- Como criar portas.
- Como conectar a outra sala.
- Como definir o spawn.
- Como testar.
- Como verificar o mapa.

Para cadastrar um item:

- Como escolher um ID.
- Como definir nome e descrição.
- Como definir categoria.
- Como definir raridade.
- Como definir atributos.
- Como adicionar ao catálogo.
- Como testar no inventário.

Para criar um baú:

- Como definir ID.
- Como escolher a sala.
- Como posicionar.
- Como definir recompensa.
- Como garantir coleta única.
- Como testar save/load.

Para criar uma sala secreta:

- Como criar a sala.
- Como esconder a entrada.
- Como configurar o requisito.
- Como conectar ao grafo.
- Como impedir revelação antecipada.
- Como testar a descoberta.

Para criar uma loja:

- Como criar o NPC.
- Como definir o diálogo.
- Como criar o estoque.
- Como cadastrar mercadorias.
- Como definir preços.
- Como testar compra e venda.

Todos os exemplos deverão corresponder à estrutura real dos arquivos RON.

---

# 32. VALIDAÇÃO DOS TUTORIAIS

Não basta escrever os arquivos.

Você deverá verificar que os exemplos são utilizáveis.

Para cada tutorial:

1. Confirme os caminhos.
2. Confirme os nomes dos campos.
3. Confirme a sintaxe.
4. Confirme os IDs utilizados.
5. Confirme as dependências.
6. Confirme o comando de teste.
7. Confirme que não exige alterar saves reais.
8. Confirme que não modifica assets protegidos.

Quando possível, valide exemplos por testes automatizados ou por parsing dos dados.

Não modifique o mundo principal apenas para testar exemplos didáticos.

Use fixtures isoladas.

Se uma funcionalidade ainda não existir, marque o tutorial como pendente.

Não invente uma implementação.

---

# 33. ÍNDICE DOS TUTORIAIS

O arquivo:

`Md-usario/tutorias/00_LEIA_PRIMEIRO.md`

deverá funcionar como uma página inicial.

Inclua:

- O que é cada pasta do projeto.
- O que é Rust.
- O que é Bevy.
- O que é RON.
- Como iniciar o jogo.
- Como testar alterações.
- Como encontrar os arquivos.
- Como usar os tutoriais.
- Ordem recomendada de aprendizado.
- Cuidados para não perder trabalho.
- Links relativos para cada tutorial.

Explique tudo de forma simples.

---

# 34. MANUTENÇÃO PERMANENTE DOS TUTORIAIS

Esta regra passa a valer para todas as versões futuras.

Sempre que criar ou modificar uma funcionalidade que o proprietário possa configurar:

1. Localize o tutorial correspondente.
2. Atualize os passos.
3. Atualize os exemplos.
4. Atualize os caminhos.
5. Atualize os comandos.
6. Atualize o índice.
7. Valide o conteúdo.
8. Informe a alteração no relatório.

Não permita que os tutoriais fiquem desatualizados.

Não modifique outros arquivos de `Md-usario/`.

---

# 35. TESTES AUTOMATIZADOS

Implemente testes para:

- Carregamento de NPCs.
- IDs duplicados.
- NPC em sala inválida.
- Distância de interação.
- Interações simultâneas.
- Diálogos.
- Escolhas.
- Condições.
- Abertura e fechamento de loja.
- Compra.
- Venda.
- Saldo insuficiente.
- Estoque esgotado.
- Item inválido.
- Preço inválido.
- Overflow.
- Compra duplicada.
- Venda duplicada.
- Equipamento comprado.
- Item vendido.
- Save/load.
- Retorno à cidade.
- Troca de sala durante diálogo.
- Bloqueio de combate durante loja.
- Regra de duas mãos.
- Progressão do mundo.
- Validação dos tutoriais.

Não enfraqueça testes existentes.

---

# 36. TESTE DE ROTA REAL

Crie um exemplo automatizado.

Sugestão:

`examples/world_town_route.rs`

A rota deverá:

1. Iniciar no castelo.
2. Chegar à cidade usando movimento real.
3. Encontrar um NPC.
4. Iniciar diálogo com D.
5. Escolher uma opção.
6. Entrar na loja.
7. Consultar equipamentos.
8. Comprar um item.
9. Equipar o item.
10. Confirmar atributos.
11. Vender um item permitido.
12. Confirmar saldo.
13. Salvar.
14. Sair da cidade.
15. Carregar.
16. Confirmar inventário, dinheiro e estado da loja.
17. Retornar ao castelo.

Use entradas reais.

Não teletransporte o jogador artificialmente.

Não injete dinheiro ou itens diretamente durante a rota, exceto por fixtures isoladas claramente identificadas como testes, que não poderão ser contabilizadas como rota real.

O contador de reposicionamentos artificiais deverá ser zero.

---

# 37. VALIDAÇÃO VISUAL

Execute o jogo real com janela.

Produza capturas mostrando:

- Entrada da cidade.
- Praça.
- NPC.
- Diálogo.
- Escolhas.
- Loja.
- Comprar.
- Vender.
- Saldo.
- Item comprado.
- Equipamento.
- Mapa da cidade.
- Save/load.
- Retorno ao castelo.

Abra e inspecione as capturas.

Não declare validação visual apenas porque o processo retornou código zero.

Se possível, produza uma demonstração curta.

---

# 38. REGRESSÃO

Preserve:

- Movimento humano.
- Salto.
- Corrida.
- Combate.
- Técnicas.
- Magias.
- Defesa.
- Esquiva.
- Dano.
- Morte.
- Revive.
- Inimigos.
- Inventário.
- Equipamentos.
- Forma Espectral.
- Baús.
- Paredes quebráveis.
- Segredos.
- Alavancas.
- Atalhos.
- Mapa.
- Save V1/V2.
- Câmera.
- Animações.
- VFX.
- Pipeline artístico.

Não altere a física humana.

Não modifique a fórmula geral de dano.

Não altere o comportamento da espada inicial.

---

# 39. COMANDOS DE VALIDAÇÃO

Execute sequencialmente:

```bash
export CARGO_BUILD_JOBS=2

cargo fmt --all -- --check
cargo check -j 2
cargo clippy --all-targets -j 2 -- -D warnings
cargo test -j 2
```

Execute também:

- Testes Python do pipeline artístico.
- Autoplay.
- Visual Check.
- World Report.
- World Main Route.
- World Secrets Route.
- World Spectral Route.
- World Equipment Route.
- World Town Route.
- Testes de save.
- Testes de economia.
- Testes de NPCs.
- Testes de diálogo.
- Testes de comércio.
- Testes de tutoriais.

Não execute comandos Cargo simultaneamente.

Não apague caches.

---

# 40. DOCUMENTAÇÃO TÉCNICA

Crie:

`docs/NPC_COMMERCE_V2_1.md`

Documente:

1. Auditoria.
2. Arquitetura.
3. NPCs.
4. Interações.
5. Diálogos.
6. Escolhas.
7. Cidade.
8. Conexões.
9. Economia.
10. Save.
11. Lojas.
12. Estoque.
13. Compras.
14. Vendas.
15. Preços.
16. Interface.
17. Equipamentos.
18. Armas de duas mãos.
19. Aparência pendente.
20. Drops.
21. Mapa.
22. Testes.
23. Rota.
24. Performance.
25. Tutoriais.
26. Riscos.
27. Próximas etapas.

Atualize documentos existentes afetados.

Registre decisões em `docs/DECISIONS.md`.

Não reescreva documentação inteira sem necessidade.

---

# 41. CRITÉRIOS DE ACEITAÇÃO

A V2.1 somente estará concluída quando:

- [ ] Estado inicial auditado.
- [ ] Arquivos protegidos preservados.
- [ ] Compilações limitadas a dois jobs.
- [ ] NPCs implementados.
- [ ] Interação pela tecla D.
- [ ] Diálogos funcionais.
- [ ] Escolhas funcionais.
- [ ] Cidade conectada ao castelo.
- [ ] Cidade navegável.
- [ ] Loja funcional.
- [ ] Compra funcional.
- [ ] Venda funcional.
- [ ] Economia persistente.
- [ ] Estoque consistente.
- [ ] Interface de comércio.
- [ ] Comparação de equipamentos.
- [ ] Integração com inventário.
- [ ] Regra de armas de duas mãos.
- [ ] Progressão preservada.
- [ ] Save V1/V2 preservado ou migração explicitamente autorizada.
- [ ] Rota real sem reposicionamentos artificiais.
- [ ] Testes automatizados passando.
- [ ] Validação visual.
- [ ] Performance medida.
- [ ] Pasta `Md-usario/tutorias/` criada.
- [ ] Tutoriais individuais criados.
- [ ] Exemplos verificados.
- [ ] Índice de tutoriais criado.
- [ ] Documentação atualizada.
- [ ] Nenhum commit.
- [ ] Nenhum push.
- [ ] V2.2 não iniciada.

Não marque uma funcionalidade como concluída se depender de uma migração de save ainda não autorizada.

Se necessário, declare conclusão parcial.

---

# 42. RELATÓRIO FINAL

Entregue:

**A. Estado inicial e Git**

**B. Auditoria da V2.0**

**C. Arquitetura de NPCs**

**D. Interações**

**E. Diálogos**

**F. Escolhas**

**G. Cidade**

**H. Conexões**

**I. Economia**

**J. Save e migração**

**K. Lojas**

**L. Estoque**

**M. Compra**

**N. Venda**

**O. Preços**

**P. Interface**

**Q. Equipamentos**

**R. Regra de duas mãos**

**S. Aparência visual pendente**

**T. Drops**

**U. Mapa**

**V. Testes**

**W. Rota real**

**X. Validação visual**

**Y. Performance**

**Z. Tutoriais criados**

**AA. Validação dos tutoriais**

**AB. Arquivos criados e modificados**

**AC. Estado final do Git**

**AD. Pendências e riscos**

**AE. Próximos passos recomendados**

Inclua comandos executados e resultados reais.

Liste cada tutorial criado e seu caminho.

Informe quais exemplos foram testados e quais são apenas documentais.

Não declare sucesso sem evidência.

---

# 43. CONDIÇÕES DE PARADA

Interrompa a operação afetada se:

- O Save V2 precisar de mudança incompatível.
- Houver risco de perda de saves.
- Houver risco de perda de trabalho Git.
- A economia não puder ser persistida corretamente.
- Uma compra puder duplicar itens.
- Uma venda puder gerar dinheiro infinito.
- Uma transição comprometer o estado da loja.
- Uma dependência nova for indispensável.
- A memória ficar insuficiente.
- Uma correção exigir reescrever o combate.
- O pipeline artístico exigir sobrescrever assets protegidos.

Documente o impedimento.

Continue apenas tarefas independentes e seguras.

Solicite autorização antes de mudanças fora do escopo.

---

# 44. INSTRUÇÃO FINAL

Você está autorizado a executar a V2.1.

Ordem obrigatória:

1. Auditoria.
2. Segurança.
3. Inspeção do Save V2.
4. Criação da pasta de tutoriais.
5. Tutoriais dos sistemas existentes.
6. Arquitetura de NPCs.
7. Diálogos.
8. Cidade.
9. Economia, se compatível com o save.
10. Lojas.
11. Compras e vendas.
12. Interface.
13. Regra de duas mãos.
14. Integração com equipamentos.
15. Tutoriais dos novos sistemas.
16. Testes.
17. Rota real.
18. Validação visual.
19. Documentação.
20. Relatório final.

**Não implemente missões completas nesta fase.**

Não implemente aprimoramento de armas.

Não inicie a V2.2.

Não crie Save V3 sem autorização.

Não modifique saves reais.

Não altere a física humana.

Não sobrescreva assets publicados.

Não faça staging.

Não crie commits.

Não faça push.

A V2.1 deve terminar com uma cidade explorável, NPCs funcionais, diálogos, lojas, economia persistente e uma biblioteca de tutoriais simples para criação de conteúdo.

Se a economia exigir migração de save não autorizada, implemente as partes independentes e declare a entrega parcial, sem improvisar persistência incompatível.

Ao concluir, apresente o relatório completo e aguarde minha avaliação.