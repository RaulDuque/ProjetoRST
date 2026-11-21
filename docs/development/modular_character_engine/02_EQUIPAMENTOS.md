# Dark Castle Modular Character Engine — 02: equipamentos independentes

Briefing: `Md-usario/engine/Engine-02.md` (do proprietário). Relatório:
[`docs/relatorios/modular-character-engine/02_EQUIPAMENTOS.md`](../../relatorios/modular-character-engine/02_EQUIPAMENTOS.md).
Arquitetura base (01): [`01_ARQUITETURA.md`](01_ARQUITETURA.md).

O proprietário aprovou a abordagem **híbrida** como base experimental (corpo aprovado +
equipamento preso ao esqueleto). A versão totalmente articulada continua no laboratório como
experimento, sem aprovação visual.

## 1. Esqueleto refinado

- **Marcos de 19 juntas** (15 da 01 e 4 da caixa do tronco: `chest_back`, `chest_front`,
  `waist_front` e `waist_back`):
  - corrida #0, #2 e #4: marcos da N4-C (`run_poses.py`), medidos no vídeo em alta resolução;
  - corrida #1, #3 e #5: medidos nos quadros do vídeo `video_050/054/058.png` pela mesma
    redução (canvas = vídeo × 0,23661 + (−3, −5));
  - idle: refeito nas grades ampliadas;
  - punho do #4 em (51, 56): o marco da N4-C (57, 58) cai na calça (teste
    `every_marked_skeleton_lands_on_the_art`).
- **Estabilidade:**
  - o ombro de perto segue o balanço do braço numa curva suave (x 60, 61, 61, 59,5, 58,
    58,5);
  - o pescoço fica em x 63,5 (na arte ele se move menos de um texel);
  - a 01 fica guardada em `stage/history/rig_lab_01.ron`.
- **Métricas** (`rig::measure`; teste `the_refined_marks_are_steadier_than_prototype_01`):

| Métrica | 01 | 02 |
|---|---|---|
| Tremor do ombro no referencial da espinha (2ª diferença média, texels) | 1,81 | 1,00 |
| Deslocamento do ombro (desvio padrão, texels; a 02 acompanha o braço de propósito) | 0,86 | 1,14 |
| Tremor do ângulo da espada (2ª diferença média, graus) | 47,8 | 5,8 |

## 2. Mão, pulso e empunhadura (`rig::grip`)

Os quadros dão a posição do punho, não a rotação do pulso. Um `GripProfile` fornece essa
rotação:
- referência `Body` (soma a inclinação da espinha), `Forearm` (comportamento da 01) ou `World`;
- ângulo da lâmina;
- `follow_forearm`, a fração do balanço do antebraço que a arma acompanha;
- profundidade opcional por quadro.

`hand_frame` devolve o referencial da mão: origem no punho, +x ao longo da lâmina. É o
modelo de arma do pixelfx (ângulo `sa` independente do braço), adaptado a quadros marcados.

| Perfil | Clipe | Valores | Resultado medido |
|---|---|---|---|
| `sword_idle` | idle | `Body`, −55°, follow 0 | diagonal para baixo e para a frente, ponta acima do chão |
| `sword_run` | corrida | `Body`, 200°, follow 0,15; atrás do corpo nos quadros 3–5 | levada para trás, oscilação de ±menos de 20° |

- **Espelhamento:** posição e rotação refletidas, mesma profundidade.
- **Troca de arma:** não mexe no relógio.

## 3. Camadas

- **Corrida #0, #2 e #4:** usam mapas de rótulos gerados das camadas da N4-C
  (`scripts/n4c_label_maps.py`, a camada visível de cima). Correção: rosto que a N4-C deixou
  nas camadas de braço volta para o tronco.
- **Idle, #1, #3 e #5:** usam as regras 02 (`layers::layer_of`), que portam a ordem do
  `classify` da N4-C:
  1. braço de perto sobre o tronco, com raio menor dentro da caixa do tronco;
  2. sombras da manga com os cinzas do cabelo e os marrons da calça contam como manga;
  3. cabelo pelo círculo da cabeça, fora do tronco ou acima dos ombros, incluindo a tinta
     atrás do rosto;
  4. abaixo da cintura (o cinto) é perna;
  5. braço de trás só fora do tronco.
- **Concordância regras × N4-C** (IoU): braço de perto 0,62 / 0,64 / 0,65 e cabelo
  0,84 / 0,78 / 0,76 nos quadros #0, #2 e #4.
- A recomposição continua idêntica texel a texel nos 7 quadros (0 texels diferentes no
  Bevy).
- **Ordem de profundidade (de trás para a frente):** `BackGear`, `MainHandBehind`, `FarArm`,
  `FarHandGear`, `Legs`, `LegGear`, `BootGear`, `Torso`, `ChestGear`, `BeltGear`, `Hair`,
  `HelmetGear`, `MainHandGear`, `NearArm`, `BracerGear`, `ShoulderGear`.

## 4. Contrato de equipamentos (`rig::equipment`)

- **Slots:** `MainHand`, `OffHand`, `Shoulder`, `Chest`, `Bracers`, `Cape`, `Helmet`, `Belt`,
  `Legs`, `Boots`.
- **Tipo de deformação (`DeformKind`):**
  - `Rigid`: uma peça;
  - `Articulated`: segmentos rígidos, cada um no seu osso;
  - `Flexible`: deformado por uma gaiola presa às juntas; hoje só a do tronco.
- **Arte:** `Sprite` (textura, tamanho, pivô) ou `Model` (modelo vetorial).
- **Montagem:** `Bone` (osso, ponta, deslocamento, `up_degrees`, `follow`), `Hand` (lado,
  perfil de empunhadura do clipe) ou `Torso` (gaiola).
- **Validação** (`RigItem::check`): pivô dentro da imagem, `follow` em 0..1, modelo ⇔ tronco,
  flexível exige gaiola, articulado exige segmentos em ossos, mão só para os slots de mão.
- **Variante de perspectiva derivada da mesma fonte:** o peitoral muda a crista entre a vista
  lateral e a de três quartos pela proporção da gaiola (`ChestModel::turn`).
- **Item novo:** é só dados no RON. Nenhuma spritesheet do personagem é necessária.

| Slot | Implementado com arte | Só contrato e testes |
|---|---|---|
| Espada (MainHand) | 2 espadas da N41-B | |
| Ombreira | 2 ombreiras derivadas da N41-B | |
| Peitoral | 1 modelo vetorial | |
| Escudo, braçais, capa, capacete, cinto, calças, botas | | validação e colocação testadas com dados sintéticos |

## 5. Peitoral: três abordagens, uma fonte

O modelo `plate_cuirass` fica no RON:
- silhueta poligonal nas coordenadas da gaiola (u da costa para a frente, v do ombro para a
  cintura), saindo um pouco além da camisa;
- rampa de 6 tons, gola dourada, 2 lamelas, crista.

A mesma fonte é posta em cada pose de três formas (`rig::chest`):

| Abordagem | Como | Resultado |
|---|---|---|
| Rígida | rasterizada uma vez no idle e girada com a espinha | estreita demais na corrida (o tronco de 3/4 tem quase o dobro da largura); descartada |
| Deformação por malha | o raster do idle amostrado pela gaiola de cada pose (vértices nas 4 juntas) | cobre o tronco, mas estica os texels: lamelas e contorno irregulares |
| **Projetada** (escolhida) | o modelo vetorial rasterizado direto na grade de texels de cada pose | contorno de 1 texel, só cores da paleta, alfa binário (`pixel_qa` aprovado nas 8 poses); acompanha as 6 poses da corrida e o idle |

**Geometria do Blender:** não foi usada. A projeção do modelo 2D na gaiola cumpre o mesmo
papel (geometria independente desenhada direto em pixels, como o `rig.js` do pixelfx) sem
exportação por pose. O rig v007 continua disponível se o peitoral precisar de volume real.

**Limitações:**
- no idle de perfil, o braço de perto cobre quase todo o peitoral;
- a gola dourada sai em traços de 1–2 texels;
- o desenho do modelo é de diagnóstico e não está aprovado.

## 6. Ferramentas externas executadas

`scripts/run_external_tools.py`, resultados em `evidence/.../02_equipamentos/tools/`.

| Ferramenta | Uso | Resultado |
|---|---|---|
| ASF `rig_animate.py` | Renderizou as 6 poses do nosso esqueleto (SVG `rig.v1` + `rig_anim.v1` exportados pelo laboratório) | 72/72 marcadores; erro médio 0,16 texel e máximo 1,55 em relação ao nosso FK. QA da ASF: `seam` e `ik_clamp` aprovados, `ground_row` reprovado (esperado: o esqueleto não tem pés no chão da ASF) |
| ASF `pixel_motion.py` | A CLI `measure` só aceita a saída do pipeline de vídeo dela (`frames.json`), então chamamos as funções `measure_walk` e `legs_picture` | Passos a cada 6 quadros e pisadas nos quadros 0, 5 e 11, iguais com e sem equipamento: o equipamento não altera a leitura da marcha |
| ASF `build_animation_clips.py` | Manifesto v2 das corridas aprovada e híbrida equipada | 4 ticks por quadro a 60 Hz, deriva máxima de 0,333 ms, 0 avisos; teste de virada espelhada gerado |
| ASF `pixel_qa.py` | Rasters do peitoral (6 + 2 poses) e ombreiras | Peitoral: `partial_alpha`, `off_palette` e `outline_gaps` em 0, `l_corners` 4 (limite 10). Ombreiras: `partial_alpha` 0 |
| pixelfx `rig.js` `skeleton()` | Poses de espada | Mudando só `sa` em 190°, o antebraço muda 3,5°; movendo só a mão, a lâmina não muda |
| pixelfx `tools/act.mjs` | `melee/sword-slash` | Tira de 21 passos (referência de sincronização de ação) |

O pixelfx e o ASF são MIT. Nenhuma linha deles foi copiada para o jogo e os créditos estão no
código. Nenhuma chamada paga foi feita.

## 7. Laboratório

`cargo run --example modular_character_engine` roda o roteiro de 131 capturas, com
verificações e `RESULT`. `MCE_INTERACTIVE=1` abre o modo interativo.

| Tecla | Efeito |
|---|---|
| ← → (ou A D) | anda: corre enquanto segura, idle ao soltar |
| C | idle / corrida parado |
| M | virar |
| Espaço | pausa / continua |
| . , | próximo / quadro anterior (pausado) |
| 1 | espada: A → B → nada |
| 2 | ombreira: A → B → nada |
| 3 | peitoral sim / não |
| 4 | abordagem do peitoral: projetada → rígida → malha |
| 9 / 0 | tudo / nada |
| S | esqueleto (com a caixa do tronco) |
| P | soquetes, pivôs e eixos dos itens |
| T | camadas coloridas |
| L | camadas uma a uma (braço de trás, pernas, tronco, cabelo, braço de perto, só equipamento) |
| X | peças explodidas |
| O | esconder / mostrar os painéis de comparação |

As capturas `ui_*` são feitas apertando as teclas de verdade pelo `ButtonInput`, e cada uma
confere o estado resultante.
