# Pipeline de conteúdo do mundo

> - **[Protótipo]:** salas em RON, validador, relatório e grafo (`examples/world_report.rs`).
> - **[Planejado]:** editor visual de mapa, checagem de alcance físico, CI.
>
> Nenhuma engine, editor externo ou dependência nova foi introduzida.

## 1. Fluxo atual

```text
editar assets/world/regions/<região>.ron
        ↓
cargo run --example world_report        valida e escreve target/world_report/{report.txt, world.dot}
        ↓
cargo test --test world_graph_tests     regras + o mundo real (falha se houver erro)
        ↓
cargo run --example world_vertical_slice -- --interactive   jogar a região
```

- **Sem código Rust por sala:** criar uma sala é só escrever uma entrada `WorldRoom`.
- **Portas:**
  - sempre em pares: a porta de A aponta para a porta de B, e B aponta de volta;
  - validadas;
  - o mapa precisa que elas encostem.

## 2. O que as ferramentas detectam hoje

| Problema | Onde |
|---|---|
| Ids duplicados (sala, região, porta) | Carga |
| Portas sem destino, não recíprocas, desalinhadas, fora do lado | Validador |
| Salas sem portas, desconectadas, inalcançáveis | Validador |
| Regiões inacessíveis | Validador |
| Requisitos impossíveis (nada concede) | Validador |
| Progressão principal que depende de segredo; objetivo inalcançável | Validador |
| Sala secreta exposta; sala sem saída | Avisos |
| Entrada de porta dentro de um sólido ou no gatilho | `tests/world_graph_tests.rs` |
| Inimigo inexistente | `tests/world_graph_tests.rs` (contra o `GameData`) |
| Visualização do grafo | `world.dot` (Graphviz, abre em qualquer visualizador) |

## 3. Planejado

| Ferramenta | Para quê | Quando |
|---|---|---|
| Checagem de alcance físico | Porta alta sem plataformas ou habilidade (pulo de 136 u) | V1.8 |
| Relatório por região | Contagem de salas, % opcional/secreto, salas de save | V1.8 |
| Gerador de esqueleto | `world_report --new-room <id> --at x,y --size w,h` cria a entrada com portas pareadas | V2.2 |
| Pré-visualização de sala | Exemplo que abre uma sala isolada com o jogador e grade de células | V2.2 |
| Editor visual | Só se o volume justificar; preferência por ferramenta própria em Bevy (sem engine externa) | Avaliar na V2.2 |
| CI | `world_report` e testes em cada mudança de `assets/world/` | Junto com o CI geral (`ROADMAP`, Etapa 1) |

## 4. Convenções

- **Ids:** `<região>_<nome>` para salas; `to_<sala>`, `from_<sala>` para portas.
- **Uma região por arquivo:** salas muito grandes podem ganhar arquivos próprios no futuro.
- **Coordenadas de mapa:** absolutas no mundo (x, y em células). Uma região nova escolhe uma área livre.
- **Papéis:**
  - `Required`: só no caminho principal;
  - `Optional`: desvios;
  - `Secret`: só atrás de `hidden_by`.
