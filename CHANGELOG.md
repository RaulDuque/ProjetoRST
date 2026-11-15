# Changelog

Formato: [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/). Versionamento semântico.

## [Unreleased]

### V2.2 — Equipamento visível, NPCs, mapa e HUD
- Corpo do protagonista renderizado sem o sabre (mesmas poses) e âncoras por
  quadro medidas no Blender, publicados em `assets/` pelo guarda (estado
  `1cd66a2977d0`) com autorização do proprietário.
- Arma equipada desenhada na mão (silhueta por categoria: espada, adaga,
  machado, lança, arma pesada) e escudo no braço, seguindo animação e direção;
  desligados enquanto a arte publicada tiver o sabre (`weapon_in_body`).
- Capa pendurada na âncora "costas" e piscando com o corpo.
- `publish_guard.py` publica e valida `player_sockets.ron`.
- NPCs com figuras provisórias por dados (`look`); mapa com marcadores de NPC e
  loja em salas visitadas.
- HUD: notificações num painel sob o status (não cobrem mais o personagem nem
  os avisos), e acima dos modais da loja/inventário.
- Vitrine `examples/equipment_visual_showcase.rs`; testes `held_visual_tests`;
  tutoriais 24–29.

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
