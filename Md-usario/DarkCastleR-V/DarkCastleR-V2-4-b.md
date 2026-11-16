# DARK CASTLE RPG — V2.4
## Checkpoint dos Blocos A e B1 — Etapas 0 a 5

Aprovo a direção técnica apresentada no relatório dos Blocos A e B1.

Esta tarefa é exclusivamente para auditar, validar e registrar o checkpoint das etapas 0 a 5.

**Não implemente as etapas 6 a 10.**

## 1. Decisões aprovadas

- Utilizar a opção de commits sem worktree.
- Manter `StrikeCancelPolicy::Preserve` como padrão.
- Manter `RemoveOnCancel` disponível, mas desativado nos dados atuais.
- Não aplicar novos valores de startup, cancelamento, controle aéreo ou reacerto às armas existentes.
- Preservar integralmente a Iron Sword.
- Não alterar o Save V3.
- Não alterar o comportamento atual das técnicas especiais.

## 2. Auditoria do Git

Confirme:

- Branch `art/v1.2-protagonist`.
- HEAD esperado `7bc9b5e`.
- Arquivos modificados e não rastreados.
- Nenhum arquivo protegido incluído.
- Nenhuma alteração involuntária em RON.

Use `git diff --check` e examine os diffs de código e documentação.

Não descarte alterações locais.

## 3. Revisão técnica

Revise especialmente:

1. Se `StrikeCancelPolicy` tem uma única fonte de configuração.
2. Se `RemoveOnCancel` remove somente as hitboxes corretas.
3. Se os registros de `RehitTimers` são eliminados junto com as entidades correspondentes.
4. Se o controle aéreo é independente da taxa de quadros.
5. Se a orientação das hitboxes permanece correta.
6. Se `turn_window` e `startup` possuem validações coerentes.
7. Se os novos campos opcionais mantêm o comportamento legado.
8. Se nenhuma nova lógica permite contornar custos ou cooldowns.
9. Se o reconhecimento de comandos permanece funcional.
10. Se a documentação corresponde à implementação real.

Se encontrar defeitos, corrija apenas os relacionados às etapas 0 a 5 e acrescente testes.

## 4. Testes

Execute sequencialmente, com no máximo dois jobs Cargo:

- `cargo fmt --all -- --check`
- `cargo check -j 2`
- `cargo clippy --all-targets -j 2 -- -D warnings`
- `cargo test -j 2`
- Autoplay e verificações relevantes de gameplay.

Compare os resultados com a referência de 483 testes e autoplay 23/23.

Não trate a simples passagem da suíte como prova absoluta de ausência de regressões.

## 5. Preparação dos commits

Não crie worktree.

Prefira dois commits:

1. `feat(combat): implement V2.4 strike feel foundations`
2. `docs(combat): document V2.4 blocks A and B1`

Os testes necessários para validar a implementação devem acompanhar o commit de código.

Somente se os testes de caracterização puderem ser separados de forma limpa e realmente passarem no estado anterior, considere um terceiro commit `test(combat)`.

Não crie estados intermediários que não foram compilados ou testados.

Use staging explícito, arquivo por arquivo.

Não use `git add .` nem `git add -A`.

## 6. Autorização de Git

**Autorizo os commits locais exclusivamente para o checkpoint das etapas 0 a 5, desde que a auditoria e todas as validações passem.**

Antes de cada commit, apresente no relatório de execução:

- Arquivos que serão incluídos.
- Justificativa do agrupamento.
- Resultado das validações.
- Confirmação de ausência de arquivos protegidos.

Se houver qualquer divergência relevante, não execute o commit e solicite aprovação.

Não execute push.

Não use reset, clean, rebase ou force push.

## 7. Relatório final

Entregue:

A. Estado inicial do Git.

B. Auditoria técnica.

C. Defeitos encontrados e correções.

D. Testes executados.

E. Comparação com o checkpoint anterior.

F. Arquivos incluídos em cada commit.

G. Hash de cada commit criado.

H. Estado final do Git.

I. Riscos restantes.

J. Recomendações específicas para as etapas 6 e 7.

Não inicie o Bloco B2.

Aguarde minha revisão antes de alterar o sistema de dano ou recuo.