# assets-user — biblioteca de referências e insumos 3D

Pasta **fora** dos assets publicados do Bevy (`assets/`).

Uso: referências ortográficas, contratos de equipamento, GLBs experimentais,
metadados e insumos para Hunyuan3D / Blender. Nada aqui é carregado pelo jogo
automaticamente.

## Árvore

```text
assets-user/
├── protagonista/     # vistas e base do herói
├── equip/            # peças independentes por categoria
└── prota/            # formas alternativas (só placeholders nesta fase)
```

Documentação do pipeline modular D15:
`art-src/visual-prototypes/d15_equip_modular/docs/`.

## Regras

1. Não sobrescrever arte publicada em `assets/`.
2. Não modificar `art-src/player/blender/swordsman_rig_v007.blend` in-place.
3. Identidade canônica: conceito B + acabamento alinhado a D7-B / D12-A (sprites).
4. Peças Hunyuan são **experimentais** até validação de escala, encaixe e animação.
