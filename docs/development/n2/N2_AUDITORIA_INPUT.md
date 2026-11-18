# N2 — Auditoria da entrada

Antes da N2, `InputBindings` guardava uma `KeyCode` por ação e `read_keyboard` enchia `PlayerInput`. Combate, movimento e forma já liam `PlayerInput`. Estes caminhos ainda liam a tecla crua, então um binding alternativo ou o mouse não chegava neles:

- `save/systems.rs`: F5 e F9
- `ui/inventory_screen.rs`: abrir, fechar, subir, descer, confirmar
- `ui/status_screen.rs`: abrir e fechar (as teclas 1–4 de atributo continuam cruas de propósito)
- `ui/world_map.rs`: abrir o mapa
- `ui/shop_screen.rs`: abas, seleção e compra
- `ui/fast_travel_screen.rs`: seleção e viagem
- `npc/systems.rs`: diálogo (confirmar ou interagir)
- `ui/hud.rs`: a dica era um texto fixo do esquema antigo

O buffer de comandos (`command_input.rs`) já recebia `axis_x` e `axis_y` como inteiros e os convertia em frente/trás pela orientação. O parser não conhece tecla. W, se virasse "cima", mudaria Rising Edge e as diagonais sem o jogador ter pedido isso.

Os testes de rota e os exemplos que injetam tecla usam setas, Z, X e D. Trocar o padrão do jogo sem fixar o clássico nesses programas quebraria o contrato deles. `play_before_boss` é sessão humana e não foi fixado.

O save está na versão 3. Os bindings não tinham campo nele, e continuam sem campo.
