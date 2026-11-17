# D15 — Relatório final (equipamentos modulares)

## Critério de sucesso

> Provar que podemos manter um único protagonista e equipá-lo com peças
> independentes, sem recriar o personagem inteiro.

**Resultado: parcialmente provado (piloto base + espada).** A espada canônica
exportada do v007 liga/desliga na mesma cena do herói. Hunyuan gerou uma
malha experimental separada (não oficial). Armadura modular completa **não**
foi produzida (fora do escopo do piloto).

---

## Árvore criada

```text
assets-user/
├── README.md
├── protagonista/{base,frente,costas,lateral,poses}/
├── equip/{arma,cabeça,calça,capa,corpo,sapatos}/
│   └── arma/espada_inicial/{referencia,modelo,blender,renders,metadata.json}
└── prota/{trans-aeria,trans-golem,trans-lobisomem,trans-fantasma}/

art-src/visual-prototypes/d15_equip_modular/
├── docs/PIPELINE_EQUIPAMENTO.md
├── sockets/{SOCKETS_D15.md,v007_bone_sockets.json}
├── scripts/export_sword_and_pilot.py
├── pilot/*_base_plus_sword.blend + pilot_report.json
└── renders/pilot_*.png
```

## Arquivos novos (principais)

- Toda a árvore `assets-user/`
- Lab `d15_equip_modular/`
- `espada_inicial_canonical_v007.glb` (~57 KiB, 592 v)
- `espada_inicial_hunyuan_mini.glb` (~6,9 MiB, ~200k v) — experimental
- Specs: `BASE_SPEC.md`, `PADRAO_EQUIPAMENTO.md`, sockets, pipeline

## Preservados (não modificados)

- `art-src/player/blender/swordsman_rig_v007.blend` (somente leitura)
- `assets/` do Bevy
- `src/` Rust
- `Md-usario/`
- Atlas / visual padrão do jogo
- Instalação `~/AI-3D/` (sem reinstalar)

## Referências encontradas

| Item | Caminho |
|---|---|
| Frente/costas/lado v007 | `assets-user/protagonista/{frente,costas,lateral}/v007_*` |
| Linhagem v002 | idem `v002_*` |
| Detalhe espada | `equip/arma/espada_inicial/referencia/detail_sword_v002.png` |
| Conceito inspiração (não canônico B) | `base/referencia/inspiration_concept_v001.png` |
| Identidade escrita | `docs/ART_DIRECTION.md` |
| Rig / ossos | `art-src/player/RIG_SPECIFICATION.md` |

## Identidade visual utilizada

Conceito B + modelo **v007** (casaca, meia-capa, botas, sabre). Acabamento de
sprites de jogo permanece D7-B / D12-A — **não** substituído.

## Estado do protagonista-base

- **Interino:** corpo v007 completo menos a espada (hide) — alinhado ao V2.2
  weaponless em espírito.
- **Base “roupa neutra” oficial:** **não produzida** — conflito documentado em
  `BASE_SPEC.md` (decisão A vs B pendente).
- Vistas L/R separadas e A-pose dedicada: **pendentes**.

## Estado da espada independente

| Variante | Estado |
|---|---|
| Canônica (export v007) | GLB ok; escala ~0,47×0,04×0,91 m; `pilot_partial` |
| Hunyuan mini | GLB + blend de inspeção; **~200k tris**, bbox ~1,3×0,9×2,0 (fora de escala); forma não oficial |

## Resultado Blender (piloto)

- Cena: `pilot/*_base_plus_sword.blend`
- Renders: equipado / desequipado (frente e lado) + espada isolada
- Empties `socket_*` parentados aos ossos
- Demonstração: mesma base, espada visível vs oculta **sem** recriar o herói

## Problemas de encaixe

1. Hunyuan a partir de `detail_sword_v002` absorveu contexto → escala/volume
   errados; exige retopo + reescala ou ref ortográfica limpa.
2. Animação (run/attack) com peça parentada **não** validada ainda.
3. Casaca/capa/botas ainda são geometria do corpo — não são slots 3D
   independentes.

## Limitações Hunyuan3D-2mini

- Geometry-only; VRAM peak ~4,2 GiB na 4050 6 GB.
- Topologia densa, inadequada a skinning direto.
- Qualidade depende fortemente da silhueta de referência.
- Não usar como identidade do protagonista.

## Próximos equipamentos recomendados

1. Decisão A/B da base limpa.
2. Ref ortográfica limpa da espada → novo Hunyuan **só** se ainda útil;
   preferir canônico v007 para o jogo.
3. Escudo / offhand (`hand.L`) — segundo piloto rígido.
4. Meia-capa como peça `capa/` deformável (ossos CAPE já existem).
5. Só então peitoral/calça/sapatos modulares.

## Segurança

Sem commit, push, sudo, alteração de Rust/`assets/`/rig original/Md-usario.
Gradio residual D14 foi encerrado para liberar VRAM do piloto.
