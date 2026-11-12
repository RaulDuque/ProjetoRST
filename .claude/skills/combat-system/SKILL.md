---
name: combat-system
description: Altera ou estende o combate do Dark Castle RPG — reconhecimento de comandos, técnicas, magias, hitboxes, dano, estados do personagem e consumo de MP/Hearts. Use para qualquer mudança em src/combat, src/skills, src/magic ou nos RON de skills/spells.
argument-hint: "<mudança de combate>"
---

# /combat-system

Mudança pedida: $ARGUMENTS

## 1. Analisar o combate existente
Leia, na ordem:
- `docs/COMBAT_SYSTEM.md`, `docs/SKILL_SYSTEM.md`, `docs/MAGIC_SYSTEM.md`, `docs/INPUT_SYSTEM.md`;
- pipeline: `combat/command_input.rs` → `skills/execution.rs::resolve_command` → pedido `SkillActivated` (o ataque básico vai por `BasicAttackRequested` direto a `basic_attack`) → `skills/systems.rs::begin_skill` (revalida com o estado atual; se confirmar: estado, custos, cooldown, `SkillStarted` + `CommandResolved`; se rejeitar: só `CommandResolved` com o motivo) → `SkillStarted` → efeito em `combat/attacks.rs` (físico) ou `magic/casting.rs` (magia) → `combat/hitboxes.rs::detect_hits` → `DamageDealt` → `receive_damage` do alvo.

## 2. Verificar colisões e hitboxes
- Hitbox: tamanho, offset (`AttachedToOwner`), `lifetime`, `pierce`, time (`Team`).
- Hurtbox do alvo e `Invulnerable` (esquiva, pós-dano).
- Projéteis somem ao bater em sólidos (`magic/projectiles.rs`).

## 3. Verificar estados do personagem
- `player/state.rs`: a ação nova é permitida em quais estados? Use `ActionPhase` no campo `usable_in` do RON.
- Garanta que morte/dano interrompem a ação e que nada roda em `Dead`.

## 4. Verificar comandos de habilidades
- Notação em `assets/data/skills|spells/*.ron` (`"D DF F + ATTACK"`).
- Conflitos: o teste `skill_commands_are_unique_per_button` impede comandos idênticos. Para comandos parecidos, ajuste `priority` (maior vence; empate → sequência mais longa).
- Janelas: `timing: (total_window, step_window, max_extra_inputs)`.
- Nunca coloque efeito dentro do parser nem crie `if skill_id == ...` no motor.

## 5. Verificar consumo de recursos
- `mp_cost`, `hearts_cost`, `cooldown` validados em `validate_activation` e pagos em `begin_skill` (no mesmo frame).
- Cooldowns aparecem no HUD automaticamente.

## 6. Implementar alterações isoladas
- Uma mudança por vez; regra pura primeiro, com teste.
- Novos tipos de efeito: nova variante em `SkillEffect` + sistema executor no módulo dono (combat ou magic).

## 7. Testar combos e danos
- `cargo test --test command_parser_tests --test combat_tests --test app_smoke_tests`.
- Novo comando: teste dos 7 resultados relevantes (executado, incompleto, inválido, lento, sem recurso, não aprendido, cooldown).
- Fórmula alterada: atualize `docs/COMBAT_SYSTEM.md` e os testes numéricos.
- Com tela: `cargo run --example autoplay`. Ele salva só em `target/autoplay/saves/` e termina com código ≠ 0 se uma verificação obrigatória falhar; confira as linhas `[autoplay] PASS/FAIL` e `RESULT:`. Se mudar o comportamento de uma habilidade coberta, ajuste a verificação correspondente em `examples/autoplay.rs`.
- Finalize com `/test-and-validate`.
