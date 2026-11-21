# Dark Castle Equipment Studio — dados e fontes (Engine 03)

- `items/*.ron`: um equipamento por arquivo, independente do personagem, no modelo genérico
  `src/player/visual/rig/design.rs`.
  - `gothic_cuirass.ron`: Modelo A, gótico roxo e dourado.
  - `leather_jerkin.ron`: Modelo B, couro escuro com placas, tiras e fivelas.
  - `arming_sword.ron`: espada da prova de importação de PNG.
  - `studio_item_1.ron`: cinto criado **só pelo editor**, salvo pelo próprio Ctrl+S no
    roteiro `--tour`.
- `import/*.png`: imagens que a tecla `I` do editor importa como parte.
  - Gerados por `scripts/build_imports.py`: a fivela renderizada da PixelSpec
    (`sources/buckle.pixelspec.json`, variantes ferro e latão) pelo `render_pixelspec.py` do
    ASF, e o emblema renderizado do SVG (`sources/emblem.svg`) pelo `svg_render.py --crisp`.
  - A espada da fábrica N41-B, copiada sem mudança.
- `scripts/make_evidence_03.py`: monta `evidence/modular_character_engine/03_equipment_studio/`.

```bash
cargo run --example equipment_studio                        # o editor
cargo run --example equipment_studio -- --tour              # roteiro de evidências (teclas reais)
cargo run --example equipment_studio -- --tour-reload       # reabre o que o --tour salvou
STUDIO_DUMP=1 cargo test --test equipment_studio_tests      # rasters 1x em target/equipment_studio/preview/
PATH=~/.local/share/dc-art-tools/asf-venv/bin:$PATH python -B art-src/equipment-studio/scripts/build_imports.py
~/.local/share/dc-art-tools/asf-venv/bin/python -B art-src/equipment-studio/scripts/make_evidence_03.py
```

O `--tour` cria `studio_item_1.ron` se ele ainda não existir. Com o arquivo já presente, ele
cria `studio_item_2`, e as verificações do roteiro (que esperam o `_1`) falham.

O Python e o Node só rodam na produção de assets e evidências. O jogo, o editor e os testes
não dependem deles. Documentação: `docs/development/modular_character_engine/03_EQUIPMENT_STUDIO.md`.
