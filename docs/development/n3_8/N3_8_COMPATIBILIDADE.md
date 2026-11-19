# N3.8 — Compatibilidade com equipamentos

## O que existe hoje (V2.0–V2.2, inalterado)

| Mecanismo | Arquivo | Como funciona |
|---|---|---|
| `VisualPlayback` | `src/player/visual/mod.rs` | clipe, índice e tempo do quadro mostrado; todas as camadas leem dele |
| `HeldLayer` (arma e escudo) | `src/inventory/held_visual.rs` | filhos do jogador, posicionados pelo soquete do quadro mostrado; ligados só com `weapon_in_body: false` **e** tabela de soquetes válida |
| `WeaponForm` | `src/inventory/held.rs` | 5 famílias (Sword, Dagger, Axe, Spear, Heavy); arte de silhueta gerada em código (`held_art.rs`) |
| Soquetes | `src/player/visual/sockets.rs` + `assets/animations/player_sockets.ron` | por quadro: `weapon` (punho + ângulo + frente/trás), `hand_l`, `head`, `chest`, `back` |
| `CapeLayer` | `src/inventory/visual.rs` | capa procedural do item de corpo; pendura do soquete `back` quando há soquetes, senão de um ombro fixo (V2.0) |
| Tom de equipamento | `EquipmentAppearance.tint` | multiplica a cor do corpo |
| Slots | `src/inventory/equipment.rs` | Weapon, Offhand, Head, Body, Gloves, Boots, Accessory1, Accessory2 |

## No modo experimental

| Peça | Comportamento | Por quê |
|---|---|---|
| Arma e escudo | **Desligados** (não desenhados) | não há soquetes medidos para os quadros N3. Desenhar com os soquetes do V2.2 poria a espada no lugar errado, e coordenadas inventadas são proibidas. É o fallback seguro do próprio runtime: sem tabela, `held_layers_enabled()` é falso |
| Item equipado | Continua equipado; dano e defesa iguais | o teste `equipment_is_kept_while_its_layers_stay_off_without_sockets` confirma a arma inicial em `HeldEquipment`; o gameplay é idêntico ao da produção |
| Capa (`emberlined_cloak`) | Desenhada no ombro fixo da V2.0 | sem soquete `back`, a capa usa o ponto antigo. Ela fica atrás do corpo e balança com a velocidade, mas o encaixe no ombro N3 não foi medido. `n38_run_with_cape.gif` |
| Tom de equipamento | Aplicado ao corpo N3 | mesmo caminho da produção |

**Na produção nada mudou:** a tabela de soquetes, as camadas e a capa continuam iguais (testes `held_visual_tests.rs` e `data_tests.rs` passando).

## O que está assado na arte N3 e o que é independente

| Parte | Corrida N3.6 / idle N3.5 |
|---|---|
| Cabelo | **assado** (até 318 texels, boa parte atrás do tronco e do braço de trás) |
| Rosto, pescoço, mãos, antebraços | **assados** (mangas arregaçadas: a pele dos antebraços é visível) |
| Camisa cinza | **assada** |
| Braceletes | **assados** (couro `L`/`l`/`d`) |
| Calça escura | **assada** |
| Botas altas | **assadas** |
| Cinto | **assado** |
| Arma | ausente (`weapon_in_body: false`) → seria camada independente |
| Escudo, capa | ausentes → seriam camadas independentes |

Hoje **só arma, escudo e capa são independentes**, e só no V2.2. No novo protagonista, tudo o que se vê está numa imagem só. Remover a camisa, a calça ou o cabelo **não é possível sem arte nova** (`N3_8_RISCOS_ARTISTICOS.md`).

## Limites para a N4

- Medir soquetes nos 6 quadros: a medição automática (`art-src/n3-8-production-run/scripts/measure_regions.py`) **não basta**. Os braceletes e o contorno partem a pele de cada braço em 2 ou 3 pedaços, e não dá para saber pelos pixels qual braço está na frente. Os valores em `analysis/regions.json` são só ordem de grandeza: punho dianteiro entre x ≈ 9 e 13 u, y ≈ 31 a 39 u; cabeça perto de (6, 46) u. A N4 precisa marcar à mão ou na fonte em camadas.
- Equipamento assimétrico espelhado: a arma troca de mão no desenho ao virar. O V2.2 aceita isso (soquete espelhado). A N4 decide se mantém ou se usa "mão de dentro / mão de fora".
- Capa: no V2.2 o soquete `back` resolve; no N3 o cabelo longo passa por cima das costas, então a capa precisa de uma ordem cabelo-capa explícita (`N3_8_ARQUITETURA_TERRARIA.md`).
