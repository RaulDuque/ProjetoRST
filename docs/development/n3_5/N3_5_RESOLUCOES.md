# N3.5 — Resoluções

Evidências: `resolution_80x128.png`, `resolution_96x160.png`, `resolution_reference.png` e `resolution_matrix.png`.

As três hipóteses foram feitas com **o mesmo método** (`reduce_fidelity.py`: máscara erodida, BOX e paleta N3.4). Só a altura muda, então a comparação mede a resolução e não a técnica.

| Hipótese | Canvas | Figura | Pivô | Origem do número |
|---|---|---|---|---|
| A | 80×128 | 106 | (40, 118) | contrato N3.3/N3.4 e resolução medida da `reducao` |
| B | 96×160 | 132 | (48, 150) | o canvas pedido no briefing, com a figura proporcional |
| C | 72×112 | 94 | (36, 104) | derivada da câmera: 53 unidades × 720/405 = 94,2 px de tela |

## Hipótese C: o que "resolução derivada da referência" virou

O briefing pedia para medir a resolução efetiva da `reducao`. Medida exatamente, ela é **106 px**, igual à hipótese A, e por isso não justifica uma terceira hipótese. A terceira foi derivada do outro lado: a resolução que, na câmera atual de 405 e com o mesmo tamanho de mundo, dá exatamente 1 texel por pixel de tela. Isso resulta em 94 px.

## O que cada resolução mostra (na própria escala, ampliação inteira)

- **106:** perfil com sobrancelha, olho, nariz, boca e queixo; mechas; manga dobrada; bracelete; dobras da calça; aba e fivelas da bota.
- **132:** o mesmo, com o olho e a boca mais separados e o cordão do peito visível. É o melhor desenho dos três.
- **94:** a pele do rosto cai para cerca de 5 px de largura, o olho e a boca quase se fundem e o cordão some. O perfil, o cabelo e a bota continuam legíveis.

## Conclusão

A diferença entre a N3.4 e a `reducao` **não vem da resolução**: as duas têm 106 px. Ela vem da técnica. A redução preserva o modelado da pintura, e o redesenho à mão da N3.4 trocou esse modelado por colunas de cor chapada. Uma resolução maior (132) melhora o desenho, mas só aparece se o personagem também ficar maior na tela (ver `N3_5_CAMERA.md`).
