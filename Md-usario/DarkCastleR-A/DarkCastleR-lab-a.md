# DARK CASTLE RPG — LABORATÓRIO DE DIREÇÃO ARTÍSTICA
## Personagem, interface, armas, armaduras, magias, transformações e inimigos

Você será responsável por preparar um laboratório visual de alternativas para o Dark Castle RPG, desenvolvido em Rust com Bevy.

O objetivo é permitir que eu, proprietário do projeto, compare exemplos reais e escolha a direção artística que mais me agrada.

**Não quero que você substitua a arte atual imediatamente. Primeiro apresente alternativas.**

## 1. Contexto e decisões

O jogo é um Metroidvania 2D de fantasia gótica, inspirado em aspectos de Castlevania: Symphony of the Night, mas com identidade própria.

O cenário de fundo da V2.3 foi aprovado como direção artística.

Quero preservar:

- Atmosfera gótica.
- Colunas e arquitetura do castelo.
- Parallax.
- Névoa.
- Iluminação.
- Sensação de exploração.
- Câmera e enquadramento atuais, salvo ajustes justificados.

Estou insatisfeito principalmente com:

- Aparência atual do protagonista.
- Apresentação dos botões e controles.
- Visual das armas e armaduras.
- Apresentação das magias.
- Efeitos das transformações.
- Variedade visual dos inimigos.
- Habilidades e efeitos dos inimigos.

Também quero investigar se a produção visual seria mais simples e sustentável usando pixel art ou Blender.

## 2. Segurança obrigatória

Antes de começar:

- Verifique o Git.
- Confirme o HEAD esperado `1bf2ed2`.
- Preserve todos os arquivos pessoais.
- Preserve os assets publicados.
- Preserve Save V3.
- Não altere código de gameplay.
- Não substitua sprites existentes.
- Não altere balanceamento.
- Não crie commits.
- Não execute push.

A pasta `art-src/` já contém arquivos pessoais. Crie apenas um novo subdiretório `art-src/visual-prototypes/`, depois de confirmar que ele não existe e não contém trabalho anterior.

Não altere outros conteúdos dessa pasta.

## 3. Auditoria da arte atual

Antes de criar qualquer alternativa, investigue:

- Como o protagonista atual foi produzido.
- Se usa Blender.
- Como são exportados os sprites.
- Quais animações existem.
- Quantos quadros cada animação possui.
- Como as armas são anexadas.
- Como armaduras e acessórios aparecem.
- Como a capa é renderizada.
- Como os efeitos são desenhados.
- Como a escala do personagem se relaciona com a câmera.
- Quais ferramentas de arte já existem no repositório.

Consulte os documentos técnicos de arte e equipamentos.

Não presuma que trocar a resolução do sprite seja suficiente para melhorar o visual.

## 4. Protótipos do protagonista

Crie alternativas comparáveis:

### A — Pixel art 32×32

Personagem gótico compacto, com silhueta clara e cores legíveis.

### B — Pixel art 64×64

Personagem mais detalhado, com armadura, cabelo, capa e arma identificáveis.

### C — Pixel art de personagem alto

Experimente uma resolução como 64×96 ou 64×128, caso seja mais adequada às proporções do protagonista.

### D — Blender aprimorado

Use o pipeline atual, quando viável, para produzir uma versão visualmente mais polida.

Investigue:

- Silhueta.
- Proporções.
- Materiais.
- Iluminação.
- Contraste.
- Espessura dos membros.
- Legibilidade em movimento.
- Compatibilidade com equipamentos.
- Custo de animação.
- Facilidade de manutenção.

Se o pipeline atual não permitir gerar uma alternativa Blender de qualidade, documente a limitação em vez de apresentar um resultado ruim como definitivo.

### Comparação obrigatória

Apresente as alternativas:

- Na mesma escala visual.
- Com o mesmo fundo.
- Na mesma pose.
- Em movimento, quando possível.
- Com uma espada equipada.
- Com uma armadura alternativa.

Não escolha a vencedora por mim.

## 5. Interface e controles

Crie pelo menos três propostas de interface:

1. Gótica clássica, com molduras ornamentadas.
2. Minimalista, com barras e ícones discretos.
3. Híbrida, com aparência gótica e alta legibilidade.

Cada proposta deve demonstrar:

- HP.
- MP.
- Hearts.
- EXP.
- Arma equipada.
- Magia selecionada, quando aplicável.
- Cooldowns.
- Estado de transformação.
- Feedback dos comandos especiais.

O texto técnico de controles que hoje aparece permanentemente na HUD deve ser avaliado para possível migração para tutorial, menu de controles ou modo de desenvolvimento.

Não remova informações de gameplay sem apresentar uma alternativa.

## 6. Magias por sequências de botões

Quero que o sistema de magias tenha uma experiência semelhante à ideia de comandos especiais de Castlevania: Symphony of the Night.

Audite o sistema atual:

- Parser de comandos.
- Buffer de entrada.
- Orientação relativa.
- Prioridade.
- Custos de MP.
- Cooldowns.
- Estado de conjuração.
- Execução de efeitos.
- Feedback de comandos inválidos.
- Comandos já existentes nos dados RON.

Separe o que já funciona do que falta implementar.

Proponha comandos originais para:

- Projétil elemental.
- Cura.
- Escudo.
- Explosão em área.
- Magia sombria.
- Invocação.
- Magia de transformação.

Não copie sequências, nomes, sprites ou efeitos protegidos de outros jogos.

Crie exemplos visuais mostrando:

- Preparação.
- Conjuração.
- Projétil ou efeito.
- Impacto.
- Recuperação.

Não altere o sistema de comandos nesta fase.

## 7. Armas e armaduras

Crie exemplos visuais para diferentes categorias de armas existentes no projeto.

Investigue quais categorias estão realmente implementadas.

Demonstre, quando possível:

- Espada.
- Adaga.
- Machado.
- Lança.
- Arma pesada.
- Escudo.

Crie também exemplos de armaduras:

- Leve.
- Média.
- Pesada.
- Gótica ornamentada.

Avalie:

- Como a arma acompanha a mão.
- Como os equipamentos se sobrepõem ao personagem.
- Como a capa se movimenta.
- Como as peças mudam sem exigir redesenhar todas as animações.
- Como os efeitos elementais aparecem nas armas.
- Como evitar clipping.

Compare pixel art e Blender quanto à produção de equipamentos.

## 8. Transformações

Audite a transformação espectral já existente.

Crie propostas visuais para:

- Transição da forma humana para a espectral.
- Forma espectral em repouso.
- Movimento.
- Ataque ou habilidade.
- Retorno à forma humana.

Explore:

- Transparência.
- Partículas.
- Distorção.
- Rastros.
- Iluminação.
- Silhueta.
- Paleta de cores.

Proponha também duas ideias de transformações futuras, claramente identificadas como conceitos não implementados.

Não altere os custos de MP ou as regras atuais de transformação.

## 9. Inimigos e habilidades

Audite os inimigos existentes e seus comportamentos reais.

Crie exemplos para diferentes arquétipos:

- Inimigo corpo a corpo.
- Inimigo com escudo.
- Inimigo à distância.
- Inimigo mágico.
- Inimigo voador.
- Inimigo pesado.
- Mini-chefe conceitual.

Para cada arquétipo, apresente:

- Silhueta.
- Animação ou pose de ataque.
- Antecipação do golpe.
- Efeito visual.
- Reação ao dano.
- Possível habilidade especial.
- Nível de dificuldade de implementação.

Não implemente IA nova nesta tarefa.

## 10. Pasta de entregas

Crie:

`art-src/visual-prototypes/`

Com as subpastas:

- `01_personagem/`
- `02_interface/`
- `03_armas_armaduras/`
- `04_magias/`
- `05_transformacoes/`
- `06_inimigos/`
- `07_comparacoes/`

Em cada subpasta, inclua imagens reais de exemplos sempre que tecnicamente possível.

Gere um catálogo principal:

`art-src/visual-prototypes/CATALOGO.md`

Cada opção deve receber um código único, por exemplo:

- CHAR-A
- CHAR-B
- HUD-A
- MAGIC-A
- ARMOR-A
- ENEMY-A

Inclua:

- Prévia visual ou link relativo para a imagem.
- Descrição.
- Técnica utilizada.
- Ferramentas necessárias.
- Compatibilidade com o projeto.
- Estimativa relativa de esforço.
- Vantagens.
- Desvantagens.
- Status: protótipo, não aprovado.

Crie também um arquivo:

`art-src/visual-prototypes/ESCOLHAS.md`

Com uma tabela para eu registrar as opções escolhidas futuramente.

## 11. Comparações visuais

Prepare pranchas lado a lado para:

- Personagem 32×32 versus 64×64 versus personagem alto versus Blender.
- Três estilos de HUD.
- Equipamentos.
- Efeitos mágicos.
- Transformação espectral.
- Inimigos.

Priorize legibilidade e consistência de escala.

Use o cenário gótico atual como referência visual, sem modificar seus arquivos.

## 12. Honestidade sobre ferramentas

Use Blender, scripts de arte e ferramentas disponíveis no ambiente.

Se não conseguir gerar imagens reais de determinada opção:

- Explique o motivo.
- Não invente capturas.
- Não apresente placeholders como arte concluída.
- Identifique o que seria necessário para produzir o exemplo.

Prefira alguns exemplos bem executados a dezenas de imagens genéricas.

Não use assets protegidos de Castlevania.

## 13. Planejamento de implementação

Após produzir os protótipos, proponha uma sequência futura de trabalho:

1. Definição do estilo do protagonista.
2. Pipeline de animação.
3. Equipamentos visuais.
4. Interface.
5. Magias e efeitos.
6. Transformações.
7. Inimigos.
8. Polimento e integração.

Não execute essa implementação agora.

Não altere o roadmap oficial sem autorização.

## 14. Relatório final

Entregue:

A. Auditoria da arte atual.

B. Comparação pixel art versus Blender.

C. Protótipos do personagem.

D. Propostas de HUD.

E. Exemplos de armas e armaduras.

F. Magias e comandos atuais versus propostos.

G. Transformações.

H. Inimigos e habilidades.

I. Lista de imagens e arquivos gerados.

J. Caminho do catálogo visual.

K. Ferramentas utilizadas.

L. Limitações encontradas.

M. Custos relativos de implementação.

N. Recomendações técnicas.

O. Estado final do Git.

**Não altere o jogo principal, não crie commits e não execute push.**

Aguarde minhas escolhas visuais antes de substituir qualquer arte existente.
