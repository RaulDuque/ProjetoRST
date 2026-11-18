# N3.6 — Relatório final

A corrida foi feita a partir do vídeo do proprietário, a única referência com continuidade real. São 6 quadros (12 fps) e uma variante de 10 quadros com o tempo do vídeo, reduzidos pelo método da N3.5, sem edição à mão e sem quadro inventado. A prova roda isolada no Bevy. Na câmera 360, cada quadro aparece pixel a pixel. As folhas ilustradas ficaram como estudo de corrida, salto, ataque e idle. Nenhum gate artístico foi aprovado. Combate, câmera e jogo não mudaram. A N4 e a N5 continuam bloqueadas.

## A. Estado inicial

`art/v1.2-protagonist` em `1d628f28caafa19d0ab97d58cf246f77fff8eaeb`, local igual ao remoto. O commit `1d628f2` é do proprietário e só adiciona as sete referências. A árvore estava limpa, exceto arquivos não rastreados em `Md-usario/`.

## B. Referências

Quatro folhas `Gemini_Generated_Image_*.jpg` (2752×1536), `al.jpg` (2400×1792), `side.png` (78×280) e o vídeo (496×864, 24 fps, 189 quadros), além de `image.png`, da ficha modular e de `white_mesh.glb`. `N3_6_REFERENCIAS.md`.

## C. Inspeção

`run_a`: 8 poses de corrida. `run_jump`: corrida, salto, acrobacia, aterrissagem e um respingo. `attack`: duas tomadas de guarda → direto → impacto → recuperação. `idle_run_jump`: idle em 3/4, corrida em 3/4 e salto, com outra roupa. `al.jpg`: 31 golpes olhando à esquerda, com marcadores verdes. Vídeo: parado, depois corrida no lugar com câmera fixa.

## D. Identidade

Mesmo personagem em todas. As cores da camisa e das botas variam (`idle_run_jump` diverge mais). O vídeo reduzido tem as cores do candidato N3.5. `N3_6_IDENTIDADE.md`.

## E. Corrida

As folhas têm 24 poses de corrida, sem ordem nem escala comuns. O vídeo tem um ciclo medido de 12 quadros (0,5 s, dois passos). `N3_6_ANALISE_CORRIDA.md`.

## F. Quadros

São 12 quadros do vídeo (48–59) mais o 0. Ficaram 10 (49 e 55 descartados por fantasma de interpolação). `run6` = 48, 50, 52, 54, 56 e 58. A procedência está em `N3_6_QUADROS.md`.

## G. Anatomia

Mesma escala em todos os quadros, com o topo da cabeça entre as linhas 14 e 16 e altura de 101 a 105 texels. Não há mudança de comprimento de braço ou perna além da perspectiva do movimento. Nas folhas, a escala e o cabelo variam entre as poses.

## H. Cabelo

Comprimento constante, sempre para trás, ondulando com a passada. É contínuo porque os quadros são consecutivos. `N3_6_CABELO.md`.

## I. Extração

GStreamer → chroma key (margem de verde 30) → máscara erodida em 1 px → limite do excesso de verde. Janela fixa de 496×552 px. Intermediários em `frames/`.

## J. Redução

O método da N3.5 sem mudanças: BOX com alfa pré-multiplicado, alfa ≥ 50 % e paleta N3.4 sem `M`/`e`, numa escala única de 0,23661 fixada pelo quadro 0, que vira 106 texels. Zero edições à mão.

## K. Resolução

Canvas de 112×128 (a passada chega a 85 texels), figura de 106, densidade 2 (53 unidades). O 80×128 da N3.5 continua para o personagem parado. `N3_6_RESOLUCAO.md`.

## L. Pivô

(56, 118) para todos os clipes. Um só deslocamento global, sem ajuste por quadro. Os quadros de voo sobem até 5 texels, como no vídeo. O quadro de pé fica 1 texel acima do chão. `N3_6_ALINHAMENTO.md`.

## M. Temporização

`run6` testado a 8, 10 e 12 fps (GIFs). A 12 fps tem a velocidade real do vídeo. `run10` usa o tempo real (2/24 e 1/24 s). Escolha do agente: **`run6` a 12 fps**, porque 8 fps parece uma corrida lenta e pesada, e o `run10` só acrescenta quadros intermediários. O GIF arredonda 83 ms para 80 ms. A corrida do jogo (310 u/s) é bem mais rápida que a passada do vídeo (cerca de 78–170 u/s): é uma decisão pendente.

## N. Bevy

Fonte `n3.6-run-proof` no `n3_visual_proof`, com R (clipe), F (fps), O (sobreposição), T (deslocamento), Espaço (pausa) e `.` (quadro a quadro). O HUD mostra resolução, figura, densidade, câmera, px por texel, temporização e `NOT APPROVED`. A sobreposição é feita com sprites, porque o renderizador de gizmos não está nas features do projeto. `N3_6_BEVY.md`.

## O. Câmera

360: os 6 quadros saem idênticos ao sprite (0 diferenças). 405: o quadro 2 cai de 85×101 para 76×89 px de tela. `camera.ron` não mudou.

## P. Fundo escuro

No quase preto, a calça e a bota perdem a borda. A borda de luz discreta (1 texel frio só nas costas) devolve a borda e fica apenas como teste. Na cena real do jogo, as pernas leem com pouco contraste. `N3_6_FUNDO_ESCURO.md`.

## Q. Salto

Há uma sequência normal completa só em 3/4 (`idle_run_jump` #15–19). Em vista lateral, as poses estão espalhadas e misturadas com acrobacia, e #17 tem uma peça de tecido estranha. Falta uma queda lateral limpa. `N3_6_ESTUDO_SALTO.md`.

## R. Ataque

Há um direto em `attack` #1–6, sem antecipação, e o efeito de impacto está desenhado junto do corpo. `al.jpg` está espelhada e tem marcadores. Nada foi integrado ao combate. `N3_6_ESTUDO_ATAQUE.md`.

## S. Idle

O idle de `idle_run_jump` está em 3/4, com outra roupa. O vídeo só tem 3 quadros parados. O idle lateral continua sendo o candidato N3.5. `N3_6_ESTUDO_IDLE.md`.

## T. Modularidade

Riscos registrados em `N3_6_AVALIACAO.md`: largura da passada, cabelo assado no corpo entre a capa e o torso, mão sem soquete por quadro, ombro, sincronização com durações desiguais e espelho. Nada foi implementado.

## U. Testes

fmt, check e clippy (`-D warnings`) limpos. `cargo test -j 2`: 50 suítes, 616 passaram, 0 falharam, incluindo `tests/n3_6_run_proof_tests.rs` (novo, 5 testes). `N3_6_TESTES.md`.

## V. Evidências

Em `docs/development/n3_6/evidence/`: `references_overview`, `character_identity`, `run_pose_analysis`, `run_anatomy_comparison`, `run_hair_comparison`, `run_selected_frames`, `run_alignment`, `run_cycle`, `run_405`, `run_360`, `run_dark_background`, `run_bevy_frames`, `jump_pose_analysis`, `attack_pose_analysis`, `idle_pose_analysis`, `bevy_n36`, `bevy_n36_405` e `bevy_n36_360` (PNG), além dos GIFs `run6_8fps`, `run6_10fps`, `run6_12fps`, `run6_12fps_dark` e `run10_video_timing`, que mostram a sequência real.

## W. Segurança

Os SHA-256 dos dez arquivos de `Md-usario/assets-user/prota/base/` estão registrados, e os originais estão intactos. Nada de `Md-usario/` foi commitado. As evidências da N3.5 que o proprietário destacou não foram regravadas.

## X. GitHub

Push em `origin/art/v1.2-protagonist`, sem force, merge ou rebase:

- `f6e156e989eccc02684e4523c65678d71caf7ef3` feat(visual): add n3.6 animation feasibility proof
- `1cf74d2dcdda320431341633f37a1e4c00886fb2` docs(n3): record animation reference analysis and evidence
- O commit que grava este parágrafo é o registro da publicação. O hash remoto final é o HEAD depois do push dele.

## Y. Limitações

- As referências de movimento foram geradas por IA (pelo proprietário). Pequenos borrões de movimento continuam em alguns quadros de voo.
- Não houve pintura humana e não há edição à mão nos quadros.
- O quadro de pé fica 1 texel acima do chão.
- A velocidade do jogo não combina com a passada.
- Calça e bota escuras perdem a borda em fundo quase preto.
- Não há salto, ataque nem idle animados.

## Z. Recomendação

1. **Avaliar** `run6_12fps.gif`, `run_cycle.png` e `bevy_n36_360.png` (Gates A e B).
2. Decidir a **velocidade**: tocar o clipe mais rápido, aumentar a passada ou reduzir a velocidade de corrida do jogo.
3. Decidir a **borda de luz** e a **câmera 360**, que continuam pendentes desde a N3.5.
4. Para as próximas animações (salto, ataque, idle), pedir **vídeos laterais no mesmo estilo**, com câmera fixa e fundo verde. O processo da corrida pode ser repetido do mesmo jeito.

Nenhuma animação entra no jogo e a N4 não começa sem autorização.
