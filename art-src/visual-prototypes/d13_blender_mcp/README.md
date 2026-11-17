# D13 — Blender MCP + pipeline modular

## Setup MCP

Ver `MCP_SETUP.md`.

## Batch (reproduzível)

```bash
blender --background art-src/visual-prototypes/d13_blender_mcp/blender/d13_lab_swordsman.blend \
  --python art-src/visual-prototypes/d13_blender_mcp/scripts/validate_rig.py \
  --python art-src/visual-prototypes/d13_blender_mcp/scripts/export_layers.py

python3 art-src/visual-prototypes/d13_blender_mcp/scripts/validate_exports.py
python3 art-src/visual-prototypes/d13_blender_mcp/scripts/compare_to_d12.py
```

## Pastas

- `blender/` — cópia lab (não o original)  
- `renders/` — estudos A/B/C + modularidade  
- `exports/` — camadas por slot  
- `comparacoes/` — vs D12  
- `scripts/` — bpy reutilizáveis  
