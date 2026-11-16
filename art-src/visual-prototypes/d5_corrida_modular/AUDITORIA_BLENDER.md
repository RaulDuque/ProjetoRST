# Auditoria Blender (v007 → lab D5)

## Arquivos

| Item | Caminho |
|---|---|
| Original (intacto) | `art-src/player/blender/swordsman_rig_v007_game_v1_5.blend` |
| Cópia lab D5 | `d5_corrida_modular/blender/swordsman_rig_v007_d5_lab.blend` |
| Inventário | `rig/blender_inventory.json`, `rig/blender_actions.json` |
| Export | `scripts/export_modular_run.py` → `camadas/`, `rig/export_manifest.json` |

Blender no host: **5.2.4 LTS** (`/snap/bin/blender`).

## Respostas obrigatórias

1. **Rig reaproveitável?** Sim. `rig_v007` + clips procedurais `game_sprites_v1_3.clips.run_pose` (10 frames @ 24 fps).
2. **Malha única?** Não. ~132 meshes: `coat`, `half_cape`, `blade`/`sword_*`, `pauldron`, boots, hair, etc.
3. **Peças separáveis?** Coat, capa, sabre, pauldron — já objetos distintos. Armadura completa / elmo / escudo **não** existem como malhas de set.
4. **Equipamento segue ossos?** Skinning Armature já liga coat/cape/sword aos ossos (`coat_*`, `cape_*`, `sword`).
5. **Clipping?** Coat longo vs pernas e capa vs braço — conhecido no v007; D5 registra, não “resolve” arte.
6. **Export separado?** `hide_render` por grupo + mesma câmera `side` + mesma ação run.
7. **Pivôs idênticos?** Mesmo frame Blender e mesma resolução 256²; pós-processo encaixa em célula 128×160 compartilhada.
8. **Silhueta D2?** Blender preserva proporção/pose do swordsman; **look** EEVEE toon ≠ pixel D2 aprovado.
9. **Custo adaptar v007:** Baixo para toggles de visibilidade; alto para novas malhas de armadura/elmo/escudo.
10. **Adaptar v007 vs rig novo?** **Adaptar cópia do v007** (feito). Rig experimental separado seria duplicação desnecessária nesta fase.

## Ações no lab blend

O arquivo `game_v1_5` não traz `GAME_run` pré-gravado; o export **reconstrói** a ação via `clips.build_action(..., "run")` (mesmo autoring V1.3). Hash em `export_manifest.json`.

## Não feito

- Não salvar sobre blends originais.
- Não republicar atlases em `assets/`.
- Não modelar set de armadura pesada completa.
