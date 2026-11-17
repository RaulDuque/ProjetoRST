# Comparação pipelines A / B / C

| Critério | A D10 refinada | B Blender EEVEE | C híbrido |
|---|---|---|---|
| Identidade D7-B | Alta | Baixa/média | Média-alta |
| Modularidade | Mantém bake D10 | Frágil (still 3D) | Parcial |
| Espada/rosto | Melhora contrast+metal | Depende luz | Rim de B em A |
| Manutenção | Script Pillow | Blend + setup | Dois pipelines |
| Tempo | Baixo | Médio | Médio |

**Recomendação:** **Pipeline A (contraste B)** como candidato principal.  
B descartado como look final. C opcional se rim Blender ajudar em peças pontuais.
