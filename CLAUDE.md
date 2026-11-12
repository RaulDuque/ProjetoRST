# Dark Castle RPG — regras do projeto

RPG de ação 2D estilo Metroidvania em **Rust 2024 + Bevy 0.20.1**. Um único
personagem jogável, combate em tempo real e técnicas/magias executadas por
**sequências de botões**. Universo, nomes e arte são originais.

Pacote: `dark_castle_rpg`. Raiz do projeto: a pasta que contém este arquivo.
`Md-usario/` guarda os briefings originais do usuário: não altere.

## Comandos

```bash
cargo run                         # joga (janela 1280x720)
cargo run --features dev          # idem, com linkagem dinâmica (compila mais rápido)
cargo run --example autoplay      # teste de fumaça roteirizado + screenshots em target/autoplay/
cargo fmt --check
cargo clippy --all-targets
cargo test                        # unitários + integração + ECS headless
cargo check
cargo build --profile dist        # build de distribuição
```

Se `cargo` não for encontrado: `source "$HOME/.cargo/env"`.

## Arquitetura em uma frase

Cada módulo de funcionalidade (`player`, `combat`, `skills`, `magic`,
`inventory`, `enemies`, `world`, `save`, `ui`) separa **regras puras** (sem
Bevy, testáveis) de **sistemas Bevy** que as executam, e expõe um `Plugin`
registrado em `src/app/plugins.rs`. Detalhes: `docs/ARCHITECTURE.md`.

Regras detalhadas (carregadas automaticamente): `.claude/rules/`.

## Regras obrigatórias

1. Antes de implementar qualquer funcionalidade, analise a arquitetura existente (`docs/ARCHITECTURE.md`, módulo afetado, testes).
2. Nunca reescreva módulos inteiros quando uma alteração pequena for suficiente.
3. Nunca remova funcionalidades existentes sem autorização.
4. Não crie duplicações de sistemas (procure antes: `grep -rn` pelo conceito).
5. Não misture responsabilidades (regra pura ≠ sistema Bevy ≠ UI).
6. Não coloque lógica no `main.rs` (ele só chama `app::run()`).
7. Funções, variáveis, structs e enums em inglês.
8. Documentação explicativa pode ser em português.
9. Use tipagem forte (enums e newtypes em vez de strings/números soltos).
10. Evite `unwrap()`/`expect()` em caminhos normais (o clippy avisa). Em testes use `unwrap_or_else(|e| panic!(...))`.
11. Evite valores mágicos espalhados pelo código.
12. Constantes de "sensação" em `src/core/constants.rs`; balanceamento em `assets/data/*.ron`.
13. Prefira componentes pequenos e sistemas com uma responsabilidade.
14. Não introduza dependências sem necessidade (justifique em `docs/DECISIONS.md`).
15. Toda funcionalidade importante precisa de testes.
16. Execute `cargo fmt`, `cargo clippy --all-targets`, `cargo test` e `cargo check` antes de considerar uma tarefa concluída.
17. Não declare uma funcionalidade concluída se ela não foi implementada e validada.
18. Não use código fictício ou funções vazias para simular funcionalidades.
19. Não esconda erros de compilação (nada de `#[allow]` amplo para silenciar problemas reais).
20. Atualize a documentação quando modificar a arquitetura.

## Commits atômicos (regra crítica)

Cada commit = **uma** alteração lógica independente, no formato Conventional Commits:

```
feat(player): implement basic movement
feat(combat): add melee attack system
fix(combat): correct damage calculation
test(player): add movement tests
docs(architecture): document plugin structure
refactor(inventory): separate equipment logic
```

Nunca misture funcionalidades diferentes, refatorações não relacionadas,
documentação sem relação ou correções fora do objetivo do commit.

Antes de commitar: `git status` → `git diff` → os arquivos pertencem à mesma
tarefa? → testes pertinentes → `cargo fmt` + `cargo clippy` → sem arquivos
temporários/assets desnecessários → commit apenas da tarefa. Use a skill
`/atomic-commit`.

- Não faça commits com código quebrado, nem commits gigantes, nem um commit por arquivo salvo.
- Nunca use `git reset --hard`, `git clean -fd`, `git push --force` ou operações destrutivas sem autorização explícita.
- Preserve alterações anteriores do usuário.

## Skills e agentes

| Skill | Uso |
|---|---|
| `/game-feature` | Implementar uma funcionalidade de jogo de ponta a ponta |
| `/combat-system` | Alterar combate, comandos, habilidades ou magias |
| `/rust-code-review` | Revisar código Rust/ECS e classificar problemas por gravidade |
| `/test-and-validate` | Rodar fmt/clippy/test/check/autoplay e relatar |
| `/atomic-commit` | Separar e commitar alterações atomicamente |

Agentes (`.claude/agents/`): `game-architect` (planeja, só edita docs),
`combat-specialist` (implementa em combat/skills/magic), `code-reviewer`
(somente leitura). Dois agentes nunca editam os mesmos arquivos ao mesmo tempo.
