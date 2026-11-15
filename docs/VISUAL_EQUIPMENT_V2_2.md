# V2.2 — Equipamento visível, NPCs, mapa e polimento da Vila das Cinzas

> Estado: **concluído** (V2.2 + autorização `DarkCastleR-V2-2-a.md`). A arte
> sem sabre e as âncoras foram publicadas em 2026-11-15 pelo guarda (estado
> `1cd66a2977d0`, backup do estado anterior `c670bdc45f61` em
> `art-src/player/renders/publish_backups/20261115T135518275682_c670bdc45f61/`).

## 1. Objetivo

Fazer a aparência do protagonista e dos NPCs refletir os sistemas que já
existiam (equipamento, duas mãos, NPCs, lojas, mapa), sem mudar combate,
inventário, diálogos, progresso ou o Save v3.

## 2. Arquitetura

| Camada | Arquivo | Papel |
|---|---|---|
| Pura | `src/player/visual/sockets.rs` | Tabela de âncoras por quadro: parse, validação contra o manifesto, espelhamento |
| Pura | `src/inventory/held.rs` | O que as mãos seguram (`HeldLook`): forma da arma, cor, duas mãos, posição do escudo, pose de repouso |
| Pura | `src/inventory/held_art.rs` | Arte provisória das 5 silhuetas e do escudo |
| Pura | `src/core/pixel_art.rs` | Rasterizador de polígonos com contorno (armas e NPCs) |
| Pura | `src/npc/art.rs` | Figuras dos NPCs a partir de `look` |
| Bevy | `src/inventory/held_visual.rs` | `HeldEquipment` + camadas `HeldLayer::{Weapon, Shield}` |
| Bevy | `src/core/pixel_image.rs` | `PixelArt` → `Image` (filtro *nearest*) |
| Bevy | `src/player/visual/{mod,systems}.rs` | `PlayerArtSource`, carga das âncoras, `PlayerSpriteSet::held_layers_enabled` |
| Bevy | `src/inventory/visual.rs` | Capa pendurada na âncora "costas" |
| Bevy | `src/npc/systems.rs`, `src/ui/world_map.rs`, `src/ui/hud.rs` | Figuras, marcadores, HUD |
| Arte | `art-src/player/scripts/bpy/build_game_sprites_v2_2.py` | Corpo sem sabre + medição das âncoras no Blender |
| Arte | `art-src/player/scripts/post/game_sprites_v2_2.py` | Staging da candidata (atlas, manifesto, âncoras) |
| Arte | `art-src/player/scripts/post/publish_guard.py` | Aceita `weapon_in_body` e publica/valida `player_sockets.ron` |

Ordem no frame (`Presentation`): `apply_equip_requests → sync_appearance →
sync_held_look` (antes do visual) e, no `PlayerVisualSystems`, depois de
`animate_player_visual`: `sync_cape_layer` e `sync_held_layers`; depois
`transform::visual::tint_form` (as camadas também ficam espectrais).

## 3. Problema da espada fixa

- **Por quê:** os masters V1.3, V1.4 e V1.5 foram renderizados com o sabre
  ×1,7 (`blade_readable`) preso ao osso `sword` da mão direita; o post-script
  empacotou corpo e sabre na mesma imagem.
- **Onde:** os 122 quadros de `player_locomotion.png` (64) e
  `player_combat.png` (58); os efeitos (`player_effects.png`) não têm sabre.
- **Como o jogo sabia a arma:** só pelo item no slot `Weapon` (golpe, rastro);
  o sprite nunca dependeu dele.
- **Assets publicados/protegidos:** os três PNG e o `player.ron` em
  `assets/`, travados por `assets/animations/player.publish.json` (guarda de
  publicação). Não foram alterados.

## 4. Solução adotada

1. **Na origem (Blender):** `build_game_sprites_v2_2.py` abre
   `swordsman_rig_v007.blend` só para leitura, reconstrói as ações com os
   módulos de clipes V1.3/V1.4/V1.5 **sem alteração**, compara o hash dos
   keyframes de cada clipe com o do build original (todos os 22 clipes
   conferem) e renderiza o acabamento C com os seis objetos do sabre ocultos
   (`blade`, `blade_readable`, `sword_collar`, `sword_guard`, `sword_grip`,
   `sword_pommel`). A mão continua fechada (empunhadura vazia).
2. **Staging:** `game_sprites_v2_2.py` monta os atlas no **layout publicado
   idêntico** (mesmos clipes, índices, durações e pivôs; verificado linha a
   linha), copia os efeitos (hash idêntico ao publicado) e grava
   `weapon_in_body: false` e `player_sockets.ron` em
   `art-src/player/renders/v2_2_weaponless/stage/`.
3. **Validação isolada:** `publish_guard.py check --stage ...` → *publishable:
   21 clips, new state 1cd66a2977d0*; folha
   `art-src/player/comparisons/v2_2_weaponless/published_vs_weaponless.png`
   (publicado | sem arma | sem arma + âncoras) inspecionada; a vitrine
   `equipment_visual_showcase` roda o jogo com a candidata (§12).

**Publicação (autorizada, feita em 2026-11-15).** Arquivos substituídos em
`assets/` (com backup e rollback automáticos do guarda):

| Arquivo | Muda |
|---|---|
| `sprites/player/player_locomotion.png` | Corpo sem sabre (mesmas dimensões, 1038×1038) |
| `sprites/player/player_combat.png` | Corpo sem sabre (1804×1744) |
| `animations/player.ron` | + `weapon_in_body: false` e cabeçalho V2.2 |
| `animations/player_sockets.ron` | **Novo** |
| `animations/player.publish.json` | Lock regravado pelo guarda |
| `sprites/player/player_effects.png` | Não muda (mesmo hash) |

Comando usado: `python3 art-src/player/scripts/post/publish_guard.py publish --stage art-src/player/renders/v2_2_weaponless/stage --confirm c670bdc45f61`.
Reverter: o guarda guarda o backup em `art-src/player/renders/publish_backups/`.

## 5. Sistema de âncoras

`animations/player_sockets.ron` (versão 1), um registro por quadro de cada
clipe de personagem:

| Campo | Tipo | Unidade | Padrão | Quando ausente |
|---|---|---|---|---|
| `weapon.x`, `.y` | `f32` | u (= px do contrato) a partir do pivô da sola; x à frente, y para cima | — | Clipe sem âncoras: camadas ocultas nesse quadro |
| `weapon.angle` | `f32` | graus, anti-horário, pomo → guarda | — | idem |
| `weapon.front`, `hand_l.front` | `bool` | mão à frente do peito (câmera) | — | idem |
| `hand_l`, `head`, `chest`, `back` | ponto | u | — | idem |

- **Medição:** projeção ortográfica na mesma câmera dos masters (centroides
  das malhas deformadas do punho, pomo e colar do sabre; pontos dos ossos
  `hand.L`, `head`, `chest`, `collar_back`).
- **Compatibilidade:** arquivo **opcional**. Manifestos antigos não têm
  `weapon_in_body` (padrão `true` = sabre no corpo): as camadas ficam
  desligadas e nada muda. Tabela inválida (versão, clipe desconhecido,
  contagem de quadros, valor não finito ou a mais de 128 u do pivô) é
  recusada e registrada no log: camadas desligadas.
- **Save:** nada novo; as posições são derivadas da arte.
- **Movimentos cobertos:** idle, corrida (= caminhada: não há clipe de
  caminhada), pulo, queda, aterrissagem, agachar, ataque (chão e ar),
  técnicas, magias, guarda, esquiva, dano, morte; mudança de direção por
  espelhamento; transição de sala sem efeito (as camadas são filhas do jogador).

## 6. Armas visíveis

Uma silhueta por categoria (`held_art.rs`), cor de destaque = cor do `Trail`
do item (latão sem `visual`):

| Categoria | Silhueta | Comprimento à frente da mão |
|---|---|---|
| Sword | Lâmina curva de uma mão | ~31 u |
| Dagger | Folha curta e larga, guarda pequena (não é espada reduzida) | ~13 u |
| Axe | Cabo de madeira, cabeça barbada | ~22 u |
| Spear | Haste segurada no meio, ponta em folha | ~44 u (25 u atrás) |
| HeavyWeapon | Montante largo, cabo longo de duas mãos | ~40 u |

- Profundidade: à frente do corpo quando a mão direita está à frente do peito
  (`FRONT_Z`), senão atrás (`BEHIND_Z`), sempre à frente da capa.
- **Pose de repouso:** em clipes que não são de golpe, lança e arma pesada
  apontando para baixo são giradas 180° (lança de pé, montante no ombro) para
  não atravessarem o chão. Nos clipes de golpe (atlas `combat`) seguem o
  sabre, sem saltos no meio do movimento.
- Pisca com o corpo (invulnerabilidade) e fica espectral com a forma.

## 7. Escudos e regra de duas mãos

- Escudo no braço esquerdo, desenhado à frente do corpo e atrás da arma
  (`SHIELD_Z`): de lado, o braço esquerdo fica atrás do tronco e esconderia o
  escudo.
- Cor do escudo: `visual: Some(Tint(...))` agora vale também para `Shield`
  (Watch Buckler e Rune Ward ganharam cor).
- Regras de equipar inalteradas (V2.1). Visual: uma arma de duas mãos nunca
  aparece com escudo na mão; se um save editado tiver os dois, o escudo vai
  para as costas (a inconsistência fica visível, não é escondida).

## 8. Armaduras

Os oito slots continuam iguais. Corpo: `Tint` (tom) ou `Cape` (capa, agora
pendurada na âncora "costas" e piscando com o corpo). Cabeça: `Tint`. Luvas,
botas, acessórios: sem arte própria (só atributos), como na V2.0 — não há
elmo, luva ou bota desenhados.

## 9. NPCs

Campo opcional `look: Some((outfit, headwear, prop, accent))` em
`assets/data/npcs/*.ron` (padrões: `headwear: Bare`, `prop: None`;
`deny_unknown_fields`; cor validada). A figura (gerada em código, provisória)
tem o centro no mesmo ponto do retângulo antigo: alcance do **D**, aviso
"D Talk", prioridade dos baús, falas, loja e save inalterados. Os cinco NPCs
da vila receberam `look` (tabela no tutorial 27).

## 10. Marcadores do mapa

Em salas **visitadas** (mesma política dos baús): violeta = NPC com loja,
rosa = outro NPC; um por NPC, sempre dentro da caixa da sala
(`marker_spot`); legenda sob o mapa quando há marcadores. Sem marcador de
ferreiro (Garrick existe, mas não há serviço de ferreiro).

## 11. Pipeline artístico

```bash
for m in v1_3 v1_4 v1_5; do
  blender --background --factory-startup art-src/player/blender/swordsman_rig_v007.blend \
    --python art-src/player/scripts/bpy/build_game_sprites_v2_2.py -- --module $m --overwrite
done
python3 art-src/player/scripts/post/game_sprites_v2_2.py
python3 art-src/player/scripts/post/publish_guard.py check --stage art-src/player/renders/v2_2_weaponless/stage
```

Blender 5.2.4, ~6 s por quadro, 135 quadros renderizados (os 122 do jogo
mais clipes de origem). Saídas em `renders/` (ignorado pelo Git).

## 12. Compatibilidade

- Save v3 sem mudança; F5/F9 restauram a aparência (resolvida dos slots).
- Manifestos e dados antigos continuam válidos (campos novos opcionais).
- Rodar com a candidata sem publicar: `app::build_with_candidate_art`
  registra o staging como fonte de assets `candidate://` e `PlayerArtSource`;
  só a arte do jogador vem de lá.

## 13. Testes

- Unitários: âncoras (parse, contagem, valores, espelhamento), `weapon_in_body`
  padrão, formas, duas mãos, pose de repouso, silhuetas distinguíveis, grip
  opaco, contorno, figuras de NPC, marcadores dentro da sala, posição das
  notificações por estado.
- `tests/held_visual_tests.rs`: arte publicada desliga as camadas; staging
  de teste: camada segue item e quadro, troca no mesmo entity, desequipar
  remove, espelhamento, escudo/duas mãos/save inconsistente, clipe sem
  âncoras oculta, F5/F9, perfil de combate igual com e sem camadas, custo.
- Python: 4 testes novos do guarda (linha opcional, publicação da tabela,
  tabela ausente/incompleta, rollback remove a tabela nova).
- Com janela: `equipment_visual_showcase` (candidata e `--published`).

## 14. Performance

Headless, perfil dev, mesma cena (espada + escudo + capa, 300 quadros):

| Medida | Com camadas (candidata) | Sem camadas (publicada) |
|---|---|---|
| Tempo médio por frame | 0,346 ms | 0,360 ms |
| Entidades vivas | 150 | 147 |
| Imagens geradas para as mãos | 2 | 0 |

A diferença de tempo está dentro do ruído. As camadas são 2 entidades,
criadas uma vez e atualizadas no lugar (o teste confere que não são
recriadas). Figuras de NPC: 1 imagem por NPC, criada ao entrar na sala pela
primeira vez e reaproveitada. Mesmo cenário da V2.1 (`town_app_tests`,
`target/tmp/town_v21_perf.txt`): frame comum 0,376 ms (V2.1: 0,26 ms), loja
aberta 0,202 ms (0,19), troca de sala com NPC 2,07 ms (1,76). Esses números
variam entre execuções na mesma máquina; não foi medida memória máxima.

## 15. Limitações

- Arte provisória gerada em código (armas, escudo, NPCs); nada de arte final.
- Uma silhueta por categoria (não por item); duas mãos não muda a pose do
  braço esquerdo (a arte só tem a mão direita segurando).
- Sem elmo/luvas/botas desenhados; NPCs sem animação.
- Não há clipe de caminhada (a corrida cobre o andar).

## 16. Pendências futuras

Arte final por categoria/item; pose de
duas mãos no Blender; elmos como camada na âncora "cabeça"; animação de NPCs;
ícones dedicados no mapa.
