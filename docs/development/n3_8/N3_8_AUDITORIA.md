# N3.8 — Auditoria inicial

## Estado do Git

- Branch: `art/v1.2-protagonist`.
- HEAD local: `a44beab8e767bd0fb5b3eb4c5a381912cbf4ba13` (`docs(n3): record published n3.7 15 fps commits`).
- HEAD remoto: igual ao local depois de `git fetch`.
- Árvore: limpa. Os únicos não rastreados estão em `Md-usario/`: os briefings N3.4–N3.8, N4 e N5 e `assets-user/prota/base/Ficha_de_Modelagem_do_Protagonista_Gotico.png`. São arquivos pessoais e não foram tocados nem commitados.

## Lido antes de qualquer mudança

`CLAUDE.md`, `.claude/rules/`, `docs/ARCHITECTURE.md`, `N3_7_RELATORIO_FINAL.md`, `N3_7_TESTE_15FPS.md`, `N3_7_CONTRATO_ANIMACAO.md`, `N1_PERSONAGEM_MODULAR.md` e `N1_EQUIPAMENTOS_VISUAIS.md`. No código: `src/player/visual/{mod,manifest,playback,selection,sockets,systems}.rs`, `src/inventory/{visual,held_visual}.rs`, `src/app/mod.rs`, `tests/data_tests.rs`, `tests/held_visual_tests.rs`, `tests/n3_7_sync_tests.rs` e `art-src/player/scripts/post/publish_guard.py`.

## O que `cargo run` mostrava

O protagonista de produção é o **V2.2**: figura encapuzada de casaco longo e capa vermelha, renderizada do modelo v007 (`assets/animations/player.ron`, atlas `locomotion` 128×128, `combat` 256×192, `effects`). Todos os estados têm clipe próprio: idle (12 quadros), run (10 quadros a 24 fps com `speed_reference: 190`, ou seja ~39 fps a 310 u/s), dodge, hurt, death, jump_rise, fall, land, crouch, guard_start, ataques, magias e efeitos. Os soquetes por quadro estão em `player_sockets.ron` (medidos no Blender), e a arma e o escudo são camadas.

## Incompatibilidade encontrada

A corrida N3.6 é **outro personagem**: cabelo prateado, camisa cinza, calça escura, sem capa e sem arma. Integrar só a corrida faria o protagonista trocar de identidade toda vez que começa ou para de correr, salta, ataca, defende, esquiva, sofre dano ou morre.

Duas consequências técnicas:

1. **Soquetes:** `SocketTable::validate` exige o mesmo número de quadros por clipe. Trocar o `run` de 10 por 6 quadros invalidaria a tabela inteira, e a arma e o escudo sumiriam em todos os estados.
2. **Publicação:** `player.ron` só é escrito por `publish_guard.py` (trava de hashes). O teste `player_sprite_manifest_matches_its_textures_and_the_contract` exige a célula 64×64 com pivô (32, 63) para o `run`, e a N3.6 usa 56×64 com pivô (28, 59).

## Decisão do proprietário (registrada na conversa da N3.8)

Opção 3, com ajustes obrigatórios:

- O V2.2 continua como **padrão funcional**, sem alterações destrutivas.
- O novo protagonista (idle N3.5 + corrida N3.6 a 15 fps) entra num **modo experimental de apresentação**.
- Idle e corrida mantêm a mesma identidade. Os outros estados **não** usam animações V2.2. Eles usam uma representação temporária claramente identificada (a pose N3.5), sem mudar física, hitboxes, duração das ações ou combate.
- A promoção ao `cargo run` padrão é um gate **PENDENTE**.

## Achado durante a integração: filtragem

O jogo principal (`app::build_inner`) usa o amostrador padrão do Bevy (**linear**). As provas aprovadas (`n3_visual_proof`) usam `ImagePlugin::default_nearest()`. Com o linear, a arte N3 desenhada pelo jogo coincidia com o arquivo aprovado em só 66,9 % dos pixels da vista 360 (0 de 24 capturas exatas). Detalhes e correção em `N3_8_ALINHAMENTO.md`.
