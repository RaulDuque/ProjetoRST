# Decisão D7

## Marco A

**Candidato selecionado: D7-B** (sprites + máscaras sobre arte D2/D4).

Motivo: única estratégia com aparência **visualmente próxima** do D2 na escala
real. A e C não passam o critério de identidade convincente como finish.

Marco B **foi executado** com base em B (promissor + próximo do D2 via fonte 2D).

## Resposta à pergunta da fase

> Conseguimos criar por código uma aparência próxima do D2, com equipamentos
> independentes, sem o acabamento Blender antigo?

**Parcialmente, com caminho claro:**

1. **Sim** para fidelidade — desde que o pipeline parta de **arte-fonte 2D D2**
   (máscaras, camadas, composição), não de EEVEE.
2. **Não** para “gerar D2 do zero” só com primitivas Pillow.
3. **Híbrido (C)** é a ponte futura (pose Blender → máscara → pintura/carimbos),
   mas o acabamento ainda precisa de mais arte 2D / shading programático.

## D8 recomendada

1. Produzir / limpar **atlas de peças 2D** no estilo D2 (corpo nu articulado,
   capa, casaco, placas, elmo, escudo) sem chroma rosa.
2. Ligar sockets reais a pivôs por peça; deformação leve de capa no run.
3. Não voltar a EEVEE/toon como look final.
