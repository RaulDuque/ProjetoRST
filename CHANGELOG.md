# Changelog

Formato: [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/). Versionamento semântico.

## [Unreleased]

### N3 — Prova visual isolada
- Exemplo `n3_visual_proof`: corpo, duas armaduras, espada, lança e capa, fora do jogo.
- Vista 640×360 em escala inteira 2× na janela 1280×720. A câmera do `cargo run` não muda.
- Pixelorama não entrou. A figura é de script e não está aprovada como estilo.

### D23 — Pronto para playtest humano do Ashen Sentinel
- Plataforma de chegada sobre o vão da porta do chão da Ashen Court; a escada do Reliquary sobe só com o pulo.
- O telégrafo do chefe (cor e contagem) prevalece sobre o flash branco de dano. Inimigos comuns continuam piscando branco.
- Probe de janela sem cura injetada (`d23_play_probe`): derrota registrada, boss em 13 HP. Sem retune. Playtest humano ainda pendente.

### D22 — Luta real contra o Ashen Sentinel
- Wind-up do chefe não é interrompido por dano; fase 2 só depois de slash e lunge (ou HP < 25%).
- Telégrafos por cor e rótulo da barra (slash / lunge / ash wave / phase shift).
- Escada de plataformas no Reliquary até a porta do teto; patrulha do sentinela fora do vão da saída.
- Teste `boss_real_fight_tests` (só `KeyX`) e probe `d22_boss_probe`. Sem bump de save.

### D21 — Primeiro chefe (Ashen Sentinel)
- `BossDefinition` + fases (slash/lunge → ash wave), arena `vigil_gates_ashen_court`.
- Encontro sela saídas; barra de HP; `defeated_bosses` + `sentinel_brand` únicos.
- Testes `boss_encounter_tests`, probe `d21_boss_probe`, docs `docs/development/d21/`.

### D20 — Variedade de inimigos
- `AiStyle` (`melee` / `ranged` / `keep_distance`) em `AiProfile`.
- `bone_bolt` (projéteis) no Watch Shaft; `ash_warden` (skirmisher) na Gallery.
- Docs: `docs/development/d20/` (inclui preparação para chefes D21).

### D19 — Viagem rápida entre altares
- Após descansar num altar, se houver outros descobertos, abre o menu de
  viagem (`GameState::FastTravel`); Enter teleporta via o ciclo de salas.
- Segundo altar `vg_causeway_save`; viagem não salva nem muda `last_save_point`.
- Docs: `docs/development/d19/`.

### D18 — Consumíveis utilizáveis
- Uso atômico pela tela `I` (`UseItemRequest` → `use_consumable`):
  `minor_tonic` (HP) e `minor_ether` (MP); recusa se o recurso já está cheio.
- Start e loja de Hessa incluem Minor Ether; Save V3 inalterado.
- Docs: `docs/development/d18/`.

### D17 — Pontos de salvamento no mundo
- `ObjectKind::SavePoint`: altar reutilizável (`D`) cura HP/MP, marca
  `discovered_save_points` / `last_save_point` e grava o slot.
- Marcador ciano no mapa após descoberta; altar piloto em Gate Approach
  (`vg_approach_save`). F5 livre permanece.
- Docs: `docs/development/d17/`.

### V2.3 — Câmera, cenários góticos, parallax e atmosfera
- Câmera configurável (`assets/data/camera.ron`): zoom ×1,78 (altura de mundo
  405), zonas mortas, olhar à frente suave, enquadramento de pulo e queda,
  limites por sala, snap em teleportes, independente da física.
- Cenário por dados: `visual` por região (fundo em parallax de 2 camadas,
  pedra, luz, céu, névoa) e `decor` por sala (12 peças góticas geradas em
  código); brilhos de tochas/janelas/lanternas e névoa; tudo desligável.
- Atmosfera em Vigil Gates, Forgotten Galleries e Village of Ashes; salas
  piloto `vigil_gates_causeway`, `fg_stair` e `av_square`.
- Vitrine `world_visual_showcase`, testes `world_scenery_tests`, tutoriais 30–35.

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
