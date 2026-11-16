# Comparação de pipelines

| Critério | 1. Pixel manual | 2. Blender + limpeza | 3. Híbrido (Blender guia + pixel final) | D3 atual (camadas) |
|---|---|---|---|---|
| Consistência anatômica | Alta (se houver artista) | Alta na pose 3D | Alta | Alta entre frames; estilo still blocky |
| Escala real (~92 px) | Excelente se desenhado na escala | Rende demais → precisa limpar | Bom | Legível; pouco “luxo” B |
| Equipamentos/sockets | Overlays manuais | Medição real (já existe V2.2) | Melhor dos dois | Overlays POC só |
| Volume de animações futuras | Caro | Escala com rig | Escala com rig + polish | Bom para pilotos |
| Manutenção | Cada clip | Reexport + polish | Idem | Script único |
| Risco D2 | Baixo se humano | Médio (se só render) | Baixo | Baixo p/ identidade |

## Recomendação técnica

1. **Curto prazo (aprovação):** usar D3-B1 ou B2 como *modelo de proporção*
   estável para o proprietário escolher o nível de detalhe.
2. **Produção:** **Método 3 — híbrido.** O rig Blender v007 já mede sockets;
   cada pose renderizada deve ser **finalizada em pixel** na escala do
   contrato (não publish de EEVEE cru).
3. **Não** publicar o script D3 como arte final: ele prova consistência, não
   o acabamento B aprovado nos conceitos D1.

Se não houver artista pixel para o polish, o bloqueio é de equipe — não de
ferramenta Blender.
