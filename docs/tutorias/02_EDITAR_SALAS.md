# 02 — Editar salas

> Situação: **funciona**.

## 1. O que vamos fazer?

Mudar uma sala que já existe: acrescentar uma plataforma, mudar a cor,
mover um inimigo ou um baú, sem quebrar nada.

## 2. O que você precisa saber antes?

- Procure a sala pelo `id`: `grep -n "id: \"av_square\"" assets/world/regions/*.ron`
- As rotas automáticas (tutorial 16) andam por algumas salas do castelo. Mexer
  em plataformas desse caminho pode fazê-las falhar: **teste sempre**.
- Em salas que já existem, **não mude IDs** (de sala, porta, baú, segredo): os
  saves guardam esses IDs.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/world/regions/ash_village.ron` | Exemplo: a praça da vila (`av_square`) |

## 4. Passo a passo

1. Copie o arquivo antes: `cp assets/world/regions/ash_village.ron ~/copia_ash_village.ron`
2. Abra o arquivo e encontre a sala `av_square`.
3. Para **acrescentar uma plataforma**, coloque uma linha nova em `platforms: [ ... ]`
   (veja a seção 5).
4. Para **mudar a cor do fundo**, troque os 3 números de `background`.
5. Para **mover algo**, mude `position` (inimigos, coletas) ou `area.min`
   (baús, alavancas, paredes).
6. Salve e teste.

## 5. Exemplo completo

Uma plataforma acima do poço da praça:

```ron
// exemplo-verificado: Plataforma@av_square
(min: (580.0, 150.0), size: (120.0, 16.0), one_way: true),
```

A sala inteira depois da mudança (compare com o arquivo real):

```ron
// exemplo-verificado: Sala
(
    id: "av_square",
    name: "Cinder Square",
    region: "ash_village",
    role: Optional,
    map: (x: 5, y: 1, w: 1, h: 1),
    background: (0.11, 0.08, 0.06),
    platforms: [
        (min: (560.0, 40.0), size: (160.0, 30.0)),
        (min: (580.0, 150.0), size: (120.0, 16.0), one_way: true),
    ],
    doors: [
        (id: "to_gate_road", side: Left, to: (room: "av_gate_road", door: "to_square")),
        (id: "to_merchant_row", side: Right, to: (room: "av_merchant_row", door: "to_square")),
    ],
)
```

## 6. Como testar?

1. `cargo run -j 2 --example world_report` → `Errors (0):`.
2. `cargo run -j 2` e vá até a sala.
3. Se mexeu no castelo, rode as rotas (tutorial 16).

## 7. Como editar depois?

Repita os passos. Mudou de ideia? Apague a linha que você acrescentou.

## 8. Problemas comuns

| Problema | Causa | Solução |
|---|---|---|
| `invalid data ... expected ...` | Vírgula ou parêntese faltando | Compare com o exemplo |
| `ObjectOutOfRoom` | Objeto fora da sala | `area` dentro de 0..largura e 0..altura |
| Plataforma "flutua" no lugar errado | `min` é o canto **inferior esquerdo** | Ajuste `min` |
| Uma rota automática falhou | Você mudou o caminho dela | Desfaça ou ajuste a mudança |

## 9. Como desfazer com segurança?

`cp ~/copia_ash_village.ron assets/world/regions/ash_village.ron`, ou apague
só as linhas que você acrescentou. Não use `git restore` (o arquivo tem
trabalho ainda não salvo no Git).
