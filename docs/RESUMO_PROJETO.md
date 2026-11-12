# Resumo do projeto — Dark Castle RPG

Estado consolidado ao fim da etapa de setup (`Md-usario/continuacao-setup.md`).
Referência: commit `56962c4`, data 2026-11-11. Este arquivo é um retrato; os
documentos detalhados continuam sendo a fonte de verdade
(`ARCHITECTURE.md`, `DECISIONS.md`, `ROADMAP.md` e os `*_SYSTEM.md`).

## 1. Arquitetura atual

- **Stack:** Rust 2024 (rustc 1.99.1) + Bevy 0.20.1, pacote `dark_castle_rpg`.
- **Entrada:** `main.rs` só chama `app::run()`; toda a lógica vive na biblioteca
  (`src/lib.rs`), reutilizada pelos testes e por `examples/autoplay.rs`.
- **Plugins** registrados em `src/app/plugins.rs`: `Core`, `Input`, `World`,
  `Player`, `Combat`, `Skills`, `Magic`, `Enemies`, `Ui`, `Save`.
- **Duas camadas por módulo:** regras puras (sem Bevy, testáveis) e sistemas
  Bevy que as executam. Exemplos: `combat/damage.rs` e `combat/command_parser.rs`
  (puros) × `combat/hitboxes.rs` e `combat/attacks.rs` (Bevy).
- **Ordem do frame:** `GameSet` encadeados, que rodam só em `GameState::Playing`:
  `Input → Commands → Actions → Physics → Combat → Resolution → Progression → Presentation`.
- **Comunicação por mensagens** (`core/events.rs`): um módulo não altera
  componentes de outro; ele emite, por exemplo, `DamageDealt`, e o módulo dono aplica.
- **Dados em RON** (`assets/data/`, `assets/maps/`), carregados e validados
  em `GameData` antes da janela abrir; dados inválidos encerram com erro claro.
- Tamanho atual: cerca de 7,3 mil linhas de Rust em `src/`.

## 2. Sistemas implementados (validados)

| Área | O que existe |
|---|---|
| Personagem | Andar, correr, pular (altura variável e *coyote time*), agachar, guarda, esquiva invulnerável, máquina de estados, câmera suave |
| Atributos/progressão | STR/CON/INT/LCK, HP/MP/Hearts e derivados, curva de EXP, level up, pontos distribuíveis, regeneração |
| Combate | Ataque com hitbox temporária, fórmula de dano (defesa, elemento, crítico, precisão, variância), recuo, invulnerabilidade, morte/respawn, números de dano |
| Comandos | Buffer temporizado relativo à orientação, parser de notação (setas ou letras), tolerância, prioridade, 8 resultados distintos com feedback no HUD |
| Técnicas | Definidas em dados: Crescent Slash, Rising Edge (custa Hearts, desbloqueia no nível 3), cooldowns |
| Magias | Ember Bolt (projétil elemental) e Mending Light (cura), custo de MP, tempo de conjuração |
| Inventário (regras) | Itens, raridades, pilhas, 8 slots de equipamento com requisitos, drops influenciados por LCK |
| Inimigos | IA pura testada (patrulha, perseguição, ataque com aviso, fuga), Bone Sentry, fraquezas, EXP, drops, respawn |
| Mundo | Sala `entry_hall` em RON, colisão AABB própria, plataformas atravessáveis |
| UI | HUD (HP/MP/Hearts/nível/EXP/cooldowns/avisos), tela de status (Tab) |
| Save | Formato versionado com migração, detecção de corrupção, escrita segura com backup, F5/F9 |

## 3. Sistemas pendentes

Lista completa e verificável em `ROADMAP.md`. Principais:

- CI (fmt/clippy/test automáticos).
- Interação com o mundo (alavancas, NPCs, baús); transições entre salas; minimapa.
- Sprites e animações reais; áudio; partículas; fonte própria.
- Combos básicos e efeitos de status.
- Tela de inventário e uso de consumíveis em jogo.
- Magias de área, dano contínuo, controle e invocação.
- Novos arquétipos de inimigo, barra de vida de inimigos, chefes com fases.
- Menu principal/pausa, vários slots de save, salvar só em pontos de save.
- Gamepad e remapeamento; ferramentas de balanceamento.

## 4. Decisões técnicas

Resumo de `DECISIONS.md`:

- **D-001** Bevy 0.20.1 sem as features padrão, só `2d`, `ui` e `audio`; versões fixadas com `=`.
- **D-002** Dependências diretas apenas `serde`, `ron`, `thiserror` e `tracing`, nas mesmas
  versões usadas pelo Bevy. Sem `rand` (SplitMix64 próprio), sem inspector, sem tilemap.
- **D-003** RON para dados e saves; comandos em notação de texto validada no carregamento.
- **D-004** Regras puras separadas dos sistemas Bevy.
- **D-005** Física AABB própria em vez de avian/rapier.
- **D-006** Comandos relativos à orientação do personagem.
- **D-007** Ajustes na estrutura de pastas sugerida no briefing.
- **D-008** Clippy: `unwrap_used`/`expect_used` como aviso; dois lints típicos de Bevy liberados.
- **D-009** Golpes corpo a corpo atingem vários alvos uma vez cada; projéteis somem no primeiro acerto.
- **D-010** Teste de fumaça visual com `examples/autoplay.rs`.

Perfis de compilação: `dev` (código com `opt-level 1`, dependências com 3),
`release` (LTO thin), `dist` (LTO fat, `codegen-units = 1`, strip, `panic = abort`);
feature `dev` para linkagem dinâmica.

## 5. Configurações do Claude Code

- **`CLAUDE.md`**: regras obrigatórias do projeto, comandos e política de commits atômicos.
- **`.claude/rules/`**: `architecture.md`, `commits.md`, `game-design.md`,
  `rust-conventions.md`, `testing.md` (carregadas por caminho de arquivo).
- **Skills (5):** `/game-feature`, `/combat-system`, `/rust-code-review`,
  `/test-and-validate`, `/atomic-commit` (esta só por invocação manual).
  A skill `/multi-agent-development` ficou para uma etapa posterior, como pedido.
- **Agentes (3):** `game-architect` (planeja, edita só docs),
  `combat-specialist` (implementa combat/skills/magic), `code-reviewer` (só leitura).
- **`.claude/settings.json`**: libera `cargo fmt/clippy/test/check/build/run/tree`
  e `git status/diff/log/show`; nega `git reset --hard`, `git clean`,
  `git push --force/-f`, `git checkout -- ` e `rm -rf`.

## 6. Testes e validações executados

- **No HEAD (`56962c4`):** `cargo fmt --check` ok; `cargo clippy --all-targets`
  sem avisos; `cargo test` com **85 testes passando** (unitários, 8 arquivos de
  integração em `tests/` e o smoke test ECS headless); `cargo check` ok.
- **Commit a commit:** a etapa tem 12 commits (`af01608` a `56962c4`).
  - Os 10 commits com código foram compilados isoladamente com clippy e testes,
    e todos passaram sem avisos: `af01608`, `78eced9`, `b16f416`, `1902c51`,
    `4d54887` e `c7b15a9` na sessão anterior; `23b8eb6`, `96a911b`, `a5c9e2e`
    e `d8522ff` nesta.
  - `56962c4` (HEAD) foi validado por completo (fmt, clippy, test, check).
  - `eba599e` altera só `CLAUDE.md` e `.claude/`. Não foi compilado
    isoladamente, mas o código dele é idêntico ao de `d8522ff`, que passou.
- **Execução real:** `cargo run --example autoplay` rodou em sessão Wayland e
  gerou 7 screenshots em `target/autoplay/` (início, Crescent Slash, Ember Bolt,
  pós-combate, Rising Edge bloqueado, tela de status, save).
- **Ambiente:** bibliotecas do Bevy presentes via pkg-config (alsa, libudev,
  wayland-client, xkbcommon, x11).

## 7. Problemas encontrados

1. **Travamento do PC durante a verificação.** A verificação commit a commit
   compilava o Bevy em `/tmp`, que neste sistema é **tmpfs (fica na RAM, cerca de 7 GB)**.
   Os builds esgotaram a memória (processo morto com exit 137) e depois a cota do `/tmp`.
   Solução: os artefatos temporários foram apagados, e a verificação foi refeita com
   um worktree em `target/` reaproveitando o `target/` do projeto.
   **Regra prática:** nunca use `CARGO_TARGET_DIR` em `/tmp` neste projeto.
2. **A proteção contra `rm` no `settings.json` é parcial.** O padrão `Bash(rm -rf:*)`
   bloqueia `rm -rf`, mas não `rm -r` nem `rm -fr`. Vale ampliar a regra numa tarefa própria.
3. O build completo do Bevy é pesado (`target/` com cerca de 47 GB, contando
   perfis e exemplos). Para não esgotar a RAM, limite o paralelismo (`-j 4`) ao compilar do zero.

## 8. Próximos passos recomendados

1. **CI** (GitHub Actions com fmt, clippy e test), para fechar a etapa 1 do roadmap.
2. Endurecer o `settings.json` (padrões de `rm` mais amplos).
3. **Transições entre salas e objetos interativos** (etapa 11), base do Metroidvania.
4. **Tela de inventário e consumíveis**: as regras puras já existem, falta a camada Bevy e a UI.
5. Barra de vida de inimigos e um segundo arquétipo (à distância), para exercitar a IA genérica.
6. Planejar a skill `/multi-agent-development`, que ficou adiada.
