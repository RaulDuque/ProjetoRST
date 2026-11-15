# DARK CASTLE RPG — MEGA PROMPT V1.5
## Polimento Artístico, VFX, Segurança do Pipeline e Validação Visual Completa

### MISSÃO OFICIAL — EXECUÇÃO AUTORIZADA

Você é o responsável técnico e artístico pela continuidade do Dark Castle RPG, desenvolvido em Rust 2024, Bevy 0.20.1 e Blender 5.2.4 LTS.

A V1.4 foi aprovada tecnicamente com base no relatório de execução.

Agora você está autorizado a implementar a **V1.5 — Artistic Polish & Visual Effects**.

Esta fase possui cinco objetivos:

1. Proteger os assets atuais contra sobrescritas acidentais por scripts antigos.
2. Melhorar a legibilidade das animações de dano, queda e morte.
3. Implementar efeitos visuais para as habilidades existentes.
4. Produzir variantes visuais de combate e transições ainda ausentes, quando houver suporte real no gameplay.
5. Criar uma demonstração animada do protagonista dentro do jogo.

A implementação deve ser incremental, reproduzível, testável e reversível.

Não reabra a arquitetura de gameplay.

Não substitua o protagonista v007.

Não descarte as animações aprovadas tecnicamente.

---

# 1. CONTEXTO TÉCNICO

O projeto possui:

- Protagonista v007 com 43 ossos.
- Pipeline Blender → renderização → pós-processamento → spritesheets → manifesto RON → Bevy.
- Controlador `PlayerVisual` independente do gameplay.
- Sistema de estados visuais e prioridades.
- Spritesheets de locomoção e combate.
- Suporte a durações exatas, interrupções e espelhamento.
- Testes automatizados e exemplos de validação visual.

A V1.4 terminou com:

| Verificação | Resultado informado |
|---|---|
| Testes Rust | 158/158 |
| Autoplay | 23/23 |
| Visual Check | 30/30 |
| Clippy | Sem avisos |
| Gameplay | Sem mudanças relatadas |
| Git | Branch `art/v1.2-protagonist`, HEAD `204f965` |
| Commits da etapa | Nenhum |

Confirme esses dados no repositório antes de utilizá-los como linha de base.

O protagonista possui animações para:

- Idle.
- Run.
- Basic Attack.
- Dodge.
- Hurt.
- Death.
- Jump.
- Fall.
- Land.
- Crouch.
- Guard.
- Crescent Slash.
- Rising Edge.
- Ember Bolt.
- Mending Light.

O total informado na V1.4 foi de 115 quadros em duas texturas, aproximadamente 15,5 MB na GPU.

---

# 2. REGRAS INVIOLÁVEIS

## 2.1 Git

Antes de qualquer modificação, registre:

- Branch atual.
- HEAD.
- Índice/stage.
- Arquivos modificados.
- Arquivos não rastreados.
- Arquivos ignorados relevantes.
- Diferenças existentes.
- Hashes dos arquivos protegidos.

Existe um PNG previamente colocado no stage pelo usuário.

Preserve-o exatamente.

As alterações das etapas V1.2-D até V1.4 ainda podem estar sem commit.

Não sobrescreva nem descarte essas alterações.

É proibido executar:

- `git clean`
- `git reset --hard`
- `cargo clean`
- `git commit`
- `git push`
- Rebase
- Merge
- Troca ou exclusão de branch
- Operações destrutivas de histórico

Não use `git add .` ou `git add -A`.

Nesta fase, não faça nenhum staging.

## 2.2 Arquivos protegidos

Não modifique:

- `Md-usario/`
- `saves/`
- `.cargo/config.toml`
- `target/verify/`
- `target/f4-stage/`
- Backups artísticos anteriores
- Arquivos `.blend` históricos
- Modelos-base v004 a v007
- RON de gameplay
- Configurações de segurança do Claude Code, salvo autorização específica

Não apague `.blend1`.

Não altere `.gitignore`.

Não configure Git LFS.

Não instale dependências sem autorização.

## 2.3 Gameplay

É proibido modificar:

- Dano.
- Hitboxes.
- Hurtboxes.
- Colisões.
- Gravidade.
- Movimento.
- Velocidade.
- Invulnerabilidade.
- Cooldowns.
- Custos de MP e Hearts.
- Progressão.
- Regras de habilidades.
- Reconhecimento de comandos.
- Save/load.
- Duração das ações.
- Câmera.

Os VFX devem observar os eventos existentes.

**Nenhum efeito visual pode produzir dano, cura ou alteração física.**

---

# 3. FASE A — AUDITORIA E LINHA DE BASE

Antes de implementar, leia os arquivos reais:

- `CLAUDE.md`
- `docs/PLAYER_VISUAL_ANIMATION_V1_3.md`
- `docs/PLAYER_ANIMATION_EXPANSION_V1_4.md`
- `docs/ASSET_CONTRACT.md`
- `docs/ART_DIRECTION.md`
- `docs/ARCHITECTURE.md`
- `docs/COMBAT_SYSTEM.md`
- `docs/MAGIC_SYSTEM.md`
- `docs/DECISIONS.md`
- `art-src/README.md`

Localize os caminhos existentes.

Inspecione:

- `src/player/visual/`
- `src/combat/`
- `src/skills/`
- `src/magic/`
- `src/world/`
- `assets/sprites/player/`
- `assets/animations/`
- `art-src/player/`

Confirme:

1. Como os atlas são gerados.
2. Como o manifesto é produzido.
3. Como os scripts escolhem o destino.
4. Como o Bevy carrega as texturas.
5. Como os eventos de habilidades são emitidos.
6. Como o controlador visual identifica as ações.
7. Como o visual reage a dano, morte e revive.
8. Como a física sinaliza pulo, queda e aterrissagem.

Execute os testes leves necessários para estabelecer a linha de base, respeitando o limite de compilação do projeto.

Se a linha de base estiver quebrada, documente antes de atribuir falhas à V1.5.

---

# 4. FASE B — SEGURANÇA DO PIPELINE ARTÍSTICO

Esta é a primeira implementação obrigatória.

Existe um risco conhecido:

**Executar o pós-processamento da V1.3 isoladamente pode sobrescrever os atlas atuais, removendo as animações adicionadas na V1.4.**

Isso não pode continuar dependendo apenas de um aviso na documentação.

## 4.1 Auditoria dos geradores

Identifique todos os scripts que podem escrever em:

- `assets/sprites/player/`
- `assets/animations/player.ron`
- Diretórios de saída usados pelo jogo.
- Spritesheets de locomoção.
- Spritesheets de combate.

Classifique cada gerador:

- Histórico.
- Experimental.
- Atual.
- Compatível com o contrato atual.
- Potencialmente destrutivo.

Não execute scripts antigos diretamente sobre os assets atuais para testar o risco.

## 4.2 Proteção obrigatória

Implemente uma proteção explícita que impeça scripts históricos de sobrescrever a versão atual por acidente.

A solução deve considerar:

- Saída experimental por padrão.
- Destino de produção explícito.
- Verificação de versão.
- Validação de manifesto.
- Verificação de clipes obrigatórios.
- Confirmação explícita para substituição.
- Escrita temporária e publicação segura.
- Falha sem alteração dos assets existentes.

Não utilize apenas um aviso impresso.

Não confie somente em nomes de diretórios.

Não permita que um script V1.3 publique quatro clipes sobre um manifesto V1.4 contendo mais animações.

Se a proteção exigir uma migração grande do pipeline, pare e apresente a proposta.

## 4.3 Publicação segura

Implemente um fluxo equivalente a:

```text
Gerar em diretório temporário
          ↓
Validar PNGs e manifesto
          ↓
Conferir versão e clipes obrigatórios
          ↓
Comparar hashes com os assets atuais
          ↓
Preparar backup recuperável
          ↓
Publicar somente com autorização explícita
          ↓
Validar novamente
```

A autorização desta V1.5 permite publicar os novos assets necessários à integração, desde que a publicação seja não destrutiva para as fontes e tenha rollback verificável.

Ela não permite usar uma opção genérica de sobrescrita para contornar uma validação.

Se não for possível garantir uma publicação segura, interrompa a publicação e mantenha os novos assets em uma área experimental.

## 4.4 Testes de segurança

Crie testes para:

1. Script antigo tentando publicar atlas incompleto.
2. Manifesto sem clipe obrigatório.
3. PNG inválido.
4. Dimensões incompatíveis.
5. Pivô inválido.
6. Duração inválida.
7. Versão incompatível.
8. Destino protegido.
9. Falha durante geração.
10. Falha durante publicação.
11. Execução repetida.
12. Recuperação após falha.

Use diretórios de teste isolados.

Não corrompa os assets reais para testar essas condições.

**Critério de aprovação:** scripts históricos não podem substituir silenciosamente os assets atuais.

---

# 5. FASE C — REVISÃO ARTÍSTICA DAS ANIMAÇÕES

As animações da V1.4 passaram na validação técnica.

Agora precisamos avaliar sua qualidade visual em movimento.

Priorize:

- Hurt.
- Fall.
- Death.

Mantenha os demais clipes existentes.

Não refaça animações sem necessidade.

---

# 6. HURT — MELHORAR A LEITURA DO DANO

O clipe atual possui:

- 9 quadros.
- Duração de 0,35 s.
- Impacto no quadro inicial.

Problema observado:

**O recuo é sutil na escala real do jogo.**

Crie uma variante experimental mais legível.

Sugestões artísticas:

- Reação mais evidente dos ombros.
- Inclinação controlada do tronco.
- Movimento da cabeça.
- Deslocamento visual secundário da capa.
- Silhueta de impacto mais forte.
- Recuperação coerente.

Não altere o deslocamento físico do jogador.

Não adicione knockback.

Não altere o tempo de Hurt.

Não altere invulnerabilidade.

Produza uma comparação:

- Hurt V1.4 original.
- Hurt V1.5 variante A.
- Hurt V1.5 variante B, somente se houver benefício real.

Compare na escala 1:1.

Não escolha automaticamente a variante com maior amplitude.

Priorize legibilidade, anatomia e continuidade.

---

# 7. FALL — MELHORAR A LEITURA DA QUEDA

O clipe atual funciona tecnicamente, mas a queda é visualmente discreta.

Crie uma variante que comunique melhor o movimento descendente.

Avalie:

- Posição das pernas.
- Inclinação do tronco.
- Orientação da espada.
- Movimento da casaca.
- Movimento da meia-capa.
- Diferença de silhueta em relação ao Jump.

A física continua responsável pelo movimento vertical.

Não altere gravidade.

Não altere velocidade.

Não introduza um estado novo de gameplay.

Não force uma animação de queda quando o jogador estiver em ataque aéreo.

Compare a versão original com a variante V1.5.

---

# 8. DEATH — COMPARAÇÃO DE POSES FINAIS

A V1.4 utiliza uma pose final ajoelhada.

Ela foi escolhida porque a tentativa anterior de morte deitada apresentou:

- Casaca erguida de maneira artificial.
- Corpo aparentemente flutuando.
- Problemas de contato com o chão.

Não descarte a pose ajoelhada.

Crie uma alternativa de morte com queda completa.

## 8.1 Variante A

Mantenha a morte ajoelhada da V1.4.

## 8.2 Variante B

Produza uma sequência em que o personagem:

1. Perde o equilíbrio.
2. Cai.
3. Encontra o chão.
4. Acomoda a casaca.
5. Termina em uma pose deitada convincente.

Corrija a geometria visual da casaca usando poses, ajustes artísticos ou deformação controlada, sem alterar o rig-base.

Não introduza simulação de tecido em tempo real no Bevy.

Se usar simulação offline no Blender, ela deve ser reproduzível e convertida em frames.

## 8.3 Comparação obrigatória

Compare:

- Silhueta.
- Contato com o chão.
- Naturalidade.
- Legibilidade em 1:1.
- Posição da espada.
- Deformação da casaca.
- Transição para revive.
- Risco de clipping.

**Não substitua definitivamente a morte ajoelhada sem aprovação artística.**

Se a variante deitada ficar melhor, mantenha ambas e recomende uma decisão.

---

# 9. FASE D — SISTEMA DE VFX

Implemente uma camada de efeitos visuais separada da lógica de gameplay.

O objetivo é tornar os golpes e magias mais legíveis.

Os VFX devem ser:

- Visuais.
- Temporários.
- Reutilizáveis.
- Leves.
- Sincronizados.
- Testáveis.
- Independentes de dano e física.

Não crie um novo sistema de combate.

## 9.1 Arquitetura

Inspecione a estrutura existente antes de criar novos módulos.

Uma organização possível:

```text
src/
└── player/
    └── visual/
        └── vfx/
            ├── mod.rs
            ├── components.rs
            ├── assets.rs
            ├── systems.rs
            └── tests.rs
```

A estrutura é ilustrativa.

Adapte ao projeto.

Não crie arquivos vazios.

Os efeitos devem observar eventos e estados já existentes.

Evite disparar efeitos duas vezes quando um evento e uma mudança de estado ocorrerem no mesmo frame.

---

# 10. VFX — CRESCENT SLASH

Crie um arco visual para Crescent Slash.

A duração da habilidade é aproximadamente 0,40 s.

Confirme no código.

O efeito deve:

- Acompanhar o golpe.
- Ter uma trajetória coerente.
- Respeitar a direção.
- Ser visível na escala real.
- Aparecer no momento do impacto.
- Desaparecer automaticamente.

O arco é puramente visual.

Não altere o alcance da hitbox.

Não altere dano.

Não altere colisões.

Não altere cooldown.

Não use o arco para decidir quando o ataque acertou.

Teste as direções esquerda e direita.

---

# 11. VFX — RISING EDGE

Crie um efeito visual de corte ascendente.

Ele deve:

- Comunicar a direção vertical do golpe.
- Acompanhar a espada.
- Ser diferente do Crescent Slash.
- Respeitar a direção.
- Não encobrir o personagem excessivamente.
- Terminar dentro da duração visual adequada.

Não altere a habilidade.

Não aplique força.

Não crie um salto físico.

Não modifique hitboxes.

---

# 12. VFX — MENDING LIGHT

Crie um efeito visual de cura.

A habilidade possui duração aproximada de 0,60 s.

Confirme no código.

O efeito pode utilizar:

- Partículas luminosas.
- Pequenos brilhos.
- Aura suave.
- Pulsação luminosa.
- Elementos ascendentes.

Mantenha a estética gótica do projeto.

Evite efeitos excessivamente modernos ou chamativos.

O efeito deve indicar visualmente a conjuração e o momento em que a cura realmente acontece, conforme os eventos existentes.

Não aplique cura pelo VFX.

Não modifique HP.

Não modifique MP.

Não altere cooldown.

Não crie um novo evento de gameplay se os sinais existentes forem suficientes.

Se o momento exato da cura não for observável, registre a limitação em vez de inventar uma sincronização.

---

# 13. EMBER BOLT — VERIFICAÇÃO VISUAL

O Ember Bolt já possui um projétil visual no jogo.

Não substitua automaticamente esse projétil.

Primeiro verifique:

- Visibilidade.
- Contraste.
- Direção.
- Spawn.
- Sincronização com a mão.
- Compatibilidade com a animação de conjuração.

Se houver problema exclusivamente visual, proponha uma melhoria localizada.

Não altere velocidade, colisão ou dano.

A criação de um novo projétil artístico completo não é obrigatória nesta fase.

---

# 14. FASE E — VARIANTES VISUAIS

Depois de estabilizar o pipeline e os VFX, avalie as variantes que faltam.

A prioridade é:

1. Ataque aéreo.
2. Impacto na guarda.
3. Respawn.
4. Virada de direção.

Não implemente todas indiscriminadamente.

Primeiro confirme os eventos e estados existentes.

---

# 15. ATAQUE AÉREO

Atualmente o ataque no ar reutiliza o clipe do chão.

Verifique se o gameplay permite atacar no ar e em quais condições.

Se permitir, crie uma variante visual apropriada.

Ela deve:

- Preservar o impacto inicial.
- Preservar a duração real.
- Manter pernas em postura aérea.
- Evitar aparência de chão invisível.
- Preservar a espada.
- Permitir transição para Fall.

Não altere o ataque.

Não altere a gravidade.

Não altere hitboxes.

Não altere o tempo da ação.

Se houver mais de um ataque aéreo, documente quais usam a variante.

---

# 16. IMPACTO NA GUARDA

Verifique se o gameplay possui um sinal confiável de ataque bloqueado.

Se houver, crie uma reação visual curta.

Ela pode incluir:

- Recuo dos braços.
- Pequeno movimento da espada.
- Impacto luminoso discreto.
- Movimento da capa.

Não crie parry.

Não altere defesa.

Não altere redução de dano.

Não altere invulnerabilidade.

Se não existir sinal confiável, não simule bloqueio com base em proximidade de inimigos.

Documente a limitação.

---

# 17. RESPAWN / REVIVE

Verifique como o jogador revive.

Crie uma transição visual somente se houver um evento ou mudança de estado confiável.

A animação deve:

- Sair corretamente da pose de morte.
- Restaurar a apresentação.
- Evitar duplicação de sprites.
- Evitar reaparecimento deslocado.
- Preservar o estado lógico.

Não altere HP.

Não altere posição.

Não altere save/load.

Não altere as regras de revive.

---

# 18. VIRADA DE DIREÇÃO E ESPELHAMENTO

Existe uma limitação artística:

Ao espelhar o sprite, a espada troca visualmente de mão e a capa troca de ombro.

Não altere a direção lógica do combate.

Avalie alternativas:

- Manter espelhamento atual.
- Criar renders específicos para esquerda.
- Ajustar artisticamente a simetria.
- Utilizar variantes de apresentação.

Compare custo de produção, legibilidade e coerência.

Não gere automaticamente todos os clipes para ambos os lados sem justificar o custo.

Não altere a solução atual antes de uma avaliação.

---

# 19. D2, D3 E FPS

Mantenha as decisões provisórias:

- D2: acabamento C.
- D3: câmera 1:1.
- Espada ×1,7.
- Combate em 24 fps.
- Locomoção conforme os clipes atuais.

Não altere o zoom.

Não altere o filtro global de texturas.

Não altere a resolução lógica.

Não converta todo o projeto para pixel art.

Prepare apenas uma comparação fundamentada para uma decisão artística posterior.

---

# 20. DEMONSTRAÇÃO ANIMADA OBRIGATÓRIA

Crie uma demonstração visual reproduzível do protagonista.

Sequência desejada:

```text
Idle
  ↓
Run
  ↓
Jump
  ↓
Fall
  ↓
Land
  ↓
Basic Attack
  ↓
Dodge
  ↓
Hurt
  ↓
Death
  ↓
Revive
```

Adicione, se possível, uma segunda demonstração com:

```text
Crescent Slash
  ↓
Rising Edge
  ↓
Ember Bolt
  ↓
Mending Light
```

## 20.1 Formato

Produza:

- GIF animado ou vídeo curto.
- Capturas de referência.
- Comparação entre V1.4 e V1.5.
- Imagens em escala real 1:1.
- Uma versão ampliada apenas para inspeção.

Se houver ferramentas de vídeo já instaladas, utilize-as.

Não instale dependências sem autorização.

Se não for possível produzir vídeo, gere GIF pelo pipeline existente.

Se também não for possível, documente a limitação e produza uma sequência de quadros com tempos identificados.

## 20.2 Autenticidade

A demonstração deve representar o resultado real do jogo.

Não apresente uma animação offline do Blender como se fosse uma captura do Bevy.

Se produzir uma comparação offline, identifique claramente a origem.

O ideal é capturar o protagonista renderizado pelo Bevy.

Não afirme que a demonstração foi inspecionada visualmente se apenas verificou a existência dos arquivos.

---

# 21. TESTES VISUAIS E AUTOMATIZADOS

Amplie os testes existentes sem enfraquecê-los.

Valide:

1. Os atlas atuais continuam íntegros.
2. Scripts antigos não conseguem sobrescrevê-los.
3. Manifestos inválidos são rejeitados.
4. Hurt mantém 0,35 s.
5. Death mantém a pose final.
6. Revive não duplica entidades.
7. Fall respeita a física.
8. Land não aparece falsamente após F9.
9. Crescent Slash mantém 0,40 s.
10. Rising Edge mantém 0,45 s.
11. Ember Bolt mantém 0,30 s.
12. Mending Light mantém 0,60 s.
13. VFX não altera gameplay.
14. VFX não cria dano.
15. VFX não aplica cura.
16. VFX respeita direção.
17. VFX desaparece corretamente.
18. Ataque aéreo preserva a física.
19. Guarda preserva as regras.
20. Save/load permanece intacto.
21. O controlador V1.3/V1.4 continua funcionando.
22. Os arquivos protegidos permanecem intactos.

Execute os comandos apropriados do projeto, incluindo:

```bash
cargo fmt --all -- --check
cargo check
cargo clippy --all-targets -- -D warnings
cargo test
```

Execute o autoplay e o visual_check com as proteções existentes para saves.

Não execute compilações simultâneas.

Não apague caches.

Não altere testes antigos para esconder regressões.

---

# 22. PERFORMANCE

Compare a V1.5 com a V1.4.

Registre:

- Quantidade de texturas.
- Quantidade de quadros.
- Memória estimada.
- Entidades temporárias de VFX.
- Frequência de criação e destruição.
- Custo por frame.
- Tempo de carregamento, quando mensurável.

Evite:

- Alocação excessiva por frame.
- Recarregamento de texturas.
- Duplicação de efeitos.
- Vazamento de entidades.
- Efeitos permanentes após interrupções.
- Processamento de imagem durante o gameplay.

Não introduza um sistema de partículas complexo se sprites simples resolverem.

---

# 23. DOCUMENTAÇÃO

Crie:

`docs/PLAYER_ART_POLISH_V1_5.md`

Documente:

1. Estado inicial.
2. Segurança do pipeline.
3. Proteções implementadas.
4. Comparações artísticas.
5. Hurt.
6. Fall.
7. Death.
8. VFX.
9. Variantes.
10. Espelhamento.
11. Decisões provisórias.
12. Demonstração animada.
13. Testes.
14. Performance.
15. Riscos.
16. Pendências.
17. Próximas etapas.

Atualize apenas os documentos existentes diretamente afetados.

Não reescreva toda a arquitetura.

---

# 24. CRITÉRIOS DE ACEITAÇÃO

A V1.5 só poderá ser declarada tecnicamente concluída se:

- [ ] Estado inicial auditado.
- [ ] Arquivos protegidos preservados.
- [ ] Pipeline protegido contra scripts antigos.
- [ ] Publicação segura validada.
- [ ] Hurt comparado.
- [ ] Fall comparado.
- [ ] Death ajoelhada preservada.
- [ ] Death deitada avaliada.
- [ ] VFX Crescent Slash funcionando.
- [ ] VFX Rising Edge funcionando.
- [ ] VFX Mending Light funcionando.
- [ ] Ember Bolt revisado.
- [ ] Ataque aéreo avaliado e implementado quando seguro.
- [ ] Impacto na guarda avaliado.
- [ ] Revive avaliado.
- [ ] Espelhamento documentado.
- [ ] Demonstração animada produzida.
- [ ] Testes automatizados passando.
- [ ] Autoplay passando.
- [ ] Visual Check passando.
- [ ] Gameplay preservado.
- [ ] Câmera preservada.
- [ ] Saves preservados.
- [ ] Git preservado.
- [ ] Nenhum commit realizado.
- [ ] Nenhum push realizado.

Se algum critério não for atendido, informe exatamente qual, por quê e o impacto.

Não declare aprovação artística definitiva sem avaliação do usuário.

---

# 25. RELATÓRIO FINAL

Entregue:

**A. Estado inicial**

Branch, HEAD, Git e hashes.

**B. Segurança do pipeline**

Riscos, proteções e testes.

**C. Hurt**

Comparação entre versões.

**D. Fall**

Comparação entre versões.

**E. Death**

Comparação entre ajoelhada e deitada.

**F. VFX Crescent Slash**

Implementação e sincronização.

**G. VFX Rising Edge**

Implementação e sincronização.

**H. VFX Mending Light**

Implementação e sincronização.

**I. Ember Bolt**

Revisão visual.

**J. Ataque aéreo**

Resultado e limitações.

**K. Guarda**

Impacto e sinais disponíveis.

**L. Revive**

Transição e integridade.

**M. Espelhamento**

Alternativas e recomendação.

**N. D2 / D3 / FPS**

Comparações e decisões pendentes.

**O. Demonstração**

Arquivos, duração, origem e método de captura.

**P. Testes**

Resultados e regressões.

**Q. Performance**

Comparação com a V1.4.

**R. Arquivos**

Criados, modificados e preservados.

**S. Git final**

Branch, HEAD, staged, alterações e hashes.

**T. Pendências e próximos passos**

Riscos, recomendações e preparação da V1.6.

---

# 26. CONDIÇÕES DE PARADA

Interrompa a operação afetada se:

- Um script ameaçar sobrescrever assets sem recuperação.
- Um arquivo protegido precisar ser alterado.
- Um teste exigir modificar gameplay.
- O rig v007 precisar de uma alteração estrutural.
- Uma dependência nova for necessária.
- A publicação dos assets não puder ser validada.
- O estado do Git apresentar risco de perda.
- O custo de uma solução exceder significativamente o escopo.
- A validação revelar uma regressão importante que não possa ser corrigida dentro da camada visual.

Documente o impedimento e continue apenas com trabalhos independentes e seguros.

---

# 27. INSTRUÇÃO FINAL

Você está autorizado a executar a V1.5.

Trabalhe na ordem:

1. Auditoria.
2. Proteção do pipeline.
3. Comparações artísticas.
4. VFX.
5. Variantes visuais.
6. Demonstração animada.
7. Testes.
8. Documentação.
9. Relatório final.

Não crie commits.

Não faça push.

Não altere gameplay.

Não modifique a câmera.

Não altere saves.

Não substitua o rig-base v007.

Não inicie a V1.6.

**Ao concluir, apresente o relatório completo, os caminhos das demonstrações e as decisões artísticas pendentes. Aguarde minha avaliação antes de prosseguir.**