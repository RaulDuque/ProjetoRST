# Notas de separação do atlas

- Cortes a partir de `idle_f00` D2 registrado na célula 128×160.
- `coat_back` / `cape_front` / `hair_back` podem ficar vazios quando o frame lateral não distingue planos — a composição usa `cape_back` + `coat_front` + `hair_front`.
- `gloves` e `scabbard` são recortes fracos; se vazios, o loadout clássico cai no sprite completo de referência para preservar D7-B.
- Quando o restack perder fidelidade vs `full_classic_ref.png`, o compositor clássico prioriza o frame D2 completo + overlays.
