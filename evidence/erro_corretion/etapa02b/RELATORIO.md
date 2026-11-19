# Erro-Corretion — Etapa 02B: manequim ajustado e camadas da base vestida

**Resultado:** a base vestida pode ser dividida em camadas e reconstruída
**sem nenhum pixel divergente** nas 4 poses. Os ombros de C1 #0/#2 foram
corrigidos e a manga do idle foi separada do tronco. Os cabelos e o rosto
classificados como braço voltaram para a cabeça. Nenhuma armadura, corpo nu
ou anatomia oculta foi criado. Arte aprovada, N4, rig original e Etapa 02
estão intactos: `out/sha256_before.txt` e `../etapa02/out/sha256_before.txt`
foram conferidos com `sha256sum -c`.

## Arquivos

| Arquivo | Conteúdo |
|---|---|
| `work/swordsman_rig_v007_02b_work.blend` | Cópia de `../etapa02/work/swordsman_rig_v007_fit_poses.blend` |
| `work/swordsman_rig_v007_02b_shoulders.blend` | A cópia acima com as ações `FIT02B_<pose>` |
| `scripts/fit_shoulders.py` | Ajuste das clavículas. Nada é renderizado, o manequim continua invisível |
| `scripts/build_layers.py` | Camadas derivadas, verificação, zonas e prévias |
| `out/shoulders.json` | Articulações antes e depois, e ângulos das clavículas |
| `out/layers/<pose>/*.png` | Camadas de teste: partição dos texels aprovados |
| `out/reconstruction_<pose>.png` | Reconstrução "vestido sem armadura" |
| `out/preview_<pose>.png` | Original, guias, divisão em camadas, reconstrução, diferença e zonas |
| `out/zones_<pose>.png`, `out/hair_fix.png` | Zonas de sobreposição e cabelo corrigido |
| `out/layers_report.json` | Contagens, texels movidos, zonas e âncoras |

Para reproduzir, a partir da raiz do projeto:

```bash
cp evidence/erro_corretion/etapa02/work/swordsman_rig_v007_fit_poses.blend evidence/erro_corretion/etapa02b/work/swordsman_rig_v007_02b_work.blend
blender --background --factory-startup evidence/erro_corretion/etapa02b/work/swordsman_rig_v007_02b_work.blend \
  --python evidence/erro_corretion/etapa02b/scripts/fit_shoulders.py -- \
  --targets evidence/erro_corretion/etapa02/out/targets.json --fit evidence/erro_corretion/etapa02/out/fit_results.json \
  --out "$PWD/evidence/erro_corretion/etapa02b/out" --save "$PWD/evidence/erro_corretion/etapa02b/work/swordsman_rig_v007_02b_shoulders.blend"
python3 -I -B evidence/erro_corretion/etapa02b/scripts/build_layers.py
```

## 1. Ombros (só as clavículas; câmera, escala, pivô e coluna sem mudança)
As ações da Etapa 02, recarregadas, reproduzem as mesmas poses (desvio 0).

| Pose | Ombro perto | Ombro longe | Clavícula R / L (rotação em X) |
|---|---|---|---|
| C1 #0 | 2,85 → 0,03 px | 2,24 → 0,03 px | +21,8° / −17,3° |
| C1 #2 | 4,63 → 0,01 px | 3,84 → 0,02 px | +36,8° / −30,8° |

Erro médio de todas as articulações: #0 2,53 → 2,14 px; #2 3,07 → 2,65 px.
Idle e #4 não mudaram.

**Limitações:**
- **Cotovelo perto:** continua entre 4,7 e 4,8 px. O braço do rig é mais
  longo que o do sprite nesses quadros.
- **Pescoço:** continua entre 2,7 e 3,9 px, fora do escopo.
- **Pernas:** não foram mexidas e seguem como na Etapa 02.
- **Ângulo da clavícula no #2:** a queda de cerca de 37° é grande. Serve como
  guia, não como anatomia.
- **Ombro do idle:** o manequim posiciona o ombro em (40,3; 32,8). O topo da
  manga visível tem centro em (36,7; 35,1), cerca de 4 px atrás. A âncora da
  ombreira no idle passa a usar a manga visível.

## 2. Manga × tronco no idle (só texels existentes)
- **Regra:** os texels da camada camisa nas linhas 31 a 51 com x ≤ 41 viram
  `sleeve_near` (192 texels). O resto vira `shirt_torso` (105).
- **Origem dos limites:** foram lidos nos próprios texels de contorno do
  sprite e conferidos na ficha do proprietário:
  - borda de trás em x 32–33;
  - vinco da frente em x 40–41;
  - punho dobrado até a linha 51 (o antebraço começa na 52).

## 3. Cabelo e rosto classificados como braço (C1)
**Causa:** os texels da frente da cabeça (franja e testa, nariz, queixo)
ficam fora do círculo da cabeça definido em `run_poses`. O classificador da
N4-C1 os entregava ao braço mais próximo.

**Correção:** todo texel de camada de braço (perto ou longe) acima da linha do
pescoço volta para a cabeça. Pele vai para `face_skin` e o resto para
`hair_front`/`hair_back`.

| Quadro | Texels devolvidos |
|---|---|
| #0 | 53 |
| #2 | 47 |
| #4 | 34 (na manga do braço longe; não aparecia na métrica da Etapa 02) |

Nenhum punho fica acima do pescoço no C1, então nenhum texel de braço real é
afetado.

## 4. Ordem de sobreposição (do fundo para a frente)
`sleeve_far, hand_far, bracer_far, trousers_far, boots_far` → **ARMOR_SHOULDER_FAR**
→ `trousers, boots, shirt_torso` → **ARMOR_TORSO** → `sleeve_near`
→ **ARMOR_SHOULDER_NEAR** → `hand_near, bracer_near, face_skin, hair_back, hair_front`

Nas prévias, as zonas aparecem translúcidas:

| Cor | Zona |
|---|---|
| Azul | Camisa que a couraça pode cobrir |
| Amarelo | Espaço para crescer além da silhueta (3 texels, entre pescoço e cinto+3) |
| Verde | Manga próxima na frente da couraça |
| Branco | Cabelo, rosto e mão sempre na frente |
| Ciano | Membros de trás |
| Laranja / roxo | Ombreira perto / longe (raio de 6 texels) |

| Pose | Camisa coberta | Crescer fora | Ombreira perto (sobre a manga) |
|---|---|---|---|
| idle | 105 | 69 | 111 (96) |
| C1 #0 | 148 | 68 | 111 (44) |
| C1 #2 | 232 | 95 | 111 (54) |
| C1 #4 | 278 | 72 | 116 (54) |

## 5. Reconstrução sem armadura

| Pose | Texels opacos | Nas camadas | Sobrepostos | **Pixels divergentes** |
|---|---|---|---|---|
| idle | 1320 | 1320 | 0 | **0** |
| C1 #0 | 2219 | 2219 | 0 | **0** |
| C1 #2 | 2458 | 2458 | 0 | **0** |
| C1 #4 | 2487 | 2487 | 0 | **0** |

Camisa, calça, botas, cabelo, rosto, mãos e braceletes estão preservados
texel a texel.

## Limitações que continuam
- **`hand_near` do idle inclui cinto e calça:** as caixas de antebraço e mão
  de `partition.py` trazem cerca de 40 texels de cinto, calça e punho atrás
  do antebraço. Só faria diferença se a couraça descesse abaixo do cinto
  junto ao antebraço.
- **Recortes interpretados:** a manga do idle e as partições do C1 são
  recortes autorais. Exigem aprovação visual sua.
- **Pescoço e pernas:** sem ajuste nesta etapa.
