# Auditoria do pipeline (D2 → D7)

## Estado observado

- **D2:** identidade aprovada; sprites laterais idle/run/attack em
  `d2_protagonista_B/`. Acabamento denso, capa vinho, metal com highlights.
- **D4:** camadas 2D modulares (`camadas/*.png`) compostas por script; prova de
  slots visuais, ainda dependente de arte-fonte.
- **D5:** corrida modular via Blender (hide_render) + compose; acabamento toon
  permanece “personagem antigo”.
- **D6:** placas ChildOf + fidelidade tentada; ainda EEVEE/toon, não D2.

## Causa principal da aparência antiga

O look final era determinado por **materiais/luzes/EEVEE** do modelo Blender
legado, não por desenho 2D D2. Filtros toon/outline e downscale não recriam o
pixel denso da referência.

## Mudança D7

```
D2 / D4 (arte 2D) → máscaras / primitivas / híbrido → camadas → composição
Blender → só anatomia/pose/sockets (opcional), nunca o finish final
```

## Contrato de escala (confirmado)

- Hurtbox ~26×46; silhueta visual alvo ~52 px de altura no lab.
- Célula de laboratório 128×160; pivô sola `(64, 156)`.
- Idle D2: 6 frames; run D2: 8 frames (não 10) — usar contagem real.
