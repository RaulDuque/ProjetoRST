---
name: test-and-validate
description: Executa a validação completa do Dark Castle RPG (fmt, clippy, testes, check, inicialização e autoplay) e relata problemas com saída real. Use antes de declarar qualquer tarefa concluída ou de commitar.
---

# /test-and-validate

Execute na raiz do projeto, **nesta ordem**, e não pare no primeiro sucesso.
Se `cargo` não existir no PATH: `source "$HOME/.cargo/env"`.
Siga "Compilação segura" no `CLAUDE.md`: use o `target/` da raiz e os 4 jobs configurados, e confira `free -h` se o build for começar do zero.

1. `cargo fmt --check` — se falhar, rode `cargo fmt` e informe quais arquivos mudaram.
2. `cargo clippy --all-targets` — zero avisos é o padrão. Liste cada aviso com arquivo:linha.
3. `cargo test` — informe a contagem por binário de teste e cada falha com a mensagem.
4. `cargo check` — confirma o build do binário.
5. **O projeto inicia?**
   - Com sessão gráfica (`$DISPLAY` ou `$WAYLAND_DISPLAY` definidos):
     1. **Compilação:** `cargo build --bins --example autoplay` (jogo e autoplay). Só depois disso o `timeout` mede a execução, e não a compilação.
     2. **Aplicação iniciada:** `timeout 15 cargo run`. Código 124 sem `panic`/`ERROR` no log prova apenas que o processo ficou 15 s de pé; não prova que a janela apareceu nem que o jogo funciona.
     3. **Teste executado e asserções aprovadas:** `cargo run --example autoplay`. Ele grava só em `target/autoplay/saves/` (recusa iniciar se o isolamento não estiver ativo) e confere jogador, sala, movimento, ataque básico, Crescent Slash, Ember Bolt, Mending Light, custos de MP/Hearts, Rising Edge bloqueado, save/load isolados e `saves/` intacto. Sucesso = código 0 **e** a linha `[autoplay] RESULT: PASS`. Código ≠ 0 ou `RESULT: FAIL`: relate cada linha `[autoplay] FAIL`. Linhas `INFO` não reprovam (ex.: D-GAME-003 pendente).
     4. **Janela criada:** abra 1–2 PNGs de `target/autoplay/` para inspeção visual. Screenshot não substitui as verificações.
   - Sem sessão gráfica: diga explicitamente que a execução visual não pôde ser testada **por limitação do ambiente** e que `tests/app_smoke_tests.rs` (headless) cobriu a inicialização do ECS.
6. Diferencie claramente: erro de compilação × teste falhando × problema de ambiente (GPU, display, bibliotecas do sistema).

## Relatório

```
fmt:      ok | arquivos ajustados: ...
clippy:   ok | N avisos (lista)
test:     X passados, Y falhos (lista)
check:    ok | erro
execução: ok | falhou (log) | não testável (motivo)
autoplay: ok (resumo do log) | falhou | não executado (motivo)
```
Nunca declare sucesso de uma etapa que não foi executada.
