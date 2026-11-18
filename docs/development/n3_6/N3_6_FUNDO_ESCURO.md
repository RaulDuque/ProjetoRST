# N3.6 — Fundo escuro

Evidências: `run_dark_background.png` e `run6_12fps_dark.gif`.

| Fundo | Leitura |
|---|---|
| Claro (226, 224, 228) | tudo lê; a calça e a bota são a massa mais escura |
| Cinza (150, 150, 156) | tudo lê |
| Quase preto (18, 14, 22) | cabelo, pele e camisa leem; a **calça e a bota perdem a borda** contra o fundo, sobretudo nas pernas de trás |
| Cena real do jogo | `target/world_visual_showcase/after_square.png` (Vila das Cinzas, câmera 405), com o sprite reduzido a 0,889 como na tela: as pernas leem contra a parede marrom, com pouco contraste |

## Teste de borda de luz (só evidência)

Em cada pixel escuro (média menor que 80) que tem transparência à esquerda, ou seja, às costas, a cor sobe (+38, +34, +44): 1 texel, tom frio, sem contorno branco. No quase preto, isso devolve a borda de trás das pernas e das botas sem mudar a frente. O efeito é discreto e aparece melhor ampliado.

**Não foi aplicado** aos clipes nem ao candidato. A decisão é do proprietário, como na N3.5.
