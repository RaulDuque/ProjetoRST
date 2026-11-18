# N3.4 — Personagem-base

Arquivo: `art-src/n3-4-original-protagonist/generated/base_idle.png`, com fonte em `source/base_idle.txt`.

## Pose

Estática, lateral, olhando para a direita, de pé, braço solto com a mão relaxada junto à coxa. É a pose da vista lateral direita da ficha. Não é uma pose de combate nem de idle animado.

## Partes e linhas (canvas 80×128)

| Parte | Linhas | Observação |
|---|---|---|
| Cabeça | 13–27 | ver `N3_4_ROSTO_E_CABELO.md` |
| Pescoço e gola aberta | 28–31 | pele em V com o cordão (`e`) no peito, linhas 32–36 |
| Ombro e braço | 29–50 | camisa cinza com dobras em `g` e `q` |
| Manga dobrada | 45–50 | duas dobras claras (`G`) com sombra (`q`) |
| Cinto | 51–52 | couro `L` e `d`, fivela `M` na frente (coluna 45) |
| Antebraço | 51–58 | pele, com o braço levemente à frente do quadril |
| Bracelete | 59–63 | couro com rebite `M` |
| Mão | 64–69 | dorso e dedos fechados, sem luva |
| Calça | 53–94 | glúteo (57–66) e panturrilha (86–89) com 1 px de volume; joelho 81–83; dobra sobre o cano 89–94 |
| Aba da bota | 95–97 | couro dobrado, com fivela `M` |
| Cano | 98–110 | fivela em tira na linha 106 |
| Pé | 111–118 | bico arredondado, salto (colunas 31–35) separado da sola por 2 px de arco |

## Proporções

`evidence/reference_analysis.png` sobrepõe as linhas de topo, queixo, ombro, cinto, bracelete, joelho, bota e sola do sprite à vista lateral da ficha, ajustada para ter a mesma coroa e a mesma sola. As linhas caem nos mesmos pontos da ficha, com no máximo 1 a 2 px de diferença no joelho.

## Modularidade

- **Separado:** nada além do próprio corpo. O corpo é uma camada única.
- **Fora da base:** casaca, armadura, capa, capuz, elmo, espada e escudo. Nenhum foi desenhado sobre o corpo.
- **Dentro da base:** camisa, cinto, calça, botas e braceletes, como em `image.png`, que é a prioridade 1.
- **Ambiguidade:** a ficha modular também lista "Calça" e "Sapatos (Botas)" como peças. Se elas tiverem de ser removíveis, será preciso redesenhar as pernas por baixo (pernas nuas ou roupa de baixo). Isso não foi feito e precisa de decisão do proprietário.
- Nenhum soquete, encaixe ou camada de equipamento foi criado nesta etapa.
