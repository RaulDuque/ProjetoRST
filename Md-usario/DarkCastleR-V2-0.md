# DARK CASTLE RPG — MEGA PROMPT V2.0
## Sistema Avançado de Armas, Armaduras, Equipamentos, Inventário e Aparência Visual

### MISSÃO OFICIAL — IMPLEMENTAÇÃO AUTORIZADA

Você é o arquiteto técnico principal e responsável pela implementação da V2.0 do Dark Castle RPG.

O projeto é um Action RPG Metroidvania 2D original, desenvolvido em Rust 2024, Bevy 0.20.1, dados RON e pipeline artístico baseado em Blender 5.2.4 LTS.

O objetivo de longo prazo é criar um jogo com a profundidade de exploração e equipamentos de Castlevania: Symphony of the Night, mas com identidade própria, personagens originais, castelo original e sistemas próprios.

O jogador deverá encontrar armas, roupas, armaduras e acessórios ao explorar o castelo, derrotar inimigos e abrir baús.

Os equipamentos deverão alterar atributos, combate e, quando aplicável, a aparência do protagonista.

A V1.9 foi aprovada tecnicamente, com ressalvas.

**Você está autorizado a implementar a V2.0 dentro das restrições deste documento.**

Não inicie a V2.1.

---

# 1. VISÃO GERAL DA V2.0

Esta versão deverá estabelecer a fundação definitiva dos equipamentos do jogo.

Objetivos:

1. Auditar o inventário e os oito slots existentes.
2. Implementar perfis funcionais de armas.
3. Criar atributos e modificadores de equipamentos.
4. Implementar defesa e resistências de armaduras.
5. Integrar acessórios.
6. Criar interface de gerenciamento de equipamentos.
7. Implementar comparação de itens.
8. Criar aparência visual híbrida.
9. Integrar equipamentos ao combate.
10. Integrar equipamentos à progressão.
11. Integrar recompensas aos sistemas existentes.
12. Preservar Save V2 e migrações.
13. Garantir compatibilidade com a Forma Espectral.
14. Preparar expansão futura do catálogo.

A V2.0 deverá entregar um conjunto pequeno, porém funcional, de equipamentos variados.

Não implemente centenas de itens nesta fase.

---

# 2. ESTADO CONHECIDO DO PROJETO

O relatório da V1.9 informou:

- Branch `art/v1.2-protagonist`.
- HEAD `204f965`.
- 304 testes Rust passando.
- 17 testes Python passando.
- Autoplay 23/23.
- Visual Check aprovado.
- Mundo com 22 salas em três regiões.
- Forma Espectral jogável.
- Relíquia do Véu integrada.
- Transformação pela tecla T.
- Save V2 preservado.
- Inventário funcional para recompensas.
- Baús e segredos persistentes.
- Rota espectral com 14 transições e zero reposicionamentos artificiais.
- Nenhum commit, push ou staging.

Confirme esses dados diretamente no repositório.

Não presuma que a documentação corresponde integralmente ao código.

---

# 3. REGRAS ABSOLUTAS DE SEGURANÇA

Antes de qualquer alteração:

1. Registre branch e HEAD.
2. Registre `git status --porcelain=v1 -uall`.
3. Identifique arquivos staged.
4. Identifique arquivos modificados.
5. Identifique arquivos não rastreados.
6. Identifique arquivos ignorados relevantes.
7. Registre hashes dos arquivos protegidos.
8. Confirme o estado dos saves.
9. Confirme o estado dos assets publicados.
10. Confirme a linha de base dos testes.

Preserve integralmente:

- `Md-usario/`, incluindo as reorganizações existentes.
- `saves/`.
- `.cargo/config.toml`.
- `target/verify/`.
- `target/f4-stage/`.
- Modelos Blender históricos.
- Backups artísticos.
- Assets publicados.
- PNG previamente staged.
- Todas as alterações não commitadas.

É proibido:

- `git clean`.
- `git reset --hard`.
- `cargo clean`.
- `git commit`.
- `git push`.
- `git add`.
- Troca de branch.
- Merge.
- Rebase.
- Criação de worktree.
- Exclusão de backups.
- Desabilitar hooks.
- Instalação de dependências sem autorização.
- Alteração de saves reais.
- Publicação artística fora do pipeline seguro.

Não modifique o índice Git.

---

# 4. REGRA PERMANENTE DE COMPILAÇÃO

A V1.7 sofreu um travamento por falta de memória durante o linking de vários binários do Bevy.

Utilize no máximo dois jobs.

```bash
export CARGO_BUILD_JOBS=2
```

Execute somente um comando Cargo por vez.

Se necessário, reduza para um job.

Não execute múltiplas compilações simultâneas.

Não apague caches.

Não altere `.cargo/config.toml`.

Registre eventuais falhas por falta de memória.

---

# 5. AUDITORIA OBRIGATÓRIA

Leia os documentos existentes:

- `CLAUDE.md`.
- `docs/GAME_VISION_MASTER.md`.
- `docs/INVENTORY_EQUIPMENT.md`.
- `docs/EQUIPMENT_VISUAL_SYSTEM.md`.
- `docs/WEAPON_EXPANSION.md`.
- `docs/CHARACTER_ATTRIBUTES.md`.
- `docs/COMBAT_SYSTEM.md`.
- `docs/SKILLS_SYSTEM.md`.
- `docs/MAGIC_SYSTEM.md`.
- `docs/TRANSFORMATION_SYSTEM.md`.
- `docs/TRANSFORMATIONS_V1_9.md`.
- `docs/WORLD_ARCHITECTURE.md`.
- `docs/WORLD_PERSISTENCE.md`.
- `docs/DECISIONS.md`.
- `docs/ROADMAP.md`.

Inspecione:

- Modelos de itens.
- Inventário.
- Equipamentos.
- Atributos.
- Cálculo de dano.
- Ataques.
- Hitboxes.
- Animações.
- PlayerVisual.
- Sistema de transformação.
- Save/load.
- Interface.
- Dados RON.

Confirme os caminhos reais.

Não crie sistemas paralelos quando já existir uma implementação reutilizável.

Produza uma matriz distinguindo:

- Implementado.
- Parcial.
- Apenas documentado.
- Inexistente.
- Dependência externa.
- Risco de regressão.

---

# 6. ARQUITETURA DE EQUIPAMENTOS

A arquitetura deverá separar:

- Definição do item.
- Instância do item, se necessária.
- Inventário.
- Slot de equipamento.
- Estado equipado.
- Modificadores.
- Perfil de arma.
- Perfil visual.
- Persistência.

Prefira dados tipados e IDs estáveis.

Não armazene Entity IDs do Bevy em saves.

Não duplique dados que possam ser derivados.

Não crie um sistema excessivamente abstrato.

A implementação deverá permitir adicionar novos itens por dados, sem reescrever o código central.

---

# 7. OITO SLOTS EXISTENTES

A auditoria da V1.6 identificou oito slots de equipamento.

Antes de implementar, confirme os nomes, a semântica e o comportamento reais desses slots.

**Preserve o contrato existente.**

Não substitua silenciosamente a organização por outra.

Se os slots atuais não corresponderem exatamente à proposta abaixo, documente a diferença e adapte a implementação ao modelo real.

Categorias desejadas:

- Arma principal.
- Cabeça.
- Tronco.
- Braços ou mãos.
- Pernas ou pés.
- Capa ou vestimenta.
- Acessório 1.
- Acessório 2.

Não adicione slots sem necessidade comprovada.

Se for indispensável modificar a estrutura persistida, interrompa essa operação e solicite autorização.

---

# 8. TIPOS DE ARMAS

A auditoria anterior identificou categorias existentes:

- Sword.
- Dagger.
- Axe.
- Spear.
- HeavyWeapon.

Atualmente, essas categorias podem representar apenas compatibilidade de slot.

A V2.0 deverá dar identidade funcional às categorias.

## 8.1 Espadas

Características propostas:

- Alcance intermediário.
- Velocidade equilibrada.
- Dano equilibrado.
- Boa compatibilidade com técnicas existentes.

## 8.2 Adagas

Características propostas:

- Alcance curto.
- Ataques rápidos.
- Menor dano por golpe.
- Boa mobilidade.

## 8.3 Machados

Características propostas:

- Velocidade menor.
- Dano maior.
- Alcance intermediário.
- Recuperação mais longa.

## 8.4 Lanças

Características propostas:

- Alcance longo.
- Ataques direcionais.
- Menor cobertura próxima ao corpo.

## 8.5 Armas pesadas

Características propostas:

- Ataques lentos.
- Dano elevado.
- Maior comprometimento da animação.
- Recuperação prolongada.

Essas características são diretrizes de design, não números finais.

Defina valores provisórios balanceados e documentados.

Não altere os valores globais do combate para implementar diferenças entre armas.

---

# 9. PERFIS DE ARMAS EM DADOS

Cada arma deverá possuir, conforme necessário:

- ID.
- Nome.
- Categoria.
- Descrição.
- Dano base.
- Tipo de dano.
- Elemento.
- Escalonamento por atributo.
- Alcance.
- Perfil de ataque.
- Multiplicador de velocidade.
- Recuperação.
- Requisitos.
- Raridade.
- Valor de venda.
- Perfil visual.
- Compatibilidade com técnicas.

Evite campos sem uso real.

Não crie dados que a engine ignore silenciosamente.

Valide limites e combinações inválidas.

Uma arma não poderá ter alcance negativo, tempo inválido ou categoria desconhecida.

---

# 10. INTEGRAÇÃO COM O COMBATE

O combate já possui timings aprovados.

Preserve os valores de referência da arma inicial.

Não modifique:

- Fórmula geral de dano sem justificativa.
- Regras de defesa.
- Cooldowns de magias.
- Consumo de MP.
- Comandos direcionais.
- Regras de morte.
- Hitboxes dos inimigos.
- Hurtboxes do jogador.

Integre os perfis de arma por meio de pontos de extensão controlados.

## 10.1 Ataque básico

O ataque básico deverá utilizar o perfil da arma equipada.

Verifique:

- Dano.
- Alcance.
- Duração.
- Recuperação.
- Direção.
- Hitbox.
- Animação.
- Feedback.

## 10.2 Técnicas especiais

Não presuma que todas as armas possam executar todas as técnicas.

Defina compatibilidade explícita.

Preserve Crescent Slash e Rising Edge com a arma inicial.

Se uma técnica exigir espada, não permita sua execução com uma arma incompatível.

Não altere comandos existentes.

## 10.3 Troca de arma durante ataque

Defina comportamento seguro.

Minha recomendação:

- Não permitir equipar ou remover arma durante um ataque ativo.
- Não permitir mudança de arma durante hitbox ativa.
- Não permitir mudança de arma durante uma transformação.
- Não permitir troca durante transição de sala.

Teste cada situação.

---

# 11. ANIMAÇÕES POR CATEGORIA

As categorias de arma deverão possuir comportamento visual coerente.

Entretanto, o protagonista atual possui animações publicadas e aprovadas para a espada inicial.

Não substitua essas animações.

## Estratégia V2.0

Priorize:

1. Preservar animações da espada inicial.
2. Criar perfis de ataque funcionais.
3. Utilizar animações provisórias claramente identificadas quando necessário.
4. Preparar o pipeline para animações específicas.
5. Não publicar assets sem o guarda seguro.

Não declare uma arma visualmente concluída se ela utilizar uma animação incompatível.

Se for possível gerar variantes reais pelo pipeline existente sem comprometer assets protegidos, faça isso em área isolada e valide.

Não crie um sistema que exija renderizar todas as combinações de armas e armaduras antecipadamente.

---

# 12. ARMADURAS E ROUPAS

Implemente equipamentos defensivos para os slots existentes.

Cada peça poderá oferecer:

- Defesa física.
- Defesa mágica, se já existir no modelo de combate.
- Modificadores de atributos.
- Resistências elementais.
- Modificadores de HP.
- Modificadores de MP.
- Modificadores de regeneração.
- Propriedades especiais futuras.

Não implemente todos os modificadores de uma vez se a arquitetura atual não os suportar.

Priorize os que possam ser integrados e testados sem reescrever o combate.

---

# 13. RESISTÊNCIAS ELEMENTAIS

O jogo já possui elementos de magia.

Avalie como as resistências são calculadas.

Equipamentos poderão modificar resistências a:

- Fogo.
- Gelo.
- Raio.
- Água.
- Vento.
- Terra.
- Luz.
- Trevas.
- Arcano.

Não crie sistemas duplicados de resistência.

Defina limites para evitar:

- Resistência infinita.
- Dano negativo.
- Cura involuntária.
- Acúmulo incorreto.
- Overflow.
- Modificadores aplicados duas vezes.

Teste equipar, remover e substituir peças.

---

# 14. ACESSÓRIOS

Implemente acessórios com efeitos mensuráveis.

Exemplos:

- Anel de aumento de MP.
- Amuleto de resistência elemental.
- Anel de regeneração.
- Talismã de atributos.
- Pingente de proteção.

Reutilize o Veilwoven Band criado na V1.9.

Verifique se seus bônus de +6 MP máximo e +0,2 MP/s estão efetivamente integrados.

Não duplique a implementação.

Dois acessórios equipados deverão combinar seus efeitos de maneira determinística.

---

# 15. CÁLCULO DE ATRIBUTOS

Separe claramente:

- Atributos base.
- Progressão por nível.
- Pontos distribuídos.
- Modificadores de equipamento.
- Modificadores temporários.
- Valores finais.

Evite aplicar modificadores diretamente aos atributos base.

O sistema deve recalcular valores finais de forma determinística.

Ao remover uma peça, os bônus devem desaparecer.

Não permita acumulação permanente por equipar e remover repetidamente.

## 15.1 HP e MP máximos

Defina o comportamento quando o máximo diminui.

Exemplo:

- HP atual acima do novo máximo deverá ser ajustado.
- MP atual acima do novo máximo deverá ser ajustado.

Não restaure HP ou MP gratuitamente ao trocar equipamento, salvo regra explícita.

Não permita recursos negativos.

---

# 16. RARIDADES

Implemente ou consolide raridades.

Proposta:

- Common.
- Uncommon.
- Rare.
- Epic.
- Legendary.

Confirme o que já existe.

Raridade deverá representar classificação e potencial de design.

Não deve modificar automaticamente todos os atributos sem regra explícita.

Não implemente geração procedural de equipamentos nesta fase.

---

# 17. CATÁLOGO INICIAL DE EQUIPAMENTOS

Crie um catálogo pequeno e representativo.

Meta sugerida:

- 2 espadas.
- 2 adagas.
- 2 machados.
- 2 lanças.
- 2 armas pesadas.
- 2 capacetes.
- 2 peças de tronco.
- 2 equipamentos de braços.
- 2 equipamentos de pernas.
- 2 capas ou vestimentas.
- 4 acessórios.

Total sugerido: aproximadamente 24 itens, ajustável conforme os slots reais.

Não sacrifique qualidade para atingir a quantidade.

Cada item deverá possuir:

- Identidade própria.
- Categoria correta.
- Atributos válidos.
- Raridade.
- Descrição.
- Compatibilidade.
- Valor de venda, quando aplicável.
- Teste de carregamento.

Use nomes originais.

---

# 18. SISTEMA VISUAL HÍBRIDO

A V1.6 recomendou a arquitetura C, híbrida.

A V2.0 deverá consolidar essa direção.

## 18.1 Objetivo

Permitir que equipamentos alterem a aparência do personagem sem exigir renderizar previamente todas as combinações possíveis.

## 18.2 Estratégia

Combine, quando tecnicamente apropriado:

- Sprite base do personagem.
- Camadas de equipamento.
- Variantes visuais.
- Anexos de armas.
- Alterações discretas de cor.
- Elementos cosméticos.
- Sprites compostos.

Não presuma que qualquer técnica funciona automaticamente com os atlas existentes.

Audite:

- Pivôs.
- Frames.
- Orientação.
- Espelhamento.
- Ordem das camadas.
- Proporções.
- Compatibilidade com animações.
- Compatibilidade com a Forma Espectral.

## 18.3 Evitar explosão combinatória

Não renderize cada combinação completa de equipamento.

Planeje uma solução que possa crescer para centenas de itens.

Não crie centenas de arquivos de sprites vazios.

---

# 19. APARÊNCIA POR PARTE DO CORPO

O sistema deverá preparar suporte visual para:

- Cabeça.
- Tronco.
- Braços.
- Pernas.
- Capa.
- Arma.

Nem todos os slots precisam possuir aparência única imediatamente.

Na V2.0, entregue pelo menos três mudanças visuais reais e distintas, se o pipeline atual permitir sem comprometer os assets protegidos.

Prioridades:

1. Arma visível.
2. Capa ou vestimenta.
3. Cabeça ou tronco.

Não use apenas mudança de texto no inventário como evidência de aparência visual.

Se o pipeline não suportar determinada camada com segurança, implemente a infraestrutura e documente a limitação.

---

# 20. ESPADA E ESPELHAMENTO

A V1.4 identificou que o espelhamento do sprite pode trocar visualmente a mão que segura a espada.

Não amplie esse problema com novas armas.

Audite:

- Orientação do personagem.
- Pivôs.
- Mão dominante.
- Posição da arma.
- Espelhamento horizontal.
- Ordem das camadas.

Se a solução exigir refazer a arte original, documente a limitação e preserve o pipeline.

Não faça alterações artísticas destrutivas.

---

# 21. FORMA ESPECTRAL E EQUIPAMENTOS

A V1.9 implementou a Forma Espectral.

Os equipamentos deverão ser compatíveis com ela.

## 21.1 Atributos

Os bônus passivos continuam aplicados conforme o sistema existente.

Exemplo: um anel que aumenta MP máximo pode aumentar a duração disponível da transformação.

## 21.2 Aparência

A representação espectral deverá considerar a aparência equipada.

Não deixe camadas humanas opacas sobre o corpo translúcido.

## 21.3 Restrições

Não permita trocar equipamento enquanto a forma estiver ativa, conforme a política de combate adotada.

Não permita que a troca de equipamento altere indevidamente o perfil de colisão espectral.

Teste transformação, reversão e save/load com equipamentos diferentes.

---

# 22. INTERFACE DE INVENTÁRIO

Implemente ou expanda uma interface de inventário jogável.

Deverá permitir:

- Visualizar itens.
- Identificar categoria.
- Identificar raridade.
- Visualizar atributos.
- Selecionar item.
- Equipar.
- Remover.
- Comparar.
- Visualizar slots ocupados.
- Visualizar itens incompatíveis.

Não substitua a tela de status sem necessidade.

Audite os controles antes de escolher a tecla.

Não reutilize comandos de combate de maneira conflitante.

---

# 23. COMPARAÇÃO DE EQUIPAMENTOS

Ao selecionar um item, mostre as diferenças em relação ao equipamento atual.

Exemplos:

- Dano.
- Defesa.
- STR.
- CON.
- INT.
- LCK.
- HP máximo.
- MP máximo.
- Resistências.
- Velocidade de ataque, quando aplicável.

Não apresente números que não sejam realmente calculados pelo jogo.

Não use valores inventados na interface.

A comparação deve refletir os atributos finais efetivos.

---

# 24. EQUIPAR E REMOVER

Implemente regras seguras.

Valide:

- Item existe.
- Item pertence ao jogador.
- Slot correto.
- Categoria compatível.
- Requisitos atendidos.
- Estado do jogador permite.
- Item não está duplicado.
- Troca não está bloqueada.

Não permita:

- Equipar o mesmo item em dois slots indevidamente.
- Perder o item anterior.
- Duplicar itens.
- Aplicar bônus duas vezes.
- Equipar durante morte.
- Equipar durante transformação.
- Equipar durante transição.
- Equipar durante ataque, quando incompatível.

---

# 25. INTEGRAÇÃO COM BAÚS

O castelo já possui baús e recompensas persistentes.

Adicione alguns equipamentos ao conteúdo existente.

Não substitua recompensas importantes da progressão sem necessidade.

Teste:

1. Abrir baú.
2. Receber equipamento.
3. Abrir inventário.
4. Equipar.
5. Ver atributos.
6. Ver aparência.
7. Salvar.
8. Mudar de sala.
9. Carregar.
10. Confirmar equipamento e aparência.

Não conceda a recompensa duas vezes.

---

# 26. INTEGRAÇÃO COM INIMIGOS

Prepare o sistema para drops futuros.

Na V2.0, implemente somente o necessário para demonstrar recompensas de combate, se o sistema de loot já existir ou puder ser estendido com segurança.

Não crie uma economia completa.

Não implemente lojas.

Não altere a IA dos inimigos.

Não altere taxas de drop existentes sem justificativa.

---

# 27. SAVE V2

Preserve o Save V2.

Não implemente Save V3 sem autorização.

Audite a representação atual de:

- Inventário.
- Equipamentos.
- Atributos.
- Recursos.
- Progressão.
- Relíquias.

Prefira ampliar definições de itens e cálculos derivados sem alterar o schema persistido.

Se o Save V2 não suportar uma funcionalidade essencial, documente a incompatibilidade e solicite autorização antes de modificar o formato.

Não modifique saves reais.

## Testes obrigatórios

- Equipar e salvar.
- Carregar equipamento.
- Remover e salvar.
- Trocar arma e salvar.
- Dois acessórios.
- Item incompatível.
- Item inexistente.
- Save V1.
- Save V2.
- Transformação após load.
- Atributos após load.
- Aparência após load.

Não grave em `saves/` durante os testes.

---

# 28. BALANCEAMENTO INICIAL

Crie um pequeno documento de balanceamento.

Compare as cinco categorias de arma.

Avalie:

- Dano por golpe.
- Ataques por segundo.
- DPS teórico.
- Alcance.
- Recuperação.
- Compatibilidade com técnicas.
- Risco durante o ataque.

Não equilibre somente pelo DPS.

Uma arma lenta pode oferecer maior alcance ou impacto.

Uma adaga rápida não deve necessariamente superar todas as outras categorias.

Registre os números reais usados no código.

Não altere inimigos globalmente apenas para compensar armas novas.

---

# 29. TESTES AUTOMATIZADOS

Crie testes para:

1. Carregamento de catálogo.
2. IDs duplicados.
3. Categoria inválida.
4. Slot incompatível.
5. Requisitos.
6. Modificadores.
7. Equipar.
8. Remover.
9. Trocar.
10. Dois acessórios.
11. HP máximo.
12. MP máximo.
13. Resistências.
14. Arma equipada.
15. Dano.
16. Alcance.
17. Velocidade.
18. Recuperação.
19. Técnica incompatível.
20. Transformação.
21. Aparência.
22. Save/load.
23. Migração.
24. Baús.
25. Ausência de duplicação.
26. Regressão do combate.

Não altere testes antigos apenas para eliminar falhas.

---

# 30. TESTE DE ROTA REAL

Crie um exemplo de rota automatizada.

Sugestão:

`examples/world_equipment_route.rs`

A rota deverá:

1. Iniciar o jogo.
2. Explorar o castelo.
3. Encontrar um baú.
4. Obter uma arma.
5. Abrir o inventário.
6. Equipar a arma.
7. Confirmar mudança de atributos.
8. Confirmar mudança de combate.
9. Encontrar uma peça de armadura.
10. Equipá-la.
11. Confirmar modificadores.
12. Ativar a Forma Espectral.
13. Confirmar compatibilidade visual.
14. Retornar à forma humana.
15. Salvar.
16. Mudar de sala.
17. Carregar.
18. Confirmar inventário, equipamentos e aparência.

Utilize entradas reais.

Não utilize teletransportes de teste.

O contador de reposicionamentos artificiais deverá ser zero.

---

# 31. VALIDAÇÃO VISUAL

Execute o jogo real com janela.

Produza capturas mostrando:

- Inventário.
- Slots.
- Comparação.
- Espada inicial.
- Outra espada.
- Adaga.
- Machado.
- Lança.
- Arma pesada.
- Peça de armadura equipada.
- Capa equipada.
- Acessórios.
- Forma Espectral com equipamentos.
- Save/load.

Não declare aparência visual concluída quando só houver mudança de estatísticas.

Abra e inspecione as capturas.

Se possível, produza uma demonstração curta.

---

# 32. PERFORMANCE

Meça:

- Tempo de equipar.
- Tempo de remover.
- Tempo de recalcular atributos.
- Tempo de abrir inventário.
- Quantidade de entidades visuais adicionais.
- Memória.
- Tempo de save/load.
- Custo das camadas visuais.

Não recalcule todos os atributos em todos os frames sem necessidade.

Não carregue centenas de texturas desnecessariamente.

Não gere combinações completas de sprites em runtime sem justificativa.

---

# 33. REGRESSÃO

Preserve:

- Movimento humano.
- Salto.
- Corrida.
- Esquiva.
- Defesa.
- Ataque inicial.
- Técnicas.
- Magias.
- Dano.
- Morte.
- Revive.
- Inimigos.
- Mapa.
- Baús.
- Segredos.
- Mecanismos.
- Atalhos.
- Save V1/V2.
- Forma Espectral.
- Câmera.
- Animações.
- VFX.
- Pipeline artístico.

Não modifique o movimento humano para acomodar armas pesadas.

Não altere a física do jogo.

---

# 34. COMANDOS DE VALIDAÇÃO

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
- Testes de save.
- Testes de combate.
- Testes de aparência.

Se houver pressão de memória, utilize um job.

Não execute comandos Cargo simultaneamente.

Não apague caches.

---

# 35. DOCUMENTAÇÃO OBRIGATÓRIA

Crie:

`docs/EQUIPMENT_EXPANSION_V2_0.md`

Documente:

1. Auditoria.
2. Arquitetura.
3. Slots.
4. Armas.
5. Perfis.
6. Ataques.
7. Técnicas.
8. Armaduras.
9. Resistências.
10. Acessórios.
11. Atributos.
12. Raridades.
13. Catálogo.
14. Aparência.
15. Camadas.
16. Inventário.
17. Comparação.
18. Save/load.
19. Transformações.
20. Baús.
21. Balanceamento.
22. Testes.
23. Performance.
24. Riscos.
25. Próximas etapas.

Atualize os documentos existentes afetados.

Registre decisões novas em `docs/DECISIONS.md`.

Não reescreva documentação inteira sem necessidade.

---

# 36. CRITÉRIOS DE ACEITAÇÃO

A V2.0 somente estará concluída quando:

- [ ] Estado inicial auditado.
- [ ] Arquivos protegidos preservados.
- [ ] Compilação limitada a dois jobs.
- [ ] Oito slots preservados.
- [ ] Perfis de armas implementados.
- [ ] Cinco categorias funcionais.
- [ ] Dano por arma.
- [ ] Alcance por arma.
- [ ] Velocidade por arma.
- [ ] Recuperação por arma.
- [ ] Compatibilidade com técnicas.
- [ ] Armaduras funcionais.
- [ ] Resistências integradas.
- [ ] Acessórios funcionais.
- [ ] Modificadores sem acumulação indevida.
- [ ] Catálogo validado.
- [ ] Raridades.
- [ ] Inventário jogável.
- [ ] Comparação de equipamentos.
- [ ] Equipar e remover.
- [ ] Aparência visual híbrida.
- [ ] Mudanças visuais reais demonstradas ou impedimento documentado.
- [ ] Compatibilidade com Forma Espectral.
- [ ] Recompensas integradas.
- [ ] Save V2 preservado.
- [ ] Migração V1 preservada.
- [ ] Rota real sem reposicionamentos.
- [ ] Testes automatizados passando.
- [ ] Validação visual.
- [ ] Performance medida.
- [ ] Regressão zero.
- [ ] Documentação atualizada.
- [ ] Nenhum commit.
- [ ] Nenhum push.
- [ ] V2.1 não iniciada.

Se qualquer requisito obrigatório estiver pendente, declare conclusão parcial e explique.

---

# 37. RELATÓRIO FINAL

Entregue:

**A. Estado inicial e Git**

**B. Auditoria da V1.9**

**C. Arquitetura de equipamentos**

**D. Slots**

**E. Armas**

**F. Perfis de combate**

**G. Técnicas especiais**

**H. Armaduras**

**I. Resistências**

**J. Acessórios**

**K. Atributos**

**L. Raridades**

**M. Catálogo**

**N. Aparência visual**

**O. Inventário**

**P. Comparação**

**Q. Equipar/remover**

**R. Baús e recompensas**

**S. Forma Espectral**

**T. Save V2**

**U. Balanceamento**

**V. Testes**

**W. Rota real**

**X. Validação visual**

**Y. Performance**

**Z. Arquivos criados e modificados**

**AA. Estado final do Git**

**AB. Pendências e riscos**

**AC. Próximas etapas recomendadas**

Inclua comandos e resultados reais.

Diferencie:

- Implementado.
- Testado.
- Inspecionado visualmente.
- Parcial.
- Planejado.

Não declare sucesso sem evidência.

---

# 38. CONDIÇÕES DE PARADA

Interrompa a operação afetada se:

- O Save V2 exigir mudança incompatível.
- Houver risco de perda de saves.
- Houver risco de perda de trabalho Git.
- Uma categoria exigir reescrever todo o combate.
- O sistema visual exigir sobrescrever assets protegidos.
- Uma dependência nova for indispensável.
- A memória ficar insuficiente.
- Uma alteração exigir modificar a física humana.
- Uma solução comprometer a Forma Espectral.
- Uma mudança de equipamento causar corrupção persistente.

Documente o problema.

Continue somente tarefas independentes e seguras.

Solicite autorização antes de mudanças fora do escopo.

---

# 39. INSTRUÇÃO FINAL

Você está autorizado a executar a V2.0.

Ordem obrigatória:

1. Auditoria.
2. Arquitetura.
3. Slots.
4. Perfis de armas.
5. Integração de combate.
6. Armaduras.
7. Resistências.
8. Acessórios.
9. Atributos.
10. Catálogo.
11. Sistema visual híbrido.
12. Inventário.
13. Comparação.
14. Baús.
15. Forma Espectral.
16. Save/load.
17. Testes.
18. Rota real.
19. Validação visual.
20. Documentação.
21. Relatório final.

**Não implemente NPCs ou comércio nesta fase.**

Não inicie a V2.1.

Não modifique saves reais.

Não altere a física humana.

Não destrua os assets publicados.

Não faça staging.

Não crie commits.

Não faça push.

A V2.0 deve terminar com o jogador encontrando armas e armaduras, equipando peças diferentes, percebendo mudanças reais nos atributos e no combate, visualizando diferenças de aparência e preservando tudo após save/load.

Ao concluir, apresente o relatório completo e aguarde minha avaliação.