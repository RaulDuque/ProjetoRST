# Estudo SOTN — interface, inventário e menus

Fase 2 da pesquisa. Convenções: [`REFERENCIAS_SOTN.md`](REFERENCIAS_SOTN.md).
Caminhos `src/...` do **sotn-decomp @ `3a0957b`**; os nossos, do **DCR @
`9f7616d`**. Legenda: **[F]** fato · **[I]** inferência · **[R]**
recomendação · **[?]** pendente.

Estuda **comportamento** (fluxo, informação, estados). Nenhum layout,
gráfico, fonte, ícone ou texto do jogo original deve ser copiado.

---

## 0. Resumo

1. O SOTN tem **um** menu de pausa com abas (equipamento, magias, relíquias,
   sistema, familiares), uma máquina de estados explícita (`g_MenuStep`) e
   a regra "abrir o menu pausa o mundo". **[F]**
2. Na troca de equipamento, ele mostra a **prévia** de ataque por mão,
   defesa e dos quatro atributos, com seta de subida/descida, calculada
   equipando temporariamente o item e rodando a fórmula real. **[F]**
3. **Magias só aparecem na lista depois da primeira execução bem-sucedida**
   (`LearnSpell` é chamado pela própria magia). O menu mostra o comando.
   **[F]**
4. O HP do HUD **escorre**: o número exibido se aproxima do real por 1/10 da
   diferença por quadro. A cor muda com o HP cheio e abaixo de 1/4. **[F]**
5. Números de dano e nome do inimigo são **recompensas** liberadas por
   relíquias, não padrão. O primeiro contato com um inimigo registra-o na
   lista de inimigos (bestiário). **[F]**
6. O DCR **já tem** a maioria dessas peças, várias em versão mais
   informativa (comparação completa antes → depois). As lacunas são: HP
   escorrendo, pausa dedicada, ordenação/filtro, mapa em tela cheia,
   bestiário e um estado "magia aprendida ao executar". (§3)

---

## 1. SOTN: fatos levantados

| Tema | Fato | Evidência |
|---|---|---|
| Abas do menu | `MENU_DG_MAIN`, `EQUIP_OVERVIEW`, `EQUIP_SELECTOR`, `INFO_BAR`, `RELICS`, `SPELLS`, `SETTINGS`, `TIME_ATTACK`, `EQUIP_SORT`, `FAMILIARS`… | `src/dra/dra.h:31-47` |
| Máquina de estados | `g_MenuStep`: INIT → FADE_FROM_GAME → OPENING → OPENED → (EQUIP, SPELL, RELIC, SYSTEM, FAMILIAR) → EXIT_* → RETURN_TO_GAMEPLAY | `src/dra/menu.c:3450-3678` |
| Entrada indisponível | Opção bloqueada (sem magias, sem familiares) toca som de erro e não muda de estado | `menu.c:3693-3729` |
| Fechar | `Start` fecha de qualquer subtela (se os botões estiverem todos configurados); `Back` volta um nível | `menu.c:3441-3446, 3679-3682, 4159-4168` |
| Pausa permitida | `g_PauseAllowed` é desligado em cenas e transições; o menu só abre quando permitido | `src/dra/5087C.c:19-21, 1258`; `62DEC.c:1376, 1413, 1532` |
| Sons do mundo na pausa | Scripts de som do mundo pausam e retomam com o menu | `menu.c:3673`; `sound.c:1401-1405` |
| Equipamento | Cursor de slot; mão (direita/esquerda) abre a lista de itens de mão; corpo abre a de acessórios; cada lista mostra quantidade, ícone e descrição | `menu.c:4142-4250` |
| Ordenação | Botão de ordenar só na lista de mão; 11 critérios | `menu.c:4152-4196`; `MenuEquipSortDraw` `:1784` |
| Prévia | Ataque das duas mãos, defesa e STR/CON/INT/LCK com seta (↑ ↓ =) entre atual e novo | `menu.c:1242-1284` |
| Duas mãos | Ao equipar duas mãos, a outra mão é ocupada/limpa | `menu.c:3209-3216, 3301-3310` |
| Status | Atributos, ataque/defesa, salas visitadas (`g_RoomCount`), mortes (`killCount`), tempo | `menu.c:1286-1352` |
| Magias | Só as aprendidas (bit `0x80`); mostra a sequência de comando | `menu.c:1426-1470`; `5D5BC.c:268-280` |
| Aprender magia | `LearnSpell` é chamado dentro da execução de cada magia | `src/dra/704D0.c:260, 335, 417, 530, 650, 766` |
| HUD: HP | Número exibido escorre 1/10 da diferença por quadro (mínimo 1); destaque com HP cheio; amarelo ≤ 1/4 | `src/dra/5F60C.c:1363-1390` |
| HUD: MP | Barra com paleta especial quando cheia | `5F60C.c:1355-1361` |
| Números de dano | Criados no acerto **só com a relíquia** correspondente | `src/st/collision.h:459-467` |
| Nome do inimigo | Texto no canto, ao acertar, **só com relíquia**, uma vez por entidade | `collision.h:410-418` |
| Registro de inimigo | O primeiro acerto marca o bit do inimigo em `g_CastleFlags[ENEMY_LIST…]` | `collision.h:405-408` |

---

## 2. DCR: o que existe

| Tema | DCR | Evidência |
|---|---|---|
| Telas | Status (`Tab`), Inventário (`I`), Loja, Diálogo, cada uma um `GameState` que pausa a simulação | `src/app/states.rs:6-18`; `src/input/mod.rs:59-66` |
| Pausa dedicada | **Não existe** (não há tela de pausa nem `GameState::Paused`) | `states.rs` |
| Mapa | Painel no canto (`M`), **não pausa**; visitado/descoberto/atual, portas, baús, segredos, % por região | `src/ui/world_map.rs:1-19, 53, 119` |
| Inventário | 8 slots + mochila; prévia **antes → depois** com os números finais pelas fórmulas reais | `src/ui/inventory_screen.rs:1-14`; `src/inventory/compare.rs:1-7` |
| Loja | Mesma comparação | `src/ui/shop_screen.rs:1-7` |
| Ordenação/filtro | **Não há** (lista com rolagem) | `inventory_screen.rs:36, 344` |
| Habilidades/magias | Listadas no Status; o comando aparece se `command_revealed` | `src/ui/status_screen.rs:180-190`; `src/skills/unlock.rs:80, 129-136` |
| HUD | Barras HP/MP/EXP com número, Hearts, nível, cooldowns, retorno do comando ("too slow"), notificações | `src/ui/hud.rs:1-7, 244-270, 341` |
| HP escorrendo | **Não**: a barra vai direto ao valor | `hud.rs:262` |
| Números de dano | Sempre ligados | `src/ui/damage_numbers.rs` |
| Bestiário | **Não existe** | — |
| Retorno de comando | Já mais rico que o SOTN (mostra por que falhou) | `hud.rs:341` |

---

## 3. Comparação e recomendações

| # | Lacuna | Proposta original | Peso | Quando |
|---|---|---|---|---|
| 1 | HP escorrendo | Barra dupla: o valor real cai na hora e uma "sombra" desce até ele em `HP_DRAIN_TIME`. Só apresentação (`ui::hud`), constante em `core/constants.rs` | Baixo | V2.4 (opcional, último passo) ou depois |
| 2 | Pausa dedicada | `GameState::Paused` (tecla `Esc`/`P`) com retomar, controles e sair; reaproveita a pausa dos outros estados; respeita uma flag "pausa permitida" (diálogo, transição) | Médio | Polimento de UI |
| 3 | Mapa em tela cheia | Modo ampliado do painel atual que pausa o mundo; mesmo `ExplorationState`, sem regra nova | Médio | Exploração |
| 4 | Ordenação/filtro | Ordenar a mochila por categoria/raridade/ataque, como regra pura testável em `inventory` | Baixo | Polimento de UI |
| 5 | Bestiário | Registro em `WorldProgress` (conjunto de `EnemyId` vistos/derrotados) a partir de `DamageDealt`/`EnemyDefeated`. **Muda o save** (campo novo com `#[serde(default)]`) | Médio | Versão própria |
| 6 | Magia descoberta ao executar | Já existe `command_revealed`; avaliar revelar também quando a sequência é executada sem ter sido ensinada (se o design quiser segredos) | Baixo | Decisão de design |
| 7 | Números de dano como opção | Manter ligados; tornar desligáveis numa futura tela de opções (sem gating por item) | Baixo | Opções |
| 8 | Contadores de status | Salas visitadas já existem (% no mapa); mortes e tempo de jogo não. Adicionar ao Status quando houver demanda | Baixo | Polimento |
| 9 | Estado de menu explícito | O DCR usa `GameState` por tela, que basta enquanto não houver submenus aninhados. Não reescrever | — | — |

**[R]** Nada disso entra no V2.4, exceto o item 1, opcional. A interface do
DCR já supera o SOTN em transparência (comparação completa, motivos de
bloqueio, retorno do comando). As lacunas reais são pausa, mapa grande e
bestiário.

---

## 4. Impacto

| Proposta | Save V3 | Controles | Animação | Risco |
|---|---|---|---|---|
| HP escorrendo | Não | Não | Não | Baixo |
| Pausa | Não | Tecla nova (verificar conflito em `InputBindings`) | Não | Médio (interação com outros estados) |
| Mapa cheio | Não | Reuso de `M` (duplo toque ou segurar) ou tecla nova | Não | Baixo |
| Ordenação | Não | Tecla na tela de inventário | Não | Baixo |
| Bestiário | **Sim**: campo novo, compatível por `serde(default)` | Tela nova | Não | Médio |

---

## 5. Pendências

- Tela de mapa em tela cheia do SOTN (`all_map_check` e desenho) não foi
  lida em detalhe nesta fase. **[?]**
- Os 11 critérios de ordenação não foram listados um a um. **[?]**
- Menu de relíquias e configuração de botões: fora do escopo. **[?]**
