# Erro-Corretion — Etapa 03: piloto de armadura gótica roxa e dourada

**Estado:** modelo 3D criado e encaixado nas 4 poses aprovadas (idle e C1 #0,
#2, #4), sobreposto ao protagonista **vestido**. Ainda não é pixel art: a
redução é o Prompt 04. Esta etapa **não se autoaprova**; o gate visual é do
proprietário.

## O que foi feito

| Item | Arquivo |
|---|---|
| Cópia de trabalho (do `.blend` da 02B) | `work/armor_pilot_work.blend` (não versionado) |
| Modelo salvo: armadura + rig + ações `FIT02B_*` e `FIT03_idle` | `work/armor_pilot_v001.blend` (não versionado, reproduzível) |
| Construtor da armadura e renders | `scripts/build_armor.py` |
| Prévias sobre o personagem vestido | `scripts/compose_preview.py` |
| Parâmetros, peças, ossos e ombro do idle | `out/armor_build.json` |
| Métricas de cobertura e silhueta | `out/fit_metrics.json` |
| Integridade (rig original, `.blend` 02B, sprites aprovados) | `out/sha256_before.txt` (`sha256sum -c` OK) |

Para reproduzir, a partir da raiz do projeto:

```bash
cp evidence/erro_corretion/etapa02b/work/swordsman_rig_v007_02b_shoulders.blend evidence/erro_corretion/etapa03/work/armor_pilot_work.blend
blender --background --factory-startup evidence/erro_corretion/etapa03/work/armor_pilot_work.blend \
  --python evidence/erro_corretion/etapa03/scripts/build_armor.py -- \
  --targets evidence/erro_corretion/etapa02/out/targets.json --fit evidence/erro_corretion/etapa02/out/fit_results.json \
  --anchors evidence/erro_corretion/etapa02b/out/layers_report.json \
  --out "$PWD/evidence/erro_corretion/etapa03/out" --save "$PWD/evidence/erro_corretion/etapa03/work/armor_pilot_v001.blend"
python3 -I -B evidence/erro_corretion/etapa03/scripts/compose_preview.py
```

### Modelo (90 objetos, geometria própria)
Cada peça está presa rigidamente a um osso do manequim invisível, nas poses
aprovadas.

| Peça | Construção | Osso |
|---|---|---|
| **Couraça** | Casca fechada que segue as seções do tronco do manequim, afastada 5,4 cm na frente e atrás e 3,2 cm dos lados. Superelipse, quilha central gótica, caneluras, decote e cavas, ponta de plackart. Espessura real (Solidify 12 mm) + subdivisão. | `chest` |
| **Gola alta (gorjal)** | Mais alta atrás e aberta para fora | `chest` |
| **3 lâminas do abdome** | Sobrepostas, cada uma um pouco maior, com rebites | `spine_01`, `pelvis` |
| **4 pares de abas (tassets)** | Frente e lado, presas às coxas | perto: `thigh.R`; longe: `thigh.L` |
| **Ombreiras** | Domo achatado + 3 lâminas + crista dourada + rebites | `upper_arm.R` / `upper_arm.L` |

- **Dourado:** frisos em curva (bordas, quilha, nervuras em arco gótico) e
  rebites.
- **Materiais (Principled):** roxo `(0,105; 0,032; 0,165)` metálico 0,72,
  rugosidade 0,36; roxo escuro; dourado `(0,78; 0,52; 0,16)` metálico 1,0.
- **O que não é renderizado:** nenhuma malha original do rig (corpo, roupas,
  casaca, capa, sabre, cabelo).

### Câmera e poses
- **Câmera:** a mesma da Etapa 02, ortográfica lateral, 58,53 px/m, coluna do
  pivô em X = 0 e borda da sola da linha 118 em Z = 0, com supersample de 8×.
- **Poses da corrida:** `FIT02B_*`, com as clavículas corrigidas na 02B.
- **Idle (ajuste novo, `FIT03_idle`):** as clavículas foram giradas para o
  ombro cair sobre o topo da manga visível medido na 02B, em (36,7; 34,1),
  em vez de (40,3; 32,8). O braço foi reapontado para os mesmos alvos da
  Etapa 02.

### Composição
- **Ordem de camadas aprovada na 02B, com as abas de trás abaixo dos membros
  de trás:** `abas longe` < membros de trás < `ombreira longe` <
  calça/botas < camisa < **couraça** < manga perto < **ombreira perto** <
  mão, bracelete, rosto, cabelo.
- **Ajuste derivado só nesta etapa (`out/layers_refined/idle/`):** a camada
  `hand_near` do idle carregava cinto, punho e calça atrás do antebraço (as
  caixas da N4-A). Esses texels agora ficam abaixo da couraça. Pele e
  contorno `K` continuam na frente.
- **Montagem sem armadura:** idêntica ao original nas 4 poses (0 px na
  prévia).

## Medidas (texels 1×; um texel conta como coberto com ≥ 50 % de armadura)

| Pose | Camisa do tronco | Coberta | Ainda visível | Armadura | Visível após manga/mão/cabelo | Silhueta nova (fora do corpo) |
|---|---|---|---|---|---|---|
| idle | 105 | 105 | 0 | 761 | 466 | 238 |
| C1 #0 | 148 | 147 | 1 | 795 | 508 | 158 |
| C1 #2 | 232 | 231 | 1 | 836 | 512 | 132 |
| C1 #4 | 278 | 249 | **29** | 882 | 585 | 107 |

## Problemas e limitações (não escondidos)

1. **Faixa de camisa no #4:** uma faixa de 1–2 texels na frente do peito,
   em x 68–72 nas linhas 34–52, continua aparecendo. A causa é o tronco do
   manequim estar cerca de 2,5 px atrás do peito do sprite nesse quadro (o
   mesmo erro de pescoço já medido na Etapa 02 e aceito para o piloto).
   Aumentar a couraça só reduziu de 36 para 29.
   **Correção possível:** inclinar o peito do manequim no #4 e refazer as
   clavículas.
2. **Acabamento ainda com aparência de CG:**
   - os frisos dourados são tubos finos (parecem arame);
   - as ombreiras ainda são bastante esféricas;
   - o metal tem brilho liso de plástico;
   - faltam bordas recortadas ou cusped, caneluras profundas e variação de
     placa.

   Na redução para cerca de 106 px de altura (Prompt 04), detalhes menores
   que 1–2 cm (17–34 mm por texel) somem. Por isso o que vai pesar é
   silhueta, blocos de cor e bordas douradas largas: os frisos provavelmente
   precisarão ficar mais grossos e planos.
3. **Prévia mistura resoluções:** a armadura em alta resolução é sobreposta
   ao sprite ampliado em nearest. Isso **não** representa o resultado final.
4. **Volume:** a armadura é mais larga que a camisa (até 238 texels de
   silhueta nova no idle). Ombreiras e abas aumentam bastante a figura; isso
   é intencional, mas é decisão de gosto.
5. **Manga do idle:** a manga dobrada da camisa (linhas 46–51) aparece na
   frente da couraça, como manda a ordem aprovada. Pode ler como uma faixa
   cinza na cintura.
6. **Abas:** acompanham a coxa (rígidas no osso). Em passadas grandes, a aba
   de trás se separa das lâminas. A gola fica quase toda escondida atrás do
   cabelo e do rosto na vista lateral.
7. **Ainda não testado:** quantização na paleta N3.4 (sem roxo/dourado),
   integração no Bevy e GIF (fazem parte do Prompt 04).

## Imagens para avaliar
- `out/model_sheet.png`: frente, lado, costas e 3/4, só a armadura.
- `out/fit_summary.png`: as 4 poses vestidas com armadura, lado a lado.
- `out/fit_idle.png`, `out/fit_run_c1_0.png`, `out/fit_run_c1_2.png`,
  `out/fit_run_c1_4.png`: original, armadura isolada, armadura sobre o
  vestido e marcas (camisa coberta, nova silhueta, camisa ainda visível).
