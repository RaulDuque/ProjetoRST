# N1 — Animação esquelética 2D

Abordagem B do pipeline: corpo em partes, ossos em runtime, equipamento troca o sprite do osso.

## Decisão

Não adotar rig 2D como caminho principal. O jogo já sincroniza camadas pelo quadro do corpo, não por osso.

O que existe:

- `AnimationManifest` descreve atlas, clipe, pivô, duração e `speed_reference`.
- Modos `Loop`, `FitToAction` e `Once`. O golpe estica na duração real da ação, não o contrário.
- `player_sockets.ron` marca âncoras por quadro. Arma, escudo e capa leem a âncora do quadro mostrado.
- A lógica de hitbox mora em `strike_timeline` e nas ações. O visual atrasado não cria nem apaga o dano.

Clipes publicados hoje: `idle`, `run`, `dodge`, `hurt`, `death`, `jump_rise`, `fall`, `land`, `crouch`, `guard_start`, `attack_basic`, `attack_basic_air`, `slash_crescent`, `slash_rising`, `cast_forward`, `cast_up`, mais VFX. Não há `walk`, `start`, `stop`, `turn`, `crouch_walk`, `jump_start`, `double_jump`.

## Lista desejada e o que fazer com ela

Locomoção fina (walk, start, stop, turn, crouch walk, jump start, double jump) entra como clipe novo no mesmo manifesto, quando a animação existir. Até lá o jogo usa `run`, `crouch`, `jump_rise` e `fall`, que já têm fallback para `idle`.

Combate (ataque pesado, combo, guard hit, use item) só ganha clipe quando a ação de gameplay existir. Desenhar o quadro antes da regra produz arte órfã.

Escalada, voo e nado esperam a relíquia correspondente. A Forma Espectral já troca a cor do corpo; não é outro esqueleto.

## Sincronização

O quadro visual pode nomear um evento (`hit`, `cast`, `step`), mas quem aplica o efeito é a ação, no tempo do gameplay. Se o evento visual faltar, o golpe sai mesmo assim. Se sobrar, é ignorado. Som e VFX assinam os mesmos eventos, sem segurar a hitbox.

Direção: a arte nasce virada para a direita (`ArtFacing::Right`) e o sprite espelha. Âncora espelha junto, como a V2.2 já faz.

## Alternativas

Spine ou um runtime de ossos dentro do Bevy. A lista oficial de runtimes do Spine não inclui Bevy, e o custo está em `N1_COMPARACAO_FERRAMENTAS.md`. Partes soltas sem osso, só com âncora, é o que a arma e a capa já são. Isso cobre troca de equipamento sem deformar pixel.

## Riscos

Osso 2D gira um sprite e o pixel deixa de ser quadrado. Articulação no cotovelo parece dobradiça. Pose de ataque com o braço cruzando o torso exige ordem de desenho por quadro, coisa que um osso genérico resolve mal e uma âncora com ordem explícita resolve.

## Recomendação

Crescer a tabela de âncoras e a lista de clipes. Não escrever um esqueleto 2D até a prova mostrar um gesto que a âncora não alcança.
