# Último relatório

| | |
|---|---|
| **Tarefa** | Modular Character Engine 01, primeiro protótipo |
| **Data** | 2026-11-21 |
| **Status** | AGUARDANDO APROVAÇÃO VISUAL. Parte técnica validada; arte e abordagem dependem do proprietário |
| **Relatório completo** | [`docs/relatorios/modular-character-engine/01_PROTOTIPO.md`](modular-character-engine/01_PROTOTIPO.md) |
| **Branch** | `art/v1.2-protagonist` |

## Resumo

- **O que foi feito:** um laboratório Rust/Bevy isolado (`cargo run --example
  modular_character_engine`) com um esqueleto 2D. São 15 marcos e 12 ossos, com FK e IK de dois
  ossos, marcados à mão no idle N3.5 e nos 6 quadros da corrida N3.6. O jogo não muda.
- **Como funciona:** cada quadro aprovado é cortado em 5 camadas de profundidade, e a
  recomposição dá 0 texels diferentes. A espada e a ombreira, desenhadas uma vez, ficam presas
  aos ossos. A troca A/B e o equipar e desequipar acontecem durante a corrida sem reiniciar o
  quadro, e o espelhamento funciona.
- **Comparação:** a versão totalmente articulada (peças do idle + FK/IK) fica bem abaixo da
  arte aprovada:
  - juntas abertas;
  - cabelo caído;
  - perspectiva fixa;
  - texels girados.

  A proposta é a abordagem **híbrida**: corpo aprovado + equipamento preso a ossos.
- **Validação:** fmt, clippy `-D warnings`, `cargo test` (57 suítes, 718 testes, 0 falhas) e
  check passaram, e o exemplo deu `RESULT: PASS`.
- **Ressalvas:**
  - no idle a espada aponta para a frente (falta osso de pulso);
  - os marcos são estimados (cerca de ±2 texels);
  - as ombreiras foram reduzidas da N41-B e são provisórias.

## Imagens principais

- `evidence/modular_character_engine/01_prototipo/06_comparison_original_hybrid_articulated.gif`
- `evidence/modular_character_engine/01_prototipo/04_swap_during_run.gif`
- `evidence/modular_character_engine/01_prototipo/05_turn_around.gif`
- `evidence/modular_character_engine/01_prototipo/08_depth_layers.png`
- `evidence/modular_character_engine/01_prototipo/01_skeleton_overlay.png`

## Próxima decisão

1. Aprovar ou não a abordagem híbrida.
2. Definir como a espada deve ser empunhada no idle e na corrida.
3. Aceitar as peças provisórias ou pedir arte própria.
4. Autorizar a integração experimental ao jogador.

Nenhuma armadura foi produzida em quantidade.
