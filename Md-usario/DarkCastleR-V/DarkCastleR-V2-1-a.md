# DARK CASTLE RPG — AUTORIZAÇÃO OFICIAL
## V2.1.1 — Conclusão da Economia Persistente e Publicação dos Tutoriais

Você recebeu autorização para resolver as duas pendências bloqueantes da V2.1.

**Esta autorização é limitada às operações descritas abaixo.**

Não inicie a V2.2.

Não implemente funcionalidades adicionais fora do escopo.

---

# 1. AUTORIZAÇÃO — SAVE V3

Autorizo implementar o Save V3 exclusivamente para persistir corretamente:

- Saldo de moedas do jogador.
- Estoque limitado das lojas.
- Outros dados estritamente indispensáveis à consistência das transações comerciais.

Preserve todos os dados já existentes no Save V2.

## Requisitos obrigatórios

1. Criar uma versão tipada do Save V3.
2. Preservar a leitura dos saves V1 e V2.
3. Implementar migração V1 → V3.
4. Implementar migração V2 → V3.
5. Implementar leitura e escrita do V3.
6. Preservar inventário e equipamentos.
7. Preservar relíquias.
8. Preservar exploração e posições.
9. Preservar progressão.
10. Preservar estados persistentes de objetos.
11. Persistir dinheiro.
12. Persistir estoque limitado.
13. Preservar a política de backup existente.
14. Não escrever em saves reais durante os testes.
15. Não modificar saves históricos do proprietário.

## Compatibilidade

A migração deverá ocorrer em memória durante a leitura.

Não sobrescreva automaticamente um save antigo apenas por carregá-lo.

Quando ocorrer a primeira gravação em V3, preserve o backup apropriado da versão anterior.

A criação do backup e a substituição do save deverão ser seguras contra falhas de escrita.

Não substitua backups históricos existentes sem uma política explícita de preservação.

## Economia inicial

Se um save V1/V2 não tiver saldo registrado, utilize um valor inicial determinístico e documentado.

Não conceda dinheiro repetidamente a cada carregamento.

Defina o estado inicial do estoque de forma determinística.

## Transações

Depois de implementar o Save V3:

- Comprar deve descontar dinheiro.
- Comprar deve adicionar o item.
- Comprar deve reduzir estoque limitado.
- Vender deve remover o item.
- Vender deve aumentar dinheiro.
- Save deve persistir o resultado.
- Load deve restaurar o resultado.

Remova a restrição provisória `Cannot save after trading` somente quando a persistência estiver comprovadamente correta.

Não permita duplicação de itens ou dinheiro.

---

# 2. TESTES OBRIGATÓRIOS DO SAVE V3

Crie testes para:

1. Carregar Save V1.
2. Carregar Save V2.
3. Carregar Save V3.
4. Migrar V1 para V3.
5. Migrar V2 para V3.
6. Preservar inventário.
7. Preservar equipamentos.
8. Preservar relíquias.
9. Preservar exploração.
10. Preservar interações persistentes.
11. Comprar e salvar.
12. Comprar e carregar.
13. Vender e salvar.
14. Vender e carregar.
15. Comprar último item do estoque.
16. Salvar estoque esgotado.
17. Carregar estoque esgotado.
18. Tentar comprar item esgotado.
19. Tentar comprar sem dinheiro.
20. Impedir duplicação após F5/F9.
21. Preservar backups.
22. Tratar arquivo inválido.
23. Tratar versão desconhecida.
24. Garantir que load não escreva arquivos.
25. Garantir que testes não alterem `saves/`.

Utilize fixtures isoladas.

Não teste diretamente sobre saves reais.

---

# 3. AUTORIZAÇÃO — Md-usario/tutorias/

Autorizo modificar exclusivamente os mecanismos de proteção necessários para permitir:

- Criar `Md-usario/tutorias/`.
- Criar arquivos `.md` nessa pasta.
- Editar arquivos `.md` nessa pasta.
- Atualizar os tutoriais dessa pasta.

Não autorizo escrita em outros caminhos de `Md-usario/`.

Não autorizo escrita em `saves/`.

Não autorizo desabilitar o hook de proteção.

## Arquivos autorizados para ajuste

- `.claude/settings.json`.
- `.claude/hooks/guard.py`.
- `.claude/hooks/test_guard.py`.

Faça somente alterações pontuais e justificadas.

## Regra de segurança

A política deverá continuar bloqueando por padrão qualquer escrita em `Md-usario/`, exceto os arquivos Markdown dentro de `Md-usario/tutorias/`.

Não considere suficiente trocar uma regra ampla por uma lista incompleta de subpastas.

Se a configuração de permissões do Claude Code não conseguir expressar essa exceção de forma segura, preserve a negação existente e solicite uma solução alternativa antes de prosseguir.

Não afirme que o hook pode anular uma negação do Claude Code.

A solução final precisa funcionar nas duas camadas de proteção.

## Proteção contra caminhos alternativos

O hook deverá validar caminhos normalizados e impedir que a exceção seja utilizada para escrever fora da pasta autorizada.

Considere:

- Caminhos relativos.
- Caminhos absolutos.
- `..`.
- Links simbólicos.
- Renomeações.
- Movimentações de arquivos.
- Escritas indiretas.
- Arquivos com extensão diferente de `.md`.

Não permita que um caminho aparentemente dentro de `tutorias/` aponte para `saves/` ou outra pasta protegida.

---

# 4. TESTES OBRIGATÓRIOS DAS PERMISSÕES

Comprove:

**Permitido:**

- Criar `Md-usario/tutorias/`.
- Criar `Md-usario/tutorias/00_LEIA_PRIMEIRO.md`.
- Editar Markdown dentro da pasta.
- Criar novos tutoriais Markdown.

**Bloqueado:**

- Editar `Md-usario/inicio.md`.
- Editar arquivos existentes de `DarkCastleR-F/`.
- Editar arquivos existentes de `DarkCastleR-V/`.
- Criar Markdown diretamente na raiz de `Md-usario/`.
- Escrever arquivos `.rs` em `Md-usario/tutorias/`.
- Escrever em `saves/`.
- Escrever usando `../` para escapar.
- Escrever por links simbólicos que apontem para fora.
- Mover arquivos protegidos.
- Renomear arquivos protegidos.

Não execute testes destrutivos sobre arquivos reais.

Utilize diretórios temporários isolados.

---

# 5. PUBLICAÇÃO DOS 24 TUTORIAIS

Os 24 tutoriais já foram criados e validados em:

`docs/tutorias/`

Após confirmar que as permissões estão corretas, copie-os para:

`Md-usario/tutorias/`

Não mova os originais até confirmar a integridade da cópia.

Preserve:

- Nomes.
- Conteúdo.
- Exemplos.
- Links.
- Estrutura.
- Formatação.

Compare hashes entre origem e destino.

Se houver links relativos que precisem mudar, documente e valide cada ajuste.

Não altere os demais arquivos de `Md-usario/`.

## Validação

Execute novamente os testes de tutoriais usando o caminho definitivo.

Confirme:

- 24 arquivos presentes.
- Índice funcional.
- Links internos válidos.
- Exemplos RON válidos.
- Exemplos de salas válidos.
- Testes de criação de região passando.
- Nenhum comando Git destrutivo recomendado.

---

# 6. ROTA REAL DA CIDADE COM SAVE V3

Atualize a rota da V2.1 para demonstrar:

1. Chegar à Vila das Cinzas.
2. Conversar com o comerciante.
3. Abrir a loja.
4. Comprar um equipamento.
5. Confirmar desconto do dinheiro.
6. Confirmar redução do estoque.
7. Equipar o item.
8. Vender um item permitido.
9. Confirmar aumento do saldo.
10. Salvar com F5.
11. Mudar de sala.
12. Carregar com F9.
13. Confirmar dinheiro restaurado.
14. Confirmar estoque restaurado.
15. Confirmar inventário restaurado.
16. Confirmar equipamento restaurado.
17. Confirmar ausência de duplicação.
18. Retornar ao castelo.

A rota deverá utilizar entradas reais.

Não utilize reposicionamentos artificiais.

Não injete dinheiro ou itens diretamente durante a rota real.

Se o saldo inicial não permitir uma compra, forneça uma fonte legítima de moedas dentro do jogo.

---

# 7. REGRESSÃO COMPLETA

Execute:

```bash
export CARGO_BUILD_JOBS=2

cargo fmt --all -- --check
cargo check -j 2
cargo clippy --all-targets -j 2 -- -D warnings
cargo test -j 2
```

Execute sequencialmente, nunca em paralelo.

Execute também:

- Testes Python do pipeline artístico.
- Testes do hook de proteção.
- Testes dos tutoriais.
- World Report.
- World Town Route.
- World Equipment Route.
- World Spectral Route.
- World Secrets Route.
- World Main Route.
- World Vertical Slice.
- Autoplay.
- Visual Check.

Não execute `cargo clean`.

Não altere `.cargo/config.toml`.

Não faça staging, commit ou push.

---

# 8. AUDITORIA FINAL

Ao terminar:

1. Confirme branch e HEAD.
2. Confirme staging inalterado.
3. Confirme hashes dos arquivos protegidos.
4. Confirme hashes dos assets publicados.
5. Confirme saves reais intactos.
6. Confirme `target/verify/` intacto.
7. Confirme `target/f4-stage/` intacto.
8. Confirme os 24 tutoriais no local definitivo.
9. Confirme Save V3 funcional.
10. Confirme migração V1/V2.
11. Confirme rota real.
12. Confirme ausência de duplicação econômica.

As mudanças explicitamente autorizadas em `.claude/settings.json`, nos dois arquivos do hook e nos novos tutoriais deverão ser identificadas separadamente das alterações não autorizadas.

---

# 9. RELATÓRIO FINAL

Entregue:

A. Auditoria inicial.

B. Alterações no Save V3.

C. Migração V1/V2.

D. Persistência do dinheiro.

E. Persistência do estoque.

F. Segurança das transações.

G. Backups.

H. Alterações de permissões.

I. Testes do hook.

J. Publicação dos tutoriais.

K. Validação dos 24 tutoriais.

L. Rota real da cidade.

M. Testes Rust.

N. Testes Python.

O. Validação visual.

P. Performance.

Q. Estado final do Git.

R. Hashes protegidos.

S. Pendências.

T. Próximos passos recomendados.

Não declare sucesso sem evidência.

---

# 10. INSTRUÇÃO FINAL

Você está autorizado a concluir a V2.1 com:

1. Save V3 e economia persistente.
2. Migração segura de saves antigos.
3. Exceção restrita para `Md-usario/tutorias/`.
4. Publicação e validação dos 24 tutoriais.

Não inicie a V2.2.

Não modifique saves reais.

Não altere assets publicados.

Não faça staging.

Não crie commits.

Não faça push.

Se a configuração de segurança não permitir uma exceção realmente restrita, não enfraqueça a proteção. Apresente o impedimento e aguarde nova autorização.

Quando terminar, entregue o relatório completo para minha avaliação.