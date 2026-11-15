# Estudo SOTN — atributos, fórmulas e progressão

Referências e convenções: [`REFERENCIAS_SOTN.md`](REFERENCIAS_SOTN.md).
Arquivos `src/...` de evidência são do **sotn-decomp @ `3a0957b`**. Os nossos
(**DCR** @ `9f7616d`): `src/player/attributes.rs`, `src/combat/damage.rs`,
`assets/data/character.ron`, `docs/CHARACTER_ATTRIBUTES.md`,
`docs/COMBAT_SYSTEM.md`, `docs/PROGRESSION_SYSTEM.md`.

Legenda: **[F]** fato confirmado no código · **[I]** inferência · **[R]**
recomendação · **[?]** pendente. "rand a..b" = inteiro aleatório uniforme.

> **Regra deste estudo:** nenhuma fórmula do DCR deve ser substituída
> automaticamente. Os itens [R] são oportunidades de balanceamento; cada uma
> precisa de decisão do proprietário e registro em `DECISIONS.md`.

---

## 1. Visão geral das duas filosofias

| Aspecto | SOTN | DCR |
|---|---|---|
| Fonte do dano físico | **Arma** (valor de ATK do item) + STR | **STR** (ATK = STR·2) + bônus do item, × potência da classe |
| Defesa | **Subtrativa**: dano − DEF, mínimo 1 | **Proporcional**: × 100/(100+DEF) |
| Elementos | Bits; vários por golpe; fraqueza ×2, resistência ÷2, imune, absorve | Um por golpe; resistência contínua em [−1, 1], teto 0,75 no equipamento |
| Crítico | Taxa da arma + pouco de LCK; efeito depende do **maior atributo sorteado** | Chance e multiplicador derivados de LCK |
| Precisão/esquiva | Não há erro de golpe (exceto acessório de esquiva) **[I]** | HIT vs DODGE, mínimo 50% |
| Variância | Só em magia/subarma (`rand 0..11 / 10`) e crítico | ±10% em todo golpe |
| Level up | Ganho aleatório de atributos (moeda por atributo) e de HP/MP pequenos | Automático fixo + pontos livres |
| EXP | Escala pela **diferença de nível** inimigo × jogador | Fixa por inimigo |

Consequência prática **[I]**: no SOTN a progressão de dano vem quase toda
de **trocar de arma**; os atributos mexem pouco. No DCR, atributos e itens
pesam parecido. Nenhuma das duas está "errada"; a nossa dá mais sentido aos
pontos distribuídos.

---

## 2. Atributos primários (STR, CON, INT, LCK)

**SOTN [F]** — `UpdateEquipStatBonuses` (`src/dra/menu.c:176-260`):

```
equip[i] = Σ bônus dos 5 itens de corpo + pedras do dia/noite (+5) + buffs (+20)
          + relíquias (+10) + forma especial (LCK +30), limitado a 99
total[STR] = base + equip
total[CON] = base + equip · 8        ← peso 8 para o bônus de equipamento
total[INT] = base + equip · 4        ← peso 4
total[LCK] = base + equip
mínimo 0
```

O menu mostra `base + equip` (`menu.c:1238, 1273`), então o peso ×8/×4 é
**interno** **[F]**. Onde `total[CON]` e `total[INT]` são usados: DEF (√CON),
duração de status (CON), dano de magia e subarma (INT), duração de
transformação (INT) — ver abaixo.

**DCR:** `BaseAttributes` + `AttributeModifiers` somados sem peso; todas as
derivadas em `attributes.rs`, coeficientes em `character.ron → formulas`.
Inicial: STR 6, CON 6, INT 6, LCK 4.

**[R]** Não adotar pesos ocultos (pouco transparentes para o jogador).

---

## 3. HP, MP, Hearts

**SOTN [F]:**

- Level up (`CheckAndDoLevelUp`, `src/dra/5D5BC.c:308-362`):
  `HPmáx += tabela[nível/10]` com tabela `{1, 3, 6, 10, 20, 30, 40, 50, 100, 200}`
  (`:10`); `MPmáx += 4 + rand 0..1`; `Heartsmáx += 2`. O HP atual sobe junto.
- Itens de aumento: HP máx, Hearts máx (`func_800FE044`, `:373-409`), limite 9999.
- MP gasto por magia (`CastSpell`), por armas especiais (`mpUsage`) e
  continuamente por transformações (`HandleTransformationMP`, `:790-843`).
- Hearts gastos por subarma (`func_800FE3C4`, `:482-505`; custo ÷2 / ÷3 com
  1 ou 2 acessórios de redução, mínimo 1); um capa converte dano recebido em
  Hearts (`:745-749`).

**DCR:** `HP = 50 + CON·10 + L·5 + mod`, `MP = 20 + INT·5 + L·2 + mod`,
`Hearts = 99`; regeneração por CON/INT. Exemplo inicial: HP 115, MP 52.

**Comparação [I]:** no SOTN o HP por nível é mínimo até o nível 10 (+1) e
cresce muito no fim; o grosso vem de itens de exploração. No DCR o HP cresce
linearmente com nível e CON.

**[R]** Opcional: **itens de exploração que aumentam HP/MP máx** (recompensa
de segredo) via `AttributeModifiers` permanentes ou flag de progresso. É
mecânica de Metroidvania, não fórmula. Prioridade média (ver plano).

---

## 4. Ataque físico

**SOTN [F]** — `CalcAttack` (`menu.c:262-322`), por mão:

```
atk = arma.attack
atk += STR            se arma.attack ≤ STR
atk += STR / 2        se arma.attack > STR
atk += Σ attBonus dos itens de corpo
(+ casos especiais de itens; +20 com buff; limitado a [0, 999])
veneno: atk / 2       (GetEquipProperties, 5D5BC.c:555-557)
```

Exemplos (variáveis: `arma.attack`, `STR`):

| arma | STR | ATK |
|---|---|---|
| 12 | 15 | 12 + 15 = **27** |
| 30 | 15 | 30 + 7 = **37** |
| 30 | 40 | 30 + 40 = **70** |

**Inferência [I]:** a regra "STR inteira só enquanto a arma é mais fraca que
STR" cria **retorno decrescente** para armas muito acima do personagem.

**DCR:** `ATK = STR·2 + mod.attack`; golpe = `ATK · potência_da_classe`
(Sword 1,0; Dagger 0,55; Axe 1,45; Spear 1,1; Heavy 2,1).
Inicial com Iron Sword: STR 6 → 12 + bônus da espada.

**[R]** Nenhuma mudança obrigatória. Se armas raras ficarem fortes demais no
começo, considerar um **teto suave** inspirado no SOTN (bônus da arma
completo até um limiar dependente de STR, metade acima). Fórmula própria,
testada em `tests/attributes_tests.rs`.

---

## 5. Defesa

**SOTN [F]** — `CalcDefense` (`menu.c:334-419`):

```
DEF = Σ defense das duas mãos + Σ defBonus dos itens de corpo
    + floor(√ total[CON]) − 2
    (+ itens especiais; +20 com buff; limitado a [0, 999])
```

Exemplos: CON 9 → +1; CON 16 → +2; CON 100 → +8. A defesa vem quase toda do
equipamento.

Dano recebido (`HandleDamage`, `5D5BC.c:593-759`):

```
bruto = ataque do inimigo (já com elemento: fraqueza ×2, resistência ÷2,
        imune → 0, absorve → cura)
tomado = bruto − DEF            (veneno: ×2)
se tomado ≤ 0:
    DEF ≥ 100 → toma 1, SEM recuo (só pisca 12 quadros)
    senão     → toma 1, recuo leve
```

Exemplo: inimigo 20 de ataque, DEF 8 → 12 de dano; DEF 25 → 1 (recuo leve);
DEF 120 → 1 sem recuo.

**DCR:** `DEF = CON + mod.defense`; `mitigado = bruto · 100/(100+DEF)`
(100 DEF = metade). Exemplo: 20 de dano, DEF 8 → 18,5 → 17–20 com variância.

**Comparação [I]:** a defesa subtrativa do SOTN torna inimigos fracos
irrelevantes rapidamente (dano 1) e inimigos fortes ainda perigosos; a nossa
é suave e nunca anula. A ideia aproveitável não é a fórmula, e sim o efeito
**"golpe fraco não derruba"**.

**[R]** Limiar de recuo (*poise*): golpe com dano final < `stagger_threshold`
(fração do HP máx, ligado aos tiers de [`ESTUDO_SOTN_COMBATE.md`](ESTUDO_SOTN_COMBATE.md) §1.11)
não causa `Hurt`. Mantém a nossa fórmula de dano intacta.

---

## 6. Ataque mágico

**SOTN [F]:** `ataque_da_magia = base_da_magia + (total[INT]·2 + rand 0..11) / 10`
(`GetSpellDef`, `5D5BC.c:200-206`); uma armadura dá +50%. Subarma: mesma
soma com INT (`:525`). Duração da transformação de "sangue" + INT·4
(`GetStatusAilmentTimer`, `:240-243`).

Exemplo: base 30, INT 20 → 30 + (40 + 0..11)/10 = 34 ou 35.

**DCR:** `MATK = INT·2 + mod`; magia = `MATK · multiplicador + dano_base`,
mitigada por MDEF. INT pesa muito mais no DCR.

**[R]** Nenhuma mudança. Registrar que, no DCR, INT é atributo principal de
build de magia (escolha nossa, coerente com pontos livres).

---

## 7. Resistências, fraquezas e dano elemental

**SOTN [F]:**

- Elementos são **bits** (`include/game.h:315-334`): HIT, CUT, POISON,
  CURSE, STONE, WATER, DARK, HOLY, ICE, THUNDER, FIRE (+ alguns `UNK`).
- No inimigo (`DealDamage`, `5D5BC.c:870-976`): se o golpe tem qualquer
  elemento "mágico" (bits ≥ 0x80), só esses contam; senão só os físicos.
  **Imune/absorve/resistente** exigem que **todos** os elementos do golpe
  estejam no conjunto; **fraqueza** basta **um**. Sem elemento → HIT.
- No jogador (`HandleDamage`): fraqueza ×2, resistência ÷2, imune 0
  (pedra tratada à parte), absorver cura.
- Fontes de resistência: itens de corpo (`CalcDefense`), escudos
  específicos, relíquias, poções temporárias (`menu.c:359-401`).

**DCR:** um elemento por golpe (10 elementos + Physical); resistência
contínua `[−1, 1]` (−0,5 = 150%, 0,5 = metade, 1 = imune); equipamento
limitado a 0,75; inimigos podem ser imunes.

**[R]** Manter o modelo contínuo (mais granular). **Não** migrar para bits
agora (mudança de schema e de save). Ideia futura de baixo custo: golpe com
**elemento secundário** opcional (ex.: espada flamejante = Physical + Fire),
resolvido como "o melhor/pior multiplicador" — exige decisão e testes.

---

## 8. Críticos

**SOTN [F]:**

- Taxa (0–255, sobre 256) calculada ao equipar/golpear
  (`GetEquipProperties`, `5D5BC.c:530-548`):
  `taxa = arma.criticalRate + floor(√(2·LCK + rand 0..15)) − 5`, limitada a [0, 255]
  (zero no prólogo).
- Teste: `taxa > rand 0..255` (`DealDamage`, `:925`).
- **Efeito depende do atributo "sorteado"**: sorteia `base[i] + rand 0..63`
  para STR, CON, INT, LCK e usa o maior (`:926-959`):
  STR → dano ×2; CON → + DEF_do_inimigo/2 (ou +50% se o golpe já supera a DEF);
  INT → + √(salas exploradas); LCK → + rand 0..LCK−1.
- Depois do crítico, ainda subtrai a DEF do inimigo.

Exemplos de taxa (LCK sem itens): LCK 7 → √(14..29) = 3..5 → −2..0;
LCK 50 → +5; LCK 99 → +9. LCK soma no máximo ~3,5 pontos percentuais.

**DCR:** `chance = 0,05 + LCK·0,005 (+mod)` em [0, 0,5];
`multiplicador = 1,5 + LCK·0,01` em [1, 3]. LCK 4 → 7%, ×1,54; LCK 50 → 30%, ×2,0.

**Comparação [I]:** no DCR a LCK é muito mais forte no crítico. Não é
problema enquanto LCK não tiver outros usos fortes — mas hoje LCK também dá
HIT, esquiva e drops. Vale um teste de tempo-para-matar (ROADMAP etapa 14).

**[R]** (a) Taxa de crítico **por classe de arma** (`crit_bonus` em
`weapon_classes.ron`, padrão 0): adagas com mais crítico, machados com
menos — diferenciação barata. (b) Não adotar o "crítico por atributo
sorteado" (opaco para o jogador).

---

## 9. Experiência e níveis

**SOTN [F]** — EXP ao matar (`func_800FE044`, `5D5BC.c:420-445`, chamado em
`src/st/collision.h:580`):

```
exp = inimigo.exp
se nível_jogador > nível_inimigo:  repetir (diferença) vezes: exp = exp·2/3   (mínimo 1)
se nível_jogador < nível_inimigo:  repetir min(diferença, 5) vezes: exp += exp/4
limite: g_ExpNext[99]
```

Exemplos (inimigo nível 10, 100 EXP): jogador 12 → 66 → **44**;
jogador 7 → 125 → 156 → **195**; jogador 20 → 100·(2/3)¹⁰ ≈ **1**.

Level up: `g_ExpNext[nível+1] ≤ exp` → nível +1; cada atributo base tem 50%
de +1; se ganhou menos de 2, +1 num atributo aleatório; atributos até 99;
recursivo para vários níveis (`:308-362`). Familiar ganha EXP proporcional
(`:448-472`).

**DCR:** `exp_to_next(L) = round(30 · L^1,6)`, máximo 99; por nível STR/CON/INT
+1 automáticos e 2 pontos livres; EXP do inimigo fixa (`experience`).

**[R]** **EXP com diferença de nível** é a oportunidade mais útil desta seção:
desestimula *grinding* em área fácil e recompensa ir fundo no castelo.
Proposta própria: campo opcional `level` no inimigo (padrão = sem escala) e
função pura `scaled_experience(base, player_level, enemy_level)` com fatores em
`character.ron` (ex.: −30% por nível acima, +20% por nível abaixo, até 5).
Testes: bordas, mínimo 1, sem `level` = comportamento atual.

---

## 10. Equipamentos e modificadores

**SOTN [F]:** duas mãos (arma, escudo, consumível, comida, remédio) +
cabeça, armadura, capa, 2 acessórios (`CheckEquipmentItemCount`,
`5D5BC.c:123-139`). Item de mão (`Equipment`, `include/game.h:1568-1597`):
ATK, DEF, elemento, categoria, anim, `lockDuration`, `chainLimit`,
`specialMove`, `mpUsage`, `stunFrames`, `hitType`, `hitEffect`,
`criticalRate`, `enemyInvincibilityFrames`. Itens de corpo (`Accessory`):
`attBonus`, `defBonus`, `statsBonus[4]`, conjuntos de fraqueza/resistência/
imunidade/absorção. Muitos efeitos especiais são **checagens por ID de item
no código** (ex.: acessórios que mudam custo de Hearts, drops, dano).

**DCR:** 8 slots, `AttributeModifiers`, requisitos, `WeaponClass` + `WeaponStats`,
`special_effects` previstos por chave em dados, catálogo de 27 equipamentos (V2.0).

**[R]** Manter o modelo por dados. **Não** adotar checagens por ID no
código (o DCR já evita isso por regra). Campos de combate do item do SOTN que
valem como inspiração para `WeaponClass`: cancelamento, hitstun, intervalo de
reacerto, crítico, limite de instâncias.

---

## 11. Efeitos de status

**SOTN [F]:** veneno (ATK do jogador ÷2, dano recebido ×2); maldição (ao
atacar, cria só um efeito de contorno em vez da entidade da arma —
`src/dra/6E42C.c:527-530` —, ou seja, o golpe não acerta **[I]**); pedra
(CON encurta; a forma de sair não foi lida **[?]**); temporizadores
reduzidos por CON (`GetStatusAilmentTimer`, `5D5BC.c:209-254`):

```
veneno:  4095 − CON·16 quadros, mínimo 256   (≈68 s → mín. 4,3 s)
maldição: 1024 − CON·4 quadros, mínimo 64    (≈17 s → mín. 1,1 s)
pedra:   ajuste (CON + rand 0..11 − 9)/10 em [0, 4]
```

> **Atualização (Fase 2):** a saída da pedra foi confirmada por código. O
> valor de pedra acima não é tempo: é um **contador de toques** (8 − ajuste,
> ou seja, 4 a 8). Cada toque direcional desconta 1 e, ao chegar a 0, o
> personagem sai para o estado de recuo; com HP 0 petrificado, morre após
> 0x20 quadros (`src/dra/71830.c:1677, 1734-1763`). Ver
> [`PROPOSTA_ROADMAP_POS_V2_4.md`](PROPOSTA_ROADMAP_POS_V2_4.md) §3.

Buffs temporários de 4096 quadros (~68 s) para ATK/DEF/STR/INT/LCK e
resistências (`GiveStatBuff`, `5D5BC.c:866`; efeitos em `menu.c:221-229, 377-401`).

**DCR:** `StatusEffect` (Poison, Curse, Petrify, Burn, Freeze, Stun) e
`StatusResistances` existem; chance `base·(1 − resistência)`; **aplicação
ainda não implementada** (roadmap).

**[R]** Ao implementar status: (a) duração reduzida por CON com piso
(fórmula própria em `character.ron`), (b) efeitos que **mudam o jogo**
(veneno reduz ATK; maldição bloqueia técnicas; congelado trava) em vez de só
dano por tempo, (c) buffs temporários pelo mesmo mecanismo. Regra pura em
`magic/elements.rs` ou módulo novo `status`, com testes de duração.

---

## 12. Drops e sorte

**SOTN [F]** (`func_800FF494`, `5D5BC.c:989-1026`):

```
r = rand 0..255 − (LCK + rand 0..31)/20 − (raro · nº de anéis de drop)
r < raro          → item raro
r −= raro; r −= (LCK + rand 0..31)/20 (− incomum · anéis)
r < incomum       → item incomum
senão             → tabela comum (corações, dinheiro; 1/32 a 12/32)
```

LCK 20 desloca ~1–2 em 256 (≈0,5 ponto percentual). Em outro ponto,
`x + x·LCK/128` multiplica chances de drops especiais (`func_800FF460`,
`:978-986`; usado em `collision.h:589`).

**DCR:** `drop_chance = clamp(base · (1 + LCK·0,02), 0, 1)` (LCK 50 → ×2).

**[R]** Nenhuma mudança necessária. Observação de balanceamento: a LCK do
DCR vale bem mais que a do SOTN em drops **e** em crítico; ver §8.

---

## 13. Tabela-resumo de oportunidades

| Oportunidade | Fonte da ideia | Mudança no DCR | Risco | Prioridade |
|---|---|---|---|---|
| EXP escalada por diferença de nível | §9 | `level` opcional no inimigo + função pura | Baixo | Alta |
| Golpe fraco não derruba (*poise*) | §5 | Limiar de `Hurt`, sem mexer na fórmula | Baixo | Alta (com tiers de recuo) |
| Crítico por classe de arma | §8 | Campo opcional em `WeaponClass` | Baixo | Média |
| Itens de HP/MP máx em segredos | §3 | Dados + flags | Baixo | Média |
| Status com duração por CON | §11 | Implementar status (roadmap) | Médio | Média |
| Revisar peso de LCK (crítico + drops + HIT + esquiva) | §8, §12 | Só números em `character.ron` | Médio (balanceamento) | Média, após ferramenta de TTK |
| Teto suave do ATK da arma | §4 | Fórmula nova | Médio | Baixa |
| Elemento secundário por golpe | §7 | Schema | Alto | Baixa |
| Defesa subtrativa, pesos ocultos, crítico por atributo sorteado, atributos aleatórios no level up | §2, §5, §8, §9 | — | — | **Não recomendado** |
