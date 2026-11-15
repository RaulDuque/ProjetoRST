# art-src: fontes de produção artística

Fontes e ferramentas de arte. **Nada aqui é carregado pelo jogo.** O runtime
lê só `assets/`. Formatos, nomes e pivôs: [`docs/ASSET_CONTRACT.md`](../docs/ASSET_CONTRACT.md).
Direção visual: [`docs/ART_DIRECTION.md`](../docs/ART_DIRECTION.md).

Status: **experimental (V1.1/V1.2)**. Nada aqui foi aprovado como arte final.

## Estrutura

| Pasta | Conteúdo |
|---|---|
| `player/scripts/bpy/` | Scripts Blender (`bpy`). v001/v002: `config`, `geometry`, `materials`, `camera`, `lighting`, `render`, `export_frames`, `previews`, `presentation`, `build_base_model` (entrada) |
| `player/scripts/bpy/swordsman_v003/` | Modelo v003 (V1.2-C): `params`, `mesh_tools`, `surfaces`, `anatomy`, `head`, `clothing`, `sword`, `staging`, `validate`; entrada `bpy/build_swordsman_v003.py` |
| `player/scripts/bpy/swordsman_v004/` | Modelo v004 (V1.2-D): substitui `params` e `surfaces` do v003 e acrescenta `shapes`, `face`, `body`, `garments`, `validate`; reusa o resto do v003 sem alterar. Entrada `bpy/build_swordsman_v004.py` |
| `player/scripts/bpy/swordsman_rig_v005/` | Rig experimental v005 (V1.2-E), derivado do v004: `rig_config`, `skeleton`, `skinning`, `poses`, `animation_tests`, `rig_validate`, `rig_render`; entradas `bpy/build_swordsman_rig_v005.py` e `bpy/validate_rig_v005.py`. Especificação: [`player/RIG_SPECIFICATION.md`](player/RIG_SPECIFICATION.md) |
| `player/scripts/bpy/swordsman_rig_v007/` | v007 e animações técnicas (V1.2-G): `rig_config`, `shell_fix`, `views_v007`, `animations`; entradas `bpy/build_swordsman_rig_v007.py`, `bpy/build_animations_v007.py`, `bpy/validate_rig_v007.py`; folhas e sprites `post/animation_sheets_v1_2g.py` |
| `player/scripts/bpy/swordsman_rig_v006/` | Rig experimental v006 (V1.2-F), correções sobre o v005: `rig_config`, `fixes`, `validate_v006`, `rig_render_v006`; entradas `bpy/build_swordsman_rig_v006.py` e `bpy/validate_rig_v006.py`; folhas `post/rig_sheets_v006.py` |
| `player/scripts/post/` | Python + Pillow: `contract` (números do contrato), `palette` (fonte única de cores), `mock_figure`, `pixelize`, `finish_compare`, `camera_compare`, `model_sheet`, `version_compare`, `scale_compare`, `rig_sheets`, `resample`, `labels` |
| `player/blender/` | Arquivos `.blend` (criados pelo `build_base_model.py`) |
| `player/references/` | Referências visuais, com origem e licença |
| `player/inspiration/` | Folha de inspiração `protagonist_concept_v001.png` (registrada em `references/README.md`) |
| `player/renders/` | Saídas geradas (masters, sprites A/B) |
| `player/comparisons/` | Folhas de comparação para aprovação |

## Estado do Blender

- **V1.1:** o Blender não estava instalado. Os scripts `bpy/` foram escritos
  sem executar e testados só contra um `bpy` falso.
- **V1.2-B:** Blender **5.2.4 LTS** instalado (snap, `/snap/bin/blender`).
  Os scripts rodaram no Blender real, em processo separado
  (`--background --factory-startup`), com EEVEE (`BLENDER_EEVEE`):
  - `v001`: geometria da V1.1 sem alterações. Rodou sem erro de API.
  - `v002`: casaca aberta com forro, gola alta aberta, meia-capa como casca,
    franja, olhos, tronco arredondado e sombreamento suave; vistas 3/4 e de
    detalhe (`presentation.py`).
  - `export_frames.py` continua **não validado** (não há rig nem animação).
- **V1.2-C:** `v003` reconstruído com malhas procedurais (seções anatômicas
  lofteadas, tubos ao longo de curvas, tecido paramétrico com Solidify e
  Subdivision), sem primitivas definindo o corpo. Detalhes, parâmetros e
  limitações: seção "Modelo v003" abaixo.
- **MCP do Blender:** o complemento está em *safe mode*, que proíbe `sys`,
  `open`, `os` e importar módulos do disco. Por isso o pipeline não roda pelo
  MCP; o MCP serve para inspecionar a cena aberta e rodar trechos curtos.
  O build em processo separado também preserva a cena aberta pelo usuário.
- **Prévias da V1.1:** vêm de `post/mock_figure.py`, a figura vetorial
  provisória. Continuam em `player/comparisons/` sem alteração.

## Como reproduzir (a partir da raiz do projeto)

Prévias atuais (só Pillow):

```bash
python3 art-src/player/scripts/post/mock_figure.py
python3 art-src/player/scripts/post/pixelize.py art-src/player/renders/mock_side_master.png \
    --out-dir art-src/player/renders
python3 art-src/player/scripts/post/finish_compare.py
python3 art-src/player/scripts/post/camera_compare.py
```

Modelo-base no Blender (V1.2-B, versão em `bpy/config.py`, hoje `v002`):

```bash
blender --background --factory-startup \
    --python art-src/player/scripts/bpy/build_base_model.py
OUT=art-src/player/comparisons/v1_2b_swordsman_base_v002
python3 art-src/player/scripts/post/pixelize.py \
    art-src/player/renders/swordsman_base_v002_side_master.png \
    --out-dir art-src/player/renders --name swordsman_base_v002_side
python3 art-src/player/scripts/post/model_sheet.py --version v002 --out $OUT/model_sheet_v002.png
python3 art-src/player/scripts/post/finish_compare.py --name swordsman_base_v002_side \
    --out $OUT/finish_A_pixel_vs_B_hd_v002.png \
    --subtitle "Render real do Blender 5.2.4 (modelo-base v002, vista lateral). Experimental, não é arte final."
python3 art-src/player/scripts/post/camera_compare.py --zooms 1,1.5,2 --projectile \
    --name swordsman_base_v002_side --out-dir $OUT
```

Sem `--zooms`, `--projectile` e `--subtitle`, os scripts de comparação geram
as mesmas imagens da V1.1 (conferido byte a byte na V1.2-B).

**Proteções do script Blender:**
- só limpa uma cena não salva (`--factory-startup`);
- só escreve em `player/blender/` e `player/renders/`;
- não sobrescreve `.blend`, masters nem vistas de apresentação da mesma versão
  sem `--overwrite` (com ele, o Blender guarda o anterior como `.blend1`).

Saídas regeneráveis (`player/renders/`, `__pycache__/`) e `.blend` são ignoradas
por `art-src/.gitignore`.

## Modelo v003 (V1.2-C)

```bash
blender --background --factory-startup \
    --python art-src/player/scripts/bpy/build_swordsman_v003.py      # modelo, validação, renders
blender --background --factory-startup \
    --python art-src/player/scripts/bpy/build_swordsman_v003.py -- --relight-v002   # v002 com a luz da v003
R=art-src/player/renders/v003; OUT=art-src/player/comparisons/v1_2c_swordsman_v003; P=art-src/player/scripts/post
for v in side_master side_master_toon side_master_readable; do
    python3 $P/pixelize.py $R/swordsman_base_v003_$v.png --out-dir $R --name swordsman_base_v003_${v/_master/}; done
python3 $P/model_sheet.py --version v003 --layout v003 --renders $R --out $OUT/model_sheet_v003.png
python3 $P/model_sheet.py --version v003 --layout v003 --renders $R --background dark --out $OUT/model_sheet_v003_dark.png
python3 $P/version_compare.py --out $OUT/compare_v002_v003.png
python3 $P/finish_compare.py --renders $R --name swordsman_base_v003_side --out $OUT/finish_A_pixel_vs_B_hd_v003.png
python3 $P/finish_compare.py --renders $R --name swordsman_base_v003_side_toon --out $OUT/finish_C_flat_v003.png
python3 $P/finish_compare.py --renders $R --name swordsman_base_v003_side_readable --out $OUT/finish_readable_blade_v003.png
python3 $P/camera_compare.py --zooms 1,1.5,2 --projectile --name v003/swordsman_base_v003_side --out-dir $OUT/camera_shaded
python3 $P/camera_compare.py --zooms 1,1.5,2 --projectile --name v003/swordsman_base_v003_side_toon --out-dir $OUT/camera_flat
```

- **Saídas:** `blender/swordsman_base_v003.blend`, `renders/v003/` (masters
  frontal, lateral e traseira na escala do contrato; 3/4; closes de rosto,
  cabelo, casaca, meia-capa, botas e espada; lateral em cores chapadas;
  lateral com lâmina ×1,7) e `renders/v003/build_report_v003.json`
  (versão do Blender, motor, todos os parâmetros, checagens).
- **Luz:** rig preso a um empty que gira com cada câmera; todas as vistas
  recebem a mesma luz relativa à câmera.
- **Teste C (cores chapadas):** materiais `v3t_*` com duas faixas de luz
  (`Shader to RGB` + rampa constante), emitindo a cor exata da paleta.
- **Limitações conhecidas:** rosto estilizado ainda simples (deslocamentos
  suaves sobre um elipsoide); peças separadas que se interpenetram por
  dentro (escondido); ~235 mil triângulos renderizados por causa da
  subdivisão, sem otimização para tempo real (o jogo usa sprites).

## Modelo v004 (V1.2-D)

Refinamento localizado do v003, não reconstrução: mesma pose, landmarks,
câmeras, luz e espada. O pacote `swordsman_v004/` vem **antes** de
`swordsman_v003/` no `sys.path` só no build do v004: `params` e `surfaces`
dele substituem os do v003, e os módulos do v003 sem equivalente (`mesh_tools`,
`sword`, `staging`, `anatomy` para pescoço e mãos, `clothing` para a gola) são
reusados sem alteração. O build do v003 nunca enxerga o v004.

```bash
blender --background --factory-startup \
    --python art-src/player/scripts/bpy/build_swordsman_v004.py -- [--overwrite]
blender --background --factory-startup \
    --python art-src/player/scripts/bpy/build_swordsman_v004.py -- --preview close_face,side_master  # renders/v004/preview/
R=art-src/player/renders/v004; OUT=art-src/player/comparisons/v1_2d_swordsman_v004; P=art-src/player/scripts/post
for v in side_master side_master_toon side_master_readable side_master_readable_toon; do
    python3 $P/pixelize.py $R/swordsman_base_v004_$v.png --out-dir $R --name swordsman_base_v004_${v/_master/}; done
python3 $P/model_sheet.py --version v004 --layout v004 --renders $R --out $OUT/model_sheet_v004.png
python3 $P/model_sheet.py --version v004 --layout v004 --renders $R --background dark --out $OUT/model_sheet_v004_dark.png
V3="art-src/player/renders/v003/swordsman_base_v003_{view}_master.png"; V4="$R/swordsman_base_v004_{view}_master.png"
python3 $P/version_compare.py --out $OUT/compare_v003_v004.png --title "Espadachim: v003 × v004, mesmas vistas, escala, luz e fundo" \
    --old "v003 (V1.2-C)" "$V3" --new "v004 (V1.2-D, refinamento)" "$V4"
python3 $P/version_compare.py --out $OUT/compare_v003_v004_closes.png --title "Espadachim: v003 × v004, closes" \
    --old "v003 (V1.2-C)" "$V3" --new "v004 (V1.2-D, refinamento)" "$V4" \
    --closes close_face,close_hair,close_coat,close_cape,close_boots,close_sword
python3 $P/scale_compare.py --old v003 --new v004 --out $OUT/game_scale_v003_v004.png --metrics $R/game_scale_metrics_v003_v004.json
python3 $P/finish_compare.py --renders $R --name swordsman_base_v004_side --out $OUT/finish_A_pixel_vs_B_hd_v004.png \
    --subtitle "Render real do Blender 5.2.4 (modelo v004, vista lateral). Experimental, não é arte final."
python3 $P/finish_compare.py --renders $R --name swordsman_base_v004_side_toon --out $OUT/finish_C_flat_v004.png \
    --subtitle "Teste C, cores chapadas (modelo v004). Experimental, não é arte final."
python3 $P/finish_compare.py --renders $R --name swordsman_base_v004_side_readable --out $OUT/finish_readable_blade_v004.png \
    --subtitle "Lâmina ×1,7 (variante visual; alcance inalterado), modelo v004. Experimental."
for n in side:camera_shaded side_toon:camera_flat side_readable:camera_readable; do
    python3 $P/camera_compare.py --zooms 1,1.5,2 --projectile --name v004/swordsman_base_v004_${n%%:*} --out-dir $OUT/${n##*:}; done
```

- **O que mudou:** rosto com malha mais densa na frente, planos e arestas
  (ponte do nariz, linha da mandíbula, maçãs), olhos amendoados com pálpebras,
  íris âmbar sem pupila e emissão baixa, sobrancelhas afiladas, lábios; cabelo
  com mechas de seção em lente e pontas finas; colete separado da calça (bainha
  com duas pontas, debrum, pesponto da braguilha); cinto com fivela vazada,
  passador e furos; casaca com bordas e forro lisos (sem o serrilhado do v003),
  fenda traseira real, três famílias de dobras, costuras e folga explícita das
  pernas; mangas, pernas e botas com perfil anatômico (`shapes.profiled_tube`);
  botas com cano mais alto na frente, costura lateral, biqueira, vira, sola e
  salto; materiais com `Specular IOR Level` por superfície e relevo procedural.
- **Bug do v003 registrado (não corrigido no v003):** em `head._base`,
  `gauss(theta, 0, 0.9)` recebe θ em [0, 2π), então a metade próxima do rosto
  nunca é achatada (bochecha inchada de um lado). Corrigido só no v004.
- **Validação:** `validate.py` também mede interseções entre peças que o rig
  vai mover (casaca × pernas e botas é falha; as demais são informativas).
- **Pillow 12.1.1 do sistema:** `Image.resize(NEAREST)` falha com
  `SystemError` em qualquer tamanho. `post/resample.py` cai para uma
  transformação afim equivalente; as folhas de câmera do v003 saem idênticas
  pixel a pixel com o fallback.
- **Limitações:** rosto continua uma superfície de deslocamentos (sem narinas
  abertas, dobras de orelha ou lábio sobreposto, que exigem escultura manual);
  ~404 mil triângulos renderizados (v003: ~235 mil), sem otimização.

## Rig experimental v005 (V1.2-E)

`v004 → v005 rig experimental`: armature de 42 ossos (35 de deformação),
pesos por regra, cadeias da casaca e da meia-capa, sabre preso a um osso
filho da mão, poses de validação P01–P13 e testes curtos de movimento.
O build abre o `.blend` do v004 só para leitura e salva arquivos novos.
Comandos, convenções e hierarquia: [`player/RIG_SPECIFICATION.md`](player/RIG_SPECIFICATION.md).
Resultados medidos e problemas conhecidos: [`player/RIG_VALIDATION_V005.md`](player/RIG_VALIDATION_V005.md).

## Rig experimental v006 (V1.2-F)

`v005 → v006`, estabilização: o build abre o `.blend` do v005 só para leitura,
mede, corrige a meia-capa (pivô no ombro), a casaca (painéis das costas e
afastamento da frente), a gola (osso `collar_back`) e o volume (tornozelo e
quadril, dual quaternion), mede de novo com o mesmo código e salva arquivos
novos. 43 ossos, 36 de deformação. Comandos: seção 11 de
[`player/RIG_SPECIFICATION.md`](player/RIG_SPECIFICATION.md). Resultados antes/depois:
[`player/RIG_VALIDATION_V006.md`](player/RIG_VALIDATION_V006.md).

## Sprites do jogo e publicação segura (V1.5)

Desde a V1.5, **nenhum script de pós-processamento escreve em `assets/`**. Eles
geram um *staging* (mesma estrutura de `assets/`); só
`post/publish_guard.py` publica, depois de validar PNG, manifesto, pivôs,
durações do jogo, versão do gerador (≥ a publicada) e a presença de todos os
clipes já publicados. A publicação faz backup em `player/renders/publish_backups/`,
grava por arquivos temporários com journal e desfaz tudo se algo falhar. O que
está publicado fica em `assets/animations/player.publish.json`.

```bash
blender --background --factory-startup art-src/player/blender/swordsman_rig_v007.blend \
    --python art-src/player/scripts/bpy/build_game_sprites_v1_5.py -- [--overwrite]
python3 art-src/player/scripts/post/game_sprites_v1_5.py [--hurt hurt_a|hurt_b|v1_4] [--fall fall_v15|v1_4]
python3 art-src/player/scripts/post/publish_guard.py status          # mostra o state id publicado
python3 art-src/player/scripts/post/publish_guard.py check --stage art-src/player/renders/v1_5_game_sprites/stage
python3 art-src/player/scripts/post/publish_guard.py publish --stage art-src/player/renders/v1_5_game_sprites/stage --confirm <state id>
python3 art-src/player/scripts/post/publish_guard.py recover         # só após uma publicação interrompida
python3 -m unittest discover -s art-src/player/scripts/post/tests    # testes do guarda
```

Os VFX (`post/vfx_v1_5.py`) são desenhados com PIL e entram no mesmo staging
(atlas `effects`). Comparações em `player/comparisons/v1_5_art_polish/`;
demonstração: `cargo run --example player_demo` e depois
`python3 art-src/player/scripts/post/demo_v1_5.py --v14-assets <backup V1.4>`.
Documento: [`../docs/PLAYER_ART_POLISH_V1_5.md`](../docs/PLAYER_ART_POLISH_V1_5.md).

**Scripts históricos:** `post/game_sprites_v1_3.py` e `post/game_sprites_v1_4.py`
continuam reprodutíveis, mas só geram staging, e o guarda recusa publicá-los
(versão antiga e, no caso da V1.3, clipes faltando).

## Sprites do jogo (V1.4)

Mais 11 clipes do v007 (dano, morte, pulo, queda, aterrissagem, agachar, guarda,
Crescent Slash, Rising Edge, Ember Bolt, Mending Light):

```bash
blender --background --factory-startup art-src/player/blender/swordsman_rig_v007.blend \
    --python art-src/player/scripts/bpy/build_game_sprites_v1_4.py -- [--overwrite]
python3 art-src/player/scripts/post/game_sprites_v1_4.py   # junta V1.3 + V1.4 (V1.5: só staging)
```

Folha e relatório em `player/comparisons/v1_4_game_sprites/`. Integração:
[`../docs/PLAYER_ANIMATION_EXPANSION_V1_4.md`](../docs/PLAYER_ANIMATION_EXPANSION_V1_4.md).
**V1.5:** o risco de `post/game_sprites_v1_3.py` regravar os atlas só com 4
clipes foi eliminado: os scripts escrevem em staging e o `publish_guard.py`
recusa a publicação (ver a seção V1.5).

## Sprites do jogo (V1.3)

Idle (12 quadros, 12 fps), run (10 quadros, 24 fps, feito para 190 px/s),
attack_basic e dodge (0,28 s, da V1.2-G), acabamento C e lâmina ×1,7, a partir
do v007. Comandos:

```bash
blender --background --factory-startup art-src/player/blender/swordsman_rig_v007.blend \
    --python art-src/player/scripts/bpy/build_game_sprites_v1_3.py -- [--overwrite]
python3 art-src/player/scripts/post/game_sprites_v1_3.py
```

O segundo comando escreve `assets/sprites/player/player_{locomotion,combat}.png`
e `assets/animations/player.ron`. Folhas e relatório:
`player/comparisons/v1_3_game_sprites/`. Integração no jogo:
[`../docs/PLAYER_VISUAL_ANIMATION_V1_3.md`](../docs/PLAYER_VISUAL_ANIMATION_V1_3.md).

## v007 e animações técnicas (V1.2-G)

`v006 → v007`: polo das ombreiras fundido e espessura simples no Solidify da
casaca e da meia-capa (cascas estáveis), rig sem mudança. Três animações de
teste (`walk` 16 quadros em loop; `attack_basic` e `dodge` em 0,28 s exatos, a
24 fps, com 12 fps derivado), sprites A/B/C no contrato de células e pivôs,
spritesheets experimentais e prévias GIF. Nada integrado ao jogo. Comandos:
seção 12 de [`player/RIG_SPECIFICATION.md`](player/RIG_SPECIFICATION.md).
Resultados: [`player/ANIMATION_VALIDATION_V1_2G.md`](player/ANIMATION_VALIDATION_V1_2G.md).

## Rig: avaliação (implementação adiada)

> Avaliação original da V1.2-C, mantida como histórico. O rig experimental
> foi implementado na V1.2-E (seção anterior) sem remodelar o corpo como
> malha única: cada peça recebe pesos próprios.

| Parte | Proposta | Observação |
|---|---|---|
| Tronco e cabeça | Cadeia quadril → coluna (2 ossos) → pescoço → cabeça | Suficiente para respirar e se inclinar no ataque |
| Braços | Ombro, braço, antebraço e mão, com IK na mão da espada | A espada fica presa à mão como filha do osso |
| Pernas | Coxa, canela e pé com IK e alvo de joelho | O pouso e a corrida dependem do IK |
| Espada | Osso próprio, filho da mão | Permite troca de arma por substituição de malha |
| Capa e casaca | 2 ou 3 ossos em cadeia, ou física de tecido **assada** | A física em tempo real nunca vai para o jogo |

O modelo atual é feito de primitivas soltas, sem malha contínua nem pesos.
Um rig útil exige remodelar o corpo como malha única, o que aumentaria muito o
escopo desta fase. Por isso fica **registrado como tarefa futura**, antes da
V1.4 (primeiras animações).
