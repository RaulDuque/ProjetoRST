# Último relatório

| | |
|---|---|
| **Tarefa** | Modular Character Engine 04, Equipment Studio V2 |
| **Data** | 2026-11-21 |
| **Status** | AGUARDANDO APROVAÇÃO VISUAL. Técnica validada; arte das peças pendente |
| **Relatório completo** | [`docs/relatorios/modular-character-engine/04_STUDIO_V2.md`](modular-character-engine/04_STUDIO_V2.md) |
| **Branch** | `art/v1.2-protagonist` |

## Resumo

- **Hierarquia de partes:** pai, grupo, transformações local e global, visibilidade e
  profundidade herdadas, vínculo de uma raiz a outro osso; ciclos recusados.
- **Encaixe:** corpo em 8 regiões, com o peitoral atrás dos braços, a ombreira sob o cabelo e
  o braçal sob a mão. As peças podem substituir regiões. Uma composição única alimenta todas
  as vistas.
- **Editor V2** (`cargo run --example equipment_studio`):
  - painéis em Bevy UI;
  - mouse: selecionar, mover, girar, vértices, pivô;
  - pincel/borracha/balde/conta-gotas para pixels;
  - material novo e rampa;
  - regiões, ornamentos e visibilidade;
  - undo/redo (Ctrl+Z, Ctrl+Y, Ctrl+Shift+Z);
  - proteção contra perda de alterações.
- **Conjunto gótico** em 4 peças independentes (peitoral, ombreira, braçais, faldas),
  combinável com o gibão de couro.
- **Validação:**
  - 780 testes;
  - clippy limpo;
  - `--tour` (72 passos) e `--tour-reload` com PASS numa cópia isolada;
  - laboratório com PASS.

## Próxima decisão

Aprovar ou reprovar a arte e o encaixe do conjunto gótico (evidências em
`evidence/modular_character_engine/04_studio_v2/`).
