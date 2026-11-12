---
name: test-and-validate
description: Executa a validação completa do Dark Castle RPG (fmt, clippy, testes, check, inicialização e autoplay) e relata problemas com saída real. Use antes de declarar qualquer tarefa concluída ou de commitar.
---

# /test-and-validate

Execute na raiz do projeto, **nesta ordem**, e não pare no primeiro sucesso.
Se `cargo` não existir no PATH: `source "$HOME/.cargo/env"`.

1. `cargo fmt --check` — se falhar, rode `cargo fmt` e informe quais arquivos mudaram.
2. `cargo clippy --all-targets` — zero avisos é o padrão. Liste cada aviso com arquivo:linha.
3. `cargo test` — informe a contagem por binário de teste e cada falha com a mensagem.
4. `cargo check` — confirma o build do binário.
5. **O projeto inicia?**
   - Com sessão gráfica (`$DISPLAY` ou `$WAYLAND_DISPLAY` definidos):
     `timeout 15 cargo run` — sucesso = código 124 (encerrado pelo timeout) sem `panic`/`ERROR` no log.
     Depois `cargo run --example autoplay` — confira as linhas `[autoplay] command:` (esperado: `Executed crescent_slash`, `Executed ember_bolt`, `NotLearned rising_edge`), `damage:`, `defeated:`, `Game saved`, e abra 1–2 PNGs de `target/autoplay/` para inspeção visual.
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
