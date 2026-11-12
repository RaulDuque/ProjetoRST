# Registro de decisões

## D-001 — Bevy 0.20.1 com features mínimas
Versão estável mais recente no crates.io em 2026-11-11 (MSRV 1.97.1).
Usamos `default-features = false` com `2d`, `ui` e `audio`: o jogo não tem 3D,
então evitamos compilar PBR, glTF etc. Versões fixadas com `=` no `Cargo.toml`
e `Cargo.lock` versionado.

## D-002 — Dependências diretas
`serde 1.0.229`, `ron 0.12.3`, `thiserror 2.0.21`, `tracing 0.1.44` — as
mesmas versões que o próprio Bevy 0.20.1 usa (sem duplicação no grafo).
`tracing` é usado diretamente nos módulos puros para não depender de Bevy.
**Não adicionados:** `bevy-inspector-egui` (0.38 não foi validado com Bevy 0.20 e
não é necessário ainda), crates de tilemap (salas simples em RON bastam),
`rand` (um SplitMix64 de 20 linhas em `core/rng.rs` é suficiente e determinístico).

## D-003 — RON para dados e saves
Formato nativo do ecossistema Rust/Bevy, legível, com enums e structs.
Comandos de habilidades usam uma string de notação (`"D DF F + ATTACK"`, ou
setas `"↓ ↘ → + ATTACK"`) convertida por `TryFrom<String>` — erros de digitação
são detectados no carregamento e no `cargo test` (`tests/data_tests.rs`).

## D-004 — Regras puras separadas dos sistemas Bevy
Fórmulas, parser, progressão, inventário e save não importam Bevy. Ganhos:
testes rápidos e determinísticos, regras reutilizáveis em ferramentas, e
nenhuma lógica de negócio presa a sistemas de renderização.

## D-005 — Física própria (AABB) em vez de engine de física
Plataforma 2D precisa de colisão de caixas, plataformas atravessáveis e
resposta previsível. `world/collision.rs` resolve por eixo; uma engine de
física (avian/rapier) traria peso e comportamento menos "arcade".

## D-006 — Comandos relativos à orientação
O personagem vira para a direção pressionada **antes** da gravação do comando
(`update_facing` roda antes de `record_and_resolve_commands`). Assim "→" é
sempre "para frente" do ponto de vista do jogador. Consequência: "trás" só é
gravado quando o personagem não pode virar (atacando, guardando...).

## D-007 — Ajustes na estrutura de pastas do briefing
- `core/time.rs` contém só o *clamp* do delta (evita atravessar plataformas em travadas).
- Não criados (sem finalidade ainda): `combat/combos.rs`, `combat/combat_state.rs`
  (o estado de combate é a `PlayerStateMachine` em `player/state.rs`),
  `skills/components.rs` (o componente `SkillBook` está em `player/components.rs`),
  `magic/spells.rs` (magias usam `SkillDefinition`), `inventory/equipment_slots.rs`
  (slots ficam em `equipment.rs`), `world/transitions.rs`, `ui/menus.rs`,
  `ui/inventory_screen.rs`, `ui/skill_screen.rs` (a lista de habilidades está na tela de status).
- Adicionados: `input/` (mapeamento de teclas), `player/state.rs`, `world/{physics,camera,progress}.rs`,
  `enemies/definitions.rs`, `ui/{theme,damage_numbers}.rs`, `save/systems.rs`,
  `examples/autoplay.rs`, `tests/{data,inventory,save,app_smoke}_tests.rs`.
- Pastas de assets sem arquivos não foram criadas (git não versiona pastas vazias); ver `assets/README.md`.

## D-008 — Lints do clippy
`unwrap_used`/`expect_used` = warn. `type_complexity` e `too_many_arguments`
liberados: parâmetros de sistemas Bevy (queries com filtros) disparam esses
lints por natureza, e o próprio projeto Bevy os desativa.

## D-009 — Hitboxes perfurantes no corpo a corpo
Golpes corpo a corpo atingem cada alvo uma vez (`already_hit`) e podem atingir
vários alvos; projéteis (`pierce: false`) somem no primeiro acerto.

## D-010 — Teste de fumaça visual
`examples/autoplay.rs` injeta teclas no app real após `InputSystems`, registra
resultados e salva screenshots com a API de screenshot do Bevy. Complementa
`tests/app_smoke_tests.rs` (headless, roda no `cargo test`).

## D-011 — Compilação limitada a 4 jobs, sempre no `target/` da raiz
O build do Bevy com o paralelismo padrão (12 jobs) e artefatos em `/tmp`
(tmpfs, que fica na RAM) esgotou os 14 GiB da máquina e travou o computador
duas vezes. `.cargo/config.toml` fixa `build.jobs = 4` como padrão do projeto.
Não fixamos `target-dir`: `target/` já é o padrão, e `CARGO_TARGET_DIR`
sobrescreveria a config de qualquer forma. A proibição de `/tmp` e de diretórios
de build extras é uma regra de processo ("Compilação segura" no `CLAUDE.md`).
Custo: um build do zero fica mais lento. Os builds incrementais não mudam.

## D-012 — Política de segurança do Claude Code em duas camadas
A auditoria (A6) mostrou que as permissões liberavam `git diff --output=<arquivo>`,
`rm -r`, `find -delete` e escritas em `Md-usario/` e `saves/`. Regras de permissão
comparam o **texto** do comando; a própria documentação do Claude Code avisa que
não são uma fronteira de segurança (`/bin/rm`, `bash -c '...'` e variáveis escapam).
Por isso usamos duas camadas em `.claude/settings.json`:

1. **Permissões:** `deny` para os padrões destrutivos conhecidos e
   `Edit(/Md-usario/**)`/`Edit(/saves/**)` (cobre Edit, Write, NotebookEdit e
   redirecionamentos `>` reconhecidos); `ask` para `git push`, `--amend` e builds
   `--release`/`--profile`; `allow` só para `cargo fmt --check` (não mais `cargo fmt`)
   e comandos de leitura/compilação; `disableBypassPermissionsMode`.
2. **Hook `PreToolUse` (`.claude/hooks/guard.py`):** tokeniza o comando, remove
   wrappers (`sudo`, `env`, `timeout`, `command`, `xargs`, atribuições) e analisa
   `bash -c`/`eval`. Bloqueia `rm` recursivo, `find -delete`, git destrutivo
   (`reset --hard`, `clean`, push forçado ou `+refspec`, `checkout --`/`.`,
   `restore` fora do `--staged`, `stash drop`, `branch -D`, `worktree add`, `rebase`,
   `--output`), `git add` de `Md-usario/`/`saves/`, escrita nessas pastas,
   `cargo clean`, `--target-dir`/`CARGO_TARGET_DIR` fora do `target/` da raiz,
   mais de 4 jobs e `cargo run --example autoplay` quando `examples/autoplay.rs`
   não contém o isolamento de saves (`isolated_save_dir` + override de
   `SaveSlots`; ver D-013). Também separa comandos por quebra de linha e crase, acompanha `cd`
   dentro do mesmo comando e bloqueia `git stash -u/-a` (tiraria os arquivos não
   rastreados de `Md-usario/` da árvore). Se não conseguir analisar a chamada
   (JSON inválido, campos com tipo errado), bloqueia (falha fechada).
   Testes: `python3 -B .claude/hooks/test_guard.py` (testam só o hook, não as
   regras de `settings.json`).

**Regras de texto × hook:** quando divergem, vale a mais restritiva. Ex.:
`git clean -n` passa pelo hook mas é negado pela regra `Bash(git clean *)`;
`--target-dir` é negado mesmo apontando para o `target/` da raiz (a exceção de
worktree usa `CARGO_TARGET_DIR`, que o hook aceita só para esse `target/`);
`git rebase --abort` também fica negado e precisa ser feito pelo usuário.
**Falsos positivos conhecidos:** o corpo de um heredoc é analisado como comandos,
então uma linha que comece com `rm -r` ou `git reset --hard` dentro de um heredoc
é bloqueada; `ln` que apenas aponte para um arquivo protegido também.

**Limitações:** continua possível escapar com scripts e interpretadores
(`python -c`, `perl -e`), variáveis expandidas (`"$PWD"/saves`), globs
parciais (`sa*/`), aliases e programas que gravam
arquivos por conta própria (o próprio jogo grava em `saves/` no F5). Sem
`python3` no PATH o hook falha sem bloquear. A proteção real de sistema de
arquivos seria o sandbox do Claude Code (`sandbox.enabled` +
`sandbox.filesystem.denyWrite`), que no Linux exige `bubblewrap` e `socat`;
`socat` não está instalado, então ele não foi ativado. `disableAllHooks`
desativa a camada 2.

## D-013 — Autoplay com verificações e saves isolados
O autoplay gravava no `saves/slot1.ron` real e saía com sucesso mesmo quando um
golpe errava (A5, M4). Agora `examples/autoplay.rs`:
- substitui o recurso `SaveSlots` por `target/autoplay/saves/` antes do primeiro
  frame, confere que o override está ativo e recusa iniciar se o diretório não
  estiver dentro de `target/` ou se sobrepuser ao `saves/` real. O jogo normal
  não muda: `SaveSlots::default()` continua apontando para `saves/`;
- roda com passo fixo de 16 ms (`TimeUpdateStrategy::ManualDuration`), para o
  roteiro não depender do FPS;
- confere jogador, sala inicial, movimento, ataque básico, Crescent Slash,
  Ember Bolt, Mending Light (cura só quando o HP não está cheio), custos de
  MP/Hearts (tolerância de 1 ponto de regeneração no frame), Rising Edge
  rejeitado, save e load no diretório isolado e `saves/` byte a byte igual;
- termina com código ≠ 0 se uma verificação obrigatória falhar ou se o app
  fechar antes das verificações. Linhas `INFO` registram pontos ainda sem regra
  aprovada (ataque básico após comando rejeitado, D-GAME-003).
O mecanismo também é coberto sem janela por
`tests/app_smoke_tests.rs::quick_save_and_load_use_the_configured_save_dir`.

**Liberação pelo hook:** `guard.py` só deixa o Claude rodar
`cargo run --example autoplay` se o arquivo contiver `fn isolated_save_dir()` e
`insert_resource(SaveSlots(`. Essa checagem é **textual e protege apenas contra
acidentes** (um exemplo antigo ou reescrito sem o override); não comprova o
isolamento e não cobre a execução direta do binário em `target/`. A garantia
vem do próprio autoplay: a recusa em iniciar sem o override e a comparação
byte a byte de `saves/` ao final, que reprova a execução.
**Limitações conhecidas:** a cura do Mending Light não é exercitada (o jogador
não toma dano no roteiro, então a verificação fica "não aplicável"); o roteiro
depende do passo de 16 ms, porque a física varia com o intervalo entre frames
(M17).
