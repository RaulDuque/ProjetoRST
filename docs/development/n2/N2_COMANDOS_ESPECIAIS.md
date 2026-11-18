# N2 — Comandos especiais

`command_parser.rs` não foi alterado. `command_buffer.rs` também não. `record_and_resolve_commands` continua recebendo `PlayerInput` e gravando `Motion::from_axes`.

`axis_y` positivo é `AimUp`. No híbrido isso é a seta para cima. W só liga `jump_pressed` / `jump_held`. O teste `aim_down_reaches_the_command_buffer_and_jump_does_not` segura S e vê `Motion::Down`, segura W e não vê `Motion::Up`, e a seta para cima grava `Motion::Up`.

As sequências iniciais continuam as mesmas, relativas à frente:

- Crescent Slash: `↓ ↘ → + ATTACK`
- Ember Bolt: `↓ ↘ → + MAGIC`
- Mending Light: `↓ N ↓ + MAGIC`
- Rising Edge, no nível 3: `↓ ↑ + ATTACK`

No híbrido, o "baixo" é S e o ataque final é o mouse ou X. A magia final continua C. Nada disso foi removido nem teve a janela de tempo alterada.

Dois controles da mesma ação não disparam o botão duas vezes: o buffer só vê `attack_pressed` ou `magic_pressed`, e cada um já é a borda combinada.
