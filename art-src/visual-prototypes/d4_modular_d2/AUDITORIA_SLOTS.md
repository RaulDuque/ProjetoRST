# Auditoria — slots, sockets e impacto

## Slots funcionais atuais (`EquipmentSlot`)

| Slot | Label UI | Visual hoje (V2.2) |
|---|---|---|
| Weapon | Weapon | `HeldLayer::Weapon` (silhueta procedural) |
| Offhand | Off-hand | `HeldLayer::Shield` se escudo |
| Head | Head | só `Tint` (sem sprite) |
| Body | Body | `Tint` ou `Cape` (capa procedural 24×64) |
| Gloves | Arms | sem sprite |
| Boots | Legs | sem sprite |
| Accessory1/2 | Accessory | sem sprite |

Fonte: `src/inventory/equipment.rs`, `appearance.rs`, `docs/VISUAL_EQUIPMENT_V2_2.md`.

## Sockets por quadro (`player_sockets.ron`)

Pontos: `weapon`, `hand_l`, `head`, `chest`, `back` (u a partir da sola).
Medidos no Blender v007; corpo publicado **sem sabre** (`weapon_in_body: false`).

### Limitações dos sockets atuais

- Não há socket de “óculos”, “elmo separado”, “calça”, “bota”.
- `head`/`chest`/`back` existem, mas Head/Gloves/Boots **não** têm arte de camada.
- Capa usa só `back` + textura gerada — não é a capa D2 detalhada.
- Armas são 5 formas procedurais, não sprites D2.

## Slots visuais propostos (D4) ≠ slots de gameplay

| Camada visual | Mapeamento sugerido | Novo slot funcional? |
|---|---|---|
| Corpo-base | sempre | Não |
| Cabelo | parte do body / futuro Head hair | Não (ou sub-slot cosmético) |
| Cabeça/rosto | base | Não |
| Chapéu / elmo | `Head` visual | Não (já existe Head) |
| Óculos / máscara | `Head` ou Accessory visual | Decisão pendente |
| Peitoral / casaco | `Body` | Não |
| Braços / luvas | `Gloves` | Não |
| Calças | `Boots` hoje é “Legs” — **ambíguo** | Possível renome/clarificar depois |
| Botas | parte de Legs ou subcamada | Decisão pendente |
| Capa | `Body` Cape **ou** slot visual Cloak | Docs já discutem Cloak |
| Arma principal | `Weapon` | Não |
| Escudo / secundária | `Offhand` | Não |
| FX de item | Trail / efeito | Não |

**Não** foram criados slots de gameplay nesta fase.

## Animações afetadas

Qualquer peça parentada a ossos/sockets precisa de arte **por quadro** (ou
deformação) em: idle (12), run (10), attack (7), e demais 22 clips se for
produção completa. Overlays estáticos só funcionam em idle.

## Impacto futuro (não implementado)

| Área | Impacto |
|---|---|
| Inventário UI | Ícones ok; preview visual precisaria das camadas |
| Equipamento | Dados `ItemVisual` teriam de crescer além de Tint/Cape/Trail |
| Save V3 | Hoje salva IDs de item nos slots — **compatível** se só visual mudar; breaking se novos slots funcionais forem adicionados |
| Combate | Nenhum, se hitboxes continuarem independentes da arte |

## Preservação

D1, D2, D3 e `assets/` não foram alterados.
