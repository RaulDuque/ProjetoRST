# Dark Castle Equipment Studio — Modular Character Engine 03

Briefing: `Md-usario/engine/Engine-03.md` (do proprietário). Relatório:
[`docs/relatorios/modular-character-engine/03_EQUIPMENT_STUDIO.md`](../../relatorios/modular-character-engine/03_EQUIPMENT_STUDIO.md).

## 1. Ideia

O `ChestModel` da 02 tinha uma silhueta, uma rampa, gola, lamelas e quilha, todos fixos no
código. A 03 troca isso por um **modelo genérico**: um equipamento é uma lista de partes
independentes, interpretada por um único pintor. Um item novo, com geometria, materiais e
estrutura próprios, é um arquivo RON criado pelo editor ou à mão, **sem mexer em Rust**.

## 2. Modelo (`player::visual::rig::design`, puro)

```text
EquipmentDesign { id, name, slot, space, depth, mount (transform do encaixe), light,
                  aspect_range, materials[], parts[], provisional, source }
  space: Torso (gaiola u, v) | Hand { side } (grip) | Bone { bone, at, follow }
  PaintMaterial { id, ramp[6] (tinta, profundo, sombra, base, luz, brilho), finish }
  finish: Metal | Leather | Cloth | Flat
  Part { id, source, transform, material, layer, outline, bevel, regions[], ornaments[],
         visible, visibility }
    source: Polygon { points, smooth (passes de Chaikin = curva) } | Image { path, pivot }
    transform: posição, rotação, escala, pivô
    outline: None | Selective | Ink
    Region { points, material?, tone }
    Ornament: Rivet { at } | Line { points, tone, dashed }
    Visibility { clips[], turn: faixa de perspectiva (0 lateral, 1 três quartos) }
```

| Pedido do briefing | Como está no modelo |
|---|---|
| Identificação | `Part::id` (único na peça) |
| Geometria própria / polígono de silhueta | `PartSource::Polygon`; curvas por `smooth` |
| Posição, rotação, escala, pivô | `PartTransform` (o pivô fica parado sob rotação e escala) |
| Material e paleta | `material` → `PaintMaterial` (rampa de 6 tons e acabamento) |
| Profundidade | `layer` (dentro da peça) e `depth` (`DepthSlot` da peça no corpo) |
| Contorno | `Outline::{None, Selective, Ink}` |
| Regiões internas | `Region` (outro material ou deslocamento de tom) |
| Ornamentos | `Ornament::{Rivet, Line}` (linha tracejada = costura) |
| Vínculos com ossos ou gaiolas | `DesignSpace::{Torso, Hand, Bone}` e `mount` |
| Regras de visibilidade | `Visibility::clips` |
| Variantes de perspectiva | `Visibility::turn` (parte só na vista lateral ou só na de três quartos) |
| Fonte PNG | `PartSource::Image`: os pixels do PNG, sem esticar (decalque no tronco; girado por mapeamento inverso no espaço da mão) |

Viabilidade das outras fontes:
- **SVG:** viável como fonte offline, rasterizado pelo `svg_render.py --crisp` do ASF e
  importado como PNG (o emblema do Modelo A). O editor não lê SVG diretamente, para não
  acrescentar dependência.
- **Malhas:** a gaiola do tronco já é uma malha de 4 vértices presos às juntas. Malhas com mais
  vértices são o caminho da capa (seção 6).

**Compatibilidade:** `design::from_chest_model` é o adaptador explícito do `ChestModel` da 02
(teste `the_02_breastplate_paints_through_the_adapter`). O laboratório da 02 continua com o
formato antigo, sem mudança de comportamento.

## 3. Pintor (`player::visual::rig::paint`, puro)

`paint(job)` desenha qualquer design numa pose, direto na grade de texels da célula:
1. cada texel vira coordenada do espaço (gaiola ou referencial) e depois coordenada local de
   cada parte;
2. a parte de cima vence;
3. a normal vem do cilindro do corpo (no tronco) somada ao bisel (distância à borda e direção da
   borda) e é iluminada por uma luz, quantizada na rampa;
4. couro faz pontilhado só perto das trocas de tom, metal faz brilho especular;
5. regiões trocam material ou tom e os ornamentos entram por cima;
6. tinta na silhueta, e emenda (dois tons abaixo) onde a parte cobre outra de camada menor.

A regra de normal e luz e a regra de tinta × emenda vêm do pintor do pixelfx (`rig.js`, MIT).
`pose_map` põe um design num quadro do clipe: espaço, profundidade (perfil de empunhadura para a
mão) e giro.

Saída: alfa binário, só cores das rampas e dos PNGs, sem reamostragem (testado).

## 4. Editor (`examples/equipment_studio.rs`)

Ferramenta separada do jogo. Reaproveita:
- o esqueleto e os clipes de `art-src/modular-character-engine/stage/rig_lab.ron`;
- as camadas do corpo por `RigClip::cut_layers`, agora compartilhado com o laboratório (o
  código duplicado saiu de lá);
- os perfis de empunhadura;
- `core::pixel_image`.

A tela tem três áreas:
- painel de texto à esquerda: designs, partes, propriedades, modo e status;
- vista principal com zoom inteiro (2× a 8×);
- prévias em 2× e 1× (resolução nativa) à direita.

| Tecla | Efeito |
|---|---|
| PageUp / PageDown | escolhe o design |
| Enter | equipa ou desequipa o design (um por slot) |
| Ctrl+N / Ctrl+S / Ctrl+R | novo design / salvar / recarregar tudo do disco |
| Ctrl+P | próximo slot do design |
| Tab / Shift+Tab | próxima / parte anterior |
| N / I / Delete / Ctrl+D | nova parte poligonal / importar o próximo PNG de `import/` / remover / duplicar |
| H / [ ] | esconder ou mostrar a parte / mandar para trás ou para a frente |
| M | modo: parte → vértice → pivô → encaixe → cor |
| Setas (Shift = 4 texels) | move a parte, o vértice, o pivô ou o encaixe; em cor: ← → escolhe o tom, ↑ ↓ clareia ou escurece |
| Q / E, - / = | gira e escala (parte ou encaixe) |
| V / B, Y, Backspace | vértice anterior / próximo, inserir vértice, apagar vértice |
| K, R G B (Shift = −) | modo cor: troca o material da parte, edita o tom escolhido |
| Espaço, . / , | tocar ou pausar, quadro seguinte / anterior |
| T, F | idle / corrida, esquerda / direita |
| Z / X ou roda do mouse | zoom inteiro, sem suavização |
| O, U, G | item isolado, personagem sem equipamento, esqueleto e gaiola do tronco |
| Mouse (arrastar) | modo vértice: arrasta o vértice mais próximo; modo parte: arrasta a parte; modo pivô: posiciona o pivô |

O mouse está implementado, mas o roteiro de evidências usa só o teclado, pelo `ButtonInput`
real.

## 5. Provas

- **Modelos A e B:** mesmo pintor, 9 e 8 partes, estruturas diferentes. Sobreposição das
  silhuetas 0,79 e das estruturas internas de 0,36 a 0,47 (teste
  `the_breastplates_differ_in_structure_not_only_colour`).
- **Espada:** PNG da N41-B importado e girado −90° no espaço da mão, com um rubi poligonal no
  pomo. Os pixels do PNG ficam intactos (teste).
- **Terceiro equipamento (`studio_item_1.ron`):** feito pelo roteiro só com teclas:
  1. Ctrl+N;
  2. slot `Belt`;
  3. vértices movidos até formar uma faixa na cintura;
  4. cores editadas;
  5. fivela de latão importada e posicionada;
  6. Ctrl+S.

  O commit dele contém só o RON. O segundo processo (`--tour-reload`) reabre o item e o encontra
  equipado.

## 6. Capa articulada: deixada para a Engine 04

O briefing permitia adiar. A arquitetura prevista:
- uma `DesignSpace::Chain` presa às costas: o ponto de fixação vem do osso da espinha, e a
  cadeia tem N segmentos com comprimento e limite de ângulo;
- cada parte da capa amarrada a um segmento, como os segmentos articulados da 02;
- a simulação da cadeia fica fora do pintor: uma integração verlet ou de molas por quadro,
  determinística, alimentada pelo deslocamento do quadril entre quadros. O pintor só recebe os
  referenciais.
- a profundidade `BackGear` já existe, mas falta uma gaiola das costas.

Não foi implementada, para não tirar qualidade do editor e dos peitorais.

## 7. Origem de cada parte

| Parte | Origem |
|---|---|
| Normal (cilindro + bisel) × luz quantizada na rampa; tinta na silhueta e emenda sobre partes de baixo | Pintor do pixelfx (`src/core/rig.js`, MIT, © 2026 Patch, Hugo Duprez), reescrito em Rust |
| Fivela (PixelSpec → PNG), emblema (SVG crisp → PNG), QA dos rasters pintados | Agent Sprite Forge `render_pixelspec.py`, `svg_render.py`, `pixel_qa.py` (MIT, © 2026 0x0funky), uso offline |
| Esqueleto, gaiola do tronco, empunhadura, camadas do corpo, laboratório e regras de profundidade | Modular Character Engine 01/02 (Dark Castle) |
| Espada | Fábrica N41-B (PNG sem mudança) |
| Modelo genérico, pintor, editor, Modelos A e B, contrato RON e roteiros | Desenvolvidos nesta etapa |

## Atualização (04)

A etapa 04 acrescentou hierarquia de partes, regiões do corpo, composição única, desfazer e o editor com painéis, e mudou os roteiros para uma cópia isolada. Ver [`04_STUDIO_V2.md`](04_STUDIO_V2.md).
