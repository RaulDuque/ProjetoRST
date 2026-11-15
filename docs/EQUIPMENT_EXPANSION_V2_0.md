# V2.0 — Armas, armaduras, equipamentos, inventário e aparência

> Briefing: `Md-usario/DarkCastleR-V2-0.md`. Estado: **implementado e testado**,
> com a aparência **parcial** (camadas de runtime; sem arte nova por item).
> Legenda: **[Implementado]**, **[Testado]**, **[Inspecionado]** (capturas
> abertas e olhadas), **[Parcial]**, **[Planejado]**.

## 1. Auditoria

Linha de base (antes de qualquer alteração): branch `art/v1.2-protagonist`,
HEAD `204f965`, 1 arquivo staged (`art-src/player/inspiration/protagonist_concept_v001.png`),
377 linhas em `git status --porcelain=v1 -uall`, 304 testes Rust e 17 Python
passando. Os hashes de `Md-usario/`, `saves/` e `.cargo/config.toml` e dos 5
assets publicados do jogador foram registrados e conferidos no fim.

Documentos pedidos que **não existem** com esse nome: `INVENTORY_EQUIPMENT.md`
(o real é `EQUIPMENT_SYSTEM.md`) e `SKILLS_SYSTEM.md` (o real é `SKILL_SYSTEM.md`).

| Item | Estado antes da V2.0 | Onde |
|---|---|---|
| Definição de item, categorias, raridades em dados | Implementado | `inventory/items.rs`, `assets/data/items/` |
| 8 slots | Implementado (sem UI) | `inventory/equipment.rs` |
| Inventário (pilhas) | Implementado | `inventory/inventory.rs` |
| Equipar/remover em jogo | **Inexistente** (só o equipamento inicial) | — |
| Modificadores de atributo, soma sem acúmulo | Implementado | `player/attributes.rs`, `CharacterSheet` |
| Categorias de arma com identidade | **Apenas compatibilidade de slot** | `ItemCategory::slots` |
| Ataque básico por arma | **Inexistente** (constantes globais) | `combat/attacks.rs` |
| Resistências elementais | Implementado, sem teto de equipamento | `magic/elements.rs` |
| Compatibilidade técnica × arma | **Inexistente** | — |
| Tela de inventário | **Inexistente** (só Status) | — |
| Aparência por equipamento | **Apenas documentado** (`EQUIPMENT_VISUAL_SYSTEM.md`) | — |
| Persistência do equipamento | Implementado (IDs no Save V2) | `save/serialization.rs` |
| Veilwoven Band (+6 MP, +0,2 MP/s) | Dados ok, **inalcançável** (não havia como equipar) | `starter_gear.ron` |
| Drops de inimigos | Implementado (itens genéricos) | `player/systems.rs::collect_rewards` |
| Arte por arma/peça | **Dependência externa** (pipeline Blender) | `art-src/player/` |
| Risco de regressão | Ataque básico, técnicas, Forma Espectral, saves, rotas | — |

## 2. Arquitetura

Separação pedida, como ficou:

| Responsabilidade | Onde | Tipo |
|---|---|---|
| Definição do item | `ItemDefinition` (+ `weapon`, `visual`) | puro, dados |
| Instância | **não existe**: itens são idênticos por ID (nada a guardar por cópia) | — |
| Inventário | `Inventory` (inalterado) | puro |
| Slot / estado equipado | `Equipment` (inalterado no formato) | puro, salvo |
| Movimentos bolsa ↔ slot e trava | `inventory/loadout.rs` (`equip_from_bag`, `unequip_to_bag`, `EquipGate`) | puro |
| Modificadores | `AttributeModifiers` somados de novo a cada mudança | puro |
| Perfil de arma | `inventory/weapons.rs` (`WeaponClass`, `WeaponStats`, `StrikeProfile`) | puro, dados |
| Perfil visual | `inventory/appearance.rs` (`ItemVisual`, `Appearance`) | puro, dados |
| Comparação | `inventory/compare.rs` | puro |
| Aplicação no ECS | `inventory/systems.rs` (`apply_equip_requests`, `sync_appearance`) | Bevy |
| Camada de capa | `inventory/visual.rs` | Bevy |
| Tela | `ui/inventory_screen.rs` | Bevy/UI |
| Persistência | `PlayerSave.equipment` (IDs), **sem mudança de schema** | puro |

Fluxo: a tela escreve `EquipRequest { slot, op }` → `apply_equip_requests`
(único escritor) confere a trava e as regras → recalcula o bônus a partir
dos slots (`set_equipment_bonus`), ajusta HP/MP máximos e o `CombatProfile`
no mesmo frame → `EquipmentChanged`. `sync_appearance` reage a
`Changed<Belongings>` (equipar, coletar, load). Nada é recalculado por frame.

Novo conteúdo entra por dados: item novo = entrada em `assets/data/items/*.ron`;
categoria de arma nova exige só uma `WeaponClass` se a categoria já existir.

## 3. Slots

Os 8 slots reais foram **preservados** (nome, ordem e serialização):

| Pedido no briefing | Slot real | Categoria(s) | Nome na tela |
|---|---|---|---|
| Arma principal | `Weapon` | Sword, Dagger, Axe, Spear, HeavyWeapon | Weapon |
| — (não pedido) | `Offhand` | Shield | Off-hand |
| Cabeça | `Head` | Helmet | Head |
| Tronco | `Body` | Armor | Body |
| Braços ou mãos | `Gloves` | Gloves | Arms |
| Pernas ou pés | `Boots` | Boots | Legs |
| Capa ou vestimenta | **não existe** → `Body` | Armor com `visual: Cape` | Body |
| Acessório 1 / 2 | `Accessory1` / `Accessory2` | Ring, Amulet | Accessory 1 / 2 |

**Diferença documentada:** não há slot de capa; há `Offhand`. Criar `Cloak`
mudaria o enum persistido, o que o briefing manda não fazer sem autorização.
Por isso o slot `Body` é também o de vestimenta: duas das quatro peças de
tronco são capas (camada visual de capa). Só os rótulos da tela mudaram
(`EquipmentSlot::display_name`); os nomes no save continuam os mesmos.

## 4. Armas

Cada categoria tem uma `WeaponClass` em `assets/data/items/weapon_classes.ron`:

| Classe | reach | height | gap | raise | attack_time | recovery | power |
|---|---|---|---|---|---|---|---|
| Sword | 46 | 26 | 8 | 4 | 0,28 | 0,00 | 1,00 |
| Dagger | 28 | 22 | 6 | 2 | 0,17 | 0,00 | 0,55 |
| Axe | 50 | 36 | 8 | 6 | 0,38 | 0,14 | 1,45 |
| Spear | 78 | 16 | 26 | 2 | 0,32 | 0,06 | 1,10 |
| HeavyWeapon | 62 | 46 | 8 | 10 | 0,52 | 0,26 | 2,10 |

- `reach`/`height`: tamanho do golpe; `gap`: espaço vazio entre o centro do
  corpo e o golpe (a lança não cobre o que está colado ao corpo); `raise`:
  altura do centro do golpe.
- A linha **Sword** é exatamente o ataque básico anterior
  (`BASIC_ATTACK_HITBOX` 46×26, `BASIC_ATTACK_TIME` 0,28, deslocamento
  `largura/2 + 8`, `4`): a Iron Sword golpeia como antes.
- Validação (`WeaponClass::validate`): categoria de arma, valores finitos,
  `reach`/`height`/`attack_time`/`power` > 0, `gap`/`recovery` ≥ 0, limites
  `MAX_REACH` 160 e `MAX_STRIKE_TIME` 1,5 s. Classe duplicada é erro.
  `GameData` exige uma classe para cada categoria de arma usada por itens.

## 5. Perfis (por item)

`ItemDefinition.weapon: Option<WeaponStats>`: `element` (padrão `Physical`)
e `reach_bonus`. Só é aceito em categorias de arma. O dano da arma continua
sendo `modifiers.attack` (ATK), como antes da V2.0; a classe multiplica.

Campos do briefing e onde ficaram: ID, nome, categoria, descrição,
raridade, valor de venda, requisitos (já existiam); dano base (`attack`),
elemento e alcance (`weapon`), multiplicador de velocidade e recuperação
(classe), perfil visual (`visual: Trail`), compatibilidade com técnicas
(na técnica, `weapons`, §7). **Não criados** por não terem uso real:
"tipo de dano" separado (o golpe básico é físico; o elemento já existe),
"escalonamento por atributo" próprio (o ATK já escala com STR).

## 6. Ataques

`StrikeProfile::resolve(classe, stats, attack_speed)`:

```text
size        = (reach + reach_bonus, height)
offset      = (size.x / 2 + gap, raise)        espelhado pela orientação
action_time = (attack_time + recovery) / attack_speed   (estado Attacking)
active_time = attack_time * HITBOX_ACTIVE_FRACTION / attack_speed
multiplier  = power;  element = stats.element
```

`combat::attacks::basic_attack` usa esse perfil (sem arma: `StrikeProfile::unarmed`,
as constantes antigas). A fórmula de dano, a defesa, os cooldowns, o MP, os
comandos, a morte e as hitboxes/hurtboxes dos inimigos **não mudaram**. O
hitbox recebe `WeaponStrike { category }` e a cor do rastro da arma.
Direção: o golpe continua horizontal à frente (comandos direcionais
inalterados); "ataques direcionais" da lança ficam [Planejado].

## 7. Técnicas

`SkillDefinition.weapons: Vec<ItemCategory>` (vazio = qualquer arma ou
nenhuma). Crescent Slash e Rising Edge: `weapons: [Sword]`. `begin_skill`
chama `execution::check_weapon` ao confirmar a ativação; com outra arma (ou
sem arma) o resultado é `CommandOutcome::WrongWeapon` e nada é gasto (MP,
cooldown, estado). O HUD diz "Crescent Slash - needs a sword". Comandos
não mudaram. Magias não têm restrição. `GameData` rejeita `weapons` com
categoria que não seja de arma.

Troca de arma: proibida durante ataque, com hitbox de golpe viva, durante
transformação e durante transição (§17, `EquipGate`).

## 8. Armaduras

Head, Body, Gloves (Arms), Boots (Legs) e Offhand usam os modificadores já
existentes: DEF, MDEF, atributos, HP/MP máx., regeneração, esquiva e
resistências. Nada novo foi preciso no cálculo; todos já eram derivados.

## 9. Resistências

Sistema único (`ElementalResistances`), sem duplicação. Novo: o teto
`StatFormulas.max_resistance` (0,75, em `character.ron`) aplicado à soma dos
equipamentos em `derive` (`ElementalResistances::capped`):

- sem imunidade nem resistência infinita (teto < 1);
- sem dano negativo nem cura involuntária (`1 − 0,75` > 0; o dano mínimo segue);
- sem acúmulo errado: a soma é refeita a partir dos slots a cada mudança;
- valores não finitos viram 0; o piso continua `MIN_RESISTANCE` (−1).

## 10. Acessórios

Os dois slots somam seus modificadores em ordem determinística
(`EquipmentSlot::ALL`). Um anel só ocupa um slot por cópia: equipar tira
uma unidade da bolsa. Com os dois ocupados, um terceiro substitui o
Acessório 1. **Veilwoven Band:** +6 MP máx. e +0,2 MP/s confirmados em
teste puro e ECS (antes da V2.0 não havia como equipá-lo). Com a Forma
Espectral, mais MP máx. significa mais segundos de forma (4 MP/s).

## 11. Atributos

| Camada | Onde | Muda com |
|---|---|---|
| Base | `BaseAttributes` | nível (automático) |
| Pontos distribuídos | `BaseAttributes` via `allocate_point` | tela Status |
| Equipamento | `CharacterSheet.equipment_bonus` | **só** `apply_equip_requests` e load |
| Temporários | não existem ainda | — |
| Finais | `DerivedStats` (sempre `derive(base, nível, bônus)`) | qualquer das acima |

Os modificadores nunca tocam a base. HP/MP: o máximo novo recorta o atual
(`ResourcePool::set_max`); um máximo maior **não** restaura nada; nunca negativo.
Requisitos de item usam os atributos **base** (um item não habilita a si mesmo).

## 12. Raridades

Common (0), Uncommon (1), Rare (2), **Epic (3, novo)**, Legendary (4). Só
classificam e colorem; não mudam atributos. O tier só ordena (não é salvo).

## 13. Catálogo

`assets/data/items/equipment_v2.ron` (22 novos) + `starter_gear.ron` (5 já
existentes) = 27 equipamentos:

| Slot | Itens |
|---|---|
| Weapon | Iron Sword, Duskwater Saber (Ice); Gutter Knife, Wren's Needle; Hearthsplitter, Grave Hatchet (Dark); Watch Pike, Lantern Glaive (Light); Bell Maul, Ossuary Greatblade (Epic, STR 10) |
| Head | Sentry Coif, Vigil Helm |
| Body | Leather Jerkin, Chain Hauberk (tint); Pilgrim's Mantle, Emberlined Cloak (capas) |
| Arms | Wrapped Gauntlets, Scribe's Bracers |
| Legs | Warden Greaves, Quiet Soles |
| Off-hand | Watch Buckler, Rune Ward |
| Acessórios | Ashen Ring, Veilwoven Band, Still Water Amulet, Lantern Talisman, Bloodstone Pendant |

Resistências cobertas: Fogo, Gelo, Raio, Água, Vento, Terra, Trevas e
Arcano (Luz aparece como elemento de arma). Todos os nomes são originais.

## 14. Aparência

O sprite publicado (atlas V1.5) é um corpo renderizado **com a espada
embutida** e sem sockets por quadro (mão, cabeça). Gerar camadas por peça
exigiria renderizar ~122 quadros por item no Blender e publicar pelo guarda,
mexendo nos assets protegidos. Por isso a V2.0 entrega o modelo híbrido com
**camadas de runtime**, sem nenhuma imagem nova em disco:

| Mudança visual | Como | Peças | Estado |
|---|---|---|---|
| Rastro do golpe (arma) | cor do item + forma da classe (largura, altura, espaço) | as 10 armas | [Implementado][Inspecionado] |
| Capa | sprite filho atrás do corpo, textura gerada em memória e tingida, balança com a velocidade | Pilgrim's Mantle, Emberlined Cloak | [Implementado][Inspecionado] |
| Tom do tronco | multiplicação discreta da cor do sprite | Chain Hauberk | [Implementado][Inspecionado] |
| Arma desenhada na mão | — | — | [Parcial]: o sprite sempre mostra a espada inicial |
| Elmo / braços / pernas | infraestrutura (`ItemVisual::Tint` aceita Head) | — | [Planejado] (precisa de socket de cabeça do pipeline) |

**Por isso nenhuma arma além das espadas é declarada visualmente concluída:**
o golpe, o alcance e o ritmo mudam e se veem, mas o corpo continua
segurando a espada. As animações aprovadas da espada não foram trocadas.

## 15. Camadas

- Ordem (z local do jogador): caixa de reserva 0 → **capa 0,03** → sprite
  0,05 → efeitos. Halo e contorno da forma ficam atrás, como na V1.9.
- Pivô: a capa pende dos ombros (`SHOULDER_Y` 13 acima do centro do corpo),
  `BEHIND_X` 5 para o lado das costas; agachar desce 12.
- Espelhamento: a capa troca de lado e `flip_x` com `FacingDirection`. O
  problema da V1.4 (mão da espada trocada ao espelhar) não aumenta: nenhuma
  camada depende da mão; o rastro é simétrico.
- Oculta durante esquiva (rolamento) e morte.
- Validação dos dados: `Trail` só em armas, `Cape` só em `Armor` (comprimento
  12–44), `Tint` só em `Armor`/`Helmet` com canais ≥ 0,45 (nunca escurece demais).
- Escala: uma textura de capa para qualquer número de capas; zero texturas por arma.

## 16. Inventário

Tecla **`I`** (estava livre; auditado em `INPUT_SYSTEM.md`). Abre
`GameState::Inventory` e pausa a simulação como a tela Status (Tab não age
dentro dela). `↑`/`↓` selecionam, `Enter` equipa o item da bolsa (no
primeiro slot livre da categoria) ou remove o do slot, `I`/`Esc` fecham.
Dentro da tela as teclas de combate não são lidas (o `GameSet::Input` não
roda pausado), então não há conflito.

Mostra os 8 slots ocupados ou vazios, os equipamentos da bolsa (cor da
raridade; vermelho com "(cannot equip)" quando um requisito falta), os
outros itens, a categoria, a raridade, a descrição, o perfil de golpe, as
técnicas compatíveis, os requisitos e o motivo de uma troca bloqueada.

## 17. Comparação

`inventory::compare` calcula o antes e o depois com `StatFormulas::derive` e
`ItemCatalog::strike_profile`, as mesmas chamadas do jogo. Linhas: poder do
golpe (`ATK × multiplicador`), alcance, tempo do golpe, ATK, DEF, MATK, MDEF,
HP/MP máx., STR/CON/INT/LCK, CRIT, DODGE, regenerações e 9 resistências. A
tela mostra só as que mudam. Teste: `the_comparison_reports_the_formulas_own_numbers`.

**Trava (`EquipGate`)**: equipar/remover só em locomoção (parado, andando,
correndo, agachado, pulando, caindo). Recusado: caído, atacando,
conjurando, esquivando, ferido, defendendo, com golpe vivo, transformado
(ativo ou terminando) ou em transição de sala. Recusas deixam bolsa e
slots intactos e avisam no HUD. Validação: item existe, está na bolsa, é
equipamento, slot certo, requisitos, sem duplicar (uma unidade por slot),
item anterior volta à bolsa, slot com item desconhecido (save antigo) não é trocado.

## 18. Save/load

**Save V2 preservado, sem mudança de schema.** O equipamento já era salvo
como IDs por slot (`PlayerSave.equipment`) e a bolsa como pilhas. No load,
o bônus é recalculado dos slots, a aparência é refeita (`Changed<Belongings>`)
e o golpe vem dos dados. Nenhum `Entity` é salvo. Itens desconhecidos num
slot são ignorados no cálculo e preservados. Saves v1 (fixture
`save_v1_entry_hall.ron`) carregam com o equipamento intacto. Nenhum teste
grava em `saves/`.

## 19. Transformações

- Bônus passivos continuam valendo na forma (mais MP máx. = forma mais longa).
- A capa é recolorida e fica translúcida com o corpo (`tint_form` agora
  inclui `CapeLayer`); nenhuma camada opaca fica sobre o corpo espectral.
- Equipar/remover é recusado com a forma ativa ou terminando; o perfil de
  colisão espectral (`Phasing`) não muda (teste).
- Rota real: forma ativada com lança + capa, tentativa de troca recusada,
  volta ao humano, F5, outra sala, F9.

## 20. Baús

Três baús novos nos Portões da Vigília, sem substituir recompensas:
`vg_approach_chest` (Watch Pike, Common), `vg_causeway_chest` (Emberlined
Cloak, Rare) e `vg_barbican_chest` (Bloodstone Pendant, Common). Os demais
itens do catálogo ficam para os próximos conteúdos (e na bancada visual).
Inimigos: o sistema de drops já aceita qualquer item (equipamento incluído);
**nenhuma tabela de drop foi alterada** (uma entrada nova consumiria
números aleatórios e mudaria o combate determinístico das rotas aprovadas).

## 21. Balanceamento

Números reais do código (personagem inicial: STR 6 → ATK base 12; DPS
teórico = poder ÷ tempo comprometido, contra DEF 0; contra o Bone Sentry,
DEF 4, multiplique por 0,96):

| Item | Classe | ATK | Poder/golpe | vs Sentry | Tempo (s) | Golpes/s | DPS teórico | Cobertura | Altura | Golpe ativo (s) | Recuperação (s) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `iron_sword` | Sword | 20 | 20,0 | 19,2 | 0,28 | 3,57 | 71 | 8–54 | 26 | 0,168 | 0,00 |
| `duskwater_saber` | Sword | 23 | 23,0 | 22,1 | 0,28 | 3,57 | 82 | 8–58 | 26 | 0,168 | 0,00 |
| `gutter_knife` | Dagger | 18 | 9,9 | 9,5 | 0,17 | 5,88 | 58 | 6–34 | 22 | 0,102 | 0,00 |
| `wrens_needle` | Dagger | 20 | 11,0 | 10,6 | 0,17 | 5,88 | 65 | 6–34 | 22 | 0,102 | 0,00 |
| `hearthsplitter` | Axe | 22 | 31,9 | 30,7 | 0,52 | 1,92 | 61 | 8–58 | 36 | 0,228 | 0,14 |
| `grave_hatchet` | Axe | 25 | 36,2 | 34,9 | 0,52 | 1,92 | 70 | 8–58 | 36 | 0,228 | 0,14 |
| `watch_pike` | Spear | 21 | 23,1 | 22,2 | 0,38 | 2,63 | 61 | 26–104 | 16 | 0,192 | 0,06 |
| `lantern_glaive` | Spear | 24 | 26,4 | 25,4 | 0,38 | 2,63 | 69 | 26–112 | 16 | 0,192 | 0,06 |
| `bell_maul` | Heavy | 26 | 54,6 | 52,5 | 0,78 | 1,28 | 70 | 8–70 | 46 | 0,312 | 0,26 |
| `ossuary_greatblade` | Heavy | 32 | 67,2 | 64,6 | 0,78 | 1,28 | 86 | 8–70 | 46 | 0,312 | 0,26 |

Leitura (não só DPS):
- **Espada:** referência, técnicas de espada, melhor equilíbrio.
- **Adaga:** menor DPS, mas o menor comprometimento (0,17 s) e crítico/sorte
  nos itens; alcance curto (34) obriga a encostar. Não supera tudo.
- **Machado:** golpe alto e largo (36), 0,14 s de recuperação = janela de risco.
- **Lança:** o maior alcance (104–112), mas não acerta a menos de 26 do
  centro e é baixa (16): perigosa quando o inimigo cola.
- **Pesada:** um golpe tira quase o HP de um Sentry (60), porém 0,78 s
  comprometido e 0,26 s de recuperação.
- Nenhuma categoria tem acesso às técnicas de espada.
- Os inimigos não foram mexidos. Valores provisórios, ajustáveis só em dados.

## 22. Testes

| Arquivo | Testes | Cobre (itens do §29 do briefing) |
|---|---|---|
| `src/inventory/weapons.rs` (unit) | 3 | 15–18, 26 |
| `tests/equipment_tests.rs` | 17 | 1–19, 21–23, 25, 26 |
| `tests/equipment_app_tests.rs` | 8 | 5–8, 11, 12, 14–22, 24, 25, perf |
| `tests/world_equipment_route_tests.rs` | 1 | rota real (§30) |

Testes antigos: só dois literais de `SkillDefinition` em `command_parser_tests.rs`
e `app_smoke_tests.rs` ganharam `weapons: Vec::new()` (campo novo; nenhuma
asserção mudou).

Rota real (`tests/support/equipment_route.rs`, `examples/world_equipment_route.rs`):
início → inventário → baú da aproximação (Watch Pike) → tela, comparação,
equipar → golpe de 78 → Sentry derrotado com a lança → baú da calçada
(capa) → equipar → capa visível → rota V1.7 até a relíquia → Forma
Espectral com o equipamento → troca recusada → humano → F5 → Postern,
troca para a espada → F9 → lança, capa, ATK e aparência restaurados → golpe
de 78 de novo. **9 trocas de sala, 0 reposicionamentos artificiais**, só teclas.

Bancada visual (`examples/equipment_showcase.rs`): **concede** os itens
(não é rota) e captura as 5 categorias, as capas, o tom, o inventário, a
comparação e a forma; capturas em `target/equipment_showcase/`.

## 23. Performance

`equipment_app_tests::equipment_timings_are_measured` (headless, perfil dev,
`target/tmp/equipment_v20_perf.txt`):

| Medida | Valor |
|---|---|
| Frame comum | 0,34 ms |
| Frame com equipar | 0,37 ms |
| Frame com remover | 0,35 ms |
| Recalcular atributos (puro) | 0,16 µs |
| Frame de abrir o inventário | 0,86 ms |
| Frame com o inventário aberto | 0,24 ms |
| Entidades visuais extras | 1 (a capa) |
| Texturas novas | 1 (capa, 24×64, em memória) |

Nada é recalculado por frame: o bônus só muda em `apply_equip_requests` e
no load; a aparência só com `Changed<Belongings>`. Memória e save/load
medidos nas execuções com janela (relatório final).

## 24. Riscos

- **Aparência parcial:** o corpo sempre segura a espada; elmo e luvas não aparecem.
- **Sem slot de capa:** capas competem com armaduras pelo `Body`.
- **Heavy/Spear de duas mãos:** resolvido na V2.1 (`WeaponClass.two_handed`,
  `loadout::check_hands`; ver `NPC_COMMERCE_V2_1.md` §18).
- **Teto de resistência** vale só para o personagem (inimigos inalterados).
- **Saves históricos reais** não foram lidos (proibido); a compatibilidade foi
  provada com a fixture v1 e saves gerados nos testes.
- **Drops de equipamento** preparados, mas nenhum inimigo os concede ainda.
- O estado "transição" no `EquipGate` só é observável enquanto a trava de
  porta está armada; o teste a mantém num frame.

## 25. Próximas etapas

1. Sockets de mão/cabeça exportados pelo pipeline de arte (JSON por quadro)
   para desenhar a arma equipada e elmos como camadas reais.
2. Animações por categoria (estocada da lança, golpe pesado) no pipeline
   seguro, em área isolada, com aprovação.
3. Decidir slot `Cloak` (exige migração do Save V2 → V3, com autorização).
4. Distribuir o resto do catálogo em baús, segredos e drops.
5. Regra de duas mãos para pesadas/lanças.
6. Técnicas próprias por categoria (ex.: investida da lança).
