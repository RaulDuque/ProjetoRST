"""Swordsman base model v003 (V1.2-C): procedural meshes instead of primitives.

Entry point: ``bpy/build_swordsman_v003.py``. Modules, one responsibility each:

- ``params``: version, seed, landmarks and every generation parameter;
- ``mesh_tools``: lofts, tubes along curves and cloth surfaces (pure bpy data);
- ``surfaces``: materials (shaded and flat "toon" variants);
- ``anatomy``: torso, neck, arms, hands and legs;
- ``head``: head, face features and hair locks;
- ``clothing``: coat, collar, sleeves, belt, half-cape and boots;
- ``sword``: blade, guard, grip and pommel;
- ``staging``: world, light rig, cameras and renders;
- ``validate``: checks of the built scene and the JSON build report.

The v002 modules (``geometry``, ``materials``, ...) are untouched, so v002
stays reproducible with ``build_base_model.py``.
"""
