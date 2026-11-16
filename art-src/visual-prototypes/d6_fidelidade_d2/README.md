# D6 — Fidelidade D2 + armaduras modulares reais

Laboratório em `art-src/` apenas. Sem integração no jogo.

## Regenerar

```bash
# Placas de armadura no lab blend (já aplicado na entrega)
blender --background --factory-startup \
  art-src/visual-prototypes/d6_fidelidade_d2/blender/swordsman_rig_v007_d6_lab.blend \
  --python art-src/visual-prototypes/d6_fidelidade_d2/scripts/build_heavy_armor.py

blender --background --factory-startup \
  art-src/visual-prototypes/d6_fidelidade_d2/blender/swordsman_rig_v007_d6_lab.blend \
  --python art-src/visual-prototypes/d6_fidelidade_d2/scripts/export_d6_layers.py --

python3 art-src/visual-prototypes/d6_fidelidade_d2/scripts/compose_d6.py
```

Body/coat/cape/sword da corrida reutilizam as camadas D5 (mesma ação `run`).

## Classificação

- **Técnico:** armadura com peças reais parentadas a ossos; troca individual; 3 loadouts.
- **Artístico:** D6-C aproxima D2 em pose estática; run modular ainda é toon v007.
  **Arte não aprovada automaticamente.**
