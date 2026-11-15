# 35 — Testar cenários visuais

> Situação: **funciona** (V2.3).

## 1. O que vamos fazer?

Conferir câmera, zoom, parallax, decoração, luzes e névoa com testes e
capturas antes/depois.

## 2. O que você precisa saber antes?

- A vitrine `world_visual_showcase` é uma **bancada**: ela põe o personagem
  em pontos fixos (como um carregamento). As rotas reais (`world_*_route`)
  não fazem isso.
- Modos: `before` (vista 720, sem efeitos: como na V2.2), `after` (o jogo
  como enviado) e `zoom720` … `zoom360` (varredura de zoom).
- Mesma sala, posição, estado e resolução em `before_*` e `after_*`.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `examples/world_visual_showcase.rs` | A bancada |
| `tests/world_scenery_tests.rs` | Testes automáticos do cenário |
| `target/world_visual_showcase/` | Capturas |
| `target/tmp/scenery_v23_perf.txt` | Custo medido com e sem efeitos |

## 4. Passo a passo

1. `cargo test -j 2 --test world_scenery_tests --test world_camera_tests`.
2. `cargo run -j 2 --example world_visual_showcase`.
3. Compare cada `before_X.png` com `after_X.png`.
4. Veja `motion_*` (corrida, pulo, queda) e `window_*` (outras janelas).

## 5. Exemplo completo

Para separar um problema de cenário de um de câmera, desligue os efeitos:

```ron
// exemplo-verificado: Camera
(effects: (parallax: false, decor: false, lights: false, fog: false))
```

O que conferir nas capturas:

| Captura | Esperado |
|---|---|
| `after_causeway` | Castelo: arcada ao fundo, pilares, tochas; plataformas claras |
| `after_stair_*` | Ruínas: névoa verde, correntes na frente, plataformas legíveis |
| `after_square` | Vila: casas com janelas acesas, lanterna, NPC visível |
| `motion_jump_apex` | A tela não sobe num pulo comum |
| `motion_fall` | A tela olha para baixo na queda |
| `window_1600x900`, `window_1024x768` | HUD do mesmo tamanho; vista dentro da sala |

## 6. Como testar?

A bancada termina com `[world_visual_showcase] RESULT: PASS` (vista dentro da
sala, personagem na tela, cenário presente/ausente conforme o modo, nenhuma
entidade acumulada após idas e voltas).

## 7. Como editar depois?

Para outra sala, acrescente uma linha em `SPOTS` (sala, posição, nome).

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `view x outside the room` | Limites da câmera | Veja `camera_rules.rs` |
| `player off screen` | Zona morta grande demais | Tutorial 30 |
| `entities ... after repeated visits` | Algo do cenário não sai com a sala | Peças precisam de `RoomEntity` |

## 9. Como desfazer com segurança?

Capturas e saves da bancada ficam em `target/` (pasta de build). Nada em
`assets/` ou `saves/` é escrito.
