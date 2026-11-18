# N1 — Personagem modular

As 17 camadas pedidas, mapeadas no que o jogo já desenha:

| Camada pedida | Hoje |
|---|---|
| Corpo-base | atlas do jogador, `weapon_in_body: false` |
| Pele, cabelo | não são sprites; a cor do corpo vem do atlas |
| Cabeça, torso, braços, pernas, luvas, botas | assados no corpo, ou ausentes |
| Armadura | slot `Body`, sem malha própria; há tint |
| Capa traseira | `CapeLayer`, atrás do corpo, âncora das costas |
| Capa frontal | não existe |
| Arma principal | `HeldLayer::Weapon`, âncora da mão, família em `WeaponForm` |
| Arma secundária, escudo | `HeldLayer::Shield`, mão esquerda ou costas se a arma for de duas mãos |
| Acessório visível | slots `Accessory1` e `Accessory2` só alteram número |
| Efeito mágico | atlas `effects`, separado do corpo |

Todas as camadas que existem leem o mesmo `VisualPlayback`: animação, quadro, espelho e pivô. A capa ainda esconde em `Dead` e `Dodging` e desce no agachamento.

## Decisão

Não fatiar o corpo em dezessete sprites na primeira prova. A prova usa quatro camadas que o runtime já sabe compor, mais uma nova só se couber sem sistema paralelo:

1. Corpo, com a armadura desenhada como variante de atlas, não como combinação.
2. Capa traseira.
3. Arma.
4. Escudo ou mão vazia.

Cabelo, elmo, luva e bota entram depois, como filhos do mesmo quadro, cada um com a própria âncora. Ordem de desenho vira uma lista por clipe (`draw_order`), porque num ataque a arma passa na frente do torso e na recuperação passa atrás. Hoje a ordem é fixa: capa atrás, corpo, arma e escudo na frente.

## Alternativas

Uma folha por combinação de armadura, arma e capa. Dez de cada são mil folhas. O doc de equipamentos recusa isso.

Ou um corpo já partido em ossos. Recusado em `N1_ANIMACAO_ESQUELETICA_2D.md` até a prova pedir.

## Riscos

Variante de armadura que não compartilha o pivô do corpo. A âncora da mão erra e a espada flutua. Por isso armadura nova reusa o mesmo clipe e o mesmo pivô; só a pintura muda, até alguém medir de novo as âncoras.

Capa frontal e capa traseira com tempos diferentes. As duas leem o mesmo quadro.

## Recomendação

Publicar `draw_order` como dado do clipe quando a primeira arma precisar cruzar o torso. Até lá, a ordem fixa da V2.2 permanece.
