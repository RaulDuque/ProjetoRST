# N1 — Visão do jogo

Dark Castle RPG continua um jogo de ação e exploração 2D, em Rust 2024 e Bevy 0.20.1, com um protagonista, combate em tempo real e técnicas por sequência. A referência de sensação é a exploração de um castelo. O conteúdo, os nomes e a arte são originais. O produto final permanece 2D.

## Decisão

A fundação de gameplay já construída fica. A N1 muda a ambição de produção: pixel art detalhada, personagem com equipamento visível, e um castelo com voltas, não uma troca de engine nem um segundo inventário.

Pilares, na ordem em que o código já os sustenta:

1. Exploração com portas, requisitos e atalhos (`assets/world/`).
2. Combate cuja lógica não depende do quadro desenhado (`combat`, `skills`, `magic`).
3. Aparência modular em cima do equipamento que já existe (`inventory`).
4. Progressão persistida (Save v3): atributos, inventário, relíquias, baús, chefes.
5. Controles que o proprietário quer remapeáveis. Hoje são teclas fixas. Isso é a N2, ainda não feita.

## O que já é o jogo

Quatro regiões e 29 salas: Vigil Gates, Forgotten Galleries, Veiled Cloister, Village of Ashes. Há loja, altares, viagem rápida, Forma Espectral, paredes quebráveis, baús e um encontro de chefe, The Ashen Sentinel. Na visão nova ele é um minichefe de entrada, não o chefe principal. O código não foi reclassificado nesta etapa (`N1_CHEFES_E_MINICHEFES.md`).

## Alternativas

Recomeçar o projeto em outra engine, ou tratar o protótipo como descartável. As duas jogam fora sistemas que já têm teste: dano, comandos, save, grafo de salas, camadas de arma e capa.

## Riscos

Documentar uma visão maior do que o pipeline aguenta. A arte publicada ainda é o corpo da V2.2 mais silhuetas geradas em código. A visão só vale se a prova de conceito (`N1_PROVA_DE_CONCEITO.md`) mostrar que a pixel art cabe nesse corpo.

## Recomendação

Aprovar esta visão como norte e não como ordem de produção. A próxima etapa, se aprovada, é a N2 de controles. Nenhuma etapa N2–N12 começa sozinha.
