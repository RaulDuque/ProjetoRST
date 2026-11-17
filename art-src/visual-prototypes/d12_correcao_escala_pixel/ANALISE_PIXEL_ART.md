# Análise pixel art / alfa

## Diagnóstico

- Frames D10/D12: alfa efetivamente binário (`semi_alpha=0` nos poses auditados).
- Sem RGB em a=0.
- Sola estável em y=156 (±0 nos frames processados).

## Conclusão

Não tratar “pixels sujos” do vídeo/screenshot como defeito do PNG.  
Correções D12 focam contraste, espada, escala/anchor e rim controlado — não threshold cego de alfa.
