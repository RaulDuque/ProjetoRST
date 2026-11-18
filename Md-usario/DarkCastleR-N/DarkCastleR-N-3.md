# DARK CASTLE RPG — MEGA PROMPT N3
# PROVA VISUAL DO PROTAGONISTA, PIXEL ART GÓTICA, ANIMAÇÕES E EQUIPAMENTOS MODULARES

**Projeto:** Dark Castle RPG  
**Etapa:** N3 — Prova de conceito visual  
**Linguagem:** Rust 2024  
**Engine:** Bevy 0.20.1  
**Branch de trabalho esperada:** `art/v1.2-protagonist`  
**Base publicada da N2:** `707a593`  
**Status:** AUTORIZADA PARA EXECUÇÃO  
**Próxima etapa:** N4 — Não autorizada.

---

# 0. MISSÃO

Você será responsável pela direção técnica de arte, integração Blender/Bevy, animação de sprites, pixel art, equipamentos visuais e validação de desempenho.

Sua missão é criar uma **prova visual executável, isolada, reproduzível e mensurável** para decidir como produziremos o protagonista definitivo do Dark Castle RPG.

O jogo continuará sendo 2D. Blender é ferramenta de produção, não uma mudança para gameplay 3D.

O objetivo é validar:

1. Qualidade da pixel art.
2. Silhueta e legibilidade.
3. Fluidez das animações.
4. Sincronização entre corpo e equipamentos.
5. Troca visual de armas.
6. Troca visual de armaduras.
7. Movimento da capa.
8. Escalabilidade para centenas de itens.
9. Resolução e enquadramento.
10. Tempo e custo de produção dos assets.

**Não considere a N3 aprovada apenas porque o protótipo compila.**

O resultado visual precisa ser apresentado ao proprietário para avaliação.

---

# 1. LEITURA OBRIGATÓRIA

Antes de qualquer implementação, leia:

- `CLAUDE.md`
- `docs/ARCHITECTURE.md`
- `docs/DECISIONS.md`
- `docs/development/n1/N1_PROVA_DE_CONCEITO.md`
- `docs/development/n1/N1_PIPELINE_3D_PARA_2D.md`
- `docs/development/n1/N1_PERSONAGEM_MODULAR.md`
- `docs/development/n1/N1_EQUIPAMENTOS_VISUAIS.md`
- `docs/development/n1/N1_PIXEL_ART_32_64_BITS.md`
- `docs/development/n1/N1_DIRECAO_ARTISTICA.md`
- `docs/development/n1/N1_ANIMACAO_ESQUELETICA_2D.md`
- `docs/development/n1/N1_COMPARACAO_FERRAMENTAS.md`
- `docs/development/n1/N1_REFERENCIAS_E_LICENCAS.md`
- `docs/development/n1/N1_ROADMAP.md`
- `docs/development/n2/N2_RELATORIO_FINAL.md`

Inspecione os scripts reais em `art-src/`, o manifesto de animações e os sistemas de composição visual.

Não presuma que o pipeline atual já produz pixel art definitiva.

---

# 2. AUDITORIA INICIAL E SEGURANÇA

Antes de trabalhar:

1. Confirmar branch e HEAD.
2. Conferir alterações locais.
3. Conferir arquivos não rastreados.
4. Identificar arquivos pessoais.
5. Conferir o pipeline V2.2.
6. Conferir os exemplos visuais existentes.
7. Identificar quais assets são gerados e quais são fontes.
8. Conferir os contratos dos atlas e sockets.
9. Confirmar a integridade dos arquivos protegidos.

Preservar:

- `Md-usario/`
- `Md-usario/DarkCastleR-D/`
- `Md-usario/assets-user/prota/base/white_mesh.glb`
- `saves/`
- Assets publicados do jogo.
- Documentação N1 e N2.
- Código funcional de gameplay.
- Save v3.

Não executar limpeza destrutiva.

Não substituir assets existentes sem autorização específica.

Não modificar o executável principal como parte da prova.

---

# 3. IDENTIDADE VISUAL

A direção é fantasia gótica sombria com pixel art detalhada.

Referências de linguagem visual:

- Arquitetura medieval e gótica.
- Silhuetas fortes.
- Armaduras ornamentadas.
- Tecidos e capas com movimento.
- Metais com reflexos controlados.
- Contraste entre luz quente e sombras frias.
- Magia com cores identificáveis.
- Animações expressivas.

O objetivo é evocar a qualidade de produção de jogos 2D clássicos, sem copiar personagens, sprites ou assets de terceiros.

"32/64 bits" descreve o estilo desejado, não limita a resolução ou o tamanho dos sprites.

O protagonista deve ser reconhecível mesmo em tamanho reduzido.

Não usar imagens de jogos comerciais como sprites do projeto.

---

# 4. PIPELINE PRINCIPAL A TESTAR

A hipótese de trabalho é:

**Blender → render ortográfico → acabamento de pixel art → atlas → Bevy.**

Investigue e reaproveite o pipeline V2.2.

O fluxo precisa permitir:

- Uma fonte editável de personagem.
- Animações reaproveitáveis.
- Renderização reproduzível.
- Separação de equipamento.
- Exportação de PNG.
- Montagem de atlas.
- Geração ou manutenção de sockets.
- Manifesto validado.
- Carregamento no Bevy.

Não produzir uma segunda arquitetura paralela se os sistemas atuais forem suficientes.

Não instalar dependências desnecessárias.

---

# 5. BLENDER E PIXELORAMA

Blender é a ferramenta principal de animação e renderização.

Pixelorama será utilizado para acabamento da pixel art, caso esteja disponível ou possa ser instalado com segurança.

**Está autorizada a instalação do Pixelorama exclusivamente para a N3**, desde que:

- A fonte seja oficial e verificável.
- A licença seja conferida.
- Não exija acesso administrativo invasivo.
- Não altere instalações do sistema sem necessidade.
- Não modifique arquivos pessoais.
- Não execute scripts externos desconhecidos.
- Não exija compra ou assinatura.

Se não for possível instalar com segurança, documentar o bloqueio e continuar com os recursos disponíveis, sem declarar a etapa de acabamento concluída.

LibreSprite pode ser avaliado como alternativa, mas não deve ser instalado automaticamente.

Não comprar Aseprite ou Spine.

Não executar Hunyuan3D ou Mixamo.

---

# 6. MALHA ORIGINAL DO PROTAGONISTA

Existe:

`Md-usario/assets-user/prota/base/white_mesh.glb`

A N1 identificou:

- Aproximadamente 94 mil vértices.
- Ausência de esqueleto.
- Ausência de skinning.
- Ausência de animações.
- Ausência de UV e material adequados.
- Geometria que não está pronta para produção animada.

**Nunca modificar o arquivo original.**

Está autorizada uma cópia para testes em diretório de trabalho separado.

Antes de tentar rigging:

1. Inspecionar a cópia.
2. Verificar topologia.
3. Verificar proporções.
4. Verificar orientação.
5. Avaliar necessidade de retopologia.
6. Avaliar se a malha pode ser usada como referência.
7. Comparar com o manequim já funcional da V2.2.

Não gastar a N3 inteira tentando corrigir uma malha inadequada.

Se o manequim V2.2 for mais apropriado, utilizá-lo como base da prova.

Registrar a justificativa.

---

# 7. PROTAGONISTA DA PROVA

Produzir um personagem com:

- Corpo-base.
- Duas variantes de armadura.
- Duas armas visualmente distintas.
- Uma capa.
- Pivôs consistentes.
- Silhueta legível.
- Direção de desenho para a direita.
- Espelhamento para a esquerda.

As armaduras devem apresentar diferenças visuais perceptíveis.

Não basta trocar a cor do corpo.

Exemplo de contraste visual aceitável:

**Armadura A:** leve, com torso estreito, braços mais expostos e menor volume.

**Armadura B:** pesada, com ombreiras, placas metálicas e silhueta mais larga.

As duas devem preservar o contrato de animação ou documentar precisamente as exceções necessárias.

As armas devem pertencer a famílias diferentes.

Sugestão:

- Espada de uma mão.
- Lança ou machado.

Não incorporar permanentemente a arma ao atlas do corpo.

---

# 8. QUATRO ANIMAÇÕES OBRIGATÓRIAS

Produzir e demonstrar:

## Idle

O personagem permanece parado, com movimento sutil de respiração e tecido.

Verificar estabilidade do pivô.

## Walk

O personagem caminha com transferência de peso, movimento de pernas e braços.

Não aceitar simplesmente deslocar um sprite estático.

Se aproveitar frames de run, identificar essa limitação.

## Jump

O personagem executa uma animação de salto.

Diferenciar visualmente impulso, subida e queda quando o material disponível permitir.

Não implementar pulo duplo na N3.

## Attack

O personagem executa um golpe.

Verificar:

- Preparação.
- Execução.
- Recuperação.
- Arma acompanhando a mão.
- Ordem de desenho.
- Sincronização com a capa.
- Ausência de arma duplicada.

A prova visual não pode modificar a fórmula de dano nem o tempo real das hitboxes do jogo principal.

---

# 9. EQUIPAMENTOS MODULARES

O objetivo é demonstrar:

**Corpo + armadura + arma + capa.**

A troca deve acontecer durante a execução do protótipo.

Testar todas as combinações:

| Armadura | Arma | Capa |
|---|---|---|
| A | Espada | Ligada |
| A | Arma B | Ligada |
| B | Espada | Ligada |
| B | Arma B | Ligada |

Adicionar comparação com a capa desligada.

A troca não pode:

- Reiniciar incorretamente a animação.
- Mudar o pivô sem controle.
- Deslocar a arma da mão.
- Duplicar o desenho da arma.
- Alterar o tamanho do personagem arbitrariamente.
- Criar uma folha para cada combinação.

Medir a quantidade de assets necessários.

Documentar quais peças precisam de variantes próprias por animação.

---

# 10. SINCRONIZAÇÃO DE QUADROS

Reutilizar o modelo existente de `VisualPlayback`, quando adequado.

O corpo, a arma, a capa e as variantes devem observar uma referência temporal coerente.

Inspecionar:

- `AnimationManifest`.
- `player_sockets.ron`.
- `held_visual`.
- `CapeLayer`.
- `weapon_in_body`.
- `held_layers_enabled`.

Verificar se esses nomes e mecanismos continuam presentes na implementação atual.

Testar espelhamento horizontal.

Testar troca de equipamento durante:

- Idle.
- Walk.
- Jump.
- Attack.

A arma deve acompanhar a mão durante todos os quadros.

A capa deve manter posicionamento plausível.

Se a ordem de desenho precisar mudar durante o ataque, implementar uma solução mínima no protótipo ou demonstrar a limitação com evidência.

Não alterar a arquitetura de animação do jogo principal para resolver um problema exclusivo da prova.

---

# 11. COMPARAÇÃO DE PIPELINES

Comparar duas abordagens para o mesmo gesto:

**A — Render 3D com acabamento de pixel art.**

**B — Ajuste ou desenho manual quadro a quadro.**

Utilizar a mesma escala de apresentação.

Comparar:

- Silhueta.
- Qualidade do contorno.
- Consistência dos pixels.
- Movimento.
- Legibilidade.
- Facilidade de modificar equipamento.
- Tempo de produção.
- Repetibilidade.
- Quantidade de correções manuais.

Se não houver ferramenta ou capacidade para produzir uma comparação manual genuína, registrar isso como limitação.

Não apresentar um filtro automático como se fosse pixel art corrigida manualmente.

O objetivo é decidir qual pipeline merece investimento futuro.

---

# 12. RESOLUÇÃO EXPERIMENTAL

A N1 aprovou testar:

**640×360 internos, apresentados em 1280×720 com escala inteira 2×.**

A câmera do jogo principal continua inalterada.

Implementar o experimento isoladamente.

Verificar:

- Nearest-neighbor.
- Pixels quadrados.
- Ausência de suavização indesejada.
- Escala inteira.
- Posição da câmera.
- Legibilidade do personagem.
- Espaço visível ao redor.
- Movimento sem tremulação excessiva.

Comparar pelo menos:

1. Visão atual equivalente ao enquadramento do jogo.
2. Visão aproximada, com personagem entre aproximadamente 80 e 110 pixels de altura.

Se possível, incluir uma terceira comparação em 960×540 com escala inteira em 1920×1080.

Não afirmar que 960×540 possui escala inteira em uma janela 1280×720.

Não modificar `camera.ron`.

Não alterar o zoom do `cargo run`.

---

# 13. PROTÓTIPO EXECUTÁVEL ISOLADO

Criar um exemplo específico, preferencialmente:

`examples/n3_visual_proof.rs`

Ou nome equivalente coerente com os exemplos existentes.

O protótipo deve permitir:

- Alternar as quatro animações.
- Trocar arma.
- Trocar armadura.
- Mostrar e ocultar capa.
- Espelhar personagem.
- Pausar animação.
- Avançar um quadro.
- Exibir nome do clipe.
- Exibir índice do quadro.
- Exibir configuração visual ativa.

Os controles desse exemplo podem ser próprios, desde que estejam documentados e não interfiram no sistema N2 do jogo principal.

Não substituir `cargo run`.

Não modificar o menu principal do jogo para acessar a prova.

---

# 14. EVIDÊNCIAS VISUAIS OBRIGATÓRIAS

Criar:

`docs/development/n3/evidence/`

Produzir, sempre que tecnicamente possível:

- Captura de idle.
- Captura de walk.
- Captura de jump.
- Captura de attack.
- Armadura A.
- Armadura B.
- Arma A.
- Arma B.
- Capa ligada.
- Capa desligada.
- Comparação de resolução.
- Comparação entre render bruto e acabamento.

Também produzir um painel comparativo com as quatro combinações principais de armadura e arma.

As imagens devem ser geradas a partir dos resultados reais.

Não utilizar mockups para fingir que o protótipo foi executado.

Se houver animação capturada em vídeo ou GIF, disponibilizar como evidência adicional.

Não declarar testes visuais realizados se apenas o código compilou.

---

# 15. MÉTRICAS DE PRODUÇÃO

Registrar:

- Tempo gasto para preparar o modelo.
- Tempo gasto no rig.
- Tempo gasto para criar cada animação.
- Tempo de renderização.
- Tempo de acabamento manual.
- Quantidade de frames.
- Tamanho dos atlas.
- Memória estimada das texturas.
- Quantidade de sprites simultâneos.
- Número de variantes de equipamento.
- Quantidade de arquivos gerados.
- Etapas automatizadas.
- Etapas manuais.

Distinguir tempo efetivamente medido de estimativas.

Não inventar horas de trabalho humano.

A meta experimental é manter as texturas do personagem abaixo de 32 MB de memória residente, conforme a N1, mas medir e relatar o resultado real.

---

# 16. TESTES AUTOMATIZADOS

Criar testes para verificar:

1. Carregamento do manifesto.
2. Existência dos quatro clipes.
3. Validade das dimensões do atlas.
4. Integridade dos frames.
5. Pivôs consistentes.
6. Sockets válidos.
7. Referências de equipamento.
8. Troca de arma.
9. Troca de armadura.
10. Troca de capa.
11. Espelhamento.
12. Sincronização de frames.
13. Ausência de referências inválidas.
14. Ausência de arma duplicada.
15. Compatibilidade com o contrato existente.

Não alterar testes antigos para mascarar regressões.

---

# 17. VALIDAÇÃO GRÁFICA

Executar o exemplo gráfico se o ambiente permitir.

Verificar visualmente:

- Idle em movimento.
- Walk em movimento.
- Jump.
- Attack.
- Troca durante animação.
- Espelhamento.
- Arma na mão.
- Capa atrás do corpo.
- Ordem de desenho.
- Escala de pixels.

Uma captura isolada não comprova fluidez.

Se possível, capturar uma sequência de frames ou vídeo.

Documentar quais verificações foram automatizadas e quais exigem avaliação humana.

---

# 18. DEZ PERGUNTAS DE ACEITAÇÃO

Responder objetivamente:

1. O resultado parece pixel art ou render reduzido?
2. A silhueta é legível em fundo escuro?
3. Walk e jump possuem peso convincente?
4. A arma acompanha corretamente a mão?
5. As duas armaduras mantêm alinhamento?
6. A capa e a arma respeitam a ordem visual?
7. Uma quinta arma exigiria o quê?
8. Quanto tempo levou a produção de cada clipe?
9. O que foi automatizado e o que foi corrigido manualmente?
10. O mesmo PNG foi validado no Linux? O Windows foi realmente testado?

Cada resposta deve ser:

- Sim.
- Parcial.
- Não.
- Não verificado.

Apresentar evidência e justificativa.

Não atribuir aprovação estética em nome do proprietário.

---

# 19. CRITÉRIOS DE QUALIDADE ARTÍSTICA

Não aceitar como resultado definitivo:

- Retângulos coloridos.
- Silhuetas geométricas provisórias.
- Modelo 3D simplesmente reduzido sem acabamento.
- Personagem sem expressão corporal.
- Armas flutuantes.
- Armaduras indistinguíveis.
- Capa imóvel quando o corpo se movimenta.
- Pixels com suavização inconsistente.
- Equipamento duplicado.
- Animações sem transferência de peso.

Esses resultados podem existir como etapas intermediárias, mas não como aprovação visual final.

Caso a qualidade ainda seja insuficiente, entregar o protótipo e explicar exatamente o que falta.

Não substituir a honestidade da avaliação por um relatório otimista.

---

# 20. DOCUMENTAÇÃO N3

Criar:

`docs/development/n3/`

Arquivos mínimos:

1. `N3_AUDITORIA_PIPELINE.md`
2. `N3_DIRECAO_VISUAL.md`
3. `N3_MODELO_E_RIG.md`
4. `N3_ANIMACOES.md`
5. `N3_EQUIPAMENTOS.md`
6. `N3_PIXEL_ART.md`
7. `N3_RESOLUCAO_E_CAMERA.md`
8. `N3_COMPARACAO_PIPELINES.md`
9. `N3_METRICAS.md`
10. `N3_TESTES.md`
11. `N3_AVALIACAO_VISUAL.md`
12. `N3_RELATORIO_FINAL.md`

Incluir comandos de reprodução.

Registrar dependências necessárias.

Registrar origem e licença de qualquer recurso externo.

Não criar documentação com resultados fictícios.

---

# 21. GATES TÉCNICOS

Executar:

`cargo fmt --check`

`cargo check`

`cargo clippy --all-targets -- -D warnings`

`cargo test`

Compilar e executar o exemplo N3.

Verificar os recursos gerados.

Verificar que o jogo principal continua funcionando, quando o ambiente permitir.

Não declarar sucesso de testes que não foram executados.

---

# 22. PUBLICAÇÃO NO GITHUB — OBRIGATÓRIA

**Nova regra permanente do proprietário: toda etapa concluída deve ser publicada no GitHub para revisão.**

Ao terminar a N3:

1. Confirmar branch de trabalho.
2. Revisar o diff completo.
3. Identificar arquivos anteriores à N3.
4. Garantir que nenhum arquivo pessoal foi incluído.
5. Executar testes.
6. Conferir licenças.
7. Criar commits organizados.
8. Incluir código, documentação e evidências necessárias.
9. Não incluir temporários, caches ou binários desnecessários.
10. Fazer push para `origin/art/v1.2-protagonist`.
11. Confirmar o SHA remoto.
12. Informar os hashes dos commits.

Se houver alterações preexistentes, não incorporá-las automaticamente.

Se a branch remota tiver avançado, interromper a publicação e explicar o conflito, sem force push.

Não executar merge ou rebase automaticamente.

Não usar `git push --force`.

Se a publicação falhar, relatar a falha em vez de afirmar que foi concluída.

O relatório deve distinguir o estado antes da publicação do estado publicado.

Não é necessário fazer commit de alterações que pertençam ao usuário e estejam fora do escopo.

---

# 23. RELATÓRIO FINAL

Entregar:

**A. Estado inicial** — Branch, HEAD, alterações anteriores.

**B. Auditoria** — Pipeline encontrado.

**C. Modelo** — Qual modelo foi utilizado e por quê.

**D. Rig** — Estrutura e limitações.

**E. Animações** — Quatro clipes.

**F. Armas** — Duas famílias.

**G. Armaduras** — Duas variantes.

**H. Capa** — Sincronização e comportamento.

**I. Composição** — Corpo, equipamento, pivôs e sockets.

**J. Pixel art** — Qualidade e acabamento.

**K. Resolução** — Comparação de enquadramentos.

**L. Protótipo** — Como executar.

**M. Evidências** — Capturas, sequências e comparações.

**N. Métricas** — Tempo, arquivos e memória.

**O. Testes** — Resultados reais.

**P. Dez perguntas** — Respostas e justificativas.

**Q. Limitações** — O que não foi comprovado.

**R. Arquivos** — Criados e modificados.

**S. Segurança** — Integridade dos arquivos protegidos.

**T. GitHub** — Commits, push e SHA remoto.

**U. Recomendação** — Seguir híbrido, usar desenho manual ou investigar outra solução.

**V. Aprovação pendente** — Decisões que exigem avaliação do proprietário.

---

# 24. LIMITES ABSOLUTOS

Não implementar:

- N4.
- N5.
- Novas regiões.
- Pulo duplo.
- Novos chefes.
- Novas relíquias.
- Novo inventário.
- Sistema de combate paralelo.
- Rig 2D completo em runtime.
- Produção em massa de personagens.
- Migração de engine.
- Alteração global de câmera.
- Alteração do Save v3.

Não modificar `white_mesh.glb` original.

Não apagar arquivos pessoais.

Não executar Hunyuan3D.

Não comprar ferramentas.

Não substituir o protagonista publicado no jogo principal.

---

# 25. CONDIÇÃO DE ENCERRAMENTO

A N3 termina quando existir uma prova visual executável, documentada, testada e publicada no GitHub.

Mas existem duas aprovações distintas:

**Aprovação técnica:** o protótipo funciona, os recursos são válidos e os testes passam.

**Aprovação artística:** o proprietário examina as evidências e decide se o estilo visual atende ao Dark Castle RPG.

Você pode concluir tecnicamente a N3 sem declarar aprovação artística.

Se o visual não atingir o objetivo, registrar as limitações e propor uma correção, sem iniciar N4.

**Execute a N3. Publique no GitHub. Entregue evidências reais. Pare e aguarde a avaliação do proprietário.**