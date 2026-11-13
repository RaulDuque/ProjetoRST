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
| `player/scripts/post/` | Python + Pillow: `contract` (números do contrato), `palette` (fonte única de cores), `mock_figure`, `pixelize`, `finish_compare`, `camera_compare`, `model_sheet`, `version_compare`, `labels` |
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

## Rig: avaliação (implementação adiada)

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
