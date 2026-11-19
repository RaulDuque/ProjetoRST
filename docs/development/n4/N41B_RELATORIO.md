# N41-B — Relatório: fábrica automatizada de equipamentos

A fábrica existe e funciona tecnicamente: equipamentos definidos por dados (templates, materiais, detalhes, perfis), gerados de forma determinística em PNG reais sobre a grade de texels, validados, com proveniência e cache. Os itens de tronco entram no compositor existente pelo loader existente, e o Bevy os desenha igual, pixel a pixel, ao que os dados descrevem. **A qualidade artística não está aprovada**: os itens são coerentes com a paleta e o contorno do protagonista, mas abaixo do acabamento N3.5/N3.6 (Gate G com o proprietário). Nada foi publicado: commits **só locais**, sem push.

## 1. Estado inicial
Branch `art/v1.2-protagonist`, local = remoto = `4cb0ac6`, árvore limpa exceto `Md-usario/` (não rastreado, preservado). Auditoria completa em `N41B_ARQUITETURA.md` §Auditoria.

## 2. Arquitetura reutilizada
`VisualFamily` `torso_plate` (slot `armor_torso`, máscaras, ordem do idle, HIDE da corrida), `EquipmentVisual` + `equip`, `resolve_facing` (OVERLAY/HIDE/REPLACE), `CompositeCharacter`/`sync_composite_layers`, `VisualPlayback`, `build_equipment.family_masks/base_layers/draw_order`, o exemplo `n4_sprite_machine`.

## 3. Arquitetura implementada
Biblioteca JSON (templates, materiais, detalhes, perfis), receitas, gerador `scripts/factory.py`, relatórios `scripts/factory_report.py`, modo `--factory` do exemplo (roteiro de capturas a partir dos dados, contagem de camadas via `resolve_facing`). **Nenhum arquivo de `src/` mudou.** Sem segundo renderizador, inventário ou relógio.

## 4. Templates criados (14)
Tronco: `segmented_cuirass`, `ridged_breastplate`, `brigandine_coat`, `scale_hauberk`. Lâminas: `arming_blade`, `estoc_blade`, `broad_falchion`, `flame_blade`. Guarda/cabo: `cross_hilt`, `crescent_hilt`, `ring_hilt`. Capa/ombro: `long_cape` (flexível), `plate_pauldron` (rígida), `fur_mantle` (acessório).

## 5. Materiais criados (17)
`iron`, `steel`, `blackened_steel`, `bronze`, `gold`, `silver`, `hardened_leather`, `dark_leather`, `wood`, `crimson_cloth`, `royal_blue_cloth`, `ash_grey_cloth`, `moss_green_cloth`, `wolf_fur`, `arcane_violet`, `ember_red` e `gilded_steel` (este pela prova de automação). Todos com `outline`/`shadow`/`base`/`highlight` (+ `deep`, `light`, `specular` opcional) e contorno colorido.

## 6. Detalhes criados (16)
Carimbos `rivet`, `boss_rivet`, `buckle`, `emblem_wolf`, `emblem_cross`, `emblem_sun`, `emblem_tower` (prova de automação), `gem`, `rune_mark`, `fuller_runes`; frisos `trim_band`, `double_trim`, `embroidery_band`; linhas `stitch_line`, `seam_line`; desgaste `wear_scratches`. Quatro operações genéricas, nenhuma por item.

## 7. Mapas de relevo
Um mapa de **altura** (0..255) por template e clipe em `stage/factory/relief/` (16 PNG), gerado pelas operações de relevo do template. Não é mapa de normais; a normal vem do gradiente. Evidência: `evidence/n41b/n41b_relief_maps.png`.

## 8. Processo de geração
Receita → validação → forma (máscara da família, polígono ou perfil de lâmina) → altura → luz (frente-alto) → rampa do material com pontilhado pela rugosidade → contorno seletivo → detalhes no interior → PNG RGBA + TXT + RON (tronco) + proveniência. `python3 -B art-src/n4-sprite-machine/scripts/factory.py [--rebuild] [--only id ...]`.

## 9. Processo de validação
Dados: esquema, ids, referências, slots, operações e posicionamento (14 casos inválidos testados). Saídas: `factory_report.py` separa **erro técnico** de **aviso artístico** (`qa.json`). Integração: `tests/n41b_factory_tests.rs` e a checagem pixel a pixel das capturas do Bevy.

## 10. Determinismo
Seeds explícitas, RNG local por texto, ordem estável, hash da receita, das entradas e das saídas, versão `n41b-1`. Duas gerações do zero: **204 arquivos idênticos** entre si e iguais ao stage.

## 11. Receitas
**42** válidas (40 da prova: 16 tronco, 12 espadas, 4 capas, 4 ombreiras, 4 manteletes; + 2 da prova de automação). 0 recusadas.

## 12. PNGs
**42 itens com PNG** (18 de tronco com folha idle — 4 também com a corrida C1 — e prévia; 24 isolados), mais 16 mapas de altura. 42/42 tecnicamente válidos; 29 com aviso artístico.

## 13. Candidatos distintos
**12** (as espadas: 4 lâminas × 3 guardas).

## 14. Variações menores
**0** (< 8% de pixels).

## 15. Duplicatas
**0** exatas. **30 itens** classificados `REVISAR_VISUALMENTE`: mesma silhueta do template, outro material e outros detalhes (recolorações com ornamentos). Entre templates a silhueta difere (tronco 6–26%, espadas 17–37%).

## 16. Integrados ao idle
**18** (todos os itens de tronco), conferidos no Bevy.

## 17. Integrados à corrida C1
**4** (a família `segmented_cuirass` nos 4 materiais; quadros #0, #2, #4). Espada na corrida: **não** (soquete não validado).

## 18. Recursos isolados
**24**: 12 espadas, 4 capas, 4 ombreiras, 4 manteletes.

## 19. Problemas artísticos
Tronco estreito (a peça cobre só a faixa entre os braços; no idle duas faixas ao redor da manga), desgaste ruidoso no ferro, ombreira isolada sem correia, gola do mantelete parecendo textura, falcão com curva discreta, ornamentos grandes não cabem no idle, recolorações dentro do template. Detalhe em `N41B_QUALIDADE.md`.

## 20. Trabalho manual necessário
Item novo: 1 JSON. Material/detalhe novo: 1 JSON. Template novo: 1 JSON. Ainda manual: revisão artística, soquetes da mão/ombro/costas, separar a manga da camisa do idle, polígonos de pose (C2) e, se o proprietário quiser, pintura final sobre as máscaras.

## 21. Alterações de código
Novos: `art-src/n4-sprite-machine/scripts/factory.py`, `factory_report.py`, `scripts/tests/test_factory.py`, `tests/n41b_factory_tests.rs`. Alterado: `examples/n4_sprite_machine.rs` (modo `--factory`, roteiro pelos dados, `Shot.item: Option<String>`, E percorre os itens carregados). `src/`: nenhuma mudança.

## 22. Testes executados
`cargo fmt --check` ok; `cargo check` ok; `cargo clippy --all-targets -j 2 -- -D warnings` limpo; `cargo test -j 2`: **56 suítes, 680 passaram, 0 falharam** (5 novos); `python3 -B -m unittest discover -s art-src/n4-sprite-machine/scripts/tests`: **13 passaram**; `tutorial_examples_tests` ok; exemplo `--factory` e `--factory --run-c1`: `RESULT: PASS`; 36/36 capturas com 0 pixels diferentes.

## 23. Evidências
`docs/development/n4/evidence/n41b/` (lista e descrição em `N41B_QUALIDADE.md` §Evidências): catálogo, templates, materiais, detalhes, mapas de altura, idle, oclusão, espelhamento, corrida C1 (PNG + GIF), diversidade, prova de automação, 36 capturas do Bevy e cinco relatórios JSON.

## 24. Commits locais
- `f58e2fd` feat(visual): add the data-driven equipment factory
- `251a681` feat(visual): generate the n41-b factory catalogue
- `63ddca7` feat(visual): add the factory mode to the sprite machine proof
- `7af3f98` test(player): validate the generated factory equipment
- `31f45d5` feat(data): add two factory items made from data only (**só dados**: nenhum `.rs`/`.py`)
- `4bde1d2` feat(visual): add factory quality, diversity and evidence reports
- e os commits de documentação que gravam este relatório.

## 25. Estado do Git
Branch `art/v1.2-protagonist` à frente de `origin/art/v1.2-protagonist`, sem push. Não rastreados: só os arquivos pessoais de `Md-usario/`.

## 26. Bloqueios
Espada no personagem: soquete da mão sem confirmação e sem ângulo/frente. Capa e ombreira: sem camada `back`/`shoulder` na composição e sem soquete no formato. HIDE no idle: camisa e manga do idle numa camada só.

## 27. Decisões pendentes
1. Avaliação artística (Gate G): `n41b_catalog.png`, `n41b_torso_templates.png`, `n41b_run_c1_family.gif`, `n41b_spotlight.png`, modo interativo `N4_INTERACTIVE=1 cargo run --example n4_sprite_machine -- --factory [--run-c1]` (tecla E).
2. Confirmar os soquetes da mão (posição, ângulo, frente) para integrar uma espada na C1.
3. Autorizar camadas/soquetes de ombro e costas para capas e ombreiras (extensão do formato).
4. Separar a manga da camisa no idle (melhora muito a leitura do tronco).
5. Arte final: refinar o gerador ou pintar à mão sobre as máscaras.
6. Publicação (push) desta etapa.

## 28. Push
**Nenhum push foi feito.** Sem force, merge ou rebase. `Md-usario/` não foi alterado nem commitado.

## Gates

### Gate A — Arquitetura
- [x] Fábrica orientada por dados
- [x] Reutilização do compositor
- [x] Sem segundo relógio
- [x] Sem segundo inventário
- [x] Contratos validados

### Gate B — Pixel art
- [x] Mapas de relevo (altura)
- [x] Quatro tons mínimos (validados; ≥ 4 tons usados conferidos em teste)
- [x] Sel-outline (sem preto puro)
- [x] Grade 1:1 (cores só das paletas declaradas, alfa binário)
- [x] Sem blur
- [x] Sem escala fracionária
- [ ] Sem preenchimento plano inadequado: **parcial** — no idle a couraça segmentada tem 41–46% numa cor (aviso)

### Gate C — Geração
- [x] Templates reutilizáveis
- [x] Materiais reutilizáveis
- [x] Detalhes reutilizáveis
- [x] Seeds determinísticas
- [x] Geração em lote (cache, recusa de sobrescrita, falhas registradas)

### Gate D — Escala
- [x] 40 receitas processadas (+2 da prova de automação)
- [x] Recursos gráficos reais
- [x] Duplicatas identificadas (0; 30 recolorações sinalizadas)
- [x] Diversidade medida
- [x] Falhas documentadas

### Gate E — Integração
- [x] Idle validado (18 itens, Bevy 0 diferença)
- [x] Corrida C1 limitada (1 família, #0/#2/#4)
- [x] Oclusão validada (ordem das camadas; teste + `n41b_occlusion.png`)
- [x] Espada integrada somente com soquete confiável (não integrada: soquete não confiável)
- [x] Sem reconstrução anatômica

### Gate F — Segurança
- [x] V2.2 preservado
- [x] N3.5/N3.6 preservadas (hashes)
- [x] N3.8 preservada
- [x] N4 preservada
- [x] N4-1 preservada (itens A e B e seus testes intactos)
- [x] Save v3 preservado
- [x] Física e combate preservados (nenhum `src/` alterado)
- [x] Testes executados

### Gate G — Aprovação artística
**Pendente até avaliação explícita do proprietário.**
