# V1.9 — Transformações e Forma Espectral

Primeira transformação jogável: a **Forma Espectral**, liberada pela Relíquia
do Véu. O jogador alterna a forma com `T`. Nela, ele atravessa **véus
espectrais** (barreiras específicas), mas não paredes comuns, e volta com
segurança à forma humana. A arquitetura é reutilizável para outras formas,
mas só esta foi implementada.

Legenda de estado: **[Implementado]** (código e testes), **[Validado visualmente]**
(capturas abertas e inspecionadas), **[Planejado]** (só desenho).

---

## 1. Auditoria

- **Git (início):**
  - branch `art/v1.2-protagonist`, HEAD `204f965`;
  - 357 linhas de `git status`;
  - staged só o PNG `art-src/player/inspiration/protagonist_concept_v001.png`.
- **Arquivos protegidos:**
  - 29 hashes conferidos e iguais à linha de base da V1.8: `Md-usario/`
    (com `DarkCastleR-F/` e `DarkCastleR-V/`), `saves/` e `.cargo/config.toml`;
  - os 5 assets publicados do protagonista também estão iguais;
  - `target/verify/` e `target/f4-stage/` existem.
- **Linha de base:** 268 testes Rust (V1.8), confirmados nesta sessão
  antes da primeira alteração de comportamento.
- **Documentos pedidos que não existem:** `INPUT_COMMANDS.md`,
  `SKILLS_SYSTEM.md` e `INVENTORY_EQUIPMENT.md`. Os equivalentes reais são
  `INPUT_SYSTEM.md`, `SKILL_SYSTEM.md` e `EQUIPMENT_SYSTEM.md`.
- **O que já existia:**
  - **Relíquia do Véu:** pickup `veil_relic` na Galeria Superior, que concede
    `Ability::VeilStep` e já abria a grade do Salão Selado.
  - **Formas planejadas:** os ids existiam em `graph::Ability` (`WraithForm`
    etc.), mas nenhum sistema de forma.
  - **`TRANSFORMATION_SYSTEM.md`:** tinha a proposta e a decisão pendente
    sobre os perfis de movimento.
- **Tecla `T`:** está livre. Nenhuma tecla de jogo, menu ou exemplo a usava.

### Matriz de impacto

| Área | Arquivo | Mudança | Risco |
|---|---|---|---|
| Colisão (pura) | `world/collision.rs` | `BarrierKind`, `Solid::barrier`, `move_and_collide_phased` | Baixo: com lista vazia, o código é o mesmo |
| Física | `world/physics.rs` | componentes opcionais `Phasing` e `GravityProfile` | Baixo: sem o componente, o ramo é o antigo |
| Movimento | `player/movement.rs` | componente opcional `LocomotionProfile` | Baixo: sem o componente, o ramo é o antigo |
| Grafo | `world/graph.rs`, `rooms.rs` | `Veil`, `WorldRoom::veils`, `passages` no manifesto, `Platform::barrier` | Médio: o layout ganhou véus |
| Validador | `world/validation.rs` | uma porta com véu exige a passagem; `BadVeil` | Médio |
| Entrada | `input/mod.rs` | `T` / `transform_pressed` | Baixo |
| Save | `save/systems.rs` | recusa salvar dentro de um véu | Baixo (schema igual) |
| HUD e mapa | `ui/hud.rs`, `ui/world_map.rs` | linha da forma, dica movida, porta com véu | Baixo |
| Novo módulo | `transform/` | definições, estado, travessia, sistemas, visual | Novo |
| Dados | `transformations.ron`, `world.ron`, regiões, item | forma, passagens, véus, 2 salas, 1 anel | Baixo |

Combate, habilidades, magia (exceto a máscara de entrada), IA inimiga,
fórmulas, hitboxes e hurtboxes **não foram alterados**.

## 2. Arquitetura **[Implementado]**

```text
transform/
├── definitions.rs  (puro)  TransformationDefinition, TransformLibrary, FormId, FormAction,
│                           MovementProfile, CollisionProfile, FormCosts, CancelRules, FormVisual, LoadRule
├── state.rs        (puro)  FormState (forma ativa, fim pendente, custo), check_activation, EndReason
├── crossing.rs     (puro)  custo de atravessar um véu (tempo e MP)
├── systems.rs      (Bevy)  máscara de entrada, ativação, manutenção, fim seguro, morte, load
└── visual.rs       (Bevy)  tom, halo, contorno, partículas, cintilação dos véus
```

- **Decidir × aplicar:** o estado da forma (`Transformation`, um componente
  do jogador) decide qual forma está ativa. A física e o movimento não
  conhecem o módulo de transformação.
- **Perfis:** a forma aplica seu perfil por três componentes genéricos,
  inseridos ao entrar e removidos no mesmo sistema que encerra a forma:
  `LocomotionProfile` (player), `GravityProfile` e `Phasing` (world/physics).
- **Forma humana:** é a ausência de forma. Nenhum componente de perfil fica
  no jogador, e o código executado é o da V1.8.
- **Mensagem:** `FormChanged` (`Entered`, `Ending`, `Left`, `Refused`),
  lida pelo visual.

## 3. Definições de transformação **[Implementado]**

- **Arquivo:** `assets/data/transformations.ron`, carregado em
  `GameData::transformations` e validado ao carregar.
- **Validação:** velocidade positiva; gravidade em (0, 1]; queda negativa;
  custos não negativos; a forma mantém `Move`; ela termina por vontade ou
  por falta de MP; e os parâmetros visuais são válidos.
- **Execução:** nenhum campo é executado. Tudo é casado por `match` nos sistemas.

| Campo | Forma Espectral |
|---|---|
| `id` | `Spectral` |
| `name` / `description` | "Spectral Form" / texto |
| `unlock` | `Ability(VeilStep)` (Relíquia do Véu) |
| `unlock_hint` | "Relic of the Veil" (HUD) |
| `costs` | ativação 8 MP; 4 MP/s |
| `movement` | 150 u/s; gravidade × 0,35; queda máx. −220 u/s |
| `collision.passes` | `[Spectral]` |
| `allowed` | `[Move]`. Bloqueadas: Jump, Run, Crouch, Attack, Spell, Guard, Dodge, Interact |
| `activation` | só no chão |
| `cancel` | voluntário e por falta de MP (a morte sempre encerra) |
| `visual` | tom (0,62, 0,86, 1,0), α 0,55, contorno (0,75, 0,95, 1,0) α 0,5, partícula a cada 0,09 s, MP baixo < 3 s |
| `on_load` | `HumanForm` |

## 4. Perfis de movimento **[Implementado]**

| | Humano (inalterado) | Espectral |
|---|---|---|
| Velocidade | 190 andando / 310 correndo (× atributo) | 150 fixo, sem corrida |
| Pulo | 720 u/s, coyote 0,08 s | não pula |
| Agachar | sim | não |
| Gravidade | −1900 u/s² | −665 u/s² (× 0,35) |
| Queda máxima | −900 u/s | −220 u/s (flutua para baixo) |

- **Valores originais:** escolhidos para este projeto. Fora da forma, o
  movimento vertical fica limitado à descida lenta: não há voo.
- **Prova:** `the_human_form_is_identical_before_and_after_a_transformation`
  mede velocidade e altura do pulo antes e depois de um ciclo de forma, e os
  valores são iguais. A rota V1.7, a V1.8 e os testes antigos passam sem
  mudança de valores físicos.

## 5. Perfis de colisão **[Implementado]**

- **Sólidos:** cada `Solid` tem `barrier: Option<BarrierKind>`. Paredes,
  pisos e plataformas usam `None`, e nenhuma forma passa por eles.
- **Véus:** são `Some(Spectral)`.
  - Para humanos, inimigos e projéteis, um véu é um sólido como outro
    qualquer, e o código deles não mudou.
  - Só um corpo com `Phasing { passes: [Spectral] }` o ignora
    (`move_and_collide_phased`).

| Elemento | Humano | Espectral |
|---|---|---|
| Parede, piso, teto | sólido | sólido |
| Plataforma de mão única | sólida por cima | sólida por cima |
| Porta | abertura | abertura |
| Véu espectral | sólido | atravessa |
| Objetos (parede quebrável intacta) | sólido | sólido |
| Inimigos | dano por contato | dano por contato |
| Projéteis inimigos | atingem | atingem |

## 6. Forma Espectral **[Implementado, Validado visualmente]**

O corpo vira um véu cinza-azulado translúcido. O sprite do protagonista é
reaproveitado com tom e transparência, sobre um halo suave, cercado por
partículas que sobem. Nenhum asset novo foi publicado (§13).

## 7. Ativação **[Implementado]**

`T` (`InputBindings::transform`) começa a forma quando todas as condições
valem, verificadas sempre nesta ordem:

1. a relíquia foi obtida (`unlock` atendido pelas capacidades de exploração);
2. o jogador está vivo;
3. nenhuma ação está em andamento (ataque, conjuração, guarda, esquiva,
   atordoamento): só estados de locomoção;
4. o jogador está no chão;
5. não há transição de sala pendente (`TransitionLock` armado);
6. há MP para a ativação.

A recusa mostra o motivo, por exemplo "Spectral Form: needs the Relic of the
Veil", "not enough MP (5/8)", "stand on the ground" ou "cannot transform now".

- **Pagamento único:** a ativação é paga uma vez (`FormState::activate`
  devolve 0 se a forma já estiver ativa).
- **Tecla segurada:** age uma vez só (`just_pressed`).
- **Quick load no mesmo frame:** se `T` e `F9` chegam no mesmo frame, a
  forma é ativada em Actions e o load (em Rooms) a encerra. O MP é o do save.

## 8. Cancelamento **[Implementado]**

`T` com a forma ativa pede o fim (`EndReason::Voluntary`). O fim acontece em
duas etapas (§18):

1. **Pedido:** registra o motivo.
2. **Fim efetivo:** acontece quando o corpo humano cabe onde o jogador está.

Fora de um véu, as duas etapas ocorrem no mesmo frame.

## 9. Consumo de MP **[Implementado]**

- **Custo:** ativação de 8 MP e manutenção de 4 MP/s.
- **Frações:** são acumuladas (`FormState::upkeep`), e o MP inteiro é
  descontado quando a soma chega a 1.
- **Regeneração:** a regeneração global (0,8 MP/s no início) não mudou e
  continua durante a forma. O consumo líquido é ≈ 3,2 MP/s.
- **Duração:** com MP cheio (52), a forma dura cerca de 13 s.
- **Falta de MP:** quando a manutenção não pode ser paga, o MP vai a 0 e a
  forma pede o fim (`MpEmpty`).
- **Durante o fim pendente:** não há manutenção.
- **HUD:** mostra o MP usado desde a ativação.

## 10. Combate **[Implementado]**

- **Máscara de entrada:** `mask_form_input` (GameSet::Input, logo após a
  leitura do teclado) zera as entradas das ações que a forma bloqueia:
  - ataque (golpes e técnicas, que terminam no botão de ataque);
  - magia;
  - guarda e esquiva;
  - interação, pulo e corrida.
- **Comandos:** nenhum comando chega a ser reconhecido. Dano, cooldowns,
  fórmulas, hitboxes, hurtboxes e IA não mudaram.
- **Início durante uma ação:** a forma não começa durante ataque,
  conjuração, guarda, esquiva ou atordoamento (`Busy`). Por isso nenhuma
  ação de combate é interrompida pela metade.
- **Volta ao humano:** todas as teclas voltam a funcionar no mesmo frame.
  Há teste do ataque e do baú logo após a reversão.

## 11. Dano **[Implementado]**

- **Vulnerabilidade:** a forma continua vulnerável. O dano usa as regras
  existentes (`receive_damage`): atordoamento, recuo e invulnerabilidade
  pós-dano normais. Não há imunidade.
- **Dentro de um véu:** o dano funciona igual.
- **Recuo:** pode tirar o corpo do véu. Nesse caso a forma pendente termina
  ao sair.

## 12. Morte **[Implementado]**

- **Fora de um véu:** a morte encerra a forma no mesmo frame
  (`EndReason::Death`).
- **Dentro de um véu:** o corpo continua em fase, para não ser reposto como
  humano dentro do véu, e fica **parado** onde caiu. Isso é feito com
  `GravityProfile` 0 e velocidade 0; sem isso, um véu de piso deixaria o
  corpo afundar para fora da sala.
- **Ao levantar (R):** o jogador volta ao ponto de respawn da sala, a forma
  termina no frame seguinte e os perfis são removidos.

## 13. Animações e VFX **[Implementado, Validado visualmente]**

- **Assets preservados:** nenhum modelo, atlas, PNG, pivô, duração ou
  animação humana foi alterado, e nada foi publicado. O guarda de
  publicação e `player.publish.json` não foram tocados.
- **Sprite:** a forma reaproveita as animações de locomoção humanas
  (`Idle`, `Walking`, `Falling`). `tint_form` roda depois do visual do sprite
  e aplica o tom e o α da definição.
- **Halo:** é um gradiente radial de 64×64 gerado **em memória** no Startup
  (`create_glow_image`), atrás do corpo. Não existe arquivo.
- **Contorno:** é uma cópia do quadro atual, 12% maior, atrás do sprite e
  com o mesmo deslocamento do filho visual.
- **Partículas:** uma a cada 0,09 s, subindo e sumindo em 0,7 s. As
  posições vêm de um hash determinístico, sem tocar no RNG do jogo.
- **Entrada e saída:** um anel de 16 partículas, ciano ao ativar, branco
  quente ao voltar e cinza na morte.
- **MP baixo:** com menos de 3 s de manutenção, sprite e halo pulsam.
- **Sincronização:** visual e física leem o mesmo `Transformation` no
  mesmo frame. Uma forma esperando para terminar dentro de um véu continua
  com aparência espectral.
- **Véus:** azul translúcido com cintilação lenta (`VeilSurface`).

## 14. HUD **[Implementado, Validado visualmente]**

- **Linha da forma, sob a barra de MP:**
  - `Form: Human  T: Spectral Form (locked)` antes da relíquia;
  - `Form: Human  T: Spectral Form (8 MP, 4 MP/s)` quando há MP, ou
    `needs 8 MP` em laranja quando falta;
  - `Form: Spectral Form  -4 MP/s  ~11s  (used 8 MP)` quando ativa, em
    laranja abaixo de 3 s, e `returning...` quando espera sair de um véu.
- **Recusas:** aparecem como notificação, com o motivo.
- **Dica de controles:** saiu da borda inferior, onde cobria o chão, e foi
  para dentro do painel de status, em três linhas curtas. Ela também
  menciona `T form`.
  - No canto superior direito, que foi tentado primeiro, ela ficava
    sobreposta ao minimapa, sempre visível.
  - A lógica do HUD não mudou: só a posição do texto.

## 15. Barreiras (véus) **[Implementado, Validado visualmente]**

```ron
veils: [
    (id: "vc_well_west_veil", door: "to_shrine", kind: Spectral, depth: 80.0),
],
```

- **Posição:** um véu preenche a abertura de uma porta.
  - **Porta lateral:** a abertura mais `depth` unidades para dentro da sala.
  - **Porta de piso:** a própria abertura (`depth` 0). Um humano anda sobre
    ele; a forma afunda.
- **Teto:** véus em porta de teto são proibidos, porque nenhuma forma sobe.
- **Na porta, não no meio da sala:** isso mantém o validador correto. A
  sala atrás do véu é outra sala, então o grafo sabe exatamente o que o véu
  protege.
- **Chegada:** quem chega por uma porta com véu aparece **depois** do véu
  (`WorldRoom::entry_point`), onde um humano cabe.
- **Layout:** véus fazem parte do layout da sala (sempre presentes, sem
  estado). Por isso `RoomDefinition::solids()`, a restauração de posição do
  load e o spawn os tratam como sólidos para humanos.
- **Os três véus do mundo:**
  - `vg_gallery_floor_veil`: piso da Galeria Superior, tutorial;
  - `vc_well_west_veil` e `vc_shrine_east_veil`: os dois lados da
    passagem entre o Poço e o Santuário.

## 16. Progressão **[Implementado]**

| Conceito | Onde fica |
|---|---|
| Relíquia obtida | `exploration.granted` (`Ability(VeilStep)`), salvo |
| Transformação desbloqueada | derivada: `unlock` da definição atendido pelas capacidades (não é salva à parte) |
| Transformação ativa | `Transformation` (estado momentâneo, nunca salvo nem usado como flag) |
| Barreira atravessável | `world.ron` `passages: { Spectral: Ability(VeilStep) }` |
| Sala descoberta | `exploration.rooms` (inalterado) |

- **Portas com véu no validador:** uma porta com véu só é usável quando a
  passagem do tipo é atendida. Outra relíquia não basta (há teste com
  `AstralKey`).
- **`passages` e as definições:** `passages` precisa ser igual ao que as
  definições derivam (`TransformLibrary::passages`). Se não for,
  `app::load_world` recusa o mundo.
- **Travessia efetiva:** para cada véu, o load calcula o custo da travessia
  com a forma que o atravessa (velocidade, queda, ativação e manutenção) e
  recusa véus que custem mais que o MP do personagem inicial.
  - O `world_report` lista os custos: véu de piso 9,5 MP; laterais 11,1 MP;
    MP inicial 52.
  - Assim o validador não considera uma sala alcançável só porque a
    relíquia existe.
- **Problemas de véu:** `Issue::BadVeil` com `VeilProblem` (`UnknownDoor`,
  `CeilingDoor`, `Depth`, `SameDoor`, `Impassable`), além de `DuplicateId`
  para ids de véu.
- **Atalho já desbloqueado:** a passagem não muda o que os atalhos alcançam.
  `ShortcutBypassesProgression` continua sem erro, com 22 salas, 0 erros e
  0 avisos.

## 17. Save/load **[Implementado]**

- **Schema:** o save V2 não mudou, e não há save V3. A forma nunca é
  salva (`LoadRule::HumanForm`).
- **Desbloqueio:** a relíquia já estava no save (`granted`), então ele
  persiste.
- **Política:** **impedir o save em posição insegura.**
  - Com a forma ativa e o corpo humano sem caber (dentro de um véu), F5
    mostra "Cannot save inside a veil" e não grava nada. O arquivo
    anterior fica intacto e nenhum progresso se perde.
  - Fora do véu, o save funciona mesmo em forma espectral. O load volta
    humano no mesmo lugar, onde um humano cabe.
- **Segunda defesa:** a restauração do load (`world::restore`) valida a
  posição contra os sólidos da sala, véus incluídos. Uma posição dentro de
  um véu (por exemplo, um save antigo onde hoje há um véu) vira
  `PositionReset` com notificação, nunca silenciosa.
- **Migração V1 e compatibilidade V2:** os testes de save existentes
  continuam passando (`save_tests`, fixtures V1/V2). As fixtures são
  sintéticas, e os saves reais não foram lidos (§21).

## 18. Softlocks **[Implementado e testado]**

A regra que evita todos os casos é esta: **o fim da forma espera o corpo
humano caber.**
- Enquanto espera, a forma continua com a mesma física e a mesma aparência,
  sem custo de MP.
- Como o véu é atravessável e finito, o jogador sempre consegue sair dele,
  andando (véu lateral) ou afundando (véu de piso).
- Não há teleporte: o jogador nunca é movido pelo sistema de forma.

| # | Cenário | Comportamento | Teste |
|---|---|---|---|
| 1 | MP acaba dentro do véu | fim pendente (`MpEmpty`), sem custo; humano ao sair por qualquer lado | `mp_running_out_inside_a_veil_never_traps_the_player` |
| 2 | Cancelar dentro do véu | fim pendente; continua parado e espectral; humano ao sair | `ending_inside_a_veil_waits_until_the_human_body_fits` |
| 3 | Morrer dentro do véu | corpo em fase e parado; R → respawn → humano | `death_inside_a_floor_veil_holds_the_body_until_it_rises` |
| 4 | Salvar dentro do véu | recusado, nada gravado | `saving_inside_a_veil_is_refused_and_a_load_restores_the_human_form` |
| 5 | Carregar dentro do véu | o load leva à posição salva (sempre segura), em forma humana | idem |
| 6 | Transformar durante transição | recusado (`InTransition`) | `the_form_does_not_start_during_a_room_transition` |
| 7 | Atravessar sem MP para voltar | toda sala com véu tem saída humana (Poço e Santuário caem no Passadiço); a regeneração recarrega | validador (`NoWayBack`) e rota |
| 8 | Sala sem saída humana | não existe no mundo: o validador exige caminho de volta ao início em cada estágio | `the_validator_needs_the_form_unlock_for_veiled_doors` |
| 9 | Perder o controle na ativação | a ativação é instantânea e só ocorre em locomoção; dano depois dela usa as regras normais | `the_form_does_not_start_in_actions_in_the_air_or_while_dead` |
| 10 | Dano durante a reversão | dano normal; se o recuo tirar o corpo do véu, a forma termina ali | `the_form_takes_damage_like_the_human` |

## 19. Testes **[Implementado]**

- **Unitários (+9):**
  - `world::collision`: véu × corpo em fase;
  - `world::graph`: geometria dos véus e chegada depois do véu;
  - `transform::definitions` (4) e `transform::state` (3).
- **`tests/world_transform_tests.rs` (26):**
  - ativação com e sem relíquia;
  - MP insuficiente, custo inicial e contínuo;
  - cancelamento e reversão;
  - durante ataque, no ar, morto e durante transição;
  - humano preservado, perfil espectral aplicado e humano restaurado;
  - parede comum sólida;
  - véu de piso e véu lateral;
  - cancelamento e falta de MP dentro do véu;
  - troca de sala na travessia, sem entidades duplicadas;
  - combate bloqueado e liberado de novo;
  - dano;
  - morte dentro e fora do véu;
  - save recusado e load humano;
  - T + F9 no mesmo frame;
  - validador e custo de travessia;
  - performance.
- **`tests/world_spectral_route_tests.rs`:** rota real (§22).
- **Testes antigos:** três asserções de totais do mundo
  (`world_graph_tests`) passaram de 20 para 22 salas. São os mesmos
  valores por região, e só o total mudou.
- **Driver de rota:** ganhou `Expect::FormActive` e `Expect::MpAtMost`.

## 20. Performance **[Medido]**

Medido por `performance_of_the_spectral_form` (`target/tmp/world_v19_perf.txt`),
headless, perfil dev otimizado, numa máquina compartilhada (valores variam
±30% entre execuções):

| Medida | V1.9 |
|---|---|
| Quadro comum (humano) | ≈ 0,8 ms |
| Quadro com a forma ativa | ≈ 0,9 ms |
| Quadro de ativação | ≈ 1,1 ms |
| Quadro de fim | ≈ 0,8 ms |
| Passo de colisão (9 sólidos) | comum ≈ 36 ns, em fase ≈ 32 ns (mesma ordem) |
| Entidades extras com a forma | +8 (halo, contorno e ~7 partículas); 0 depois do fim |
| Travessia de véu + troca de sala, espectral | ≈ 66 ms no total (~50 quadros), ≈ 1,3 ms por quadro |
| Quadro de F5 / F9 | ≈ 0,6 / 0,7 ms (V1.8: 0,7 / 0,8) |

- **Memória (RSS máximo, `/usr/bin/time -v`, exemplos com janela):** rota
  espectral 578,5 MB; rota de segredos da V1.8 579,7 MB; autoplay 578,6 MB.
  Não houve aumento mensurável.
- **Rotas com janela:**
  - `world_spectral_route`: PASS, 14 trocas de sala, 0 reposicionamentos;
  - `world_secrets_route` (V1.8): 37 trocas, 0 reposicionamentos;
  - `world_main_route` (V1.7): 14 trocas, 0 reposicionamentos.
- **Consultas:** nenhum sistema faz consulta global cara por quadro. Todos
  leem o jogador por `Single`/`single()`, e o halo e o contorno são filhos
  dele.
- **Outras formas:** nenhum recurso de forma inexistente é carregado.

## 21. Riscos

- **Arte:** a forma reaproveita o sprite humano com tom e halo; um conjunto
  de quadros próprio exige o pipeline do Blender (V2.x).
- **Saves reais:** a compatibilidade com saves reais antigos não foi
  comprovada, porque eles não foram lidos. O caminho de defesa é o
  `PositionReset` com notificação.
- **Custo de travessia:** é verificado contra o personagem **inicial**. Um
  véu mais grosso num lugar alcançado com mais MP passaria na verificação,
  mas o custo continua listado no `world_report`.
- **Validador e física:** o validador não simula física (altura de pulo).
  Um véu de piso exige chegar sobre ele a pé, como as outras portas.
- **`TransitionLock`:** no jogo atual, a trava quase nunca fica armada mais
  de um frame. O caso foi testado com a trava armada à mão.
- **Comando de status (Tab):** continua abrindo durante a forma. A tela pausa
  o jogo, e não há troca de equipamento nela.

## 22. Futuras transformações **[Planejado]**

Para criar outra forma (voadora, terrestre veloz, pequena, aquática, de
sombra):

1. Uma variante nova em `FormId` e uma entrada em `transformations.ron`.
2. Se ela atravessar algo novo, uma variante em `BarrierKind` e a passagem
   em `world.ron`.
3. Movimento especial que o perfil não cobre, por exemplo voo ou nado:
   - um campo novo em `MovementProfile`;
   - o ramo correspondente em `transformed_locomotion`.
4. A tecla: hoje `T` alterna a forma principal. Com mais de uma forma, a
   escolha entre elas exige uma decisão de design: roda de formas ou um
   comando por forma, usando o reconhecedor de comandos.

Nenhuma dessas formas foi implementada, e não há estados vazios para elas.
