# Equipamentos por parte do corpo e aparência dinâmica

> **V2.0 [Parcial]:** a direção C (híbrida) foi adotada com **camadas de
> runtime** (rastro de golpe por arma, capa atrás do corpo, tom discreto
> do tronco), sem arte nova por item e sem tocar nos atlas publicados. Os
> slots novos propostos abaixo (`Arms`, `Legs`, `Cloak`, `Amulet`) **não**
> foram criados: o enum é persistido no Save V2. Arma desenhada na mão e
> elmos dependem de sockets por quadro do pipeline. Detalhes:
> [`EQUIPMENT_EXPANSION_V2_0.md`](EQUIPMENT_EXPANSION_V2_0.md) §3, §14–15.

> **[Planejado]**, com **[Decisão pendente]** sobre a arquitetura visual.
> Nada foi implementado na V1.6.
> - **Existe [Implementado]:** 8 slots em `inventory/equipment.rs`
>   (`Weapon`, `Offhand`, `Head`, `Body`, `Gloves`, `Boots`, `Accessory1`,
>   `Accessory2`) e as categorias `Helmet`, `Armor`, `Gloves`, `Boots`,
>   `Shield`, `Ring`, `Amulet`.
> - **Sem aparência:** o sprite do protagonista é um só.

## 1. Slots

| Slot pedido | Hoje | Proposta (sem duplicar) |
|---|---|---|
| Cabeça | `Head` | Mantém |
| Peitoral | `Body` | Mantém |
| Braços | — | **Novo** `Arms` (braçadeiras). Só se houver itens que justifiquem; senão, parte de `Gloves` |
| Luvas | `Gloves` | Mantém |
| Pernas | — | **Novo** `Legs` |
| Botas | `Boots` | Mantém |
| Capa | — | **Novo** `Cloak` (afeta a silhueta: prioridade visual) |
| Arma principal | `Weapon` | Mantém |
| Arma secundária ou escudo | `Offhand` | Mantém |
| Acessórios, anéis | `Accessory1/2` | Renomear para `Ring1/2` com migração; `Accessory` continua aceito na leitura |
| Amuleto | dentro de `Accessory` | **Novo** `Amulet` |

**Compatibilidade:**
- `EquipmentSlot` é serializado no save. Slots novos entram no fim do enum.
- A renomeação de `Accessory` exige a migração do save v2 ([`WORLD_PERSISTENCE.md`](WORLD_PERSISTENCE.md)).

## 2. Arquiteturas de aparência

Números de referência:
- **Atlas atuais (V1.5):** 122 quadros de personagem, 19,3 MB na GPU (com efeitos).
- **Corpo:** ~17 MB por conjunto completo de animações.
- **Escopo:** ~15 peças visíveis por categoria no jogo final.

| Critério | A — camadas separadas | B — combinações pré-renderizadas | C — híbrido |
|---|---|---|---|
| Como | Corpo, armadura, capa e arma: um sprite por camada, mesmo quadro e pivô | Um atlas completo por combinação de equipamento | Corpo + roupa principal pré-renderizados juntos por "conjunto de roupa"; capa, arma e escudo como camadas |
| Assets | Peças × animações | Combinações × animações (explode: 15⁴ ≈ 50 mil) | Conjuntos de roupa (~10–20) × animações + camadas leves |
| Memória (estimativa) | Corpo 17 MB + ~3–8 MB por peça equipada (só as equipadas carregadas) | Impraticável além de ~5 combinações | 17 MB por conjunto de roupa carregado + 2–6 MB por camada |
| Complexidade | Alta: ordem de camadas por quadro (braço na frente/atrás), oclusão, pivôs iguais | Baixa no runtime, altíssima na produção | Média: 2–3 camadas, ordem estável |
| Animações | Toda peça precisa de todos os quadros | Todo o conjunto renderizado de novo | Roupas: todos os quadros; capa e arma: só onde mudam a silhueta |
| Pivô e direção | Mesmo pivô de sola; espelhamento igual ao atual (`anchor_for`) | Igual ao atual | Igual ao atual |
| Desempenho | 4–6 sprites por quadro (barato) | 1 sprite | 2–3 sprites |
| Blender | Renderizar cada peça isolada com máscara do corpo (holdout) | Renderizar tudo junto (pipeline atual) | Pipeline atual para o conjunto + holdout para a capa e a arma |
| Escala (centenas de itens) | Boa, se cada peça tiver arte | Não escala | Boa: a maioria dos itens muda status, não visual; só conjuntos e peças de silhueta têm arte |

**Recomendação [Proposta]: C — híbrido.**
- **Conjunto de roupa:** corpo + peitoral + pernas + botas formam um "conjunto
  de roupa" pré-renderizado, usando o pipeline atual sem mudança.
- **Camadas:** capa e arma viram camadas. São as peças que mais mudam a
  silhueta, e a espada já é um objeto separado no rig (`blade_readable`).
- **Cabeça:** elmos completos substituem a cabeça como camada.
- **Sem arte própria:** itens sem arte usam o visual do conjunto equipado,
  e o item só muda os atributos.

## 3. Integração (planejada)

1. **Dados:** `ItemDefinition.visual: Option<VisualRef>` (`outfit: "iron_plate"` ou `layer: (slot: Cloak, set: "red_cloak")`).
2. **Manifesto:** `player.ron` ganha atlas por conjunto de roupa e por camada. O guarda de publicação ([`PLAYER_ART_POLISH_V1_5.md`](PLAYER_ART_POLISH_V1_5.md)) já valida vários atlas.
3. **Bevy:** `PlayerVisual` vira um pai com filhos-camada que tocam o mesmo
   clipe e quadro (o controlador escolhe o clipe uma vez e as camadas copiam o índice).
4. **Carregamento:** só os atlas dos itens equipados (handles carregados ao equipar, liberados ao desequipar).

**[Decisão pendente]:** aprovar C antes de produzir arte de equipamento. Isso depende também da D2 (acabamento).
