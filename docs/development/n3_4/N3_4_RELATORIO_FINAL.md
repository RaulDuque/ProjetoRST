# N3.4 — Relatório final

Um quadro lateral de 106 px do protagonista-base, tirado da ficha original e corrigido pixel a pixel. Ele parece o personagem da ficha na silhueta, no cabelo e na roupa. O rosto só lê em close. Não houve pintura em programa gráfico nem ataque. O estilo aguarda o proprietário. A N4 e a N5 continuam bloqueadas.

## A. Estado inicial

`art/v1.2-protagonist` em `8a39ab1a0d4401e97a75fdacb105eea784ef1ed0`, igual ao remoto. Árvore limpa, exceto arquivos não rastreados em `Md-usario/`, que continuam intocados.

## B. Arquivos originais

Abertos e vistos: `image.png`, `Ficha_Modular_do_Protagonista_Gotico.png` e `Ficha_de_Modelagem_do_Protagonista_Gotico.png` (não rastreada, só leitura). `white_mesh.glb` foi inspecionado tecnicamente: malha branca de 94 020 vértices, sem rig. A referência equipada em pixel art não está no repositório. Ver `N3_4_AUDITORIA.md`.

## C. Identidade

Guerreiro esguio de cabelo longo branco-prateado, rosto masculino sério, camisa cinza de manga dobrada com cordão no peito, cinto com fivela, calça marrom-escura, braceletes de couro, mãos nuas e botas de cano alto com aba dobrada. Ver `N3_4_REFERENCIAS_ORIGINAIS.md`.

## D. Fidelidade

As linhas de queixo, ombro, cinto, bracelete, joelho, bota e sola do sprite caem nos mesmos pontos da vista lateral da ficha (`reference_analysis.png`). A silhueta bate com a da ficha reduzida (`silhouette_comparison.png`).

## E. Rosto

Perfil em 7×12 px, com testa, sobrancelha, olho, nariz, boca, queixo e linha do maxilar. Foram quatro iterações vistas em close. Lê em 4× e 8×; em 1×, não carrega identidade. `face_study.png`.

## F. Cabelo

Massa longa em diagonal até o meio das costas, mechas agrupadas, sombras frias, contorno azulado e uma mecha à frente da orelha. Funciona em fundo claro, neutro e escuro. A coroa ainda é lisa demais. `hair_study.png`.

## G. Corpo

Cerca de 7 cabeças. Glúteo, joelho e panturrilha têm 1 px de volume. A calça e o cano da bota ainda têm colunas retas.

## H. Roupa-base

Camisa com duas dobras de manga e cordão, cinto com fivela, bracelete com rebite, dobra da calça sobre o cano, bota com aba, fivelas, bico arredondado e salto.

## I. Resolução

Contrato N3.3 mantido: 80×128, 106 px opacos, sola na linha 118, pivô `(40, 118)`. Na câmera do jogo (405 unidades, densidade 2), o sprite aparece em cerca de 94 px de tela, reduzido. A câmera 360 daria 1 texel por pixel: fica como proposta, não aplicada. `N3_4_RESOLUCAO.md`.

## J. Processo artístico

Redução automática da ficha como decalque, depois correção pixel a pixel pelo agente (Claude), digitada como grade de texto. 1 248 pixels diferem do rascunho. Nenhuma hora de artista humano. Nenhuma pintura no Pixelorama ou em outro programa: o agente não opera interfaces gráficas. Nenhuma IA de imagem. `N3_4_PROCESSO_ARTISTICO.md`.

## K. Modularidade

O corpo é uma camada única. Casaca, armadura, capa, capuz, elmo, espada e escudo ficam de fora. Camisa, calça, botas e braceletes estão na base, como em `image.png`. Se calça e botas tiverem de ser removíveis, como sugere a ficha, as pernas precisam ser redesenhadas. Isso aguarda decisão.

## L. Bevy

Fonte 5 (`n3.4-base`) no `examples/n3_visual_proof.rs`, sem capa e sem arma. Capturas em `bevy_n34.png` e `bevy_n34_game.png`. As capturas antigas foram regravadas idênticas. O jogo principal não mudou.

## M. Evidências

Em `docs/development/n3_4/evidence/`: `original_reference.png`, `reference_analysis.png`, `face_study.png`, `hair_study.png`, `body_base.png`, `body_comparison.png`, `silhouette_comparison.png`, `native_scale.png`, `nearest_preview.png`, `dark_background.png`, `bevy_n34.png` e `bevy_n34_game.png`. Todas foram geradas por script a partir do sprite. Nenhuma foi retocada.

## N. Testes

fmt, check e clippy (`-D warnings`) limpos. `cargo test -j 2`: 48 suítes, 607 passaram, 0 falharam, incluindo `tests/n3_4_original_protagonist_tests.rs` (novo). `N3_4_TESTES.md`.

## O. Segurança

SHA-256 dos originais registrados em `N3_4_AUDITORIA.md`. `git diff HEAD -- Md-usario` vazio, e nada de `Md-usario/` foi commitado. `camera.ron`, `src/`, o save, os controles, o protagonista publicado, o rig Blender e `white_mesh.glb` estão intactos.

## P. GitHub

Preenchido depois do push.

## Q. Limitações

- Não houve pintura humana nem em programa gráfico. O acabamento tem os limites de pixels digitados.
- O rosto não sobrevive em 1× nem na câmera 405.
- A coroa do cabelo está lisa. A calça e a bota têm colunas retas. O antebraço tem pele chapada.
- O apoio dos pés fica centrado na coluna 39, 1 px atrás do pivô.
- A referência equipada em pixel art não estava disponível.

## R. Recomendação

Aprovar **para avaliação**, não como arte final. O proprietário julga o Gate A e decide a câmera e o escopo da base. Uma passada de pintura no Pixelorama, partindo de `base_idle.png`, é o caminho mais curto para o acabamento. Não começar animação antes disso.

## S. Aprovação pendente

O proprietário vê `body_comparison.png`, `face_study.png` e `reference_analysis.png` e responde: o sprite ainda parece o protagonista original? Esta nota não aprova o visual.
