# Último relatório

| | |
|---|---|
| **Tarefa** | Modular Character Engine 03, Equipment Studio |
| **Data** | 2026-11-21 |
| **Status** | AGUARDANDO APROVAÇÃO VISUAL. Ferramenta e provas validadas; arte das peças pendente |
| **Relatório completo** | [`docs/relatorios/modular-character-engine/03_EQUIPMENT_STUDIO.md`](modular-character-engine/03_EQUIPMENT_STUDIO.md) |
| **Branch** | `art/v1.2-protagonist` |

## Resumo

- **Equipment Studio:** editor Rust/Bevy separado do jogo (`cargo run --example equipment_studio`).
  Tem modos parte, vértice, pivô, encaixe e cor, importa PNG e salva e recarrega em RON. Mostra
  o item equipado ou isolado, em idle ou corrida, quadro a quadro e espelhado, com zoom
  inteiro e prévias em 1× e 2×.
- **Modelo genérico (`rig::design`) e pintor (`rig::paint`):** um equipamento é uma lista de
  partes independentes, cada uma com:
  - polígono com curvas ou PNG;
  - transformação e pivô;
  - material e rampa;
  - regiões e ornamentos;
  - visibilidade e perspectiva.

  O pintor desenha qualquer design direto na grade de texels de cada pose.
- **Provas:**
  - peitoral A (gótico roxo e dourado, 9 partes) e B (couro com placas assimétricas, tiras e
    fivelas, 8 partes), estruturalmente diferentes;
  - espada importada de PNG;
  - cinto criado e salvo só pelo editor (commit `350e615`, só RON) e reaberto num segundo
    processo.
- **Ferramentas externas:** o ASF renderizou a fivela (PixelSpec) e o emblema (SVG crisp), e o
  `pixel_qa` conferiu os rasters. O sombreamento é portado do pintor do pixelfx.
- **Capa articulada:** adiada para a Engine 04, com a arquitetura documentada.
- **Validação:** 58 suítes e 762 testes, 0 falhas. Clippy limpo. Os dois roteiros do editor dão
  `RESULT: PASS`, e o laboratório da 02 continua em PASS.

## Imagens principais

- `evidence/modular_character_engine/03_equipment_studio/01_editor_capture.png`
- `evidence/modular_character_engine/03_equipment_studio/05_six_poses_a_b.png`
- `evidence/modular_character_engine/03_equipment_studio/06_breastplates_compared_isolated.png`
- `evidence/modular_character_engine/03_equipment_studio/04_create_and_swap.gif`
- `evidence/modular_character_engine/03_equipment_studio/03_pivot_and_socket.gif`

## Próxima decisão

1. Avaliar a arte de A e B e decidir se o peitoral deve avançar além do braço na corrida.
2. Definir a prioridade da Engine 04: capa ou desfazer, hierarquia de partes e edição de
   regiões no editor.
3. Integração ao jogo continua não autorizada.
