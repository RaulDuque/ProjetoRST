# 15 — Editar mapa e exploração

> Situação: **funciona**. O mapa é **gerado** a partir das salas: não existe
> um arquivo de mapa separado para desenhar. **Pendente:** ícones de lojas e
> NPCs no mapa.

## 1. O que vamos fazer?

Entender como o mapa é montado, mudar a posição de uma sala na grade, a cor
de uma região, e entender a porcentagem de exploração.

## 2. O que você precisa saber antes?

- Cada sala tem `map: (x, y, w, h)`: canto inferior esquerdo `(x, y)` na
  grade e tamanho `w × h` em células. `y` cresce para **cima**; `x` pode ser negativo.
- **Duas salas não podem ocupar a mesma célula**, e portas só ligam salas
  que se tocam.
- A cor de cada sala no mapa vem de `region.color`.
- O minimapa fica no canto superior direito, sempre ligado; `M` esconde/mostra.
- **Exploração:** `Map N%` = salas visitadas + segredos achados, sobre o total.
  Salas vizinhas de uma visitada aparecem como contorno (descobertas, não
  visitadas). Nada é revelado de uma vez só.
- Mapa atual (y=1): x=0 Postern, 1 Shrine, 2 Well, 3 Watch Shaft (x=3, y=0..2),
  4 a 9 a Vila das Cinzas. Livres para o exemplo: (10,1), (11,1), (12,1).

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/world/regions/*.ron` | `map` de cada sala e `color` da região |
| `target/world_report/world.dot` | Gerado pelo `world_report`: o grafo das portas |

## 4. Passo a passo

1. Rode `cargo run -j 2 --example world_report` e veja as salas existentes.
2. Para **mover uma sala**, mude `map` dela **e** confira as vizinhas: as
   portas precisam continuar se tocando (senão `Misaligned`).
3. Para **mudar a cor**, edite `region.color`.
4. Teste.

## 5. Exemplo completo

A sala inicial da cripta, ao lado da casa da Wenna (x=9) — note `x: 10`:

```ron
// exemplo-verificado: Sala
(
    id: "ac_crypt_stair",
    name: "Crypt Stair",
    region: "ashen_crypt",
    role: Optional,
    map: (x: 10, y: 1, w: 1, h: 1),
    background: (0.07, 0.06, 0.06),
    doors: [
        (id: "to_cottage", side: Left, to: (room: "av_cottage", door: "to_crypt")),
    ],
)
```

## 6. Como testar?

1. `cargo run -j 2 --example world_report` → `Errors (0):`. O relatório fica
   em `target/world_report/report.txt`.
2. `cargo run -j 2`: ande e olhe o minimapa; o texto mostra `Map N%`,
   `rooms visitadas/total`, `secrets` e a porcentagem da região atual.
3. `cargo test -j 2 --test world_graph_tests`.

## 7. Como editar depois?

Mude `map`/`color` e rode o `world_report` de novo.

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `Misaligned` | Porta entre salas que não se tocam | Ajuste `map` ou `cell` |
| `DoorCellOutOfRange` | `cell` além do tamanho | Ajuste `cell` |
| Sala sobreposta a outra no mapa | Células repetidas | Escolha células livres |
| `world_graph_tests` falha no total de salas | Você acrescentou salas ao jogo real | Normal: o teste conta as salas reais; avise o desenvolvedor |

## 9. Como desfazer com segurança?

Volte os valores antigos de `map`/`color` (ou a cópia do arquivo).
