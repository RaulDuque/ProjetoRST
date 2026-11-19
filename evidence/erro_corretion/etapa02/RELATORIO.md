# Erro-Corretion — Etapa 02: encaixe do rig v007 nas quatro poses aprovadas

**Resultado: PRECISA DE AJUSTE.** O rig encaixa como manequim invisível para
tronco e ombros, sem redesenhar o personagem. Ainda há erros localizados,
principalmente nas pernas e na altura dos ombros durante a corrida, e eles
estão listados abaixo. Nenhuma armadura foi criada.

## O que foi feito

| Passo | Arquivo |
|---|---|
| Evidências da Etapa 01, copiadas sem alteração | `evidence/erro_corretion/etapa01/` |
| Marcos das poses, só a partir de dados existentes | `scripts/build_targets.py` → `out/targets.json` |
| Encaixe e guias no Blender, só na cópia de trabalho | `scripts/fit_rig.py` → `out/fit_results.json`, `out/guide_*.png`, `work/swordsman_rig_v007_fit_poses.blend` (ações `FIT_idle`, `FIT_run_c1_0/2/4`) |
| Medições e folhas | `scripts/analyze.py` → `out/fit_*.png`, `out/occlusion_*.png`, `out/summary.png`, `out/measurements.json` |
| Integridade | `out/sha256_before.txt` (`sha256sum -c` OK: rig v007, ações, `candidate_106.png` e `run6.png` intactos) |

Para reproduzir, a partir da raiz do projeto:

```bash
python3 -I evidence/erro_corretion/etapa02/scripts/build_targets.py
cp art-src/player/blender/swordsman_rig_v007.blend evidence/erro_corretion/etapa02/work/swordsman_rig_v007_work.blend
blender --background --factory-startup evidence/erro_corretion/etapa02/work/swordsman_rig_v007_work.blend \
  --python evidence/erro_corretion/etapa02/scripts/fit_rig.py -- \
  --targets evidence/erro_corretion/etapa02/out/targets.json --out "$PWD/evidence/erro_corretion/etapa02/out" \
  --save "$PWD/evidence/erro_corretion/etapa02/work/swordsman_rig_v007_fit_poses.blend"
python3 -I evidence/erro_corretion/etapa02/scripts/analyze.py
```

### Origem dos marcos
- **C1 #0, #2 e #4:** `run_poses.py`, esqueletos marcados à mão na N4-1.
  Usados como estão.
- **Idle:** não existe esqueleto do idle. Os marcos foram derivados
  mecanicamente das tabelas de anatomia de `reconstruct.py` e das caixas de
  `partition.py`, ambas da N4-A. A regra de cada marco está em
  `targets.json → derivation`. Os membros de trás ficam ocultos no perfil e
  não foram medidos.

### Câmera, escala e pivô
- **Escala:** constante de 58,53 px/m. A altura de repouso do modelo
  (1,811 m) vale 106 texels, igual ao contrato N3.5/N3.6.
- **Câmera:** ortográfica lateral em −Y, com `ortho_scale` = 128 / escala e
  centralizada no canvas.
- **Pivô:** a coluna do pivô corresponde a X = 0. A borda de baixo da linha
  118 corresponde a Z = 0.
- **Render:** Workbench com cor plana por grupo, sem antialiasing, fundo
  transparente e supersample de 8×.
- **Redução para 1×:** pelo critério da N3.5. Um texel fica opaco quando
  ≥ 50 % do bloco está coberto, e o grupo é o da maioria.
- **Grupos ocultos:** casaca, meia-capa, ombreira, sabre e mechas de cabelo
  ficam escondidos. O rig é só manequim.

### Como o rig é posado
- FK no plano lateral. Cada osso gira em torno do eixo Y do mundo, passando
  pela cabeça do osso, até a ponta projetada apontar para o marco.
- Os comprimentos dos ossos nunca mudam. Por isso o erro que sobra é o que as
  proporções do rig não alcançam.
- A torção do peito e a da pelve (em torno de Z) reproduzem a separação
  horizontal entre os ombros e entre os quadris.
- A raiz é reajustada pelo resíduo médio (parcela de câmera/posição) e o
  encaixe é refeito, em 4 passes.

## Erros após a conversão final (texels do canvas)

| Pose | Média | Máx. | ≤ 2 px | ≤ 4 px | Raiz vs. quadril travado | Torção peito / pelve |
|---|---|---|---|---|---|---|
| idle | 1,63 | 6,14 (joelho) | 7/10 | 9/10 | (−0,3; +0,8) | 0° / 0° |
| C1 #0 | 2,53 | 7,07 (tornozelo perto) | 6/18 | 15/18 | (+2,0; +1,4) | −12° / +23° |
| C1 #2 | 3,07 | 6,07 (joelho longe) | 4/18 | 14/18 | (+2,6; −1,4) | −16° / +23° |
| C1 #4 | 2,12 | 5,78 (tornozelo perto) | 9/18 | 17/18 | (−2,5; +1,4) | +19° / −23° |

Topo e sola do idle: o guia ocupa as linhas 13 a 118, e o sprite também.
No #0, a bota de trás do rig passa 3 linhas abaixo do chão (linha 121).

### Câmera/escala × diferença real
- **Câmera, escala e posição não explicam os erros.** O deslocamento de
  câmera/posição é de no máximo 2,6 px e o topo e a sola do idle coincidem.
- **As proporções explicam.** Comparando o comprimento dos segmentos (sprite
  medido em projeção, máximo entre as poses, contra o rig):

  | Segmento | Sprite (máx.) | Rig |
  |---|---|---|
  | Braço | 18,1 | 15,2 |
  | Antebraço | 16,6 | 14,1 |
  | Coxa | 25,0 | 26,4 |
  | Canela | 30,1 | 23,7 |

  Os sinais são mistos: a coxa do rig é mais longa e a canela mais curta.
  Isso indica **diferença real de proporção nas pernas**, não de escala. É a
  causa dos erros de joelho e tornozelo de 4 a 7 px.
- **Ombros na corrida:** nos quadros #0 e #2, os ombros e o pescoço do rig
  ficam de 3 a 4,6 px mais altos que os do sprite. No idle a diferença é de
  só 0,3 px. Conclusão: é postura de corrida (ombros caídos), e o rig ainda
  não usa as clavículas.
- **Cintura:** no idle, o rig está na linha 55,2 e o cinto do sprite na 51.
  Na corrida coincide (59). O `belt_y` do idle vem de onde a camada de calça
  começa e o da corrida vem de `run_poses`, ou seja, as duas convenções já
  diferem nos dados da N4.
- **Cotovelo do idle (3,5 px):** o marco é o fim da manga (`UPPER_ARM`), que
  não necessariamente coincide com a articulação. É ambíguo.

## Oclusão (braço cruzando o peito)

| Pose | Braço do sprite sobre o tronco | Braço do rig sobre o tronco | Rig dentro do braço do sprite | Braço do sprite coberto pelo rig | Cabelo nas camadas do braço |
|---|---|---|---|---|---|
| idle | 106 (braço interpretado, sob a camisa) | 170 | 89 % | 69 % | 0 |
| C1 #0 | 201 | 136 | 79 % | 50 % | 53 |
| C1 #2 | 164 | 107 | 73 % | 45 % | 47 |
| C1 #4 | 144 | 64 | 71 % | 47 % | 0 |

- **A ordem de profundidade bate:** nos quatro quadros, o braço próximo do rig
  passa na frente do peito, como no sprite.
- **A forma do oclusor não bate:** a manga do rig é fina (manga de casaca) e a
  do sprite é ampla e dobrada. Por isso as máscaras de oclusão do braço devem
  sair das **camadas aprovadas**: `arm_front`, `shirt` (manga próxima) e
  `bracers` na corrida. O render do rig não serve para isso. O rig fornece só
  o eixo e a profundidade.
- **Defeito da partição N4-C1:** nos quadros #0 e #2, de 47 a 53 texels de
  cabelo acima do pescoço foram classificados em `arm_front`/`shirt`. É
  preciso filtrá-los antes de usar essas camadas como oclusor. Nada foi
  alterado.

## Volume externo além da camisa

Teste só de geometria de máscara, sem armadura:
1. Cresci o tronco do sprite em 2 texels.
2. Subtraí o que precisa ficar na frente: o braço próximo e o cabelo.
3. No idle, o oclusor é o braço interpretado (`arm_front` + `bracers`). A
   camada `shirt` original não foi tocada.

| Pose | Texels novos fora da silhueta do corpo | Visíveis após oclusores | Ocultos pelo braço/cabelo | Tronco do rig fora do corpo do sprite |
|---|---|---|---|---|
| idle | 53 | 263 | 128 | 23 |
| C1 #0 | 42 | 225 | 150 | 29 |
| C1 #2 | 61 | 326 | 154 | 45 |
| C1 #4 | 42 | 369 | 133 | 0 |

**Sim, o pipeline consegue preservar a silhueta original do corpo e
acrescentar volume externo à armadura.** As camadas aprovadas não mudam. A
peça passa do contorno da camisa (42 a 61 texels novos por pose) e é cortada
por máscaras de braço por pose. O tronco do rig tem volume parecido com o do
sprite (até 45 texels de diferença) e serve como superfície-base.

**Ressalva do idle:** a manga está dentro de `shirt`. Para a manga aparecer
**por cima** da armadura, os texels de camisa dentro da área do braço precisam
ser desenhados acima da peça. Há dois caminhos: recortar isso na composição
ou criar uma camada `sleeve` no idle. Os dois dependem da sua decisão, e nada
foi feito.

## Ajustes por pose (antes ou durante o Prompt 03)

| Pose | Ajustes |
|---|---|
| idle | Joelho 6 px (proporção da perna; não afeta peça de tronco). Confirmar o cotovelo (fim da manga ≠ articulação). A borda inferior da peça deve seguir o cinto do sprite (51), não o do rig (55). Decidir a separação da manga. |
| C1 #0 | Baixar as clavículas cerca de 3 px. Tornozelo perto 7 px e bota de trás 3 px abaixo do chão (perna). Cotovelo perto 4,8 px. |
| C1 #2 | Baixar clavículas e pescoço cerca de 4 px. Joelho de trás 6 px e pés cerca de 5 px (perna). |
| C1 #4 | Antebraço de trás curto (pulso e punho a 3–4 px). Tornozelo perto 5,8 px (perna). |
| Todas | Opcional, na cópia: escalar a coxa em cerca de −5 % e a canela em cerca de +15 % para equipamentos de perna. Filtrar o cabelo das camadas de braço do #0 e do #2. |

## Riscos que continuam
- Os esqueletos do C1 e os marcos do idle são interpretação da N4. O encaixe
  só é tão bom quanto eles.
- A torção do peito/pelve é aproximada: o rig gira rígido, enquanto o sprite
  é um vídeo reduzido.
- Peças que dependem de mão, perna ou pé (armas, botas, grevas) vão precisar
  do ajuste de proporção. Peça de tronco ou ombro, não.
- Ainda não sabemos se a renderização + redução fica bonita. Isso é assunto
  dos Prompts 03 a 05.
