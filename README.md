# Dark Castle RPG

RPG de ação 2D estilo Metroidvania em **Rust + Bevy 0.20.1**. Um personagem,
combate em tempo real e técnicas/magias executadas por sequências de botões.

## Requisitos

- Rust stable ≥ 1.97.1 (`rustup update stable`)
- Linux: `sudo apt install libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev`
- Windows: Visual Studio Build Tools (MSVC); macOS: Xcode Command Line Tools

## Executar

```bash
cargo run                       # jogar
cargo run --features dev        # compilação incremental mais rápida
cargo run --example autoplay    # roteiro com verificações (saves isolados em target/autoplay/saves/)
cargo build --profile dist      # build de distribuição
```

## Controles

| Ação | Tecla | | Ação | Tecla |
|---|---|---|---|---|
| Mover | Setas | | Guarda | A |
| Correr | Shift | | Esquiva | S |
| Pular | Z | | Status | Tab (1–4 distribuem pontos) |
| Ataque | X | | Salvar / Carregar | F5 / F9 |
| Magia | C | | Levantar após morrer | R |

Comandos do protótipo (olhando para frente):

| Habilidade | Comando |
|---|---|
| Crescent Slash | ↓ ↘ → + X |
| Ember Bolt | ↓ ↘ → + C (8 MP) |
| Mending Light | ↓ (soltar) ↓ + C (15 MP) |
| Rising Edge | ↓ ↑ + X (2 Hearts, aprendida no nível 3) |

## Desenvolvimento

```bash
cargo fmt --check && cargo clippy --all-targets && cargo test && cargo check
```

- Regras do projeto e fluxo de trabalho com Claude Code: [`CLAUDE.md`](CLAUDE.md)
- Arquitetura: [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) · Roadmap: [`docs/ROADMAP.md`](docs/ROADMAP.md)
- Conteúdo (habilidades, itens, inimigos, salas) em `assets/data/` e `assets/maps/`, formato RON

## Estrutura

```
src/
  app/        montagem do app, estados, ordem dos sistemas
  core/       constantes, mensagens, dados carregados, RNG, tempo
  input/      mapeamento de teclas → PlayerInput
  player/     atributos, progressão, máquina de estados, movimento
  combat/     buffer/parser de comandos, dano, hitboxes, ataques
  skills/     definições, aprendizado, cooldowns, validação
  magic/      elementos, magias, projéteis
  inventory/  itens, raridades, inventário, equipamentos
  enemies/    definições, IA, comportamento, spawn
  world/      salas, colisão, física, câmera, progresso
  ui/         HUD, tela de status, números de dano
  save/       formato versionado, armazenamento, F5/F9
tests/        testes de integração e ECS headless
examples/     autoplay (teste de fumaça visual)
docs/         documentação técnica
assets/       dados RON e mapas
```
