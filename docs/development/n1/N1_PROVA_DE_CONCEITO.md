# N1 — Prova de conceito

A prova não foi executada. Este arquivo é o roteiro. Ela só começa com autorização, numa cópia de trabalho, sem tocar em `white_mesh.glb`, em `assets/` publicado nem no save real.

## O que ela contém

- Um corpo, vindo de uma cópia da malha ou de um manequim já riggado do pipeline V2.2. O GLB original não tem osso.
- Quatro clipes: idle, walk, jump, attack. Walk pode ser um recorte do run se o tempo apertar, mas o quadro de walk tem de existir para a pergunta 3 ter resposta.
- Duas armas de famílias diferentes (espada e lança, ou espada e machado).
- Duas armaduras que compartilham o mesmo pivô do corpo.
- Uma capa na âncora das costas.
- Troca das quatro peças em runtime, com as camadas que o jogo já tem.
- O mesmo quadro para corpo, arma e capa.

O app da prova é isolado, no estilo dos exemplos de visual que já existem. Não substitui o `cargo run`.

## As dez perguntas

Cada uma vira uma linha no relatório da prova, com sim, não ou parcial, e o quadro que justifica.

1. O quadro parece pixel art, ou parece render reduzido?
2. A silhueta se lê em fundo escuro, do tamanho em que a opção A desenha (80–110 texels)?
3. Walk e jump têm peso, ou deslizam?
4. A mão e a arma compartilham a âncora em todos os quadros do ataque?
5. A armadura não sai do pivô do corpo?
6. A capa fica atrás e a arma na frente, e o ataque que cruza o torso está anotado?
7. A quinta arma seria um registro e uma pintura, ou uma folha nova do corpo?
8. Quantas horas levou o primeiro clipe, e quantas o segundo?
9. O que foi script e o que foi mão no Pixelorama?
10. O mesmo PNG abre no Linux desta máquina. Windows fica como verificação do proprietário, se ele tiver a máquina. Não se afirma compatibilidade sem rodar.

## Fora da prova

Castelo, relíquia, controle novo, chefe, centenas de itens, Hunyuan, Mixamo, compra de Spine ou Aseprite.

## Decisão que a prova pode tomar

Seguir o híbrido, ou mandar o corpo do ataque para quadro a quadro porque o render não segurou o pixel. As duas respostas são sucesso da prova. Sucesso não é "ficou bonito". É saber qual caminho produz a próxima arma.

## Alternativas

Pular a prova e produzir o elenco. É o custo que a N1 existe para evitar.

## Riscos

Julgar a malha crua de 94 mil vértices como se fosse o personagem. Sem rig e sem pixel, ela não responde nenhuma das dez perguntas.

## Recomendação

Autorizar a prova como N3, depois da N2 ou em paralelo se outra pessoa fizer a arte. Quem for fazer instala o Pixelorama na hora, não antes.
