# Erro-Corretion — Etapa 03B: refinamento visual e preparação para pixel art

**Estado:** a armadura piloto da Etapa 03 foi refinada no Blender, em cópia, e
reencaixada nas 4 poses (idle, C1 #0, #2 e #4) sobre o protagonista vestido
aprovado. Também há uma **prévia experimental** de redução para pixel art.
Ela não é o Prompt 04, não foi integrada ao jogo e não foi editada à mão.
Esta etapa **não se autoaprova**: o gate visual é do proprietário.

A Etapa 03 não foi alterada. Os scripts, renders e `.blend` dela foram
apenas lidos, e os dados "antes" vêm deles.

## Arquivos

| Item | Arquivo |
|---|---|
| Construtor refinado (armadura, correção do #4, materiais, luz, renders) | `scripts/build_armor_refined.py` |
| Comparações antes/depois e métricas | `scripts/compose_refined.py` |
| Prova experimental de redução | `scripts/pixel_readability.py` |
| **Modelo refinado** (armadura, rig e ações `FIT03B_*`/`FIT02B_*`) | `work/armor_refined_v002.blend` (**versionado**, 1,5 MB; exceção em `../.gitignore`) |
| Cópia de trabalho (cópia da 02B) | `work/armor_refined_work.blend` (não versionada, recriada pelo `cp` abaixo) |
| Parâmetros, peças, busca da inclinação do #4 e interseções | `out/armor_build.json` |
| Métricas antes/depois | `out/fit_metrics_refined.json` |
| Paleta complementar proposta (não aprovada) | `out/pixel_palette_03b.json` |
| Integridade (rig v007 e variantes, `.blend` da 02B e da 03, sprites aprovados) | `out/sha256_before.txt` (`sha256sum -c` OK, 0 falhas) |

Para reproduzir, a partir da raiz do projeto:

```bash
E=evidence/erro_corretion
cp $E/etapa02b/work/swordsman_rig_v007_02b_shoulders.blend $E/etapa03b/work/armor_refined_work.blend
blender --background --factory-startup $E/etapa03b/work/armor_refined_work.blend \
  --python $E/etapa03b/scripts/build_armor_refined.py -- \
  --targets $E/etapa02/out/targets.json --fit $E/etapa02/out/fit_results.json \
  --anchors $E/etapa02b/out/layers_report.json --layers $E/etapa02b/out/layers \
  --out "$PWD/$E/etapa03b/out" --save "$PWD/$E/etapa03b/work/armor_refined_v002.blend"
python3 -I -B $E/etapa03b/scripts/compose_refined.py
python3 -I -B $E/etapa03b/scripts/pixel_readability.py
```

Ambiente: Blender 5.2.4 com EEVEE (cerca de 75 s). As composições usam Python
com Pillow.

## O que foi corrigido

### 1. Quadro C1 #4 (ação nova `FIT03B_run_c1_4`)
- **Causa:** o tronco do manequim estava cerca de 2,5 px atrás do peito do
  sprite. O pescoço do rig ficava em (61,5; 32,5) e o do sprite em (64; 33).
- **Correção:** `spine_01` foi inclinado **+10°** e `spine_02` **−7°**
  (rotação no eixo Y do mundo, a partir da cabeça de cada osso).
  - O par de ângulos veio de uma busca em ±12° que minimiza a camisa do
    tronco descoberta (extensão projetada da couraça, da gola e das lâminas
    por linha). A busca também aproxima o pescoço do sprite e penaliza
    ângulos grandes.
  - Depois as clavículas foram recalculadas sobre os ombros do sprite
    (método da 02B, resíduo de 0,02–0,03 px), e os braços foram reapontados
    para os alvos da Etapa 02.
  - Pernas, raiz, câmera, escala e pivô não mudaram.
- **Resultado:**
  - Pescoço em (63,7; 33,1).
  - Ombros em (57,0; 38,0) e (63,0; 38,0); no sprite são (57; 38) e (63; 38).
  - A cintura não se moveu.
  - O sprite não foi alterado.
- **Camisa visível sobre o peito:** de **29 texels para 1**.
  - A faixa entre x 68 e 72, nas linhas 34 a 52, desapareceu.
  - O texel restante, em (70; 38), fica isolado no decote, logo abaixo do
    queixo. Não forma faixa sobre o peitoral.
- **Interseções:** contagem de pares de triângulos sobrepostos.
  - Ombreira perto × tronco: 299 → 263.
  - Ombreira longe × tronco: 264 → 293.
  - Couraça × lâminas: 414 → 316.
  - Abas × lâminas: 807 → 835.

  A inclinação não criou interseção nova relevante. Essas contagens incluem
  a sobreposição **proposital** das placas, então não medem penetração
  isoladamente. Nas prévias não notei peça atravessando outra de forma visível.

### 2. Ombreiras
- **Antes:** domo liso (subdivisão), parecido com uma esfera.
- **Agora:**
  - **Calota facetada:** 12 setores e 4 anéis, sombreamento plano, achatada
    (0,22 → 0,34).
  - **Aresta mediana** de 16 mm em relevo.
  - **Borda inferior em ponta gótica**, mais longa para baixo e larga de
    frente para trás. Na vista lateral o contorno lembra um escudo, não um
    círculo.
  - **Crista dourada curta** no topo (26 mm).
- **As 3 lâminas:** cada uma avança 10 mm para fora, em degrau, e alternam
  roxo e roxo escuro. Só a última tem faixa dourada, e cada lâmina tem um
  rebite grande.
- **Bordas douradas:** a borda da calota é dourada só no lado externo e
  inferior. Uma borda contínua em volta formava um "aro" flutuando sobre a
  couraça na vista lateral, e por isso foi removida.
- **Movimento dos braços:** a ombreira continua rígida em `upper_arm`.

### 3. Detalhes dourados
- **Tubos substituídos por faixas planas** recortadas da superfície da placa.
  - Bordas principais: 30 mm, cerca de 1,8 texel.
  - Faixas secundárias: 20 mm, cerca de 1,2 texel.
- **Quilha:** a aresta central ficou mais afiada (dois planos) e ganhou uma
  faixa dourada de duas colunas por toda a altura.
- **Nervuras em arco:** passaram a ser faixas de 20 mm, mais afastadas da
  quilha. Não leem mais como três arames paralelos.
- **Simplificação:**
  - as caneluras foram removidas;
  - só a lâmina e a aba mais baixas de cada pilha têm borda dourada;
  - os 24 rebites do abdome viraram 3 rebites frontais maiores (12 mm).

### 4. Materiais e iluminação
- **Metal pintado/esmaltado:**
  - roxo metálico 0,30, rugosidade 0,52, especular 0,30;
  - dourado metálico 1,0, rugosidade 0,40.
  - Antes o roxo era metálico 0,72 com rugosidade 0,36.
- **Luz:** os sóis `v3_*` do rig foram desligados só no render desta cópia.
  Entraram luzes controladas:
  - principal (alta, pela frente e pelo lado da câmera, 3,4);
  - preenchimento (por trás e baixo, 0,7);
  - recorte (1,6);
  - mundo mais escuro (0,45).
- **Resultado:** o reflexo redondo de plástico sumiu. A luz cai em planos:
  as facetas da ombreira e os dois lados da quilha.

### 5. Gola
- **Mais alta e mais aberta na frente:**
  - frente 1,485 → 1,545 m;
  - costas 1,525 → 1,550 m;
  - abertura 34 → 50 mm;
  - base mais larga na frente;
  - faixa dourada de 30 mm no topo.
- **Ordem de camadas aprovada mantida:** a gola continua sob o cabelo e o
  rosto.
- **Texels de gola visíveis:** idle 78, #0 22, #2 7, #4 43.
- **Testado e descartado:** costas mais altas (1,59 m). A gola aparecia como
  um bloco roxo entre as mechas do cabelo no #4.

## Medidas (texels 1×, coberto = pelo menos 50 % de armadura)

| Pose | Camisa visível (antes → depois) | Silhueta nova (antes → depois) | Gola visível (03B) |
|---|---|---|---|
| idle | 0 → 0 | 239 → 285 | 78 de 127 |
| C1 #0 | 1 → 0 | 158 → 182 | 22 de 126 |
| C1 #2 | 1 → 1 | 132 → 167 | 7 de 131 |
| C1 #4 | **29 → 1** | 108 → 145 | 43 de 130 |

- **"Antes" refeito:** os valores foram recalculados com o mesmo script. Por
  arredondamento do filtro BOX, diferem em até 1 texel do relatório da
  Etapa 03.
- **Montagem sem armadura:** idêntica ao original nas 4 poses (0 px).

## Prova experimental de redução (`out/pixel_readability_test.png`)

- **Método:**
  - BOX 8× → 1× com alfa ≥ 50 %, como na N3.5.
  - Camadas vestidas exatas da 02B.
- **Variantes de cor:**
  - **A, só a paleta N3.4:** a armadura vira cinza e marrom. Confirma que a
    N3.4 não tem roxo nem dourado.
  - **B, N3.4 com paleta complementar proposta:** 5 roxos e 3 dourados,
    escolhidos por matiz e luma, sem pontilhado.
  - **C:** B com o contorno `K` na borda externa da armadura.
- **Comparação:** a coluna "ANTES 03" é a armadura da Etapa 03 reduzida com a
  mesma paleta B.

Leitura honesta, por pergunta:

| Pergunta | Resposta |
|---|---|
| Ombreiras ainda parecem armadura? | **Em parte.** No idle e no #4 leem como placa com borda dourada e degraus. No #0/#2 a ombreira de perto, vista de lado, ainda lê como um **disco escuro com aro dourado** (medalhão), menos esférica que antes, mas ainda arredondada nessa vista. |
| Quilha central visível? | **Na vista de frente sim** (model sheet). Na câmera do jogo (perfil) a quilha fica no contorno da frente, então vira borda dourada da silhueta, não uma linha central. |
| Detalhes dourados legíveis? | **Sim:** as faixas planas viram linhas contínuas de 1 px (antes eram pontos soltos). As lâminas do abdome ainda geram algumas linhas próximas no #0. |
| Silhueta externa marcante? | **Sim**, mais larga nos ombros e no saiote. A variante C (contorno K) separa melhor do fundo. |
| Ainda parece plástico? | **Não:** blocos de roxo chapados, sem reflexo redondo. |
| Combina com o protagonista? | Combina em escala e valor. O roxo saturado domina a figura e pede, no Prompt 04, contorno e um degrau de sombra a mais. Decisão de gosto do proprietário. |

## O que ainda não está satisfatório

1. **Ombreira de perto no #0/#2:** ainda lê como medalhão redondo na vista
   lateral. Opções para depois da aprovação:
   - girar o eixo da calota para frente/baixo;
   - alongar mais a ponta;
   - deixar a crista mais alta.
2. **#4:** sobra 1 texel de camisa no decote, em (70; 38). Não é faixa, mas
   também não é zero absoluto.
3. **Gola no #2:** só 7 texels visíveis, porque o cabelo cobre quase tudo.
   Isso é correto pela ordem de camadas, mas na prática a gola não aparece
   nessa pose.
4. **Lâminas do abdome e abas:** no #2 as abas formam um "V" dourado grande
   sobre a coxa e, no #0, algumas linhas paralelas próximas.
5. **Calota da ombreira:** o facetado é perceptível em alta resolução (visual
   low-poly). Ele foi pensado para a redução, não para render final.
6. **Prévia da 03B:** mistura resoluções (render suave sobre sprite em
   nearest), como na 03. A prova de redução é a referência mais fiel.
7. **Paleta complementar:** é proposta e não foi aprovada. O mapeamento por
   matiz/luma é automático; o Prompt 04 deve revisar à mão.
8. **Não feito, porque pertence ao Prompt 04:** edição em pixel, GIF,
   manifesto e integração ao Bevy.

## Imagens para avaliar (`evidence/erro_corretion/etapa03b/out/`)

- `model_sheet_before_after.png`: armadura isolada, Etapa 03 em cima e 03B
  embaixo. Vistas: frente, lado, costas e 3/4.
- `armor_isolated_refined.png`: só a armadura 03B, nas 4 vistas.
- `fit_summary_refined.png`: as 4 poses, antes em cima e depois embaixo.
- `fit_idle_refined.png`, `fit_run_c1_0_refined.png`,
  `fit_run_c1_2_refined.png`, `fit_run_c1_4_refined.png`: cada um mostra
  original, antes, depois, armadura isolada, marcas e números.
- `pixel_readability_test.png`: prova experimental de redução.
