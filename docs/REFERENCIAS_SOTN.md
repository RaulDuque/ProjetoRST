# Referências técnicas — Castlevania: Symphony of the Night (SOTN)

Estudo de referência para o Dark Castle RPG. **Nada deste estudo foi copiado
para o jogo**: nem código, nem tabelas, nem nomes, sprites, músicas, mapas ou
textos. Os documentos só descrevem comportamentos observados e propõem
implementações próprias em Rust/Bevy.

Documentos do estudo:

| Documento | Conteúdo |
|---|---|
| `REFERENCIAS_SOTN.md` (este) | Repositórios, commits, licenças, mapa de arquivos, convenções de leitura, limitações |
| [`ESTUDO_SOTN_COMBATE.md`](ESTUDO_SOTN_COMBATE.md) | Combate, movimentação, magias/comandos e reação de inimigos, comparados com o nosso código |
| [`ESTUDO_SOTN_ATRIBUTOS.md`](ESTUDO_SOTN_ATRIBUTOS.md) | Atributos, fórmulas de dano, defesa, críticos, EXP, níveis, drops, status |
| [`PLANO_MELHORIAS_SOTN.md`](PLANO_MELHORIAS_SOTN.md) | Tabela comparativa, melhorias priorizadas, riscos e pendências |
| [`ESTUDO_SOTN_CHEFES.md`](ESTUDO_SOTN_CHEFES.md) | Fase 2: chefes e arquitetura de chefe proposta |
| [`ESTUDO_SOTN_EXPLORACAO.md`](ESTUDO_SOTN_EXPLORACAO.md) | Fase 2: salas, transições, mapa, respawn, segredos |
| [`ESTUDO_SOTN_EQUIPAMENTOS.md`](ESTUDO_SOTN_EQUIPAMENTOS.md) | Fase 2: campos de arma/acessório, duas mãos, escudo |
| [`ESTUDO_SOTN_ANIMACOES.md`](ESTUDO_SOTN_ANIMACOES.md) | Fase 2: animação × gameplay, sensação de impacto |
| [`ESTUDO_SOTN_INTERFACE.md`](ESTUDO_SOTN_INTERFACE.md) | Fase 2: menus, HUD, inventário |
| [`PLANO_V2_4_COMBAT_FEEL.md`](PLANO_V2_4_COMBAT_FEEL.md) | Plano aprovado como direção da V2.4 |
| [`PROPOSTA_ROADMAP_POS_V2_4.md`](PROPOSTA_ROADMAP_POS_V2_4.md) | Blocos futuros e classificação das pendências da Fase 1 |

Data da pesquisa: 2026-11-15. Estado do nosso repositório no início: branch
`art/v1.2-protagonist`, HEAD `9f7616d` (V2.3), 5 commits à frente de
`origin/art/v1.2-protagonist` (`6db2667`).

## 1. Repositórios consultados

| | Projeto A — SymphonyRecomp | Projeto B — sotn-decomp |
|---|---|---|
| URL | <https://github.com/BlackLabelHQ/SymphonyRecomp> | <https://github.com/xeeynamo/sotn-decomp> |
| Commit analisado | `2deead5dc5f03f656c4ed625c1865fad5a544b8f` (2026-11-14) | `3a0957bee537554b08e830413f3ae2572b552e6e` (2026-11-14) |
| Natureza | **Recompilação** estática do executável de PS1 para PC, em C#/.NET 10, com patches (widescreen, tradução, QoL, randomizer). O código do jogo é **gerado a partir do disco do usuário** (`RecompOne` contra `config/sotn.json`) | **Decompilação** em C que recompila *byte a byte* para o binário original (PS1, PSP, Saturn) |
| Licença | **Nenhum arquivo `LICENSE`** no repositório (só `README.md`). Sem licença explícita = todos os direitos reservados | **AGPL-3.0-or-later** (`LICENSE`, `src/LICENSE`, cabeçalho SPDX em cada `.c`) |
| Submódulos | `RecompOne` (o recompilador), `mods` (`SymphonyRecompMods`) | ferramentas: `asm-differ`, `m2c`, `saturn-splitter`, `mwccgap`, `decomp-permuter`, `mipsmatch`, `sotn_utils`, `psyz`, `sound-player`, `satz` |
| Depende do jogo original | Sim: exige `disc/` com o CD norte-americano do próprio usuário (bin/cue ou CHD) | Sim para compilar/conferir (extrai assets do disco do usuário). O código C é legível sem o disco |
| Regras de contribuição | README: "PRs made with AI will be closed"; issues totalmente escritas por IA são fechadas; ameaça de banimento | `CONTRIBUTING.md`: commits e PRs **não** podem ser enviados por ferramentas autônomas (LLM/agente); aviso prévio obrigatório para iniciantes |

Clones usados (fora do projeto e fora do `/tmp`, que é RAM nesta máquina),
rasos e somente leitura:

```bash
mkdir -p ~/.cache/sotn-research && cd ~/.cache/sotn-research
git clone --depth 1 --filter=blob:limit=2m https://github.com/xeeynamo/sotn-decomp.git
git clone --depth 1 https://github.com/BlackLabelHQ/SymphonyRecomp.git
git -C sotn-decomp checkout 3a0957bee537554b08e830413f3ae2572b552e6e   # para reproduzir
```

Nenhum disco, ROM, ISO, BIN ou CHD foi baixado. Nenhum submódulo foi
inicializado. Nada foi modificado nos clones.

## 2. Licenças e restrições para o Dark Castle RPG

**Fatos:**

- O sotn-decomp é AGPL-3.0. Incorporar código dele (mesmo traduzido para
  Rust linha a linha) tornaria o nosso jogo obra derivada sob AGPL. Além
  disso, o código é uma reinterpretação de um programa da Konami: a AGPL
  cobre o trabalho dos contribuidores, não dá direitos sobre o jogo.
- O SymphonyRecomp não tem licença: não há permissão para reutilizar nada.
- Os dados de tabelas (armas, inimigos, magias, tabelas de EXP) e todos os
  assets são conteúdo do jogo original.

**Regras adotadas neste estudo (e recomendadas para as próximas sessões):**

1. Usar os repositórios só para **entender comportamento** (o que acontece,
   em que ordem, com que proporções).
2. Não copiar funções, estruturas, nomes de campos, tabelas numéricas,
   nomes de itens/inimigos/magias, nem layouts de salas.
3. Valores numéricos citados nos estudos servem para **comparar proporções
   de sensação** (por exemplo, "o pulo sobe 4,9 px/quadro e a gravidade é
   0,17 px/quadro²"). O balanceamento do nosso jogo continua sendo nosso,
   em `assets/data/*.ron` e `src/core/constants.rs`.
4. Não abrir issues nem PRs nesses projetos a partir desta pesquisa.

## 3. Convenções para ler o sotn-decomp

- **Quadros:** a lógica roda uma vez por quadro. Assumimos **60 Hz (NTSC)**.
  Isso é inferência, apoiada por `HandleTransformationMP`
  (`src/dra/5D5BC.c:790-843`), que gasta MP a cada `g_GameTimer % 60`, o
  que corresponde ao "1 MP por segundo" da forma de morcego. Conversões:
  `px/quadro × 60 = px/s`; `px/quadro² × 3600 = px/s²`; `N quadros / 60 = s`.
- **Ponto fixo:** `FIX(x) = x · 65536` (`include/common.h:95`). Velocidades
  são `FIX(px por quadro)`.
- **Nomes provisórios:** `func_XXXXXXXX`, `D_XXXXXXXX` e `unkNN` são nomes
  ainda não identificados pela comunidade. Quando um campo for interpretado
  aqui, a interpretação é **inferência** e está marcada como tal.
- **Tela:** a detecção de colisão descarta entidades fora de
  `x ∈ (-32, 288)` e `y ∈ (-32, 256)` (`src/st/collision.h:243-244`):
  tela de ~256 px de largura (inferência).
- Jogador = `PLAYER` (entidade 0); `g_Player` guarda o estado de controle;
  `g_Status` guarda atributos, equipamento, HP/MP/Hearts.

## 4. Mapa de arquivos relevantes (sotn-decomp @ `3a0957b`)

| Arquivo | Funções / estruturas | Assunto |
|---|---|---|
| `include/player_steps.h:1-48` | `PlayerSteps` | Estados ("steps") do protagonista |
| `include/game.h:882-935` | `Entity` | Campos de colisão: `hitboxOffX/Y`, `hitboxWidth/Height`, `hitboxState`, `hitPoints`, `attack`, `attackElement`, `hitParams`, `hitFlags`, `nFramesInvincibility`, `stunFrames` |
| `include/game.h:1527-1629` | `EnemyDef`, `SubweaponDef`, `Equipment`, `Accessory`, `SpellDef` | Formato dos dados de inimigos, subarmas, armas, armaduras, magias |
| `include/game.h:315-334` | elementos | Elementos como **bits** (um golpe pode ter vários) |
| `include/game.h:1945-1961` | `AluTimers` | Temporizadores do jogador (veneno, maldição, efeito de dano, invencibilidade...) |
| `src/dra/692E8.c:618-1490` | `EntityAlucard` | Laço principal do jogador: timers, entrada, dano recebido, transformações, despacho de estados |
| `src/dra/6FDF8.c:5-147` | `func_8010FDF8` | **Transições permitidas por bits** (cair, virar, pular, atacar, agachar, backdash) e gravidade |
| `src/dra/71830.c:241-765` | `PlayerStepStand/Walk/Jump/Fall` | Locomoção e sub-estados de ataque (`step_s` 0x40–0x5D) |
| `src/dra/71830.c:1146-1644` | `func_80113D7C`, `AlucardHandleDamage` | Recuo ao receber dano, impacto em parede/teto |
| `src/dra/6E42C.c:52-922` | `func_8010E570`, `func_8010E83C`, `func_8010EDB8`, `CheckChainLimit`, `func_8010EB5C`, `PerformHellfire`, `func_8010FD88` | Pouso, pulo, **início de ataque por arma**, limite de instâncias, subarma, início de magia, backdash |
| `src/dra/6DF70.c:79-149` | `func_8010E168`, `CheckMoveDirection`, `func_8010E3B8` | Concessão de invencibilidade, virar, velocidade de recuo |
| `src/dra/704D0.c:5-828` | `Check*Input`, `CheckSpecialMoveInputs` | Reconhecimento de comandos (uma máquina de estados por comando) |
| `src/dra/alu_anim.c:191-214` | `UpdateUnarmedAnim` | **Hitbox por quadro de animação** (`frameProps`) |
| `src/dra/5D5BC.c` | `HandleDamage`, `DealDamage`, `CheckAndDoLevelUp`, `func_800FE044`, `GetEquipProperties`, `GetSpellDef`, `GetStatusAilmentTimer`, `CastSpell`, `LearnSpell`, `func_800FF494` | Dano recebido e causado, level up, EXP, crítico, magia, status, drops |
| `src/dra/menu.c:176-425` | `UpdateEquipStatBonuses`, `CalcAttack`, `CalcDefense`, `make_all` | Atributos totais, ataque e defesa |
| `src/dra/5F60C.c:1363-1381` | HUD | Atraso da animação da barra de HP |
| `src/st/collision.h:119-700` | `HitDetection`, tabelas `g_testColl*` | Colisão hitbox×hurtbox, i-frames por atacante, hitstun, EXP e drops ao matar |
| `src/st/e_bone_scimitar.h:45-334` | `EntityBoneScimitar` | Exemplo de IA de inimigo comum |
| `src/st/st_common.h:61-67` | `PreventEntityFromRespawning` | Inimigo morto não reaparece **enquanto o jogador estiver na sala**; a máscara é zerada a cada carga de sala (Fase 2: `src/dra/5087C.c:1833-1834`) |
| `src/weapon/w_008.c` | `EntityWeaponAttack` | Exemplo de escudo como entidade separada |
| `src/decelerate.h`, `src/set_speed_x.h` | `DecelerateX`, `SetSpeedX` | Frenagem linear e velocidade pela orientação |

SymphonyRecomp @ `2deead5` (só consultado como apoio):

| Arquivo | Uso |
|---|---|
| `README.md:5-13, 33-41, 69-75` | Natureza do projeto, exigência do disco, política contra IA |
| `.gitmodules` | Submódulos `RecompOne` e `mods` |
| `wrapers/States.cs`, `wrapers/Player.cs` | Nomes e endereços de memória em C# que espelham o decomp |

## 5. Discrepância encontrada entre os dois projetos

`wrapers/Player.cs` do SymphonyRecomp chama o endereço `0x8013B5E8` de
`KnockbackInvincibilityAddr`. No sotn-decomp esse endereço é `D_8013B5E8`, e
o único efeito dele é **atrasar a animação da barra de HP no HUD** por 64
quadros depois de mudar o HP (`src/dra/5D5BC.c:570-574` grava 0x40;
`src/dra/5F60C.c:1363-1381` só lê no HUD). A invencibilidade real do jogador
vem de `g_Player.timers[ALU_T_INVINCIBLE]` e `[ALU_T_INVINCIBLE_CONSUMABLES]`
(`src/dra/692E8.c:887-907`). Conclusão: **o rótulo do SymphonyRecomp é uma
hipótese incorreta**; use o decomp como fonte primária.

## 6. Limitações da evidência

- O decomp ainda tem funções sem nome e campos `unk`. As interpretações
  marcadas como "inferência" podem mudar quando a comunidade nomeá-las.
- Os **valores das tabelas** (armas, inimigos, `lockDuration`, `stunFrames`
  de cada arma, curva `g_ExpNext`) ficam em arquivos de dados extraídos do
  disco ou em `.c` de dados que não foram lidos. Este estudo descreve as
  regras, não os números de cada item.
- As overlays de cada arma (`src/weapon/w_*.c`) e de cada chefe
  (`src/boss/`) não foram lidas em profundidade; só uma arma (escudo) e um
  inimigo comum (esqueleto com cimitarra).
- Não houve execução do jogo; nada foi medido em tempo real.
- O SymphonyRecomp não contém lógica de jogo legível (é gerada a partir do
  disco); seu valor para este estudo é pequeno.

## 7. Como continuar a pesquisa

Prioridades sugeridas para uma próxima sessão (detalhes em
[`PLANO_MELHORIAS_SOTN.md`](PLANO_MELHORIAS_SOTN.md) §6):

1. Chefes: `src/boss/` e `src/st/*` (fases, telegráficos, limiar de HP).
2. Sala/câmera/mapa: carregamento de salas, `g_Tilemap`, rolagem e mapa do castelo.
3. Overlays de armas de duas mãos e de lança (`src/weapon/w_051.c`) e de
   escudos (`w_008.c` em diante), para entender guarda e prioridade de hitbox.
4. Onde os bits de `D_80097428` (respawn por sala) são zerados no PS1.

> **Atualização (Fase 2):** os itens 1, 2 e 4 foram investigados
> ([`ESTUDO_SOTN_CHEFES.md`](ESTUDO_SOTN_CHEFES.md),
> [`ESTUDO_SOTN_EXPLORACAO.md`](ESTUDO_SOTN_EXPLORACAO.md)); o item 3 foi
> parcialmente ([`ESTUDO_SOTN_EQUIPAMENTOS.md`](ESTUDO_SOTN_EQUIPAMENTOS.md):
> duas mãos confirmado; escudo parcial; valores por arma ausentes do
> repositório). Classificação final em
> [`PROPOSTA_ROADMAP_POS_V2_4.md`](PROPOSTA_ROADMAP_POS_V2_4.md) §3.
