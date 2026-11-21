# Último relatório

| | |
|---|---|
| **Tarefa** | Modular Character Engine 02, equipamentos independentes |
| **Data** | 2026-11-21 |
| **Status** | AGUARDANDO APROVAÇÃO VISUAL. Parte técnica validada; arte e integração dependem do proprietário |
| **Relatório completo** | [`docs/relatorios/modular-character-engine/02_EQUIPAMENTOS.md`](modular-character-engine/02_EQUIPAMENTOS.md) |
| **Branch** | `art/v1.2-protagonist` |

## Resumo

- **Esqueleto:** marcos refinados com os marcos da N4-C (vídeo em alta resolução) e medições
  nos quadros #1, #3 e #5, mais 4 juntas da caixa do tronco.
  - Tremor do ombro: 1,81 → 1,00 texel.
  - Tremor do ângulo da espada: 47,8° → 5,8°.
- **Empunhadura:** perfis reutilizáveis, independentes do antebraço:
  - idle: diagonal para baixo, acima do chão;
  - corrida: levada para trás, passando atrás do corpo quando o braço recua;
  - espelhamento e troca sem reiniciar.
- **Camadas:** mapas da N4-C nos quadros #0, #2 e #4 e regras portadas da N4-C nos demais. A
  recomposição continua idêntica (0 texels diferentes).
- **Contrato:** 10 slots, peças rígidas, articuladas e flexíveis, e montagem em osso, mão ou
  tronco.
- **Peitoral:** um único modelo nas 7 poses, comparando rígida × malha × projetada. A
  projetada foi escolhida e passou no `pixel_qa` em todas as poses.
- **Ferramentas:** ASF e pixelfx executados de fato. O `rig_animate.py` confere o nosso FK com
  erro médio de 0,16 texel.
- **Laboratório:** interativo, com andar, liga e desliga de cada peça e camadas uma a uma.
- **Validação:** 57 suítes e 741 testes, 0 falhas. Clippy limpo. O laboratório dá
  `RESULT: PASS` com 131 capturas.

## Imagens principais

- `evidence/modular_character_engine/02_equipamentos/05_combo_sword_pauldron_breastplate.gif`
- `evidence/modular_character_engine/02_equipamentos/03_breastplate_approaches.png`
- `evidence/modular_character_engine/02_equipamentos/01_run_sword_grip.gif`
- `evidence/modular_character_engine/02_equipamentos/02_idle_sword.gif`
- `evidence/modular_character_engine/02_equipamentos/04_swap_during_run.gif`
- `evidence/modular_character_engine/02_equipamentos/08_lab_interactive.png`

## Próxima decisão

1. Aprovar a empunhadura e a abordagem projetada do peitoral.
2. Definir a arte definitiva das peças.
3. Autorizar a integração experimental ao jogo. O laboratório continua separado até lá.
