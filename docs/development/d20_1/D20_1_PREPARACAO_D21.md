# D20.1 — Preparação D21 (primeiro chefe)

Base: `docs/development/d20/D20_PREPARACAO_CHEFES.md` + validação visual desta etapa.

## Reutilizável (confirmado jogável)

- Spawn por sala + limpeza em transição  
- `AiStyle` melee / ranged / keep_distance (dados RON)  
- Projéteis inimigos (Bone Bolt)  
- Hitboxes, dano, knockback, `EnemyDefeated` → EXP/drops  
- `TransitionLock` / requisitos de porta  
- Save V3 + altares + fast travel (não misturar com morte de chefe)  

## Recomendação mínima D21

1. **Dados:** flag `boss: true` (ou `kind: Boss`) + id estável.  
2. **Persistência:** sem respawn; gravar derrota em progresso/save.  
3. **Fases:** tabela `(hp_ratio, AiProfile)` no RON — sem FSM nova no núcleo.  
4. **Arena:** trancar portas enquanto vivo (`TransitionLock` + requisito).  
5. **UI:** barra de HP Presentation só para entidade boss.  
6. **Arena plana** no primeiro chefe — evitar ledge (vide D201-02).  
7. **Testes:** headless (fases, persistência, sem duplicar ao reload) + um probe de janela curto.  
8. **Não** acoplar descanso de altar / fast travel à luta.

## Ordem sugerida

Arena + flag persistente → um padrão de ataque → barra de HP → fases → recompensa → polish.

**Não implementar o chefe na D20.1.**
