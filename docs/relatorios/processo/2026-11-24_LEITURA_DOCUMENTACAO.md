# Processo — Leitura da documentação e mapa dos próximos passos

**Status:** CONCLUÍDO COM RESSALVAS
**Aprovação artística:** não se aplica

## 1. Identificação
- Etapa: leitura de `Md-usario/` e `docs/` para entender o projeto e os próximos passos
- Frente: processo
- Branch: `art/v1.2-protagonist`

## 2. Data e hora da execução
2026-11-24 (hora não registrada)

## 3. Objetivo original
Ler toda a documentação de `Md-usario/` e de `docs/` para entender o projeto e os próximos
passos.

## 4. Prompt executado
Pedido feito no chat (não versionado): "pode ler toda a documentaçao de Md-usario e da pasta
docs para entender do que se trata o projeto e seu procimos passos".

## 5. Contexto técnico inicial
- HEAD `74aadaa` (relatório da Modular Character Engine 05), árvore limpa e sincronizada.
- Única pendência local: a pasta não versionada `Md-usario/assets-user/prota/base/a/`
  (`f1.jpg`, `f2.png`, `refin2.png`, `refin2-removebg-preview.png`), que nenhum briefing cita.

## 6. Ferramentas, plugins, skills e versões
Claude Code com 5 subagentes de leitura em paralelo (somente leitura). Nenhum Cargo, Blender
ou Python foi executado.

## 7. Arquivos consultados
- 467 arquivos `.md` (cerca de 3 MB):
  - `Md-usario/`: `inicio.md`, `promptMaster.md`, `futuro`, séries `DarkCastleR-D`, `-F`,
    `-N` e `-V`, `Erro-Corretion/`, `tutorias/`;
  - `docs/`: os 52 arquivos de topo, `development/` (d17–d23, n1–n4,
    modular_character_engine), `relatorios/`, `tutorias/`.
- Divisão da leitura: série D com `inicio`/`promptMaster`; séries V e F; série N com
  Erro-Corretion e `futuro`; `docs/development/`; docs de topo com tutoriais. Eu li
  diretamente `ARCHITECTURE.md`, `INDICE.md`, `ULTIMO_RELATORIO.md` e o relatório da MCE 05.

## 8. Arquivos criados
- `docs/relatorios/processo/2026-11-24_LEITURA_DOCUMENTACAO.md`

## 9. Arquivos modificados
- `docs/relatorios/INDICE.md`
- `docs/relatorios/ULTIMO_RELATORIO.md`

## 10. Arquivos removidos
Nenhum.

## 11. Descrição detalhada das alterações
Nenhuma alteração de código ou de documentação técnica. Este relatório registra o
entendimento consolidado, abaixo.

### Projeto
- RPG de ação 2D metroidvania em Rust 2024 + Bevy 0.20.1, com inspiração mecânica no SOTN e
  todo o conteúdo original.
- Diferencial: técnicas e magias ativadas por sequências de comandos definidas em dados.

### Jogável hoje
- Movimento e controles híbridos remapeáveis.
- Combate com comandos, V2.4 parcial: linha do tempo do golpe, cancelamento, recuo, com as
  políticas novas desligadas.
- Atributos e EXP, inventário de 8 slots com 27 equipamentos, consumíveis.
- Mundo por grafo com cerca de 29 salas em 4 regiões: Vigil Gates, Forgotten Galleries,
  Veiled Cloister e Ash Village.
- Segredos, baús e alavancas; Forma Espectral; altares de save e viagem rápida.
- NPCs e lojas, Save v3.
- Inimigos: 3 arquétipos. Chefe: The Ashen Sentinel, com 2 fases.

### Frente atual de arte
Do protagonista modular, linha N3 → N4 → Erro-Corretion → Modular Character Engine 01–05.
Essa engine existe só em exemplos isolados e não está integrada ao jogo.

## 12. Decisões técnicas e artísticas
Nenhuma tomada. A leitura foi delegada a subagentes por causa do volume (cerca de 3 MB). Os
arquivos maiores foram lidos parcialmente: cabeçalho, missão, decisões e pendências.

## 13. Comandos relevantes executados
`find`, `wc`, `cat`, `grep`, `git status`, `git log`, `git branch`.

## 14. Testes realizados
Não se aplica (sem alteração de código). `git diff --check` antes do commit.

## 15. Resultado dos testes
`git diff --check` sem problemas.

## 16. Métricas e comparações
467 `.md` lidos ou resumidos; 226 commits no histórico.

## 17. Problemas encontrados
Contradições e trechos desatualizados na documentação:
1. **Status da V2.4.**
   - `ROADMAP.md`, `GAME_VISION_MASTER.md` e `PLANO_V2_4_COMBAT_FEEL.md` dizem "não
     implementada".
   - `COMBAT_FEEL_V2_4.md` e `COMBAT_SYSTEM.md` descrevem os blocos A, B1 e B2 implementados.
2. **Economia no save.** `ROADMAP.md` (8b), `NPC_COMMERCE_SYSTEM.md` e `NPC_COMMERCE_V2_1.md`
   dizem que moedas e estoque não são salvos. O Save v3 (D-024) salva.
3. **Consumíveis.** No índice dos tutoriais, o item 05 diz "usar poções: pendente", mas foi
   implementado na D18.
4. **`ROADMAP.md` defasado.**
   - Marca como abertos o remapeamento (feito na N2), os ícones no mapa (V2.2) e a arena
     trancada (D21).
   - Não cita as etapas N nem a engine.
5. **Cabeçalhos antigos da V1.6.**
   - `WORLD_ARCHITECTURE.md` e `ROOM_SYSTEM.md` ainda dizem "sala única `entry_hall`".
   - `WORLD_PERSISTENCE.md` diz "save v1".
   - `SAVE_SYSTEM.md` tem o título "versão 1".
6. **Câmera.** `ART_DIRECTION.md` (D3, 1:1), V2.3 (`FixedVertical`) e D-042/D-043 (405 × 360)
   não estão alinhados.
7. **Tabela do D17.** `D17_ROADMAP_FUTURO.md` tem as linhas D21 e D22 duplicadas.
8. **Linha solta no roadmap.** O fim de `n1/N1_ROADMAP.md` tem a linha fora de contexto "a
   palavra chave e batata azul".
9. **Briefings vazios.** `DarkCastleR-d-16.md`, `DarkCastleR-N-5.md` e `Erro-Corretion/a`
   estão vazios.
10. **Prompts ausentes.** Os prompts da Engine 01–05 (`Md-usario/engine/Engine-0X.md`, citados
    nos relatórios) não estão no disco.

## 18. Problemas corrigidos
Nenhum. A tarefa era só leitura.

## 19. Problemas pendentes
**Aguardando o proprietário (aprovação visual):**
- MCE 01–05: conjunto gótico refinado, couro e duas capas (decisão aberta no relatório 05).
- Erro-Corretion 03B e 03D Take 03.
- Os prompts 04 a 08 do Erro-Corretion nunca foram executados; o 04 está bloqueado até haver
  aprovação.

**Gameplay:**
- Playtest humano do Ashen Sentinel: a ficha em `D23_PLAYTEST_HUMANO.md` está em branco.
- A esquiva no fim do aviso ainda pode levar o golpe.
- O chefe fica fora do quadro na chegada.
- Defeitos P3 da D20.1: cursor do inventário e queda na plataforma do Watch Shaft.
- D-GAME-002 a 005 ainda abertos.
- Correção do quick load (Fase 6).

**Roadmap:**
- V2.4, etapas 8 a 10: atordoamento de inimigos, hurtbox por postura e retorno visual.
- `PROPOSTA_ROADMAP_POS_V2_4.md`: 13 blocos, ordem não aprovada.
- N1, etapas N5 a N12: equipamento visual no inventário, castelo, relíquias e habilidades
  (`DoubleJump` e `WallBreaker` existem só como nomes), minichefe, transformações e
  polimento.
- Menu principal e pausa, gamepad, áudio, CI, missões e facções.

**Adiado por ordem do proprietário:** produção visual definitiva (`Md-usario/futuro`:
Hunyuan3D, Blender MCP, inimigos, chefes e NPCs).

## 20. Limitações técnicas
- Leitura de 3 MB via resumos de subagentes. Nos briefings maiores (d-7 a d-23, E2, E3 e as
  listas internas da série V), as checklists não foram lidas linha a linha.
- As inferências dos resumos (por exemplo, o total de cerca de 29 salas) não foram
  conferidas no código.

## 21. Evidências visuais
Não se aplica.

## 22. Caminhos das imagens, GIFs e vídeos
Não se aplica.

## 23. Links do GitHub
Não registrado.

## 24. Resultado final
Visão do projeto, estado atual e pendências consolidados neste relatório. Nada foi alterado
no jogo.

## 25. Recomendações para a próxima etapa
1. Fechar a decisão visual em aberto. A MCE 05 decide se a arte serve, e isso destrava a
   integração da engine ao jogo ou um novo refinamento.
2. Se a arte for aprovada, integrar a engine ao jogador, começando pelo item 1 do §9 de
   `01_ARQUITETURA.md`, que liga o rig ao `CompositeCharacter` do jogo. Isso também cobre a
   N5 do roadmap.
3. Em paralelo, e sem depender de arte, fazer o playtest humano do chefe (D23), que hoje
   bloqueia o balanceamento.
4. Fazer uma tarefa curta de documentação para corrigir as contradições do item 17. A mais
   relevante é atualizar o `ROADMAP.md`.

## 26. Decisões que dependem do proprietário
- Aprovar ou reprovar a arte da MCE 05 (e da 01–04). Dizer se a capa sempre atrás do corpo
  atende.
- Próxima frente: refinamento artístico, integração da engine ao jogo, retorno ao gameplay
  (V2.4, etapas 8 a 10, ou pós-V2.4) ou revisão da documentação.
- Destino da pasta não versionada `Md-usario/assets-user/prota/base/a/`.
- D-GAME-002 a 005.

## 27. Estado do Git e publicação
- Branch: `art/v1.2-protagonist`
- Commit(s): o commit deste relatório
- Push: ver o resumo no terminal
- Local × remoto: sincronizado antes da tarefa
