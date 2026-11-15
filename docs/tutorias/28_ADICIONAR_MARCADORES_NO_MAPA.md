# 28 — Marcadores de NPCs e lojas no mapa

> Situação: **funciona** (automático). **Pendente:** marcador de ferreiro
> (não há serviço de ferreiro) e marcadores de missão (não há sistema de
> missões).

## 1. O que vamos fazer?

Entender quando o mapa (tecla **M**) mostra lojas e NPCs, e como fazer um NPC
novo aparecer lá.

## 2. O que você precisa saber antes?

- Não há arquivo de marcadores: o mapa lê os NPCs de `assets/data/npcs/`.
- Cores: **violeta** = NPC com loja (`shop`); **rosa** = outro NPC;
  **laranja** = baú fechado; cinza = baú aberto; dourado = segredo achado.
- Um marcador só aparece em sala **visitada** (a mesma regra dos baús): o
  mapa não revela NPCs de salas que você não visitou.
- Um marcador por NPC, sempre dentro da caixa da sala.
- A legenda aparece embaixo do mapa quando há marcadores de NPC.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/data/npcs/*.ron` | Posição (`position`) e loja (`shop`) do NPC |
| `src/ui/world_map.rs` | Só leitura: desenho do mapa |

## 4. Passo a passo

1. Crie o NPC (tutorial 18), com `room` e `position`.
2. Se vende, ligue a loja (`shop`, tutorial 20): o marcador fica violeta.
3. Jogue até visitar a sala e aperte **M**.

## 5. Exemplo completo

O NPC do tutorial 18 já gera um marcador violeta na sala `ac_crypt_stair`
porque tem `shop`. Trecho que decide a cor:

```ron
// exemplo-verificado: Npc
(
    id: "crypt_watcher",
    name: "Crypt Watcher",
    kind: Informant,
    room: "ac_crypt_stair",
    position: (300.0, 63.0),
    size: (26.0, 46.0),
    color: (0.4, 0.42, 0.5),
    greetings: [(node: "morrow_greet")],
),
```

Sem `shop` → marcador **rosa** no ponto `x = 300` da sala.

## 6. Como testar?

1. `cargo test -j 2 --lib world_map` (marcadores sempre dentro da sala).
2. `cargo run -j 2 --example world_town_route`: `t08_village_on_the_map.png`.

## 7. Como editar depois?

Mudar `position` move o marcador; tirar `shop` troca violeta por rosa.

## 8. Problemas comuns

| Sintoma | Causa | Solução |
|---|---|---|
| NPC não aparece no mapa | Sala ainda não visitada | Visite a sala |
| Mapa não abre | Painel fechado | Aperte **M** |

## 9. Como desfazer com segurança?

Os marcadores saem dos dados dos NPCs: desfazer o NPC desfaz o marcador.
