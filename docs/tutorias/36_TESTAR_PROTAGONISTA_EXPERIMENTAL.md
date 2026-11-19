# 36 — Testar o novo protagonista (modo experimental)

> Situação: **experimental** (N3.8). O protagonista padrão do jogo continua sendo o V2.2.

## 1. O que vamos fazer?

Abrir o jogo com o novo protagonista (cabelo prateado, camisa cinza): parado na pose aprovada da N3.5 e correndo com os 6 quadros aprovados da N3.6, a 15 quadros por segundo. É só um teste visual. O jogo, os itens e o save continuam iguais.

## 2. O que você precisa saber antes?

- O modo é ligado por uma **variável de ambiente**: um texto que você escreve antes do comando, na mesma linha.
- Só **parado** e **correndo** têm arte aprovada. Saltar, atacar, defender, esquivar, sofrer dano e morrer mostram a pose parada, e um aviso no canto da tela diz `PLACEHOLDER` com o nome da animação que falta. Isso é esperado.
- A arma e o escudo **não aparecem** neste modo (ainda não há pontos de encaixe medidos para a arte nova). Eles continuam equipados e funcionando.
- Nada é salvo de forma diferente: o save é o mesmo nos dois modos.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| nenhum (só o terminal) | ligar o modo |
| `art-src/n3-8-production-run/stage/` | arte usada pelo modo (não edite os PNG) |
| `docs/development/n3_8/` | relatórios e GIFs de evidência |

## 4. Passo a passo

1. Abra o terminal na pasta do projeto.
2. Digite `DARK_CASTLE_PROTAGONIST=n3 cargo run` e tecle Enter.
3. Espere a janela abrir. No canto inferior direito aparece `EXPERIMENTAL PROTAGONIST (N3.8)`.
4. Ande e corra para os dois lados. Pule, ataque, defenda e esquive para ver os avisos de `PLACEHOLDER`.
5. Feche a janela para sair.

## 5. Exemplo completo

```bash
# jogo normal (protagonista V2.2)
cargo run

# novo protagonista experimental
DARK_CASTLE_PROTAGONIST=n3 cargo run

# o mesmo que o normal, escrito por extenso
DARK_CASTLE_PROTAGONIST=production cargo run
```

## 6. Como testar?

- O selo amarelo aparece no canto inferior direito.
- Correndo, o personagem troca de quadro 15 vezes por segundo e anda na mesma velocidade de sempre.
- Para ver tudo gravado automaticamente: `cargo run --example n3_8_production_run`. Ele termina com `RESULT: PASS` e grava as imagens em `target/n3_8_production_run/`.
- Testes automáticos: `cargo test -j 2 --test n3_8_production_run_tests`.

## 7. Como editar depois?

Não há nada para configurar neste modo. A arte aprovada não deve ser editada à mão: um teste confere que os PNG são idênticos aos originais. Novas animações vão entrar pela N4, depois da sua aprovação.

## 8. Problemas comuns

| Sintoma | Causa | O que fazer |
|---|---|---|
| O jogo abre com o protagonista antigo e o terminal diz `unknown DARK_CASTLE_PROTAGONIST value` | valor escrito errado | use exatamente `n3` |
| O terminal diz `experimental protagonist stage missing` | a pasta `art-src/n3-8-production-run/stage/` sumiu | recupere a pasta pelo Git, ou peça ajuda |
| Não vejo a espada | esperado neste modo | ela continua equipada; só não é desenhada |
| O personagem morto fica em pé | esperado: ainda não há animação de morte | é um aviso visual, não um erro |
| No Windows o comando não funciona | a forma de definir variáveis é outra | no PowerShell: `$env:DARK_CASTLE_PROTAGONIST="n3"; cargo run` |

## 9. Como desfazer com segurança?

Basta rodar `cargo run` sem a variável: o jogo volta ao protagonista V2.2. Nada fica gravado. No Linux e no macOS, a variável escrita antes do comando só vale para aquele comando. No PowerShell ela vale até fechar o terminal; para desligar antes, use `Remove-Item Env:DARK_CASTLE_PROTAGONIST`.
