# Modular Character Engine 01 — Primeiro protótipo (rig 2D com equipamento preso a ossos)

**Status:** AGUARDANDO APROVAÇÃO VISUAL
**Aprovação artística:** pendente

A execução técnica está concluída e validada. O julgamento artístico é do proprietário: a
aprovação da abordagem híbrida e a qualidade das peças.

## 1. Identificação
- Etapa: Modular Character Engine 01, primeiro protótipo
- Frente: modular-character-engine
- Branch: `art/v1.2-protagonist`

## 2. Data e hora da execução
2026-11-21, das 15:20 às 16:05 (-03). O início é aproximado; a hora das capturas está nos
arquivos.

## 3. Objetivo original
Suspender a geração de imagens completas do protagonista equipado (Takes 03D, Prompt 04) e
provar, num laboratório Rust/Bevy isolado, um subsistema próprio de personagem 2D modular. O
pedido incluía:
- esqueleto compatível com o protagonista aprovado;
- arte existente;
- idle e corrida usando os 6 quadros aprovados;
- espada presa à mão e ombreira presa ao ombro;
- troca entre duas variantes durante a animação;
- equipar e desequipar sem reiniciar a animação;
- espelhamento;
- profundidade correta entre cabelo, braços, tronco e equipamentos.

O briefing pediu também uma comparação honesta com uma abordagem totalmente articulada e
um registro da origem de cada parte (PixelFX, ASF, projeto, novo).

## 4. Prompt executado
`Md-usario/engine/Engine-01.md`, arquivo do proprietário que eu não editei nem commitei. Na
execução ele ainda não estava versionado (`?? Md-usario/engine/`).

## 5. Contexto técnico inicial
- Branch sincronizada em `4760b6f`.
- A última etapa era o 03D Take 03, AGUARDANDO APROVAÇÃO VISUAL.
- Já existiam:
  - a composição em camadas por dados (N4-A, `composition.rs`/`composite.rs`, um
    `VisualPlayback`);
  - o equipamento visual por folhas (N4-1-A);
  - a fábrica N41-B, com espadas e ombreiras isoladas "sem soquete validado";
  - a corrida C1 em 3 quadros (N4-C);
  - os soquetes por quadro da V2.2 (`sockets.rs`);
  - o laboratório D12 (`modular_lab.rs`, loadouts assados).
- Arte aprovada: idle N3.5 de 80×128 e corrida N3.6 de 6 quadros de 112×128, cópias na N3.8.
- Não havia esqueleto 2D nem IK no código Rust.

## 6. Ferramentas, plugins, skills e versões
- Rust 2024 (MSRV 1.97.1) e Bevy 0.20.1, sem dependência nova.
- pixelfx 0.3.1 e Agent Sprite Forge 0.6.0 (plugins locais): só leitura e estudo. Nenhuma API
  paga e nenhuma geração de imagem.
- Python 3.11 + Pillow (venv `~/.local/share/dc-art-tools/asf-venv`), só para ferramentas
  offline.
- Um subagente de leitura (Explore) investigou os dois plugins e as licenças.

## 7. Arquivos consultados
- `CLAUDE.md`, `.claude/rules/*`, `docs/relatorios/PADRAO_RELATORIO.md`, `docs/ARCHITECTURE.md`,
  `docs/DECISIONS.md`.
- `src/player/visual/{mod,composite,composition,sockets,equipment_visual,modular_lab,playback,manifest}.rs`,
  `src/core/pixel_image.rs`, `examples/n4_sprite_machine.rs`, `tests/n4c_run_c1_tests.rs`.
- `docs/development/n4/N41B_RELATORIO.md`, `N4C_RESULTADO_C1.md`, `art-src/n4-sprite-machine/README.md`,
  `art-src/n3-8-production-run/{README.md,stage/animations/player.ron,analysis/regions.json}`,
  `art-src/n3-6-animation-proof/{README.md,source/video_0*.txt}`,
  `art-src/n3-4-original-protagonist/scripts/n34_palette.py`,
  `art-src/n4-sprite-machine/stage/factory/{isolated,provenance}/*`.
- pixelfx: `LICENSE`, `NOTICE.md`, `src/core/{rig,poses,act,actor,chain}.js`.
- ASF: `LICENSE`, `THIRD_PARTY_NOTICES.md`, `skills/codeart2d/{references/rig-animation.md,scripts/rig_animate.py,render_pixelspec.py,pixel_qa.py}`,
  `generate2dsprite/scripts/build_animation_clips.py`, `video2dsprite/scripts/pixel_motion.py`.
  A leitura foi feita pelo subagente.

## 8. Arquivos criados
- `src/player/visual/rig/{mod,xform,skeleton,ik,layers,equipment,articulated,data}.rs`
- `examples/modular_character_engine.rs`
- `tests/modular_character_engine_tests.rs`
- `art-src/modular-character-engine/README.md`, `stage/rig_lab.ron`,
  `stage/equipment/pauldron_{steel,bronze}_half.png`, `scripts/derive_pauldrons.py`,
  `scripts/make_evidence.py`
- `evidence/modular_character_engine/01_prototipo/` (16 imagens/GIFs + `metrics.json`)
- `docs/development/modular_character_engine/01_ARQUITETURA.md`
- este relatório

## 9. Arquivos modificados
- `src/player/visual/mod.rs`: `pub mod rig;` e a linha da documentação do módulo.
- `docs/ARCHITECTURE.md`: um parágrafo e a tabela de módulos.
- `docs/DECISIONS.md`: D-046.
- `.claude/rules/architecture.md`: lista de módulos puros.
- `.claude/rules/testing.md`: linha do teste novo.
- `docs/relatorios/INDICE.md` e `ULTIMO_RELATORIO.md`.

## 10. Arquivos removidos
Nenhum.

## 11. Descrição detalhada das alterações

### Esqueleto
- 15 marcos: pelve, pescoço, cabeça; ombro, cotovelo e punho dos braços de perto e de longe;
  joelho, tornozelo e ponta do pé das pernas A e B.
- 12 ossos em hierarquia com a espinha como raiz.
- Marquei os marcos à mão em grades ampliadas 8× dos 7 quadros aprovados (idle + 6 de
  corrida).
- `fit` calcula, para cada osso, o quadro do mundo e o quadro local relativo ao pai.
- `forward` recompõe por FK. O teste confere que a ida e volta reproduz todo marco.

### IK de dois ossos
- Lei dos cossenos, com o lado da dobra nomeado por rotação.
- Limite de alcance e limite de "perto demais" (sem NaN).

### Camadas de profundidade (abordagem híbrida)
- Cada quadro aprovado vira 5 camadas: braço de longe, pernas, tronco, cabelo e braço de
  perto.
- O corte usa cápsulas em volta dos ossos dos braços e a paleta N3.4: só pele, camisa, couro
  e contorno entram no braço; o cabelo é o que tem cor de cabelo atrás da espinha ou acima do
  ombro.
- É uma partição: o Bevy recompõe o quadro com 0 texels diferentes nos 7 quadros.
- Os slots de equipamento ficam entre as camadas: espada sob o punho e sobre o corpo e o
  cabelo; ombreira sobre o braço.

### Equipamento preso a ossos
- `RigItem` define imagem, tamanho, pivô, osso, ponta, deslocamento, `up_degrees`, `follow`
  (fração do giro do osso), profundidade, `provisional` e origem.
- `place` transforma a pose do quadro em posição e rotação; `mirror` usa `sockets::place`.
- `RigLoadout` guarda um item por slot. Trocar de item não toca o relógio: o quadro sai
  sempre de `playback::loop_frame(elapsed)`.

### Comparação totalmente articulada
- As peças são recortadas do idle aprovado, um pedaço por osso da cadeia de perto (o mais
  próximo).
- Ficam nas proporções do idle com os ângulos de cada quadro (FK + `retarget`).
- As pernas são resolvidas por IK até o tornozelo marcado, e o pé mantém o ângulo do quadro.
- Os membros de trás reutilizam as peças de perto, escurecidas, porque o idle lateral os
  esconde.

### Laboratório Bevy
Três painéis (original, híbrido, articulado) a 3 px por texel, com sobreposição de
esqueleto, soquetes, camadas coloridas e vista explodida. Inclui um roteiro de 76 capturas e
verificações objetivas: partição sem perda, continuidade do quadro durante as trocas e
`RESULT: PASS/FAIL`.

### Idle
O idle aprovado tem 1 quadro. Criei um segundo quadro **experimental** de respiração: braços,
tronco e cabelo descem 1 texel, e as pernas ficam fixas. A ideia é a "breath" do pixelfx. Só
é um deslocamento de camadas: nenhum texel foi redesenhado.

### Peças
- Espadas `sword_arming_cross` (A) e `sword_estoc_ring` (B) da N41-B, usadas como estão.
- Ombreiras `pauldron_steel` (A) e `pauldron_bronze` (B) da N41-B, reduzidas 2:1. As
  originais têm 28×30 texels, cerca de duas vezes o ombro. A redução usa a cor mais frequente
  por bloco 2×2, sem cor nova. São derivadas e **provisórias**.

## 12. Decisões técnicas e artísticas
- **Abordagem híbrida como proposta.** Segundo o briefing, valia testar a articulação
  completa. Testei e o resultado fica muito abaixo da arte aprovada (seção 20), então a
  proposta é corpo aprovado + equipamento preso a ossos.
- **Partição em tempo de carga, no Rust.** Não criei arquivo de camada nem cópia da arte, e
  há uma única fonte de marcos (o RON). Python não entra no jogo nem no exemplo.
- **Mão principal = braço de perto** (lado da câmera), como proposto na N4-C. Ao espelhar, a
  ordem de profundidade fica igual.
- **A espada fica perpendicular ao antebraço** (`up_degrees: 90`), por falta de um osso de
  pulso. A consequência está na seção 20.
- **Ombreira com `follow: 0.5`:** acompanha metade do giro do braço, porque uma ombreira
  rígida virava de cabeça para baixo quando o braço ia para trás.
- **Nada no jogo:** o módulo não foi registrado no plugin. Combate, física, inventário,
  progressão e saves não mudaram.
- **A fábrica de máscaras da camisa não foi usada.**

## 13. Comandos relevantes executados
```bash
~/.local/share/dc-art-tools/asf-venv/bin/python -B art-src/modular-character-engine/scripts/derive_pauldrons.py
cargo test --lib -j 2 rig::
cargo test -j 2 --test modular_character_engine_tests -- --nocapture
cargo run --example modular_character_engine -j 2 > target/modular_character_engine/run.log
~/.local/share/dc-art-tools/asf-venv/bin/python -B art-src/modular-character-engine/scripts/make_evidence.py
cargo fmt && cargo fmt --check
cargo clippy --all-targets -j 2 -- -D warnings
cargo test -j 2
cargo check -j 2
git diff --check
```

## 14. Testes realizados
- 31 testes unitários novos em `rig::*`:
  - transformações;
  - FK de ida e volta;
  - pai carregando filhos;
  - respiração;
  - retarget;
  - IK (alcance, lado da dobra, fora de alcance, perto demais);
  - paleta e materiais;
  - partição cobrindo os texels opacos;
  - ordem de profundidade;
  - recorte de célula e máscara;
  - colocação da espada no punho;
  - lâmina em pé com antebraço horizontal;
  - `follow`;
  - espelho;
  - loadout;
  - contrato RON válido e 7 casos inválidos;
  - pose articulada.
- 7 testes de integração (`tests/modular_character_engine_tests.rs`) com os arquivos reais:
  - caminhos da arte aprovada e das peças;
  - corrida 6 × 1/15 s;
  - marcos em cima da arte (punho em texel de mão, pelve e cabeça opacas);
  - partição das grades de paleta reais (cada texel opaco uma vez, as 5 camadas presentes,
    punho na camada do braço de perto);
  - espada e ombreira no punho e no ombro de todos os quadros, espelhadas, com rotações
    diferentes entre as poses;
  - troca de equipamento sem mexer no quadro;
  - IK medida.
- Roteiro com janela (`cargo run --example modular_character_engine`), com partição,
  continuidade e `RESULT`.
- Comparação pixel a pixel original × híbrido sem equipamento, nas capturas
  (`make_evidence.py`).

## 15. Resultado dos testes
- `cargo fmt --check`: ok.
- `cargo clippy --all-targets -j 2 -- -D warnings`: limpo, depois de trocar um `chunks_exact`
  por `as_chunks`.
- `cargo test -j 2`: **57 suítes, 718 passaram, 0 falharam**.
- `cargo check -j 2`: ok.
- Exemplo: `RESULT: PASS` (76 capturas). Partição: 0 texels diferentes em idle #0 e run #0–#5.
- Original × híbrido sem equipamento: 0 px diferentes em `bare_idle_f0` e `bare_run_f0..f5`.
  Em `bare_idle_f1` há 4671 px diferentes de propósito: é o quadro de respiração, sem
  equivalente no idle aprovado de 1 quadro.
- Troca durante a corrida: o quadro seguiu 0,1,2,3,4,5,0,… nos 24 passos e nos 18 passos de
  virada, sem reinício.
- `git diff --check`: limpo.

## 16. Métricas e comparações
- `evidence/modular_character_engine/01_prototipo/metrics.json`.
- IK do modo articulado: 2 de 12 tornozelos da corrida estão além da perna do idle. A perna
  A do quadro 1 passa 2,94 texels e a perna B do quadro 4 passa 1,92. Nesses casos a perna
  estica até o comprimento de repouso e para antes do alvo, o que confirma que a corrida
  aprovada desenha pernas mais longas que o idle.
- Tamanho: 16 evidências, 1,1 MB.
- Código novo: cerca de 3200 linhas (módulo puro, exemplo e teste).

## 17. Problemas encontrados
- Uma assertiva antiga do teste de IK exigia alcançar todo tornozelo e falhou no quadro 1
  (2,96 texels). Era um limite real das proporções, não um bug.
- As ombreiras da N41-B são grandes demais para o ombro em 1×.
- A espada perpendicular ao antebraço fica apontada para a frente no idle (braço caído).
- O lado da dobra da IK estava invertido na primeira versão (o joelho ia para trás). Um teste
  unitário pegou o erro.
- O clippy pediu `as_chunks` em vez de `chunks_exact(4)`.

## 18. Problemas corrigidos
- Lado da dobra da IK corrigido, com teste.
- O teste de IK virou uma medida: alcança o que a perna alcança e estica em linha reta até o
  resto. Os casos fora de alcance são impressos e limitados a no máximo 4. Nenhuma assertiva
  foi enfraquecida sem explicação.
- Ombreiras reduzidas 2:1 como peça derivada e marcada como provisória.
- Lint do clippy resolvido.

## 19. Problemas pendentes
- **Espada no idle:** aponta para a frente. Falta um osso de pulso ou um ângulo de empunhadura
  por clipe.
- **Marcos estimados (cerca de ±2 texels):** a espada e a ombreira tremem um pouco entre
  quadros, sobretudo a ombreira nos quadros 3–5.
- **Camadas aproximadas:**
  - as cápsulas do braço de perto pegam alguns texels da camisa do tronco nos quadros 1–2;
  - os tons de cabelo usados no sombreamento da camisa podem ir para a camada de cabelo.

  Isso só importa onde uma peça passa entre essas camadas.
- **Braço de longe e pernas A/B:** são interpretação (o braço de longe fica parcialmente
  escondido).
- **Arte das peças:** não aprovada. As espadas e as ombreiras originais vêm da N41-B, que
  também não estava aprovada.

## 20. Limitações técnicas

### Abordagem articulada (comparação)
Visivelmente inferior. Peças do idle movidas por FK/IK:
- juntas abertas no quadril e nos joelhos;
- membros finos de perfil;
- cabelo caído em vez de solto ao vento;
- tronco sempre de perfil, sem mudança de perspectiva;
- texels girados em ângulos arbitrários, que serrilham e não respeitam a grade;
- membros de trás como cópias escurecidas;
- 2 tornozelos fora de alcance.

Não serve como corpo do jogo hoje.

### Abordagem híbrida
- O corpo continua preso aos quadros aprovados: cada pose ou animação nova exige arte do corpo
  (com marcos).
- Só o equipamento é modular.
- Roupas e armaduras de tronco que precisam deformar com o corpo não foram resolvidas. A
  camada `Torso` está pronta, mas o item deformável não existe.
- Capa (`BackGear`), escudo (`FarHandGear`), paletas, equipamento articulado ou flexível e
  efeitos por evento estão só previstos (arquitetura §9).

### Profundidade
Fixa por quadro: uma lâmina longa passa sempre por cima de cabeça e tronco. É plausível na mão
de perto, mas não há casos de "atrás da cabeça".

### Espelhamento
Por inversão do sprite, convenção 2D: a espada troca de mão anatômica.

## 21. Evidências visuais
- **01_skeleton_overlay.png:** esqueleto sobre o idle e os 6 quadros aprovados.
- **02_skeleton_motion.gif:** o esqueleto na corrida aprovada, com o mesmo esqueleto movendo
  as peças do idle ao lado.
- **03_run_equipped.gif / _frames.png:** corrida híbrida com espada A + ombreira A.
- **03b_idle_equipped.gif:** idle de 2 quadros (respiração) com o equipamento acompanhando.
- **04_swap_during_run.gif / _frames.png:** 24 passos com troca e remoção durante a corrida.
  A legenda mostra quadro e itens.
- **05_turn_around.gif / _frames.png:** virada para a esquerda e de volta no meio da corrida.
- **05b_mirror_still.png:** espelhado parado, com as variantes B e o esqueleto.
- **06_comparison_original_hybrid_articulated.gif / _frames.png:** original × híbrido ×
  articulado, quadro a quadro.
- **07_pieces_authored.png:** as 4 peças como foram desenhadas (4×), com o pivô e o osso de
  cada uma.
- **07_pieces_and_sockets.png:** soquetes (ciano), eixo do item (magenta) e osso vinculado no
  idle e na corrida, mais a vista explodida.
- **08_depth_layers.png / .gif:** as 5 camadas coloridas (braço de perto vermelho, de longe
  azul, cabelo amarelo, tronco cinza, pernas marrom), com a espada entre o cabelo e o punho e
  a ombreira por cima.

## 22. Caminhos das imagens, GIFs e vídeos
`evidence/modular_character_engine/01_prototipo/`:
- `01_skeleton_overlay.png`
- `02_skeleton_motion.gif`
- `03_run_equipped.gif`, `03_run_equipped_frames.png`, `03b_idle_equipped.gif`
- `04_swap_during_run.gif`, `04_swap_during_run_frames.png`
- `05_turn_around.gif`, `05_turn_around_frames.png`, `05b_mirror_still.png`
- `06_comparison_original_hybrid_articulated.gif`, `06_comparison_frames.png`
- `07_pieces_authored.png`, `07_pieces_and_sockets.png`
- `08_depth_layers.png`, `08_depth_layers.gif`
- `metrics.json`

As capturas brutas (76 PNG e `run.log`) ficam em `target/modular_character_engine/`, que não é
versionado.

## 23. Links do GitHub
Já publicados no push `4760b6f..f18b381`:
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/docs/development/modular_character_engine/01_ARQUITETURA.md
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/modular_character_engine/01_prototipo/06_comparison_original_hybrid_articulated.gif
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/modular_character_engine/01_prototipo/04_swap_during_run.gif
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/modular_character_engine/01_prototipo/05_turn_around.gif
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/modular_character_engine/01_prototipo/08_depth_layers.png
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/src/player/visual/rig/mod.rs

## 24. Resultado final
Os 9 itens do protótipo foram demonstrados no Bevy:
1. esqueleto nas proporções do protagonista;
2. arte aprovada como base;
3. idle e corrida (os 6 quadros);
4. espada no punho;
5. ombreira no ombro;
6. troca A/B durante a corrida;
7. equipar e desequipar sem reiniciar o quadro;
8. espelhamento;
9. profundidade entre cabelo, braços, tronco e equipamento.

A mesma peça, desenhada uma vez, acompanha 8 poses diferentes (2 de idle e 6 de corrida) e a
pose articulada.

A comparação mostra que a articulação completa ainda não alcança a qualidade da arte aprovada.
Por isso a proposta é a abordagem híbrida.

## 25. Recomendações para a próxima etapa
1. Avaliar `06_comparison_*.gif`, `04_swap_during_run.gif` e `08_depth_layers.png`, e decidir
   se a abordagem híbrida segue.
2. Se seguir, há três passos técnicos, nesta ordem:
   - osso de pulso e ângulo de empunhadura por clipe, para a espada ficar natural no idle;
   - refinar os marcos com uma passada de medição (subpixel e consistência entre quadros);
   - levar camadas e itens ao `CompositeCharacter` com o `VisualPlayback` do jogador, ainda
     atrás de uma flag experimental.
3. Arte das peças: desenhar uma ombreira e uma espada na escala do protagonista, à mão ou
   refinando a fábrica, em vez da redução 2:1.
4. Não produzir armaduras em quantidade antes da etapa 2. Primeiro, provar uma capa
   (`BackGear`) e um item de tronco deformável.

## 26. Decisões que dependem do proprietário
1. Aprovar ou não a abordagem híbrida como base do motor.
2. Confirmar a mão principal: o braço de perto, com a espada trocando de mão anatômica ao
   espelhar.
3. Definir como a espada deve ser empunhada no idle e na corrida: na mão, apontando para cima
   ou para trás, ou na bainha.
4. Aceitar as peças provisórias para a próxima prova ou pedir arte própria.
5. Autorizar a integração experimental ao jogador (etapa seguinte).

## 27. Estado do Git e publicação
- Branch: `art/v1.2-protagonist`
- Commit(s) da tarefa:
  - `7c70e6d` feat(player): add the modular character rig rules
  - `412bef2` feat(data): add the modular character engine lab rig data
  - `787838e` test(player): validate the rig lab against the approved art
  - `2ba3d48` feat(player): add the modular character engine lab example
  - `f9e29e4` docs(art): add the modular character engine prototype 01 evidence
  - `2298e7c` docs(architecture): document the modular character engine prototype
  - `f18b381` docs(claude): list the modular character rig module and tests
  - O commit deste relatório, com `INDICE.md` e `ULTIMO_RELATORIO.md`, vem depois. O hash
    dele é o HEAD publicado.
- Push: feito, sem force (`4760b6f..f18b381`), seguido do push do commit do relatório.
- Local x remoto: sincronizado. `Md-usario/` não foi alterado nem commitado.
