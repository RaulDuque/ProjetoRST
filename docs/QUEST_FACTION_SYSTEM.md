# Missões e facções

> **[Planejado]**, nada implementado na V1.6. Hoje só existem flags livres
> em `WorldProgress.flags`, salvas no save v1 e usadas pelos requisitos de
> habilidades. O protótipo usa uma flag (`vigil_gates_cleared`) concedida
> pelo Sigilo do Guardião.

## 1. Missão (formato proposto)

```ron
(
    id: "lost_scout",
    name: "The Lost Scout",
    giver: "corvin_scout",
    available_if: Some(All([Flag("vauvrel_reached"), Flag("vigil_gates_cleared")])),
    objectives: [
        (id: "find", kind: VisitRoom("cisterns_ledge")),
        (id: "recover", kind: CollectPickup("scout_satchel")),
        (id: "return", kind: TalkTo("corvin_scout")),
    ],
    rewards: [Currency(300), Item("scout_map_cisterns"), Flag("scout_saved")],
    fails_if: Some(Flag("cisterns_flooded")),
    exclusive_with: [],
)
```

- **Objetivos:** reutilizam os sinais que já existem ou estão planejados:
  - visitar sala: `ExplorationState`;
  - coletar: `pickups`;
  - derrotar chefe: `defeated_bosses`;
  - falar: serviço do NPC;
  - flag: `WorldProgress`.
- **Sem scripts:** só condições tipadas, iguais aos requisitos do mundo.

## 2. Estados

```text
Locked ──(available_if)──▶ Available ──(aceitar)──▶ Active ──(objetivos)──▶ Completed
                                                       │
                                                       └──(fails_if)──▶ Failed
```

- **Recalculado:** `Locked` e `Available`, a partir das condições; não são salvos.
- **Salvos:** `Active` (com os objetivos feitos), `Completed` e `Failed`.
- **Dependências:** `available_if` pode exigir `Flag("quest:<id>:completed")`,
  uma flag escrita ao completar.
- **Validador:** detecta ciclos de dependência entre missões, objetivos que
  apontam para salas, coletas ou NPCs inexistentes, e recompensas inexistentes.

## 3. Facções e reputação (opcional, futuro)

- **[Proposta]:** 3 facções (Vigília de Vauvrel, Ordem do Arquivo, Ferrugem —
  mercenários das Casernas), com reputação de −100 a 100 por facção.
- **Efeitos:** preços condicionais, NPCs que aparecem ou somem, missões exclusivas.
- **Persistência:** `reputation: BTreeMap<FactionId, i32>` no save.
- Só depois da versão de missões e narrativa (item futuro, sem número;
  `GAME_VISION_MASTER.md` §6), se o design pedir.
