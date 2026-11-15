# 33 — Fundos em parallax

> Situação: **funciona** (V2.3): duas camadas por região, geradas em código.
> **Pendente:** imagens próprias (PNG) para as camadas.

## 1. O que vamos fazer?

Entender e escolher o fundo em camadas (parallax) de uma região.

## 2. O que você precisa saber antes?

- O fundo é escolhido por região, em `backdrop` (tutorial 34):

| `backdrop` | Camada distante (move pouco) | Camada intermediária |
|---|---|---|
| `Castle` | Céu frio, torres, lua | Arcada de pilares e arcos |
| `Ruins` | Breu, arcos quebrados, estalactites | Colunas partidas |
| `Village` | Entardecer, castelo no morro, telhados | Fachadas com janelas acesas |

- A distante move 12% do quanto o mundo se move; a intermediária, 40%.
- As camadas são ladrilhos sem costura e sempre cobrem a tela inteira (testado).
- Elas ficam atrás de plataformas, personagens e inimigos.
- Pode desligar em `camera.ron`: `effects: (parallax: false)`.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/world/regions/*.ron` | `backdrop` da região |
| `src/world/scenery_art.rs` | Só leitura: desenho de cada camada |
| `assets/data/camera.ron` | Ligar/desligar |

## 4. Passo a passo

1. Escolha o `backdrop` da região (tutorial 34).
2. Rode o jogo ou a vitrine.
3. Ande de um lado ao outro: a camada distante quase não se move.

## 5. Exemplo completo

Só o parallax ligado (decoração, luzes e névoa desligadas), útil para conferir:

```ron
// exemplo-verificado: Camera
(effects: (parallax: true, decor: false, lights: false, fog: false))
```

## 6. Como testar?

1. `cargo test -j 2 --lib scenery` (sem costuras; cobre a tela em qualquer posição).
2. `cargo test -j 2 --test world_scenery_tests` (as camadas seguem a câmera).
3. `cargo run -j 2 --example world_visual_showcase`.

## 7. Como editar depois?

Troque o `backdrop` da região. Desenhos novos exigem mudar `scenery_art.rs`.

## 8. Problemas comuns

| Sintoma | Causa | Solução |
|---|---|---|
| Sem fundo | Região sem `visual` ou `parallax: false` | Tutorial 34 / ligue o efeito |
| Fundo claro demais | `sky` claro na região | Escureça `sky` |

## 9. Como desfazer com segurança?

`effects: (parallax: false)` em `camera.ron`, ou tire o `visual` da região.
