# N3.8 — Pivô, escala, câmera e espelhamento

## Pivôs (respeitados por clipe)

| Clipe | Canvas | Pivô | Densidade | Tamanho no mundo |
|---|---|---|---|---|
| `idle` (N3.5) | 80×128 | (40, 118) | 2 | 40×64 u |
| `run` (N3.6) | 112×128 | (56, 118) | 2 | 56×64 u |

O runtime usa o pivô de cada clipe (`anchor_for(pivot, cell, flip)`). Nada é alinhado pelo canto da textura nem pela caixa opaca, e nenhum quadro foi movido. O balanço natural da corrida fica como foi aprovado.

- Os dois pivôs estão no **centro horizontal** do canvas, então a âncora `x` é 0 nos dois sentidos. Espelhar não desloca o corpo (teste `mirroring_keeps_the_pivot_and_the_sole_line`).
- Os dois estão 10 texels acima da borda inferior, então a linha da sola é a mesma nos dois clipes.
- A sola fica na base do corpo de colisão (`VISUAL_OFFSET_Y = −23`), como na produção.

## Escala

Densidade 2: o personagem de ~106 texels mede ~53 u no mundo, contra ~55 u do V2.2 (caixa opaca 110 texels). A colisão (26×46 u) não mudou.

## Câmera

- **405 (padrão, `camera.ron`, não alterado):** 720/405 = 1,778 px por unidade, 0,889 px por texel. A arte reduz: cerca de 1 em 9 linhas e colunas some, como a N3.5 já documentou.
- **360 (só prova):** o exemplo troca `CameraConfig.view_height` em memória (`--view 360`). Fica 1 px por texel.

Evidência: `evidence/n38_camera_405_vs_360.png`.

## Filtragem: o achado da N3.8

O jogo principal carregava as texturas do jogador com o amostrador **linear** padrão do Bevy. As provas aprovadas usam *nearest*. Medição na vista 360, comparando 24 capturas reais com o `run6.png` (`make_evidence.py`, `pixel_check_360`):

| Amostragem | Capturas idênticas | Pixels opacos iguais (±2) |
|---|---|---|
| linear (padrão do jogo) | 0 / 24 | 66,9 % |
| **nearest (modo experimental)** | **24 / 24** | **100 %** |

O desvio médio de cor era ~0, então não era luz nem névoa. A diferença acompanhava a fração de pixel da câmera: era borrão da filtragem linear.

**Correção, só para o modo experimental:** o recurso `NearestPlayerArt` faz `load_player_sprites` carregar os atlas com `ImageLoaderSettings { sampler: nearest }`. Sem o recurso, que é o caso da produção, a chamada é `server.load(path)`, idêntica à anterior. O V2.2 continua com a aparência de sempre. Se a produção também deve passar a *nearest* é decisão separada (`N3_8_AVALIACAO.md`).

## Pés

`evidence/n38_feet_alignment.png`: os 6 quadros no jogo (405) com a linha da sola em verde. Os pés de apoio tocam a linha. O quadro 0 (vídeo 48) e a partida ficam 1 texel acima, como a N3.7 registrou. Não foi corrigido, porque os pixels aprovados não mudam.

## Espelhamento

`flip_x` + âncora espelhada pelo mesmo `anchor_for`. Não existe folha espelhada. Validado no jogo headless (correr para a esquerda, virar correndo, parar virado para a esquerda) e nas capturas `n38_run_left_15fps.gif` e `n38_run_turn.gif`.

Limites para a N4: a arte é assimétrica (cabelo puxado para trás, braço da frente). Espelhada, o braço "da frente" vira o de trás no desenho. Arma em mão específica, escudo no braço esquerdo e mechas com lado definido vão precisar de uma regra de lado (`N3_8_ARQUITETURA_TERRARIA.md`, §soquetes).
