# D18 — Auditoria dos consumíveis

## Estado inicial (pré-implementação)

| Área | Situação |
|---|---|
| Definições RON (`consumable: Some(RestoreHp/Mp/Hearts)`) | Existia (`minor_tonic` RestoreHp(40)) |
| Categoria `ItemCategory::Consumable` | Existia |
| Inventário (`add`/`remove`/pilhas/`max_stack`) | Existia |
| Coleta / baús / drops / comércio | Existia (tonic em baús/loja) |
| Regras puras de **uso** | Ausentes |
| Mensagens `UseItemRequest` / `ItemUsed` | Ausentes |
| Sistema Bevy de aplicação | Ausente (só equip) |
| UI inventário | Consumíveis no painel “Other”, **não selecionáveis** |
| Persistência Save V3 | Inventário/HP/MP já serializados |

## Lacuna

Dados e obtenção já cobriam o ciclo “ter a poção”. Faltava **usar** uma unidade
com efeito atômico, feedback e seleção na tela `I`.

## Decisão de escopo

Completar o uso (HP + MP), sem viagem rápida, sem arte, sem bump de save.
