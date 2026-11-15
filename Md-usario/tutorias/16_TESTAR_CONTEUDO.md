# 16 — Testar conteúdo

> Situação: **funciona**.

## 1. O que vamos fazer?

Conhecer os comandos que conferem se o que você criou está certo, do mais
rápido ao mais completo.

## 2. O que você precisa saber antes?

- Sempre com `-j 2` e **um comando `cargo` por vez** (o computador já travou
  por falta de memória).
- A primeira compilação demora; as seguintes são rápidas.
- Os testes **nunca** gravam em `saves/`.

## 3. Quais arquivos vamos utilizar?

Nenhum arquivo é editado. Os resultados aparecem no terminal e em `target/`.

## 4. Passo a passo

| Ordem | Comando | Confere | Demora |
|---|---|---|---|
| 1 | `cargo run -j 2 --example world_report` | Mundo: portas, salas, segredos, véus | segundos |
| 2 | `cargo test -j 2 --test data_tests` | Itens, inimigos, NPCs, lojas | segundos |
| 3 | `cargo test -j 2 --test tutorial_examples_tests` | Exemplos destes tutoriais | segundos |
| 4 | `cargo run -j 2` | Você joga e olha | — |
| 5 | `cargo test -j 2` | Todos os testes do projeto | alguns minutos |
| 6 | `cargo run -j 2 --example world_main_route` | Rota principal com janela e capturas | ~1 min |
| 7 | `cargo run -j 2 --example world_town_route` | Rota da vila com janela e capturas | ~1 min |

Outras rotas com janela: `world_secrets_route`, `world_spectral_route`,
`world_equipment_route`, `autoplay`, `visual_check`. As capturas ficam em
`target/<nome>/`.

## 5. Exemplo completo

Depois de criar a região do tutorial 03:

```bash
cargo run -j 2 --example world_report
cargo test -j 2 --test data_tests
cargo test -j 2 --test tutorial_examples_tests
cargo run -j 2
```

Resultados esperados:
- `world_report`: `Errors (0):` e `Warnings (0):`;
- testes: `test result: ok.`;
- jogo: abre a janela. Se algo estiver errado, **não abre** e o terminal diz o quê.

## 6. Como testar?

Este tutorial é o próprio teste. Para conferir que tudo funciona:
`cargo test -j 2 --test tutorial_examples_tests`.

## 7. Como editar depois?

Não há o que editar: sempre que mudar um arquivo, rode de novo os comandos 1 a 3.

## 8. Problemas comuns

| Problema | Solução |
|---|---|
| `cargo: command not found` | `source "$HOME/.cargo/env"` |
| O computador fica lento | Feche programas; nunca rode dois `cargo` juntos; sempre `-j 2` |
| `test result: FAILED` | Leia a linha `panicked at ...` e a mensagem logo abaixo; veja o tutorial 17 |
| Rota falhou com `timeout in ...` | Algo no caminho mudou (plataforma, porta); veja `target/<rota>/route.log` |

## 9. Como desfazer com segurança?

Testes não alteram nada. Para desfazer conteúdo, veja a seção 9 do tutorial
correspondente.
