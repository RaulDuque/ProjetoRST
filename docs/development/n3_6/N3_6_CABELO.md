# N3.6 — Cabelo

Evidências: `run_hair_comparison.png` (cabeça e costas dos 6 quadros do `run6`, em fundo escuro) e os GIFs `run6_*.gif`.

## Vídeo (fonte do clipe)

- **Comprimento:** o mesmo em todos os quadros. A ponta das mechas chega à altura das escápulas e não cresce nem encolhe entre os quadros.
- **Direção:** sempre para trás, acompanhando o deslocamento.
- **Movimento secundário:** a massa ondula: nos quadros de impulsão (50, 56) as mechas sobem e se abrem, e nos de contato (53, 58) assentam. É a inércia vista no vídeo, não um efeito adicionado.
- **Continuidade:** como os quadros são consecutivos no vídeo, não há troca de forma entre eles. Em 106 px as mechas viram grupos de 2 a 4 texels nas cores `W`, `w`, `h` e `H`.
- **Sobreposição:** o cabelo passa por trás do ombro e não cobre o braço próximo em nenhum quadro.

## Folhas Gemini (estudo)

Nas folhas, o cabelo muda de comprimento e de volume de uma pose para outra. Em `run_a`, por exemplo, #3 tem as mechas bem mais longas e horizontais que #2. Usadas em sequência, essas poses fariam o cabelo "pular". É um dos motivos para não montar o ciclo a partir delas.

## Limite

O vídeo foi gerado. Os fios finos das pontas viram 1 texel e às vezes somem na redução. A redução não acrescenta fios, e nenhum foi desenhado.
