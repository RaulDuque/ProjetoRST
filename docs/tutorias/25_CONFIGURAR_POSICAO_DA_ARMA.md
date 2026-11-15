# 25 — Configurar a posição da arma (âncoras)

> Situação: **funciona** (publicada na V2.2). As posições **não se
> editam à mão**: são medidas no Blender sobre as mesmas poses dos sprites.
> **Pendente:** ajuste fino por item (ex.: segurar uma lança mais atrás) não
> existe; o ajuste é por categoria, no código (`src/inventory/held_art.rs`).

## 1. O que vamos fazer?

Entender de onde vem a posição da arma em cada quadro e como gerar de novo
a tabela de âncoras depois de mudar uma animação.

## 2. O que você precisa saber antes?

- Cada quadro de cada animação tem **âncoras**: mão direita (arma), mão
  esquerda (escudo), cabeça, peito e costas (capa).
- Ficam em `assets/animations/player_sockets.ron` (publicada pelo guarda
  junto com os sprites; o staging fica em `art-src/player/renders/v2_2_weaponless/stage/`).
- Unidades: unidades de mundo a partir da sola (`x` para a frente, `y` para
  cima). `angle`: direção pomo → guarda, em graus. `front: true`: a mão está à
  frente do peito (a arma é desenhada por cima do corpo).
- Virar para a esquerda espelha `x` e o ângulo automaticamente.
- Em repouso, lança e arma pesada viram de ponta para cima (não atravessam o
  chão); nos golpes elas seguem o sabre.
- As âncoras são derivadas da arte: **não vão para o save**.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `art-src/player/scripts/bpy/build_game_sprites_v2_2.py` | Mede as âncoras e renderiza o corpo sem arma |
| `art-src/player/scripts/post/game_sprites_v2_2.py` | Monta atlases, manifesto e `player_sockets.ron` no staging |
| `art-src/player/renders/v2_2_weaponless/stage/animations/player_sockets.ron` | A tabela gerada |
| `src/player/visual/sockets.rs` | Só leitura: formato e validação |

## 4. Passo a passo

1. Com o Blender instalado, rode para cada módulo (`v1_3`, `v1_4`, `v1_5`):
   `blender --background --factory-startup art-src/player/blender/swordsman_rig_v007.blend --python art-src/player/scripts/bpy/build_game_sprites_v2_2.py -- --module v1_3 --overwrite`
2. Monte o staging: `python3 art-src/player/scripts/post/game_sprites_v2_2.py`.
3. Veja `art-src/player/comparisons/v2_2_weaponless/published_vs_weaponless.png`:
   a linha amarela tem de seguir a lâmina antiga em cada pose.
4. Valide: `python3 art-src/player/scripts/post/publish_guard.py check --stage art-src/player/renders/v2_2_weaponless/stage`.

## 5. Exemplo completo

O primeiro quadro do `idle` na tabela gerada (o `idle` tem 12):

```ron
// exemplo-verificado: QuadroAncoras
(weapon: (x: 8.28, y: 27.83, angle: -66.23, front: true), hand_l: (x: -0.11, y: 25.31, front: false), head: (x: 0.78, y: 47.59), chest: (x: 0.49, y: 38.77), back: (x: 0.18, y: 41.43))
```

| Campo | Significado |
|---|---|
| `weapon` | Centro da empunhadura, ângulo da lâmina e frente/trás |
| `hand_l` | Mão do escudo |
| `head`, `chest` | Cabeça e peito |
| `back` | Gola da capa (a capa pendura daqui) |

## 6. Como testar?

1. `cargo test -j 2 --test held_visual_tests` (as camadas seguem a âncora do quadro).
2. `cargo run -j 2 --example equipment_visual_showcase` e inspecione as capturas.

## 7. Como editar depois?

Mudou uma pose no Blender? Rode de novo os passos 1–4: as âncoras saem das
mesmas poses. Não edite números à mão: o guarda recusa contagem errada de
quadros e o jogo recusa valores absurdos.

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `pose keyframes differ from ...` | A pose mudou desde o build publicado | Regere também os sprites da versão correspondente |
| `sockets: clip X has N frames, expected M` | Tabela desatualizada | Rode o passo 2 de novo |
| `invalid player socket table` (log do jogo) | Tabela editada à mão | Regere; sem tabela válida as camadas ficam desligadas |
| Arma some num movimento | Clipe sem âncoras | Normal como proteção: regere a tabela |

## 9. Como desfazer com segurança?

O staging fica em `art-src/player/renders/` (fora do Git). Apagar a pasta
`v2_2_weaponless/` só remove a candidata; `assets/` não muda.
