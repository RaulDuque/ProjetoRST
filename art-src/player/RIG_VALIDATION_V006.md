# Validação do rig v006 (V1.2-F)

> Experimental. Estabilização do rig v005, não é rig definitivo e não está
> integrado ao jogo (D8). Números medidos no Blender 5.2.4 LTS (EEVEE), no
> mesmo processo, com o mesmo código para o v005 (antes) e o v006 (depois).
> Arquitetura: [`RIG_SPECIFICATION.md`](RIG_SPECIFICATION.md) (seção 11 para
> o v006). Resultado anterior: [`RIG_VALIDATION_V005.md`](RIG_VALIDATION_V005.md).
> Dados brutos (regeneráveis, fora do Git): `renders/v006/rig_manifest_v006.json`,
> `reopen_check_*.json`, `sheet_metrics_v006.json`.

## 1. Objetivo

Corrigir as deformações que o v005 deixou em aberto (meia-capa, casaca, gola,
tornozelo) com a menor mudança possível, sem refazer o personagem, sem
alterar o v004/v005 e sem tocar no gameplay. Nada aqui decide D2, D3, a
variante da espada ou a taxa de quadros.

## 2. Estado inicial

- Branch `art/v1.2-protagonist`, HEAD `204f965` (conferido, sem commits novos).
- Já existiam, sem relação com esta missão:
  - staged: `art-src/player/inspiration/protagonist_concept_v001.png`;
  - modificações e arquivos novos da V1.2-D e da V1.2-E, não commitados;
  - `Md-usario/DarkCastleR-V1-2-e.md` e `-f.md`, não rastreados.
- Nada disso foi alterado, adicionado ao stage ou misturado.
- **MCP:** offline. O Blender do usuário estava aberto, mas nada escutava a porta
  9876, e `get_addon_status` respondeu "Could not connect". A sessão
  interativa não foi tocada. Todo o trabalho rodou em processos separados
  com `blender --background --factory-startup`.
- **Pillow:** 12.1.1 (pacote `python3-pil 12.1.1-2ubuntu1.5`), Python 3.14.4.
  - `resize(NEAREST)` continua falhando com `SystemError`; BILINEAR e LANCZOS funcionam.
  - O desvio `post/resample.py` foi mantido sem mudança.
- **Hashes:** registrados antes de começar para 598 arquivos fora de `target/`
  e `.git/`, e para o estado de `target/verify` e `target/f4-stage`.

### Arquivos inspecionados

- Documentação: `RIG_SPECIFICATION.md`, `RIG_VALIDATION_V005.md`,
  `art-src/README.md`, `docs/ART_DIRECTION.md`, `docs/ASSET_CONTRACT.md`.
- O pacote `swordsman_rig_v005/` inteiro, `build_swordsman_rig_v005.py`,
  `validate_rig_v005.py` e `post/rig_sheets.py`.
- O manifesto do v005 e os `.blend` do v001 ao v005.
- Tempos de gameplay, só leitura: `src/core/constants.rs` e `src/combat/attacks.rs`.

## 3. Estratégia

1. **Fonte:** o build abre `swordsman_rig_v005.blend` só para leitura.
2. **Integridade da fonte**, antes de qualquer mudança:
   - 42 ossos, 35 de deformação, 132 malhas;
   - estrutura e pesos válidos;
   - hash da geometria do v004 com o modificador Armature desligado: `a907cec1…`, o mesmo da V1.2-D.
3. **Medição "antes":** poses, testes, movimentos e renders de comparação no
   rig do v005, como ele está salvo.
   - A medição reproduz exatamente os números do manifesto da V1.2-E
     (`v005_manifest_reproduced.mismatches = []`).
   - Isso prova que antes e depois usam o mesmo método.
4. **Correções** (`swordsman_rig_v006/fixes.py`): uma função por problema,
   cada uma com uma causa medida.
5. **Medição "depois"** com o mesmo código, as mesmas câmeras, a mesma luz e os
   mesmos materiais.
6. **Gravação:** o build salva só como v006 e recusa caminhos do v001 ao v005.
   - Os módulos do v005 são reaproveitados sem nenhuma alteração.
   - O `rig_config` do v006 sobrepõe o do v005 no `sys.path`, como o v004
     fazia com o v003, e carrega os valores do v005 do próprio arquivo dele.

Antes de cada correção, um script de diagnóstico localizou as interseções:
ângulo em torno do corpo e altura dos triângulos que se cruzam. Isso
permitiu atacar a causa, e não o sintoma.

## 4. Ossos adicionados ou alterados

| Osso | Mudança | Por quê |
|---|---|---|
| `collar_back` (novo, deformação) | Filho de `chest`, com o mesmo referencial do `neck`. Copia 0,8 da rotação local do pescoço e 0,6 da cabeça. | A gola alta precisa acompanhar parte do giro e da inclinação da cabeça, sem ficar rígida. |
| `cape_arm.L_01` / `_02` (pivô movido) | Cabeça no ombro (`upper_arm.L`), com a mesma direção e rolagem do braço. | No v005 o pivô ficava no topo da capa e o referencial era outro. A rotação local copiada do braço virava outra rotação no mundo, e o braço atravessava a capa. |

Os demais 40 ossos mantêm cabeça, cauda e rolagem idênticas ao v005; a
comparação é automática (`pivots.moved`). O total passa a 43 ossos, 36 de
deformação.

Restrições novas ou trocadas:

| Osso | Restrição | Efeito |
|---|---|---|
| `cape_arm.L_01` | `follow_arm_back_out`, `follow_arm_fwd_in` (substituem `follow_arm`) | Copia 100 % do balanço para trás e da abdução; 0 do balanço para a frente e da adução. |
| `cape_arm.L_01` | `counter_chest_twist` | Desfaz 35 % da torção do peito para o lado da espada. Só nesse sentido: no outro piora o T09. |
| `coat_back.*_01` | `follow_thigh_fwd` (refeita) + `release_thigh_fwd` | Segue o avanço da coxa até 40° (18° de painel) e devolve tudo até 100°. Em flexão extrema, o glúteo vai para trás. |
| `coat_front.*_01` | `flare_thigh` | Afasta o painel 8° para fora a 90° de flexão da coxa, proporcional. |
| `collar_back` | `follow_neck`, `follow_head` | Cópia parcial da rotação local, misturada depois. |

No manifesto, `fixes.coat.back_panels` também lista os painéis da frente, que
receberam `flare_thigh`. Esse nome do registro é impreciso, a lista não.

## 5. Mudanças nos pesos

Só quatro malhas foram repintadas, e cinco tipos de peça de bota e a calça
passaram a usar *dual quaternion* (preservar volume). A geometria em
repouso não muda: o maior deslocamento contra o v004 é 6,0×10⁻⁷ m.

| Malha | Regra v006 |
|---|---|
| `half_cape`, `cape_clasp` | Regra do v005; o topo sobre a cabeça do ombro passa até 55 % para `cape_arm.L_01`, de 0 junto ao pescoço a 55 % sobre o ombro. |
| `coat` | Regra do v005; na frente, os ossos da saia assumem a cintura a partir de 1,10 m, com rampa de 0,10 m. No v005 eram 1,06 m com rampa de 0,14 m, e 60 % da cintura ficava na pelve enquanto a coxa subia nela. |
| `collar` | Costas e laterais vão para `collar_back`, com peso por ângulo (a partir de 0,2 rad da frente, rampa de 0,8 rad) e por altura (de 1,46 m, rampa de 0,12 m). A frente aberta mantém a regra do v005. |
| Peças da bota (`boot_foot_*`, `boot_sole_*`, `boot_welt_*`, `boot_heel_*`, `boot_toe_seam_*`) e `torso_trousers` | Mesmos pesos; modificador Armature com preservar volume. |

**Preservação fora das regiões corrigidas:** em todas as 24 poses e testes, os
vértices produzidos pelo rig (saída do modificador Armature) de todas as
outras malhas são idênticos aos do v005. A tolerância é de 10 µm, contra ruído
float32 de ~1 µm, e a comparação é relativa ao `root`. Resultado:
`unexpected_changed_meshes = {}`.

**Limitação encontrada (anterior ao rig):** o Solidify da ombreira
(`pauldron`) é numericamente instável.
- **Sintoma:** com a saída do rig idêntica (diferença ≤ 2,4×10⁻⁷ m), 217 dos 2 954
  vértices da casca final trocam de lado da espessura de 4 mm, um salto de 3,5 mm.
- **Gatilho:** basta outro objeto passar a usar *dual quaternion*. Foi
  reproduzido ligando e desligando o DQS da sola: o salto é determinístico.
- **Onde aparece:** só na P12 e na P13, e por isso a checagem compara o estágio do rig.
- **Risco para animação:** um "pulo" de sombreado na ombreira entre quadros.
- **Correção futura:** limpar a geometria da tampa da ombreira, que é modelagem
  e fica fora desta missão.

## 6. Resultados antes/depois

Severidade = pior problema medido. Os limiares são os mesmos do v005; a tabela
de severidade está em `RIG_VALIDATION_V005.md` §3. Interseções são contadas em
pares de triângulos acima do repouso. Volume = razão contra o repouso.

### Poses obrigatórias

| Pose | v005 | v006 | Antes → depois (Médio ou pior no v005; o que restou no v006) |
|---|---|---|---|
| P01 Neutral | OK | OK | — |
| P02 Walk | Baixo | Baixo | Capa × braço 154 → 154 |
| P03 Run | Baixo | Baixo | Manga × casaca 260 → 244 |
| P04 Jump Rise | Baixo | Cosmético | Gola × cabelo 112 → 0 |
| P05 Jump Fall | **Médio** | Baixo | Capa × braço 610 → 242 |
| P06 Basic Attack | Baixo | Baixo | Capa × braço 134 → 0 |
| P07 Crescent Slash | **Médio** | Baixo | Capa × braço 616 → 0 |
| P08 Rising Edge | Baixo | Baixo | Capa × braço 236 → 0 |
| P09 Dodge | **Médio** | Baixo | Capa × casaca 62 → 0; capa × braço 312 → 134 |
| P10 Guard | Baixo | Baixo | Capa × braço 120 → 120 (braço para a frente: a capa não acompanha, por escolha) |
| P11 Hurt | **Médio** | Baixo | Capa × braço 714 → 28 |
| P12 Magic Cast | Baixo | Baixo | Sem mudança (62/114) |
| P13 Death | **Médio** | Baixo | Casaca × perna 149 → 0; pés 0,923 → 0,938 |

### Testes de deformação e estresse

| Teste | v005 | v006 | Antes → depois |
|---|---|---|---|
| T01 Braço abaixado | Baixo | Baixo | Igual |
| T02 Braços na horizontal | **Médio** | Baixo | Capa × braço 990 → 128 |
| T03 Braços elevados | Baixo | Baixo | Capa × braço 0 → 208 (Baixo): a capa agora sobe com o braço e encosta nele |
| T04 Braço cruzando | Baixo | Baixo | Igual (manga × colete/casaca, esperado num cruzamento) |
| T05 Braço estendido | Baixo | Baixo | Igual |
| T06 Olhar para cima | **Médio** | Baixo | Gola × cabeça 607 → 0; nuca × gola 44 → 32 |
| T07 Olhar para baixo | OK | OK | — |
| T08 Rotação lateral 60° | **Médio** | OK | Mandíbula × gola 226 → 0 |
| T09 Rotação do tronco | OK | OK | — |
| T10 Agachamento | **Alto** | Baixo | Casaca × pernas 314/502 → 0/0; casaca × cano 244 → 0; quadril 0,923 → 0,983 |
| T11 Braço erguido + torção | **Alto** | Cosmético | Capa × casaca 224 → 0 |

### Testes de movimento

As mesmas ações do v005 foram usadas: 24 fps só para o teste; ataque e esquiva
com 7 quadros (0,28 s); caminhada com 16 quadros.

| Teste | v005 (pior) | v006 (pior) | Observação |
|---|---|---|---|
| Caminhada | Baixo | Baixo | Quadros 7, 9 e 11 passam de Baixo a OK. |
| Ataque | **Médio** (quadros 2 e 3) | Baixo | Capa × casaca 82/108 → 0. |
| Esquiva | **Médio** (quadro 5) | Baixo | Igual à P09; o quadro 4 tem manga direita × casaca 347 (Baixo, v005: 300). |

**Nenhum problema Crítico, Alto ou Médio nas 13 poses, em T01–T11 e nos três
testes de movimento.**

## 7. Classificação dos problemas

| Problema (V1.2-E) | Situação | Evidência |
|---|---|---|
| Meia-capa × braço esquerdo (P05, P07, P11, T02) | **CORRIGIDO** | Médio → Baixo/OK; closes da capa em `v005_vs_v006_problem_poses.png` |
| Meia-capa × casaca (P09, ataque nos quadros 2 e 3, esquiva no quadro 5, T11) | **CORRIGIDO** | 62/82/108/224 → 0 |
| Casaca × pernas (P13, T10) | **CORRIGIDO** nas medições; aparência **PARCIALMENTE CORRIGIDA** | 0 interseções; no T10 os painéis da frente ficam retos sobre as coxas, como uma mesa (ver §9) |
| Gola × cabeça (T06, T08) | **CORRIGIDO** | 607/226 → 0; restam toques Baixos do cabelo da nuca na gola |
| Tornozelo na P13 | **PARCIALMENTE CORRIGIDO** | Volume 0,923 → 0,938 (−7,7 % → −6,2 %), abaixo do limiar de Médio |
| Quadril no T10 | **CORRIGIDO** | 0,923 → 0,983 |
| Manga direita × ombro da casaca | **NÃO CORRIGIDO** (Baixo) | Já existe no repouso do v004 (684/1011 pares); exige unir manga e casaca numa malha (remodelagem, fora do escopo) |
| Casca da ombreira instável | **NÃO CORRIGIDO**, encontrado nesta missão | Solidify da ombreira; ver §5 |
| Sem ossos de dedos | **NÃO TESTADO** | Fora do escopo; as mãos seguem em punho fixo |

### Tornozelo: alternativa medida e não adotada

Alargar a transição de pesos canela/pé de 0,03 m para 0,05 m ou 0,07 m leva
a P13 a 0,947 e 0,953. Em troca, o salto e a sola passam a seguir a canela.
O ganho é pequeno e invisível na escala do jogo, então ficou registrado como
opção, não aplicado. O *dual quaternion* sozinho resolve o quadril (T10) e
reduz a perda do tornozelo.

### Casaca: três ossos por painel não foram adotados

O briefing pediu comparar antes de implementar. As interseções do v005 estavam:
- na cintura e no glúteo (z ≈ 0,9 a 1,0 m), **acima** da primeira articulação dos painéis;
- na lateral dos painéis da frente contra o joelho, no mesmo plano lateral.

Um terceiro osso, abaixo, não age em nenhum desses pontos.
- **O que resolveu, sem ossos novos:** a liberação dos painéis das costas, a
  cintura da frente nos ossos da saia e o afastamento lateral de 8°.
- **Tentativa que piorou:** fazer a barra da frente acompanhar 30 % da dobra
  do joelho piorou P03, P04, P09 e T10, e foi descartada.

## 8. Testes automáticos

| Verificação | Resultado |
|---|---|
| Fonte v005 íntegra (42/35 ossos, 132 malhas, hash `a907cec1…`, pesos válidos) | OK |
| Medição "antes" = manifesto da V1.2-E | OK (0 diferenças) |
| Armature, 30 ossos obrigatórios, nomes únicos, sem ciclos | OK (43 ossos, 36 de deformação) |
| 132 malhas com Armature primeiro, apontando para o `rig_v006` | OK |
| Grupos de vértices = ossos de deformação | OK |
| 38 718 vértices com influência, pesos em [0, 1], normalizados (erro 0), ≤ 4 influências | OK |
| Vértices deformáveis sem peso | 0 |
| IK (pernas e mão) | OK (erro ≤ 0,04 mm; deriva em repouso ≤ 0,02 mm) |
| Repouso igual ao v004 | OK (6,0×10⁻⁷ m) |
| Pivôs: só `cape_arm.L_01/_02` movidos e `collar_back` adicionado | OK |
| Geometria fora das regiões corrigidas | OK (estágio do rig idêntico em 24 poses) |
| Sabre preso à mão (offset, ângulo, vértices da lâmina e do cabo) | OK (deriva 0 em 24 poses e testes e nas ações reabertas) |
| Escala das poses | OK (escala 1) |
| Aterramento: ponto mais baixo em z = 0 nas poses no chão (linha do pivô do contrato) | OK nas 11 poses no chão; saltos acima do chão |
| Altura do root contra o v005 | Igual em 12 poses; P03 com 0,2 mm a menos (bota com DQS) |
| Reabertura do `swordsman_rig_v006.blend` em outro processo | OK (0 falhas): 13 ações `POSE_` repetidas, restrições do v006, DQS, 4 luzes |
| Reabertura do `swordsman_rig_v006_anim_tests.blend` | OK (0 falhas): também `TEST_walk`, `TEST_attack`, `TEST_dodge` |
| PNG das folhas (dimensões, RGB) e masters com alfa | OK (ver §10) |
| Arquivos protegidos (hashes antes/depois) | OK (ver §13) |

**Limitação da detecção de colisões:** ela conta pares de triângulos que se
cruzam, mas não diz se a parte cruzada aparece. Por exemplo, a capa atrás do
braço é invisível na lateral. Por isso cada correção foi conferida também nos
renders do lado em que o problema acontece.

## 9. Testes visuais

As folhas estão em `comparisons/v1_2f_rig_v006/`. As vistas 3/4 foram refeitas:
85 mm a 7 m, com o personagem ocupando ~80 % da altura com a espada erguida
(no v005 era ~1/5 do quadro). Foi acrescentada uma vista de costas pelo lado
da meia-capa, que a lateral do jogo esconde. Os closes acompanham o osso da
região, para não perder a área em saltos e quedas.

- **Meia-capa:**
  - P07, P11, T11: no v005 a manga e a casaca aparecem através da capa, com
    uma mancha escura na barra; no v006 a capa cobre o braço e a barra fica limpa.
  - P05: a capa abre junto com o braço erguido para o lado.
  - Não fica rígida: a parte das costas continua na cadeia própria.
- **Casaca:**
  - P09 e P13 sem atravessamento; a fenda e o forro vinho continuam visíveis.
  - P13: a frente da casaca fica mais aberta que no v005, por causa do
    afastamento de 8° e da cintura nos ossos da saia, e mostra canos e botas
    que antes ficavam cobertos. É uma mudança visível de silhueta nessa pose;
    no v005 a casaca cobria as pernas porque as atravessava.
  - T10: os painéis da frente ficam quase horizontais sobre as coxas e parecem
    duros. Num sprite, essa pose precisaria de ajuste manual da barra, e o T10
    não é uma pose do jogo.
- **Gola:** continua alta, aberta na frente e com o forro visível.
  - T06: a nuca encosta sem atravessar.
  - T08: a lateral da gola gira com a cabeça e não corta a mandíbula.
- **Tornozelo:** na P13 o peito do pé continua comprimido no close, mas sem
  colapso visível; na escala do jogo não aparece.

## 10. Comparação na escala do jogo

O contrato foi conferido e não mudou:
- ~52 px de altura;
- célula de movimento 64×64, pivô (32, 63);
- célula de combate 128×96, pivô (40, 95).

As poses do jogo (idle, walk, run, dodge e o ataque na célula de combate) foram
refeitas no v006, nos acabamentos A, B e C, com as câmeras 1:1, ×1,5 e ×2 e as
lâminas N e W (`game_scale_v006.png`).

- As correções são de 1 a 3 cm no modelo, ou seja, ≤ 1 px em 1:1. A leitura das
  silhuetas é a mesma do v005: em 1:1, corrida e esquiva continuam pequenas, e
  a capa vinho e a lâmina são o que se lê.
- A meia-capa fica do lado distante da câmera lateral: a correção mais visível
  do v006 quase não aparece na vista do jogo, mas evita o atravessamento
  quando o braço passa à frente.
- Neutral v005 × v006: diferença média de 0,0001 a 0,0002/255 e no máximo 5 px
  acima de 8/255 (`v005_vs_v006_neutral.png`). O repouso é o mesmo.

**Nada aqui decide D2, D3 ou a variante da espada.**

## 11. Estudo de FPS e sincronização (sem implementar)

Tempos conferidos no código, só leitura:
- A hitbox nasce no **primeiro quadro** da ação (`basic_attack` e
  `melee_techniques` em `src/combat/attacks.rs`).
- Ela dura 60 % da ação (`HITBOX_ACTIVE_FRACTION = 0.6`).
- A esquiva dá invulnerabilidade durante toda a sua duração.
- As durações são divididas por `attack_speed`.

| Ação | Duração | Hitbox ativa | Quadros a 12 fps | Quadros a 24 fps |
|---|---|---|---|---|
| Ataque básico | 0,28 s | 0–0,168 s | 3,4 → 3 ou 4 | 6,7 → 7 |
| Esquiva | 0,28 s | invulnerável 0–0,28 s | 3,4 → 3 ou 4 | 6,7 → 7 |
| Dano recebido | 0,35 s | — | 4,2 → 4 | 8,4 → 8 |
| Ember Bolt | 0,30 s | — | 3,6 → 4 | 7,2 → 7 |
| Crescent Slash | 0,40 s | 0–0,24 s | 4,8 → 5 | 9,6 → 10 |
| Rising Edge | 0,45 s | 0–0,27 s | 5,4 → 5 | 10,8 → 11 |
| Mending Light | 0,60 s | — | 7,2 → 7 | 14,4 → 14 |

- **12 fps:**
  - **Vantagens:** metade dos desenhos e da limpeza manual (acabamento A), e o
    aspecto "quadro a quadro" combina com pixel art.
  - **Problemas:** o ataque básico e a esquiva ficam com 3 a 4 quadros; o primeiro
    já precisa ser o golpe e sobra pouco para recuperação. Uma divisão
    fracionária deixa a última imagem com tempo diferente das outras.
- **24 fps:**
  - **Vantagens:** 7 quadros no ataque, e é a taxa dos testes atuais. Permite
    quadro de impacto, arrasto e recuperação.
  - **Problemas:** dobra a produção e a memória dos atlas. A diferença só se nota
    nas ações rápidas (≤ 0,3 s).
- **Híbrido (sugestão a decidir):**
  - 24 fps nas ações de combate curtas (ataque, esquiva, técnicas);
  - 12 fps em idle, caminhada e magia longa.
  - Exige que o controlador da V1.3 aceite taxa por animação.

**Sincronização sem mudar o gameplay:** o efeito acontece no quadro 0, então
a animação não pode ter antecipação antes do golpe.
- **Impacto primeiro:** o quadro 0 já mostra a lâmina no meio do arco, com um
  arrasto (*smear*) ou um arco de VFX nascendo junto com a hitbox; o resto da
  duração é acompanhamento e recuperação. É o que o teste de ataque do v006
  já faz parcialmente.
- **Antecipação fora da ação:** uma postura de guarda pronta no idle de
  combate, ou no fim da corrida, faz o papel da preparação sem atrasar a hitbox.
- **Escala de tempo:** como as durações dependem de `attack_speed`, a
  animação deve ser esticada para a duração real (quadros por tempo), não
  tocada com um número fixo de quadros.
- **Se um dia** quiser antecipação real dentro da ação, isso é mudança de
  gameplay (atraso da hitbox), que exige autorização e fica fora daqui.

## 12. Riscos para a animação

1. **Correções baseadas em restrições:** a capa e a casaca dependem de fatores
   e limites. Poses muito diferentes das testadas podem pedir ajuste
   (`rig_config.py`) ou correção manual por quadro, que é aceitável porque o
   resultado vira sprite.
2. **A capa não acompanha o braço para a frente** (P10, P12, T03): o braço sai
   por baixo dela. Fica Baixo e é visualmente natural, mas a borda da capa
   pode tocar a manga.
3. **Ombreira com casca instável:** possível "pulo" de sombreado entre
   quadros (§5).
4. **Interpolação Bézier entre chaves:** os pés podem deslizar entre as chaves
   nos testes de movimento, como no v005.
5. **Manga direita × casaca no ombro:** os toques Baixos existentes podem
   aparecer em closes, não na escala do jogo.

## 13. Arquivos

**Criados:**
- `scripts/bpy/build_swordsman_rig_v006.py` e `scripts/bpy/validate_rig_v006.py`.
- O pacote `scripts/bpy/swordsman_rig_v006/`:
  - `rig_config.py`;
  - `fixes.py`;
  - `validate_v006.py`;
  - `rig_render_v006.py`.
- `scripts/post/rig_sheets_v006.py`.
- Este arquivo.
- As folhas em `comparisons/v1_2f_rig_v006/`.
- Fora do Git (`.blend` e `renders/` são ignorados):
  - `blender/swordsman_rig_v006.blend`;
  - `blender/swordsman_rig_v006_anim_tests.blend`;
  - `renders/v006/`;
  - `blender/swordsman_rig_v006.blend1` e `swordsman_rig_v006_anim_tests.blend1`,
    backups automáticos do Blender ao regravar o v006 com `--overwrite`; são
    versões anteriores do próprio v006, com as mesmas correções e as mesmas
    medições (só os closes foram reenquadrados). Podem ser apagados quando quiser.

**Modificados** (só as seções do rig): `RIG_SPECIFICATION.md` (seção 11),
`art-src/README.md`, `docs/ART_DIRECTION.md`, `docs/ASSET_CONTRACT.md`.

**Preservados:**
- os scripts e pacotes do v003, v004 e v005, e `post/rig_sheets.py`;
- os `.blend` do v001 ao v005 (hashes iguais);
- os renders e as comparações anteriores;
- gameplay, RON, saves, `Md-usario/`, `.cargo/`, `target/verify` e `target/f4-stage`.

## 14. Recomendações para a V1.3

1. Aceitar o v006 como base de animação experimental ou pedir mais ajuste
   na aparência do T10, se agachamentos forem previstos.
2. Decidir a taxa de quadros: 12, 24 ou híbrido (§11).
3. Adotar a regra do "impacto primeiro" nas animações de combate.
4. Antes das animações definitivas: limpar a geometria da ombreira (Solidify
   instável) e, se quiser, unir manga e casaca no ombro. Ambas são modelagem.
5. Decidir o versionamento dos `.blend` (proposta no relatório final da V1.2-F).
6. D2, D3 e a variante da espada continuam pendentes.
