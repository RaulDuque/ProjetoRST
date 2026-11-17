# Relatório final D13

**Sem commit. Sem push. Visual padrão e Save V3 intactos.**

## A. Estado inicial do Git

Branch `art/v1.2-protagonist`, HEAD `1115f0d`. Labs D9–D12 e experimental já untracked.

## B. Instalação MCP

`uvx mcp-for-blender setup --yes` (Cursor + addon Blender 5.2). Ver `MCP_SETUP.md`.

## C. Versões

Blender 5.2.4 LTS · Bevy 0.20.1 · uv 0.13.8 · addon MCP 2.0 / protocol 16.

## D. Ferramentas MCP

Lista em `evidencias/mcp_conexao.json` (`execute_code`, `get_scene_info`, `look`, …).

## E. Conexão

**Concluído.** `get_addon_status` OK; smoke cube create/material/delete OK; telemetria OFF.  
Batch longo via MCP pode dropar socket → exports headless.

## F. Auditoria

Ver `AUDITORIA_MODELO.md`. 132 meshes já modulares; rig 43 bones; **reutilizar**.

## G. Scripts

`inspect_scene`, `validate_rig`, `configure_camera/lighting/render`, `build_materials`, `export_layers`, `validate_exports`, `compare_to_d12`, refine/build stubs (coat/cape/sword/armor/helmet/shield/body), `start_mcp_server`.

## H. Modelagem

Sem remodelagem completa. Coleções `D13_*` + câmera ortográfica lab. Peças existentes: corpo, casaco, capa, espada, pauldron.

## I–J. Materiais / luz

Estudos A/B/C em `renders/materials_lights_*.png`. Default de estudo: **B**.

## K. Rig

Validado (`validate_rig`). Actions POSE/GAME presentes; idle/run 2D **não** promovidos.

## L. Equipamentos

Hide/show: `mod_no_cape`, `mod_no_coat`, `mod_with_armor`. Helmet/escudo: parcial (ausentes).

## M. Exportação 2D

512×640 RGBA camadas em `exports/`; downscale experimental `blender_to_128x160.png`.

## N. Comparação

`comparacoes/d12_vs_blender.png` + `escala_real.png`. **D12-A vence na escala do jogo.**

## O. Modularidade

Camadas independentes por coleção — **não** loadout bake único.

## P. Evidências

`renders/`, `exports/`, `comparacoes/`, `evidencias/mcp_conexao.json`, lab `.blend`.

## Q. Limitações

Ver `LIMITACOES.md`.

## R. Estado final Git

HEAD `1115f0d`; lab D13 adicionado untracked. Original `.blend` mtime inalterado. **Sem commit.**

## S. D14

1. Usar Blender só para peças novas (elmo/escudo/cuirass) com contrato de slots.  
2. Manter D12-A como base pixel até pintura 2D superar.  
3. Pipeline: MCP para edição interativa + `export_layers.py` para lote.  
4. Só então avaliar Bevy experimental.

## Critérios (resumo)

| Área | Estado |
|---|---|
| MCP instalado/conectado/ferramentas/python/render | **concluído** |
| Modelo corpo+equip+rig | **concluído** (reuso) / elmo-escudo **parcial** |
| Arte D7-B na escala real via Blender | **bloqueado** (não supera D12-A) |
| Bevy novo | **não iniciado** (ponto de parada 5) |
| Segurança / sem commit | **concluído** |
a