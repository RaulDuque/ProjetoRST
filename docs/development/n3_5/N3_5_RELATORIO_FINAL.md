# N3.5 — Relatório final

A `reducao` que o proprietário escolheu tem a mesma resolução da N3.4 (106 px). O que reprovou a N3.4 foi a técnica, não a resolução. O candidato N3.5 é a mesma redução da ficha, sem o halo e com 10 pixels corrigidos à mão. Na câmera 360 ele aparece pixel a pixel na tela. Na câmera atual (405), parte das linhas se perde. O estilo aguarda o proprietário. Sem animações. A N4 e a N5 continuam bloqueadas.

## A. Estado inicial

`art/v1.2-protagonist` em `c1eb46c7e31cdb58926ab6dcdd8f04c13109f868`, igual ao remoto. Árvore limpa, exceto arquivos não rastreados em `Md-usario/`, que continuam intocados.

## B. Referências

`image.png`, a vista lateral e o perfil da `Ficha_Modular` (recortes com coordenadas fixas) e a `reducao`, que é `art-src/n3-4-original-protagonist/source/base_idle_draft.txt`: o arquivo original, não uma reconstrução. As folhas citadas pelo proprietário são `face_study.png` e `body_comparison.png`, da N3.4. `N3_5_REFERENCIAS.md`.

## C. Medições

Ficha lateral: 329 px, cerca de 7,8 cabeças, cabelo em 33 % da altura, perna em 60 %, bota em 21 %. Em 106 px: cabeça de cerca de 13,5 linhas e largura de 23. A `reducao` tem exatamente 106×25. `N3_5_MEDICOES.md`.

## D. Hipóteses

A 80×128 (106), B 96×160 (132), C 72×112 (94, derivada da câmera 405). Todas com o mesmo método de redução.

## E. Resolução

106 reproduz a `reducao`. 132 desenha melhor, mas na tela ou é reduzida (0,71 ou 0,80 px por texel) ou exigiria um personagem maior. 94 dá 1:1 na câmera 405, perde o rosto e precisa de densidade não inteira. `N3_5_RESOLUCOES.md`.

## F. Câmera

405: 0,889 px por texel; na captura real, 106×23 vira 94×20. 360: 1,000 px por texel; a captura real é idêntica ao sprite (0 diferenças em 1 320 texels). `N3_5_CAMERA.md`.

## G. Rosto

Perfil sério, com sobrancelha, olho junto à sobrancelha, nariz corrigido, boca, queixo e maxilar, como na `reducao`. Afina na 405. Em 1× é só o perfil. `N3_5_ROSTO.md`.

## H. Cabelo

Coroa com grupos de mechas, fluxo para trás, pontas irregulares mantidas e as mesmas 4 cores frias. `N3_5_CABELO.md`.

## I. Roupa

Torso com volume, manga dobrada, bracelete, calça com dobras diagonais, cano e aba da bota com tiras. Não sobrevivem a fivela do cinto e o cordão. Perde a borda em fundo quase preto. `N3_5_ROUPAS.md`.

## J. Anatomia

As guias da ficha coincidem com as do candidato (`reference_measurements.png`).

## K. Arte

Redução automática da ilustração do proprietário mais 10 edições do agente (7 remoções e 3 recolorações), listadas em `metrics.json`. Sem programa gráfico, sem IA de imagem, sem hora de artista humano. `N3_5_PROCESSO_ARTISTICO.md`.

## L. Bevy

Fonte `n3.5-fidelity` e enquadramento `game-360-1x` no `n3_visual_proof`. O HUD mostra arte, figura, mundo, densidade, câmera, px por texel e "NOT APPROVED". As capturas antigas continuam idênticas e o jogo principal não mudou. `N3_5_INTEGRACAO_BEVY.md`.

## M. Evidências

Em `docs/development/n3_5/evidence/`: `reference_sources`, `reference_measurements`, `resolution_80x128`, `resolution_96x160`, `resolution_reference`, `resolution_matrix`, `camera_405`, `camera_360`, `face_comparison`, `hair_comparison`, `clothing_comparison`, `palette_test`, `n34_vs_n35`, `native_scale`, `dark_background`, `final_candidate`, `bevy_n35`, `bevy_n35_405`, `bevy_n35_360`, `bevy_n34_405` e `bevy_n34_360` (todas `.png`). Não há imagem vazia.

## N. Testes

fmt, check e clippy (`-D warnings`) limpos. `cargo test -j 2`: 49 suítes, 611 passaram, 0 falharam, incluindo `tests/n3_5_fidelity_tests.rs` (novo, 4 testes). `N3_5_TESTES.md`.

## O. Segurança

Os SHA-256 dos originais são iguais aos da N3.4. Nada de `Md-usario/` foi alterado ou commitado. `camera.ron`, `src/`, o save, os controles, o protagonista publicado, a N3 à N3.4 e `white_mesh.glb` estão intactos.

## P. GitHub

Preenchido depois do push.

## Q. Limitações

- Sem pintura humana. A qualidade é a da redução.
- Calça e bota escuras perdem a borda em fundo quase preto.
- A fivela do cinto e o cordão do peito não aparecem em 106 px.
- Na câmera atual (405), o candidato é reduzido na tela.
- A densidade 1,78 (cenário 94 px) não cabe no manifesto atual, porque `density` é inteiro.

## R. Recomendação

**Arte 80×128 (figura 106) feita por redução fiel da ficha, apresentada com a câmera 360 e densidade 2: 1 texel = 1 pixel de tela.** Se a câmera tiver de ficar em 405, aceitar a perda ou pagar uma mudança de densidade no código. Não adotar 96×160 sem antes decidir aumentar o personagem na tela.

## S. Gate A

Pendente. Nenhuma aprovação foi atribuída por métrica.

## T. Próxima decisão do proprietário

1. O candidato N3.5 parece o protagonista e tem o acabamento da `reducao`? Veja `n34_vs_n35.png`, `face_comparison.png` e `camera_360.png`.
2. A câmera do jogo vai para 360? Isso muda o enquadramento de todas as salas e precisa de autorização própria.
3. Cordão e fivela: aceitar a ausência ou permitir um pixel de acento à mão?
4. Fundo escuro: aceitar, ou autorizar um teste de luz de recorte nas pernas?
