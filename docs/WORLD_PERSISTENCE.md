# Persistência do mundo

> - **[Implementado]:** save v1 (sem mudança na V1.6), em [`SAVE_SYSTEM.md`](SAVE_SYSTEM.md).
> - **[Protótipo]:** arquivo de exploração independente (`ExplorationState`, versão 1).
> - **[Planejado]:** save v2 com migração. O formato do save real **não** mudou e nada foi gravado em `saves/`.

## 1. O que é salvo hoje (v1)

| Dado | Campo |
|---|---|
| Sala e posição | `player.room`, `player.position` |
| Atributos, nível, EXP, HP/MP/Hearts | `player.*` |
| Habilidades aprendidas | `player.learned_skills` |
| Inventário e equipamento | `player.inventory`, `player.equipment` |
| Mundo | `world: WorldProgress` (portas desbloqueadas, baús, pontos de save, chefes, flags, último save) |
| Tempo de jogo | `play_time` |

## 2. Protótipo: arquivo de exploração

- **Formato:** `ExplorationState::to_ron` grava `(version: 1, state: (...))` com:
  - descoberta por sala (`rooms: {id: Unknown | Discovered | Visited}`);
  - `secrets`, `pickups`, `granted` (concessões em ordem);
  - `current_room`, `previous_room`.
- **Onde é gravado:** só em `target/` (`target/world_vertical_slice/exploration.ron`
  no exemplo, `CARGO_TARGET_TMPDIR` nos testes).
- **Versão:** uma versão desconhecida é recusada.
- **Testado:** o estado sobrevive a uma reinicialização. A barreira continua
  aberta, o segredo continua encontrado e a coleta não reaparece.

## 3. Contrato do save v2 [Implementado na V1.7, em parte]

**Implementado na V1.7:** `version: 2` com `exploration: ExplorationRecord`
(`rooms`, `secrets`, `pickups`, `granted`).

**Diferenças em relação ao contrato abaixo:**
- **Migração v1:** usa `SaveFileV1` congelado (`deny_unknown_fields`) e
  `upgrade_v1`. A exploração começa vazia, e a sala do save passa a
  `Visited` ao ser carregada.
- **Backup:** há um backup permanente `slotN.ron.vN.bak` antes da primeira
  gravação v2.
- **Sala desconhecida:** volta à porta inicial do mundo, com aviso
  (`world::restore`).
- **Campos ainda não implementados:** `unlocked_forms`, `currency` e
  `quests`.

Detalhes em [`WORLD_INTEGRATION_V1_7.md`](WORLD_INTEGRATION_V1_7.md) §8–§10.

Contrato original (V1.6):

```text
SaveFile { version: 2, data: SaveData {
    player: PlayerSave {
        room, position, ...v1...,
        unlocked_forms: BTreeSet<Ability>,          // V1.9
        currency: u64,                              // V2.1
    },
    world: WorldProgress { ...v1... },              // portas, baús, chefes, flags: mantidos
    exploration: ExplorationState,                  // novo: mapa, segredos, coletas, concessões
    quests: BTreeMap<QuestId, QuestState>,          // V2.4 (vazio antes)
    play_time,
}}
```

**Regras do contrato:**
1. **Só ids estáveis:** nunca entidades, índices ou posições, exceto a do jogador.
2. **Campos novos com default:** `#[serde(default)]`, para que v1 → v2 seja só
   preencher padrões.
3. **Migração v1 → v2 (`save::serialization::migrate`, que já tem o passo a passo):**
   - `exploration` vazio, com a sala atual como `Visited`;
   - `world` copiado;
   - slots `Accessory` renomeados se a V2.0 mudar os slots ([`EQUIPMENT_VISUAL_SYSTEM.md`](EQUIPMENT_VISUAL_SYSTEM.md)).
4. **Ids desconhecidos na carga** (conteúdo removido): ignorados com log, sem
   erro. A porcentagem já ignora salas que não existem (testado).
5. **Sala inexistente na carga:** volta à sala inicial do mundo, com aviso.
   Hoje o v1 recusa a carga.
6. **Dois lugares para concessões:** `granted` (exploração) e `learned_skills`
   (técnicas e magias) continuam separados. Uma relíquia que ensina uma magia
   grava nos dois, sempre pelo sistema dono.
7. **Teste obrigatório:** um save v1 real de exemplo (fixture em `tests/`, nunca em `saves/`) carrega no v2.

## 3.1 Objetos interativos (V1.8)

O save v2 não mudou. Os objetos são guardados nos campos que já existiam:

| Objeto | Campo |
|---|---|
| Parede quebrada, interruptor encontrado | `exploration.secrets` |
| Baú aberto | `exploration.pickups` (recompensas de uma vez) |
| Alavanca puxada | `exploration.granted` (`Flag`) e `world.flags` |

- **Fixture V2:** `tests/fixtures/save_v2_galleries.ron`, sintético.
- **Save histórico real:** a compatibilidade ainda não foi comprovada, porque
  os saves reais não foram lidos.

## 3.2 Transformações (V1.9)

- **Schema:** não mudou.
- **Desbloqueio:** a Forma Espectral é liberada pela relíquia, que já estava
  em `exploration.granted` (`Ability(VeilStep)`), então persiste.
- **Forma ativa:** nunca é salva; o load volta sempre humano.
- **Save recusado:** F5 é recusado ("Cannot save inside a veil") enquanto o
  corpo humano não couber onde o jogador está, e nada é gravado.
- **Load:** a restauração valida a posição contra os sólidos da sala, véus
  incluídos (`PositionReset` com aviso).
- **Detalhes:** [`TRANSFORMATIONS_V1_9.md`](TRANSFORMATIONS_V1_9.md) §17.

## 4. O que fica fora do save

- Inimigos (reaparecem ao entrar na sala).
- Projéteis, hitboxes, efeitos.
- A forma ativa (carrega sempre humano).
- `Locked` e `Available` de missões (recalculados).
