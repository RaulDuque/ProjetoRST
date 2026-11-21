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
cargo build --profile dist        # build de distribuição (pesado; não é validação rotineira)
```

Se `cargo` não for encontrado: `source "$HOME/.cargo/env"`.

## Compilação segura (obrigatório)

O build do Bevy consome muita memória, e `/tmp` nesta máquina é **tmpfs (RAM)**.
Compilações em `/tmp` e com paralelismo alto já travaram o computador.

1. **Paralelismo:** `.cargo/config.toml` limita a 4 jobs. Não passe `-j` maior nem
   defina `CARGO_BUILD_JOBS` acima de 4.
2. **Um único diretório de build:** sempre o `target/` da raiz. `CARGO_TARGET_DIR`
   e `--target-dir` só podem apontar para esse mesmo `target/` (caso do item 3);
   nunca para `/tmp`, pastas temporárias ou externas. Nunca crie um segundo `target`.
3. **Sem cópias do projeto para compilar:** nada de clones ou worktrees em `/tmp`
   ou fora do projeto, e nada de worktrees para validação rotineira. A validação
   usa a árvore de trabalho atual. Exceção rara, só com necessidade técnica
   comprovada **e autorização explícita do usuário**: worktree **dentro de
   `target/`** com `CARGO_TARGET_DIR="<raiz>/target"`, uma por vez, removida com
   `git worktree remove` ao final.
4. **Reaproveite artefatos:** não rode `cargo clean` nem apague `target/`, e não
   repita uma validação que já passou e continua válida.
5. **Uma compilação por vez:** nunca rode mais de um comando Cargo que compile ao
   mesmo tempo (em background, em subagentes ou em terminais diferentes), mesmo
   que compartilhem o mesmo `target/`.
6. **Memória antes de builds pesados** (build do zero, mudança de perfil,
   `--release`, `--profile dist`): confira `free -h`. Com menos de 6 GiB
   disponíveis, não compile e avise o usuário.
7. **Perfis:** o desenvolvimento normal usa `cargo check`, `cargo test`,
   `cargo clippy` e `cargo run` (perfil `dev`). O perfil `dist` (LTO fat,
   `codegen-units = 1`) só é usado quando o usuário pedir uma build de distribuição.

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

## Relatório ao final de toda tarefa (regra permanente)

Toda tarefa termina com um relatório em `docs/relatorios/`, mesmo se ficou bloqueada,
parcial ou rejeitada. Isso vale para prompt, implementação, correção, investigação, teste
ou etapa artística.

1. Siga o modelo de `docs/relatorios/PADRAO_RELATORIO.md`: 27 itens e um status, entre
   CONCLUÍDO, CONCLUÍDO COM RESSALVAS, PARCIALMENTE CONCLUÍDO, BLOQUEADO, REJEITADO e
   AGUARDANDO APROVAÇÃO VISUAL.
2. Atualize `docs/relatorios/INDICE.md` e `docs/relatorios/ULTIMO_RELATORIO.md`.
3. Faça commit e push, sem force, dos arquivos da tarefa junto com os relatórios.
4. No terminal, responda só com o resumo curto.

Execução técnica não é aprovação artística. Detalhes em `.claude/rules/relatorios.md`.

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
