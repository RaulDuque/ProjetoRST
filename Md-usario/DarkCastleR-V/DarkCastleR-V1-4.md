# DARK CASTLE RPG — MEGA PROMPT V1.4
## Expansão das Animações, Combate Visual e Integração com Bevy 0.20.1

### MISSÃO PRINCIPAL

Você é o engenheiro responsável pela animação, integração visual e validação técnica do protagonista do Dark Castle RPG.

Estamos iniciando a **V1.4 — Animation Expansion**.

O projeto utiliza:

- Rust 2024.
- Bevy 0.20.1.
- Blender 5.2.4 LTS.
- Protagonista 3D procedural v007.
- Pipeline Blender → PNG → Spritesheet → Metadados → Bevy.
- Sistema de combate com técnicas, magias, dano, morte e movimentação.
- Arquitetura modular com separação entre gameplay e apresentação visual.

A V1.3 foi concluída tecnicamente.

O protagonista já possui quatro animações integradas:

1. Idle.
2. Run.
3. Basic Attack.
4. Dodge.

O controlador visual utiliza uma entidade filha `PlayerVisual`, separada da lógica de gameplay.

A integração existente foi validada com:

- 146 testes Rust.
- Clippy sem avisos.
- Autoplay 23/23.
- Visual Check 13/13.
- Spritesheets e metadados validados.
- Backup dos arquivos Blender.
- Nenhuma alteração nas regras de gameplay.

**Sua missão é expandir esse sistema para representar os demais estados existentes do protagonista, sem modificar as mecânicas do jogo.**

Não reconstrua o personagem.

Não substitua o controlador V1.3.

Não altere o rig v007 sem justificativa técnica e autorização.

---

# 1. REGRAS ABSOLUTAS

Antes de qualquer operação:

- Inspecione o estado real do Git.
- Registre branch, HEAD, staged e alterações existentes.
- Preserve o PNG previamente staged.
- Preserve alterações não commitadas das etapas anteriores.
- Registre hashes dos arquivos protegidos.
- Confirme a integridade dos arquivos Blender.

É proibido:

- Executar `git clean`.
- Executar `git reset --hard`.
- Executar `cargo clean`.
- Criar commits.
- Executar push.
- Criar ou trocar branches sem autorização.
- Apagar backups.
- Sobrescrever modelos anteriores.
- Modificar `Md-usario/`.
- Modificar saves reais.
- Modificar `.cargo/config.toml`.
- Apagar `target/verify/` ou `target/f4-stage/`.
- Instalar dependências sem autorização.
- Alterar versões de Rust ou Bevy.
- Desabilitar proteções do Claude Code.
- Executar operações destrutivas.

Não modifique:

- Dano.
- Hitboxes.
- Hurtboxes.
- Física.
- Gravidade.
- Velocidade.
- Invulnerabilidade.
- Cooldowns.
- Custos de MP ou Hearts.
- Fórmulas de atributos.
- Progressão.
- Save/load.
- Duração das ações.
- Regras de habilidades.
- Câmera.

A V1.4 é uma fase de **apresentação visual**, não de gameplay.

---

# 2. AUDITORIA INICIAL DO PROJETO

Antes de produzir animações, leia:

- `docs/PLAYER_VISUAL_ANIMATION_V1_3.md`
- `docs/ASSET_CONTRACT.md`
- `docs/ART_DIRECTION.md`
- `docs/ARCHITECTURE.md`
- `docs/COMBAT_SYSTEM.md`
- `docs/MAGIC_SYSTEM.md`
- `art-src/player/ANIMATION_VALIDATION_V1_2G.md`
- `art-src/player/RIG_SPECIFICATION.md`
- `CLAUDE.md`

Localize os caminhos reais, caso algum documento esteja em outro lugar.

Inspecione:

- `src/player/`
- `src/player/visual/`
- `src/combat/`
- `src/skills/`
- `src/magic/`
- `src/world/`
- `src/input/`
- `assets/animations/`
- `assets/sprites/player/`

Identifique todos os estados do jogador que já existem.

Não invente estados apenas para acomodar uma animação.

Crie uma matriz de auditoria:

| Estado | Existe no gameplay? | Duração real | Prioridade | Animação atual | Nova animação necessária |
|---|---|---|---|---|---|
| Idle | Verificar | — | Verificar | Sim | Não |
| Run | Verificar | — | Verificar | Sim | Não |
| Basic Attack | Verificar | Verificar | Verificar | Sim | Não |
| Dodge | Verificar | Verificar | Verificar | Sim | Não |
| Hurt | Verificar | Verificar | Verificar | Fallback | Sim |
| Death | Verificar | Verificar | Verificar | Fallback | Sim |
| Jump | Verificar | Verificar | Verificar | Fallback | Sim |
| Fall | Verificar | Verificar | Verificar | Fallback | Sim |
| Land | Verificar | Verificar | Verificar | Fallback | Avaliar |
| Crouch | Verificar | Verificar | Verificar | Fallback | Sim |
| Guard | Verificar | Verificar | Verificar | Fallback | Sim |
| Crescent Slash | Verificar | Verificar | Verificar | Fallback | Sim |
| Rising Edge | Verificar | Verificar | Verificar | Fallback | Sim |
| Ember Bolt | Verificar | Verificar | Verificar | Fallback | Sim |
| Mending Light | Verificar | Verificar | Verificar | Fallback | Sim |

A matriz deve ser preenchida com evidências do código real.

Se algum estado não existir ou não for observável pelo controlador visual, registre a limitação.

Não crie uma nova mecânica para resolver isso.

---

# 3. PLANEJAMENTO DAS ANIMAÇÕES

A ordem de produção deve ser:

**Grupo A — Sobrevivência**

1. Hurt.
2. Death.

**Grupo B — Movimentação**

3. Jump.
4. Fall.
5. Land, se houver condição confiável para detectar aterrissagem.
6. Crouch.

**Grupo C — Defesa**

7. Guard.

**Grupo D — Técnicas físicas**

8. Crescent Slash.
9. Rising Edge.

**Grupo E — Magias**

10. Ember Bolt.
11. Mending Light.

Antes de produzir cada grupo:

1. Confirme o estado de gameplay correspondente.
2. Confirme a duração.
3. Confirme as regras de interrupção.
4. Confirme a direção.
5. Confirme o enquadramento.
6. Confirme o contrato de sprites.
7. Confirme como o controlador visual reconhecerá a ação.

Não produza animações que não possam ser integradas corretamente.

Se houver uma incompatibilidade estrutural importante, interrompa somente o grupo afetado, documente o impedimento e continue com os grupos independentes que puderem ser executados com segurança.

---

# 4. MODELO E PIPELINE ARTÍSTICO

Utilize o protagonista v007 como fonte.

Preserve:

- 43 ossos.
- Hierarquia.
- Pesos.
- Correção da ombreira.
- Espada vinculada à mão.
- Casaca.
- Meia-capa.
- Gola.
- Botas.
- Materiais.
- Proporções.
- Silhueta.

Utilize uma cópia de trabalho independente para as novas animações.

Não sobrescreva:

- v004.
- v005.
- v006.
- v007.
- Arquivo de animações V1.2-G.
- Arquivos de animação V1.3.

Não crie v008 apenas porque novas animações foram adicionadas.

O modelo continua sendo v007.

Crie uma versão específica dos arquivos de animação da V1.4.

Sugestão:

`art-src/player/blender/swordsman_v007_animations_v1_4.blend`

Adapte ao padrão real.

Mantenha scripts de geração reproduzíveis.

---

# 5. PADRÃO VISUAL

Para esta fase, mantenha:

- Acabamento C.
- Densidade 2.
- Espada ×1,7.
- Câmera 1:1.
- Aproximadamente 52 px de altura no jogo.
- 24 fps como referência para combate.
- 12 fps quando adequado para animações cíclicas.
- PNG RGBA.
- sRGB.
- Alfa reto.
- Pivôs conforme o contrato existente.

Essas escolhas continuam provisórias.

Não altere D2 nem D3 definitivamente.

Não modifique a câmera.

---

# 6. HURT — REAÇÃO A DANO

Crie uma animação de reação a dano.

Ela deve:

- Mostrar impacto corporal.
- Preservar a espada na mão.
- Produzir movimento secundário da capa.
- Produzir movimento secundário da casaca.
- Evitar interseções graves.
- Manter uma silhueta legível.
- Permitir retorno ao estado correto.

A duração deve acompanhar o estado real de dano ou stun.

Se o gameplay não possuir stun, não crie um.

A animação não pode conceder invulnerabilidade.

A animação não pode interromper ações que o gameplay não interrompe.

Se houver dano sem transição de estado explícita, investigue os eventos existentes.

Não invente um novo sistema de dano.

---

# 7. DEATH — MORTE

Crie uma animação de morte completa.

A sequência deve representar:

1. Perda de equilíbrio.
2. Queda.
3. Contato com o chão.
4. Pose final.

Preserve:

- Espada.
- Casaca.
- Capa.
- Proporções.
- Posição lógica do jogador.

A animação deve respeitar o estado real de morte.

Não altere a lógica de HP.

Não altere revive.

Não altere respawn.

Não altere saves.

Se a animação terminar antes do revive, mantenha a pose final.

Se o jogador reviver, a apresentação deve retornar ao estado adequado sem criar entidades duplicadas.

A morte deve ter prioridade sobre as demais animações.

---

# 8. JUMP — SALTO

Crie uma animação de salto.

Ela deve representar:

- Impulso.
- Elevação.
- Flexão das pernas.
- Movimento secundário da casaca.
- Movimento secundário da capa.
- Espada estável.

O salto é controlado pela física existente.

A animação não deve aplicar força.

A animação não deve alterar a velocidade vertical.

A animação não deve alterar a gravidade.

Use o estado real de movimento para determinar a transição para Fall.

Se o jogo permitir ataques no ar, a prioridade visual deve respeitar essas ações.

Não force Jump quando o personagem estiver atacando.

---

# 9. FALL — QUEDA

Crie uma animação de queda.

Ela deve:

- Diferenciar visualmente subida e descida.
- Manter pernas e braços coerentes.
- Evitar deformações exageradas.
- Preservar a espada.
- Permitir transição para aterrissagem.

A transição deve depender do estado real da física.

Não utilize um timer arbitrário para decidir que o jogador começou a cair.

Não crie falsos aterrissamentos após save/load.

---

# 10. LAND — ATERRISSAGEM

Primeiro determine se o gameplay fornece uma transição confiável entre estar no ar e estar no chão.

Se fornecer, crie uma animação curta de aterrissagem.

Ela deve:

- Mostrar absorção do impacto.
- Flexionar as pernas.
- Movimentar a casaca.
- Retornar ao estado adequado.

A animação não deve bloquear movimento.

Não deve alterar colisões.

Não deve alterar velocidade.

Se não houver detecção confiável de aterrissagem, não crie um estado visual falso.

Documente a limitação e mantenha a transição direta Fall → Idle/Run.

---

# 11. CROUCH — AGACHAMENTO

Crie uma animação de agachamento.

Ela deve:

- Reduzir visualmente a altura do personagem.
- Preservar a espada.
- Evitar atravessamentos da casaca.
- Manter os pés no chão.
- Preservar o pivô.

Não altere a hitbox.

Não altere a altura física do personagem.

Não altere colisões.

Se o gameplay possuir apenas um estado de agachamento estático, uma pose estática é suficiente.

Não crie uma mecânica de rastejar.

---

# 12. GUARD — DEFESA

Crie uma animação de guarda.

Ela deve:

- Mostrar postura defensiva.
- Posicionar a espada de maneira coerente.
- Preservar a silhueta.
- Manter o personagem estável.
- Respeitar a direção.

A defesa visual deve acompanhar o estado real de guarda.

Não altere:

- Redução de dano.
- Parry.
- Custos.
- Stamina, caso exista.
- Cooldowns.
- Janelas de defesa.

Se o jogo não possuir uma janela de parry, não crie uma.

---

# 13. CRESCENT SLASH

Crie uma animação própria para a técnica Crescent Slash.

Duração de referência:

**0,40 segundo.**

Confirme no código.

A técnica deve apresentar:

- Golpe amplo.
- Arco de espada legível.
- Movimento do tronco.
- Movimento secundário da capa.
- Movimento secundário da casaca.
- Recuperação coerente.

A hitbox existente começa no início da ação.

Portanto, preserve a regra:

**Impacto primeiro.**

Não coloque uma longa preparação antes do quadro inicial.

Não altere:

- Dano.
- Alcance.
- Hitbox.
- Custos.
- Cooldown.
- Comando de ativação.

O controlador deve distinguir Crescent Slash do ataque básico.

---

# 14. RISING EDGE

Crie uma animação própria para Rising Edge.

Duração de referência:

**0,45 segundo.**

Confirme no código.

A animação deve representar:

- Golpe ascendente.
- Espada em trajetória vertical.
- Movimento coordenado do tronco.
- Movimento secundário da capa.
- Recuperação.

Não faça o personagem saltar fisicamente se a habilidade não produzir esse movimento no gameplay.

A animação deve representar o efeito existente, não criar uma nova mecânica.

Preserve a regra de impacto inicial.

---

# 15. EMBER BOLT

Crie uma animação de conjuração para Ember Bolt.

Duração de referência:

**0,30 segundo.**

Confirme no código.

A animação deve:

- Mostrar gesto de conjuração.
- Manter a espada coerente.
- Preservar a direção.
- Representar o início da magia.
- Permitir retorno ao estado adequado.

O projétil continua sendo criado pelo sistema de magia existente.

A animação não deve criar projéteis.

Não altere:

- MP.
- Cooldown.
- Elemento.
- Dano.
- Velocidade do projétil.
- Colisão.

Se o sistema já possuir um efeito visual de projétil, preserve-o.

Se não houver arte final, mantenha o efeito existente e documente a necessidade de uma etapa futura de VFX.

---

# 16. MENDING LIGHT

Crie uma animação de conjuração para Mending Light.

Duração de referência:

**0,60 segundo.**

Confirme no código.

A animação deve representar:

- Gesto de concentração.
- Postura de conjuração.
- Movimento secundário da capa.
- Movimento secundário da casaca.
- Recuperação.

A cura deve continuar sendo aplicada exclusivamente pelo sistema de gameplay.

A animação não deve alterar HP.

Não altere MP.

Não altere cooldown.

Não altere as regras de ativação.

Se a magia não possuir um evento visual explícito para a cura, registre essa limitação.

Não implemente uma nova mecânica de cura.

---

# 17. INTEGRAÇÃO COM O CONTROLADOR V1.3

A V1.3 já possui:

- Seleção de estados.
- Reprodução temporal.
- Espelhamento.
- Pivôs.
- Fallbacks.
- Carregamento de assets.
- Integração com o jogador.

Expanda essa infraestrutura.

Não a substitua.

Inspecione os módulos reais em:

`src/player/visual/`

Adicione as novas animações de maneira modular.

O controlador deve distinguir:

- Ataque básico.
- Crescent Slash.
- Rising Edge.
- Ember Bolt.
- Mending Light.

Não utilize um único estado visual genérico para todas as habilidades se o gameplay permitir identificá-las corretamente.

Se a identidade da habilidade não estiver disponível, documente o impedimento antes de criar novos componentes.

Não altere o reconhecedor de comandos.

Não altere o sistema de habilidades.

---

# 18. PRIORIDADES E INTERRUPÇÕES

A seleção visual deve seguir as regras reais do jogo.

Ordem conceitual:

1. Death.
2. Hurt, quando representar uma interrupção real.
3. Dodge.
4. Ataques e habilidades.
5. Guard.
6. Jump/Fall/Land.
7. Crouch.
8. Run.
9. Idle.

Essa ordem é apenas uma hipótese inicial.

Confirme no código.

Não altere a prioridade do gameplay para corresponder à animação.

A apresentação deve seguir o estado existente.

Teste:

- Ataque interrompido por dano.
- Esquiva durante movimentação.
- Ataque no ar.
- Morte durante habilidade.
- Revive.
- Save/load durante movimento.
- Mudança de direção.
- Retorno a Idle.
- Retorno a Run.

Não permita reinicializações indevidas de clipes a cada frame.

---

# 19. SPRITESHEETS E METADADOS

Utilize o pipeline existente da V1.3.

Não crie outro formato de metadados sem necessidade.

Preserve:

- PNG RGBA.
- sRGB.
- Alfa reto.
- Pivôs.
- Grade.
- Nomenclatura.
- Limite de dimensões.
- Duração temporal.
- Ordem dos frames.

Verifique se o contrato atual comporta as novas animações.

Se alguma animação exceder a célula prevista, não corte a imagem silenciosamente.

Registre o problema.

Não altere o contrato sem autorização.

Organize os assets de forma coerente com os existentes.

Evite duplicar texturas.

---

# 20. TESTES DE REGRESSÃO

Antes de integrar os novos assets, execute os testes existentes.

Registre a linha de base.

Depois da integração, execute novamente.

Priorize:

- `cargo fmt --all -- --check`
- `cargo check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`

Utilize as regras de compilação segura do `CLAUDE.md`.

Não utilize `--all-features` se isso ativar configurações incompatíveis com o ambiente atual.

Não execute compilações simultâneas.

Não apague caches.

Execute o autoplay e o visual_check quando seguro, preservando os saves reais.

Compare:

- Quantidade de testes.
- Falhas.
- Estados.
- Movimento.
- Dano.
- Ataques.
- Magias.
- Save/load.

Não modifique testes antigos apenas para obter aprovação.

---

# 21. TESTES VISUAIS

Amplie o exemplo visual_check existente.

Crie capturas para:

- Hurt.
- Death.
- Jump.
- Fall.
- Land, se implementado.
- Crouch.
- Guard.
- Crescent Slash.
- Rising Edge.
- Ember Bolt.
- Mending Light.

Inclua:

- Direita.
- Esquerda.
- Transições.
- Interrupções.
- Escala real.
- Verificação de pivô.
- Verificação de cortes.
- Verificação de transparência.

Não basta confirmar que o atlas carregou.

Inspecione o resultado renderizado.

Diferencie:

- Teste automatizado.
- Teste de integração.
- Inspeção visual.
- Avaliação artística.

Não declare uma animação artisticamente aprovada apenas porque o código passou nos testes.

---

# 22. PERFORMANCE

Evite:

- Carregar imagens a cada frame.
- Recriar atlases continuamente.
- Alocar recursos desnecessariamente.
- Duplicar entidades.
- Executar processamento de imagem no gameplay.
- Introduzir dependências circulares.

Meça ou estime:

- Quantidade de texturas.
- Quantidade de frames.
- Memória adicional.
- Custo de atualização.
- Impacto na inicialização.

Compare com a V1.3.

---

# 23. DOCUMENTAÇÃO

Crie:

`docs/PLAYER_ANIMATION_EXPANSION_V1_4.md`

Inclua:

1. Objetivo.
2. Auditoria dos estados.
3. Animações produzidas.
4. Durações.
5. Prioridades.
6. Interrupções.
7. Pipeline Blender.
8. Spritesheets.
9. Metadados.
10. Integração Bevy.
11. Testes.
12. Performance.
13. Limitações.
14. Pendências.
15. Próximas etapas.

Atualize apenas as seções necessárias dos documentos existentes.

Não reescreva a arquitetura inteira.

---

# 24. CRITÉRIOS DE ACEITAÇÃO

A V1.4 será considerada concluída quando:

- [ ] O estado inicial do Git tiver sido auditado.
- [ ] Os arquivos protegidos permanecerem intactos.
- [ ] O rig v007 permanecer preservado.
- [ ] A matriz de estados estiver documentada.
- [ ] Hurt funcionar.
- [ ] Death funcionar.
- [ ] Jump funcionar.
- [ ] Fall funcionar.
- [ ] Crouch funcionar.
- [ ] Guard funcionar.
- [ ] Crescent Slash funcionar.
- [ ] Rising Edge funcionar.
- [ ] Ember Bolt funcionar.
- [ ] Mending Light funcionar.
- [ ] Land funcionar, se houver suporte seguro.
- [ ] Os spritesheets forem válidos.
- [ ] Os pivôs forem preservados.
- [ ] As durações forem corretas.
- [ ] As interrupções respeitarem o gameplay.
- [ ] O controlador V1.3 continuar funcionando.
- [ ] Os testes Rust passarem.
- [ ] O autoplay passar.
- [ ] O visual_check passar.
- [ ] Os saves permanecerem intactos.
- [ ] A câmera permanecer intacta.
- [ ] O gameplay permanecer intacto.
- [ ] A documentação estiver atualizada.
- [ ] Nenhum commit ou push for realizado.

Se algum item não puder ser concluído, explique o motivo.

Não esconda pendências.

---

# 25. RELATÓRIO FINAL OBRIGATÓRIO

Entregue o relatório nesta ordem:

**A. Estado inicial e Git**

Branch, HEAD, staged, modificados e hashes.

**B. Auditoria dos estados**

Matriz completa com durações e prioridades.

**C. Pipeline artístico**

Arquivos Blender, scripts e preservação do rig.

**D. Hurt**

Animação, integração e testes.

**E. Death**

Animação, integração e revive.

**F. Jump / Fall / Land**

Transições e física preservada.

**G. Crouch / Guard**

Posturas e comportamento.

**H. Crescent Slash**

Duração e sincronização.

**I. Rising Edge**

Duração e sincronização.

**J. Ember Bolt**

Conjuração e integração.

**K. Mending Light**

Conjuração e integração.

**L. Controlador visual**

Arquitetura, prioridades e interrupções.

**M. Spritesheets**

Arquivos, dimensões e metadados.

**N. Testes automatizados**

Comandos e resultados.

**O. Validação visual**

Capturas, problemas e avaliação.

**P. Performance**

Memória, texturas e custo adicional.

**Q. Arquivos modificados**

Lista e justificativas.

**R. Pendências**

Problemas restantes e riscos.

**S. Git final**

Branch, HEAD, staged e integridade.

**T. Próximos passos**

Recomendações para V1.5.

---

# 26. REGRA DE ENCERRAMENTO

Você está autorizado a executar a V1.4, respeitando as restrições deste briefing.

Não faça commits.

Não execute push.

Não altere gameplay.

Não modifique a câmera.

Não altere saves.

Não substitua o rig v007.

Não inicie a V1.5 automaticamente.

**Ao concluir, apresente o relatório completo e aguarde minha avaliação.**

A prioridade é transformar o protagonista em um personagem visualmente completo para os estados e habilidades já existentes, preservando a estabilidade conquistada nas etapas anteriores.