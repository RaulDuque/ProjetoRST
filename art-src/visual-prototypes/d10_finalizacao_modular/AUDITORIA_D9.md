# Auditoria D9 (entrada da D10)

## Antifake — causa real dos “partial”

Reprodução dos testes e inspeção pixel a pixel:

| Peça | Status D9 | Causa |
|------|-----------|-------|
| Capa | partial (114) | **110 pixels idênticos compartilhados com `gloves`**; 4 com `scabbard`. Máscara de capa engoliu região do braço/luva. |
| Luvas | partial (110) | Mesmos 110 pixels também presentes em `cape_back`. Ao remover luvas, a capa mantém o RGBA gêmeo. |
| Casaco/espada/botas/bainha | pass | OK |

Não é chroma residual nem buraco de corpo-base. É **falha de exclusividade de máscara** (pixels compartilhados).

## Outros pontos D9

- Compose clássico já não usa `full_classic_ref` (bom).
- Idle/run clássico ainda usam frames D2 para fidelidade de movimento.
- Clássico modular ≈ D7-B, não pixel-identical.
- Pesado: design ok, shading/encaixe a refinar.

## Ação D10

1. Separar capa ↔ luvas com máscaras mutuamente exclusivas.
2. Reparar casaco sob capa quando necessário.
3. Animação modular por camadas + sockets.
4. After-image lab; plano Bevy sem tocar `src/`.
