# Auditoria técnica — Etapa 2

Auditoria da fundação do Dark Castle RPG feita em 2026-11-11, conforme
`Md-usario/E2-Auditoria-Tecnica.md`. Base: commit `9dcdfcc` na branch `master`.
Nenhum código, configuração ou documento existente foi alterado; este arquivo é
o único criado.

**Observação:** o briefing termina truncado em "Para cada problema," (seção 12).
Por isso cada problema traz local, descrição, cenário de falha e recomendação,
que é o formato usado nas seções anteriores do briefing.

## Método

- O código, os testes, os dados RON, os documentos e a configuração do Claude Code
  foram lidos por quatro revisões somente leitura em paralelo:
  1. arquitetura, ECS, mundo e save;
  2. jogador, combate, comandos, skills e magia;
  3. progressão, inventário, inimigos e dados;
  4. testes, Claude Code e documentação.
- Todo achado CRÍTICO, ALTO e a maioria dos MÉDIOS foram reconferidos direto no
  código antes da classificação. Itens não confirmados estão marcados como *plausível*.
- A validação reaproveitou o `target/` existente: sem diretório temporário e sem
  recompilar o Bevy.

## Validação técnica

| Comando | Resultado |
|---|---|
| `cargo fmt --check` | ok (exit 0) |
| `cargo clippy --all-targets -j 4` | 0 avisos, 0 erros |
| `cargo test -j 4` | **85 aprovados**, 0 falhas (lib 23, smoke 3, attributes 7, combat 9, command_parser 24, data 3, inventory 5, progression 5, save 6) |
| `cargo check -j 4` | ok |
| `cargo run --example autoplay` | exit 0, 7 screenshots, sessão Wayland com RTX 4050 |

O que o autoplay mostrou:

- `Executed crescent_slash`: o golpe **não acertou**, porque o inimigo estava fora de alcance (screenshot 02).
- `Executed ember_bolt` acertou: `Hit 27 Fire`.
- Os dois ataques básicos aparecem como `Invalid`, que é o nome usado para "sem comando".
- `NotLearned rising_edge` veio junto de um `Hit 21`, ou seja, disparou um ataque básico (ver M8).
- `Game saved` foi gravado no `saves/slot1.ron` real (ver M4).

Limitação: o autoplay não faz asserções; os resultados acima vieram da leitura do log.

## Resumo

| Gravidade | Qtde | Destaques |
|---|---|---|
| CRÍTICO | 1 | As regras do Claude Code permitem repetir o travamento do PC (build do Bevy em `/tmp`, que é tmpfs) |
| ALTO | 6 | Skill executa quando o lock falha; dano não interrompe ações; dano duplo dentro dos i-frames; validação de RON mínima; autoplay não verifica nada; permissões permitem sobrescrever arquivos |
| MÉDIO | 19 | Save/load (sala, entidades vivas, fsync, backup), comandos ao virar, guarda, testes ausentes, documentação acima do código |
| BAIXO | 20+ | Ordem ambígua no mesmo set, constantes espalhadas, comentários desatualizados |

**Conclusão:** a fundação é sólida. Módulos puros, ordem do frame, pausa, fórmulas
e formato de save estão corretos e bem testados. Os problemas se concentram em
**três pontos de integração** (início de skill, recebimento de dano e load do save),
na **validação dos dados** e na **segurança do ambiente de trabalho do Claude Code**.
Nenhum exige reescrever um módulo; todos se resolvem com mudanças localizadas.

---

## CRÍTICO

### C1. A configuração do Claude Code permite repetir o travamento da máquina
- **Onde:** `.claude/rules/commits.md:6`, `.claude/skills/atomic-commit/SKILL.md:19-21`, `.claude/settings.json`.
- **Descrição:**
  - As regras exigem que "cada commit compile e passe nos testes sozinho", mas não dizem como verificar isso.
  - Nada proíbe `/tmp` (tmpfs de 7,3 GB que fica na RAM), `CARGO_TARGET_DIR` separado ou builds paralelos.
  - Sem limite de `-j`, o cargo usa 12 jobs em 14 GB de RAM.
  - A única orientação está em `docs/RESUMO_PROJETO.md`, que não é carregado automaticamente.
  - `cargo test:*` e `cargo build:*` estão liberados sem confirmação, inclusive com `--target-dir /tmp/...`.
- **Cenário:** esta sessão e a anterior travaram exatamente assim (exit 137 e cota do `/tmp` esgotada).
- **Recomendação:**
  - Regra explícita em `CLAUDE.md`/`commits.md`: verificar cada commit só com worktree dentro de `target/`, reaproveitando o `target/` do projeto; nunca em `/tmp`.
  - Limitar os jobs com `CARGO_BUILD_JOBS=4` no `env` do `settings.json` ou com `.cargo/config.toml`.
  - Avisar que `--profile dist` (LTO fat) exige muita RAM.

## ALTO

### A1. O efeito da skill acontece mesmo quando o início da skill falha
- **Onde:** `src/skills/systems.rs:52-64`, `src/combat/attacks.rs:95-127`, `src/magic/casting.rs:35-97`.
- **Descrição:**
  - O comando é validado em `Commands` com o estado do frame anterior.
  - Em `Actions`, `guard_and_dodge` roda antes de `begin_skill`.
  - Se `start_action(Casting)` falha, `begin_skill` só registra um `warn!`, mas o MP/Hearts já foi cobrado, o cooldown começou e o lunge é aplicado.
  - `melee_techniques` e `cast_spells` leem `SkillActivated` por conta própria e geram hitbox ou projétil de qualquer jeito.
- **Cenário:** com ↓↘→ no buffer, apertar Esquiva e Magia no mesmo frame lança o Ember Bolt durante a esquiva invulnerável. O HUD mostra "Executed".
- **Recomendação:**
  - Validar o lock antes de cobrar.
  - Emitir uma mensagem `SkillStarted` só quando der certo e fazer os executores lerem essa mensagem.
  - Criar um teste ECS de esquiva + skill no mesmo frame.

### A2. Dano ou morte não interrompe técnica nem magia
- **Onde:** `src/combat/hitboxes.rs:102-116` (`follow_owners` só remove a hitbox se o dono deixar de existir), `src/player/systems.rs:157-164`, `src/magic/casting.rs:72-96`.
- **Descrição:**
  - Quando o jogador entra em `Hurt` ou `Dead`, a hitbox com `AttachedToOwner` continua viva até o `lifetime` acabar e segue o jogador.
  - Magias aplicam o efeito no frame da ativação: `Casting` é só uma recuperação, não um preparo.
- **Contradiz:** `docs/MAGIC_SYSTEM.md:25,29` ("Tempo de conjuração", "dano coloca o personagem em `Hurt`, encerrando `Casting`").
- **Cenário:** atingido no meio do Crescent Slash, o jogador continua acertando inimigos enquanto é empurrado, e até depois de morrer. O Mending Light cura antes de qualquer interrupção ser possível.
- **Recomendação:**
  - Remover as hitboxes do dono ao entrar em `Hurt`/`Dead`.
  - Decidir se magia terá preparo e alinhar a documentação.

### A3. Vários danos no mesmo frame ignoram a invulnerabilidade
- **Onde:** `src/player/systems.rs:142-165`; `contact_damage` (`enemies/behavior.rs`) e `detect_hits` (`combat/hitboxes.rs:165`) no set `Combat`.
- **Descrição:**
  - A checagem de `Invulnerable` acontece em `Combat`, antes de qualquer dano ser aplicado.
  - `receive_damage` aplica todas as mensagens do frame e só redefine `remaining`, sem pular os acertos seguintes.
- **Cenário:** contato com um inimigo mais o golpe de outro (ou do mesmo) no mesmo frame aplica dano duas vezes, apesar dos 0,9 s de i-frames.
- **Recomendação:** no laço, ignorar mensagens depois do primeiro dano aplicado enquanto `invulnerable.remaining > 0`.

### A4. A validação dos dados RON é mínima
- **Onde:** `src/core/resources.rs:129-183`, `src/player/attributes.rs:293,297,307`.
- **Descrição:** `validate()` só confere a sala inicial, os inimigos dos spawns e os itens iniciais. Ficam de fora:
  - **IDs duplicados de inimigos e salas:** são sobrescritos em silêncio (`enemies.insert`, `rooms.insert`). Skills e itens detectam duplicata.
  - **Valores inválidos aceitos:** `max_hp: 0` gera um inimigo **imortal**, porque `receive_damage` pula alvo com `health.is_empty()`, e ele continua causando dano por contato. Também passam velocidade, tamanho, cooldown e `respawn_time` negativos, `chance` acima de 1 e `leash_range < aggro_range`.
  - **`formulas` de `character.ron` não validadas:** `max_critical_damage < 1.0` faz `f32::clamp` dar **pânico** ao criar o jogador, já com a janela aberta.
  - **Referências não conferidas:** `drops→items` vira só `warn!("invalid drop")` em runtime e o item some; `requirements.items` e `progress_flags` não são verificados; o slot de `start.equipped` não é checado.
  - **Categoria × efeito:** uma técnica com efeito `Projectile` cobra o custo e não faz nada.
- **Recomendação:** concentrar tudo em `GameData::validate` com erros tipados (arquivo e id) e testar cada caminho de rejeição.

### A5. O autoplay não verifica nada, e a skill de validação o trata como prova
- **Onde:** `examples/autoplay.rs:113-118`, `.claude/skills/test-and-validate/SKILL.md:17-18`.
- **Descrição:**
  - O autoplay sai sempre com `AppExit::Success`, sem nenhuma asserção.
  - Nesta execução, o Crescent Slash errou o alvo e o resultado foi exit 0 do mesmo jeito.
  - A skill manda rodar `timeout 15 cargo run` logo depois de `cargo check`, que não gera executável. Se a compilação passar de 15 s, o código 124 é contado como "o jogo iniciou".
  - O texto da skill espera `Executed crescent_slash`, mas o log real é `Executed { id: "crescent_slash" }`.
- **Recomendação:**
  - Acumular os resultados num recurso e sair com `AppExit::error()` se faltar algum resultado esperado (comandos, inimigo derrotado, save que decodifica).
  - Na skill, rodar `cargo build` antes do `timeout`.

### A6. As permissões liberadas permitem sobrescrever arquivos sem confirmação
- **Onde:** `.claude/settings.json:5-24`.
- **Descrição:**
  - `git diff:*`, `git log:*` e `git show:*` aceitam `--output=<arquivo>`, que sobrescreve qualquer arquivo.
  - `cargo fmt:*` reformata o crate inteiro, inclusive arquivos do usuário fora da tarefa.
  - `cargo run:*` executa o autoplay, que grava no save real.
  - A lista `deny` só cobre `rm -rf` literal. Escapam `rm -r`, `rm -fr`, `find -delete`, `git checkout .`, `git restore`, `git stash drop`, `git branch -D`, `git reset -q --hard` e `git push origin +master`.
  - `Md-usario/` não tem proteção (o `rm -r` usado nesta sessão passou pela regra atual).
- **Recomendação:**
  - Negar `--output`.
  - Trocar a permissão para `cargo fmt --check:*`.
  - Ampliar a lista `deny`.
  - Negar `Edit`/`Write` em `Md-usario/**`.

## MÉDIO

| # | Onde | Problema e cenário | Recomendação |
|---|---|---|---|
| M1 | `src/save/systems.rs:107-180` | O F9 não recarrega a sala (só confere se ela existe) e não limpa hitboxes, projéteis nem inimigos. Resultado: inimigos com HP atual e EXP do save, e projéteis voando após o load. Com várias salas, o jogador cairia na geometria errada | Uma única operação `LoadRoom(id)` no módulo world, usada no início, no load e nas transições |
| M2 | `src/save/save_manager.rs:50-55`, `docs/SAVE_SYSTEM.md` | Escrita sem `sync_all`/fsync. O `.bak` é sobrescrito mesmo quando o principal está corrompido. `load_backup` nunca é usado no jogo. A doc promete "nunca corrompe" e "backup oferecido" | Fazer fsync, copiar o backup só se o principal decodificar, oferecer o backup no erro `Corrupted` e corrigir a doc |
| M3 | `src/save/systems.rs:150-170`, `src/player/progression.rs:84-92` | Não há validação semântica do save: `hp: 0` gera um jogador vivo com 0 HP; `Progression` deriva `Deserialize` e aceita `level: 0` sem passar por `from_parts`; IDs desconhecidos são aceitos | Validar depois de `decode` |
| M4 | `examples/autoplay.rs`, `src/save/systems.rs:28` | O autoplay grava em `saves/slot1.ron` e destrói o save e o backup do jogador (confirmado nesta execução) | No exemplo, apontar `SaveSlots` para `target/autoplay/saves` |
| M5 | `src/combat/command_buffer.rs:168-175`, `src/player/movement.rs:23-39` | A deduplicação compara movimentos **relativos**. Olhando para a direita, ↓ ↘ ← faz o personagem virar; ↘→↙ vira DF→DF e não é gravado, e o ← vira F. O Crescent Slash sai para a esquerda. Comandos com "trás" não podem ser feitos andando (está no D-006, mas não no INPUT_SYSTEM) | Gravar a direção absoluta e converter no reconhecimento; adicionar teste |
| M6 | `src/enemies/behavior.rs:175-216`, `src/player/systems.rs:151-153` | Um Miss no dano de contato não dá invulnerabilidade, então é rolado de novo a cada frame. A esquiva por atributo quase não protege contra contato | Cooldown de contato por inimigo ou janela curta depois de um Miss |
| M7 | `src/player/state.rs:231` | A guarda anula o stagger também em golpes pelas costas; a redução de dano vale só pela frente. A doc diz "dano frontal sem stagger" | Passar `from_front` para `hurt()` |
| M8 | `src/combat/command_input.rs:73-77` | Qualquer resultado diferente de `Executed` (cooldown, não aprendido, MP insuficiente, lento) dispara um ataque básico. Visto no autoplay: `NotLearned` + `Hit 21`. `INPUT_SYSTEM.md` diz que só `Invalid` faz isso | Decidir a regra e alinhar código ou doc |
| M9 | `src/ui/status_screen.rs:140-145` | Ao distribuir pontos na tela de status, o HP/MP máximo exibido fica desatualizado até fechar a tela, porque `sync_derived_stats` só roda em `Playing` | Mostrar `sheet.derived()` ou sincronizar também nesse estado |
| M10 | `src/inventory/equipment.rs:61-75` | `equip` não confere nem move o item do inventário: um único anel cabe em `Accessory1` e `Accessory2`, com bônus em dobro. Hoje só o equipamento inicial usa isso | Fechar a regra pura antes da tela de inventário |
| M11 | `src/enemies/spawning.rs:80-89`, `src/enemies/components.rs:40`, `src/world/maps.rs:14` | A estrutura não está pronta para transições: os spawns usam `start.room` em vez de `CurrentRoom`; `PendingRespawns` não guarda a sala; nenhum sistema consulta `RoomEntity`; hitboxes e projéteis não são marcados com a sala | Tratar junto com M1 antes da etapa 11 |
| M12 | `tests/` | Cobertura: nenhum caminho de rejeição de dados testado; round trip do save só com coleções vazias; o ECS não cobre acerto, morte, EXP, respawn, F5/F9, pausa, guarda, auto-learn, i-frames nem A1–A3; os testes de atributos usam `StatFormulas::default()`, uma cópia dos valores do RON que o jogo não usa | Priorizar testes ECS dos fluxos de integração e testes de rejeição |
| M13 | `docs/ARCHITECTURE.md:59-62`, `.claude/rules/architecture.md` | A regra "um módulo nunca altera componentes de outro" é violada por `begin_skill` (Vitals, Velocity), `cast_spells` (cura), `allocate_points`, `quick_load` e `guard_and_dodge` (Invulnerable) | Documentar as exceções ou migrar aos poucos para mensagens |
| M14 | `docs/SKILL_SYSTEM.md`, `docs/ROADMAP.md:102`, `docs/RESUMO_PROJETO.md` | Recursos documentados como prontos que só existem como API ou estrutura: rank (`upgrade` nunca é chamado e não afeta dano), descoberta (`reveal_command` sem chamador, nenhuma skill `Discovery`), migração de save (só existe o braço da v1) | Marcar como pendente no roadmap e na doc |
| M15 | vários | Constantes de balanceamento e de sensação fora de `core/constants.rs` e do RON: `ATTACK_WINDUP`/`ACTIVE` e patrulha `* 0.6` (`enemies/behavior.rs`), `ENEMY_CRITICAL_*`, `LUCK_DROP_BONUS`, capacidade e idade do buffer, `ACTION_FRICTION`, offsets de hitbox, `constitution / 2` na MDEF | Mover conforme as regras 11 e 12 do CLAUDE.md |
| M16 | `.claude/agents/*.md`, `.claude/skills/atomic-commit/SKILL.md:16` | `game-architect` diz que "só edita docs", mas tem `Edit`/`Write` sem restrição. `code-reviewer` é "somente leitura", mas tem Bash e é instruído a rodar `cargo test` (pesado). Contradição: CLAUDE.md manda `cargo fmt` (altera arquivos) e a skill usa `--check`. "Separar trechos" exige `git add -p`, que é interativo | Restringir por permissão, não por texto, e unificar a regra de fmt |
| M17 | `src/core/time.rs` | Não há `FixedUpdate`: abaixo de 30 FPS o jogo fica em câmera lenta, e a altura do pulo varia um pouco com o FPS. Não está registrado como decisão | Registrar em DECISIONS.md ou adotar passo fixo |
| M18 | `src/combat/attacks.rs:123` | A hitbox da técnica não acompanha o `attack_speed`, mas o lock sim (o ataque básico escala os dois). Latente, porque nenhum item tem `attack_speed` hoje | Dividir o `lifetime` por `attack_speed` |
| M19 | `src/save/systems.rs:28` | Saves ficam em `project_root()/saves`. No build `dist` instalado num diretório só de leitura, o F5 sempre falha (com aviso, sem crash) | Diretório de dados do usuário (`$XDG_DATA_HOME`) |

## BAIXO

- **Ordem ambígua no mesmo set:**
  - `tick_cooldowns` × `begin_skill`;
  - `contact_damage` × `detect_hits` (os dois usam `GameRandom`, o que quebra a reprodutibilidade);
  - `think_and_move` × `respawn`;
  - `toggle_status_screen` × `quick_load` (os dois chamam `NextState::set`).
- **Respawn reseta menos que o load:** não limpa buffer, cooldowns nem hitboxes, e não chama `snap_to` na câmera. Hearts não são restaurados (pode ser intencional).
- **Esquiva:** perde 1 frame de i-frames no fim (`tick_invulnerability` desconta já no frame inicial).
- **Cobrança não atômica:** `try_spend(mp) && try_spend(hearts)` pode gastar o MP e falhar no Hearts. Hoje é inalcançável.
- **Sem fallback de skill:** se a primeira skill que casou estiver em cooldown, a próxima não é tentada. Com ataque e magia no mesmo frame, a magia é descartada.
- **Agachar é só visual:** a hurtbox não muda.
- **Ataque básico:** sem buffer durante `Attacking`.
- **Teto baixo (plausível):** bater no teto dentro do coyote time pode permitir um segundo pulo.
- **Pausa incompleta:** notificações e números de dano continuam animando na tela de status.
- **`player::receive_damage`:** usa `break` com o jogador morto e deixa mensagens no leitor (plausível e improvável).
- **Consistência:** `regenerate` usa `Single` sem `With<Player>`; `track_play_time` está fora de qualquer set (inofensivo, porque tem `run_if`).
- **Dependências circulares:** `core/events.rs` e `core/resources.rs` importam de 5 módulos que dependem de `core`; `world` usa `player::Facing` e `Player`.
- **Save síncrono:** gravado no thread principal (possível travada curta no F5).
- **Dados de inventário:**
  - `max_stack: 0` vira 1 sem aviso;
  - equipamento com `max_stack > 1` é aceito;
  - o comentário de `tier` diz "menor primeiro", mas a ordenação é decrescente.
- **Equipamento inicial:** valida requisitos com atributos base e sem itens nem flags.
- **Level up:** aumenta o máximo de HP/MP mas não o valor atual. É testado, mas não documentado.
- **Testes fracos ou duplicados:**
  - `data_tests` (`!is_empty()`, `>= 2` quando o certo é 3);
  - `assert_ne!(.., Complete)`;
  - `outcome_executed`/`wrong_button_never_matches` repetem outros testes;
  - "seven outcomes" em comentário e em `combat-system/SKILL.md`, quando são 8.
- **Autoplay dirigido pelo relógio real:** janelas de 60 ms podem falhar com FPS baixo (plausível).
- **Docs:**
  - README diz "Shift", mas só `ShiftLeft` funciona;
  - `INPUT_SYSTEM.md` diz que nada lê teclas direto, mas a tela de status usa `Digit1..4` fixos e o save lê `ButtonInput`;
  - `ENEMY_SYSTEM.md` diz "laranja" (a cor é avermelhada) e omite a condição de distância da fuga;
  - `RESUMO_PROJETO.md` diz que `Presentation` só roda em `Playing` (roda sempre);
  - comentários citam `assets/data/progression.ron` e `items/rarities.ron`, que não existem; `rooms.rs` fala em "exits";
  - o comentário de `snap_to` cita início e respawn.
- **Mensagens de erro:** `CatalogError::DuplicateId` não diz se é item ou raridade, nem o arquivo.
- **Atributos sem limite superior:** um save adulterado poderia estourar `i32` (plausível).
- **RNG com semente fixa:** as rolagens são iguais em toda sessão (intencional, segundo o comentário).
- **Commits:** `56962c4` e `9dcdfcc` usam `docs:` sem escopo, contra o formato de `commits.md`.

---

## Verificado e correto

- **Arquitetura:**
  - `main.rs` só chama `app::run()`;
  - `app/` centraliza janela, dados e `GamePlugin`;
  - 10 plugins idênticos entre código, ARCHITECTURE, README e RESUMO;
  - os 22 arquivos "puros" não importam Bevy;
  - o maior arquivo tem 432 linhas.
- **Módulos sem plugin próprio:** `state`, `physics`, `camera` e `progress` não têm plugin, e está correto. São submódulos de `player`/`world` registrados pelo plugin dono.
- **ECS:**
  - os `GameSet` estão encadeados na ordem documentada;
  - todo sistema de `Update` está num set ou tem `run_if`;
  - toda mensagem é escrita num set anterior ou no mesmo set (ordenado) do leitor, sem perda nem atraso de frame;
  - a tela de status congela todos os timers de gameplay e não acumula delta ao despausar;
  - o respawn roda antes do dano, e o F5 é bloqueado com o jogador morto.
- **Fórmulas:**
  - dano (acerto/esquiva com piso de 0,5, DEF/MDEF, resistência limitada, crítico, variância, mínimo 1, imunidade 0) bate com `COMBAT_SYSTEM.md`;
  - todos os derivados de `CHARACTER_ATTRIBUTES.md` e o exemplo HP 135 / MP 52 conferem;
  - a curva de EXP (30/91/174/1194) confere.
- **Progressão:**
  - sem underflow;
  - EXP satura;
  - o nível máximo trata o excedente;
  - `ResourcePool` nunca sai de [0, máx];
  - equipamento recalcula derivados e máximos via `Changed<CharacterSheet>`.
- **Inimigos:**
  - a recompensa é entregue uma única vez, mesmo com vários acertos no frame;
  - o respawn cria uma entidade nova com todo o estado zerado;
  - a IA (patrulha, perseguição com leash, ataque com aviso de 0,35 s, cooldown, fuga) funciona como documentado;
  - drops entram de fato no inventário.
- **Comandos:**
  - buffer de 32 entradas e 1,5 s;
  - prioridade, depois tamanho, depois id;
  - limpeza a cada botão e na guarda (sem execução dupla);
  - os 8 resultados testados com valores exatos.
- **Magia:**
  - Ember Bolt usa MATK/MDEF e as resistências do alvo;
  - a cura é limitada ao HP máximo;
  - projétil sem `pierce` some no primeiro acerto.
- **Save:**
  - formato `SaveFile { version, data }` com cabeçalho lido primeiro;
  - erros `TooNew`/`TooOld`/`Corrupted`;
  - arquivo temporário + `rename`;
  - sem `unwrap` no caminho do save;
  - F5/F9 ligados nos `InputBindings`.
- **Dados:** a validação roda antes de abrir a janela, com erro de parse informando caminho e posição, e o processo sai com código diferente de zero.
- **Testes de regra pura:** valores exatos, RNG injetado, sem depender do relógio; os testes de save usam um diretório isolado dentro de `target/`.
- **Claude Code:**
  - todos os caminhos e identificadores citados nas regras e skills existem;
  - a preservação das alterações do usuário está instruída;
  - `unsafe_code = "forbid"` e os lints conferem.

## Próximos passos recomendados (em ordem)

1. **Ambiente (C1, A6):** limitar jobs, proibir builds em `/tmp`, fechar `--output` e ampliar a lista `deny`. Só configuração, nenhum risco ao jogo.
2. **Autoplay com asserções e save isolado (A5, M4):** transforma o teste de fumaça em verificação real antes de mexer em combate.
3. **Integração de combate (A1, A2, A3), com testes ECS para cada um:** são os bugs que o jogador sente.
4. **Validação de dados (A4, M12):** validar fórmulas, valores, duplicatas, referências e categoria × efeito, com testes de rejeição.
5. **Save/load e salas (M1, M2, M3, M11):** antes da etapa 11 (transições).
6. **Alinhar a documentação (M13, M14, M8 e BAIXOs de doc):** numa tarefa só de documentação.

Cada item é independente e cabe em um ou poucos commits atômicos.
