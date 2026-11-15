# Validação do rig v005 (V1.2-E)

> Experimental. Números medidos no Blender 5.2.4 LTS (EEVEE) pelo build e pela
> reabertura em outro processo. Arquitetura: [`RIG_SPECIFICATION.md`](RIG_SPECIFICATION.md).
> Dados brutos: `renders/v005/rig_manifest_v005.json`, `reopen_check_*.json`,
> `sheet_metrics_v005.json` (regeneráveis, fora do Git).

## 1. Origem

- Fonte: `blender/swordsman_base_v004.blend`, aberto só para leitura.
- Hash da geometria avaliada do v004: `a907cec1da088c4c…`, igual ao da V1.2-D.
- Ligar o rig não muda o modelo em repouso: o maior deslocamento de vértice é
  4,8×10⁻⁷ m.
- Masters neutros v004 × v005: diferença média de 0,0002/255 (frente e
  costas) e 0,013/255 na lateral, onde 124 px passam de 8/255. O EEVEE já
  varia nessa ordem de grandeza entre dois renders da mesma cena.

## 2. Testes automáticos

| Teste | Resultado |
|---|---|
| Armature, 30 ossos obrigatórios, nomes únicos, sem ciclos | OK (42 ossos, 35 de deformação) |
| 132 malhas com modificador Armature primeiro e apontando para o rig | OK |
| Grupos de vértices = ossos de deformação existentes | OK |
| 38 718 vértices com influência, pesos em [0, 1], normalizados (erro 0), ≤ 4 influências | OK |
| IK: pernas e mão alcançam alvos deslocados | OK (erro ≤ 0,04 mm; deriva em repouso ≤ 0,02 mm) |
| Escala das poses (proporções) | OK (escala 1 em todas as poses) |
| Sabre preso à mão (offset, ângulo e vértices da lâmina e do cabo) | OK (deriva 0 em todas as 24 poses e testes) |
| Reabertura do `swordsman_rig_v005.blend` | OK (0 falhas): 13 ações `POSE_`, 26 materiais, 13 câmeras, 4 luzes, P06 move o braço, sabre preso |
| Reabertura do `swordsman_rig_v005_anim_tests.blend` | OK (0 falhas): também `TEST_walk`, `TEST_attack`, `TEST_dodge` |
| PNGs (RGBA, alfa 0–255; contrato 1024×1024 e combate 1024×768) | OK |

## 3. Como os problemas são medidos

- **Interseções:** pares de triângulos que se cruzam entre peças que se movem
  umas contra as outras. Conto só o que passa do repouso, porque mangas e
  casaca já se cruzam por dentro no v004 (684 e 1011 pares escondidos).
- **Volume:** malhas fechadas (pernas, mangas, pés, pescoço, quadril). Uma
  perda grande indica articulação colapsada; volume negativo indica normais
  invertidas.
- **Escala de severidade:**

| Problema | Alto | Médio |
|---|---|---|
| Casaca × perna | ≥ 300 pares | ≥ 60 pares |
| Capa × tronco ou casaca | ≥ 150 pares | ≥ 20 pares |
| Capa × braço | — | ≥ 300 pares |

Na escala de volume, abaixo de 0,75 é Crítico, abaixo de 0,85 é Alto e entre
0,93 e 1,12 fica fora (variação menor é Médio).

## 4. Poses obrigatórias

| Pose | Resultado | Problemas (Médio ou pior) |
|---|---|---|
| P01 Neutral | OK | — |
| P02 Walk contact | Baixo | — |
| P03 Run extreme | Baixo | — |
| P04 Jump rise | Baixo | — |
| P05 Jump fall | Médio | Braço esquerdo erguido para o lado entra na capa (610) |
| P06 Basic attack | Baixo | — |
| P07 Crescent Slash | Médio | Braço esquerdo para trás e para o lado entra na capa (616) |
| P08 Rising Edge | Baixo | — |
| P09 Dodge | Médio | Capa × casaca (62); capa × braço (312) |
| P10 Guard | Baixo | — |
| P11 Hurt | Médio | Capa × braço esquerdo (714) |
| P12 Magic cast (Ember Bolt) | Baixo | — |
| P13 Death | Médio | Casaca × perna esquerda (149); pés −7,7 % de volume no tornozelo dobrado |

**Nenhum problema Crítico ou Alto nas poses obrigatórias.**

Os problemas Baixos se repetem: a manga direita na casaca perto do ombro e o
cabelo da nuca na gola quando a cabeça inclina para trás.

## 5. Testes de deformação

| Teste | Resultado | Observação |
|---|---|---|
| T01 Braço abaixado | Baixo | — |
| T02 Braços na horizontal | Médio | O deltoide esquerdo levanta para dentro do topo da capa, que segue o peito (990) |
| T03 Braços elevados | Baixo | A manga direita entra no ombro da casaca (245) |
| T04 Braço cruzando o peito | Baixo | Manga × colete/casaca (178/358), esperado num cruzamento |
| T05 Braço estendido | Baixo | — |
| T06 Olhar para cima | Médio | Nuca × gola alta (607) |
| T07 Olhar para baixo | OK | — |
| T08 Rotação lateral 60° | Médio | Mandíbula × gola (226) |
| T09 Rotação do tronco | OK | — |
| T10 Agachamento (coxas 90°, joelhos 120°) | **Alto** | Casaca × pernas (314 e 502); quadril −7,7 % de volume |
| T11 Meia-capa com braço erguido | **Alto** | Capa × casaca (224) com braço a 120° e tronco torcido |

T10 e T11 são testes de estresse, além do que as poses obrigatórias pedem.
Ficam registrados como limites do rig atual, não como bloqueadores.

## 6. Testes de movimento

Os testes rodam a 24 fps (só para o teste). Ataque e esquiva têm 7 quadros
(0,28 s, como no jogo); a caminhada tem um ciclo de 16 quadros.

- **Caminhada:** pior resultado Baixo.
- **Ataque:** Médio nos quadros 2 e 3. O balanço do braço da espada para cima
  puxa a capa contra a casaca (82 e 108). O sabre passa perto da cabeça na
  preparação; isso é escolha de pose, não falha do rig.
- **Esquiva:** Médio no quadro 5 (os mesmos números da P09).
- **Interpolação entre chaves:** é a Bézier padrão. Os pés podem deslizar ou
  afundar um pouco entre as chaves.

## 7. Problemas conhecidos e menores correções propostas

1. **Meia-capa × braço esquerdo em abdução ou balanço para trás (Médio).**
   - Causa: o topo da capa segue o peito e a clavícula, então o deltoide sobe
     por dentro dele.
   - Correção proposta: dar ao topo da capa um pouco de `upper_arm.L` perto do
     ombro (pesos), ou limitar a abdução do braço esquerdo nas animações.
2. **Gola × cabeça olhando para cima ou girando (Médio).**
   - Correção proposta: um osso `collar_back` com cópia parcial da rotação do
     pescoço; ou limitar a cabeça a cerca de −20° para cima e 40° de giro.
3. **Casaca × pernas em flexão extrema (T10 Alto, P13 Médio).**
   - Correção proposta: cadeias de 3 ossos nos painéis da frente; ou ajuste
     manual da casaca por pose, que é viável porque o resultado vira sprite.
4. **Manga direita × ombro da casaca (Baixo).**
   - Vem de interseções que já existiam no v004. A correção de verdade seria
     unir manga e casaca numa malha só; ficou fora do escopo.
5. **Pés no tornozelo muito dobrado (−7,7 % de volume, Médio).**
   - Invisível na escala do jogo.
6. **Sem ossos de dedos:** as mãos estão em punho fixo, na espada e na mão
   livre. A pose de magia usa a mão fechada.
7. **Imagens 3/4 da folha de validação:** saíram pequenas (câmera longe).
   Servem de conferência; a vista lateral é a referência.

## 8. Escala do jogo

O contrato foi conferido em `docs/ASSET_CONTRACT.md`:

- célula de movimento 64×64, pivô (32, 63);
- célula de combate 128×96, pivô (40, 95);
- figura de cerca de 52 px.

| Câmera | Leitura |
|---|---|
| 1:1 | A silhueta, a capa vinho e a faixa da lâmina se leem. Braços e pernas da corrida e da esquiva viram poucos pixels, e o rosto some. Risco real de o personagem ficar pequeno demais nos movimentos rápidos. A área do cenário é a maior: 43 % da sala. |
| ×1,5 | Pernas, casaca e espada se separam bem em B e C. Pixel art (A) com escala fracionária fica irregular. |
| ×2 | Movimentos claros nos três acabamentos. A área visível cai para cerca de 11 % da sala, como já mostrado na V1.1. |

- **C (cores chapadas)** dá o melhor contraste da capa e da lâmina.
- **B** é o mais suave.
- **A** precisa de limpeza manual (a lâmina fica pontilhada).
- **Lâmina W** ajuda a ler o sabre em 1:1 nas poses de ataque e corrida; na
  neutra, pouca diferença.

**Nada aqui decide D2, D3 nem a variante da espada.**
