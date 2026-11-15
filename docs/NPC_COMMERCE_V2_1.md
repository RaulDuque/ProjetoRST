# V2.1 — Cidade, NPCs, diálogos, comércio e tutoriais

> **V2.2:** NPCs ganharam figuras por dados (`look`), o mapa marca NPCs e
> lojas, e o HUD deixou de sobrepor notificações ao chão; ver
> [`VISUAL_EQUIPMENT_V2_2.md`](VISUAL_EQUIPMENT_V2_2.md) §9–10.

> Briefing: `Md-usario/Md-usario/DarkCastleR-V2-1.md` e a autorização
> `DarkCastleR-V2-1-a.md` (V2.1.1: Save v3). Estado: **parcial** (economia
> persistente concluída; tutoriais ainda fora de `Md-usario/tutorias/`).
> - Cidade, NPCs, diálogos, lojas, compra/venda, regra de duas mãos e
>   tutoriais estão **implementados e testados**.
> - **Persistência da economia** (moedas e estoque no save) **aguarda
>   autorização** de mudança de schema (§10).
> - Os tutoriais estão em `docs/tutorias/`, porque `Md-usario/` está bloqueada
>   para escrita pela configuração do projeto (§25).

## 1. Auditoria

Linha de base: branch `art/v1.2-protagonist`, HEAD `204f965`, 412 linhas de
status (a única nova era o próprio briefing em `Md-usario/Md-usario/`). Staged:
só o PNG anterior. 333 testes passando; assets publicados idênticos aos da V2.0.

| Item | Antes da V2.1 |
|---|---|
| NPCs, diálogos, lojas | **Inexistentes** (só a proposta em `NPC_COMMERCE_SYSTEM.md`) |
| Moeda / saldo | **Inexistente**; nenhum campo no Save V2 (`PlayerSave`, `WorldProgress`) |
| Interação `D` | Implementada para objetos (V1.8) |
| Valor de venda dos itens | Existia (`sell_value`) |
| Comparação de equipamentos | Implementada (V2.0) |
| Regra de duas mãos | **Inexistente** |
| Drops de equipamento | Sistema de drops genérico existe; nenhuma entrada de equipamento |
| Âncoras de mão e cabeça no pipeline | **Inexistentes**; o rig v007 tem os ossos `hand.R` e `head` |

## 2. Arquitetura

| Parte | Onde | Tipo |
|---|---|---|
| NPCs, nós de diálogo, condições, validação | `src/npc/definitions.rs` | puro |
| Conversa em andamento | `src/npc/dialogue.rs` | puro |
| Spawn por sala, `D`, aviso, estado de diálogo | `src/npc/systems.rs`, `NpcPlugin` | Bevy |
| Bolsa, lojas, estoque, `buy`/`sell` | `src/economy/shop.rs` | puro |
| Bolsa do jogador, pedidos de troca, reset no load | `src/economy/systems.rs`, `EconomyPlugin` | Bevy |
| Caixa de diálogo | `src/ui/dialogue_box.rs` | UI |
| Tela de loja | `src/ui/shop_screen.rs` | UI |
| Dados | `assets/data/{npcs,dialogues,shops}/*.ron` | dados |

Estados novos: `GameState::Dialogue` e `GameState::Shop`, pausados como o
Status e o Inventário. A mensagem nova `TradeRequest` só é lida por
`apply_trades`, o único escritor de moedas e estoque. Nada é lido do disco
depois do início do jogo.

## 3. NPCs

Formato: `NpcDefinition { id, name, kind, room, position, size, color, greetings, shop }`.
`kind` pode ser `Villager`, `Merchant`, `Smith`, `Informant` ou `QuestGiver`.
São retângulos coloridos estáticos, sem arte e sem pathfinding.

Validação, feita ao carregar e em `app::check_npcs`:
- IDs únicos;
- tamanho e posição válidos;
- uma saudação padrão (sem condições);
- nós referenciados existentes e do mesmo NPC;
- a sala existe e o corpo do NPC fica dentro das paredes.

Cinco NPCs: Bram (guardião da estrada), Hooded Pilgrim (pista), Hessa
(comerciante), Garrick (ferreiro, sem serviço ainda) e Old Wenna (moradora).

## 4. Interações

- `D` a até **56** de distância horizontal e **40** vertical do centro do NPC.
- Com vários NPCs ao alcance, fica o mais próximo; no empate, o menor `id`.
- **Objetos têm prioridade.** `talk_to_npcs` roda **antes** de
  `interact_with_objects` e consulta `world::interactions::usable_object`. Se
  houver um baú, alavanca ou interruptor utilizável, ele não faz nada.
  - Um teste pegou o defeito da primeira versão, em que o baú abria e a
    conversa também começava no mesmo frame.
- Não funciona caído, em ação, na Forma Espectral (a entrada `D` é mascarada)
  nem com NPCs de outra sala (os NPCs são entidades da sala atual).
- O aviso "Nome / D Talk" aparece só sobre o NPC ao alcance.

## 5. Diálogos

`DialogueNode { id, speaker, lines, choices, next }`. As falas avançam com
**Enter** ou **D**; **Esc** sai a qualquer momento.

Condições (`DialogueCondition`, conjunto fechado e tipado):
- `Ability`;
- `Flag`;
- `Item`;
- `Visited(sala)`;
- `RegionVisited(região)`.

Todas leem dados que o jogo já guarda, sem flags novas. Não existe linguagem
de script: qualquer outro valor nem passa no parse (há teste para isso).

## 6. Escolhas

`Choice { text, when, action }`, em que a ação é `Goto(nó)`, `OpenShop` ou
`End`. Regras:
- o cursor fica sempre dentro das escolhas visíveis;
- uma lista em que **todas** as escolhas são condicionais é recusada (o
  jogador nunca fica preso);
- `OpenShop` só é aceito em NPC com loja.

## 7. Cidade

Região `ash_village` ("Village of Ashes", a Vila das Cinzas), com 6 salas
`Optional` em (4..9, 1):
1. Ash Road Gate;
2. Cinder Square;
3. Merchant Row;
4. Hessa's Outfitters;
5. Garrick's Smithy;
6. Wenna's Cottage.

Foi navegada com o movimento humano atual na rota real.

## 8. Conexões

- Porta direita do Watch Shaft na célula 1 (`to_village`) ↔ `av_gate_road.to_shaft`.
- Novo degrau `one_way` em (750, 760): saindo do degrau de y=810, o jogador cai
  nele e depois na saliência da porta.
- A cidade é opcional; a sala inicial e a progressão aprovada não mudaram.
- Main, secrets, spectral e equipment continuam passando, com 0 reposicionamentos.

## 9. Economia

- Moeda única: **Marks**, guardada em `Purse(u32)`.
- Limite de `MAX_COINS` 999 999; somas e subtrações usam `checked_add` e
  `checked_spend`, sem float, sem overflow e sem valor negativo.
- Moedas iniciais: `coins: 120` em `character.ron`.
- Fontes de moeda: só a venda. Recompensas em moedas ficam para quando a
  persistência for autorizada.
- O HUD mostra "Marks N" na linha de Hearts.

## 10. Save v3 (V2.1.1, autorizado em `DarkCastleR-V2-1-a.md`)

- **Formato:** `SaveData.economy: Option<EconomyRecord { coins, stock }>`;
  `stock` guarda, por loja e item, as unidades vendidas das entradas
  limitadas. Só ids estáveis. `CURRENT_SAVE_VERSION = 3`.
- **Migração em memória:** v1 → v2 → v3 e v2 → v3 (`SaveFileV2` congelado com
  `deny_unknown_fields`). Todos os campos antigos são mantidos (inventário,
  equipamento, relíquias, exploração, objetos, posição, progressão, mundo,
  tempo). `economy: None` → moedas iniciais (`start.coins`, 120) e estoque
  cheio, determinístico e igual a cada load (não acumula).
- **Load nunca escreve** (testado). Arquivo inválido, versão desconhecida ou
  antiga demais → erro, arquivo intacto.
- **Backups:** política existente, agora também para v2: a primeira gravação v3
  copia o v2 para `slot<N>.ron.v2.bak` (uma vez, nunca sobrescrito) e para o
  `.bak` rotativo; backups permanentes anteriores (`.v1.bak`) não são tocados;
  gravação por `.tmp` + rename.
- **Transações:** `quick_save` grava moedas e estoque no mesmo arquivo da
  bolsa; `quick_load` restaura os três juntos. A trava provisória "Cannot save
  after trading", o `EconomyLedger` e o `reset_on_load` foram **removidos**,
  depois de os testes provarem a persistência (compra/venda + F5 + troca de
  sala + F9; estoque esgotado continua esgotado; F9 repetido não concede nada).

## 11. Lojas

- Formato: `ShopDefinition { id, name, keeper, entries, buys }`, em `ShopLibrary`.
- O NPC e a loja precisam se apontar mutuamente (`DataError::ShopKeeper`).
- Os itens vêm do catálogo existente; nenhum item foi recriado.

## 12. Estoque

- `Stock::Unlimited` ou `Limited(n ≥ 1)`; o estado fica em `ShopStock`, por loja e item.
- A última unidade só sai uma vez: um teste manda duas compras no mesmo frame.
- Não reaparece indevidamente: o load restaura o estado do save, conforme a §10.

## 13. Compras

`economy::shop::buy` valida, nesta ordem:
1. a mercadoria está visível e suas condições valem;
2. há estoque;
3. as moedas bastam.

Depois faz a única mudança que pode falhar (pôr o item na bolsa) e só então
desconta as moedas e o estoque. Se qualquer validação falhar, nada muda.

## 14. Vendas

`economy::shop::sell` paga o `sell_value` do item. Recusa:
- loja que não compra;
- item fora da bolsa (equipados não ficam na bolsa, então é preciso tirar antes);
- `QuestItem` ou item sem valor;
- a última arma, quando nenhuma está equipada;
- bolsa cheia.

## 15. Preços

- O preço de compra fica na loja; o de venda é o `sell_value` do item.
- O carregamento recusa:
  - `price ≤ sell_value`, o que impede lucro comprando e revendendo;
  - `price` 0 ou acima de 99 999;
  - `Limited(0)`;
  - item duplicado na loja;
  - `QuestItem`.
- Um teste compra e revende a mesma faca 20 vezes e o dinheiro só diminui.
- Política adotada: preço entre 2 e 2,5 vezes o `sell_value`.

## 16. Interface

**Caixa de diálogo:** fica embaixo, opaca e sem cobrir o HUD. Mostra o nome, a
fala, as escolhas com cursor e a dica de teclas ("continue >>", "close",
"choose").

**Tela de loja:** mostra o nome da loja, o comerciante, as Marks e as abas
Buy/Sell. Na lista aparecem categoria, preço, estoque ou quantidade e "sold
out". Os detalhes trazem categoria, raridade, "Two-handed" e a comparação com o
que está vestido, usando `inventory::compare` (as mesmas linhas do inventário).
Os erros aparecem como mensagens do HUD.

Em ambos o jogo fica pausado, então não há ataque. Esc ou o fim da conversa
voltam a `Playing`, e um load fecha os dois.

## 17. Equipamentos

- A loja da Hessa vende parte do catálogo: Minor Tonic, Gutter Knife e Sentry
  Coif (ilimitados).
- Itens limitados:
  - Watch Buckler (1 unidade, desde a V2.1.1);
  - Quiet Soles (1 unidade);
  - Still Water Amulet (1 unidade, só depois da Relíquia do Véu).
- Nada que trivialize o início: as melhores armas continuam em baús e em conteúdo opcional.

## 18. Armas de duas mãos

- `WeaponClass.two_handed` (com `serde(default)`) vale `true` em Spear e HeavyWeapon.
- `loadout::check_hands` recusa nos dois sentidos, com mensagens próprias na tela:
  - arma de duas mãos com algo na mão secundária (`OffhandInUse`);
  - mão secundária com arma de duas mãos (`TwoHandedWeapon`).
- Nada é removido sozinho e o save não mudou.
- Saves antigos com lança e escudo carregam como estão; a regra vale para as próximas trocas.
- A tela de inventário mostra "Two-handed" e o motivo da recusa **antes** do
  Enter. Isso foi corrigido depois da inspeção visual: antes, a comparação
  aparecia sem aviso.

## 19. Aparência pendente (V2.0)

- O rig v007 tem os ossos `head` e `hand.R`.
- Para resolver a espada embutida no sprite faltam:
  1. Na renderização (`build_game_sprites_v1_5.py` e `render.py`), projetar a
     cabeça desses ossos na câmera em cada quadro e gravar `anchors` (mão,
     cabeça, ângulo da lâmina) no JSON do manifesto.
  2. Estender `publish_guard.py` e o manifesto `player.ron` para aceitar e
     validar `anchors`.
  3. Fazer um passe de renderização **sem a lâmina** (`blade_readable` oculto)
     para um atlas do corpo desarmado, mais um sprite por arma posicionado pela
     âncora.
- Os passos 1 a 3 exigem re-renderizar e republicar os atlas protegidos.
  - É uma etapa artística própria, com aprovação, e não foi feita.
- **Aparência não é declarada completa.**

## 20. Drops

Avaliado e **não implementado**:
- o único inimigo real (Bone Sentry) está nas rotas aprovadas;
- uma entrada a mais em `drops` consumiria números do `GameRandom` e mudaria o
  combate determinístico delas.

Extensão mínima proposta: um gerador de loot separado, com semente por inimigo
e entradas de equipamento, gravado por morte. Fica pendente.

## 21. Mapa

- As salas da vila entram no mapa como qualquer região: visitadas, descobertas
  (contorno), conexões e localização atual. A cor é laranja.
- Totais: 28 salas, 3 segredos. A fórmula de exploração não mudou.
- Os testes de total do mundo foram atualizados de 22 para 28, como na V1.9.
- **Pendente:** ícones de loja e de NPC no mapa.

## 22. Testes

| Arquivo | Testes | Cobre |
|---|---|---|
| `tests/npc_commerce_tests.rs` | 10 | carregamento de NPCs, IDs duplicados, sala inválida, diálogos, escolhas, condições, preços inválidos, overflow, compra, venda, estoque, duas mãos |
| `tests/town_app_tests.rs` | 6 | distância, interação simultânea baú+NPC, outra sala, load durante diálogo, loja pausa o combate, compra e venda duplicadas no mesmo frame, save/load da economia, timings |
| `tests/world_town_route_tests.rs` | 1 | rota real |
| `tests/tutorial_examples_tests.rs` | 3 | estrutura dos 24 tutoriais, parse de todos os blocos, exemplo completo validado e percorrido com teclas |

Testes antigos: `world_graph_tests` teve três totais globais atualizados de 22
para 28 salas. Nenhuma asserção foi afrouxada.

## 23. Rota

`tests/support/town_route.rs` e `examples/world_town_route.rs`:
1. Do início do jogo, pela rota V1.7, até o degrau de y=849 do Watch Shaft.
2. Degrau novo, saliência e vila.
3. Bram: diálogo, escolha "ask" (duas falas) e volta.
4. "Farewell", praça (com o mapa).
5. F5 antes de negociar.
6. Hessa: loja, compra da Gutter Knife (120 → 40), venda de um tônico (→ 50).
7. Equipar a faca; golpe de 28.
8. F5 recusado.
9. Volta ao castelo e F9: outfitter, 120 Marks, Iron Sword, 3 tônicos.
10. A loja abre de novo.
11. Volta ao castelo.

**19 trocas de sala, 0 reposicionamentos artificiais.**

## 24. Performance

Medido em headless, perfil dev (`target/tmp/town_v21_perf.txt`):

| Medida | Valor |
|---|---|
| Troca de sala com NPC | 1,76 ms |
| Entidades na loja | 256 (o mundo inteiro) |
| Frame comum | 0,26 ms |
| Abrir diálogo (2 frames) | 0,51 ms |
| Abrir loja (2 frames) | 0,47 ms |
| Frame com a loja aberta | 0,19 ms |
| Compra | 0,19 ms |
| Venda | 0,20 ms |

- Só os NPCs da sala atual existem.
- Nenhum NPC é atualizado por frame, exceto o aviso do que está ao alcance.
- O RON é lido só no início.

## 25. Tutoriais

24 arquivos, com os nomes exigidos pelo briefing. Cada um segue o modelo de 9
seções (`00_LEIA_PRIMEIRO.md` traz também o índice).

**Local:** `docs/tutorias/`. O `.claude/settings.json` tem
`"Edit(/Md-usario/**)"` em `deny`, e uma regra `deny` vence qualquer `allow`.
Por isso a gravação em `Md-usario/tutorias/` foi recusada, e alterar
permissões não está entre as autorizações do briefing. Mover os arquivos é uma
cópia simples quando a permissão for ajustada; o teste lê `Md-usario/tutorias/`
assim que essa pasta existir.

**Validação** (`tutorial_examples_tests`):
- Todo bloco ```` ```ron ```` marcado com `// exemplo-verificado: <Tipo>` passa
  no parse do tipo real.
- O exemplo completo (região Ashen Crypt, 3 salas; porta na casa da Wenna;
  itens; inimigo; NPC; diálogo; loja) é somado a uma **cópia** dos dados e do
  mundo.
  - Passa no validador sem erros nem avisos.
  - Passa nas checagens de `load_world` e em `check_npcs`.
  - É percorrido com teclas: luta, alavanca, conversa e loja, relíquia, baú,
    parede quebrada, sala secreta e dois baús.
- Blocos ```` ```text ```` são documentais e não são verificados.

## 26. Riscos

- **Economia:** sem persistência, não se pode salvar depois de negociar (§10).
- **Tutoriais:** fora do local definitivo até a permissão ser ajustada.
- **Arte:** NPCs sem arte; a espada continua embutida no sprite (§19).
- **Saves históricos reais:** não foram lidos (é proibido).
- **Aviso "D Talk":** usa `Text2d` no mundo; com o NPC na borda da tela, o
  texto pode sair dela.

## 27. Próximas etapas

1. Autorizar o Save v3 da economia (§10).
2. Ajustar a permissão de `Md-usario/tutorias/` e mover os tutoriais.
3. Exportar âncoras e fazer o passe sem lâmina no pipeline artístico (§19).
4. Gerador de loot isolado para drops de equipamento (§20).
5. Ícones de loja e de NPC no mapa; arte dos NPCs.
6. Serviços do ferreiro (V2.x, fora desta versão).
