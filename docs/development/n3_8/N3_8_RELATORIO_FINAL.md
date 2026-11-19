# N3.8 — Relatório final

A corrida aprovada (6 quadros, 15 fps, 310 u/s) roda no jogo real, sem nenhum pixel alterado, como **novo protagonista experimental** (`DARK_CASTLE_PROTAGONIST=n3 cargo run`) junto com o idle N3.5. O protagonista de produção V2.2 continua padrão e intocado, por decisão do proprietário. **A promoção ao `cargo run` padrão está PENDENTE, e a substituição do protagonista NÃO está concluída.** A arquitetura modular da N4 está proposta para aprovação; nada dela foi implementado.

## A. Estado inicial

`art/v1.2-protagonist` em `a44beab8e767bd0fb5b3eb4c5a381912cbf4ba13`, local igual ao remoto. Árvore limpa, exceto os não rastreados de `Md-usario/` (briefings N3.4–N5 e uma ficha PNG), que foram preservados. `N3_8_AUDITORIA.md`.

## B. Decisão artística

Corrida N3.6 de 6 quadros (vídeo 48–58) a 15 fps, ciclo de 0,4 s, 5 passos por segundo, 310 u/s. Deslizamento de ~37 % aceito. Física, quadros e fps automáticos: inalterados.

## C. Integração

A auditoria mostrou que a produção é outro personagem (V2.2). O proprietário escolheu um modo experimental: `art_profile::ProtagonistArt` + `app::build_for_protagonist` carregam a arte de `art-src/n3-8-production-run/stage/` pelo mesmo carregador, manifesto, seleção, relógio e espelhamento de sempre. Não há sistema paralelo. Um selo de UI identifica o modo. `N3_8_CORRIDA_PRODUCAO.md`.

## D. Temporização

6 × `0.06666667` s = 0,4 s, `Loop(fps: 15)`, `speed_reference: None` (taxa fixa). Relógio único (`VisualPlayback.elapsed` + `loop_frame` com `rem_euclid`), dt limitado a 1/30 s, sem reinício por velocidade. Medido no jogo: **15,0 fps**, quadros de 3 ou 4 updates de 20 ms. Testado a 30, 60, 144 fps e com taxa variável. `N3_8_TEMPORIZACAO.md`.

## E. Arte

As cópias no staging são **byte a byte** iguais a `candidate_106.png` e `run6.png` (teste). Os originais passam no teste de hash da N3.7. Na vista 360, **24 de 24 capturas do jogo são idênticas pixel a pixel ao `run6.png`**.

## F. Escala

Densidade 2: ~53 u de altura (V2.2: ~55 u). Colisão 26×46 inalterada.

## G. Pivô

Por clipe: idle (40,118) em 80×128 e corrida (56,118) em 112×128. A sola fica na base da colisão. Nada foi alinhado pela caixa opaca, e nenhum quadro foi movido. `N3_8_ALINHAMENTO.md`, `n38_feet_alignment.png`.

## H. Espelhamento

`flip_x` + âncora espelhada (x = 0 nos dois canvas): sem deslocamento ao virar e sem folha espelhada. Testado e capturado (`n38_run_left_15fps.gif`, `n38_run_turn.gif`). A assimetria (braço da frente, cabelo, arma) fica para a N4.

## I. Transições

Idle ↔ corrida e mudança de direção funcionam, mas partida e parada são **cortes abruptos** (a `run_start` da N3.7 não foi usada: falta a regra de "próximo clipe"). Salto, ataque, guarda, esquiva, dano e morte mostram a **pose N3.5 parada**, com tom e selo `PLACEHOLDER`. Nenhuma animação foi inventada e nenhum quadro V2.2 é usado. O gameplay é idêntico com as duas artes (teste em 220 updates). A morte de pé é a pior leitura. `N3_8_TRANSICOES.md`.

## J. Equipamentos

Na produção nada mudou. No modo experimental, a arma e o escudo ficam **ocultos**, porque não há soquetes medidos e inventar coordenadas é proibido. O equipamento continua equipado e funcionando. A capa usa o ombro fixo da V2.0, e o tom vai para o corpo. `N3_8_COMPATIBILIDADE.md`.

## K. Câmera

405 preservada (`camera.ron` intocado), 0,889 px por texel. A vista 360 só existe no exemplo, em memória. **Achado:** o jogo usava o filtro linear do Bevy, e as provas aprovadas usam *nearest*. Com linear, só 66,9 % dos pixels coincidiam. O modo experimental agora carrega o atlas com *nearest* (`NearestPlayerArt`), e a produção ficou como estava (decisão pendente).

## L. Modularidade: arquitetura atual

O que é independente: arma e escudo (`HeldLayer`, soquetes), capa (`CapeLayer`), tom de equipamento e efeitos. Todas as camadas leem o mesmo `VisualPlayback`. Na arte N3, cabelo, rosto, antebraços, camisa, braceletes, cinto, calça e botas estão assados numa imagem só.

## M. Terraria: princípios adotados (proposta)

Um personagem, um relógio, camadas no mesmo canvas e pivô, peças independentes com folha própria, ocultação e variantes, armas por soquete, aparência separada dos atributos, sem esqueleto e sem folha por combinação. `N3_8_ARQUITETURA_TERRARIA.md`.

## N. Corpo-base

Precisa de arte nova para a cabeça atrás do cabelo, ombros e braços, torso, quadril, pernas e pés. Rosto, antebraços e mãos são recuperáveis. Maior risco: torso e pernas, cujo modelado vem da roupa, e a cabeça, porque o cabelo é a silhueta. `N3_8_RISCOS_ARTISTICOS.md`.

## O. Cabelo

Redesenhar como camada sobre a fonte, em duas fatias (traseiro e frontal), com uma variante "sob elmo". Não recortar: as mechas atrás do tronco se perderiam.

## P. Roupas

Camisa, calça e botas como folhas inteiras alinhadas, divididas em fatias de frente e trás. Famílias por silhueta compartilham estrutura e mudam só pintura e paleta.

## Q. Armaduras

Uma peça pode cobrir várias regiões (peitoral + mangas + ombreiras), esconde as camadas que cobre (`hides`) e entra numa família de silhueta. Peça volumosa = arte específica por quadro. É limite por peça, não por combinação.

## R. Armas

As 5 famílias atuais (`WeaponForm`) compartilham o golpe. A arma segue o soquete do quadro mostrado. Os soquetes N3 precisam de **marcação manual**: a medição automática (`measure_regions.py`) parte os braços em pedaços e não sabe qual braço está na frente.

## S. Cobertura

Contrato mínimo: ocultação de camada → variantes de composição → máscaras por região, só se necessário e aplicadas offline. Ordem padrão de 11 camadas, com exceções por quadro num `draw_order.ron` opcional.

## T. Animação

Um relógio só (`VisualPlayback`). Toda camada usa o mesmo índice. Movimento secundário (capa, mechas) é desvio derivado do estado principal, nunca um temporizador próprio.

## U. Save

O v3 não mudou. Aparência cosmética persistida exige um campo novo, ou seja, save v4 com struct congelado e autorização. Antes disso, só em memória. Não foi criado nenhum slot nem inventário.

## V. N4: plano

Fases A (idle modular) → B (troca de peças) → C (corrida modular, quadros #0, #2, #4 primeiro) → D (armadura, arma e peça cosmética independentes). Lista do que falta para substituir o V2.2: 13 clipes de personagem, mais soquetes e frenagem. `N3_8_PLANO_MODULAR_N4.md`.

## W. Evidências

`docs/development/n3_8/evidence/`: 10 GIFs em tempo real (direita, esquerda, partida, parada, virar, salto, ataque, guarda, esquiva, capa), comparação produção × N3, alinhamento dos pés, 405 × 360, `run6` × jogo, 3 telas inteiras e `measurements.json`. Capturas brutas em `target/n3_8_production_run/` (não versionadas). Scripts: `examples/n3_8_production_run.rs` e `art-src/n3-8-production-run/scripts/make_evidence.py`. Sem capturas de dano e morte (o roteiro não leva dano): cobertos por teste.

## X. Testes

fmt, check e clippy `-D warnings` limpos. `cargo test -j 2`: **52 suítes, 641 passaram, 0 falharam**. Guard de publicação: 21 OK. Exemplo N3.8: PASS nas 3 variantes, rodado duas vezes. `n3_visual_proof`: 41 capturas idênticas às versionadas. `N3_8_TESTES.md`.

## Y. GitHub

Ver o registro de publicação abaixo (preenchido depois do push).

## Z. Próxima decisão

1. Avaliar `n38_run_right_15fps.gif`, `n38_production_vs_n3_run.gif` e `DARK_CASTLE_PROTAGONIST=n3 cargo run`.
2. Aprovar ou ajustar a arquitetura N4 (camadas no mesmo canvas, relógio único, ocultação + variantes, sem esqueleto).
3. Fornecer ou aprovar as referências do corpo neutro.
4. Decidir: *nearest* na produção? câmera 360? arma troca de mão no espelho?
5. Autorizar o início da N4 (Fase A). A N4 e a N5 **não** foram iniciadas.
