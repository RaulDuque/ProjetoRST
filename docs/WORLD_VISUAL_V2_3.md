# V2.3 — Câmera, cenários góticos, parallax e atmosfera

> Estado: **implementado no jogo normal** (sem assets novos em `assets/sprites/`:
> toda a arte de cenário é gerada em código, provisória). Pilotos decorados:
> `vigil_gates_causeway`, `fg_stair`, `av_square`. Atmosfera de região:
> `vigil_gates`, `forgotten_galleries`, `ash_village`.

## 1. Arquitetura visual

| Camada | Arquivo | Papel |
|---|---|---|
| Pura | `src/world/camera_rules.rs` | `CameraSettings` (`assets/data/camera.ron`), `CameraRig` (zona morta, olhar à frente, pulo, queda, limites, teleporte) |
| Pura | `src/world/scenery.rs` | Dados: `RegionVisual` (por região), `Decor` (por sala), validação, regra do parallax |
| Pura | `src/world/scenery_art.rs` | Arte provisória: 12 tipos de decoração, 2 camadas de fundo por tema |
| Pura | `src/core/pixel_art.rs` | Rasterizador (V2.2) + ladrilhos sem contorno, brilho radial, faixa de névoa |
| Bevy | `src/world/camera.rs` | Projeção `FixedVertical`, seguimento, snap em início/transição |
| Bevy | `src/world/scenery_systems.rs` | Cria o cenário da sala, move o parallax, anima luzes e névoa |
| Dados | `assets/data/camera.ron`, `assets/world/regions/*.ron` | Câmera; `visual` das regiões; `decor` das salas |

A geometria funcional (colisões, plataformas, portas, segredos, alavancas,
baús, paredes quebráveis, véus, inimigos) não mudou: o cenário é só `Sprite`
+ `RoomEntity`, sem componentes de colisão. Não há segundo sistema de salas:
`decor` é um campo opcional de `WorldRoom` e `visual` de `RegionDef`.

Ordem no frame: `RoomStep::React` → `spawn_scenery` (com os NPCs);
`Presentation` → `apply_zoom → snap_on_start → follow_player →
update_parallax → animate_scenery`.

## 2. Câmera

**Diagnóstico da câmera anterior (V2.2):** escala 1:1 fixa (a sala de 1
célula, 1280×720, cabia inteira: o personagem de ~52 u ocupava ~7% da
altura), limites calculados com as constantes da janela (não com a vista
real), Y do jogador seguido diretamente (subia e descia a cada pulo),
olhar à frente fixo de 60 u.

**Agora** (`assets/data/camera.ron`; todo campo é opcional):

| Campo | Padrão | Unidade | Faz |
|---|---|---|---|
| `view_height` | 405 | u | Altura do mundo visível (zoom ×1,78); 240–720 |
| `follow_rate_x` / `_y` | 7 / 5 | 1/s | Rapidez do acompanhamento |
| `dead_zone_x` | 24 | u | Zona morta horizontal (meia largura) |
| `dead_zone_up` / `_down` | 90 / 40 | u | Zona morta vertical no ar |
| `look_ahead` / `look_ahead_rate` | 70 / 2,5 | u, 1/s | Olhar à frente, virada suave |
| `vertical_offset` | 40 | u | Centro da vista acima do personagem em pé |
| `fall_look_down` / `fall_speed` | 70 / 420 | u, u/s | Olhar para baixo em quedas rápidas |
| `effects` | tudo ligado | — | `parallax`, `decor`, `lights`, `fog` |

- **Limites:** a vista nunca sai da sala; sala menor que a vista fica centrada.
- **Teleporte** (renascer, carregar, troca de sala): mais de 200 u num
  quadro → snap, sem deslizar pela sala.
- **Resolução:** `ScalingMode::FixedVertical`: a altura do mundo visível é
  fixa e a largura segue a janela (16:9 de referência sem janela).
- **Física independente:** o zoom muda só a projeção.
- **Escolha do zoom:** varredura 720/540/480/405/360 em três salas (castelo
  horizontal, escada vertical, vila). Em 405 o protagonista fica legível
  (~92 px de altura numa janela de 720 px) e a sala vertical mostra 3–4 níveis
  de plataforma; em 360 o HUD passa a cobrir plataformas.

## 3. Decoração

`decor: [(kind: ..., at: (x, y), layer: Back|Front, flip: bool)]` na sala.
Tipos e ponto de apoio no tutorial 32. Regras:
- validada pelo validador do mundo (`Issue::BadScenery`): dentro da sala,
  tamanhos sensatos, cores 0–1, `Front` só para `Chain`/`DeadVine`;
- camada `Back` atrás das plataformas (z 0,5); `Front` translúcida (z 9);
- pedra da decoração = 80% da pedra da região, sempre mais escura que as
  plataformas (teste de luminância); o arco é um **nicho** (fundo com fiadas
  de pedra), nunca lido como passagem.

## 4. Parallax

Por região (`backdrop: Castle | Ruins | Village`), duas camadas geradas em
ladrilhos de 640×720 u (densidade 1 texel/u):

| Camada | Fator | Repete | z |
|---|---|---|---|
| Distante (céu + silhuetas) | 0,12 | horizontal | 0,1 |
| Intermediária (arcada, ruínas, fachadas) | 0,40 | horizontal e vertical | 0,2 |

- `parallax_centre(câmera, sala, fator) = sala/2 + (câmera − sala/2)(1 − fator)`.
- O tamanho da camada é um número inteiro de ladrilhos ≥ `vista + fator ×
  (sala − vista) + margens`: cobre a vista em qualquer posição permitida
  (testado em salas pequenas, largas e verticais) e nunca começa com um
  ladrilho cortado.
- Ladrilhos sem costura (formas nunca cruzam a borda; testado coluna a coluna).
- Camadas que repetem na vertical não têm faixa horizontal de lado a lado
  (repetida, ela pareceria uma plataforma; testado).

## 5. Iluminação e atmosfera

Sem pós-processamento e sem dependências novas:
- **Brilhos**: discos com queda suave (`radial_glow`), da cor `light` da
  região, atrás das plataformas e dos personagens (z 0,6): tochas e lanternas
  oscilam (alfa ±9%), janelas ficam estáveis;
- **Névoa**: faixa sobre o chão (`vertical_fade`), opacidade ≤ 0,35,
  derivando devagar, atrás das plataformas (z 0,7);
- **Clima por região**: céu, pedra e luz.

Nenhum efeito fica na frente de inimigos, portas, plataformas ou perigos
(só correntes/vinhas finas e translúcidas). Tudo desligável em `effects`.

## 6. Regiões

| Região (ID real) | `backdrop` | Pedra | Luz | Névoa |
|---|---|---|---|---|
| `vigil_gates` | Castle | fria (0,34; 0,33; 0,40) | tocha (1,0; 0,68; 0,36) | 0,12 |
| `forgotten_galleries` | Ruins | musgosa (0,30; 0,34; 0,32) | espectral (0,55; 0,90; 0,78) | 0,22 |
| `ash_village` | Village | quente (0,36; 0,30; 0,27) | brasa (1,0; 0,62; 0,30) | 0,10 |

`veiled_cloister` não recebeu `visual` (mantém a V2.2).

## 7. Salas piloto

| Sala | Situação | Decoração |
|---|---|---|
| `vigil_gates_causeway` (2×1) | horizontal com inimigo e baú | 3 pilares, 2 janelas, arco-nicho, 3 tochas, 2 estandartes, estátua, entulho, 2 correntes (frente) |
| `fg_stair` (1×3) | vertical, 12 plataformas | pilar partido, grade, arco-nicho, 2 tochas, lanterna, vinhas, 2 correntes + 1 vinha (frente) |
| `av_square` (1×1) | vila com NPC | fachada de loja, janela, 2 tochas, lanterna em corrente, estandarte, estátua, entulho |

Avaliação: a arquitetura funcionou nas três situações sem mudar colisões nem
interações; aplicar às demais salas é só dados. Não foi feito nesta versão
(pedido do briefing: validar antes de escalar).

## 8. Compatibilidade

Save v3 inalterado (nada de cenário é salvo; teste confere o arquivo).
Regiões/salas sem `visual`/`decor` carregam como antes; `camera.ron` ausente
ou inválido → padrões. Combate, magias, forma espectral, armas visíveis,
âncoras, NPCs, lojas e mapa: rotas reais e suíte completa aprovadas.

## 9. Testes

- Unitários: regras da câmera (9), cenário (5), arte de cenário (5),
  ladrilhos/brilho/névoa (2), tamanho das camadas (1).
- `tests/world_scenery_tests.rs` (10): pilotos com física idêntica com e sem
  cenário, salas/regiões sem dados, descarregamento sem acúmulo, efeitos
  desligáveis, parallax segue a regra, save sem cenário e load que o recria,
  HUD não escala com o zoom, `camera.ron` = padrões, dados inválidos
  recusados e arquivos antigos aceitos, custo medido.
- `tests/world_camera_tests.rs` adaptado à vista com zoom (mesmas checagens).
- Com janela: `examples/world_visual_showcase.rs` (antes/depois, varredura de
  zoom, movimento, duas resoluções, 6 idas e voltas sem acúmulo).

## 10. Performance

Headless, perfil dev, mesma sala (`vigil_gates_causeway`), 300 quadros
(`target/tmp/scenery_v23_perf.txt`):

| Medida | Com cenário | Sem cenário |
|---|---|---|
| Tempo médio por quadro | 0,36 ms | 0,32 ms |
| Pior quadro | 0,52 ms | 0,52 ms |
| Entidades | 168 | 144 |
| Sprites | 40 | 16 |
| Peças de cenário | 24 | 0 |
| Imagens geradas (cache) | 27 | 3 |
| Troca de sala (1ª visita / revisita) | 25,9 / 1,6 ms | 1,3 / 1,1 ms |

- A diferença de tempo por quadro está perto do ruído (varia entre execuções).
- A 1ª visita de uma região gera a arte uma vez (cache); antes da otimização
  (camadas regeradas a cada troca, sem pré-filtro por caixa) a troca levava
  ~220 ms.
- Nada é recriado por quadro; repetir 6 idas e voltas deixa o mesmo número
  de entidades (vitrine e teste).
- Memória e GPU não foram medidas (sem medição confiável headless).

## 11. Limitações

- Arte de cenário provisória, gerada em código.
- Só três salas decoradas; Veiled Cloister sem atmosfera.
- "Iluminação" são brilhos aditivos aparentes, não luz dinâmica.
- Uma câmera para todas as salas (sem ajuste por sala).
- A 1ª entrada numa região custa ~26 ms (geração da arte) no perfil dev.

## 12. Próximas melhorias

Decorar as demais salas por dados; atmosfera do Veiled Cloister; arte final
das peças e camadas (PNG pelo pipeline); zoom/limites por sala; pré-geração
da arte de cenário no carregamento.
