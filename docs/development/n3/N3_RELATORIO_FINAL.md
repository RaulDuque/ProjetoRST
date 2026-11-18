# N3 — Relatório final

Aprovação técnica: o protótipo abre, os arquivos passam no parser do jogo e os testes passaram. Aprovação artística: não. O proprietário ainda não viu estes quadros como estilo do jogo, e este relatório não fala por ele.

## A. Estado inicial

Branch `art/v1.2-protagonist`, HEAD `707a593` (controles da N2). A árvore estava limpa, fora os briefings `DarkCastleR-N-3.md`, `DarkCastleR-N-4.md` e `DarkCastleR-N-5.md`. N4 e N5 continuam de fora: não estão autorizados.

## B. Auditoria

O jogo já compõe corpo, arma e capa por camadas, com `weapon_in_body: false` no protagonista publicado. O render atual do manequim não é pixel art. Detalhe em `N3_AUDITORIA_PIPELINE.md`.

## C. Modelo

A figura da prova não usa a malha branca. Ela não tem osso. O único render Blender desta etapa é um idle do manequim V2.2, guardado à parte do `assets/`.

## D. Rig

Não há rig novo. As poses são ângulos no gerador. O Bevy não ganhou esqueleto 2D.

## E. Animações

Idle (4), walk (6), jump (6) e attack (6). O walk tem troca de pernas. O jump tem agachar, subir e cair. Nenhum dos dois está no nível de uma animação final.

## F. Armas

Espada e lança, sprites separados, grip no cabo. A espada aponta para +x e gira com o ângulo do socket.

## G. Armaduras

Leve (tabardo) e pesada (placa, ombreiras, elmo). Mesmo pivô, folhas diferentes, não é recolor.

## H. Capa

Uma folha sincronizada com o quadro do corpo. A gola segue o socket das costas. No ataque ela fica atrás. No jump ela cruza o corpo.

## I. Composição

`proof.ron` e `sockets.ron` usam o contrato que o jogo já valida. Trocar peça não reinicia o clipe. Não há uma folha por combinação.

## J. Pixel art

Paleta fechada e contorno de um pixel, desenhados por script. Não é o render reduzido, e também não é acabamento de produção. O filtro em `pipeline_compare.png` está rotulado e não é pixel art.

## K. Resolução

Janela da prova 1280×720, nearest. Vista 360 com sprite em tamanho 1: escala inteira 2×, figura perto de 193 px. Vista 405 com sprite pela metade: perto de 86 px, escala não inteira. `camera.ron` intacto.

## L. Protótipo

```text
python3 art-src/n3-proof/scripts/build_n3_proof.py
cargo run --example n3_visual_proof
N3_INTERACTIVE=1 cargo run --example n3_visual_proof
```

Teclas só dentro dessa janela: 1–4 clipes, Z arma, X armadura, C capa, M espelho, V vista, Espaço pausa, ponto avança um quadro, Esc sai. Não mexem no perfil de controles do jogo.

## M. Evidências

`docs/development/n3/evidence/`: idle, seis quadros de walk, jump, ataque, preparo, armadura pesada, lança, capa desligada, espelho, as duas vistas, a faixa composta, as quatro combinações, o par filtro/pixels e o idle do Blender.

## N. Métricas

Gerador 1,25 s. Blender 11 s para um quadro. Texturas descomprimidas cerca de 3,1 MiB. Sem hora de artista. Ver `N3_METRICAS.md`.

## O. Testes

`cargo test --jobs 2` passou, com os 4 testes novos. `cargo clippy --all-targets --jobs 2 -- -D warnings` passou depois do exemplo. O exemplo gráfico rodou e gravou os PNG.

## P. Dez perguntas

1. Parece pixel art ou render reduzido? **Parcial.** É pixel de script, não o render do Blender e não um filtro. Também não é arte final.
2. A silhueta lê em fundo escuro? **Parcial.** No close, o golpe e as duas armaduras se distinguem. No enquadramento do jogo (86 px) sobra a massa, não o rosto.
3. Walk e jump têm peso? **Parcial.** As pernas trocam e o pulo sobe. Os dois ainda são poses duras, e o pulo emaranha a capa.
4. A arma acompanha a mão? **Sim.** O socket do golpe sai de x −12,9 para x 47,1, e a captura mostra a lâmina na mão.
5. As armaduras ficam no mesmo pivô? **Sim.** O mesmo pivô `(48, 120)` e o mesmo grid. A troca não move o chão.
6. Capa e arma respeitam a ordem? **Parcial.** No ataque, capa atrás e arma na frente ou atrás conforme o preparo. No jump a capa cruza o corpo.
7. Uma quinta arma exigiria o quê? **Parcial.** Outro sprite e outro grip, não outra folha de corpo. Nenhuma quinta arma foi produzida.
8. Quanto tempo levou cada clipe? **Não verificado** como hora humana. O script fez os quatro em 1,25 s.
9. O que foi script e o que foi mão? **Parcial.** Tudo que está no PNG da prova é script. Correção em Pixelorama: nenhuma.
10. O PNG abriu no Linux? E no Windows? **Parcial.** Abriu no Linux desta máquina, na janela e no Pillow. Windows não foi testado.

## Q. Limitações

Pixelorama não instalado. Sem traço manual do render. Sem vídeo. A figura é provisória de propósito de leitura, e mesmo assim fica abaixo do critério de arte final da seção 19 do briefing: silhueta geométrica, pouca expressão, pulo sem peso limpo. A N4 não começa.

## R. Arquivos

Novos: `art-src/n3-proof/`, `examples/n3_visual_proof.rs`, `tests/n3_visual_proof_tests.rs`, `docs/development/n3/`. Tocados de propósito: `docs/DECISIONS.md`, `CHANGELOG.md`, `docs/development/n1/N1_ROADMAP.md`, e o briefing `Md-usario/DarkCastleR-N/DarkCastleR-N-3.md` porque é a autorização. Não entram N4 e N5.

## S. Segurança

`white_mesh.glb` permanece com 3.387.148 bytes. `assets/animations/player.ron`, `camera.ron` e o save v3 não foram editados. O render do Blender ficou fora de `assets/`. Nenhuma dependência nova.

## T. GitHub

Publicado em `origin/art/v1.2-protagonist`, em cima de `707a593`. Sem force push. O remoto não tinha andado.

- `cc48f6ed26ae51192ec1ba3c2a990baad23d8bef` — `feat(visual): add an isolated pixel proof for equipment layers`
- `fafe69dfea8b822243bb95939edfc3814191c304` — `docs(n3): record the visual proof without claiming the style`

O remoto ficou em `fafe69d` antes deste parágrafo. O commit que grava estes hashes é o seguinte, na mesma branch.

## U. Recomendação

Não promover esta figura para o jogo. O contrato de camadas (corpo único, arma e capa à parte, socket, escala inteira 2×) vale a pena guardar. O caminho híbrido continua hipótese até alguém acabar, no Pixelorama, alguns quadros do manequim que já existe. Desenho só manual, sem esse teste, também não foi medido em horas.

## V. Aprovação pendente

O proprietário precisa dizer se esta silhueta, mesmo provisória, indica a direção, ou se a próxima prova tem de ser o acabamento do render V2.2. Enquanto isso, N4, N5 e a câmera do jogo ficam paradas.
