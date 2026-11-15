# Roadmap

`[x]` = implementado **e** validado (testes e/ou execução real). `[ ]` = pendente.
Validação de referência: `cargo test` (unitários, integração e ECS headless) e
`cargo run --example autoplay` (execução real com screenshots).

A direção de longo prazo (V1.6 → V2.4, mais itens futuros sem número) está em
[`GAME_VISION_MASTER.md`](GAME_VISION_MASTER.md) §6. Esta lista registra o que
já está feito, etapa por etapa.

## Versões recentes e próxima

- [x] **V2.2 — Equipamentos visuais:** arma em camada com âncoras, figuras de NPC, marcadores no mapa, HUD empilhado ([`VISUAL_EQUIPMENT_V2_2.md`](VISUAL_EQUIPMENT_V2_2.md))
- [x] **V2.3 — Câmera, cenários góticos, parallax e atmosfera** ([`WORLD_VISUAL_V2_3.md`](WORLD_VISUAL_V2_3.md))
- [ ] **V2.4 — Combat Feel / Peso do Golpe** (planejada, D-028; implementação aguarda autorização): [`PLANO_V2_4_COMBAT_FEEL.md`](PLANO_V2_4_COMBAT_FEEL.md)
- Futuro, sem número: conteúdo do castelo, chefes e progressão, missões e narrativa, expansão do mundo ([`PROPOSTA_ROADMAP_POS_V2_4.md`](PROPOSTA_ROADMAP_POS_V2_4.md))

## Etapa 1 — Infraestrutura
- [x] Projeto Cargo `dark_castle_rpg`, Rust 2024, dependências fixadas e `Cargo.lock` versionado
- [x] Perfis `dev` (deps otimizadas), `release`, `dist`; feature `dev` (linkagem dinâmica)
- [x] `cargo fmt`, `clippy`, `test`, `check`, `run` funcionando na raiz
- [x] CLAUDE.md, regras, 5 skills, 3 agentes, `settings.json`
- [x] Arquitetura modular por plugins, `GameSet` ordenados, estados de jogo
- [x] Carregamento e validação de dados RON (`GameData`)
- [x] Teste ECS headless e exemplo de autoplay com screenshots
- [ ] Integração contínua (CI) rodando fmt/clippy/test

## Etapa 2 — Personagem e movimentação
- [x] Andar, correr, pular (altura variável, coyote time), queda, gravidade
- [x] Colisão com chão, paredes e plataformas atravessáveis
- [x] Agachar, guarda, esquiva invulnerável
- [x] Máquina de estados que impede ações incompatíveis
- [x] Câmera suave limitada à sala
- [x] Visual temporário por estado (cor, achatamento, viseira de orientação)
- [ ] Interação (alavancas, NPCs, baús)
- [ ] Sprites e animações reais
- [ ] Suporte a gamepad e remapeamento em jogo

## Etapa 3 — Combate básico
- [x] Ataque físico com hitbox temporária
- [x] Fórmula de dano (defesa, elemento, crítico, precisão, variância) testada
- [x] Receber dano, recuo, invulnerabilidade, morte e respawn (R)
- [x] Números de dano flutuantes
- [ ] Combos de ataque básico
- [ ] Efeitos de status (veneno, queimadura, maldição, petrificação...)

## Etapa 4 — Reconhecimento de comandos
- [x] Buffer temporizado, relativo à orientação, com diagonais e neutro
- [x] Parser de notação (setas ou letras) com erros descritivos
- [x] Tolerância a entradas extras, janelas configuráveis por habilidade
- [x] Prioridade e desempate entre comandos parecidos
- [x] Sem execução duplicada; cancelamento pela guarda
- [x] 8 resultados distintos (executado, incompleto, inválido, lento, sem recurso, não aprendido, cooldown, estado inválido) com testes
- [x] Feedback do resultado no HUD
- [ ] Modo de treino que mostra o buffer na tela

## Etapa 5 — Técnicas especiais
- [x] Definições em dados, sem código por habilidade
- [x] Crescent Slash (comando, avanço, hitbox própria)
- [x] Rising Edge com custo de Hearts e desbloqueio automático no nível 3
- [x] Cooldowns exibidos no HUD
- [ ] Revelação de comandos por livros/mestres; ranks de melhoria aplicados ao dano
- [ ] Técnicas de armas pesadas, defensivas e de movimento

## Etapa 6 — Magias
- [x] Execução separada do parser; custo de MP; tempo de conjuração
- [x] Projétil elemental (Ember Bolt) com colisão em sólidos e inimigos
- [x] Cura (Mending Light, comando com neutro)
- [ ] Explosões/área, dano contínuo, controle, proteção, invocação
- [ ] Interrupção específica de conjuração longa

## Etapa 7 — Atributos e progressão
- [x] STR/CON/INT/LCK, HP/MP/Hearts, todos os derivados documentados e testados
- [x] Curva de EXP configurável, level up, crescimento automático, pontos distribuíveis (tela de status)
- [x] Requisitos por nível, atributos, itens e flags de progresso
- [x] Tela de status
- [x] Regeneração de HP/MP
- [ ] Efeitos passivos

## Etapa 8 — Inventário e equipamentos
- [x] Itens, categorias, raridades configuráveis, pilhas, descarte protegido, ordenação
- [x] 8 slots com validação de categoria e requisitos; bônus recalculam atributos
- [x] Drops influenciados por LCK
- [x] Tela de inventário: equipar, remover, inspecionar e comparar (V2.0, tecla I)
- [x] Perfis de arma por categoria, compatibilidade de técnicas, teto de resistência, catálogo de 27 equipamentos (V2.0)
- [x] Aparência por equipamento em camadas de runtime: rastro, capa, tom (V2.0, parcial: sem arma/elmo desenhados)
- [ ] Consumir itens pela tela de inventário
- [x] Regra de duas mãos: lança e arma pesada não usam escudo (V2.1)

## Etapa 8b — Cidade, NPCs e comércio (V2.1)
- [x] Vila das Cinzas (6 salas), ligada ao Watch Shaft, opcional
- [x] NPCs por dados, `D` para conversar (objetos têm prioridade)
- [x] Diálogos com escolhas e condições de progresso (sem scripts)
- [x] Lojas: comprar, vender, estoque ilimitado/limitado, comparação, moeda "Marks"
- [ ] Moedas e estoque no save (aguarda autorização do Save v3)
- [ ] Drops de equipamento (gerador de loot isolado)
- [ ] Ícones de loja/NPC no mapa; arte de NPCs
- [x] Tutoriais do proprietário (24, em `docs/tutorias/` até liberar `Md-usario/tutorias/`)
- [ ] Uso de consumíveis em jogo

## Etapa 9 — Inimigos
- [x] Arquitetura genérica orientada a dados; IA pura testada
- [x] Patrulha, perseguição, ataque com aviso, espera, fuga; dano por contato
- [x] Fraquezas/resistências, EXP, drops, respawn
- [ ] Mais arquétipos (voadores, à distância, defensores) e uso de habilidades
- [ ] Barra de vida de inimigos

## Etapa 10 — Chefes
- [ ] Máquina de estados de chefe, múltiplas fases, padrões de ataque em dados
- [ ] Arena com portas trancadas e recompensa

## Etapa 11 — Mundo e mapas
- [x] Sala definida em RON com plataformas e spawns
- [x] Transições entre salas e portas no jogo principal (V1.7: `world::transitions`, save v2, rota real sem atalhos)
- [x] Áreas secretas, baús e objetos interativos (V1.8: paredes quebráveis, interruptor oculto, baús, alavancas, atalho; região Galerias Esquecidas)
- [x] Primeira transformação (V1.9: Forma Espectral com a Relíquia do Véu, véus espectrais, região Claustro Velado; `TRANSFORMATIONS_V1_9.md`)
- [ ] Outras transformações (voadora, veloz, pequena, aquática, sombra) e arte própria da forma espectral
- [ ] Pontos de save no mapa, placas de pressão, baús de chefe
- [ ] Mapa do castelo (minimapa) (V1.7: painel no jogo com a tecla M; V1.8: baús, atalhos e % por região; tela cheia pendente)
- [x] Grafo de mundo em RON com validador de conexões e alcance (V1.6, `world_report`)
- [ ] Carregamento de assets por sala

## Etapa 12 — Interface completa
- [x] HUD: HP, MP, Hearts, nível, EXP, cooldowns, notificações
- [x] Tela de status
- [ ] Menu principal e pausa
- [ ] Telas de inventário e de habilidades dedicadas
- [ ] Fonte gótica própria (com setas) e arte de UI

## Etapa 13 — Salvamento
- [x] Formato versionado, migração, detecção de corrupção e versão incompatível
- [x] Escrita segura com backup; F5/F9
- [x] Salva posição, atributos, nível, EXP, habilidades, inventário, equipamentos, progresso do mundo
- [ ] Vários slots e tela de seleção
- [ ] Salvar apenas em pontos de save

## Etapa 14 — Balanceamento
- [ ] Planilha/ferramenta de curvas (EXP, dano por nível)
- [ ] Testes de tempo-para-matar por inimigo

## Etapa 15 — Polimento e otimização
- [ ] Áudio (música e efeitos)
- [ ] Partículas e efeitos visuais
- [ ] Perfilamento e build de distribuição testado em outras plataformas
