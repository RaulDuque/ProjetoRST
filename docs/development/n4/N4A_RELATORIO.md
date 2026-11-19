# N4-A — Relatório da Fase A

A máquina de sprites existe e funciona na prova isolada. O Rust lê um personagem em camadas de um RON e o compõe no Bevy, com todas as camadas presas ao `VisualPlayback`. O idle N3.5 recomposto a partir de 8 camadas é **idêntico texel a texel** ao aprovado, e também pixel a pixel no renderizador real. Cabelo, camisa, calça, botas e braceletes saem um a um, sem buracos. A anatomia escondida é **interpretação do agente**, mais simples que a arte aprovada, e **não está aprovada**. A produção, o modo N3.8, o save e o inventário não mudaram. A corrida não foi modularizada, e a N5 não começou.

## 1. O que foi implementado

Contrato de dados RON (`composition.rs`), compositor Bevy (`composite.rs`), estado compartilhado (`VisualPlayback.frame` e `.flip`), separação do idle N3.5 em 8 camadas, reconstrução da anatomia escondida, prova isolada no Bevy e evidências. `N4A_ARQUITETURA.md`.

## 2. Arquivos criados

- Código: `src/player/visual/composition.rs`, `src/player/visual/composite.rs`; alterados `src/player/visual/mod.rs` e `systems.rs`.
- Teste: `tests/n4_sprite_machine_tests.rs`.
- Prova: `examples/n4_sprite_machine.rs`.
- Arte e dados: `art-src/n4-sprite-machine/` (`README.md`, `stage/protagonist.ron`, `stage/parts/idle/*.png|txt`, `scripts/{partition,reconstruct,build_idle_layers,check_bevy}.py`).
- Documentação: `docs/development/n4/` (9 documentos + `evidence/`); `docs/ARCHITECTURE.md` e `.claude/rules/{architecture,testing}.md` atualizados.

## 3. Partes realmente independentes

Cabelo (2 camadas), camisa, calça, botas e braceletes, cada uma com folha própria, removível sozinha; o corpo-base (2 camadas) fica sempre. `N4A_COMPOSICAO.md`.

## 4. Como o Rust lê e compõe

`CharacterComposition::from_ron` → validação → `CompositeCharacter::load` (handles por folha, grade da célula do clipe) → `spawn_layers` (um filho por slot) → `sync_composite_layers` a cada frame: `resolve(composição, roupa, clipe, frame)` → cada filho recebe folha, índice, espelho, âncora do pivô, z e visibilidade.

## 5. Sincronização

Um relógio só: `animate_player_visual` calcula o quadro como sempre e o guarda em `VisualPlayback.frame` (e o espelho em `.flip`). As camadas só leem esses campos. Testado no puro, no ECS e no jogo headless.

## 6. Corpo neutro

Desenhado linha a linha, com a paleta aprovada, só onde uma peça o cobre no original: crânio, pescoço, torso, braço superior, pulso, roupa de baixo neutra, pernas e pés (874 + 126 texels). Base: o rosto, o antebraço e a mão visíveis, a silhueta das roupas e a ficha lateral do proprietário. `N4A_RECONSTRUCAO_ARTISTICA.md`.

## 7. Comparação com a N3.5

Original × modular lado a lado: `evidence/n4a_recomposition.png` (offline) e `evidence/bevy_n4a_original_vs_modular.png` (Bevy).

## 8. Diferenças

**0** texels na grade, **0** no PNG e **0** pixels no Bevy (escuro, claro e espelhado). A igualdade é exata por construção (partição + pintura só sob cobertura). Ela prova a composição, não a qualidade da anatomia. `N4A_DIFERENCAS_VISUAIS.md`.

## 9. Remoção das peças

Original, sem cabelo, sem camisa, sem calça, sem botas, sem braceletes e corpo-base: `evidence/n4a_removal.png`, `n4a_removal_light.png` e as capturas `bevy_n4a_no_*.png` / `bevy_n4a_base_body*.png`. Nenhum buraco transparente dentro do corpo (métrica e teste). Um defeito opaco (a coxa aparecendo onde devia estar o pulso) foi achado na revisão visual e corrigido; a métrica sozinha não o pegaria.

## 10. Testes

fmt, check e clippy `-D warnings` limpos; `cargo test -j 2`: **53 suítes, 655 passaram, 0 falharam**; 14 testes novos. `N4A_TESTES.md`.

## 11. Evidências (`docs/development/n4/evidence/`)

`n4a_recomposition.png`, `n4a_diff_map.png`, `n4a_layers.png`, `n4a_pieces.png`, `n4a_removal.png`, `n4a_removal_light.png`, 11 capturas `bevy_n4a_*.png` e `metrics.json`. Sem GIF: o idle tem um quadro, então não há movimento a mostrar.

## 12. Commits e SHA

Push em `origin/art/v1.2-protagonist` (`2a2cc85..85c83db`), sem force, merge ou rebase:

- `f645211` feat(player): add data-driven layered sprite composition
- `ca5742e` feat(visual): add n4-a modular layers of the n3.5 idle
- `531987e` test(player): validate the n4-a sprite machine
- `cf8b4b6` feat(visual): add the n4-a sprite machine proof example
- `a5c8791` docs(architecture): document the data-driven sprite composition
- `85c83dbb4bb5d54e4fac2b59ebbf9ad97bea2a1b` docs(n4): record the n4-a phase report and evidence (SHA remoto confirmado igual ao local)

O commit que grava este parágrafo é o registro da publicação; o hash remoto final é o HEAD depois do push dele. `Md-usario/` não foi commitado.

## 13. Bloqueios artísticos

1. O corpo neutro precisa de arte ou referência de qualidade final, na mesma pose e resolução, para virar base de produção.
2. O cano das botas (linha 90) é estimado.
3. Tirar só as botas deixa a canela nua sob a barra: uma calça solta até o tornozelo é uma variante que pede arte nova.
4. A roupa de baixo (cor e corte) é decisão do proprietário.

## 14. Próximos passos recomendados (não iniciados)

1. Avaliação do proprietário: `bevy_n4a_original_vs_modular.png`, `n4a_removal.png` e `N4_INTERACTIVE=1 cargo run --example n4_sprite_machine`.
2. Decidir o corpo neutro: aceitar como prova, pedir refinamento ao agente ou fornecer arte.
3. Só com a Fase A aprovada: Fase B (trocar peças, por exemplo uma camisa alternativa como variante) e depois a Fase C (corrida: começar pelos quadros #0, #2 e #4).

## Gates

### Gate A: arquitetura
- [x] Compositor orientado por dados
- [x] Reutilização de `VisualPlayback`
- [x] Camadas independentes
- [x] Sem segundo inventário
- [x] Sem segundo relógio

### Gate B: arte (**avaliação do proprietário pendente**)
- [ ] Corpo neutro coerente: proporções coerentes segundo o agente; qualidade abaixo da aprovada
- [x] Cabelo separável (tecnicamente)
- [x] Camisa separável (tecnicamente)
- [x] Calça separável (tecnicamente)
- [x] Botas separáveis (tecnicamente; cano estimado)
- [ ] Proporções preservadas: o original está preservado (0 diferenças); as proporções do corpo neutro dependem da avaliação

### Gate C: recomposição
- [x] Original comparado ao modular
- [x] Diferenças medidas (0)
- [ ] Qualidade avaliada: pelo proprietário
- [x] Nenhuma regressão artística escondida (bloqueios listados)

### Gate D: segurança
- [x] V2.2 preservado
- [x] N3.5 preservada
- [x] N3.6 preservada
- [x] N3.8 preservada
- [x] Física e combate preservados
- [x] Save v3 preservado
- [x] Testes executados
