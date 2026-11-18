# DARK CASTLE RPG — MEGA PROMPT N1
# NOVA FUNDAÇÃO: DIREÇÃO ARTÍSTICA, PIXEL ART 2D, ANIMAÇÕES, EQUIPAMENTOS VISUAIS E CASTELO LABIRÍNTICO

**Projeto:** Dark Castle RPG  
**Etapa:** N1 — Nova Fundação do Projeto  
**Linguagem:** Rust, edição 2024  
**Engine:** Bevy 0.20.1  
**Plataformas:** Windows e Linux  
**Direção visual:** Pixel art 2D de alta fidelidade, inspirada nas gerações de 32 e 64 bits  
**Ferramentas inicialmente preferidas:** Blender + Pixelorama + Bevy  
**Natureza da etapa:** Auditoria, pesquisa, decisões arquiteturais, documentação e planejamento de provas de conceito.

---

# 0. INSTRUÇÃO PRINCIPAL — LEIA ANTES DE EXECUTAR

Você está assumindo a função de:

- Diretor técnico.
- Diretor de arte.
- Arquiteto de software.
- Especialista em animação 2D.
- Especialista em pipelines 3D → 2D.
- Designer de Metroidvania.
- Designer de sistemas de equipamentos.
- Responsável pela documentação técnica.
- Responsável pelo planejamento de produção.

O proprietário decidiu redefinir os fundamentos do Dark Castle RPG.

As etapas D17–D24 estabeleceram um protótipo funcional, mas a direção definitiva do jogo exige uma arquitetura mais ambiciosa.

**A N1 não é uma etapa de implementação massiva.**

Seu objetivo é investigar o projeto existente, identificar os sistemas aproveitáveis e definir um plano tecnicamente viável para construir o jogo desejado.

Não comece a N2 automaticamente.

Não substitua o protagonista atual.

Não produza centenas de sprites.

Não modifique sistemas estáveis apenas para adequá-los a uma arquitetura ainda não aprovada.

Ao terminar, apresente os resultados e aguarde aprovação.

---

# 1. VISÃO DEFINITIVA DO DARK CASTLE RPG

## 1.1 Conceito central

O Dark Castle RPG será um jogo de ação e exploração 2D, com elementos de RPG e Metroidvania.

Sua principal inspiração de gameplay é Castlevania: Symphony of the Night.

Entretanto, o jogo deverá possuir:

- Universo próprio.
- Personagens originais.
- Inimigos originais.
- Castelo original.
- Equipamentos originais.
- Magias originais.
- Trilha sonora original ou devidamente licenciada.
- Direção artística própria.
- Sistemas desenvolvidos especificamente para Rust e Bevy.

O objetivo é alcançar profundidade e qualidade comparáveis às grandes referências do gênero, sem copiar seu conteúdo protegido.

## 1.2 Pilares fundamentais

O jogo deverá priorizar:

1. Exploração labiríntica.
2. Combate responsivo.
3. Animações fluidas.
4. Pixel art detalhada.
5. Equipamentos com aparência visual variável.
6. Progressão por habilidades e relíquias.
7. Segredos e recompensas.
8. Salas grandes e interconectadas.
9. Inimigos variados.
10. Minichefes e chefes principais.
11. Controles personalizáveis.
12. Persistência do progresso.
13. Desempenho adequado.
14. Arquitetura escalável.
15. Ferramentas gratuitas sempre que viáveis.

---

# 2. DECISÃO ARTÍSTICA — PIXEL ART 2D, ESTÉTICA 32/64 BITS

## 2.1 Direção escolhida

O proprietário escolheu uma estética de pixel art 2D detalhada, inspirada na evolução visual dos jogos das gerações de 32 e 64 bits.

A referência mais importante é a riqueza visual de Castlevania: Symphony of the Night.

Entretanto, o projeto não deve ser limitado artificialmente às restrições dos consoles antigos.

Queremos combinar:

- Pixel art clássica.
- Personagens detalhados.
- Cenários góticos.
- Animações suaves.
- Efeitos modernos controlados.
- Iluminação atmosférica.
- Camadas de profundidade.
- Parallax.
- Equipamentos visualmente distintos.
- Boa leitura dos movimentos.

## 2.2 Esclarecimento obrigatório

A expressão "32/64 bits" representa uma referência estética e histórica.

Não representa:

- Tamanho obrigatório de sprite.
- Quantidade fixa de pixels.
- Resolução de tela.
- Profundidade de cor obrigatória.
- Quantidade de quadros por animação.

O jogo poderá usar sprites maiores que 64×64 pixels.

Não restringir o protagonista a sprites pequenos caso isso prejudique a qualidade visual.

## 2.3 Proporções e resolução

Estudar e recomendar:

- Resolução interna.
- Proporção de tela.
- Escala de renderização.
- Tamanho de tiles.
- Dimensões aproximadas do protagonista.
- Dimensões dos inimigos.
- Dimensões dos chefes.
- Escala de objetos.
- Distância da câmera.
- Quantidade de cenário visível.
- Densidade de detalhes.

Comparar pelo menos duas configurações.

Uma hipótese inicial pode ser trabalhar com resolução interna próxima de 640×360 e outra de 960×540, mas essas opções não são decisões finais.

Avaliar legibilidade, custo de produção e escalonamento em monitores modernos.

## 2.4 Pixel perfect

Definir políticas para:

- Filtragem nearest-neighbor.
- Pixel snapping.
- Escala inteira quando possível.
- Movimento de câmera.
- Subpixels.
- Zoom.
- Rotação.
- Efeitos de partículas.
- Renderização de texto.
- Interfaces.
- Efeitos de luz.
- Mistura de pixel art com efeitos de maior resolução.

O resultado não deve apresentar borrões, pixels inconsistentes ou deformações visuais desnecessárias.

---

# 3. IDENTIDADE ARTÍSTICA DO MUNDO

## 3.1 Ambientação

O mundo será uma fantasia sombria gótica.

Elementos visuais:

- Castelos monumentais.
- Torres.
- Bibliotecas antigas.
- Salões nobres.
- Capelas.
- Catacumbas.
- Criptas.
- Laboratórios.
- Jardins abandonados.
- Cisternas.
- Cavernas.
- Passagens subterrâneas.
- Fortificações.
- Ruínas.
- Templos esquecidos.

## 3.2 Linguagem visual

Definir:

- Paletas por região.
- Materiais predominantes.
- Arquitetura.
- Iluminação.
- Atmosfera.
- Contraste entre fundo e plataformas.
- Silhuetas dos inimigos.
- Identidade das relíquias.
- Linguagem dos efeitos mágicos.
- Linguagem visual das armadilhas.

O jogador deve conseguir identificar superfícies sólidas, plataformas atravessáveis, perigos e elementos interativos.

## 3.3 Cenários em camadas

Planejar:

- Fundo distante.
- Arquitetura intermediária.
- Geometria jogável.
- Decoração frontal.
- Névoa.
- Partículas.
- Iluminação.
- Parallax.

Evitar cenários visualmente poluídos que dificultem enxergar inimigos ou ataques.

---

# 4. FERRAMENTAS — PRIORIDADE PARA SOLUÇÕES GRATUITAS

## 4.1 Pipeline preferencial

A preferência inicial do proprietário é:

**Blender + Pixelorama + Bevy.**

Essas ferramentas podem ser utilizadas gratuitamente e são adequadas, em princípio, às seguintes funções:

### Blender

- Modelagem 3D.
- Preparação de manequins.
- Rigging.
- Armatures.
- Animação.
- Equipamentos modulares.
- Câmeras ortográficas.
- Renderização de quadros.
- Automação com Python.

### Pixelorama

- Pixel art.
- Animações 2D.
- Edição de sprites.
- Correção de quadros.
- Camadas.
- Paletas.
- Acabamento visual.

### Bevy

- Engine do jogo.
- Renderização 2D.
- ECS.
- Animações em runtime.
- Equipamentos.
- Combate.
- Mundo.
- Interface.
- Persistência.

## 4.2 Não descartar alternativas

Comparar também:

- LibreSprite.
- Aseprite.
- Spine 2D.
- Godot como ferramenta auxiliar, se houver justificativa.
- Mixamo.
- Hunyuan3D.
- Outras ferramentas livres relevantes.

Aseprite e Spine podem exigir pagamento, dependendo da distribuição e licença.

Não comprar nenhuma ferramenta.

Não instalar novas ferramentas sem necessidade e autorização.

Não migrar o jogo de Bevy para outra engine.

## 4.3 Pesquisa técnica

Verificar, quando houver acesso às fontes:

- Licença atual.
- Custo.
- Compatibilidade.
- Exportação.
- Automação.
- Integração com Rust.
- Integração com Bevy 0.20.1.
- Qualidade das animações.
- Suporte a troca de equipamentos.
- Limitações de produção.

Não inventar compatibilidade.

Não declarar que uma ferramenta possui determinada funcionalidade sem evidência.

---

# 5. COMO PRODUZIR ANIMAÇÕES 2D COM MANEQUIM

Este é um dos problemas técnicos centrais da N1.

O proprietário deseja criar um personagem-base e reutilizar suas animações.

Não queremos desenhar manualmente todas as combinações de personagens, armaduras e armas.

Investigar quatro abordagens.

## Abordagem A — 3D para sprites 2D

Pipeline conceitual:

1. Criar ou importar um manequim 3D.
2. Preparar a geometria.
3. Criar um esqueleto.
4. Aplicar rigging.
5. Criar animações.
6. Equipar roupas e armas.
7. Renderizar cada quadro com câmera ortográfica.
8. Converter os quadros em sprites.
9. Ajustar pixel art.
10. Exportar sprite sheets.
11. Integrar no Bevy.

Vantagens esperadas:

- Reutilização de animações.
- Troca de equipamentos 3D.
- Consistência de proporções.
- Automação.
- Produção de múltiplos movimentos.

Riscos:

- Aparência excessivamente tridimensional.
- Pixels irregulares.
- Falta de personalidade nas poses.
- Artefatos de redução.
- Dificuldade de acabamento.
- Quantidade elevada de quadros e combinações.

## Abordagem B — Rig esquelético 2D

Criar personagem composto por partes.

Exemplos:

- Cabeça.
- Tronco.
- Braços.
- Antebraços.
- Pernas.
- Pés.
- Cabelo.
- Capa.
- Arma.

Animar essas partes por ossos ou transformações.

Vantagens:

- Reutilização.
- Equipamentos intercambiáveis.
- Menor necessidade de redesenho.
- Possibilidade de animações parametrizadas.

Riscos:

- Aparência de boneco recortado.
- Distorção de pixels.
- Articulações artificiais.
- Dificuldade para poses complexas.
- Integração técnica.

## Abordagem C — Animação quadro a quadro

Criar sprites específicos para cada movimento.

Vantagens:

- Controle artístico.
- Silhuetas expressivas.
- Animações estilizadas.
- Qualidade tradicional de pixel art.

Riscos:

- Custo de produção.
- Dificuldade de criar equipamentos modulares.
- Necessidade de muitos quadros.
- Manutenção trabalhosa.

## Abordagem D — Pipeline híbrido

Combinar:

- Manequim 3D.
- Animações reutilizáveis.
- Renderização ortográfica.
- Correção em Pixelorama.
- Camadas modulares no Bevy.
- Quadros específicos desenhados manualmente quando necessário.

Esta é a hipótese preferencial para investigação.

**Não escolher definitivamente antes de validar a qualidade.**

---

# 6. ARQUITETURA DE ANIMAÇÕES

Planejar uma arquitetura que suporte:

## Locomoção

- Idle.
- Walk.
- Run.
- Start.
- Stop.
- Turn.
- Crouch.
- CrouchWalk.
- JumpStart.
- JumpRise.
- JumpApex.
- Fall.
- Land.
- DoubleJump.
- Dodge.

## Combate

- AttackLight.
- AttackHeavy.
- Combo.
- Guard.
- GuardHit.
- Hurt.
- Knockback.
- Cast.
- SpecialAttack.
- UseItem.
- Death.

## Exploração

- Interact.
- Climb.
- LedgeGrab, se aprovado futuramente.
- Transform.
- Fly.
- Swim, se aprovado futuramente.

Não implementar todos agora.

Definir como o sistema sincronizará animações com:

- Hitboxes.
- Hurtboxes.
- Eventos.
- Efeitos sonoros.
- Projéteis.
- Custos de habilidades.
- Cancelamentos.
- Estados do jogador.

O visual não deve determinar sozinho a lógica de combate.

A lógica de gameplay precisa continuar confiável mesmo se uma animação visual atrasar.

---

# 7. EQUIPAMENTOS QUE ALTERAM A APARÊNCIA

## 7.1 Requisito obrigatório

Quando um equipamento possuir representação visual, equipá-lo deverá alterar imediatamente a aparência do personagem.

Exemplos:

- Espada curta.
- Espada longa.
- Machado.
- Lança.
- Cajado.
- Escudo.
- Armadura leve.
- Armadura pesada.
- Botas.
- Luvas.
- Capacete.
- Capa.

Não queremos um sistema em que centenas de equipamentos alterem apenas números no inventário.

## 7.2 Personagem modular

Projetar camadas visuais:

1. Corpo-base.
2. Pele.
3. Cabelo.
4. Cabeça.
5. Torso.
6. Braços.
7. Pernas.
8. Luvas.
9. Botas.
10. Armadura.
11. Capa traseira.
12. Capa frontal.
13. Arma principal.
14. Arma secundária.
15. Escudo.
16. Acessórios visíveis.
17. Efeitos mágicos.

A arquitetura deve permitir que essas camadas compartilhem o mesmo estado de animação.

## 7.3 Sincronização

Todas as peças precisam respeitar:

- Animação.
- Frame.
- Direção.
- Pivô.
- Posição.
- Profundidade.
- Escala.
- Pontos de ancoragem.

O sistema deve prever mudanças na ordem de desenho conforme a pose.

Exemplo:

Durante um ataque, o braço ou a arma pode precisar passar visualmente à frente ou atrás do torso.

## 7.4 Famílias de armas

Planejar famílias:

- Espadas de uma mão.
- Espadas de duas mãos.
- Adagas.
- Machados.
- Martelos.
- Lanças.
- Chicotes.
- Arcos.
- Cajados.
- Escudos.

Não criar uma animação completamente nova para cada espada.

Reutilizar famílias quando o movimento for compatível.

## 7.5 Evitar explosão combinatória

Não gerar uma sprite sheet completa para cada combinação possível de equipamentos.

Com 10 armaduras, 10 armas e 10 capas, já teríamos 1.000 combinações.

A solução deve favorecer peças reutilizáveis e composição modular.

Avaliar também:

- Ocultação de camadas.
- Compatibilidade de armaduras.
- Sobreposição de tecidos.
- Ajuste de pivôs.
- Deformações.
- Equipamentos volumosos.
- Armas de duas mãos.
- Exceções artísticas.

## 7.6 Dados

Propor campos como:

- visual_id.
- equipment_slot.
- animation_family.
- sprite_atlas.
- palette_id.
- attachment_point.
- layer_order.
- visible.
- overrides.

Esses nomes são conceituais.

Adaptar às estruturas reais do projeto.

---

# 8. PROVA DE CONCEITO VISUAL

Antes de produzir centenas de itens, planejar um experimento pequeno.

A prova de conceito deverá incluir:

- Um personagem-base.
- Uma animação Idle.
- Uma animação Walk.
- Uma animação Jump.
- Uma animação Attack.
- Duas armas.
- Duas armaduras.
- Uma capa.
- Troca de equipamentos em runtime.
- Animações sincronizadas.
- Renderização em Bevy.

## Perguntas obrigatórias

1. Parece pixel art de qualidade?
2. O personagem possui silhueta reconhecível?
3. O movimento parece natural?
4. As armas acompanham as mãos?
5. As roupas permanecem alinhadas?
6. As camadas se sobrepõem corretamente?
7. O sistema permite novos equipamentos facilmente?
8. O custo de produção é aceitável?
9. O processo pode ser automatizado?
10. O resultado funciona em Windows e Linux?

A N1 deve especificar o experimento, não executá-lo integralmente.

---

# 9. PROTÓTIPO 3D EXISTENTE

Existe o arquivo:

`Md-usario/assets-user/prota/base/white_mesh.glb`

Ele deve permanecer intacto.

Auditar somente por leitura, se tecnicamente possível:

- Geometria.
- Quantidade de meshes.
- Materiais.
- Esqueleto.
- Animações.
- Topologia.
- Proporções.
- Viabilidade de rigging.

Não presumir que o arquivo está pronto para animação.

Não alterar o original.

Não iniciar Hunyuan3D.

Não abrir Blender para modificar o modelo nesta etapa.

Se for necessário testar, propor uma cópia isolada para etapa futura.

---

# 10. CASTELO LABIRÍNTICO — PILAR CENTRAL DO JOGO

O proprietário deseja um castelo complexo, com exploração semelhante à sensação proporcionada por Symphony of the Night.

Não queremos apenas salas pequenas conectadas em sequência.

## 10.1 Estrutura desejada

O castelo deverá incluir:

- Salas horizontais longas.
- Torres verticais.
- Poços profundos.
- Abismos.
- Plataformas.
- Passagens escondidas.
- Atalhos.
- Elevadores.
- Portas trancadas.
- Salas opcionais.
- Salas de recompensa.
- Salas de descanso.
- Salas de chefes.
- Regiões secretas.
- Rotas que retornam ao começo.

## 10.2 Exploração não linear

O jogador poderá encontrar locais que ainda não consegue alcançar.

Exemplos:

- Um abismo largo demais para o salto normal.
- Uma abertura acima do alcance atual.
- Uma passagem estreita.
- Uma parede que só pode ser destruída com uma habilidade.
- Uma porta que exige uma relíquia.
- Uma região acessível apenas por voo.

Esses obstáculos devem ser compreensíveis e memoráveis.

## 10.3 Retorno a regiões antigas

Ao adquirir uma habilidade, o jogador deve ter motivos para retornar.

Exemplos:

- Pulo duplo abre uma passagem elevada.
- Voo permite atravessar um poço.
- Transformação permite entrar em uma passagem estreita.
- Relíquia revela elementos ocultos.
- Nova técnica permite destruir barreiras.

## 10.4 Design de salas

Cada sala deve ter função.

Possíveis funções:

- Exploração.
- Combate.
- Plataforma.
- Armadilha.
- Recompensa.
- Segredo.
- Atalho.
- Narrativa ambiental.
- Desafio de habilidade.
- Transição.
- Chefe.

Não aumentar salas apenas para criar distância.

## 10.5 Planejamento do castelo

Produzir uma proposta de mapa conceitual original.

Definir:

- Regiões.
- Conexões.
- Progressão.
- Caminhos obrigatórios.
- Caminhos opcionais.
- Atalhos.
- Habilidades necessárias.
- Locais de recompensa.
- Pontos de salvamento.
- Viagem rápida.
- Minichefes.
- Chefes principais.

Não copiar o mapa de Castlevania.

Não implementar o castelo inteiro na N1.

---

# 11. RELÍQUIAS E HABILIDADES DE EXPLORAÇÃO

Planejar um sistema de relíquias que modifica permanentemente as possibilidades de gameplay.

Exemplos:

- Pulo duplo.
- Salto elevado.
- Voo.
- Transformação.
- Resistência ambiental.
- Visão especial.
- Passagem por obstáculos.
- Destruição de barreiras.
- Abertura de portas especiais.

Cada relíquia deverá possuir:

- Identificador.
- Nome.
- Descrição.
- Efeito.
- Requisitos.
- Estado de obtenção.
- Persistência.
- Feedback.
- Integração com o mapa.

Diferenciar relíquias de:

- Equipamentos.
- Consumíveis.
- Itens-chave.
- Tesouros.
- Materiais.

---

# 12. ARMADILHAS, SEGREDOS E RECOMPENSAS

Planejar:

- Espinhos.
- Pisos que desabam.
- Plataformas móveis.
- Projéteis ambientais.
- Pêndulos.
- Lâminas.
- Portas temporizadas.
- Mecanismos.
- Paredes quebráveis.
- Baús ocultos.
- Salas secretas.
- Itens únicos.
- Equipamentos raros.

As recompensas devem ser proporcionais ao desafio e à descoberta.

O mapa não deve revelar automaticamente todos os segredos.

Itens únicos e baús abertos devem possuir persistência.

---

# 13. CHEFES E MINICHEFES

Registrar decisão do proprietário:

**The Ashen Sentinel é um minichefe inicial.**

Não tratá-lo como o primeiro chefe principal do jogo.

Categorias desejadas:

1. Inimigos comuns.
2. Inimigos de elite.
3. Minichefes.
4. Chefes principais.
5. Chefes opcionais.

Chefes principais deverão possuir:

- Arena dedicada.
- Identidade visual própria.
- Padrões de ataque.
- Recompensas.
- Progressão persistente.
- Feedback de vitória.

Minichefes poderão utilizar encontros menores.

Não reclassificar o chefe no código nesta etapa sem auditoria da persistência existente.

---

# 14. CONTROLES — DECISÃO CONFIRMADA

O proprietário escolheu o esquema híbrido.

Padrão desejado:

- A: mover para esquerda.
- D: mover para direita.
- W: pular.
- Espaço: pular.
- S: agachar.
- Mouse esquerdo: ataque básico.

As demais ações devem ser remapeáveis.

Planejar:

- Configuração de teclas.
- Configuração de mouse.
- Múltiplas teclas para uma ação.
- Detecção de conflitos.
- Persistência.
- Restauração de padrões.
- Interface de configuração.
- Compatibilidade futura com gamepad.

As sequências especiais de combate devem continuar relativas à orientação do personagem.

Exemplo:

Baixo → Frente → Ataque.

A sequência precisa funcionar independentemente das teclas físicas escolhidas.

Não implementar a reformulação completa dos controles na N1.

---

# 15. SISTEMAS DE RPG

Preservar e planejar expansão dos sistemas existentes:

- Atributos.
- Níveis.
- EXP.
- HP.
- MP.
- Hearts.
- Equipamentos.
- Inventário.
- Consumíveis.
- Magias.
- Técnicas.
- Drops.
- Comerciantes.
- Salvamento.
- Viagem rápida.

A futura arquitetura visual deve se integrar ao sistema de equipamentos existente.

Não criar um segundo inventário.

Não duplicar os dados de itens.

Não criar sistemas paralelos de progressão.

---

# 16. REFERÊNCIAS TÉCNICAS EXTERNAS

Referências autorizadas para estudo:

https://github.com/BlackLabelHQ/SymphonyRecomp/tree/master

https://www.vgmuseum.com/mrp/2/extras-sotn.htm

Estudar:

- Organização das mecânicas.
- Exploração.
- Animações.
- Comportamento dos itens.
- Combate.
- Progressão.
- Transições.
- Interações.
- Regras de movimentação.

## Direitos autorais

O SymphonyRecomp está relacionado à recompilação de um jogo comercial protegido.

O fato de um repositório estar público não concede automaticamente autorização para reutilizar seus recursos.

Não copiar para o Dark Castle RPG:

- Código recompilado.
- Sprites originais.
- Animações originais.
- Mapas originais.
- Músicas.
- Efeitos sonoros.
- Personagens.
- Recursos extraídos do jogo.

Usar como referência para estudar comportamentos e desenvolver soluções próprias.

Registrar origem e licença de qualquer recurso externo que venha a ser considerado futuramente.

---

# 17. ARQUITETURA TÉCNICA

Propor uma arquitetura compatível com o Bevy 0.20.1.

Áreas de estudo:

## Renderização

- Sprites em camadas.
- Atlases.
- Materiais.
- Ordenação.
- Animações.
- Câmera.
- Pixel snapping.

## Equipamentos

- Dados visuais.
- Slots.
- Famílias de animação.
- Pontos de fixação.
- Troca em runtime.

## Mundo

- Salas de diferentes tamanhos.
- Geometria.
- Conexões.
- Persistência.
- Streaming ou carregamento por região.
- Requisitos de progressão.

## Combate

- Sincronização entre animação e hitbox.
- Cancelamentos.
- Feedback.
- Projéteis.
- Efeitos.

## Persistência

- Relíquias.
- Segredos.
- Baús.
- Chefes.
- Portas.
- Equipamentos.
- Configurações.

Priorizar extensões dos sistemas existentes.

Não propor uma reescrita integral sem justificativa técnica.

---

# 18. DESEMPENHO E ESCALABILIDADE

Investigar:

- Uso de memória.
- Quantidade de texturas.
- Tamanho de sprite sheets.
- Atlases.
- Renderização por camadas.
- Batching.
- Carregamento.
- Descarregamento.
- Cache.
- Custos de animação.
- Efeitos.
- Câmera.
- Salas extensas.

O sistema deve suportar centenas de itens sem carregar todas as combinações visuais possíveis na memória.

Não otimizar prematuramente.

Definir metas e testes mensuráveis.

---

# 19. DOCUMENTAÇÃO OBRIGATÓRIA

Criar:

`docs/development/n1/`

Arquivos:

1. `N1_VISAO_DO_JOGO.md`
2. `N1_DIRECAO_ARTISTICA.md`
3. `N1_PIXEL_ART_32_64_BITS.md`
4. `N1_COMPARACAO_FERRAMENTAS.md`
5. `N1_PIPELINE_3D_PARA_2D.md`
6. `N1_ANIMACAO_ESQUELETICA_2D.md`
7. `N1_PERSONAGEM_MODULAR.md`
8. `N1_EQUIPAMENTOS_VISUAIS.md`
9. `N1_CASTELO_LABIRINTICO.md`
10. `N1_RELIQUIAS_E_EXPLORACAO.md`
11. `N1_CHEFES_E_MINICHEFES.md`
12. `N1_CONTROLES.md`
13. `N1_REFERENCIAS_E_LICENCAS.md`
14. `N1_ARQUITETURA_TECNICA.md`
15. `N1_PROVA_DE_CONCEITO.md`
16. `N1_ROADMAP.md`
17. `N1_RELATORIO_FINAL.md`

Não criar arquivos vazios.

Evitar documentação redundante.

Cada documento deve possuir decisões, alternativas, riscos e recomendações.

---

# 20. ROADMAP INICIAL N2–N12

Preparar roadmap detalhado, sujeito à aprovação do proprietário.

## N2 — Controles personalizáveis

- WASD.
- W/Espaço para pular.
- Mouse.
- Remapeamento.
- Persistência.
- Comandos especiais.

## N3 — Pipeline visual experimental

- Manequim.
- Rigging.
- Animação.
- Exportação.
- Pixel art.
- Testes de qualidade.

## N4 — Personagem modular

- Corpo.
- Armaduras.
- Armas.
- Capas.
- Camadas.
- Sincronização.

## N5 — Equipamentos visuais

- Troca em runtime.
- Famílias de armas.
- Dados visuais.
- Integração com inventário.

## N6 — Arquitetura do castelo

- Salas longas.
- Verticalidade.
- Conexões.
- Progressão.
- Requisitos de acesso.

## N7 — Primeira região labiríntica

- Rotas alternativas.
- Abismos.
- Atalhos.
- Exploração.
- Retorno a regiões anteriores.

## N8 — Relíquias de exploração

- Pulo duplo.
- Novas possibilidades de movimento.
- Portas e obstáculos condicionais.

## N9 — Segredos e armadilhas

- Baús.
- Paredes quebráveis.
- Salas secretas.
- Armadilhas.
- Recompensas persistentes.

## N10 — Minichefes e chefes

- Reclassificação do Ashen Sentinel.
- Categorias.
- Arenas.
- Recompensas.
- Progressão.

## N11 — Transformações

- Arquitetura de transformação.
- Novos movimentos.
- Acesso a regiões especiais.
- Testes de exploração.

## N12 — Integração e polimento

- Combate.
- Equipamentos.
- Animações.
- Exploração.
- Desempenho.
- Playtest humano.

Ajustar a sequência se a auditoria identificar dependências diferentes.

Não executar automaticamente essas etapas.

---

# 21. TESTES E VERIFICAÇÕES DA N1

Como a N1 é principalmente documental, não é necessário criar novos sistemas de gameplay.

Entretanto, verificar:

- Estado do Git.
- Integridade do projeto.
- Estrutura de arquivos.
- Dependências.
- Compatibilidade das propostas.
- Riscos de migração.
- Limitações de ferramentas.
- Viabilidade do pipeline.

Se algum código for alterado excepcionalmente, executar os testes adequados e justificar a alteração.

Não declarar uma ferramenta validada apenas porque sua documentação parece promissora.

---

# 22. SEGURANÇA ABSOLUTA

Não executar:

- Commit.
- Push.
- Merge.
- Rebase.
- Reset destrutivo.
- Limpeza indiscriminada.
- Exclusão de arquivos.
- Migração de engine.
- Refatoração massiva.
- Instalação de ferramentas sem autorização.
- Download de assets protegidos.
- Produção massiva de sprites.
- Geração massiva de modelos 3D.

Preservar:

- `Md-usario/`
- `Md-usario/assets-user/prota/base/white_mesh.glb`
- `saves/`
- Assets existentes.
- Código funcional.
- Documentos anteriores.
- Evidências D17–D24.

Não modificar arquivos do proprietário.

---

# 23. CRITÉRIOS DE ACEITAÇÃO

A N1 estará concluída quando:

- [ ] Estado inicial auditado.
- [ ] Visão do jogo documentada.
- [ ] Direção artística definida.
- [ ] Estética 32/64 bits esclarecida.
- [ ] Resolução e escala propostas.
- [ ] Ferramentas comparadas.
- [ ] Pipeline gratuito recomendado.
- [ ] Métodos de animação comparados.
- [ ] Personagem modular especificado.
- [ ] Equipamentos visuais especificados.
- [ ] Estratégia para centenas de itens definida.
- [ ] Prova de conceito planejada.
- [ ] Castelo labiríntico especificado.
- [ ] Relíquias planejadas.
- [ ] Segredos e recompensas planejados.
- [ ] Categorias de chefes definidas.
- [ ] Ashen Sentinel registrado como minichefe.
- [ ] Controles híbridos registrados.
- [ ] Arquitetura técnica proposta.
- [ ] Riscos documentados.
- [ ] Roadmap N2–N12 preparado.
- [ ] Arquivos protegidos preservados.
- [ ] Sem commit.
- [ ] Sem push.

---

# 24. RELATÓRIO FINAL OBRIGATÓRIO

Ao terminar, apresentar:

## A. Estado inicial
Branch, HEAD e working tree.

## B. Auditoria
Sistemas existentes e reaproveitamento.

## C. Visão
Identidade e pilares do jogo.

## D. Direção artística
Estilo, escala e resolução.

## E. Ferramentas
Comparação e recomendação.

## F. Pipeline
Fluxo de criação de sprites e animações.

## G. Personagem
Arquitetura modular.

## H. Equipamentos
Estratégia de aparência variável.

## I. Animações
Famílias, estados e sincronização.

## J. Castelo
Arquitetura labiríntica.

## K. Relíquias
Progressão e exploração.

## L. Chefes
Categorias e decisões.

## M. Controles
Esquema híbrido e remapeamento.

## N. Referências
Licenciamento e limites de reutilização.

## O. Prova de conceito
Experimento recomendado.

## P. Roadmap
N2–N12.

## Q. Riscos
Problemas técnicos e dependências.

## R. Arquivos
Criados e modificados.

## S. Segurança
Estado final do Git, saves e assets.

## T. Aprovações necessárias
Decisões que dependem do proprietário.

---

# 25. DIRETRIZ FINAL

A N1 deve estabelecer a fundação artística e técnica do Dark Castle RPG.

A prioridade não é quantidade de conteúdo.

A prioridade é garantir que os próximos anos de desenvolvimento possam utilizar uma arquitetura adequada.

O projeto deve ser capaz de suportar:

- Pixel art 2D detalhada.
- Animações fluidas.
- Equipamentos visualmente intercambiáveis.
- Centenas de itens.
- Castelo extenso.
- Exploração não linear.
- Relíquias.
- Segredos.
- Inimigos variados.
- Chefes.
- Controles configuráveis.
- Persistência.
- Expansão contínua.

**Não transformar o jogo em 3D.**

O Blender poderá ser utilizado como ferramenta auxiliar de produção, mas o produto final continuará sendo um jogo 2D.

**Não substituir o personagem por um modelo 3D renderizado em tempo real sem autorização.**

**Não iniciar N2 sem aprovação.**

**Não fazer commit nem push.**

Ao finalizar, entregar a documentação, o relatório e as recomendações.

**OBJETIVO FINAL: criar uma fundação profissional para um Metroidvania original em pixel art 2D de alta fidelidade, com animação modular, equipamentos visíveis e um castelo verdadeiramente labiríntico.**