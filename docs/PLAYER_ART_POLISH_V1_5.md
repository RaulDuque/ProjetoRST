# Polimento artístico, VFX e segurança do pipeline (V1.5)

> Fase de apresentação. Nenhuma regra mudou: dano, hitboxes, hurtboxes,
> colisões, gravidade, movimento, invulnerabilidade, cooldowns, custos,
> progressão, comandos, save/load, durações e câmera estão idênticos. Base:
> [`PLAYER_VISUAL_ANIMATION_V1_3.md`](PLAYER_VISUAL_ANIMATION_V1_3.md) e
> [`PLAYER_ANIMATION_EXPANSION_V1_4.md`](PLAYER_ANIMATION_EXPANSION_V1_4.md).
> Modelo: v007, rig sem alteração. **Nenhuma decisão artística está aprovada:**
> as escolhas abaixo são provisórias e aguardam avaliação.

## 1. Estado inicial (auditado em 2026-11-13)

| Item | Valor |
|---|---|
| Branch / HEAD | `art/v1.2-protagonist` / `204f965` |
| Índice | só `art-src/player/inspiration/protagonist_concept_v001.png` (blob `dfeb9ea…`), preservado |
| Alterações V1.2-D a V1.4 | não commitadas (16 modificados, ~60 não rastreados), preservadas |
| Linha de base | `cargo fmt` OK; `cargo test` **158/158**; V1.4 informou autoplay 23/23 e visual_check 30/30 |
| Assets V1.4 | `player.ron` `fd7d9f3c…`, `player_locomotion.png` `b0d0e351…`, `player_combat.png` `4450e359…` |

Hashes de `Md-usario/`, `saves/`, `.blend`/`.blend1` e `.cargo/config.toml`
foram registrados antes e conferidos no fim: nenhum mudou.

**Como o pipeline funcionava (V1.4):**
- **Masters:** Blender (`bpy/build_game_sprites_v1_x.py`) → `renders/`.
- **Pós-processamento:** `post/game_sprites_v1_x.py` → atlas + manifesto
  escritos **direto** em `assets/`. O destino padrão era `assets/`.
- **Carregamento:** um sistema de Startup lê o RON e cria os handles (`PlayerSpriteSet`).
- **Identidade da habilidade:** chega por `SkillStarted` + campo `animation` dos RON.
- **Sinais:**
  - dano: `ActionState` = `Hurt`;
  - morte: `Dead`;
  - revive: sai de `Dead`;
  - física: `Grounded` e estados `Jumping`/`Falling`.

## 2. Segurança do pipeline

**Risco confirmado:** `post/game_sprites_v1_3.py` gravava os atlas e o
manifesto em `assets/` com só 4 clipes. O `_v1_4.py` também gravava
mesmo quando a sua validação acusava falhas. Os scripts não foram executados
sobre os assets reais para testar o risco.

| Gerador | Classificação |
|---|---|
| `post/game_sprites_v1_3.py` | Histórico, potencialmente destrutivo (4 clipes) |
| `post/game_sprites_v1_4.py` | Histórico, compatível com o contrato, destrutivo por gravar mesmo com falhas |
| `post/game_sprites_v1_5.py` | Atual |
| `bpy/build_game_sprites_v1_x.py` | Só escrevem `renders/` e um `.blend` novo; recusam sobrescrever `.blend` anteriores |
| Demais `post/*` (folhas, comparações) | Só `comparisons/` e `renders/` |

## 3. Proteções implementadas

`art-src/player/scripts/post/publish_guard.py` é o **único** escritor de
`assets/sprites/player/` e `assets/animations/player.ron`.

```text
script de pós → staging (estrutura de assets/)        destino em assets/ → recusado
publish_guard check   → valida sem escrever
publish_guard publish → valida PNG, manifesto, pivôs, durações, quadros não vazios
                      → versão do gerador ≥ publicada; todos os clipes publicados presentes
                      → arquivos publicados iguais ao lock (senão: alteração fora do guarda)
                      → hashes: igual ao publicado → "unchanged" (idempotente)
                      → exige --confirm <state id atual>
                      → backup em renders/publish_backups/<data>_<state>/
                      → journal → escrita por temporário + os.replace → lock
                      → revalida; qualquer falha → rollback pelo backup
publish_guard recover → restaura uma publicação interrompida (journal pendente)
```

- **Lock:** `assets/animations/player.publish.json`.
  - Guarda a versão do pipeline, o gerador, os clipes e o SHA-256 de cada arquivo.
  - Foi criado para a V1.4 (`init-lock`, estado `0c947b7a16be`) antes da publicação da V1.5.
- **Scripts antigos:** o V1.3 e o V1.4 passaram a gravar só em `renders/v1_x_game_sprites/stage/`.
  - Recusam `--assets-dir` dentro de `assets/` antes de ler ou escrever qualquer coisa.
  - O guarda recusa o staging deles: versão 1.3/1.4 < 1.5 e, na V1.3, clipes faltando.
- **Sem sobrescrita genérica:** `--confirm` só aceita o state id do que está
  publicado. Nenhuma opção pula uma validação.

**Publicação da V1.5:** `0c947b7a16be` → `c670bdc45f61`. O primeiro `publish`
sem `--confirm` foi recusado. A execução repetida respondeu `unchanged`. O
backup confere byte a byte com os hashes da V1.4.

## 4. Comparações artísticas

Pasta: `art-src/player/comparisons/v1_5_art_polish/`. Cada folha existe em 1:1
e ×3; os GIFs estão em `previews/`. As variantes têm render id próprio
(`hurt_a`, `hurt_b`, `fall_v15`, `death_lying`, `attack_basic_air`) em
`bpy/game_sprites_v1_5/clips_v1_5.py`. A escolha do que vai para o jogo é do
pós-processamento (`--hurt`, `--fall`).

Validação do Blender: pior severidade "Baixo" em todos os clipes (sem
interseções médias ou altas) e sabre acima do chão. No pós-processamento:
sem toques na borda da célula, sola na linha do pivô e durações do jogo.

## 5. Hurt (`hurt_compare_1x.png`, `_3x.png`)

| | V1.4 | V1.5 A (no jogo) | V1.5 B |
|---|---|---|---|
| Ideia | Tronco ~10° para trás | Recuo de tronco e quadril, braço livre lançado, cabeça com atraso, capa abre e volta | Torção: o ombro da espada é jogado para trás |
| Leitura em 1:1 | Sutil | Clara: silhueta inclinada e assimétrica nos quadros 0–3 | Clara, mas mostra o peito de frente e lança a espada para trás, longe do corpo |
| Continuidade | — | Espada fica baixa, volta à base sem salto | Espada atravessa a silhueta ao voltar |

- **Comuns às três:** 9 quadros, 0,35 s (`FitToAction`), impacto no quadro 0,
  sem knockback visual. O recuo físico continua do gameplay.
- **Escolha provisória:** A. Não é a de maior amplitude; foi escolhida pela
  legibilidade e pela continuidade.
- **Primeira iteração:** a capa entrava na casaca (interseção alta) e foi corrigida
  abrindo a capa para fora.

## 6. Fall (`fall_compare_*.png`)

| | V1.4 | V1.5 (no jogo) |
|---|---|---|
| Silhueta | Quase em pé | Pernas abertas (a da frente busca o chão, a de trás recolhida), braço da espada aberto para equilíbrio, casaca e capa levantadas |
| Contra o `jump_rise` | Pouco diferente | Diferente: o pulo recolhe as pernas e leva o braço para trás |

- **Timing:** loop de 4 quadros a 12 fps, sem estado novo. A física move o corpo.
- **Primeira versão:** a espada para a frente saía 18–25 texels da célula; a
  validação acusou. O braço e a lâmina foram baixados.
- **Ataque no ar:** a queda nunca é forçada durante um ataque aéreo (o estado é `Attacking`, §10).

## 7. Death (`death_compare_*.png`)

| Critério | A: ajoelhada (V1.4, **continua no jogo**) | B: deitada (V1.5, experimental) |
|---|---|---|
| Silhueta | Compacta, legível como "caiu" | Horizontal, lida como "morto" sem ambiguidade |
| Contato com o chão | Joelho e pé | Peito, coxas e botas; pelve a 0,16 m (espessura do corpo) |
| Naturalidade | Boa; termina numa pose mais estática | Desequilíbrio, queda, mãos no chão, assentamento; os quadros 6–8 mostram as mãos amparando, o que dá um leve "flutuar" no meio da queda |
| Legibilidade 1:1 | Boa | Boa; o corpo deitado ocupa ~60 u de largura |
| Espada | No chão à frente | Ao longo do chão, ao lado do corpo (0,02 m acima) |
| Casaca | Cai naturalmente | Posada: abas da frente sob as coxas (sem levantar o corpo, como na tentativa da V1.4), abas de trás e capa sobre as pernas, com um quadro "no ar" antes de assentar |
| Revive | Sai da pose ajoelhada | Sairia do chão: mais distante da pose de pé (o respawn teleporta, então não há transição de pose) |
| Clipping | Nenhum relevante | Nenhum médio ou alto; precisa da célula de combate 128×96 (não cabe em 64 u) |

- **Técnica:** sem simulação de tecido. Até 0,333 s os joelhos usam IK; depois
  a pelve gira até 90° e as pernas passam a FK. As poses de braço, mão e casaca
  foram escolhidas por busca numérica, sem interseções e com o sabre acima do chão.
- **Recomendação:** B é mais legível como morte. Se aprovada, entra como `death`
  no grupo de combate. Isso custa 12 quadros (~2,4 MB na GPU), e o quadro 7
  merece um ajuste manual.
- **Por enquanto:** A continua no jogo e nada foi substituído.

## 8. VFX

**Arquitetura:** `src/player/visual/vfx/`.
- `rules.rs` (puro): clipe por chave de dados, escala de tempo, quadro, interrupção e revive.
- `systems.rs` (Bevy): os quatro sistemas, encadeados em `PlayerVisualSystems`
  depois de `animate_player_visual`.
- `mod.rs`: o componente `PlayerVfx`.

**Como funciona:**
- **Entidade:** cada efeito é um filho-sprite do jogador. Ele acompanha o
  avanço do Crescent Slash e o respawn, e é removido junto com o jogador.
- **Quadros:** clipes `vfx_<chave>` do atlas `effects` no mesmo manifesto,
  desenhados por `post/vfx_v1_5.py` (PIL, determinístico, supersampling 4×).
- **Dados:** a chave é o campo `vfx` que os RON já tinham (`arc_white`,
  `arc_gold`, `light_motes`, `fire_bolt`). Uma habilidade nova com chave existente não pede código.
- **Duração:** o efeito dura a ação real (`time_scale` = tempo restante da ação ÷ duração do clipe), inclusive com `attack_speed`.
- **Duplicação:** só a mensagem dispara o efeito, nunca a mudança de estado.
  O mesmo clipe no mesmo jogador substitui o anterior, e há no máximo um efeito
  por jogador e clipe por frame.
- **Remoção:** no fim do clipe. Os efeitos de técnica somem também quando o
  jogador sai de `Casting` (dano, morte, load). Nada fica na tela.
- **Pausa:** os efeitos param junto com a animação.
- **Isolamento:** nenhum sistema de gameplay lê `PlayerVfx`. Os efeitos não têm
  `Hitbox`, colisão nem velocidade e não escrevem em componentes de outros.

| Efeito | Sinal | Sincronização |
|---|---|---|
| Crescent Slash `vfx_arc_white` (8 q., 0,40 s) | `SkillStarted` | Quadro 0 no mesmo frame da hitbox; o arco branco-prateado vai do alto-frente ao baixo-frente, acompanhando a lâmina, e esmaece. Some com o fim da ação |
| Rising Edge `vfx_arc_gold` (9 q., 0,45 s) | `SkillStarted` | Arco dourado subindo da frente-baixo até acima da cabeça; mais estreito, sentido oposto |
| Mending Light `vfx_light_motes` (12 q., 0,60 s) | `SkillStarted` de uma magia `Heal` | A cura é **instantânea**, no frame do `SkillStarted` (`cast_spells`). O anel dos quadros 0–2 marca esse momento; depois sobem partículas discretas e há um brilho sobre a mão |
| Guarda `vfx_guard_spark` (5 q., 0,15 s) | `DamageDealt` (acerto, não erro) com o jogador em `Guarding` e virado para o golpe | Explosão curta azulada à frente do peito |
| Revive `vfx_revive` (10 q., 0,5 s) | Estado sai de `Dead` | Coluna de luz pálida e anel no chão |

- **Direção:** `flip_x` e o pivô espelham pela direção no momento do disparo (`anchor_for`).
- **Regra da guarda:** "golpe de frente" virou uma única função pura,
  `Facing::faces_hit`, usada pelo `receive_damage` (refatoração sem mudança de
  comportamento) e pelo VFX. Não há inferência por proximidade de inimigos.
- **Ember Bolt (revisão):**
  - O projétil existente é um quadrado laranja de 18×18 u, com bom contraste no fundo escuro.
  - Nasce 25 u à frente e 6 u acima do centro, que é a altura da mão
    estendida do `cast_forward` (o quadro 0 já tem a mão à frente), e segue a direção.
  - A V1.5 não o substitui: a chave `fire_bolt` não tem clipe.
  - **Proposta localizada (não implementada):** um sprite redondo de brasa com
    halo, filho visual do projétil, mantendo o `Hitbox` e o tamanho.

**Achado da revisão em jogo:** as hitboxes de técnica e do ataque básico
desenham um retângulo translúcido (`TECHNIQUE_COLOR`/`SLASH_COLOR` em
`combat/attacks.rs`), que cobre parte dos arcos. Não foi tocado, porque o
briefing proíbe mexer em hitboxes. **Recomendação para a V1.6:** tornar esse
retângulo só de depuração.

## 9. Variantes

| Variante | Sinal | Resultado |
|---|---|---|
| Ataque aéreo | `Attacking` + `Grounded.on_ground == false` (só leitura) | **Implementado:** `attack_basic_air`. Mesmo tronco, braço e lâmina do ataque do chão; pernas aéreas (joelho da frente alto, perna de trás estendida); 7 quadros, 0,28 s. Se o jogador aterrissa durante o ataque, o clipe troca para o do chão **no mesmo progresso** (a duração da ação é preservada). Rising Edge no ar continua `slash_rising`; não há outro ataque aéreo |
| Impacto na guarda | Confiável (§8) | Só o VFX. A pose de guarda não muda; uma pose `guard_impact` fica pendente |
| Respawn | Confiável (saída de `Dead`) | VFX de revive. A sequência visual é morte → luz → `fall` → `land`, porque o respawn põe o jogador no ar. O load (F9) a partir da morte mostra a mesma luz. O teste confirma um único sprite |
| Virada | Ver §10 | Não implementada |

## 10. Espelhamento (avaliação, nada alterado)

| Opção | Custo | Legibilidade | Coerência |
|---|---|---|---|
| Manter o espelhamento atual | 0 | Boa; a espada troca de mão | Inconsistência conhecida em 2D |
| Renders próprios para a esquerda | ~2× quadros (≈ +19 MB na GPU), 2× tempo de render, manutenção dupla | Igual | Total |
| Simetrizar a arte (capa nas costas, espada centralizada) | Muda o design aprovado do personagem | Boa | Total, mas descaracteriza |
| Variantes só nos clipes onde a mão aparece muito (ataques, guarda) | Médio (~60 quadros) | Melhor onde importa | Parcial: há troca de mão ao alternar clipes |

**Recomendação:** manter o espelhamento até a D2. Se a D2 escolher pixel art
(quadros menores), a última opção fica barata. O controlador já resolve o
pivô espelhado.

## 11. Decisões provisórias mantidas

- **Mantidos:** D2 = acabamento C; D3 = câmera 1:1; espada ×1,7; combate a 24 fps; locomoção como nos clipes.
- **Não alterados:** zoom, filtro global, resolução lógica.
- **Efeitos:** usam densidade 1 (1 texel por pixel de tela na câmera 1:1).
- **Para uma decisão futura sobre a D2:** os efeitos já estão na resolução de tela.
  No pixel art só os personagens mudariam, e os arcos suaves contrastariam com
  bordas duras. Por isso a escolha do acabamento deve ser feita olhando os GIFs
  da demonstração em 1:1.

## 12. Demonstração animada

Pasta: `art-src/player/comparisons/v1_5_art_polish/demo/`

| Arquivo | Origem | Conteúdo |
|---|---|---|
| `seq1_bevy_1x.gif`, `seq1_bevy_2x_inspecao.gif` | **Captura do Bevy** (`cargo run --example player_demo`) | idle → run → jump → fall → land → ataque → esquiva → hurt → death → revive; 159 quadros, 7,6 s |
| `seq2_bevy_1x.gif`, `seq2_bevy_2x_inspecao.gif` | **Captura do Bevy** | Crescent Slash → Rising Edge → Ember Bolt → Mending Light → ataque aéreo → guarda com golpe bloqueado; 179 quadros, 8,6 s |
| `seq*_bevy_stills_1x.png` | Captura do Bevy | Primeiro quadro de cada troca de clipe, 1:1 |
| `offline_{hurt,fall}_v1_4_vs_v1_5_{1x,3x}.gif` | **Offline**: sprites dos atlas publicados (backup V1.4 × V1.5), com o rótulo na imagem | Comparação lado a lado no tempo do clipe |

**Método de captura:**
- passo fixo de 16 ms;
- screenshot da janela a cada 3 frames (48 ms);
- recorte de 360×220 u em torno do jogador, calculado pela posição e pela câmera do log (`frames.log`);
- GIF montado sem ffmpeg, via PIL (`post/demo_v1_5.py`).

**Atalhos de teste do roteiro (nunca usados pelo jogo):**
- todas as habilidades concedidas;
- dano e morte forçados na máquina de estados;
- golpe na guarda enviado como `DamageDealt` de frente, aplicado pelo `receive_damage` do jogo;
- o revive é a tecla R real.

**Inspeção visual:** os stills e quadros-chave (faísca, morte e revive) foram
abertos e conferidos. O número de dano exibido na guarda ("8") é o valor da
mensagem, mostrado pela UI existente. O HP perdido segue a redução da guarda.

## 13. Testes

| Verificação | V1.4 | V1.5 |
|---|---|---|
| `cargo fmt --all -- --check` | OK | OK |
| `cargo clippy --all-targets -- -D warnings` | 0 | 0 |
| `cargo test` | 158 | **175**, 0 falhas |
| `cargo check` | OK | OK |
| Python `unittest` (guarda) | — | **17/17** |
| `cargo run --example autoplay` | 23/23 | **23/23** (custos MP 8/15 e Hearts 0 iguais aos dados, saves isolados, saves reais intactos; o log da V1.4 não estava guardado para comparar linha a linha) |
| `cargo run --example visual_check` | 30/30 | **PASS 30/30** (`fall`/`hurt` novos nas capturas 17 e 27–28) |
| `cargo run --example player_demo` | — | **PASS** (todo clipe e efeito do roteiro apareceu) |

**Testes novos:**
- **`vfx/rules.rs` (4):** chave → clipe; escala de tempo e quadros com `attack_speed`; interrupção; revive.
- **`selection.rs` (+1):** variante aérea só em `Attacking`.
- **`tests/player_vfx_tests.rs` (11):**
  - um efeito por chave de dados e nenhum para o projétil;
  - o arco começa com o golpe e dura a técnica;
  - Rising e Mending com efeito e duração próprios;
  - Ember sem efeito;
  - direção espelhada;
  - interrupção da técnica, mas não da magia;
  - faísca só de frente e uma por frame;
  - revive com um único sprite;
  - ataque aéreo e saída para a queda;
  - efeito e ação com o mesmo número de frames;
  - **gameplay bit a bit idêntico com e sem efeitos** (posição, estado, HP, MP e
    contagem de hitboxes em 260 frames com as quatro habilidades).
- **`tests/data_tests.rs` (+1):** lock × manifesto.
- **Python (17):** os 12 cenários do briefing mais quadro vazio, alteração por
  fora, confirmação errada, variante aérea e assets reais × lock.

**Testes alterados (sem enfraquecer):**
- `air_attack_shows_the_attack` esperava `attack_basic`. Agora espera
  `attack_basic_air`, que é a nova regra; a asserção de prioridade continua.
- Nos testes de duração dos VFX, o esperado passou de `round` para `ceil`
  (0,45 s cobre 29 frames de 16 ms). Um teste novo exige igualdade exata entre
  os frames do efeito e os da ação.

## 14. Desempenho

| | V1.4 | V1.5 |
|---|---|---|
| Texturas | 2 | 3 (`effects` novo) |
| Quadros | 115 | 122 de personagem + 44 de efeito = 166 |
| GPU (RGBA) | 15,5 MB | 4,31 + 12,58 + 2,43 = **19,3 MB** (~39 MB com a cópia em RAM do Bevy) |
| PNG em disco | 1,0 MB | 1,17 MB |
| Entidades de VFX | — | No máximo 1 por clipe e jogador; na prática 0–2 vivas, de 0,15 a 0,6 s |
| Criação/destruição | — | 1 spawn por habilidade, golpe bloqueado ou revive. Nenhum por frame |
| Custo por frame | 2 sistemas | +4 sistemas: leitura de mensagens (normalmente vazias), um `HashMap` de estado anterior por jogador e iteração sobre 0–2 efeitos. Sem carregamento, handle novo ou processamento de imagem |
| Carregamento | Uma vez no Startup | Igual; um PNG a mais (87 kB) |

**Alocação:** uma `String` por efeito criado e um `Vec` das mensagens do frame (vazio na maior parte do tempo).

## 15. Riscos

- **Retângulos das hitboxes:** cobrem os arcos (§8).
- **Lock e hashes:** o lock depende de o guarda ser usado. Uma edição manual em
  `assets/` é detectada pelo teste Python e pelo `publish`, mas o teste Rust
  só confere a lista de clipes e de arquivos (não há dependência de hash).
- **Morte deitada:** o quadro 7 mostra o corpo amparado pelas mãos e
  levemente no ar. Só é aceitável no ritmo de 12 fps.
- **Ataque aéreo:** quando o jogador aterrissa no meio do ataque, a troca para o
  clipe do chão é instantânea (mesmo quadro, pernas diferentes).
- **Arcos:** foram desenhados para o golpe da arte V1.4. Uma mudança de pose
  exige revisar `vfx_v1_5.py`.

## 16. Pendências

**Aprovação artística:**
- Hurt A × B × V1.4.
- Fall V1.5.
- Morte ajoelhada × deitada (recomendação: deitada, com ajuste do quadro 7).
- O tom dos efeitos.

**Para decidir:**
- Retângulos das hitboxes.
- Sprite do Ember Bolt (proposta em §8).
- Pose `guard_impact`.

**Em aberto:** D2, D3 e a taxa definitiva; renders para a esquerda (§10).

**Git:** nada foi commitado. A preparação está em §17.

## 17. Próximas etapas (V1.6, não iniciada)

1. **Decisões artísticas** desta fase; republicação pelo guarda (`--hurt`, `--fall`, morte).
2. **Commits atômicos:**
   - guarda + lock;
   - VFX (código + atlas);
   - clipes V1.5;
   - demonstração;
   - documentação.
3. **Hitboxes de depuração** e sprite do Ember Bolt.
4. **Revisão final** da D2 com os GIFs 1:1.
