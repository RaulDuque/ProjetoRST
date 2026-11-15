# Proposta de roadmap após o Combat Feel (para aprovação)

**Status: proposta.** A numeração V2.2–V2.4 foi decidida na D-028; a ordem
dos blocos abaixo ainda depende de aprovação.
Os blocos abaixo **não têm número de versão**: a numeração é decisão do
usuário (§1).

Legenda: **[F]** fato · **[I]** inferência · **[R]** recomendação · **[?]**
pendente.

---

## 1. Conflito de numeração encontrado

| Fonte | V2.2 | V2.3 | V2.4 | V2.5 |
|---|---|---|---|---|
| `GAME_VISION_MASTER.md` §6 (direção) | Conteúdo do castelo | Chefes e progressão | **Missões e narrativa** | Expansão do mundo |
| Entregue de fato (`docs/VISUAL_EQUIPMENT_V2_2.md`, `docs/WORLD_VISUAL_V2_3.md`) | Equipamento visual | Mundo visual | — | — |
| Briefing da pesquisa Fase 2 | — | — | **Combat Feel** | — |

**[F]** A tabela-direção já divergia do que foi entregue a partir da V2.2.
**[R]** Decidir uma das opções antes de iniciar o Combat Feel:

1. Adotar a numeração do briefing (V2.4 = Combat Feel) e atualizar a tabela
   de `GAME_VISION_MASTER.md` num commit `docs` próprio.
2. Manter a tabela e dar outro nome à versão (por exemplo, "V2.3.1" ou
   "V2.4a").

Nenhuma das duas foi aplicada.

> **Resolvido (consolidação):** o proprietário escolheu a opção 1
> (`DECISIONS.md`, D-028). `GAME_VISION_MASTER.md` §6 e `ROADMAP.md` agora
> registram V2.2 e V2.3 como entregues e V2.4 = Combat Feel; os demais
> temas são itens futuros sem número. Os blocos da §2 continuam sem número
> até nova aprovação.

---

## 2. Blocos propostos (ordem recomendada)

| Ordem | Bloco | Conteúdo | Depende de | Save | Estudo |
|---|---|---|---|---|---|
| 1 | **Combat Feel** | Pacote P1 | — | Não | [`PLANO_V2_4_COMBAT_FEEL.md`](PLANO_V2_4_COMBAT_FEEL.md) |
| 2 | **Chefes I** | Arquitetura de chefe sobre `Enemy` (fases por HP, tabela de ataques, arena com portas, recompensa `BossReward`), 1 chefe completo | 1 (telegrafia e atordoamento) | Não (`defeated_bosses` já existe) | [`ESTUDO_SOTN_CHEFES.md`](ESTUDO_SOTN_CHEFES.md) §6 |
| 3 | **IA avançada** | Estados `Retreat` (já declarado e sem uso), patrulha, distância preferida, projéteis com limite, reação por postura (`poise`) | 1, 2 | Não | `ESTUDO_SOTN_CHEFES.md` §2, §4 |
| 4 | **Exploração II** | Pontos de save funcionais (`discovered_save_points` existe sem uso), viagem rápida, mapa em tela cheia, decisão `respawn_time` | — | Usa campos existentes | [`ESTUDO_SOTN_EXPLORACAO.md`](ESTUDO_SOTN_EXPLORACAO.md) |
| 5 | **Progressão por habilidades** | `DoubleJump` e `WallBreaker` (hoje só nomes) como `Requirement`s reais | 4 | Sim (habilidades obtidas, provavelmente em `flags`) | `ESTUDO_SOTN_EXPLORACAO.md` |
| 6 | **Armas especiais e escudos** | Ler `special_effects` (hoje ignorado), escudo com bloqueio ativo, campos `crit_bonus`/`lunge`/`max_active` | 1 | Não | [`ESTUDO_SOTN_EQUIPAMENTOS.md`](ESTUDO_SOTN_EQUIPAMENTOS.md) |
| 7 | **Combos** | Encadeamento de golpes básicos e técnica-depois-de-técnica, sobre a janela de cancelamento | 1 (etapa 3) | Não | [`ESTUDO_SOTN_COMBATE.md`](ESTUDO_SOTN_COMBATE.md) §1.13 |
| 8 | **Bestiário** | Registro por contato/derrota em `WorldProgress` | 2 | **Sim** (campo novo, `serde(default)`) | [`ESTUDO_SOTN_INTERFACE.md`](ESTUDO_SOTN_INTERFACE.md) §3 |
| 9 | **Interface** | Pausa dedicada, HP escorrendo, ordenação de inventário, opções (números de dano) | — | Não | `ESTUDO_SOTN_INTERFACE.md` |
| 10 | **Animações** | Marcas no manifesto + esticamento por fase + checagem arte × dados | 1, arte nova | Não | [`ESTUDO_SOTN_ANIMACOES.md`](ESTUDO_SOTN_ANIMACOES.md) §3.3 |
| 11 | **Retorno audiovisual** | Tremor de câmera, flash por fases, som (exige decisão de assets próprios) | 1 | Não | `ESTUDO_SOTN_ANIMACOES.md` §4 |
| 12 | **Arte final** | Por categoria/item (`VISUAL_EQUIPMENT_V2_2.md` §16) | 10 | Não | — |
| 13 | **Balanceamento** | Passe global: armas, inimigos, chefes, EXP | 1–8 | Não | [`ESTUDO_SOTN_ATRIBUTOS.md`](ESTUDO_SOTN_ATRIBUTOS.md) |

**[R]** Missões e narrativa (a V2.4 da tabela-direção) não aparecem nos
estudos SOTN. O lugar delas no roadmap é decisão de design e independe
desta pesquisa.

---

## 3. Anexo — classificação das pendências da Fase 1

| Pendência | Classificação | Evidência principal |
|---|---|---|
| Quando inimigos reaparecem | **Confirmada por código** | Máscara de mortos zerada a cada carga de sala (`src/dra/5087C.c:1833-1834, 1843-1844, 1885-1886, 1895-1896`); checagem no spawner (`src/st/create_entity.h:205, 256, 340, 383`) |
| Saída da petrificação | **Confirmada por código** | Contador de "sacudidas" = 8 − ajuste por CON (0 a 4, com sorteio) (`src/dra/5D5BC.c:228-238`); cada toque direcional desconta 1; ao chegar a 0 → `Player_Hit` (`src/dra/71830.c:1743-1763`); com HP 0 petrificado, morte após 0x20 quadros (`:1734-1741`) |
| Regras de duas mãos | **Confirmada por código** | `src/dra/menu.c:3209-3216, 3301-3310` |
| Bloqueio com escudo | **Parcialmente confirmada** | Segurar mantém; caixa própria; cai ao sofrer dano (`w_008.c:60-65, 91-113, 188-203`); sem redução de dano no caminho do jogador; reação de cada inimigo **[?]** |
| Valores das propriedades de armas | **Significado confirmado; valores não encontrados** | Campos em `include/game.h:1568-1597`; valores vêm de arquivo gerado do disco, ausente do repositório |
| Comportamento de chefes | **Confirmada** para Medusa, Olrox (forma normal), Scylla e Hippogryph; **não encontrada** para Olrox (forma real), Minotauro e Lobisomem (só assembly) | `ESTUDO_SOTN_CHEFES.md` §2 |
| Estrutura de mapa e transições | **Confirmada por código** (salas, overlays, spawner, bits do mapa); fluxo da sala de save **[?]** | `ESTUDO_SOTN_EXPLORACAO.md` |

Observação para o DCR: `StatusEffect::Petrify` (e `Freeze`, `Stun`) existe
só como resistência (`src/magic/elements.rs:64-73`); nenhum sistema aplica
esses estados. A mecânica de "sacudir para sair" seria uma adição futura,
não uma correção.
