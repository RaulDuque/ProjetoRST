# Roadmap

`[x]` = implementado **e** validado (testes e/ou execução real). `[ ]` = pendente.
Validação de referência: `cargo test` (unitários, integração e ECS headless) e
`cargo run --example autoplay` (execução real com screenshots).

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
- [ ] Tela de inventário (equipar/desequipar/consumir/inspecionar/comparar)
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
- [ ] Transições entre salas e portas
- [ ] Áreas secretas, baús, objetos interativos, pontos de save no mapa
- [ ] Mapa do castelo (minimapa)
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
