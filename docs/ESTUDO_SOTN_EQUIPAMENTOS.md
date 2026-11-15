# Estudo SOTN — armas e equipamentos

Fase 2 da pesquisa. Convenções: [`REFERENCIAS_SOTN.md`](REFERENCIAS_SOTN.md).
Caminhos `src/...`/`include/...` do **sotn-decomp @ `3a0957b`**; os nossos, do
**DCR @ `9f7616d`**. Legenda: **[F]** fato · **[I]** inferência · **[R]**
recomendação · **[?]** pendente.

Complementa [`ESTUDO_SOTN_COMBATE.md`](ESTUDO_SOTN_COMBATE.md) §1.2–1.4, 1.16–1.18
e [`ESTUDO_SOTN_ATRIBUTOS.md`](ESTUDO_SOTN_ATRIBUTOS.md) §4, §10.

---

## 0. Resumo

1. **Os valores por arma não estão no repositório.** A tabela
   `g_EquipDefs` (`src/config_us.h:27-29`) inclui
   `dra/gen/<versão>/equipment.h`, gerado **a partir do disco do usuário**
   na compilação (`src/config_jp.h:7`). O clone só tem o formato
   (`tools/splat_ext/equipment_config.json`). Nada há para copiar, e isso
   está alinhado com a regra de não usar dados do jogo. **[F]**
2. **Duas mãos confirmadas:** equipar uma arma de duas mãos grava o mesmo
   item **nas duas mãos** (`src/dra/menu.c:3301-3310`). **[F]**
3. **Escudo, parcialmente confirmado:** não existe "redução de dano por
   guarda" no código do jogador. O escudo é uma **entidade com hitbox** à
   frente do corpo, com propriedades de arma; o bloqueio acontece por
   **contato de hitboxes** e pela reação de cada inimigo. **[F/I]**
4. O item de mão define **toda a sensação do golpe**: família, animação,
   ponto de liberação, limite de instâncias, atordoamento, intervalo de
   reacerto, tipo de hitbox, efeito, crítico, MP. **[F]**
5. O DCR tem um modelo por **classe de arma** (`WeaponClass`) mais um
   **ajuste por item** (`WeaponStats`). A recomendação é **acrescentar campos
   de tempo e reação na classe**, com padrões neutros (plano V2.4), e **não**
   migrar para dados por item.

---

## 1. Campos do item de mão e seu significado

`Equipment` (`include/game.h:1568-1597`). Campos `unk` são **[I]**.

| Campo | Significado | Onde é usado | Equivalente no DCR |
|---|---|---|---|
| `attack`, `defense` | ATK/DEF somados aos atributos | `CalcAttack`/`CalcDefense` (`menu.c:262-419`) | `AttributeModifiers.attack/defense` |
| `element` | Bits de elemento do golpe | `SetWeaponProperties` → `attackElement` (`src/weapon/shared.h:96`) | `WeaponStats.element` |
| `itemCategory` | Categoria (`ItemCategory`, `game.h:1041-1054`): espada curta, espada, arremesso, punho, clava, **duas mãos**, comida, bomba, arremesso 2, **escudo**, remédio | Menu, ataque, cálculo de ATK | `ItemCategory` + `WeaponClass` |
| `weaponId` | Qual pacote de código de arma (`src/weapon/w_NNN.c`) desenha e controla o golpe | Fábrica de entidades | Silhueta por categoria (`held_art.rs`) |
| `unk11` | **Família do golpe** (`step_s = 0x40 + família`) | `func_8010EDB8` (`6E42C.c:563-839`) | `WeaponClass` |
| `playerAnim` | Animação do corpo | Idem | Clipe `attack` |
| `lockDuration` | **Ponto de liberação** (pose a partir da qual o jogador volta a agir) | `g_Player.unk54` (Fase 1, §1.4) | **Ausente** (`attack_time + recovery` travados) |
| `chainLimit` | Instâncias simultâneas; bit `0x80` = uma por vez até liberar | `CheckChainLimit` (`6E42C.c:334-339`) | **Ausente** |
| `unk17` | Golpe alternativo por comando ←→ | `6E42C.c:485-494` | Técnicas por comando |
| `specialMove` | Arte da arma por ↓↘→ | `6E42C.c:464-480` | Técnicas (`skills/`) filtradas por categoria |
| `comboMain`/`comboSub` | Sequências de comando ligadas à arma **[I]** | — | `CommandSequence` das técnicas |
| `mpUsage` | MP por uso (armas especiais) | `HasEnoughMp` | Custo MP das técnicas |
| `enemyInvincibilityFrames` | **Intervalo de reacerto** do mesmo alvo | → `nFramesInvincibility` (`shared.h:98`) → `unk6D[]` do alvo | **Ausente** (`already_hit`: uma vez por hitbox) |
| `stunFrames` | **Atordoamento** do alvo | → `stunFrames` (`shared.h:99`, `collision.h:560-562`) | **Constante** (`ENEMY_HURT_FLASH`) |
| `hitType` | **Tipo da hitbox** (bits de com quem colide). `4` = colide sem dano | → `hitboxState` (`shared.h:97`, `:108-110`); dano zerado (`collision.h:456-458`) | `Team` + `pierce` |
| `hitEffect` | Efeito visual/sonoro e reação (ex.: metal) | → `hitEffect` (`shared.h:100`); `collision.h:316-326` | Partículas genéricas |
| `criticalRate` | Taxa de crítico do item | `GetEquipProperties` (`5D5BC.c:530-548`) | Crítico só por LCK |
| `isConsumable` | Item de mão gasto ao usar (comida, remédio, bomba) | Menu e ataque | Consumíveis no inventário (fora das mãos) |
| `palette`, `icon`, `iconPalette`, `unk13`, `unk14` | Arte e fábrica de entidades | — | `visual` do item |

**Itens de corpo** (`Accessory`, `game.h:1599-1615`): `attBonus`,
`defBonus`, `statsBonus[4]`, conjuntos de fraqueza/resistência/imunidade/
absorção, `equipType`. Equivale a `AttributeModifiers` +
`ElementalResistances` no DCR.

---

## 2. Os itens pedidos

### 2.1 Categorias de armas

- **SOTN [F]:** 11 categorias (`game.h:1041-1054`). Comida, remédio e bomba
  também são "armas de mão". O comportamento real vem da família (`unk11`)
  e do pacote de código (`weaponId`).
- **DCR:** Sword, Dagger, Axe, Spear, HeavyWeapon (+ Shield como off-hand),
  `weapon_classes.ron`.
- **Classificação:** Implementado adequadamente.

### 2.2 Alcance

- **SOTN [F]:** a caixa vem da tabela por quadro do pacote da arma (exemplo
  do escudo: `src/weapon/w_008.c:60-65`).
- **DCR:** `reach`, `height`, `gap`, `raise` por classe + `reach_bonus` por
  item (`src/inventory/weapons.rs:34-67`).
- **Classificação:** Implementado adequadamente. A forma por fase continua
  opcional (Fase 1, §1.7).

### 2.3 Velocidade

- **SOTN [F]:** duração da animação + ponto de liberação. Não há atributo
  de "velocidade de ataque".
- **DCR:** `attack_time` por classe ÷ `attack_speed` derivado
  (`items.rs:339-348`).
- **Classificação:** Implementado adequadamente.

### 2.4 Temporização e 2.5 Ponto de cancelamento

- **SOTN [F]:** golpe ativo só nos quadros com caixa; liberação em
  `lockDuration`.
- **DCR:** ativo nos primeiros 60% desde o quadro 0; tudo travado até o fim.
- **Classificação:** Implementado, mas melhorável → **V2.4**
  ([`PLANO_V2_4_COMBAT_FEEL.md`](PLANO_V2_4_COMBAT_FEEL.md) etapas 1–2).

### 2.6 Atordoamento e 2.7 Knockback

- **SOTN [F]:** `stunFrames` por arma; o deslocamento é decidido por cada
  inimigo ao ler `hitFlags`/`hitParams`.
- **DCR:** constantes globais.
- **Classificação:** Implementado, mas melhorável → **V2.4** etapa 5.

### 2.8 Intervalo de reacerto

- **SOTN [F]:** `enemyInvincibilityFrames` por arma, contado **por atacante**
  no alvo (`unk6D[slot]`) e propagado às partes ligadas
  (`collision.h:551-570`).
- **DCR:** `already_hit` — cada hitbox acerta cada alvo uma vez.
- **Classificação:** Parcial → **V2.4** etapa 6.

### 2.9 Propriedades especiais

- **SOTN [F]:** muitos efeitos são **checagens por id de item no código**
  (ex.: `ITEM_SHIELD_ROD` com escudo, `menu.c:305-306, 352-354`; itens
  especiais em `CalcAttack`).
- **DCR:** `special_effects: Vec<String>` em `ItemDefinition`
  (`items.rs:135`). **Nenhum sistema lê esse campo.** **[F]**
- **Classificação:** Parcial (previsto, sem efeito).
- **[R]** Quando houver o primeiro efeito, criar um **enum** de efeitos
  (regra 9 do projeto: tipagem forte), não strings soltas, e um único
  sistema que os aplica. Não fazer checagens por id.

### 2.10 Equipamentos de duas mãos

- **SOTN [F]:** categoria `ITEM_TWOHAND`. Equipar escreve o item nas duas
  mãos e devolve ao inventário o que estava na outra (se não for de duas
  mãos); desequipar limpa as duas (`menu.c:3209-3216` na prévia,
  `:3301-3310` na confirmação).
- **DCR:** `two_handed: true` em Spear e HeavyWeapon impede escudo (V2.1);
  visual: escudo vai para as costas se um save editado tiver os dois
  (`VISUAL_EQUIPMENT_V2_2.md` §7).
- **Classificação:** Implementado adequadamente. **Pendência da Fase 1
  resolvida: confirmada por código.**

### 2.11 Restrições de mão esquerda e direita

- **SOTN [F]:** as duas mãos são **simétricas**: cada uma recebe qualquer
  item de mão, e cada botão usa a sua (`g_HandId`; `w_008.c:86-90`). Duas
  armas = dois golpes independentes.
- **DCR:** `Weapon` + `Offhand` (escudo) — assimétrico.
- **Classificação:** Não recomendado mudar (decisão de design do DCR;
  armas duplas exigiriam um segundo botão de ataque).

### 2.12 Escudos e 2.13 Bloqueio

- **SOTN [F]:**
  - **Segurar** o botão da mão mantém a pose; soltar encerra
    (`w_008.c:91-95`). `g_Player.unk48 = 1` mantém o jogador "em pé com
    escudo" (`w_008.c:131`, `71830.c:264, 322`).
  - O escudo é uma entidade com caixa estreita (4×12) à frente do corpo,
    trocada por quadro (`w_008.c:60-65, 205-207`). Recebe as propriedades
    do item (`SetWeaponProperties`, `w_008.c:132`).
  - Ao sofrer dano (`PLAYER_STATUS_UNK10000`), o escudo **cai** girando e
    some (`w_008.c:101-113, 188-203`).
  - Escudos com ATK 1 **não contam** como ataque (`menu.c:272-275`).
  - Na colisão, entidades do lado do jogador (armas, escudos, subarmas)
    são testadas contra inimigos e projéteis; o contato liga
    `hitFlags`/`hitParams` do inimigo (`collision.h:258-331`). **Nenhum**
    caminho de dano do jogador lê a pose de escudo (`unk48` não aparece em
    `692E8.c`, `5D5BC.c` nem `collision.h`).
- **[I]** O bloqueio de projéteis vem de cada projétil reagir ao contato
  (muitos se destroem quando `hitFlags` liga). Golpes corpo a corpo que
  alcançam o corpo além da caixa do escudo **acertam**.
- **Pendência da Fase 1: parcialmente confirmada** (mecanismo de contato
  confirmado; reação de cada inimigo não verificada).
- **DCR:** guarda (tecla A) reduz 70% do dano frontal, sem stagger
  (`player/systems.rs:156-160`); funciona **sem escudo**; o escudo equipado
  dá DEF e resistências.
- **Classificação:** Implementado, mas melhorável (modelos diferentes; o do
  DCR é mais legível para o jogador).
- **[R]** Manter a guarda por regra. Opcional depois do V2.4:
  `guard_reduction` vindo do escudo equipado (padrão = constante atual) e
  **projéteis frontais anulados** em guarda com escudo. Ambos por dados.

### 2.14 Interação com ataques

- **SOTN [F]:** o tipo da hitbox (`hitType` → `hitboxState`) decide com quem
  ela colide; `4` colide sem causar dano (`collision.h:456-458`); metal faz
  "clang" e reduz a reação (`collision.h:316-326`).
- **DCR:** `Team` (Player/Enemy) e `pierce`.
- **[R]** Nenhuma mudança agora. "Clang" em inimigo blindado pode vir com
  a armadura de chefes ([`ESTUDO_SOTN_CHEFES.md`](ESTUDO_SOTN_CHEFES.md) §6).

### 2.15 Modificadores e 2.16 Efeitos elementais

- **SOTN [F]:** bônus de atributo nos itens de corpo e pesos ocultos (Fase 1,
  Atributos §2); elemento do golpe por bits.
- **DCR:** `AttributeModifiers`, `WeaponStats.element` (um elemento),
  resistências contínuas.
- **Classificação:** Implementado adequadamente.

### 2.17 Equipamentos raros

- **SOTN [F]:** drops raros por inimigo marcam um bit próprio
  (`src/st/e_collect.h:468-471`); acessórios aumentam a chance (Fase 1,
  Atributos §12).
- **DCR:** raridades em dados (`rarities.ron`, tiers 0–4), drops com LCK,
  baús `Rare`/`Secret`/`BossReward`.
- **Classificação:** Implementado adequadamente.

---

## 3. Comparação com as armas visuais da V2.2

| Aspecto | SOTN | DCR (V2.2) |
|---|---|---|
| Quem desenha a arma | Pacote de código por arma (`w_NNN.c`), entidade própria | Camada de runtime sobre o corpo sem arma, por âncoras medidas (`player_sockets.ron`) |
| Hitbox × arte | A caixa vem da **mesma tabela por quadro** do pacote | A caixa vem de `WeaponClass`; a arte segue as âncoras. **São independentes** |
| Silhueta | Uma por item | Uma por categoria (`held_art.rs`) |
| Escudo | Entidade, cai ao sofrer dano | Camada no braço esquerdo, não cai |

**Consequência para o V2.4 [I]:** como a arte é **provisória**, as janelas de
golpe (antecipação, ativo, cancelamento) devem ficar em **segundos, nos
dados da classe**, e não ligadas a quadros da arte atual. Quando a arte final
chegar, os números são reajustados nos dados, sem mudar código. Ver
[`ESTUDO_SOTN_ANIMACOES.md`](ESTUDO_SOTN_ANIMACOES.md) §4.

---

## 4. Campos propostos para `WeaponClass` (todos opcionais)

| Campo | Padrão (= comportamento atual) | Inspiração | Versão |
|---|---|---|---|
| `startup` | `0.0` | Quadros antes da caixa | V2.4 |
| `active_fraction` | `0.6` (`HITBOX_ACTIVE_FRACTION`) | Quadros com caixa | V2.4 |
| `cancel_after` | `None` = fim da ação | `lockDuration` | V2.4 |
| `stun` | `None` = `ENEMY_HURT_FLASH` | `stunFrames` | V2.4 |
| `knockback` | `None` = `ENEMY_KNOCKBACK` | Reação por `hitFlags` | V2.4 |
| `rehit_interval` | `None` = um acerto por hitbox | `enemyInvincibilityFrames` | V2.4 |
| `crit_bonus` | `0.0` | `criticalRate` | Posterior |
| `lunge` | `0.0` | Estocada com avanço | Posterior |
| `max_active` | `None` | `chainLimit` | Posterior (armas de arremesso) |

Nenhum valor numérico do SOTN deve ser usado. Os números do DCR saem de
testes de sensação e ficam registrados em `DECISIONS.md`.

---

## 5. Pendências

- Reação de inimigos específicos ao contato com escudo: não verificada. **[?]**
- Significado exato de `unk13`, `unk14`, `unk1B` e `comboMain/Sub`: não
  identificado pela comunidade. **[?]**
- Tabelas numéricas por item: **não presentes no repositório** (geradas do
  disco). Não serão obtidas.
