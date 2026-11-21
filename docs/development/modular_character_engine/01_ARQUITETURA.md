# Dark Castle Modular Character Engine — arquitetura e protótipo 01

Briefing do proprietário: `Md-usario/engine/Engine-01.md` (não versionado por mim; a pasta
`Md-usario/` é do proprietário). Relatório da etapa:
[`docs/relatorios/modular-character-engine/01_PROTOTIPO.md`](../../relatorios/modular-character-engine/01_PROTOTIPO.md).

## 1. Ideia central

A **movimentação pertence ao personagem**: quadros aprovados, cada um com um esqueleto
invisível marcado. O **equipamento só declara** a que osso se prende, em que ponto, com que
ângulo, quanto acompanha o osso e em que profundidade é desenhado. Uma peça desenhada uma vez
acompanha qualquer pose que tenha esqueleto, sem gerar outra imagem do protagonista.

## 2. Duas abordagens comparadas no protótipo

| | Híbrida (proposta) | Totalmente articulada (comparação) |
|---|---|---|
| Corpo | Quadros aprovados N3.5/N3.6, intactos | Peças recortadas do idle aprovado |
| Movimento | O dos quadros aprovados | FK com os ângulos dos quadros + IK nas pernas |
| Profundidade | 5 camadas cortadas pelo esqueleto, sem perda (0 texels diferentes) | Ordem por osso; membros de trás são cópias escurecidas |
| Equipamento | Preso aos ossos do quadro | Preso aos mesmos ossos, na pose articulada |
| Qualidade | Igual à arte aprovada no corpo | Muito abaixo: juntas abertas, cabelo caído, perspectiva fixa, texels girados |
| Pose nova | Exige arte do corpo (só o equipamento é modular) | Sai dos dados, mas com a qualidade acima |

Conclusão do protótipo: a abordagem **híbrida** é a que preserva a identidade visual; a
articulada serve hoje só como diagnóstico e como fonte de poses-guia.

## 3. Módulos

Regras puras em `src/player/visual/rig/`, sem Bevy, com testes unitários:

| Arquivo | Responsabilidade |
|---|---|
| `xform.rs` | Transformação rígida 2D (`Xform2`): composição pai·local, inversa, ângulos, distâncias |
| `skeleton.rs` | `Joint` (15 marcos), `Bone` (12 ossos, hierarquia com a espinha como raiz), `fit` (pose a partir dos marcos), `forward` (FK), `retarget` (ângulos de uma pose nas proporções de outra) |
| `ik.rs` | IK de dois ossos (lei dos cossenos, lado da dobra, limites de alcance) |
| `layers.rs` | Material pela paleta N3.4, `BodyLayer` (5), `DepthSlot` (ordem de desenho com slots de equipamento entre as camadas), partição sem perda, recorte por osso |
| `equipment.rs` | `RigItem` (dados do item), `place` (pose → posição/rotação), `mirror`, `RigLoadout` |
| `articulated.rs` | Pose articulada: FK com proporções do repouso + IK por perna |
| `data.rs` | Contrato RON do laboratório (`RigLab`), validação |

Sistemas Bevy só no exemplo isolado `examples/modular_character_engine.rs` (três painéis,
roteiro de capturas, verificação). **O jogo não usa o subsistema**: nada foi registrado em
`PlayerVisualPlugin`; combate, física, inventário, progressão e saves não mudaram.

Dados: `art-src/modular-character-engine/stage/rig_lab.ron`. Ferramentas offline (Python,
fora do jogo): `art-src/modular-character-engine/scripts/`.

## 4. Convenções

- **Espaço do rig:** texels, `x` para onde a arte olha, `y` para cima, origem no pivô da sola
  do clipe; ângulos em radianos, anti-horário. É a convenção de `sockets.rs` (V2.2).
- **Marcos no RON:** coordenadas de texel lidas na imagem (y para baixo), convertidas ao
  carregar.
- **Perto/longe:** "near" é o lado da câmera com a arte olhando à direita. Ao espelhar, o
  sprite inteiro vira e a ordem de profundidade fica igual (convenção dos jogos 2D de
  referência): o braço da frente continua na frente, agora como a outra mão anatômica.
- **Pernas A/B:** cruzam na vista lateral; qual é a de perto é interpretação (mesma ressalva
  da N4-C). A perna A é a de trás nos quadros 0–2 da corrida e a da frente nos quadros 3–5.

## 5. Ordem de profundidade (de trás para a frente)

`BackGear` → `FarArm` → `FarHandGear` → `Legs` → `Torso` → `Hair` → `MainHandGear` →
`NearArm` → `ShoulderGear`.

A espada fica sobre corpo e cabelo, mas sob o punho que a segura (a mão cobre o cabo); a
ombreira fica sobre o braço. Capa (`BackGear`) e escudo na mão de trás (`FarHandGear`) já têm
lugar, mas não foram implementados.

## 6. Contrato de um item

```ron
"sword_arming_cross": (
    slot: MainHand, texture: "factory://sword_arming_cross.png",
    size: (21, 64), pivot: (10.0, 53.5),        // texel da imagem que vai no soquete
    bone: forearm_near, at: End, offset: (0.0, 0.0),
    up_degrees: 90.0,   // o "para cima" da imagem, relativo ao osso
    follow: 1.0,        // quanto do giro do osso o item acompanha (ombreira: 0.5)
    depth: MainHandGear, provisional: false, source: "...",
),
```

Rotação final = ângulo do quadro (osso, ou pai + fração do giro desde o repouso) +
`up_degrees` − 90°.

## 7. Origem de cada parte

| Parte | Origem |
|---|---|
| Composição FK `world = parent · local`, pais antes dos filhos | Agent Sprite Forge, `rig_animate.py` (MIT, © 2026 0x0funky), reescrito em Rust |
| IK de dois ossos (lei dos cossenos, lado da dobra) | Agent Sprite Forge, `solve_two_bone` (MIT), reescrito; limite de alcance como em pixelfx `rig.js` `ik` (MIT, © 2026 Patch, Hugo Duprez); limite "perto demais" novo |
| Pé mantendo o ângulo do mundo depois da IK | Agent Sprite Forge (`end.rotate = foot_angle − …`) |
| Ordem de desenho por slot, independente da hierarquia | Agent Sprite Forge `rig.v1` (`data-z`) |
| Arma presa à mão por um pivô de empunhadura com eixo fixo à mão | pixelfx `rig.js` (modelo de arma) |
| Respiração de 1 texel no idle | pixelfx `act.js` ("breath"), que credita Hollow Crown (MIT, © 2026 rehan-remade) |
| Ferramentas de QA consideradas (`pixel_qa.py`, `pixel_motion.py`, `build_animation_clips.py`) | Agent Sprite Forge; avaliadas e não usadas nesta etapa |
| `VisualPlayback`/`playback::loop_frame` (um relógio), `sockets::place` (espelho), `manifest::anchor_for` (pivô), fontes de asset isoladas e capturas do Bevy | Dark Castle (V1.3, V2.2, N4-A) |
| Paleta N3.4 e grades `.txt` dos quadros aprovados | Dark Castle (N3.4, N3.5, N3.6) |
| Espadas | Fábrica N41-B (usadas como estão; arte não aprovada) |
| Ombreiras | Fábrica N41-B, reduzidas 2:1 por `derive_pauldrons.py` (derivadas e **provisórias**) |
| Esqueleto marcado à mão, partição em camadas por cápsulas do esqueleto, `follow` parcial, comparação híbrido × articulado, contrato RON, laboratório | Desenvolvido para o Dark Castle nesta etapa |

Nenhum código foi copiado literalmente dos projetos externos: os algoritmos foram reescritos em
Rust, e os créditos estão nos comentários de `ik.rs`, `equipment.rs` e `articulated.rs`.

## 8. O que foi reaproveitado e o que não foi

- **Mantido e reaproveitado:** `VisualPlayback`/`loop_frame` (o relógio único),
  `sockets::place`, `anchor_for`, o padrão de laboratório do `n4_sprite_machine` (fontes de
  asset, roteiro, capturas, `RESULT: PASS/FAIL`), a arte aprovada da N3.8 e as peças da N41-B.
- **Não usado nesta etapa:**
  - `composite.rs`/`composition.rs` (N4-A): compõem folhas por camada já pintadas; aqui as
    camadas saem do esqueleto em tempo de carga. A integração natural é a próxima etapa
    (seção 9).
  - `equipment_visual.rs` (N4-1-A): equipamento como folhas por clipe e quadro. O modelo novo
    troca isso por uma imagem única presa a um osso.
  - Rig v007 do Blender: não foi aberto. Os ossos daqui são 2D e marcados nos quadros
    aprovados. O v007 vale como fonte futura de marcos para clipes novos.
  - `modular_lab.rs` (D12): loadouts já assados por combinação, o oposto do pedido.
- **Fábrica de máscaras da camisa:** não foi usada.

## 9. Evolução prevista (não implementada)

1. Integrar a partição e os itens ao `CompositeCharacter` (um slot de camada por
   `DepthSlot`), lendo o mesmo `VisualPlayback` do jogador.
2. Osso de mão/pulso com ângulo próprio por clipe, para que a espada não fique sempre
   perpendicular ao antebraço (ver o idle).
3. Equipamento articulado (cadeia de peças rígidas, por exemplo um cinturão de placas) e
   flexível (capa: malha deformada pelos ossos), sobre o mesmo `place`.
4. Paletas e materiais por item (recoloração por rampa, como na N41-B).
5. Efeitos visuais sincronizados por eventos de quadro (`act.js`/`rig_anim.v1` têm o modelo).
6. Marcação assistida dos marcos (as medidas `pixel_motion.py` e as poses do Blender v007).

## Atualização (02)

A etapa 02 estendeu este desenho: empunhadura por perfil, contrato de 10 slots, peitoral na gaiola do tronco, camadas da N4-C e marcos refinados. Ver [`02_EQUIPAMENTOS.md`](02_EQUIPAMENTOS.md).
