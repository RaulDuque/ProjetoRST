# 32 — Decorar uma sala

> Situação: **funciona** (V2.3), com arte **provisória gerada em código**.
> Decoradas: `vigil_gates_causeway`, `fg_stair` e `av_square` (pilotos).
> **Pendente:** arte final e as demais salas.

## 1. O que vamos fazer?

Pôr pilares, arcos, janelas, tochas, estandartes e correntes numa sala sem
mudar nada da jogabilidade.

## 2. O que você precisa saber antes?

- A decoração fica no campo `decor` da sala, em `assets/world/regions/*.ron`.
- Ela **nunca colide**: não é chão, parede nem porta. Plataformas continuam em `platforms`.
- Cores vêm da região (tutorial 34); a pedra da decoração é sempre mais
  escura que as plataformas, para ninguém tentar pisar nela.
- `at` é o ponto de apoio, em coordenadas da sala (chão em `y = 40`):

| Tipo | `at` é | Tamanho |
|---|---|---|
| `Pillar(height: ...)` | base | altura |
| `Arch(width: ..., height: ...)` | base (nicho de parede, não é passagem) | largura e altura |
| `TallWindow(height: ...)` | base | altura; tem luz |
| `Torch` | suporte na parede | tem luz que oscila |
| `Statue` | base | — |
| `Rubble(width: ...)` | base | largura |
| `IronGrate(width: ..., height: ...)` | base | largura e altura |
| `Shopfront(width: ..., awning: (r, g, b))` | base | largura e cor do toldo |
| `Chain(length: ...)` | ponto no teto | comprimento |
| `DeadVine(length: ...)` | ponto no teto/parede | comprimento |
| `Banner(length: ..., color: (r, g, b))` | ponto de cima | comprimento e cor |
| `Lantern` | ponto de cima | tem luz que oscila |

- `layer: Front` (na frente do personagem, translúcido) só para `Chain` e `DeadVine`.
- `flip: true` espelha.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/world/regions/vigil_gates.ron` | Exemplo real: `vigil_gates_causeway` |
| `src/world/scenery.rs` | Só leitura: tipos e validação |

## 4. Passo a passo

1. Abra o arquivo da região e ache a sala.
2. Antes de `doors`, acrescente `decor: [ ... ],`.
3. Ponha coisas altas (pilares) longe de plataformas finas, e nada sobre portas.
4. Teste.

## 5. Exemplo completo

Uma tocha e um pilar:

```ron
// exemplo-verificado: Decoracao
(kind: Torch, at: (560.0, 300.0))
```

```ron
// exemplo-verificado: Decoracao
(kind: Pillar(height: 640.0), at: (420.0, 40.0))
```

Uma corrente na frente do personagem:

```ron
// exemplo-verificado: Decoracao
(kind: Chain(length: 150.0), at: (880.0, 680.0), layer: Front)
```

## 6. Como testar?

1. `cargo run -j 2 --example world_report` (0 erros).
2. `cargo test -j 2 --test world_scenery_tests` (física igual com e sem decoração).
3. `cargo run -j 2` e entre na sala.

## 7. Como editar depois?

Mude `at`, o tamanho ou apague a linha. Nada é salvo no save.

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `BadScenery(Outside ...)` | `at` fora da sala | Corrija a posição |
| `BadScenery(Size ...)` | Tamanho menor que 1 ou maior que a sala | Ajuste o tamanho |
| `BadScenery(Front ...)` | Algo grosso em `layer: Front` | Use `Front` só em `Chain`/`DeadVine` |
| Algo parece uma plataforma | Peça horizontal larga na altura de pulo | Mova ou troque a peça |

## 9. Como desfazer com segurança?

Apague o bloco `decor: [...]` da sala: ela volta ao visual sem decoração.
