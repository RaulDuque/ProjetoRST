# Dark Castle Equipment Studio — dados e fontes (Engines 03 a 05)

- `items/*.ron`: um equipamento por arquivo, independente do personagem, no modelo genérico
  `src/player/visual/rig/design.rs`.
  - `gothic_cuirass.ron`: Modelo A, gótico roxo e dourado.
  - `leather_jerkin.ron`: Modelo B, couro escuro com placas, tiras e fivelas.
  - `arming_sword.ron`: espada da prova de importação de PNG.
  - `studio_item_1.ron`: cinto criado **só pelo editor**, salvo pelo próprio Ctrl+S no
    roteiro `--tour`.
  - `gothic_set_cuirass.ron`, `gothic_set_pauldron.ron`, `gothic_set_bracers.ron`,
    `gothic_set_faulds.ron` (Engine 04): o Modelo A refeito como quatro peças
    independentes e hierárquicas (peitoral, ombreira, braçais, proteção da cintura).
  - Engine 05 (os anteriores ficam intactos):
    - `gothic_v2_cuirass.ron`, `gothic_v2_pauldron.ron`, `gothic_v2_bracers.ron`,
      `gothic_v2_faulds.ron`: o conjunto gótico refinado;
    - `leather_v2_jerkin.ron`: o Modelo B com o couro refinado;
    - `gothic_cape.ron` e `gothic_cape_short.ron`: capas flexíveis (`flex`), a segunda feita
      só com dados.
- `import/*.png`: imagens que a tecla `I` do editor importa como parte.
  - Gerados por `scripts/build_imports.py`: a fivela renderizada da PixelSpec
    (`sources/buckle.pixelspec.json`, variantes ferro e latão) pelo `render_pixelspec.py` do
    ASF, e o emblema renderizado do SVG (`sources/emblem.svg`) pelo `svg_render.py --crisp`.
  - A espada da fábrica N41-B, copiada sem mudança.
- `scripts/make_evidence_03.py`: monta `evidence/modular_character_engine/03_equipment_studio/`.

```bash
cargo run --example equipment_studio                        # o editor
cargo run --example equipment_studio -- --tour              # roteiro de evidências na cópia isolada
cargo run --example equipment_studio -- --tour-reload       # reabre a cópia que o --tour salvou
~/.local/share/dc-art-tools/asf-venv/bin/python -B art-src/equipment-studio/scripts/make_evidence_04.py
STUDIO_DUMP=1 cargo test --test equipment_studio_tests      # rasters 1x em target/equipment_studio/preview/
PATH=~/.local/share/dc-art-tools/asf-venv/bin:$PATH python -B art-src/equipment-studio/scripts/build_imports.py
~/.local/share/dc-art-tools/asf-venv/bin/python -B art-src/equipment-studio/scripts/make_evidence_03.py
```

Desde a Engine 04 os roteiros trabalham numa cópia em `target/equipment_studio/workspace/`:
esta pasta só é lida e os roteiros repetem. `--items DIR` abre outra pasta.
`scripts/make_evidence_04.py` monta `evidence/modular_character_engine/04_studio_v2/`.
Engine 05:
- `scripts/preview_sheet.py` monta folhas das composições de `cargo run --example equipment_preview`;
- `scripts/run_tools_05.py` roda as verificações do ASF;
- `scripts/make_evidence_05.py` monta `evidence/modular_character_engine/05_art_and_flexible_equipment/`.

O Python e o Node só rodam na produção de assets e evidências. O jogo, o editor e os testes
não dependem deles. Documentação: `docs/development/modular_character_engine/03_EQUIPMENT_STUDIO.md` e
`04_STUDIO_V2.md` e `05_ART_AND_FLEXIBLE_EQUIPMENT.md`.
