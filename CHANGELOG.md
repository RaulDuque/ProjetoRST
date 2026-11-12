# Changelog

Formato: [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/). Versionamento semântico.

## [Unreleased]

### Adicionado
- Projeto Cargo `dark_castle_rpg` com Bevy 0.20.1 (2D/UI/áudio), perfis dev/release/dist.
- Regras puras testadas: atributos e fórmulas derivadas, progressão, máquina de estados,
  dano, buffer e parser de comandos, habilidades orientadas a dados, inventário e
  equipamentos, colisão AABB, IA de inimigos, formato de save versionado.
- Protótipo jogável: sala de teste, personagem com andar/correr/pular/agachar/guarda/esquiva,
  ataque básico, Crescent Slash, Rising Edge, Ember Bolt, Mending Light, inimigo Bone Sentry,
  EXP/level up/drops, HUD, tela de status com distribuição de pontos, F5/F9.
- Teste ECS headless e exemplo `autoplay` com screenshots.
- Infraestrutura Claude Code: CLAUDE.md, regras, 5 skills, 3 agentes, settings.
- Documentação técnica em `docs/`.
