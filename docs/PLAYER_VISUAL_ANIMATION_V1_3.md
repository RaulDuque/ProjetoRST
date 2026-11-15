# Visual animado do protagonista (V1.3)

> **Atualização V1.4:** todos os estados ganharam animação própria (dano,
> morte, pulo, queda, aterrissagem, agachar, guarda e as quatro habilidades).
> Ver [`PLAYER_ANIMATION_EXPANSION_V1_4.md`](PLAYER_ANIMATION_EXPANSION_V1_4.md).
> A tabela de fallbacks da §4 abaixo descreve a V1.3; o fallback com tom agora
> só aparece se o manifesto não tiver o clipe.

> Primeira integração dos sprites do protagonista (modelo v007, Conceito B) no
> jogo. Só apresentação: nenhuma regra, duração, hitbox, física, câmera ou
> formato de save mudou. Contrato de sprites: [`ASSET_CONTRACT.md`](ASSET_CONTRACT.md).
> Arte: `art-src/player/ANIMATION_VALIDATION_V1_2G.md` e
> `art-src/player/comparisons/v1_3_game_sprites/`.

## 1. Objetivo e decisões provisórias

O protagonista aparece e se anima no jogo com quatro animações: `idle`, `run`,
`attack_basic` e `dodge`. As decisões abaixo são provisórias para a integração,
não escolhas artísticas definitivas.

| Item | V1.3 |
|---|---|
| Modelo | v007 (`swordsman_rig_v007_game_v1_3.blend` guarda as ações) |
| Acabamento no jogo | C (cores chapadas), densidade 2 (2 texels por unidade). O B é gerado só para comparação, fora de `assets/` |
| Espada | Variante ×1,7 (`blade_readable`) |
| Câmera | 1:1, sem mudança |
| Taxa | `attack_basic` e `dodge` a 24 fps; `run` a 24 fps (10 quadros); `idle` a 12 fps (12 quadros) |
| Velocidade | 190 px/s preservados; a corrida foi feita para essa velocidade |
| Ações | 0,28 s preservados; o impacto aparece no quadro 0 |

## 2. Arquitetura

```text
Player (entidade de gameplay, sem mudança: Transform, Body, Velocity,
│       ActionState, FacingDirection, Invulnerable, Sprite = caixa provisória)
├── Visor (filho provisório, escondido quando o sprite aparece)
└── PlayerVisual (filho de apresentação)
    ├── Sprite (image + TextureAtlas + custom_size + flip_x + color)
    ├── Anchor (pivô da sola)
    ├── Transform (0, −23, 0,05): a sola fica na base do corpo de 26×46
    ├── Visibility (Hidden até as texturas carregarem)
    └── VisualPlayback (clipe, tempo, duração da ação, índice)
```

Módulo `src/player/visual/` (registrado por `PlayerPlugin` como `PlayerVisualPlugin`):

| Arquivo | Camada | Papel |
|---|---|---|
| `manifest.rs` | Pura | Tipos e validação de `assets/animations/player.ron`; `anchor_for` (pivô → `Anchor`, com espelhamento) |
| `playback.rs` | Pura | Quadro de um loop, de uma ação (por progresso) e a taxa da locomoção pela velocidade |
| `selection.rs` | Pura | Estado de gameplay → clipe, temporização, tom e animação que falta |
| `systems.rs` | Bevy | `load_player_sprites` (Startup), `attach_player_visual` e `animate_player_visual` (Update) |
| `mod.rs` | Bevy | Componentes, recurso `PlayerSpriteSet`, `PlayerVisualSystems`, plugin |

Mudanças fora do módulo, todas de apresentação ou só de leitura:

- **`player/state.rs`:** ganhou o getter `remaining()`, só de leitura. É o
  tempo restante da ação, para o visual acompanhar a duração real (inclusive
  com `attack_speed`). Nenhuma regra mudou.
- **`player/animation.rs`:** enquanto o sprite está visível, a caixa provisória
  fica transparente, com escala 1 (uma raiz achatada achataria o sprite,
  contrato §5 regra 5) e sem visor. Sem sprite, funciona como antes.
- **`player/mod.rs`:** registra o plugin e ordena o visual provisório depois do sprite.

**Regra fundamental:** o visual só lê `ActionState`, `FacingDirection`,
`Velocity` e `Invulnerable`; nunca escreve. O fim de uma animação não encerra
ações, o início de um sprite não causa dano e nenhuma transição dá
invulnerabilidade.

## 3. Ordem dos sistemas

`Input → Commands → Actions → Physics → Combat → Resolution → Progression`
(só em `Playing`), depois `Presentation`. Dentro de `Presentation`:

1. `quick_save` / `quick_load`;
2. `PlayerVisualSystems`: `attach_player_visual`, depois `animate_player_visual`.
   O conjunto é ordenado depois de `quick_load`, para o load aparecer no mesmo frame;
3. `attach_visor` / `update_player_visuals` (caixa provisória), depois do sprite.
   Assim nenhum frame mostra caixa e sprite juntos.

O visual lê o estado calculado no mesmo frame: não há atraso de um frame. A
câmera (`follow_player`) não foi tocada.

## 4. Estados, prioridades e fallbacks

A máquina de estados já garante as prioridades:
- morte > dano > esquiva > ataque/conjuração > locomoção;
- uma ação trava a locomoção até acabar.

A seleção (`selection::select`) é, por isso, um mapeamento direto:

| Estado de gameplay | Visual | Temporização |
|---|---|---|
| `Idle` | `idle` | loop a 12 fps |
| `Walking`, `Running` | `run` | loop; taxa = \|vx\| / 190 px/s, limitada entre 0,5 e 2,5 |
| `Attacking` | `attack_basic` | progresso da ação (`FitToAction`) |
| `Dodging` | `dodge` | progresso da ação |
| `Hurt` | quadro 0 do `idle`, tom avermelhado | parado (falta a animação `hurt`) |
| `Dead` | quadro 0 do `idle`, tom escuro | parado (falta `death`) |
| `Casting` | quadro 0 do `idle`, tom violeta | parado (falta `cast`; vale para técnicas e magias) |
| `Guarding` | quadro 0 do `idle`, tom azulado | parado (falta `guard`) |
| `Crouching`, `Jumping`, `Falling` | quadro 0 do `idle` | parado (faltam `crouch`, `jump`, `fall`) |

- **Ação inexistente:** os estados sem sprite não fingem uma ação; mostram a
  pose neutra e um tom de feedback.
- **Invulnerabilidade:** o piscar (alfa 0,35 a 16 Hz) continua, agora no sprite.
- **Pausa:** na tela de status a animação fica parada (`dt = 0`), como o gameplay.
- **Reinício do clipe:** muda o clipe ou o estado, ou o tempo restante sobe (nova ação).
- **Mesmo clipe:** nenhum clipe reinicia a cada frame.

## 5. Sincronização

- **Ações:** o quadro sai de `progresso = (total − restante) / total`.
  - `total` é o tempo restante lido no primeiro frame da ação.
  - Por isso o ataque mostra o quadro 0 (impacto) no mesmo frame em que a hitbox
    nasce, e termina exatamente com o estado.
  - As durações por quadro mantêm a proporção: 6 × 1/24 s + 0,03 s = 0,28 s, e o
    último quadro continua curto.
  - Com `attack_speed`, o mesmo clipe acompanha a duração mais curta.
- **Corrida:** a taxa acompanha a velocidade real.
  - A 190 px/s o ciclo toca a 1×; com o botão de correr (310 px/s), a 1,63×.
  - Assim o pé de apoio continua acompanhando o chão.

## 6. Corrida para 190 px/s

| Medida | Valor |
|---|---|
| Velocidade de referência | 190 px/s = 6,65 m/s (0,035 m/px) |
| Ciclo | 10 quadros a 24 fps = 0,417 s, 2 passos, com fase de voo |
| Passada do pé de apoio | 0,76 m relativos ao quadril, em 27,4 % do ciclo |
| Avanço do corpo por ciclo | 2,77 m = 79,2 px |
| Pé de apoio medido | 7,0–7,17 m/s (5–8 % acima de 6,65) |
| Deslizamento residual | ~0,35–0,5 m/s, ou seja ~10–15 px/s (≈ 0,2 px por frame de 60 Hz) |

- **Por que não a caminhada acelerada:** o ciclo de caminhada da V1.2-G (36 px/s)
  não foi reaproveitado acelerado 5,28×.
- **Origem do resíduo:** o pé rola durante o apoio e a medição usa o centro da sola.
- **12 fps:** daria 5 quadros num ciclo de 0,417 s, cada um cobrindo 55 cm de
  avanço, o que fica entrecortado. Por isso a corrida vai a 24 fps. O idle vai a
  12 fps, onde a diferença não aparece.

## 7. Assets e metadados

| Arquivo | Conteúdo |
|---|---|
| `assets/sprites/player/player_locomotion.png` | 1038×518: `idle` (0–11), `run` (12–21), `dodge` (22–28); célula 128×128 texels, 8 colunas, 4 linhas, padding 2 |
| `assets/sprites/player/player_combat.png` | 1804×192: `attack_basic` (0–6); célula 256×192 texels, 7 colunas, 1 linha, padding 2 |
| `assets/animations/player.ron` | Manifesto gerado (não editar à mão) |

- **Geração:** `art-src/player/scripts/bpy/build_game_sprites_v1_3.py` (Blender)
  e `art-src/player/scripts/post/game_sprites_v1_3.py`.
- **Formato:** PNG RGBA sRGB, alfa reto, filtro linear (padrão do Bevy).
- **Variantes:** só as necessárias; nenhuma variante experimental foi copiada.

O manifesto segue a proposta do contrato (§7), com dois campos a mais:
- `durations`: duração de cada quadro, em segundos;
- `speed_reference`: velocidade, em u/s, para a qual o loop foi feito.

Validação estrutural (`AnimationManifest::validate`):
- versão;
- identificadores em snake_case;
- atlas existente;
- `first + count` dentro da grade;
- pivô dentro da célula;
- uma duração por quadro, todas positivas;
- fps e velocidade positivos;
- `fallback` existente e sem ciclos;
- o clipe raiz `idle` presente.

`tests/data_tests.rs` confere, a partir do manifesto real:
- o tamanho dos PNG contra a grade, e ≤ 2048 px de lado;
- as células 64×64 e 128×96 em unidades e os pivôs (32, 63) e (40, 95);
- 0,28 s em `attack_basic` e `dodge`.

**Carregamento:** um sistema de Startup lê o manifesto e cria os handles uma vez
(recurso `PlayerSpriteSet`, compartilhado).
- **Sem manifesto:** visual provisório, com log de info.
- **Manifesto inválido:** visual provisório, com log de erro. O contrato deixava
  essa política pendente; aqui um export quebrado não derruba o jogo, e o teste
  de dados impede que um manifesto inválido seja entregue.
- **Sem `AssetServer`** (testes headless): visual provisório.
- **Até as texturas carregarem:** a caixa continua visível e o sprite, escondido.

## 8. Direção e pivôs

- **Arte:** desenhada olhando para a direita. A espada está na mão direita do
  personagem, que é o lado próximo da câmera; a meia-capa fica no ombro esquerdo,
  o lado distante.
- **Esquerda:** `flip_x = true`, e o `anchor.x` troca de sinal junto
  (`anchor_for`; contrato §5 regra 3).
  - Locomoção: anchor (0, −0,484), centrado em x.
  - Combate: (−0,1875, −0,4896) para a direita e (+0,1875, −0,4896) para a esquerda.
- **Sem salto entre animações:** o pivô fica na linha da sola nas duas células, e
  o filho fica em y = −23. Na validação real, a sola ficou em y = 40 em todos os
  clipes, com o jogador em y = 63.
- **Limitação do espelhamento:** virado para a esquerda, a imagem é a mesma
  espelhada. A espada passa a aparecer na mão esquerda e a capa troca de ombro.
  É o padrão em sprites 2D e está registrado como limitação; desenhar o lado
  esquerdo exigiria um segundo conjunto de sprites.

## 9. Save/load e respawn

- O formato do save não mudou.
- O `quick_load` atualiza a entidade existente e não a recria; o visual
  acompanha no mesmo frame (ordenado depois de `quick_load`).
- O respawn também atualiza a entidade, e continua havendo um único filho.
- Se o jogador fosse removido, o filho iria junto (despawn recursivo), e
  `attach_player_visual` criaria um novo para o jogador novo.
- Testado em `tests/player_visual_tests.rs`.

## 10. Testes

| Onde | O que cobre |
|---|---|
| `manifest.rs` (6) | Parse, tamanhos, faixa de quadros, pivô, durações, fallback, raiz, snake_case, anchor e espelhamento |
| `playback.rs` (5) | 0,28 s exatos com último quadro de 0,03 s, `attack_speed`, loop sem quadro duplicado, taxa pela velocidade, limites |
| `selection.rs` (3) | Clipes de locomoção e ação, fallbacks, prioridades seguindo a máquina de estados |
| `tests/player_visual_tests.rs` (8) | App sem `AssetServer`; filho único; texturas não carregadas; idle, run e direção; ataque e esquiva com a duração do gameplay e o quadro de impacto; anchor de combate espelhado; fallbacks e respawn; **gameplay idêntico com e sem sprites** (posição e estado bit a bit em 240 frames de entrada roteirizada); quick load |
| `tests/data_tests.rs` (+1) | Manifesto real, PNG e contrato |
| `examples/visual_check.rs` | Jogo real com janela: roteiro com idle, corrida, ataque, esquiva e os dois lados, 13 screenshots e conferência do clipe e do flip em cada uma |

Execução (V1.3):
- `cargo fmt --check`, `cargo clippy --all-targets` (0 avisos), `cargo test`
  (146 testes, contra 123 antes) e `cargo check`: todos OK.
- `cargo run --example autoplay`: PASS 23/23.
- `cargo run --example visual_check`: PASS 13/13.

## 11. Desempenho

| Item | Valor |
|---|---|
| Texturas | 2 PNG (227 kB + 74 kB). Na GPU, 1038×518 + 1804×192 RGBA = 3,5 MB; o Bevy mantém por padrão também uma cópia na RAM (~7 MB no total) |
| Quadros | 29 de locomoção + 7 de combate |
| Entidades | +1 por jogador |
| Por frame | 2 sistemas; nenhum carregamento, handle novo ou processamento de imagem. A única alocação é uma `String` quando o clipe muda |

Nenhum gargalo observado.

## 12. Limitações e próximas animações

- **Faltam:**
  - `hurt`, `death`/`dead`, `cast_forward`, `cast_up`;
  - `slash_crescent`, `slash_rising`;
  - `guard_*`, `crouch`, `jump_*`/`fall`/`land`;
  - `turn` e `attack_basic_air`.
  - Hoje usam o fallback da §4. As técnicas e magias aparecem como `Casting`.
- **Acabamento:** o C foi escolhido para integrar; D2 continua pendente. O B fica
  pronto em `art-src/player/renders/v1_3_game_sprites/atlas_B/`.
- **Espelhamento:** troca a mão da espada e o lado da capa (§8).
- **Corrida:** deslizamento residual de ~5–8 % (§6).
- **Caminhada:** o jogo não tem estado de caminhada lenta; o ciclo de caminhada
  da V1.2-G não é usado.
- **Pixel inteiro (acabamento A):** não se aplica ao C; a câmera continua sem
  arredondar (§5 regra 6 do contrato).
