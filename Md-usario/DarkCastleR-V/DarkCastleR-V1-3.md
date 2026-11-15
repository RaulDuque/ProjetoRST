# DARK CASTLE RPG — MEGA PROMPT V1.3
## Integração do Protagonista Animado no Bevy 0.20.1
### Player Visual Animation Controller — Implementação, Validação e Auditoria

---

# 0. SUA MISSÃO

Você é o engenheiro principal responsável pela integração visual do protagonista do Dark Castle RPG.

O jogo utiliza:

- Rust 2024.
- Bevy 0.20.1.
- Arquitetura modular baseada em plugins.
- Blender 5.2.4 LTS para produção artística.
- Sprites 2D renderizados a partir de modelos 3D.
- Sistema de combate com habilidades e comandos direcionais.
- Separação entre lógica de gameplay e apresentação visual.

Estamos iniciando oficialmente a **V1.3 — Player Visual Animation Controller**.

A fase artística anterior produziu o protagonista v007, baseado no conceito aprovado:

**Espadachim Aristocrático Sombrio.**

O personagem possui casaca escura, detalhes vinho, meia-capa, cabelos curtos e sabre curvo.

O rig v007 foi validado com:

- 43 ossos.
- Pesos normalizados.
- Espada vinculada à mão.
- Ombreira corrigida.
- Animações técnicas de caminhada, ataque e esquiva.
- Spritesheets experimentais.
- JSONs de metadados.
- Testes de reprodutibilidade.

A V1.2-G foi aprovada como base técnica.

Sua missão é fazer esse protagonista aparecer e se animar corretamente dentro do jogo, **sem alterar as regras existentes do gameplay**.

O trabalho deve ser incremental, verificável e reversível.

Não implemente toda a biblioteca de animações nesta fase.

---

# 1. REGRAS DE SEGURANÇA

Antes de começar, estas restrições são obrigatórias.

## 1.1 Proibido sem nova autorização

Não execute:

- `git clean`
- `git reset --hard`
- `cargo clean`
- `git push`
- `git commit`
- Exclusão de branches
- Rebase
- Merge
- Alterações destrutivas de histórico

Não modifique:

- `Md-usario/`
- `saves/`
- `target/verify/`
- `target/f4-stage/`
- `.cargo/config.toml`
- Modelos Blender anteriores
- Scripts artísticos históricos
- Arquivos RON de gameplay, salvo autorização posterior

Não instale dependências sem autorização.

Não modifique versões de Rust ou Bevy.

Não utilize serviços externos de geração de assets.

Não envie arquivos do projeto para terceiros.

Não faça commits ou push.

## 1.2 Alterações autorizadas

Você pode:

- Criar componentes de apresentação visual.
- Criar sistemas de animação.
- Criar módulos Rust relacionados à apresentação.
- Adicionar assets visuais ao diretório apropriado.
- Criar testes automatizados.
- Criar exemplos ou ferramentas de validação.
- Criar documentação técnica.
- Adaptar exclusivamente o código necessário para associar a apresentação ao jogador existente.

Qualquer mudança no gameplay deve ser interrompida e apresentada como proposta.

---

# 2. AUDITORIA INICIAL OBRIGATÓRIA

Antes de modificar arquivos, inspecione o repositório.

Execute verificações equivalentes a:

```bash
git branch --show-current
git rev-parse HEAD
git status --short
git diff --check
```

Registre:

1. Branch atual.
2. HEAD.
3. Arquivos staged.
4. Arquivos modificados.
5. Arquivos não rastreados.
6. Arquivos ignorados relevantes.
7. Estado das versões artísticas.
8. Estado dos diretórios protegidos.

O último HEAD conhecido era `204f965`, na branch `art/v1.2-protagonist`.

**Não presuma que ainda seja esse o estado atual.**

Existe um PNG de referência anteriormente colocado no stage pelo usuário.

Não o remova nem o inclua inadvertidamente em qualquer operação.

As alterações das etapas V1.2-D até V1.2-G podem continuar sem commit.

Preserve todas.

Crie um inventário de hashes dos arquivos protegidos antes de começar.

Ao final, compare novamente.

---

# 3. LEITURA DA DOCUMENTAÇÃO

Leia os documentos existentes e utilize-os como fonte de verdade.

Priorize:

- `README.md`
- `docs/ARQUITETURA.md`, se existir.
- `docs/GAME_DESIGN.md`, se existir.
- `docs/COMBAT_SYSTEM.md`, se existir.
- `docs/INPUT_COMMANDS.md`, se existir.
- `docs/ART_DIRECTION.md`
- `docs/ASSET_CONTRACT.md`
- `art-src/README.md`
- `art-src/player/RIG_SPECIFICATION.md`
- `art-src/player/RIG_VALIDATION_V006.md`
- `art-src/player/ANIMATION_VALIDATION_V1_2G.md`

Localize os documentos pelos caminhos reais.

Não crie duplicatas desnecessárias.

Inspecione os módulos Rust existentes:

- `src/player/`
- `src/combat/`
- `src/input/`
- `src/skills/`
- `src/magic/`
- `src/world/`
- `src/ui/`
- `src/app/`, se existir.

Identifique:

- Entidade principal do jogador.
- Componentes de movimento.
- Estado de ataque.
- Estado de esquiva.
- Estado de magia.
- Estado de dano.
- Estado de morte.
- Direção do personagem.
- Sistemas de spawn.
- Sistemas de câmera.
- Sistemas de renderização.
- Ordem de execução dos sistemas.

Não invente APIs do Bevy.

Confirme a compatibilidade com Bevy 0.20.1 no código instalado e nos exemplos locais.

---

# 4. BACKUP VERIFICÁVEL ANTES DA INTEGRAÇÃO

Antes de modificar o pipeline visual, preserve os marcos artísticos.

Arquivos prioritários:

- Modelo-base v004.
- Rig v007.
- Arquivo das animações v007.
- Scripts de geração correspondentes.
- Metadados necessários para reconstrução.

Os arquivos `.blend` estão ignorados pelo Git.

Portanto, não dependa exclusivamente do repositório para preservá-los.

## 4.1 Procedimento

Primeiro identifique os caminhos e os arquivos reais.

Depois:

1. Crie um backup local não destrutivo, em um diretório novo e exclusivo para esta etapa, preferencialmente fora da árvore Git.
2. Preserve os nomes e a estrutura relativa.
3. Gere um manifesto SHA-256.
4. Verifique os hashes das cópias.
5. Confirme que os originais permaneceram intactos.
6. Registre a localização do backup.

Se não houver destino seguro disponível, não escolha um caminho arbitrário com risco de sobrescrita.

Nesse caso, registre o impedimento e solicite um destino antes de modificar arquivos artísticos indispensáveis.

Não apague `.blend1`.

Não configure Git LFS.

Não altere `.gitignore`.

Não realize operações destrutivas.

**A autorização para criar um backup local verificável está concedida.**

---

# 5. CONTRATO VISUAL DA V1.3

As decisões provisórias para esta fase são:

| Item | Decisão |
|---|---|
| Modelo | v007 |
| Conceito | Espadachim Aristocrático Sombrio |
| Acabamento principal | C — cores chapadas |
| Acabamento alternativo | B — alta resolução |
| Acabamento A | Experimental, não prioritário |
| Câmera | 1:1, sem alterações |
| Espada | Variante ×1,7 para testes |
| Combate | 24 fps |
| Locomoção | 12 fps quando visualmente adequado |
| Velocidade do jogador | Preservar 190 px/s |
| Ataque básico | Preservar 0,28 s |
| Esquiva | Preservar 0,28 s |
| Sincronização | Impacto no início da ação |

Essas escolhas são provisórias para a integração.

Não as trate como decisões artísticas definitivas.

---

# 6. ARQUITETURA DO PLAYER VISUAL

O personagem deve possuir uma apresentação visual separada da entidade lógica.

A arquitetura desejada é:

```text
Player (entidade de gameplay)
│
├── Componentes existentes
│   ├── Transform
│   ├── Movimento
│   ├── Vida
│   ├── Combate
│   ├── Habilidades
│   └── Estados
│
└── PlayerVisual (entidade filha)
    ├── Sprite
    ├── TextureAtlas
    ├── AnimationState
    ├── AnimationTimer
    ├── Facing
    └── VisualOffset
```

Este diagrama é conceitual.

Adapte os nomes e componentes às APIs reais de Bevy 0.20.1 e à arquitetura do projeto.

Não crie componentes redundantes se já existirem equivalentes.

## 6.1 Responsabilidades

A entidade principal continua responsável por:

- Movimento.
- Física.
- Colisões.
- Vida.
- Dano.
- Ataques.
- Hitboxes.
- Habilidades.
- Recursos.
- Cooldowns.
- Save/load.

A entidade visual fica responsável por:

- Sprite atual.
- Frame atual.
- Direção visual.
- Animação em reprodução.
- Temporização visual.
- Pivô.
- Offset gráfico.
- Transições de apresentação.

A entidade visual não deve controlar física nem combate.

---

# 7. ANIMAÇÕES DA PRIMEIRA INTEGRAÇÃO

Implemente somente as animações necessárias para validar a arquitetura:

1. Idle.
2. Run.
3. Basic Attack.
4. Dodge.

Walk poderá permanecer como animação experimental de referência, sem precisar representar o deslocamento normal do jogador.

Não implemente nesta etapa:

- Toda a biblioteca de magias.
- Todas as habilidades.
- Animações completas de chefes.
- Novos estados de combate.
- Novos sistemas de física.
- Novas mecânicas de movimento.

## 7.1 Idle

Crie ou derive uma animação simples de Idle.

Ela deve:

- Manter o protagonista reconhecível.
- Preservar a espada.
- Permitir movimento secundário discreto da capa.
- Evitar movimento excessivo.
- Repetir sem saltos.
- Manter os pés no chão.

Se o pipeline ainda não tiver Idle definitivo, utilize uma versão técnica temporária.

Não bloqueie a integração por refinamentos cosméticos.

## 7.2 Run

A velocidade real de deslocamento é aproximadamente 190 px/s.

O ciclo Walk experimental foi construído para aproximadamente 36 px/s.

Não reutilize o ciclo Walk acelerando-o 5,28 vezes.

Crie um ciclo de corrida apropriado à velocidade existente.

A corrida deve possuir:

- Passadas mais amplas.
- Tronco inclinado de maneira coerente.
- Movimento dos braços.
- Movimento da espada.
- Movimento secundário da casaca.
- Movimento secundário da meia-capa.
- Contato convincente dos pés.
- Silhueta legível em 52 px.
- Ciclo contínuo.

### 7.2.1 Sincronização com velocidade

Meça o deslocamento visual por ciclo.

Calcule a cadência necessária para corresponder à velocidade do jogador.

Documente:

- Comprimento visual da passada.
- Duração do ciclo.
- Quantidade de passos.
- Velocidade de referência.
- Deslizamento residual estimado.

Se o ciclo não puder corresponder perfeitamente aos 190 px/s sem parecer artificial, priorize uma corrida visualmente convincente e registre a diferença.

Não altere a velocidade do gameplay.

Não introduza uma nova mecânica de caminhada lenta.

---

# 8. BASIC ATTACK

O ataque básico possui duração aproximada de 0,28 segundo.

Confirme o valor no código.

A hitbox começa no início da ação e permanece ativa durante parte dela.

O sistema visual deve:

1. Detectar o início real do ataque.
2. Iniciar a animação no frame correspondente ao impacto.
3. Preservar a duração total.
4. Avançar os frames corretamente.
5. Respeitar a direção do jogador.
6. Permitir interrupções quando o gameplay permitir.
7. Retornar ao estado visual adequado após a ação.

Não altere:

- Hitbox.
- Hurtbox.
- Dano.
- Cooldown.
- Attack speed.
- Janela de ataque.
- Física.

## 8.1 Sincronização temporal

Não assuma que todos os frames possuem a mesma duração.

O pipeline experimental produziu uma animação com duração total de 0,28 s e um último frame mais curto.

O controlador deve suportar duração por frame ou uma representação temporal equivalente.

Evite arredondar 0,28 s para 0,25 s ou 0,2917 s.

Se `attack_speed` modificar a duração efetiva, a animação visual deverá acompanhar essa duração, sem alterar a lógica de gameplay.

---

# 9. DODGE

A esquiva também possui duração aproximada de 0,28 segundo.

Confirme no código.

O gameplay controla o deslocamento real.

O sprite não deve deslocar a entidade principal.

O controlador visual deve:

- Detectar o início da esquiva.
- Iniciar a animação.
- Respeitar a duração real.
- Preservar a direção.
- Permitir interrupções apenas quando autorizadas pelo gameplay.
- Retornar a Idle ou Run conforme o estado real.

A animação experimental não possui translação de raiz.

Preserve essa característica.

Não altere invulnerabilidade, colisão ou física.

---

# 10. ESTADOS E PRIORIDADES VISUAIS

Implemente um resolvedor de estados visuais.

A ordem de prioridade deve refletir o gameplay real.

Como ponto de partida conceitual:

```text
Death
  ↓
Hurt / Stun
  ↓
Dodge
  ↓
Attack / Skill / Casting
  ↓
Run
  ↓
Idle
```

Essa ordem não é uma autorização para alterar a máquina de estados existente.

Inspecione o gameplay e ajuste a prioridade visual às regras reais.

## 10.1 Regra fundamental

**O estado visual deve refletir o gameplay, nunca comandá-lo.**

Não permita que o fim de uma animação encerre um ataque antes da hora.

Não permita que o início de um sprite gere dano.

Não permita que uma transição visual conceda invulnerabilidade.

Não permita que uma animação de morte modifique a lógica de revive.

## 10.2 Estados ainda sem sprites

Se Hurt, Death, Guard, Casting ou habilidades ainda não possuírem sprites próprios:

- Utilize fallback visual seguro.
- Preserve o estado real.
- Não represente uma ação inexistente como se estivesse acontecendo.
- Registre quais animações faltam.

Não crie automaticamente toda a biblioteca artística.

---

# 11. DIREÇÃO DO PERSONAGEM

O personagem deve continuar virando para a esquerda e direita conforme a lógica existente.

Verifique:

- Qual lado é a orientação original dos sprites.
- Qual mão segura a espada.
- Qual lado da capa aparece na câmera.
- Como o jogo representa a direção.

Implemente o espelhamento visual quando apropriado.

Não altere a direção lógica do combate.

Não altere as hitboxes.

Não altere a posição real do jogador.

Se houver inconsistência visual ao espelhar a espada ou a capa, documente.

---

# 12. CARREGAMENTO DOS ASSETS

Inspecione os spritesheets e metadados existentes.

Eles foram gerados experimentalmente em:

`art-src/player/comparisons/v1_2g_animation_tests/`

Não presuma os nomes exatos.

Identifique os arquivos reais.

Crie uma estrutura apropriada dentro de `assets/`, respeitando a organização do projeto.

Sugestão:

```text
assets/
└── player/
    └── protagonist/
        ├── idle.png
        ├── run.png
        ├── attack_basic.png
        ├── dodge.png
        └── animations.ron
```

A estrutura é apenas uma proposta.

Se o projeto já possui convenções de assets, siga-as.

## 12.1 Requisitos

- Caminhos estáveis.
- Spritesheets válidos.
- Metadados coerentes.
- Pivôs corretos.
- Tamanhos de célula corretos.
- Alfa preservado.
- Sem cortes na espada.
- Sem cortes na casaca.
- Sem distorção de proporção.
- Carregamento compatível com Bevy 0.20.1.

Não copie indiscriminadamente todas as variantes experimentais para o jogo.

Inclua apenas os assets necessários à integração inicial.

---

# 13. CONTROLADOR DE ANIMAÇÕES

Crie uma implementação modular.

Sugestão conceitual:

```text
src/player/
├── visual/
│   ├── mod.rs
│   ├── components.rs
│   ├── assets.rs
│   ├── animation.rs
│   ├── state.rs
│   ├── systems.rs
│   └── tests.rs
```

Adapte à estrutura existente.

Não crie arquivos vazios apenas para cumprir essa sugestão.

## 13.1 Funcionalidades

O controlador deve:

- Carregar animações.
- Associar animações à entidade visual.
- Identificar o estado de gameplay.
- Resolver a animação correta.
- Controlar o tempo.
- Atualizar o frame.
- Preservar pivôs.
- Aplicar espelhamento.
- Suportar loops.
- Suportar animações não cíclicas.
- Suportar interrupções.
- Evitar reiniciar animações a cada frame.
- Lidar com assets ainda não carregados.
- Lidar com estados sem animação própria.

## 13.2 Determinismo

Sempre que possível, mantenha a lógica de seleção e avanço temporal testável sem renderização.

Evite depender exclusivamente de testes manuais.

Use testes unitários para:

- Seleção de estado.
- Prioridade.
- Avanço de frame.
- Loop.
- Interrupção.
- Mudança de direção.
- Duração de ações.
- Retorno a Idle/Run.

---

# 14. ORDEM DE EXECUÇÃO DOS SISTEMAS

Este é um ponto crítico.

Inspecione a ordem atual dos sistemas de:

- Input.
- Movimento.
- Combate.
- Habilidades.
- Magia.
- Atualização de estados.
- Transform.
- Renderização.

A atualização visual deve ocorrer depois de os estados relevantes do gameplay terem sido calculados.

Não introduza atrasos artificiais de um frame.

Não crie dependências circulares entre plugins.

Se for necessário estabelecer ordenação explícita de sistemas, faça isso apenas na camada de apresentação.

Documente a ordem final.

---

# 15. CÂMERA E ESCALA

A câmera atual deve permanecer intacta.

Não modifique:

- Zoom.
- Projeção.
- Seguimento.
- Limites.
- Resolução lógica.
- Escala do mundo.

O protagonista deve respeitar o contrato:

- Altura aproximada de 52 px.
- Movimento: 64×64.
- Pivô: (32, 63).
- Combate: 128×96.
- Pivô: (40, 95).

Verifique a correspondência entre pivôs e o sistema de coordenadas do Bevy.

Não suponha que os pivôs de spritesheets diferentes podem usar o mesmo offset sem cálculo.

O personagem não deve saltar visualmente ao trocar de Idle/Run para Attack.

Teste a posição dos pés antes, durante e depois da troca.

---

# 16. COMPATIBILIDADE COM SAVE/LOAD

O jogo já possui sistema de save/load.

Não altere o formato dos saves.

O controlador visual deve ser reconstruível a partir do estado real do personagem.

Após carregar um save:

- A entidade visual deve existir.
- A direção deve estar correta.
- A animação deve corresponder ao estado atual.
- Não deve haver duplicação de sprites.
- Não deve haver referências inválidas.
- Não deve haver alteração de vida, MP, posição ou inventário.

Se o jogo recriar o jogador durante o carregamento, o sistema visual deve acompanhar corretamente esse fluxo.

---

# 17. TESTES AUTOMATIZADOS

Implemente testes para:

1. Spawn da entidade visual.
2. Associação ao jogador.
3. Ausência de duplicação.
4. Carregamento de assets.
5. Seleção Idle.
6. Seleção Run.
7. Seleção Basic Attack.
8. Seleção Dodge.
9. Prioridades.
10. Espelhamento.
11. Loop.
12. Interrupção.
13. Duração exata de 0,28 s.
14. Troca entre spritesheets de tamanhos diferentes.
15. Pivô dos pés.
16. Fallback de animação.
17. Ausência de alteração em gameplay.
18. Compatibilidade com save/load.
19. Reconstrução visual após respawn.
20. Comportamento com assets ainda não carregados.

Execute também os testes existentes do projeto.

Use os comandos reais do repositório.

Quando aplicável:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo check --all-targets --all-features
```

Respeite as configurações de compilação existentes.

Não apague caches de compilação.

Se algum comando falhar por causa de ambiente, dependência ou problema anterior, diferencie claramente a causa.

---

# 18. TESTES DE GAMEPLAY

Verifique que a integração não altera:

- Velocidade do jogador.
- Pulo.
- Gravidade.
- Colisões.
- Dano.
- Defesa.
- Esquiva.
- Ataques.
- Habilidades.
- Magias.
- Cooldowns.
- Experiência.
- Inventário.
- Save/load.
- Revive.

Não basta compilar.

Execute os testes de regressão existentes.

Se o projeto possuir exemplo de autoplay ou smoke tests, utilize-os.

Compare os resultados antes e depois, quando possível.

Não altere testes antigos apenas para fazê-los passar.

---

# 19. VALIDAÇÃO VISUAL REAL

Execute o jogo em modo apropriado.

Confirme visualmente:

1. O protagonista aparece corretamente.
2. Idle funciona.
3. Run funciona.
4. Basic Attack funciona.
5. Dodge funciona.
6. A direção muda corretamente.
7. Os pés não saltam entre animações.
8. A espada não é cortada.
9. A casaca não é cortada.
10. A capa permanece visível.
11. O personagem mantém proporção correta.
12. Não há duplicação de sprites.
13. O fundo não fica opaco.
14. O personagem permanece alinhado ao chão.
15. A câmera continua igual.
16. A movimentação mantém a velocidade original.

Se não houver ambiente gráfico disponível, utilize um método de renderização ou teste visual reproduzível.

Não declare que viu o jogo funcionando se só executou testes sem janela.

Documente a diferença entre validação automatizada e inspeção visual.

---

# 20. PERFORMANCE

A integração visual deve ser leve.

Evite:

- Carregar texturas a cada frame.
- Criar novos handles continuamente.
- Recriar entidades desnecessariamente.
- Recalcular metadados estáticos.
- Fazer alocações evitáveis por frame.
- Executar processamento de imagem durante o gameplay.
- Criar sistemas excessivamente acoplados.

Use recursos compartilhados para spritesheets e metadados.

Registre:

- Quantidade de spritesheets.
- Dimensões.
- Memória estimada das texturas.
- Quantidade de frames.
- Custo adicional de entidades.
- Eventuais gargalos observados.

Não implemente otimizações complexas sem evidência.

---

# 21. DOCUMENTAÇÃO

Crie um documento de integração, por exemplo:

`docs/PLAYER_VISUAL_ANIMATION_V1_3.md`

Inclua:

- Objetivo.
- Arquitetura.
- Componentes.
- Sistemas.
- Estados visuais.
- Prioridades.
- Assets.
- Metadados.
- FPS.
- Sincronização.
- Pivôs.
- Direção.
- Fallbacks.
- Save/load.
- Testes.
- Performance.
- Limitações.
- Próximas animações necessárias.

Atualize documentos existentes somente quando necessário.

Não reescreva toda a arquitetura do projeto.

---

# 22. CRITÉRIOS DE ACEITAÇÃO

A V1.3 só poderá ser declarada concluída se:

- [ ] Backup verificável realizado.
- [ ] Arquivos Blender anteriores preservados.
- [ ] PlayerVisual separado do gameplay.
- [ ] Assets carregados corretamente.
- [ ] Idle funcionando.
- [ ] Run funcionando.
- [ ] Basic Attack funcionando.
- [ ] Dodge funcionando.
- [ ] Direção correta.
- [ ] Prioridades corretas.
- [ ] Duração do ataque preservada.
- [ ] Duração da esquiva preservada.
- [ ] Velocidade do jogador preservada.
- [ ] Câmera preservada.
- [ ] Pivôs corretos.
- [ ] Sem cortes de sprites.
- [ ] Sem saltos graves entre animações.
- [ ] Sem duplicação de entidades visuais.
- [ ] Save/load preservado.
- [ ] Testes automatizados passando.
- [ ] Regressões de gameplay verificadas.
- [ ] Inspeção visual realizada ou limitação declarada.
- [ ] Documentação atualizada.
- [ ] Git auditado.
- [ ] Nenhum commit ou push realizado.

Se houver falhas, apresente-as claramente.

Não declare aprovação integral quando houver critérios importantes pendentes.

---

# 23. RELATÓRIO FINAL OBRIGATÓRIO

Apresente o resultado nesta estrutura:

**A. Estado inicial**

Branch, HEAD e alterações preexistentes.

**B. Backup**

Arquivos copiados, destino, hashes e verificação.

**C. Arquitetura**

Componentes, sistemas e integração com o jogador.

**D. Assets**

Spritesheets, metadados e organização.

**E. Idle**

Implementação e resultados.

**F. Run**

Velocidade, ciclo, sincronização e deslizamento residual.

**G. Basic Attack**

Duração, impacto inicial e interrupções.

**H. Dodge**

Duração, deslocamento visual e interrupções.

**I. Estados e prioridades**

Regras implementadas e fallbacks.

**J. Direção e pivôs**

Espelhamento, offsets e alinhamento.

**K. Save/load**

Comportamento após carregamento e respawn.

**L. Performance**

Memória, frames e custo estimado.

**M. Testes automatizados**

Comandos e resultados.

**N. Testes de gameplay**

Regressões e comparações.

**O. Validação visual**

O que foi realmente observado.

**P. Arquivos criados e modificados**

Caminhos completos e justificativas.

**Q. Git**

Branch, HEAD, staged, modificados, untracked e hashes protegidos.

**R. Pendências**

Problemas restantes e riscos.

**S. Recomendações**

Próximas etapas artísticas e técnicas.

---

# 24. CONDIÇÕES DE PARADA

Interrompa a execução e peça autorização se:

- A integração exigir alterar física ou combate.
- For necessário modificar saves.
- For necessário alterar a câmera.
- O contrato de sprites precisar ser quebrado.
- O rig v007 apresentar uma regressão grave.
- A arquitetura exigir uma mudança estrutural significativa não prevista.
- O backup obrigatório não puder ser realizado com segurança.
- Houver risco de perda de arquivos existentes.
- For necessário instalar dependências.
- Uma falha impedir a validação mínima e não houver solução segura dentro do escopo.

Não improvise mudanças fora do escopo.

---

# 25. REGRA FINAL

Esta autorização permite implementar e testar a V1.3.

Ela não autoriza:

- Commits.
- Push.
- Alterações de gameplay.
- Alterações na câmera.
- Mudanças no formato dos saves.
- Reestruturação completa do projeto.
- Implementação de todas as animações futuras.

Trabalhe de forma incremental.

Preserve o projeto.

Valide cada etapa.

Documente os resultados reais.

**Ao terminar, apresente o relatório completo e aguarde minha avaliação antes de iniciar a V1.4.**

O objetivo final desta etapa é que o protagonista v007 apareça no Dark Castle RPG, com Idle, Run, Basic Attack e Dodge funcionando de forma consistente e sincronizada com o gameplay existente.