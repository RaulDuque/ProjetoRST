# Entrada e reconhecimento de comandos

## Teclas padrão (`src/input/mod.rs`, `InputBindings`)

| Ação | Tecla |
|---|---|
| Mover / direções de comando | Setas |
| Correr | Shift esquerdo |
| Pular | Z |
| Ataque | X |
| Magia | C |
| Guarda (segurar) / cancelar comando | A |
| Esquiva | S |
| Tela de status | Tab |
| Distribuir pontos (na tela de status) | 1–4 |
| Levantar após morrer | R |
| Salvar / carregar | F5 / F9 |

Agachar: segurar ↓ no chão. Os sistemas de jogo leem `PlayerInput`, nunca as teclas.
Suporte a gamepad: roadmap.

## Notação de comandos

Escrita como se o personagem olhasse para a direita; tokens separados por espaço, vírgula ou `+`:

| Token | Significado |
|---|---|
| `U` `↑` · `D` `↓` · `F` `→` · `B` `←` | cima, baixo, frente, trás |
| `UF` `↗` · `UB` `↖` · `DF` `↘` · `DB` `↙` | diagonais |
| `N` | neutro (soltar o direcional) |
| `ATTACK`/`ATK` · `MAGIC`/`MAG` · `JUMP` · `GUARD` | botão final (obrigatório, último) |

Exemplos: `"D DF F + ATTACK"`, `"↓ ↘ → + MAGIC"`, `"D N D + MAGIC"` (dois toques para baixo).
A interface mostra a forma ASCII porque a fonte padrão não tem setas.

## Buffer (`src/combat/command_buffer.rs`)

- Cada mudança de posição do direcional vira um `TimedInput { token, time }` **relativo à orientação** (frente/trás).
- Segurar uma direção grava uma única entrada.
- Capacidade 32, idade máxima 1,5 s (`prune`).
- Toda pressão de ATAQUE/MAGIA é resolvida e **limpa o buffer**: as mesmas entradas nunca disparam duas vezes.
- Apertar GUARDA limpa o buffer (cancelamento).

## Reconhecimento (`src/combat/command_parser.rs`)

1. A última entrada precisa ser o botão do comando.
2. As direções são casadas de trás para frente como subsequência, tolerando até
   `max_extra_inputs` entradas estranhas (variações de execução). Neutros são
   ignorados, exceto se o comando usa `N`. Nunca se olha além de um botão anterior.
3. Tempo: `total_window` (do primeiro direcional ao botão) e `step_window` (entre entradas).
4. Vários comandos casando → maior `priority`, depois sequência mais longa.

Classificação (`Recognition`) → validação (`skills/execution.rs`) → `CommandOutcome`:

| Resultado | Quando |
|---|---|
| `Executed` | sequência certa, no tempo, aprendida, sem cooldown, com recursos, estado permitido |
| `Incomplete` | pelo menos metade das direções finais certas, faltando as primeiras |
| `Invalid` | não corresponde a nenhum comando (ATAQUE vira ataque básico) |
| `TooSlow` | sequência certa fora da janela |
| `InsufficientResources` | falta MP ou Hearts |
| `NotLearned` | comando existe mas não foi aprendido |
| `OnCooldown` | aprendido mas recarregando |
| `NotAllowedNow` | estado atual não permite (`usable_in`) |

Todos os casos têm testes em `tests/command_parser_tests.rs`. O HUD exibe o resultado (canto superior direito).

O `Executed` do reconhecedor é provisório: ele usa o estado do frame anterior e
só emite o pedido `SkillActivated`. `skills/systems.rs::begin_skill` revalida
com o estado atual e emite o resultado definitivo (`Executed`, ou o motivo da
rejeição, por exemplo `NotAllowedNow` depois de um pulo ou esquiva no mesmo
frame). Cada tentativa gera um único `CommandResolved`; os demais resultados
continuam vindo do reconhecedor (ver `COMBAT_SYSTEM.md`).
