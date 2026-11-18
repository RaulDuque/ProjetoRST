# N2 — Arquitetura

`src/input/` separa a regra da leitura.

| Módulo | Papel |
|---|---|
| `action` | `Action` e o contexto (jogo ou menu). Sem string solta. |
| `binding` | Tecla ou botão do mouse que a tela pode capturar. Escape não está na lista. |
| `edges` | `combine`: vários controles de uma ação viram um pressionar, segurar ou soltar. Soltar e apertar de novo o mesmo controle antes da leitura conta como outro toque. Apertar o segundo enquanto o primeiro segue segurado não conta. Puro, sem Bevy. |
| `profile` | Slots, recusa de conflito, ação essencial, validação do arquivo. |
| `presets` | Híbrido e clássico. São funções que devolvem um perfil novo. Editar não altera a função. |
| `store` | Caminho do usuário, leitura, escrita atômica. |
| `mod` | Plugin. `read_controls` roda todo frame, também com o jogo pausado. `publish_player_input` só roda em `GameState::Playing`. |

`PlayerInput` manteve os campos que o resto do jogo já usava. `axis_y` é mira, não pulo. O pulo é `jump_pressed` / `jump_held`.

A máscara da forma espectral continua em `transform::systems::mask_form_input`, agora depois de `publish_player_input`.

Não entrou crate nova. `KeyCode` do Bevy não é o tipo gravado: o arquivo usa o enum local, que já serializa com o `serde` do projeto.

Gamepad: não há campo, sistema nem botão. O ponto de extensão é `combine`. Outro dispositivo teria de dizer se a ação está segurada. A borda continua a mesma.
