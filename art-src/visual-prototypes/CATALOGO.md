# Catálogo — Laboratório de direção artística

**Status de todas as opções:** protótipo, **não aprovado**.
**Data:** 2026-11-16. **HEAD de referência:** `1bf2ed2`.
**Escopo:** alternativas visuais apenas. Nenhum asset publicado em `assets/`
foi substituído. O jogo principal não foi alterado.

Cenário gótico V2.3 (aprovado) permanece a referência de atmosfera:
`07_comparacoes/ref_scenery_causeway.png`.

Pranchas resumidas: `07_comparacoes/COMPARE_*.png`.

---

## Personagem

| Código | Prévia | Descrição | Técnica | Ferramentas | Compatibilidade | Esforço | Vantagens | Desvantagens |
|---|---|---|---|---|---|---|---|---|
| **CHAR-current** | [in-game](01_personagem/CHAR-current_in_game.png), [sprite 3×](01_personagem/current_idle_3x.png) | Protagonista publicado V2.2 (corpo sem arma no atlas, finish C, modelo v007) | Blender → sprites 2 texels/u + PIL VFX | Blender 5.2.4, `build_game_sprites_v2_2.py`, `publish_guard.py` | Já no jogo | — | Pipeline real, sockets, capa e armas procedurais | Silhueta fraca no fundo escuro; D2 de acabamento ainda aberto |
| **CHAR-A** | [imagem](01_personagem/CHAR-A_pixel_32.jpg) | Pixel art compacto ~32×32 de legibilidade | Conceito gerado (pixel) | Geração de conceito + limpeza manual futura | Exigiria novo atlas e pivôs | Baixo–médio | Silhueta clara, produção rápida de clips | Pouco detalhe para capa/armadura; redesenho manual de animações |
| **CHAR-B** | [imagem](01_personagem/CHAR-B_pixel_64.jpg) | Pixel art ~64×64 com armadura/cabelo/capa/arma legíveis | Conceito gerado (pixel) | Idem + folha de animação | Encaixa melhor na densidade 2 atual (células 128²) | Médio | Bom meio-termo legibilidade × esforço | Ainda exige arte quadro a quadro; conversão automática do Blender não basta |
| **CHAR-C** | [imagem](01_personagem/CHAR-C_pixel_tall.jpg) | Personagem alto (~64×96/128), proporção 7 cabeças | Conceito gerado (pixel alto) | Idem; células de combate já são 128×96 u | Próximo do combate atual; locomação precisaria células mais altas | Médio–alto | Melhor aristocrático vertical; cabeças ~ legíveis | Mais pixels por clip; possível desalinhamento com hurtbox 26×46 |
| **CHAR-D** | [Blender v007](01_personagem/CHAR-D_blender_current_side.png) | Pipeline Blender atual (master lateral v007) | Render EEVEE existente | Blender + scripts `bpy/` | Já alimenta o publish | Alto para polir | Uma malha anima todas as poses; sockets medidos | Na câmera atual (~92 px de altura) perde para pixel limpo; polimento novo **não** foi gerado neste laboratório (ver limitações) |

Referências de apoio: `ref_inspiration_concept.png`, `current_attack_2x.png`.

**Não há vencedor declarado.** Compare em `07_comparacoes/COMPARE_characters.png`.

---

## Interface

| Código | Prévia | Descrição | Técnica | Esforço | Notas |
|---|---|---|---|---|---|
| **HUD-current** | [captura](02_interface/HUD-current_game_capture.png) | HUD real: barras + **lenda permanente de teclas** (3 linhas) | Bevy UI (`src/ui/hud.rs`) | — | Controles sempre visíveis |
| **HUD-A** | [imagem](02_interface/HUD-A_gothic.jpg) | Gótica clássica, molduras ornamentadas | Mockup | Médio | Atmosfera forte; risco de ocupar visão |
| **HUD-B** | [imagem](02_interface/HUD-B_minimal.jpg) | Minimalista, barras discretas | Mockup | Baixo–médio | Máxima área de jogo; menos “castelo” |
| **HUD-C** | [imagem](02_interface/HUD-C_hybrid.jpg) | Híbrida: gótica + legibilidade | Mockup | Médio | Candidata de equilíbrio |
| **HUD-CMD** | [imagem](02_interface/HUD-command_feedback.jpg) | Feedback de comandos especiais (buffer/sucesso/falha) | Mockup | Médio | Complementa A/B/C; a lenda de teclas pode ir para tutorial / menu / modo `dev` |

Cada proposta cobre: HP, MP, Hearts, EXP, arma, magia, cooldowns, forma, feedback de comando.
**Não remover** informações de gameplay sem alternativa (a migração da lenda de teclas exige tela de controles ou tutorial).

---

## Armas e armaduras

Categorias **implementadas** em `assets/data/items/weapon_classes.ron`: Sword, Dagger, Axe, Spear, HeavyWeapon. Escudo existe como offhand visual (`HeldLayer::Shield`).

| Código | Prévia | Conteúdo |
|---|---|---|
| **GEAR-sheet** | [imagem](03_armas_armaduras/WEAPONS_ARMORS_sheet.jpg) | Espada, adaga, machado, lança, arma pesada, escudo + armaduras leve/média/pesada/ornamentada |
| **ARMOR-AB** | [imagem](03_armas_armaduras/ARMOR-on_character.jpg) | Mesmo personagem com armadura leve vs pesada |

**Produção:** armas atuais são **silhuetas procedurais** (`src/inventory/held_art.rs`) ancoradas em sockets Blender. Head/Gloves/Boots sem sprite dedicado. Capa de equipamento é camada 24×64 gerada em runtime.

| Critério | Pixel art | Blender |
|---|---|---|
| Armas | Contorno limpo por categoria; fácil variar | Modelo 3D por arma; mais custo |
| Armaduras | Overlays/tints por peça ou recortes | Troca de mesh/material; clipping a tratar |
| Capa | Camada 2D + sway (já existe) | Simulação de pano possível, cara |
| Manutenção | Redesign por clip se o corpo mudar | Uma malha; reexportar sprites |

---

## Magias e comandos

### O que já funciona

| Peça | Estado |
|---|---|
| Parser + buffer + orientação relativa | Funciona (`command_parser` / `command_input`) |
| Prioridade, MP, cooldown, estados | Funciona (`skills/execution`) |
| Resultados (executado, incompleto, inválido, lento, sem recurso, não aprendido, cooldown, arma/estado) | Funciona |
| Técnicas | `crescent_slash` `↓↘→+ATTACK`; `rising_edge` `↓↑+ATTACK` |
| Magias | `ember_bolt` `↓↘→+MAGIC` (VFX `fire_bolt` **sem clip no atlas**); `mending_light` `↓N↓+MAGIC` |

### Propostas originais (não implementadas; não copiar IPs)

| Código | Função | Comando proposto | Notas |
|---|---|---|---|
| **MAGIC-bolt** | Projétil elemental | *(já)* `↓↘→+MAGIC` | Sequência visual: [MAGIC-ember_sequence](04_magias/MAGIC-ember_sequence.jpg) |
| **MAGIC-heal** | Cura | *(já)* `↓N↓+MAGIC` | — |
| **MAGIC-shield** | Escudo | `↓↙←+MAGIC` | Barreira espectral curta |
| **MAGIC-nova** | Explosão em área | `↓↓+MAGIC` | Anel ao redor do jogador |
| **MAGIC-shade** | Magia sombria | `→↓↘+MAGIC` | Projétil lento, alto stagger |
| **MAGIC-summon** | Invocação | `↑↑+MAGIC` | Familiar breve (conceito) |
| **MAGIC-form** | Magia de transformação | `↓↑↓+MAGIC` | Conceito; a forma espectral atual usa **T** + relíquia |

Grade visual: [MAGIC-concepts_grid](04_magias/MAGIC-concepts_grid.jpg).
**Nenhum comando foi alterado nos dados RON nesta fase.**

---

## Transformações

| Código | Prévia | Conteúdo | Status |
|---|---|---|---|
| **FORM-current** | [captura](05_transformacoes/FORM-current_spectral_capture.png) | Forma espectral real: tint ciano α0,55, outline, motes; **sem atlas novo** | Implementado |
| **FORM-spectral** | [sequência](05_transformacoes/FORM-spectral_sequence.jpg) | Transição → idle → movimento → retorno | Protótipo visual |
| **FORM-golem** | [futuro](05_transformacoes/FORM-future_concepts.jpg) (esq.) | Forma Golem de Pedra — conceito **não implementado** | Conceito |
| **FORM-raven** | [futuro](05_transformacoes/FORM-future_concepts.jpg) (dir.) | Forma Corvo de Névoa — conceito **não implementado** | Conceito |

Custos de MP / regras de véu **não** foram alterados.

---

## Inimigos

Hoje só há definição rica de **Bone Sentry** (`castle_undead.ron`): retângulo colorido, AI Patrol→Chase→Attack.

| Código | Arquétipo | Prévia (folha) | Dificuldade de implementação |
|---|---|---|---|
| **ENEMY-melee** | Corpo a corpo | [sheet](06_inimigos/ENEMY-archetypes_sheet.jpg) | Baixa (já próximo do Bone Sentry) |
| **ENEMY-shield** | Com escudo | idem | Média (bloqueio / flanco) |
| **ENEMY-ranged** | À distância | idem | Média (projétil inimigo) |
| **ENEMY-mage** | Mágico | idem | Média–alta |
| **ENEMY-flyer** | Voador | idem | Alta (física/altura) |
| **ENEMY-heavy** | Pesado | idem | Média |
| **ENEMY-miniboss** | Mini-chefe conceitual | idem | Alta (telegraphs + fases) |

IA nova **não** foi implementada. Exemplos = silhueta / pose / antecipação / VFX conceitual.

---

## Comparações

| Arquivo | Conteúdo |
|---|---|
| [COMPARE_characters.png](07_comparacoes/COMPARE_characters.png) | Current × A × B × C × D |
| [COMPARE_hud.png](07_comparacoes/COMPARE_hud.png) | HUD atual × A × B × C |
| [COMPARE_systems.png](07_comparacoes/COMPARE_systems.png) | Equipamentos, magias, formas, inimigos, cenário |

---

## Sequência futura sugerida (não executar agora)

1. Estilo do protagonista (escolher CHAR-*).
2. Pipeline de animação (pixel manual vs Blender+limpeza).
3. Equipamentos visuais.
4. Interface (e destino da lenda de teclas).
5. Magias e efeitos (incl. clip `fire_bolt`).
6. Transformações.
7. Inimigos.
8. Polimento e integração.

O roadmap oficial **não** foi alterado.
