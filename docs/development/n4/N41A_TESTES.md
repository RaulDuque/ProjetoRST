# N4-1-A — Testes (resultados reais)

| Comando | Resultado |
|---|---|
| `cargo fmt --check` | limpo |
| `cargo check -j 2` | limpo |
| `cargo clippy -j 2 --all-targets -- -D warnings` | limpo, depois de corrigir 6 avisos `clone` em testes (forçado a reanalisar tocando um arquivo da biblioteca) |
| `cargo test -j 2` | **55 suítes, 675 passaram, 0 falharam** (C1: 54 suítes, 660) |
| `cargo run --example n4_sprite_machine -- --equipment` | `RESULT: PASS` (5 capturas) |
| `cargo run --example n4_sprite_machine -- --equipment --run-c1` | `RESULT: PASS` (11 capturas) |
| `cargo run --example n4_sprite_machine` (idle sem equipamento) | `RESULT: PASS` |
| `python3 -B .../build_equipment.py` | 0 buracos e 0 texels ocultos descobertos nos 2 itens × 4 quadros |
| `python3 -B .../check_bevy.py` | 0 pixels diferentes em 21 capturas comparadas (3 idle, 4 corrida C1 e 14 de equipamento) |

## Testes novos

Unitários em `src/player/visual/equipment_visual.rs` (7):

| Teste | Cobre |
|---|---|
| `two_items_of_one_family_need_only_data` | família reutilizada por dois itens; OVERLAY no slot, HIDE só da camisa do tronco na corrida, mangas mantidas, sem HIDE no idle; item fora da roupa padrão |
| `replace_hides_the_base_part` | REPLACE, e a sobrescrita do item (`replaces: Some([])`) |
| `mirrored_sheets_are_used_when_the_body_is_flipped` | espelhamento com folha própria (`DrawLayer.mirrored`), mesmo quadro |
| `missing_clips_follow_the_family_rule` | fallback `Hide` e erro `Refuse` |
| `frame_order_exceptions_move_the_family_slot` | exceção de ordem por clipe e quadro |
| `family_frame_order_is_shared_and_ties_keep_the_layer_order` | exceção compartilhada pela família; desempate estável pela ordem declarada |
| `invalid_data_is_refused_with_a_descriptive_error` | slot ou clipe ausentes, família duplicada ou desconhecida, item duplicado ou com id da base, HIDE inválido (camada inexistente ou o próprio slot), REPLACE do corpo-base, folha fora da tira, versão, RON quebrado |

Integração em `tests/n41a_equipment_tests.rs` (8):

| Teste | Cobre |
|---|---|
| `every_family_and_item_loads_from_data_alone` | leitura dos manifestos reais por diretório; A herda HIDE, B sobrescreve |
| `without_equipment_the_approved_frames_are_unchanged` | compatibilidade: sem item, idle e corrida C1 iguais aos quadros aprovados texel a texel |
| `both_items_overlay_without_holes_or_exposed_texels` | sem buracos; item só sobre a silhueta (não flutua); todo texel oculto sob o item; a camada oculta não é desenhada; nada da camisa por cima do peitoral |
| `item_b_reuses_the_family_silhouette_of_item_a` | os dois dentro da máscara da família; B é um recorte da silhueta de A |
| `the_items_mirror_with_the_body_and_keep_their_frame` | espelhamento com o corpo; mesmo quadro em todas as camadas |
| `worn_items_follow_the_one_visual_playback` | ECS: com A e com B, todas as camadas visíveis no mesmo quadro e espelho; A esconde a camisa do tronco e B não |
| `a_third_item_of_the_family_is_data_only` | um terceiro item só em RON, sem código; manifesto quebrado gera erro descritivo |
| `the_save_format_and_the_approved_art_do_not_move` | save v3; hashes do `run6.png` e do idle N3.5 |

## Testes atualizados

`tests/n4_sprite_machine_tests.rs`: a lista exata de slots agora tem 15 (com `armor_torso`). A ordem do idle continua conferida, igual a antes. Nenhuma asserção foi enfraquecida.

Entre A e B não houve mudança no código de produção: o mesmo código carrega os dois itens, e o terceiro de teste, por diretório e por RON.
