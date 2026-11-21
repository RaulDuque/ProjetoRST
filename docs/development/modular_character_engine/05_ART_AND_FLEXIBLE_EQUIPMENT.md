# Modular Character Engine 05 — produção artística e equipamentos flexíveis

Briefing: `Md-usario/engine/Engine-05.md` (do proprietário). Relatório:
[`docs/relatorios/modular-character-engine/05_ART_AND_FLEXIBLE_EQUIPMENT.md`](../../relatorios/modular-character-engine/05_ART_AND_FLEXIBLE_EQUIPMENT.md).
Etapa anterior: [`04_STUDIO_V2.md`](04_STUDIO_V2.md).

```bash
cargo run --example equipment_studio                         # o editor (capas incluídas)
cargo run --example equipment_studio -- --tour               # roteiro numa cópia isolada (inclui a Engine 05)
cargo run --example equipment_preview -- NOME ITEM...        # composições 1x sem janela (iteração de arte)
cargo run --example equipment_preview -- --trace gothic_cape # ângulo da barra da capa: parado -> corrida -> parado
python3 -B art-src/equipment-studio/scripts/preview_sheet.py NOME
PATH=~/.local/share/dc-art-tools/asf-venv/bin:$PATH python -B art-src/equipment-studio/scripts/run_tools_05.py
python3 -B art-src/equipment-studio/scripts/make_evidence_05.py
```

## 1. Como a arte foi produzida

Tudo é dado no formato do Equipment Studio (RON). Nenhuma imagem do protagonista equipado
foi gerada. O laço de trabalho:
1. editar o RON (ou ajustar no editor);
2. `equipment_preview` compõe o item sobre a arte aprovada nas 8 poses, pelas mesmas funções
   puras que o editor mostra;
3. ver em 1×, 3× e 6×;
4. ajustar e repetir;
5. validar no editor e no roteiro.

Uma sobreposição da gaiola do tronco (u, v) sobre os quadros nus orientou as silhuetas.

Itens novos (os da Engine 04 continuam intactos):

| Item | Mudanças em relação à Engine 04 |
|---|---|
| `gothic_v2_cuirass` | perfil anatômico: o peitoral avança sobre o peito (u 1,15) e recolhe embaixo; borda inferior em V sobre a placa do abdome, que afina na cintura; quilha com luz e sombra; gola escura com um friso dourado; placa de trás mais escura |
| `gothic_v2_pauldron` | espaldar em cúpula mais largo que o braço (muda a silhueta), duas lâminas em crescente, crista; um único friso dourado |
| `gothic_v2_bracers` | cotoveleira, placa que afina do cotovelo ao pulso, punho dourado que termina antes da mão; braçal de trás no antebraço de trás |
| `gothic_v2_faulds` | duas faldas curtas de borda curva e duas escarcelas, uma presa a cada coxa (`bind` em `thigh_a` / `thigh_b`, `LegGear`): acompanham as pernas |
| `leather_v2_jerkin` | o gibão do Modelo B com as rampas de couro, correia e ferro refinadas (geometria igual) |

Materiais: metal roxo mais escuro e menos saturado (sombras profundas, luz e brilho
definidos), aço escuro para partes secundárias, dourado só em frisos. A linha de ornamento
ganhou `material` opcional: o friso é contínuo, não uma faixa fina que se partia em traços.
Os frisos ficam pelo menos um texel dentro da borda, porque a borda é repintada pelo contorno.
Foram evitadas faixas horizontais repetidas.

## 2. Equipamento flexível (`rig::flex`, puro)

`EquipmentDesign.flex: Vec<FlexChain>` (opcional, os RON antigos continuam válidos). Uma parte
raiz com `bind: Flex(chain: k)` é desenhada no espaço da cadeia:
- x: texels ao longo da cadeia a partir da fixação;
- y: texels de través (+ para o corpo).

| Campo (`FlexChain`) | Significado |
|---|---|
| `anchor` | fixação na gaiola do tronco (u, v) |
| `segments`, `length` | quantidade de segmentos (1–16) e comprimento em texels |
| `stiffness`, `damping` | mola para o alvo (encurta a resposta) e amortecimento por 1/60 s |
| `influence`, `gravity` | arrasto do ar da corrida e gravidade |
| `rest_deg`, `limits`, `bend` | ângulo de repouso, faixa de ângulos (a partir da vertical, + para trás) e dobra máxima entre segmentos |
| `flutter`, `flutter_hz`, `shelter` | ondulação forçada que desce pela cadeia; parte coberta pelo corpo (o arrasto cresce ao longo dela) |

Simulação (`FlexState`):
- cada segmento é um pêndulo amortecido em torno de um ângulo alvo (repouso + elevação do ar),
  acoplado ao segmento de cima;
- os segmentos de baixo são mais lentos, então a barra se atrasa;
- a aceleração do corpo joga o pano para trás na partida e para a frente na parada;
- os saltos da fixação entre poses mexem os ângulos como num pêndulo;
- nenhum segmento fica à frente da linha das costas, exceto logo depois de uma virada, quando
  volta;
- `turn_around` espelha o pano na virada: ele fica onde estava no mundo e depois volta para
  trás.

**Primeira versão descartada:** partículas Verlet com restrição follow-the-leader. Ela
balançava caoticamente a cada salto da fixação entre poses (medido: a barra ia de 0° a 92° no
mesmo loop). A forma por ângulos é estável com os mesmos dados.

`settle` roda o loop do clipe até repetir e devolve a cadeia em cada quadro. É a pose
determinística usada pelo pintor, pelos testes e pelas prévias. O editor passa a simulação ao
vivo (`paint_on_frame_with`).

**Pintor:** `SpaceMap::Chain(Arc<ChainMap>)` projeta cada texel na curva suavizada da cadeia
(Chaikin aberto, 2 passes) e devolve (ao longo, de través). Polígonos, regiões, ornamentos e
partes de pixels funcionam no espaço da cadeia como em qualquer outro.

**Profundidade:** a capa usa `BackGear`, atrás do corpo, do cabelo e da espada. Por isso não
há interseção visível com braços, pernas e espada. O fecho na garganta é uma parte rígida na
gaiola (`ChestGear`). A ordem cabelo × ombreira não mudou, e nenhuma exceção de profundidade
foi necessária.

**Velocidade do corpo:** `RigClip.ground_speed` (opcional). A corrida aprovada usa 110 texels/s,
uma estimativa de dois passos de ~22 texels por loop de 0,4 s. O editor suaviza a velocidade
com `BODY_ACCEL` (450 texels/s²) nas partidas e paradas.

## 3. Capas

| Item | Cadeia | Silhueta |
|---|---|---|
| `gothic_cape` | 8 segmentos, 50 texels, rigidez 0,6, arrasto 4, ondulação 0,6 a 2,5 Hz, abrigo 0,4 | pano longo com forro vinho, barra dourada, duas dobras |
| `gothic_cape_short` | 6 segmentos, 32 texels, rigidez 0,8, arrasto 3,5, ondulação 0,5 a 5 Hz, abrigo 0,3 | manto sobre os ombros e barra recortada |

A segunda capa foi feita só com dados: nenhuma linha de Rust mudou para ela.

Medido no traço (`--trace gothic_cape`):
- partida: a barra sobe em cerca de 0,5 s, passa um pouco e fica entre 47° e 54° na corrida;
- parada: desce em cerca de 0,4 s, com 19% de ultrapassagem para a frente, e assenta em 0,65 s.

## 4. Editor

- Pano ao vivo por item vestido, com o mesmo relógio:
  - trocas e equipar ou desequipar não reiniciam a animação;
  - um pano novo começa na forma assentada do quadro atual.
- Partida e parada com a velocidade suavizada.
- O espelhamento é uma virada.
- Os roteiros usam passo fixo de 1/60 s (repetível).
- Painel "CLOTH CHAIN" à esquerda: segmentos, comprimento, arrasto, amortecimento, rigidez,
  gravidade, repouso, alcance, dobra, ondulação, abrigo e fixação (u, v), todos com desfazer.
- A cadeia aparece nas guias (pontos e fixação).
- Regiões: botões de material e de tom da região em edição.
- O vínculo de parte oferece as cadeias do design.
- Espaçamento dos painéis mais compacto. Não houve reescrita.

Roteiro (`--tour`): 98 passos, 26 da Engine 05. Os passos da Engine 05 são:
1. tira as peças da 04;
2. mostra o corpo nu;
3. veste cada peça sozinha;
4. veste o conjunto com espada;
5. veste o couro com as peças góticas e a capa;
6. grava, quadro a quadro, idle, corrida, partida e parada, viradas, trocas durante a corrida
   e a capa curta;
7. edita um parâmetro do pano e desfaz.

A gravação fica em `target/equipment_studio/v2/rec/`.

## 5. Ferramentas externas

| Ferramenta | Uso | Resultado |
|---|---|---|
| ASF `pixel_qa.py` | 7 itens novos isolados nas 8 poses contra a paleta das próprias rampas (mais os PNGs importados) | `off_palette` 0 e `partial_alpha` 0 em todos |
| ASF `pixel_qa.py --onion` | 6 quadros da corrida (conjunto com capa e espada; só a capa) | `loop_seam` 1,041 e 1,017 (limite 2,0) |
| ASF `rig_animate.spring(k, d)` | referência independente para a parada da capa (resposta normalizada) | melhor mola k 106, d 16,5, RMS 0,086, subamortecida |
| pixelfx 0.3.1 | procurado módulo de pano ou movimento secundário | não existe (só efeitos, `rig.js` e poses); o modelo de pintura do `rig.js` continua no pintor |
| Blender | não usado: as peças são planas no espaço do corpo, e a projeção 2D bastou | — |

Nenhuma chamada paga e nenhuma instalação.

## 6. Limitações

- A arte não está aprovada. O pintor de polígonos dá um acabamento mais "liso" que o pixel
  art feito à mão.
- A capa fica sempre atrás do corpo. Uma capa que passe à frente (de lado, envolvendo) pediria
  uma regra de profundidade por pose.
- A velocidade da corrida é uma estimativa.
- Partida e parada usam a pose idle e a de corrida, porque não há clipe de transição.
- A ondulação é forçada, não aerodinâmica.
- As escarcelas presas às coxas usam a mesma profundidade nas duas pernas.
