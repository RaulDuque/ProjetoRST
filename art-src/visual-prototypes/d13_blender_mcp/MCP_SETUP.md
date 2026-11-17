# Blender MCP — instalação e conexão (D13)

## Versões

- OS: Linux (Ubuntu)
- Blender: **5.2.4 LTS** (`/snap/bin/blender`)
- `uv`/`uvx`: 0.13.8
- Pacote: `mcp-for-blender` (addon protocol 16)
- Cursor: `~/.cursor/mcp.json` → server `blender` (`uvx mcp-for-blender`)
- Addon: `~/.config/blender/5.2/scripts/addons/blender_mcp.py`

## Comandos usados

```bash
uvx mcp-for-blender setup --yes
# server socket:
blender <lab.blend> --python scripts/start_mcp_server.py
```

## Ferramentas Cursor (`user-blender`)

`get_addon_status`, `get_scene_info`, `execute_blender_code`, `look`, `open_viewport`, `viewport_*`, `search_assets`, `import_asset`, `generate_3d`, `generate_gaussian_world`, `disable_telemetry`, …

Telemetria: **OFF** nesta sessão.

## Smoke test

Cubo temporário criado → material → removido. Arquivo original não alterado.
