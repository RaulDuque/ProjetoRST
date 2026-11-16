# DARK CASTLE RPG — PESQUISA SOTN FASE 2
## Chefes, exploração, equipamentos, animações e arquitetura de combate

Você atuará como pesquisador técnico e arquiteto de jogos, trabalhando no projeto Dark Castle RPG, desenvolvido em Rust e Bevy.

Esta tarefa é uma continuação de uma pesquisa anterior sobre Castlevania: Symphony of the Night.

**Não implemente funcionalidades nesta etapa. Sua missão é concluir a investigação, validar as descobertas e preparar o plano técnico da V2.4 e das versões posteriores.**

---

## 1. Estado do projeto

Projeto: Dark Castle RPG.

Stack conhecida:
- Rust, edition 2024.
- Bevy 0.20.1.
- RON para dados.
- Blender para o pipeline artístico.
- Windows e Linux como plataformas-alvo.

A V2.3 foi publicada na branch `art/v1.2-protagonist`.

Último commit confirmado no remoto:

`9f7616d2d444f5751b06ece5ad7823f3dd420777`

O projeto já possui sistemas de combate, magia, habilidades, inventário, equipamentos, inimigos, NPCs, comércio, exploração, câmera, parallax e Save V3.

Não presuma que todas as funcionalidades estejam completas. Verifique a implementação real.

---

## 2. Pesquisa anterior

Uma sessão anterior analisou:

- https://github.com/xeeynamo/sotn-decomp
- https://github.com/BlackLabelHQ/SymphonyRecomp

Commits utilizados como referência:

- sotn-decomp: `3a0957b`
- SymphonyRecomp: `2deead5`

Os clones podem estar disponíveis em:

`~/.cache/sotn-research/`

Verifique se existem antes de tentar baixá-los novamente.

A pesquisa anterior produziu:

- `docs/REFERENCIAS_SOTN.md`
- `docs/ESTUDO_SOTN_COMBATE.md`
- `docs/ESTUDO_SOTN_ATRIBUTOS.md`
- `docs/PLANO_MELHORIAS_SOTN.md`

**Esses quatro documentos são o ponto de partida obrigatório.**

Leia-os integralmente antes de iniciar a nova investigação.

Não os substitua por resumos genéricos.

Verifique as referências de código, os caminhos e as linhas citadas.

Caso os documentos não estejam disponíveis, interrompa e informe quais estão faltando.

---

## 3. Auditoria inicial

Antes de qualquer trabalho:

1. Execute `git status --short`.
2. Confirme branch e HEAD.
3. Identifique arquivos não rastreados.
4. Confirme que os quatro documentos da pesquisa estão presentes.
5. Verifique os clones externos.
6. Leia as regras do projeto.
7. Identifique o estado atual do roadmap.

Não altere arquivos protegidos.

Não crie commits.

Não execute push.

Não inicie a V2.4.

---

## 4. Prioridade A — Chefes

A pesquisa anterior não investigou os chefes do SOTN.

Estude exemplos representativos de chefes, escolhidos conforme a disponibilidade de código verificável.

Investigue:

- Estados da IA.
- Transições entre estados.
- Escolha de ataques.
- Distância do jogador.
- Tempo de preparação.
- Recuperação após ataques.
- Fases de combate.
- Mudanças de comportamento conforme HP.
- Padrões de movimento.
- Projéteis.
- Invulnerabilidade.
- Vulnerabilidades.
- Reações ao dano.
- Colisões.
- Recompensas.
- Condições de vitória.
- Persistência após derrotar o chefe.

Para cada chefe estudado, apresente uma descrição técnica do comportamento, com referências aos arquivos e funções.

Não copie nomes, personagens, animações ou padrões específicos para o Dark Castle RPG.

Identifique princípios reutilizáveis para criar chefes originais.

### Entrega

Crie:

`docs/ESTUDO_SOTN_CHEFES.md`

Inclua uma proposta de arquitetura de chefes para Rust/Bevy que reutilize o sistema atual de inimigos sempre que possível.

---

## 5. Prioridade B — Exploração e mundo

Investigue a organização do mundo de SOTN.

Estude:

1. Estrutura de regiões.
2. Identificadores de salas.
3. Geometria.
4. Transições.
5. Portas.
6. Limites da câmera.
7. Salas verticais.
8. Plataformas.
9. Salas secretas.
10. Passagens ocultas.
11. Atalhos.
12. Bloqueios por habilidade.
13. Backtracking.
14. Mapa.
15. Salas descobertas.
16. Eventos persistentes.
17. Reaparecimento de inimigos.
18. Posicionamento de itens.
19. Pontos de salvamento.
20. Progressão não linear.

Compare com a arquitetura real do Dark Castle RPG.

Não copie mapas nem a distribuição exata das salas do SOTN.

Procure oportunidades para tornar a exploração mais interessante, mantendo o mundo original do nosso jogo.

### Entrega

Crie:

`docs/ESTUDO_SOTN_EXPLORACAO.md`

---

## 6. Prioridade C — Armas e equipamentos

A pesquisa anterior não confirmou completamente as regras de armas de duas mãos e escudos.

Investigue:

- Categorias de armas.
- Alcance.
- Velocidade.
- Temporização de ataques.
- Ponto de cancelamento.
- Atordoamento.
- Knockback.
- Intervalo de reacerto.
- Propriedades especiais.
- Equipamentos de duas mãos.
- Restrições de mão esquerda e direita.
- Escudos.
- Bloqueio.
- Interação com ataques.
- Modificadores.
- Efeitos elementais.
- Equipamentos raros.

Quando existirem tabelas numéricas, identifique os campos e seu significado, mas não copie automaticamente valores para o Dark Castle RPG.

Compare com nossos sistemas de equipamento e de armas visuais da V2.2.

Considere que os visuais atuais de armas ainda são provisórios.

### Entrega

Crie:

`docs/ESTUDO_SOTN_EQUIPAMENTOS.md`

---

## 7. Prioridade D — Animações e sensação de combate

Investigue como o SOTN coordena:

- Animação.
- Movimento.
- Colisão.
- Ataque.
- Recuperação.
- Cancelamento.
- Recuo.
- Invulnerabilidade.
- Efeitos visuais.
- Efeitos sonoros.
- Congelamento breve de impacto, se comprovado.
- Transições entre estados.

Compare com a arquitetura atual do Dark Castle RPG.

A pesquisa anterior recomendou preservar os temporizadores de gameplay, em vez de fazer toda a lógica depender dos quadros de animação.

Avalie criticamente essa recomendação.

Se houver alternativas melhores, apresente vantagens, desvantagens e riscos.

Não implemente mudanças.

### Entrega

Crie:

`docs/ESTUDO_SOTN_ANIMACOES.md`

---

## 8. Prioridade E — Interface e inventário

Investigue:

- Navegação por menus.
- Organização de equipamentos.
- Exibição de atributos.
- Comparação de itens.
- Seleção de armas.
- Informações de magias.
- Interface do mapa.
- Indicadores de HP e MP.
- Feedback de dano.
- Estados de menu.
- Pausa e retorno ao jogo.

Compare com o HUD e os menus existentes.

Não copie elementos gráficos da interface original.

### Entrega

Crie:

`docs/ESTUDO_SOTN_INTERFACE.md`

---

## 9. Pendências da pesquisa anterior

Revise explicitamente estas questões:

- Quando inimigos podem reaparecer.
- Como funciona a saída do estado de petrificação.
- Regras de armas de duas mãos.
- Bloqueio com escudo.
- Valores e significados das propriedades de armas.
- Comportamento de chefes.
- Estrutura de mapa e transições.

Classifique cada uma como:

- Confirmada por código.
- Parcialmente confirmada.
- Inferida.
- Não encontrada.

Não invente resultados para fechar pendências.

---

## 10. Integração com o Dark Castle RPG

Depois de investigar os sistemas externos, examine o código real do nosso jogo.

Para cada melhoria proposta, informe:

- Problema atual.
- Evidência encontrada no SOTN.
- Solução original para Rust/Bevy.
- Componentes envolvidos.
- Sistemas envolvidos.
- Alterações de dados necessárias.
- Impacto no Save V3.
- Impacto nas animações.
- Impacto nos controles.
- Testes necessários.
- Riscos de regressão.
- Dificuldade.
- Prioridade.

Evite recomendar grandes reescritas.

Preserve as funcionalidades existentes.

---

## 11. Planejamento da V2.4

Prepare um plano de implementação para:

**V2.4 — Combat Feel / Peso do Golpe**

Considere o pacote P1 identificado anteriormente:

1. Linha do tempo de ataque por categoria de arma.
2. Regras de cancelamento.
3. Ações permitidas durante cada fase.
4. Recuo em diferentes intensidades.
5. Proteção contra múltiplos danos simultâneos.
6. Atordoamento e knockback.
7. Intervalo de reacerto.
8. Controle aéreo durante ataques.
9. Ajuste de hurtboxes por postura.

Entretanto, não considere essas propostas automaticamente aprovadas.

Verifique primeiro se são compatíveis com a arquitetura atual.

Defina uma ordem de implementação incremental.

Cada etapa deverá ter testes e critérios de aceitação.

Preserve o comportamento atual como padrão para os novos campos de configuração, sempre que tecnicamente possível.

Não altere a Iron Sword nem outras armas existentes sem uma decisão explícita.

### Entrega

Crie:

`docs/PLANO_V2_4_COMBAT_FEEL.md`

---

## 12. Roadmap posterior

Organize as melhorias que não pertencem à V2.4 em versões futuras.

Considere:

- Chefes.
- IA avançada.
- Exploração.
- Progressão por habilidades.
- Armas especiais.
- Escudos.
- Combos.
- Bestiário.
- Animações.
- Melhorias visuais.
- Arte final.
- Interface.
- Balanceamento.

Não redefina unilateralmente a numeração das versões existentes.

Apresente uma proposta de roadmap separada para aprovação.

---

## 13. Regras de propriedade intelectual

O Dark Castle RPG deve continuar sendo um jogo original.

Não copie:

- Código-fonte dos repositórios externos para nosso projeto.
- Sprites.
- Texturas.
- Músicas.
- Efeitos sonoros.
- Personagens.
- Nomes protegidos.
- Mapas.
- Diálogos.
- Assets extraídos do jogo original.

Utilize as fontes externas para estudar princípios técnicos e comportamentos.

O repositório sotn-decomp possui licença AGPL; o SymphonyRecomp não apresenta uma licença geral explícita em sua página principal. Não presuma permissões amplas sobre os materiais. <Cite ref="turn242255view0"/><Cite ref="turn242255view1"/>

Não envie issues ou pull requests aos projetos externos.

---

## 14. Controle de contexto e recursos

Evite vários agentes Opus simultâneos.

Priorize a pesquisa sequencial.

Não repita a Fase 1.

Trabalhe por módulos.

Se uma etapa consumir muito contexto, conclua sua documentação antes de avançar.

Registre descobertas e pendências continuamente.

Não execute tarefas longas de compilação sem necessidade.

---

## 15. Proteção do projeto

Não modifique:

- `Md-usario/`
- `saves/`
- `.cargo/config.toml`
- `.claude/settings.json`
- `.claude/hooks/`
- Assets publicados.
- Backups.
- Arquivos de inspiração.

Não execute:

- `git reset --hard`
- `git clean`
- `cargo clean`
- `git push`
- `git commit`
- `git rebase`

Não altere o código do jogo.

Não modifique dependências.

Não crie branches ou worktrees.

Não sobrescreva os quatro documentos da Fase 1.

---

## 16. Entrega final

Apresente um relatório organizado:

A. Estado inicial do Git.

B. Documentos anteriores analisados.

C. Repositórios externos e commits utilizados.

D. Chefes investigados.

E. Arquitetura de exploração.

F. Equipamentos e armas.

G. Animações e sincronização.

H. Interface e inventário.

I. Pendências resolvidas.

J. Pendências não resolvidas.

K. Comparação com Dark Castle RPG.

L. Melhorias prioritárias.

M. Plano V2.4.

N. Roadmap posterior.

O. Documentos criados.

P. Referências verificáveis.

Q. Estado final do Git.

R. Confirmação dos arquivos protegidos.

Não faça commit.

Não faça push.

Não implemente a V2.4.

**O objetivo é entregar uma base técnica suficientemente sólida para que a próxima sessão implemente a V2.4 sem precisar repetir a pesquisa.**

Comece pela auditoria do Git e pela leitura integral dos quatro documentos anteriores.