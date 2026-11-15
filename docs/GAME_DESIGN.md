# Game design — Dark Castle RPG

> Nome de trabalho. Universo, personagens, inimigos, mapas e arte são originais.

## Conceito

RPG de ação 2D em visão lateral, estilo Metroidvania. Um único personagem
explora um castelo interconectado, evolui por EXP, níveis, atributos e
equipamentos, e luta em tempo real. Técnicas e magias são executadas por
**sequências de botões**, como em jogos de luta: o jogador precisa aprender a
habilidade e dominar o comando.

| Característica | Definição |
|---|---|
| Linguagem / engine | Rust / Bevy 0.20 |
| Câmera | lateral, acompanhamento suave, presa aos limites da sala |
| Combate | tempo real, hitboxes, guarda, esquiva, invulnerabilidade pós-dano |
| Habilidades | sequências de comandos definidas em dados |
| Progressão | EXP, níveis, atributos automáticos + pontos livres, equipamentos |
| Mundo | salas interconectadas (castelo) |

## Pilares

1. **Domínio de comandos** — habilidades fortes exigem execução; o jogo informa o que deu errado (incompleto, lento, sem recurso...).
2. **Progressão legível** — toda fórmula é documentada e visível na tela de status.
3. **Exploração recompensadora** — segredos, baús, portas e habilidades que abrem caminhos.
4. **Conteúdo em dados** — novas habilidades, itens, inimigos e salas sem recompilar.

## Movimentação

Andar, correr (Shift), pular (altura variável, *coyote time*), cair, atravessar
plataformas finas por baixo, agachar, atacar, guardar, esquivar (invulnerável),
receber dano (recuo), morrer, interagir (roadmap).

## Mecânicas planejadas

Armas por categoria com movimentos próprios, armaduras e acessórios, técnicas
(espada, armas pesadas, defensivas, movimento, especiais), magias (elementais,
suporte, invocação), passivas, inimigos com comportamentos diferentes, chefes
multifase, drops, baús, segredos, portas e áreas desbloqueáveis, pontos de save.

## Identidade visual

Gótica, medieval e fantástica. Enquanto não há arte: formas coloridas
(personagem vermelho, inimigos ossos, magias com a cor do elemento) e UI com
painéis escuros e bordas douradas.

## Documentos relacionados

`CHARACTER_ATTRIBUTES.md`, `PROGRESSION_SYSTEM.md`, `COMBAT_SYSTEM.md`,
`INPUT_SYSTEM.md`, `SKILL_SYSTEM.md`, `MAGIC_SYSTEM.md`, `EQUIPMENT_SYSTEM.md`,
`ENEMY_SYSTEM.md`, `SAVE_SYSTEM.md`, `ROADMAP.md`.

Visão de grande escala (V1.6): `GAME_VISION_MASTER.md` e os documentos de
mundo citados nele.
