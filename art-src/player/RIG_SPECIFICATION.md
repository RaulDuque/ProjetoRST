# Especificação do rig do protagonista (v005 experimental, V1.2-E)

> **Status: experimental.** Rig técnico para validar deformações e iniciar a
> produção de animações. Não é arte final e não está integrado ao jogo
> (decisão D8). Resultados medidos: [`RIG_VALIDATION_V005.md`](RIG_VALIDATION_V005.md).

## 1. Origem e arquivos

`v004 → v005 rig experimental`. O build abre o `swordsman_base_v004.blend`
**somente para leitura** e salva o resultado em arquivos novos:

| Arquivo | Conteúdo |
|---|---|
| `blender/swordsman_rig_v005.blend` | v004 + armature `rig_v005` + pesos + 13 ações de pose (`POSE_P01…P13`) |
| `blender/swordsman_rig_v005_anim_tests.blend` | o mesmo + 3 ações de teste (`TEST_walk`, `TEST_attack`, `TEST_dodge`) |
| `renders/v005/` | renders, `rig_manifest_v005.json`, `reopen_check_*.json` (regeneráveis, ignorados pelo Git) |
| `comparisons/v1_2e_rig_v005/` | folhas de validação |

Scripts (nenhum script do v003/v004 foi alterado):

| Script | Papel |
|---|---|
| `scripts/bpy/build_swordsman_rig_v005.py` | Entrada: rig, pesos, validação, poses, save, renders, testes de movimento |
| `scripts/bpy/validate_rig_v005.py` | Reabre um `.blend` salvo em outro processo e revalida |
| `swordsman_rig_v005/rig_config.py` | Caminhos, versão, fatores de acompanhamento de roupa |
| `swordsman_rig_v005/skeleton.py` | Ossos, rolagens, coleções, IK e restrições de roupa |
| `swordsman_rig_v005/skinning.py` | Regras de peso por objeto e modificador Armature |
| `swordsman_rig_v005/poses.py` | Poses P01–P13 e testes T01–T11 |
| `swordsman_rig_v005/animation_tests.py` | Caminhada, ataque e esquiva curtos |
| `swordsman_rig_v005/rig_validate.py` | Verificações automáticas e severidades |
| `swordsman_rig_v005/rig_render.py` | Câmeras de revisão, célula de combate e diagrama de ossos |
| `scripts/post/rig_sheets.py` | Folhas PNG a partir do manifesto |

Reprodução (da raiz do projeto):

```bash
blender --background --factory-startup art-src/player/blender/swordsman_base_v004.blend \
    --python art-src/player/scripts/bpy/build_swordsman_rig_v005.py -- [--overwrite]
blender --background --factory-startup art-src/player/blender/swordsman_rig_v005.blend \
    --python art-src/player/scripts/bpy/validate_rig_v005.py
blender --background --factory-startup art-src/player/blender/swordsman_rig_v005_anim_tests.blend \
    --python art-src/player/scripts/bpy/validate_rig_v005.py
python3 art-src/player/scripts/post/rig_sheets.py
python3 art-src/player/scripts/post/camera_compare.py --zooms 1,1.5,2 --projectile \
    --name v005/game/P03_run_extreme_N_shaded --out-dir art-src/player/comparisons/v1_2e_rig_v005/camera_run
```

`--factory-startup` mantém add-ons do usuário (como o servidor MCP) fora do
processo. O build confere o caminho de origem, recusa salvar sobre o v004 e
não sobrescreve saídas do v005 sem `--overwrite`. Modo rápido:
`-- --preview P03_run_extreme,T03_arm_raised` (renders laterais em
`renders/v005/preview/`, nada salvo).

## 2. Convenções

- **Eixos:** o modelo olha para +X, +Z para cima, sola em z = 0.
- **Lados:** a esquerda do personagem é +Y. O lado próximo da câmera lateral
  (−Y, braço da espada) é a **direita**: sufixo `.R`. O lado distante: `.L`.
- **Repouso:** a pose do v004 (não é pose em T). O braço da espada já segura
  o sabre para baixo e para a frente.
- **Rolagem:** o eixo Z local de cada osso aponta para a frente (+X) quando o
  osso é vertical; pés, dedos e clavículas usam +Z. Assim, o X local de pernas,
  braços pendentes e painéis da casaca é o −Y do mundo, e uma rotação local X
  positiva é um balanço **para a frente** em todos eles.
- **Nomes:** ossos de deformação sem prefixo (`thigh.R`), controles com
  `CTRL_`. Não há ossos `MCH_`: o rig não precisou deles.
- **Coleções de ossos:** `DEF` (corpo), `COAT`, `CAPE`, `SWORD` (todas de
  deformação) e `CTRL`.

## 3. Hierarquia (42 ossos, 35 de deformação)

```text
root (CTRL: movimento global)
└── pelvis (quadril: controle e deformação)
    ├── spine_01 ── spine_02 ── chest
    │                 │          ├── neck ── head
    │                 │          ├── clavicle.R ── upper_arm.R ── forearm.R ── hand.R ── sword
    │                 │          └── clavicle.L ── upper_arm.L ── forearm.L ── hand.L
    │                 │                 └── cape_arm.L_01 ── cape_arm.L_02
    │                 └── cape_back_01 ── cape_back_02
    ├── thigh.R ── shin.R ── foot.R ── toe.R        (thigh.L … toe.L)
    ├── coat_front.R_01 ── coat_front.R_02          (coat_front.L, coat_back.R, coat_back.L)
root
├── CTRL_foot_ik.R / CTRL_foot_ik.L       (mesmo repouso do pé)
├── CTRL_knee_pole.R / CTRL_knee_pole.L   (40 cm à frente do joelho)
├── CTRL_hand_ik.R                         (mesmo repouso da mão da espada)
└── CTRL_elbow_pole.R                      (atrás e por fora do cotovelo)
```

## 4. Controles e IK/FK

| Controle | Osso |
|---|---|
| Movimento global | `root` (as poses aterram o personagem movendo o `root` em z) |
| Quadril | `pelvis` |
| Tronco | `spine_01`, `spine_02`, `chest` (distribua torções entre os três) |
| Cabeça | `neck`, `head` |
| Mãos | FK: `hand.R`, `hand.L`; IK: `CTRL_hand_ik.R` |
| Pés | FK: `foot.*`, `toe.*`; IK: `CTRL_foot_ik.*` + `CTRL_knee_pole.*` |
| Espada | `sword` (filho de `hand.R`; gira o sabre na mão) |
| Casaca | `coat_front.*_01/_02`, `coat_back.*_01/_02` |
| Meia-capa | `cape_arm.L_01/_02`, `cape_back_01/_02` |

- **IK:** pernas (`shin.*`, cadeia de 2, polo no joelho) e mão da espada
  (`forearm.R`, cadeia de 2, polo no cotovelo). Os pés/mão copiam a rotação
  do controle em espaço de mundo. `use_stretch` desligado (proporções fixas).
- **Alternância IK/FK:** a influência das restrições `IK`, `IK_foot` e
  `IK_hand` (0 = FK, 1 = IK); `skeleton.set_ik()` liga e desliga todas. No
  build ficam em **0**: as poses de validação são FK; o IK é exercitado por um
  teste próprio (alcance e repouso).
- **Ataques:** FK no braço da espada (arcos amplos); IK útil para guarda e
  pés plantados na caminhada/corrida.

## 5. Pesos (skinning)

Calculados por regra a partir da geometria de repouso (não por *heat
weights*, que falham com ~130 cascas abertas e sobrepostas). Máximo de 4
influências por vértice, normalizadas, mínimo 0,01. O modificador Armature
fica **antes** de Solidify e Subdivision.

| Regra | Objetos | Ossos |
|---|---|---|
| Rígido | cabeça, olhos, pálpebras, sobrancelhas, orelhas, boca, 45 mechas + calota | `head` |
| Rígido | luvas | `hand.R` / `hand.L` |
| Rígido | lâmina (N e W), guarda, cabo, pomo, colar | `sword` |
| Mistura fixa | ombreira e lâmina inferior | `upper_arm.R` 0,55 + `clavicle.R` 0,45 |
| Cadeia | pescoço | `chest` → `neck` → `head` |
| Cadeia + ombro | colete, botões, debrum, cinto e fivela, cordão | coluna por altura; perto de cada ombro, parte para clavícula e braço |
| Cadeia + coxa | tronco da calça, braguilha | coluna; perto da virilha, até 70 % para a coxa do lado |
| Cadeia | mangas, punhos, faixas | clavícula → braço → antebraço → mão |
| Cadeia | pernas, canos e costuras das botas | `pelvis` → coxa → canela → pé |
| Cadeia | pés, solas, viras, saltos, biqueiras | canela → pé → dedo |
| Gola | gola | parte de cima do tronco + até 35 % `neck` no alto |
| Casaca | casaca | acima da cintura: tronco + ombros; abaixo: 4 cadeias (frente/trás × lados) por ângulo, `_01`/`_02` por altura |
| Meia-capa | meia-capa, fecho | topo: `chest`/`clavicle.L`; resto: cadeia do braço (sobre o braço) ou das costas, com 60 % das costas seguindo o tronco embaixo |

Janelas de mistura nas articulações: ombro 4 cm, cotovelo 4,5 cm, punho
2 cm, quadril 6 cm, joelho 4,5 cm, tornozelo 3 cm. Volume preservado (dual
quaternion) em mangas, punhos, pernas, canos e pescoço.

## 6. Roupas

- **Casaca:** 4 cadeias de 2 ossos, filhas da `pelvis`. Cada `_01` copia o
  balanço da coxa do mesmo lado com restrições `Transform` (decomposição
  *swing-twist*, sem extrapolação): frente 0,9 para a frente e 0,35 para trás;
  costas 0,45 para a frente e 0,8 para trás. `coat_back.*_02` copia 0,45 da
  flexão do joelho (perna de trás chutada na corrida). Rotações extras nas
  poses dão o “drapejado” (barra das costas abrindo na corrida e na esquiva).
- **Meia-capa:** `cape_arm.L_*` (filha de `clavicle.L`) copia 100 % da
  abdução e do balanço **para trás** do braço esquerdo; o balanço para a
  frente é limitado a 0 (senão a parte das costas atravessa o tronco).
  `cape_back_*` é filha de `spine_02`. Sem física de tecido.
- **Física:** nenhuma simulação; nada vai para o Bevy (o jogo usa sprites).

## 7. Espada

- Osso `sword` filho de `hand.R`, com cabeça no centro da empunhadura e eixo
  ao longo da lâmina. Todas as peças do sabre (inclusive a variante
  `blade_readable`) têm peso 1 nele: o sabre é rígido e não atravessa a mão.
- Variantes: **N** (`blade`, largura original) e **W** (`blade_readable`,
  largura ×1,7, mesmo comprimento, fixação, pose, câmera e luz). Troca por
  `hide_render`. Nenhuma foi escolhida.

## 8. Poses e testes

As rotações das poses são escritas em **eixos do mundo no repouso**
(`poses.py`), aplicadas em FK. Y positivo inclina para a frente um osso que
aponta para cima e leva para trás um osso que aponta para baixo; X e Z
seguem a regra da mão direita. Poses no chão são aterradas pela bota mais
baixa; saltos são elevados; a morte é aterrada pelo corpo (sabre excluído).
Cada pose P01–P13 é salva como ação `POSE_<chave>` com usuário falso.

## 9. Compatibilidade com o gameplay

O rig não define dano, alcance, hitboxes, velocidade, física, tempos nem
custos. Tempos de referência (conferidos no código, não alterados):

| Ação | Tempo | Fonte |
|---|---|---|
| Ataque básico | 0,28 s | `BASIC_ATTACK_TIME`, `src/core/constants.rs` |
| Esquiva | 0,28 s | `PLAYER_DODGE_TIME` |
| Dano recebido | 0,35 s | `PLAYER_HURT_TIME` |
| Crescent Slash | 0,4 s | `execution_time`, `assets/data/skills/sword_techniques.ron` |
| Rising Edge | 0,45 s | idem |
| Ember Bolt | 0,3 s (`cast_forward`) | `assets/data/spells/elemental.ron` |
| Mending Light | 0,6 s (`cast_up`) | idem |

O jogo aplica efeitos no quadro inicial da ação. Animações definitivas terão
de acompanhar isso (impacto visual cedo) ou depender de uma mudança de
gameplay aprovada explicitamente. A taxa de quadros dos sprites não foi
decidida; os testes usam 24 fps só como referência artística.

## 10. Manutenção

- Mudou a geometria do v004? Rode o build com `--overwrite`; o relatório
  registra o hash da geometria de origem.
- Nova pose: acrescente um `Pose` em `poses.py` (eixos do mundo) e rode.
- Novo objeto de malha: acrescente uma regra em `skinning.rule_for` (o build
  falha com `no skinning rule` em vez de deixar o objeto solto).
- Mudou um fator de roupa: `rig_config.py`.

## 11. v006 (V1.2-F): estabilização

`v005 → v006`: o build abre o `swordsman_rig_v005.blend` só para leitura, mede o
v005, aplica correções pontuais, mede de novo com o mesmo código e salva
arquivos novos. O v005 e os seus scripts não mudam. Resultados e
justificativas: [`RIG_VALIDATION_V006.md`](RIG_VALIDATION_V006.md).

| Arquivo | Conteúdo |
|---|---|
| `blender/swordsman_rig_v006.blend` | v005 corrigido, armature `rig_v006` (43 ossos, 36 de deformação), ações `POSE_P01…P13` refeitas |
| `blender/swordsman_rig_v006_anim_tests.blend` | o mesmo + `TEST_walk`, `TEST_attack`, `TEST_dodge` |
| `renders/v006/` | renders antes/depois, `rig_manifest_v006.json`, `reopen_check_*.json` (ignorados pelo Git) |
| `comparisons/v1_2f_rig_v006/` | folhas de validação e comparação v005 × v006 |

| Script | Papel |
|---|---|
| `scripts/bpy/build_swordsman_rig_v006.py` | Entrada: checa a fonte, mede antes, corrige, mede depois, salva, renderiza |
| `scripts/bpy/validate_rig_v006.py` | Reabre um `.blend` do v006 em outro processo e revalida (inclui repetir as 13 ações) |
| `swordsman_rig_v006/rig_config.py` | Sobrepõe o `rig_config` do v005 (carrega os valores dele e acrescenta os do v006) |
| `swordsman_rig_v006/fixes.py` | As quatro correções (meia-capa, casaca, gola, volume) |
| `swordsman_rig_v006/validate_v006.py` | Pares extras, integridade da fonte, geometria preservada, pivôs |
| `swordsman_rig_v006/rig_render_v006.py` | Vistas de revisão (3/4 corrigida, costas pelo lado da capa, closes que acompanham ossos) |
| `scripts/post/rig_sheets_v006.py` | Folhas PNG a partir do manifesto |

```bash
blender --background --factory-startup art-src/player/blender/swordsman_rig_v005.blend \
    --python art-src/player/scripts/bpy/build_swordsman_rig_v006.py -- [--overwrite] [--no-render]
blender --background --factory-startup art-src/player/blender/swordsman_rig_v006.blend \
    --python art-src/player/scripts/bpy/validate_rig_v006.py
blender --background --factory-startup art-src/player/blender/swordsman_rig_v006_anim_tests.blend \
    --python art-src/player/scripts/bpy/validate_rig_v006.py
python3 art-src/player/scripts/post/rig_sheets_v006.py
```

Mudanças sobre as seções 3 a 6 (o resto vale como está):

- **Ossos:**
  - `collar_back` (novo, deformação, filho de `chest`, mesmo referencial do `neck`)
    copia 0,8 da rotação local do pescoço e 0,6 da cabeça.
  - `cape_arm.L_01/_02` passam a ter o pivô no ombro e o referencial de `upper_arm.L`.
- **Meia-capa:**
  - copia 100 % do balanço para trás e da abdução do braço esquerdo, e 0 do
    balanço para a frente e da adução;
  - desfaz 35 % da torção do peito para o lado da espada;
  - o topo sobre o ombro passa até 55 % para `cape_arm.L_01`.
- **Casaca:**
  - os painéis das costas seguem o avanço da coxa até 40° (18°) e voltam a 0 em 100°;
  - os painéis da frente se afastam 8° para fora a 90° de flexão;
  - na frente, os ossos da saia assumem a cintura a partir de 1,10 m.
- **Volume:** dual quaternion nas peças do pé da bota e na calça.
- **Fatores:** todos em `swordsman_rig_v006/rig_config.py`.

## 12. v007 (V1.2-G): cascas estáveis e animações técnicas

`v006 → v007`: a geometria muda em dois pontos e o rig não muda (43 ossos,
restrições e pesos idênticos). Resultados e diagnóstico:
[`ANIMATION_VALIDATION_V1_2G.md`](ANIMATION_VALIDATION_V1_2G.md).

- **Ombreiras:** o anel final de cada placa (37 vértices no mesmo ponto, 36 faces de
  área zero) foi fundido num polo; o Solidify deixou de saltar 3,5 mm.
- **Casaca e meia-capa:** Solidify com espessura simples (`use_even_offset = False`);
  a espessura uniforme invertia a casca nas dobras fortes (até 83 mm num quadro de esquiva).

| Arquivo | Conteúdo |
|---|---|
| `blender/swordsman_rig_v007.blend` | modelo + rig + 13 ações `POSE_` |
| `blender/swordsman_rig_v007_animations.blend` | o mesmo + `ANIM_walk`, `ANIM_attack_basic`, `ANIM_dodge` (chave em todo quadro, interpolação constante) |
| `renders/v007/`, `renders/v1_2g_animation_tests/` | renders, manifestos, reaberturas, sprites (ignorados pelo Git) |
| `comparisons/v1_2g_animation_tests/` | folhas, spritesheets experimentais com JSON, prévias GIF, relatório automático |

```bash
blender --background --factory-startup art-src/player/blender/swordsman_rig_v006.blend \
    --python art-src/player/scripts/bpy/build_swordsman_rig_v007.py -- [--overwrite]
blender --background --factory-startup art-src/player/blender/swordsman_rig_v007.blend \
    --python art-src/player/scripts/bpy/build_animations_v007.py -- [--overwrite] [--no-save --out DIR]
blender --background --factory-startup art-src/player/blender/swordsman_rig_v007.blend \
    --python art-src/player/scripts/bpy/validate_rig_v007.py
blender --background --factory-startup art-src/player/blender/swordsman_rig_v007_animations.blend \
    --python art-src/player/scripts/bpy/validate_rig_v007.py
python3 art-src/player/scripts/post/animation_sheets_v1_2g.py
```

O pacote `swordsman_rig_v007/` traz quatro módulos:
- `rig_config`: sobrepõe o do v006;
- `shell_fix`: correção das cascas e teste de estabilidade;
- `views_v007`: closes da ombreira;
- `animations`: clipes, IK sagital das pernas e amostragem a 24 fps.

O build das animações relê os tempos do jogo em `src/core/constants.rs`, só
leitura, e para se algum valor divergir de `rig_config`.
