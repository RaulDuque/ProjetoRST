# DARK CASTLE RPG — MEGA PROMPT N3.8
## INTEGRAÇÃO DA CORRIDA APROVADA E PREPARAÇÃO DA ARQUITETURA MODULAR ESTILO TERRARIA

**Projeto:** Dark Castle RPG  
**Etapa:** N3.8 — Production Run Integration & Modular Architecture Preparation  
**Branch:** `art/v1.2-protagonist`  
**Engine:** Bevy 0.20.1  
**Linguagem:** Rust 2024  
**Próxima etapa:** N4 — Modular Character System  
**N5:** não iniciar

---

# 0. MISSÃO

A N3.7 consolidou a direção artística do protagonista e permitiu comparar diferentes velocidades de reprodução da corrida.

O proprietário avaliou os resultados e tomou uma decisão definitiva:

**A CORRIDA DE SEIS QUADROS A 15 FPS ESTÁ ARTISTICAMENTE APROVADA.**

Agora devemos integrar essa corrida ao jogo principal e preparar a arquitetura necessária para que o personagem futuramente utilize um sistema de equipamentos visuais inspirado no Terraria.

A integração da corrida e o planejamento da modularidade são dois objetivos diferentes.

Nesta etapa:

1. Integrar a corrida aprovada ao jogo real.
2. Preservar integralmente os sprites originais.
3. Garantir temporização correta.
4. Validar escala, pivô, espelhamento e transições.
5. Verificar compatibilidade com equipamentos existentes.
6. Auditar a arquitetura atual de animação e aparência.
7. Projetar a futura modularidade inspirada no Terraria.
8. Documentar a migração para spritesheets em camadas.
9. Produzir evidências reais.
10. Executar testes.
11. Publicar os resultados no GitHub.

**Não implementar a modularidade completa na N3.8. Essa será a missão da N4.**

---

# 1. DECISÕES DEFINITIVAS DO PROPRIETÁRIO

## 1.1 Corrida

A corrida aprovada utiliza:

- Seis quadros originais da N3.6.
- Índices do vídeo: 48, 50, 52, 54, 56 e 58.
- Canvas: 112×128.
- Pivô: `(56, 118)`.
- Altura artística aproximada: 106 texels.
- Reprodução: 15 FPS.
- Ciclo: 0,4 segundo.
- Cadência: 5 passos por segundo.
- Velocidade horizontal: 310 unidades/s.

O deslizamento estimado dos pés é de aproximadamente 114 u/s, ou 37%.

Essa limitação é conhecida e foi aceita pelo proprietário ao escolher a cadência visual.

Não modificar a física para eliminá-la.

Não aumentar automaticamente o FPS.

Não regenerar os quadros.

Não alterar os pixels aprovados.

## 1.2 Identidade artística

Preservar:

- Cabelo branco-prateado.
- Rosto masculino sério em perfil.
- Camisa cinza detalhada.
- Braceletes.
- Calça escura.
- Botas altas.
- Anatomia humana estilizada.
- Sombras e modelado.
- Movimento natural do cabelo.
- Silhueta original.

A aparência N3.5 e a corrida N3.6 constituem as referências artísticas aprovadas.

## 1.3 Modularidade futura

O proprietário decidiu que **todas as partes visuais do personagem deverão poder ser removidas ou substituídas**.

Isso inclui:

- Cabelo.
- Camisa.
- Calça.
- Botas.
- Luvas.
- Braceletes.
- Casacos.
- Armaduras.
- Elmos.
- Capuzes.
- Capas.
- Armas.
- Escudos.
- Acessórios visíveis.

Não reinterpretar essa decisão como roupa básica permanentemente fixa.

---

# 2. AUDITORIA INICIAL OBRIGATÓRIA

Antes de qualquer alteração:

1. Confirmar branch.
2. Confirmar HEAD local.
3. Confirmar HEAD remoto.
4. Inspecionar `git status`.
5. Identificar alterações preexistentes.
6. Ler `CLAUDE.md`.
7. Ler a arquitetura geral do projeto.
8. Ler os relatórios N3.5, N3.6 e N3.7.
9. Ler `N3_7_TESTE_15FPS.md`.
10. Inspecionar os sprites aprovados.
11. Inspecionar os manifestos de animação.
12. Inspecionar o código real de apresentação do jogador.
13. Inspecionar os sistemas de equipamentos existentes.
14. Inspecionar os soquetes e camadas atuais.
15. Verificar como `cargo run` apresenta o protagonista.

Documentos particularmente relevantes:

- `docs/development/n1/N1_PERSONAGEM_MODULAR.md`
- `docs/development/n1/N1_EQUIPAMENTOS_VISUAIS.md`
- `docs/development/n3_6/N3_6_RELATORIO_FINAL.md`
- `docs/development/n3_7/N3_7_RELATORIO_FINAL.md`
- `docs/development/n3_7/N3_7_TESTE_15FPS.md`

Inspecionar também os documentos de animação esquelética da N1, se presentes.

Não presumir nomes de arquivos ou APIs sem verificar.

Se a branch avançou desde o último relatório, trabalhar sobre o estado atual, sem sobrescrever alterações posteriores.

---

# 3. INTEGRAÇÃO DA CORRIDA NO JOGO PRINCIPAL

A corrida aprovada não deve permanecer apenas no exemplo experimental.

Integrá-la ao pipeline real de apresentação do jogador.

Ao executar:

`cargo run`

O personagem deve utilizar os seis quadros aprovados quando estiver no estado de corrida.

Não basta alterar o exemplo `n3_visual_proof`.

A implementação deve:

- Reutilizar o controlador de animação existente.
- Reutilizar manifestos existentes quando possível.
- Não criar um segundo sistema de animação.
- Manter a lógica de movimento atual.
- Não alterar combate.
- Não alterar input.
- Não alterar colisões.
- Não alterar progressão.
- Não alterar o save.

Se houver uma incompatibilidade estrutural grave com equipamentos atuais, não esconder o problema.

Registrar a incompatibilidade e utilizar uma integração controlada, com fallback seguro, até que a N4 resolva a modularidade.

---

# 4. TEMPORIZAÇÃO EXATA DE 15 FPS

A corrida deve apresentar seis quadros em 0,4 segundo.

Cada quadro tem duração ideal de:

`1 / 15 segundo`

Utilizar a precisão disponível no motor de animação.

Evitar erros cumulativos de arredondamento.

Evitar avanço duplo do relógio.

Evitar reiniciar o ciclo a cada atualização de velocidade.

Evitar saltar quadros indevidamente em FPS variável.

Adicionar testes para:

- Quantidade de quadros.
- Duração total.
- Duração individual.
- Loop.
- Reprodução contínua.
- Estabilidade em diferentes taxas de atualização.

---

# 5. PIVÔ, ESCALA E CÂMERA

A N3.5 utiliza:

- Canvas 80×128.
- Pivô `(40, 118)`.

A corrida N3.6 utiliza:

- Canvas 112×128.
- Pivô `(56, 118)`.

Essas diferenças devem ser respeitadas.

Não alinhar pelo canto da textura.

Não recalcular o pivô usando a bounding box opaca de cada quadro.

Não mover individualmente os quadros para esconder o balanço natural.

Manter a câmera principal em 405.

A câmera experimental 360 continua disponível apenas para provas visuais.

Não modificar `camera.ron`.

Validar a apresentação em ambas as escalas.

---

# 6. ESPELHAMENTO

O personagem deve correr para a direita e para a esquerda.

Verificar:

- Pivô após espelhamento.
- Posição dos pés.
- Alinhamento do cabelo.
- Posição das mãos.
- Compatibilidade com armas.
- Compatibilidade com escudos.
- Ausência de deslocamento horizontal inesperado.

Não criar uma segunda spritesheet espelhada se o sistema existente já suporta espelhamento correto.

Documentar limitações de equipamentos assimétricos para a N4.

---

# 7. TRANSIÇÕES DE ESTADO

Validar:

1. Idle → corrida.
2. Corrida → idle.
3. Corrida → salto.
4. Corrida → ataque.
5. Corrida → guarda.
6. Corrida → esquiva.
7. Corrida → dano.
8. Corrida → morte.
9. Corrida → mudança de direção.

A N3.7 identificou quadros reais de início de corrida no vídeo original.

Reutilizá-los somente quando a transição for compatível com a lógica e o tempo do jogo.

Não inventar uma animação de frenagem.

Não inventar idle animado.

Se uma transição continuar visualmente abrupta, documentar o problema.

Não permitir que uma animação bloqueie comandos ou ações existentes.

---

# 8. COMPATIBILIDADE COM EQUIPAMENTOS

Inspecionar os mecanismos já existentes:

- `VisualPlayback`.
- `CapeLayer`.
- `HeldLayer`.
- `WeaponForm`.
- Soquetes.
- Camadas de armas.
- Camadas de escudo.
- Slots de equipamentos.

A N1 já documenta que armas e capas podem ser representadas separadamente.

A corrida atual, entretanto, contém roupas e cabelo incorporados à imagem.

Não confundir essas duas situações.

**O protagonista ainda não é totalmente modular.**

Na N3.8, identificar quais partes estão incorporadas à spritesheet e quais já são independentes.

Não tentar remover automaticamente todas as roupas.

Não iniciar a reconstrução anatômica completa.

Essa será uma responsabilidade da N4.

---

# 9. NOVA DIRETRIZ ARQUITETURAL — REFERÊNCIA TERRARIA

O proprietário escolheu o Terraria como referência de funcionamento para o sistema visual de equipamentos.

A N4 deverá seguir o conceito de composição de sprites em camadas.

O objetivo não é copiar o código ou os recursos artísticos do Terraria.

O objetivo é utilizar os mesmos princípios gerais:

- Um personagem.
- Um estado de animação compartilhado.
- Várias regiões visuais.
- Equipamentos independentes.
- Spritesheets por peça ou família visual.
- Regras de sobreposição.
- Ocultação de regiões cobertas.
- Armas e acessórios separados.
- Possibilidade de aparência cosmética independente dos atributos.

Não criar uma spritesheet completa para cada combinação de equipamentos.

Não criar um esqueleto 2D obrigatório.

Não dividir o personagem em dezenas de ossos rígidos que prejudiquem a qualidade da pixel art.

---

# 10. ARQUITETURA FUTURA DE REGIÕES CORPORAIS

A N4 deverá suportar, no mínimo, estas regiões lógicas:

## Corpo-base

- Cabeça.
- Pescoço.
- Torso.
- Braços.
- Mãos.
- Quadril.
- Pernas.
- Pés.

## Aparência

- Cabelo traseiro.
- Cabelo frontal.
- Camisa.
- Calça.
- Botas.
- Luvas.
- Braceletes.

## Equipamentos

- Elmo.
- Capuz.
- Casaco.
- Peitoral.
- Ombreiras.
- Armadura de braços.
- Armadura de pernas.
- Capa traseira.
- Capa frontal.
- Arma principal.
- Escudo.
- Arma secundária.
- Acessórios visíveis.

Essas são categorias lógicas, não necessariamente entidades independentes.

Uma peça visual pode conter várias regiões.

Por exemplo, uma armadura de torso pode incluir mangas e ombreiras.

A arquitetura deve permitir essa composição.

---

# 11. UM CONTROLADOR DE ANIMAÇÃO

A futura arquitetura modular deve possuir um único estado de animação compartilhado.

Exemplo:

`run / frame 3 / facing_right / 15fps`

Todas as camadas relevantes devem acompanhar esse estado.

Corpo: quadro 3.

Cabelo: quadro 3.

Camisa: quadro 3.

Calça: quadro 3.

Botas: quadro 3.

Armadura: quadro 3.

A arma utiliza o soquete correspondente ao quadro 3.

Não permitir temporizadores independentes que causem dessincronização.

Peças que possuam animação secundária, como uma capa, podem ter movimentos próprios, mas devem continuar vinculadas ao estado principal e possuir regras explícitas de sincronização.

---

# 12. CORPO ANATÔMICO NEUTRO

A N4 deverá introduzir um corpo anatômico neutro.

Isso significa que, ao remover a camisa, o torso continua existindo.

Ao remover a calça, as pernas continuam existindo.

Ao remover as botas, os pés continuam existindo.

Ao remover o cabelo, a cabeça continua existindo.

Entretanto, o sprite atual possui regiões anatômicas ocultas pelas roupas.

Essas regiões precisam ser reconstruídas artisticamente.

**Não é suficiente apagar pixels da roupa.**

A N3.8 deve documentar:

- Quais regiões estão ocultas.
- Quais informações existem nas referências.
- Quais regiões exigirão arte nova.
- Quais podem ser recuperadas com segurança.
- Quais possuem maior risco de perda de qualidade.

Não gerar um corpo geométrico e apresentá-lo como arte definitiva.

---

# 13. SISTEMA DE COBERTURA

A futura N4 deverá prever regras como:

- Elmo cobre cabelo.
- Capuz oculta parte da cabeça.
- Armadura cobre camisa.
- Luva cobre punho.
- Bota cobre parte da calça.
- Casaco cobre torso.
- Capa pode passar atrás e à frente do corpo.

Não depender apenas de esconder camadas inteiras.

Investigar a necessidade de:

- Máscaras.
- Regiões visíveis.
- Variantes de composição.
- Ordem de desenho por animação.
- Ordem de desenho por quadro.

Não implementar uma infraestrutura complexa nesta etapa.

Documentar um contrato mínimo e alternativas.

---

# 14. SOQUETES E PONTOS DE ENCAIXE

A N1 já possui mecanismos de ancoragem.

Auditar os pontos existentes.

A N4 provavelmente precisará de referências para:

- Cabeça.
- Pescoço.
- Ombros.
- Torso.
- Costas.
- Cintura.
- Mãos.
- Pés.
- Arma.
- Escudo.

Não inventar coordenadas definitivas.

Medir a posição dos elementos nos quadros aprovados.

Quando não houver informação suficiente, marcar como estimativa.

A posição da mão deve acompanhar a animação.

Uma arma não pode permanecer parada enquanto o braço se move.

---

# 15. EQUIPAMENTOS E APARÊNCIA COSMÉTICA

Preparar uma proposta de sistema semelhante ao conceito de vanity do Terraria.

Separar:

**Equipamento funcional:** determina atributos, defesa, dano e efeitos.

**Aparência exibida:** determina o visual apresentado.

Exemplo:

O jogador equipa uma armadura de ferro para receber defesa.

Mas escolhe exibir visualmente uma camisa comum.

O jogo mantém os atributos da armadura e desenha a camisa.

Esse sistema deve ser planejado para a N4/N5.

Não modificar o save v3 nesta etapa.

Não criar novos slots persistidos sem estudo de compatibilidade.

Não duplicar o inventário.

---

# 16. FAMÍLIAS DE EQUIPAMENTOS

Não criar uma animação específica para cada item.

Investigar famílias de equipamentos:

- Espadas.
- Adagas.
- Machados.
- Lanças.
- Armas pesadas.
- Escudos.
- Elmos.
- Armaduras.
- Calças.
- Botas.
- Capas.

Itens da mesma família podem compartilhar estruturas de animação.

Porém, uma peça que muda radicalmente a silhueta pode precisar de arte específica.

Documentar esse limite.

O objetivo é evitar o crescimento combinatório das spritesheets, não eliminar todo trabalho artístico.

---

# 17. PRESERVAÇÃO DA QUALIDADE ARTÍSTICA

Este é um requisito absoluto.

A N4 não poderá destruir:

- Detalhes do rosto.
- Mechas do cabelo.
- Dobras da camisa.
- Sombras da calça.
- Volume das botas.
- Proporções.
- Silhueta.
- Movimento corporal.

Não retornar ao manequim geométrico da N3.3.

Não utilizar blocos coloridos como resultado artístico final.

Não substituir a corrida aprovada por uma animação esquelética genérica.

Não afirmar que uma composição modular é equivalente à arte original sem comparar os resultados.

---

# 18. ESTUDO DE VIABILIDADE PARA N4

Produzir um relatório técnico e artístico respondendo:

1. É possível separar o cabelo sem destruir as mechas?
2. Como reconstruir a cabeça atrás do cabelo?
3. Como reconstruir o torso atrás da camisa?
4. Como reconstruir as pernas atrás da calça?
5. Como reconstruir os pés atrás das botas?
6. Como separar braços e mangas?
7. Como manter o alinhamento dos seis quadros?
8. Como garantir o mesmo pivô?
9. Como representar a capa atrás do cabelo?
10. Como manter a espada presa à mão?
11. Como lidar com roupas volumosas?
12. Como implementar ocultação de regiões?
13. Como evitar spritesheets por combinação?
14. Como preservar o desempenho?
15. Como permitir aparência cosmética?
16. Quais partes exigem novas referências artísticas?

Não implementar a N4 inteira.

Entregar um plano de migração verificável.

---

# 19. PROPOSTA DE PRIMEIRA PROVA N4

Recomendar uma prova vertical pequena.

A proposta deve incluir:

## Fase A — Idle modular

Separar experimentalmente:

- Corpo anatômico.
- Cabelo.
- Camisa.
- Calça.
- Botas.

Comparar a recomposição com a N3.5.

## Fase B — Troca de peças

Demonstrar:

- Aparência original.
- Sem cabelo.
- Sem camisa.
- Sem calça.
- Sem botas.
- Corpo-base.

## Fase C — Corrida modular

Escolher quadros representativos da corrida N3.6.

Validar:

- Contato com o chão.
- Passada aberta.
- Recuperação.
- Cabelo.
- Roupas.
- Pivô.
- Sobreposição.

Só expandir para todos os seis quadros após a prova artística.

## Fase D — Equipamentos

Adicionar:

- Uma armadura alternativa.
- Uma arma alternativa.
- Uma peça cosmética.

Comprovar que essas peças são independentes.

**Essas fases são o plano da N4, não entregas obrigatórias da N3.8.**

---

# 20. EVIDÊNCIAS DA N3.8

Criar:

`docs/development/n3_8/evidence/`

Produzir evidências reais de:

- Corrida a 15 FPS no jogo.
- Corrida para a direita.
- Corrida para a esquerda.
- Idle → corrida.
- Corrida → idle.
- Corrida → salto.
- Corrida → ataque.
- Alinhamento dos pés.
- Câmera principal 405.
- Comparação com a N3.7.
- Equipamentos existentes durante a corrida.

Produzir GIFs quando necessário para avaliar movimento.

Não utilizar apenas capturas estáticas para afirmar que uma animação funciona.

Não inventar evidências.

---

# 21. DOCUMENTAÇÃO

Criar:

`docs/development/n3_8/`

Documentos mínimos:

1. `N3_8_AUDITORIA.md`
2. `N3_8_CORRIDA_PRODUCAO.md`
3. `N3_8_TEMPORIZACAO.md`
4. `N3_8_ALINHAMENTO.md`
5. `N3_8_TRANSICOES.md`
6. `N3_8_COMPATIBILIDADE.md`
7. `N3_8_ARQUITETURA_TERRARIA.md`
8. `N3_8_PLANO_MODULAR_N4.md`
9. `N3_8_RISCOS_ARTISTICOS.md`
10. `N3_8_TESTES.md`
11. `N3_8_AVALIACAO.md`
12. `N3_8_RELATORIO_FINAL.md`

Não criar documentos vazios.

Registrar o que foi efetivamente implementado.

Distinguir claramente arquitetura proposta de arquitetura existente.

---

# 22. TESTES

Executar:

`cargo fmt --check`

`cargo check`

`cargo clippy --all-targets --jobs 2 -- -D warnings`

`cargo test --jobs 2`

Validar também:

`cargo run`

E o exemplo isolado:

`cargo run --example n3_visual_proof`

Adicionar testes para:

- FPS da corrida.
- Duração do ciclo.
- Quantidade de quadros.
- Integridade da arte.
- Manifestos.
- Pivôs.
- Escala.
- Espelhamento.
- Transições.
- Compatibilidade com equipamentos.
- Preservação dos estados existentes.

Não enfraquecer testes anteriores.

Não afirmar que a execução visual foi validada se apenas os testes headless passaram.

---

# 23. ARQUIVOS PROTEGIDOS

Não alterar:

- `Md-usario/`
- Fontes originais N3.5.
- Quadros originais N3.6.
- Evidências N3.7.
- `white_mesh.glb`
- Blender V2.2 original.
- `assets/data/camera.ron`
- Save v3.
- Física do jogador.
- Combate.
- Magia.
- Controles.

São permitidas alterações pontuais no sistema de apresentação e nos recursos de animação de produção estritamente necessárias para integrar a corrida aprovada.

Registrar cada uma delas.

Não modificar os arquivos originais de arte.

Não implementar a N4 automaticamente.

---

# 24. GATES DA N3.8

## Gate A — Integração

- [ ] Corrida funciona em `cargo run`.
- [ ] Seis quadros aprovados.
- [ ] Reprodução a 15 FPS.
- [ ] Velocidade de 310 u/s.
- [ ] Física inalterada.

## Gate B — Apresentação

- [ ] Pivô correto.
- [ ] Escala correta.
- [ ] Espelhamento correto.
- [ ] Alinhamento do chão.
- [ ] Câmera principal preservada.
- [ ] Identidade artística preservada.

## Gate C — Compatibilidade

- [ ] Idle funciona.
- [ ] Salto funciona.
- [ ] Ataque funciona.
- [ ] Guarda funciona.
- [ ] Esquiva funciona.
- [ ] Dano funciona.
- [ ] Morte funciona.
- [ ] Equipamentos existentes não apresentam regressões.

## Gate D — Preparação N4

- [ ] Arquitetura existente auditada.
- [ ] Conceito estilo Terraria documentado.
- [ ] Regiões removíveis definidas.
- [ ] Corpo anatômico neutro especificado.
- [ ] Cobertura e sobreposição estudadas.
- [ ] Soquetes analisados.
- [ ] Compatibilidade com save estudada.
- [ ] Plano de migração definido.

## Gate E — Segurança

- [ ] Arte original preservada.
- [ ] Arquivos pessoais preservados.
- [ ] Física preservada.
- [ ] Combate preservado.
- [ ] Save preservado.
- [ ] Testes executados.
- [ ] Evidências reais.
- [ ] GitHub publicado.

A aprovação artística final continua sendo do proprietário.

---

# 25. PUBLICAÇÃO OBRIGATÓRIA

Publicar na branch:

`origin/art/v1.2-protagonist`

Procedimento:

1. Confirmar branch.
2. Confirmar HEAD.
3. Inspecionar working tree.
4. Preservar alterações preexistentes.
5. Implementar a integração.
6. Executar testes.
7. Conferir diffs.
8. Verificar arquivos protegidos.
9. Gerar evidências.
10. Criar commits organizados.
11. Fazer push.
12. Confirmar SHA remoto completo.
13. Confirmar estado final do Git.

Sugestões de commits:

`feat(player): integrate approved 15 fps run animation`

`test(player): validate production run animation`

`docs(n3): prepare terraria-style modular architecture for n4`

`docs(n3): record n3.8 integration evidence`

Não criar commits vazios.

Não utilizar `git add -A` indiscriminadamente.

Não incluir arquivos pessoais não autorizados.

Não executar force push.

Não fazer merge ou rebase automático.

---

# 26. RELATÓRIO FINAL

Responder obrigatoriamente:

**A. Estado inicial:** branch, HEAD e working tree.

**B. Decisão artística:** corrida aprovada a 15 FPS.

**C. Integração:** funcionamento no jogo principal.

**D. Temporização:** duração e sincronização.

**E. Arte:** preservação dos seis quadros.

**F. Escala:** apresentação do personagem.

**G. Pivô:** alinhamento.

**H. Espelhamento:** direita e esquerda.

**I. Transições:** estados existentes.

**J. Equipamentos:** compatibilidade atual.

**K. Câmera:** apresentação em 405.

**L. Modularidade:** arquitetura atual.

**M. Terraria:** princípios adotados.

**N. Corpo-base:** requisitos de reconstrução.

**O. Cabelo:** estratégia de separação.

**P. Roupas:** estratégia de separação.

**Q. Armaduras:** composição visual.

**R. Armas:** soquetes e famílias.

**S. Cobertura:** ocultação de regiões.

**T. Animação:** relógio compartilhado.

**U. Save:** compatibilidade.

**V. N4:** plano de execução.

**W. Evidências:** arquivos produzidos.

**X. Testes:** resultados reais.

**Y. GitHub:** commits e SHA remoto.

**Z. Próxima decisão:** critérios para iniciar N4.

---

# 27. REGRA FINAL

O protagonista alcançou uma qualidade artística aprovada.

Essa qualidade é a referência obrigatória para todo desenvolvimento futuro.

A corrida a 15 FPS deve entrar no jogo principal sem modificar os seis quadros.

O sistema modular futuro deve seguir a filosofia de composição visual do Terraria:

**Um personagem, várias camadas, equipamentos independentes e animações sincronizadas.**

Todas as roupas e partes visuais devem ser removíveis.

Não gerar uma spritesheet completa para cada combinação.

Não criar um segundo inventário.

Não abandonar o sistema de animação existente.

Não sacrificar o acabamento da pixel art.

Não iniciar N4 ou N5 automaticamente.

**Entregue a N3.8 integrada, testada, documentada e publicada. Depois apresente a arquitetura proposta da N4 para aprovação antes de começar a separação artística do protagonista.**