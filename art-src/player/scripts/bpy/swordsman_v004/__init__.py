"""Swordsman base model v004 (V1.2-D): localized refinement of v003.

Entry point: ``bpy/build_swordsman_v004.py``. The entry script puts this
package *before* ``swordsman_v003`` on ``sys.path``:

- modules defined here replace the v003 module of the same name for the
  v004 build only: ``params`` (version, paths, new parameters) and
  ``surfaces`` (less plastic materials, ``v4_`` names);
- v003 modules without a v004 counterpart are reused unchanged and read
  the v004 ``params``/``surfaces``: ``mesh_tools``, ``sword``, ``staging``
  and ``anatomy`` (neck and hands) and ``clothing`` (collar);
- new modules: ``shapes`` (profiled tubes), ``face`` (head and hair),
  ``body`` (torso, waistcoat, arm and leg shapes), ``garments`` (coat,
  sleeves, belt, half-cape, pauldron, boots), ``validate`` (checks and
  intersection report).

The v003 build never has this directory on its path, so it is reproduced
exactly as before.
"""
