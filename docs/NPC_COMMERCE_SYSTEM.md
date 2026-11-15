# NPCs, comércio e economia

> **V2.1 [Implementado, parcial]:** NPCs, diálogos com escolhas e condições,
> lojas com compra e venda, a moeda "Marks" e a Vila das Cinzas existem. Os
> nomes e formatos reais diferem da proposta abaixo (que fica como histórico).
> As moedas e o estoque ainda não estão no save (aguardam autorização).
> Detalhes: [`NPC_COMMERCE_V2_1.md`](NPC_COMMERCE_V2_1.md).

> **[Planejado]**. Nada implementado na V1.6.
> - **Pré-requisito:** a **interação** (tecla de usar), que ainda não existe
>   ([`ROADMAP.md`](ROADMAP.md), Etapa 2).
> - **Já existe:** `ItemDefinition.sell_value`, o inventário com pilhas, `Notification` e o HUD.

## 1. Vauvrel (cidade) [Proposta]

| NPC | Serviço | Desbloqueio |
|---|---|---|
| Comerciante (Maren) | Compra e venda de consumíveis e equipamento básico | Início |
| Ferreira (Odile) | Melhoria de armas (+1 a +5 com materiais) | Entregar o "Martelo da Vigília" |
| Alquimista (Ivo) | Poções; transforma materiais | Destilaria Rubra visitada |
| Arquivista (Selene) | Revela comandos de técnicas (`command_revealed_on_learn` já existe) | Arquivo Velado visitado |
| Batedor (Corvin) | Vende cartas de região (mapa) | 3 regiões visitadas |
| Curandeira (Agna) | Restaura HP/MP; cura estados | Início |
| NPCs de missão | Missões | Por missão |

Os nomes são provisórios e originais.

## 2. NPC (formato proposto)

```ron
(
    id: "maren_merchant",
    name: "Maren",
    room: "vauvrel_market",
    position: (640.0, 63.0),
    appears_if: Some(Flag("vauvrel_reached")),      // mesmo `Requirement` do mundo
    dialogue: "maren",                               // assets/data/dialogue/maren.ron
    services: [Shop("maren_stock"), Talk],
)
```

- **Diálogo:**
  - uma lista de falas com `requires: Option<Requirement>`, das mais específicas para as gerais;
  - a primeira atendida é dita;
  - efeitos opcionais (`set_flag`, `start_quest`).
  - Sem árvore narrativa nem IA nesta etapa.
- **Estado persistente:** só flags em `WorldProgress.flags` (que já é salvo).

## 3. Comércio

```ron
(
    id: "maren_stock",
    currency: "ash_marks",
    entries: [
        (item: "minor_tonic", price: 30, stock: Unlimited),
        (item: "iron_sword", price: 120, stock: Limited(1)),
        (item: "silver_saber", price: 900, stock: Limited(1), requires: Some(Flag("vigil_gates_cleared"))),
    ],
    buy_back_ratio: 0.5,                // preço de venda = sell_value; recompra a 50 %
    restock: OnRest,                    // Never | OnRest | AfterBosses(n)
)
```

| Opção | Avaliação | Proposta |
|---|---|---|
| Economia fixa (preços constantes) | Previsível, fácil de balancear | **Base** |
| Estoque limitado | Torna itens raros escolhas reais | Para equipamento |
| Reposição | Evita bloqueio por falta de consumível | `OnRest` só para consumíveis |
| Itens raros | Recompensa exploração e progresso | Liberados por flags e chefes |
| Preços condicionais (reputação, desconto) | Complexidade extra | **[Planejado]**, só com facções |
| Melhorias de equipamento | Dá uso ao dinheiro e aos materiais | Ferreira, V2.1 |

- **Moeda [Proposta]:** "Marcas de Cinza" (`ash_marks`), um contador no
  inventário (não é item em pilha) e um campo novo do save v2.
- **Fontes de moeda:** inimigos (com variância, seed) e baús.
- **Validação:** um teste de dados confere se todo `item` de loja existe e se
  `price` > `sell_value` (nada dá lucro com compra e venda).
