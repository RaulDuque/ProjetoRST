Rust + Bevy Engine.
Conceito: RPG de ação 2D, estilo Metroidvania, com um único personagem jogável, progressão de níveis, equipamentos, técnicas de combate e magias executadas por sequências de botões.
Característica	Definição
Linguagem	Rust
Engine	Bevy
Dimensão	2D
Câmera	Lateral (side-scrolling)
Estilo	Metroidvania / RPG de ação
Personagens	Um personagem jogável
Combate	Tempo real
Habilidades	Sequências de botões
Progressão	XP, níveis, atributos e equipamentos
Mundo	Castelo interconectado
Arquitetura	Modular, orientada a dados

Sistema de atributos
Vamos aproveitar a filosofia de atributos de Symphony of the Night, preservando os nomes clássicos, mas criando espaço para evoluir o sistema.
Atributo	Função
STR	Força, influencia o dano físico
CON	Constituição, influencia a resistência
INT	Inteligência, influencia o dano mágico
LCK	Sorte, influencia críticos e obtenção de itens
HP	Pontos de vida
MP	Pontos de magia
Hearts	Recurso especial para habilidades que utilizem corações
EXP	Experiência
Level	Nível do personagem
ATK	Poder de ataque
DEF	Defesa
HIT	Precisão dos ataques
CRITICAL	Chance de acerto crítico
RESISTANCES	Resistências elementais e a efeitos negativos

Também teremos:
- Resistência a fogo, gelo, raio, água, sagrado e sombrio.
- Resistência a veneno, maldição, petrificação e outros estados.
- Ataque físico e mágico.
- Velocidade de movimento e ataque.
- Recuperação de HP e MP.
- Atributos derivados dos equipamentos.
Os atributos clássicos serão a base, enquanto os adicionais serão sistemas próprios do nosso RPG.