# Commits

- Conventional Commits: `tipo(escopo): descrição no imperativo, em inglês, minúscula`.
  Tipos: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`, `perf`, `build`.
  Escopos usuais: `player`, `combat`, `skills`, `magic`, `inventory`, `enemies`, `world`, `ui`, `save`, `core`, `app`, `data`, `claude`, `architecture`.
- Um commit = uma alteração lógica. Cada commit deve compilar e passar nos testes sozinho.
- Fluxo: `git status` → `git diff` → separar por responsabilidade → `cargo fmt` → `cargo clippy --all-targets` → `cargo test` → `git add <arquivos>` (nunca `git add -A` às cegas) → `git commit`.
- Proibido sem autorização explícita: `git reset --hard`, `git clean -fd`, `git push --force`, `git rebase` em histórico publicado, `git checkout -- .`.
- Não commitar: `target/`, `saves/`, arquivos temporários, `.claude/settings.local.json`.
- Commits só quando o usuário pedir ou quando o fluxo de trabalho combinado incluir commit.
