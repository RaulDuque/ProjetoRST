# DARK CASTLE RPG — CHECKPOINT GIT OFICIAL
## Consolidação das versões implementadas até a V2.1.1

### OBJETIVO

A V2.1.1 está encerrada e aprovada.

Antes de iniciar a última etapa de desenvolvimento de hoje, quero consolidar no Git todo o trabalho válido realizado até agora.

Você está autorizado a preparar commits organizados e publicar o trabalho no repositório remoto, seguindo rigorosamente as regras de segurança abaixo.

**Esta autorização substitui exclusivamente a proibição anterior de staging, commit e push para esta operação de checkpoint.**

Não autoriza apagar arquivos, reescrever histórico ou alterar conteúdo funcional desnecessariamente.

---

# 1. AUDITORIA INICIAL

Antes de qualquer operação de escrita no Git:

1. Confirme o diretório do projeto.
2. Confirme a branch atual.
3. Confirme o HEAD.
4. Confirme o remoto configurado.
5. Confirme o upstream.
6. Execute `git status --porcelain=v1 -uall`.
7. Execute `git diff --stat`.
8. Execute `git diff --cached --stat`.
9. Liste os arquivos não rastreados.
10. Liste os arquivos ignorados relevantes.
11. Verifique a existência de conflitos.
12. Verifique se existem operações Git incompletas.

O estado conhecido é:

- Branch: `art/v1.2-protagonist`.
- HEAD: `204f965`.
- Staging anterior: somente `protagonist_concept_v001.png`.
- Centenas de alterações acumuladas desde versões anteriores.

Confirme esses dados.

Não suponha que o estado continua igual.

---

# 2. PROTEÇÃO DE ARQUIVOS

Não publique:

- Saves reais.
- Backups pessoais.
- Credenciais.
- Tokens.
- Chaves privadas.
- Arquivos `.env` contendo segredos.
- Arquivos temporários.
- Caches.
- Diretórios `target/`.
- Binários de compilação.
- Logs desnecessários.
- Arquivos pessoais não relacionados ao projeto.

Preserve integralmente:

- `saves/`.
- `.cargo/config.toml`.
- `target/verify/`.
- `target/f4-stage/`.
- Arquivos protegidos em `Md-usario/`.
- Assets publicados.
- Backups artísticos.
- Configurações de proteção do Claude Code.

Não modifique nenhum desses arquivos apenas para preparar o commit.

**A presença de um arquivo no Git não significa que ele pode ser publicado automaticamente.**

---

# 3. AUDITORIA DE Md-usario/

A pasta `Md-usario/` contém materiais do proprietário.

Os 24 tutoriais de `Md-usario/tutorias/` são cópias dos arquivos de `docs/tutorias/`.

Para este checkpoint:

- Versione `docs/tutorias/` como fonte oficial.
- Não adicione `Md-usario/` ao Git automaticamente.
- Não publique arquivos pessoais de `Md-usario/`.
- Preserve a pasta e todos os seus arquivos.

Se algum arquivo dessa pasta já estiver rastreado, não o remova do índice automaticamente.

Apresente sua situação no relatório.

Não altere `.gitignore` para esconder arquivos já rastreados sem avaliar as consequências.

---

# 4. AUDITORIA DE SEGREDOS

Antes de publicar:

1. Inspecione os arquivos que entrarão nos commits.
2. Procure credenciais.
3. Procure tokens.
4. Procure senhas.
5. Procure chaves privadas.
6. Procure endpoints privados.
7. Procure arquivos pessoais.
8. Procure dumps de dados.
9. Procure backups.
10. Procure arquivos grandes ou binários inesperados.

Se encontrar dados sensíveis, interrompa a publicação.

Não imprima valores secretos no relatório.

Informe apenas o caminho e o tipo do problema.

---

# 5. INVENTÁRIO DAS ALTERAÇÕES

Classifique todos os arquivos alterados em grupos:

A. Código Rust.

B. Dados RON.

C. Testes.

D. Exemplos e rotas.

E. Assets e pipeline artístico.

F. Documentação técnica.

G. Tutoriais.

H. Configuração de desenvolvimento.

I. Arquivos pessoais ou protegidos.

J. Arquivos temporários.

K. Arquivos suspeitos ou desconhecidos.

Não deixe arquivos desconhecidos entrarem no commit sem análise.

Não descarte arquivos apenas porque parecem antigos.

---

# 6. VERIFICAÇÃO DO STAGING ORIGINAL

O staging anterior contém:

`protagonist_concept_v001.png`

Confirme o caminho completo e o conteúdo do arquivo.

Verifique se ele pertence ao projeto e se pode ser publicado.

Se for um asset legítimo e seguro, inclua-o no commit artístico correspondente.

Se houver dúvida sobre sua publicação, interrompa antes do push e peça minha decisão.

Não substitua o arquivo.

Não altere seu conteúdo.

Não remova o staging original sem necessidade.

---

# 7. ESTRATÉGIA DE COMMITS

Prefiro commits organizados por responsabilidade.

Sugestão:

1. `feat(world): consolidate castle regions and exploration`
2. `feat(combat): consolidate skills and transformations`
3. `feat(equipment): add expanded equipment and inventory`
4. `feat(town): add Ash Village and NPC commerce`
5. `feat(save): add Save V3 economy persistence`
6. `test: add world routes and regression coverage`
7. `docs: consolidate project documentation and tutorials`
8. `art: consolidate approved protagonist assets`

Esses nomes são sugestões.

Adapte os grupos ao conteúdo real.

Não invente commits para arquivos inexistentes.

Não divida alterações de modo que um commit dependa de arquivos ausentes de outro commit sem necessidade.

Se os sistemas estiverem muito interligados, prefira menos commits coerentes.

Não reescreva commits históricos.

Não utilize amend em commits anteriores.

---

# 8. STAGING CONTROLADO

Não utilize:

`git add .`

Não utilize:

`git add -A`

Não utilize:

`git add --all`

Adicione arquivos por caminhos explicitamente auditados.

Antes de cada commit:

1. Liste os arquivos staged.
2. Confira o diff staged.
3. Verifique alterações inesperadas.
4. Verifique arquivos protegidos.
5. Verifique segredos.
6. Confirme que nenhum arquivo temporário entrou.

Não utilize `git reset --hard`.

Não utilize `git clean`.

Não utilize comandos destrutivos.

---

# 9. VALIDAÇÃO ANTES DOS COMMITS

Execute sequencialmente:

```bash
export CARGO_BUILD_JOBS=2

cargo fmt --all -- --check
cargo check -j 2
cargo clippy --all-targets -j 2 -- -D warnings
cargo test -j 2
```

Execute também:

- Testes Python do pipeline artístico.
- Testes do hook.
- Testes dos tutoriais.
- Testes Save V3.
- Testes da economia.
- Rota da cidade.
- Autoplay.
- World Report.

Não execute comandos Cargo em paralelo.

Não execute `cargo clean`.

Não modifique `.cargo/config.toml`.

Se algum teste falhar, interrompa os commits e apresente o problema.

---

# 10. EXECUÇÃO DOS COMMITS

Depois da auditoria e dos testes:

1. Crie commits coerentes.
2. Use mensagens claras.
3. Não altere código apenas para organizar commits.
4. Não inclua arquivos protegidos sem autorização específica.
5. Registre o hash de cada commit.
6. Confirme o conteúdo de cada commit.
7. Verifique se não houve inclusão acidental de segredos.

Preserve a branch atual.

Não faça rebase.

Não faça merge.

Não force atualização de referências.

---

# 11. PUBLICAÇÃO REMOTA

Antes do push:

1. Confirme o endereço do remoto.
2. Confirme a branch de destino.
3. Confirme que a branch remota não exige sobrescrita.
4. Confirme que não existem commits remotos desconhecidos.
5. Confirme que os commits locais não contêm dados sensíveis.
6. Confirme que a branch não é `main` ou `master`, salvo autorização específica.

**Publique somente na branch de desenvolvimento atual.**

Se não existir remoto ou upstream adequado, não invente um destino.

Se a publicação exigir force push, interrompa.

Se a publicação exigir autenticação indisponível, interrompa.

Se houver divergência remota, interrompa.

Se houver risco de publicar arquivos pessoais, interrompa.

Não utilize `--force` nem `--force-with-lease`.

---

# 12. AUDITORIA APÓS O PUSH

Depois da publicação:

1. Confirme a branch local.
2. Confirme o HEAD.
3. Confirme a branch remota.
4. Confirme o upstream.
5. Confirme os hashes publicados.
6. Confirme o estado do working tree.
7. Confirme o staging.
8. Confirme que saves reais continuam intactos.
9. Confirme que arquivos protegidos não foram modificados.
10. Confirme que os tutoriais continuam disponíveis.
11. Confirme que os assets publicados continuam intactos.

Não declare a árvore limpa se arquivos não rastreados ou modificados continuarem presentes.

Separe claramente:

- Arquivos versionados.
- Arquivos ignorados.
- Arquivos pessoais preservados.
- Arquivos ainda pendentes.

---

# 13. RELATÓRIO FINAL

Entregue:

A. Branch inicial.

B. HEAD inicial.

C. Remoto e upstream.

D. Quantidade de arquivos auditados.

E. Arquivos excluídos da publicação.

F. Verificação de segredos.

G. Testes executados.

H. Commits criados.

I. Hash de cada commit.

J. Arquivos de cada commit.

K. Resultado do push.

L. Branch remota atualizada.

M. Estado final do Git.

N. Arquivos protegidos preservados.

O. Saves preservados.

P. Assets preservados.

Q. Tutoriais preservados.

R. Pendências.

S. Recomendação para a próxima versão.

---

# 14. REGRAS FINAIS

Você está autorizado a:

- Auditar as alterações.
- Preparar staging controlado.
- Criar commits organizados.
- Publicar a branch de desenvolvimento no remoto existente.

Você não está autorizado a:

- Apagar arquivos.
- Reescrever histórico.
- Executar force push.
- Alterar saves reais.
- Alterar backups.
- Publicar segredos.
- Publicar arquivos pessoais.
- Alterar proteções do Claude Code.
- Iniciar a V2.2.
- Criar uma nova branch sem autorização.
- Fazer merge em main/master.

Se qualquer condição de segurança não puder ser comprovada, interrompa a operação correspondente e solicite minha decisão.

**Objetivo final: consolidar e publicar todo o desenvolvimento válido até a V2.1.1, com histórico organizado, sem perder trabalho e sem expor arquivos pessoais.**

Depois de concluir, apresente o relatório completo e aguarde o próximo mega prompt.