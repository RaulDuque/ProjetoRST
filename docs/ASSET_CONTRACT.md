# Contrato de assets V1.2: sprites do protagonista

> **Status: preliminar, aguardando aprovação.** Esta é a **fonte única** para
> formatos, nomes, pivôs, filtros, atlases e manifesto dos sprites. Outros
> documentos apontam para cá em vez de repetir valores. Os scripts em
> `art-src/` leem estes números de `art-src/player/scripts/post/contract.py`,
> que deve mudar junto.
>
> Marcadores:
> - **[COND: acabamento]** depende da escolha entre pixel art e alta resolução (D2, em aberto);
> - **[COND: câmera]** depende da decisão definitiva de zoom. Hoje a D3 mantém
>   a câmera 1:1 de forma **provisória**, e o ×2 está só em estudo.
>
> Nada aqui está integrado ao jogo. O manifesto **não** é carregado pelo
> código, e nenhum RON existente foi alterado. As referências ao código do
> Bevy são da versão 0.20.1, conferidas no código-fonte do crate.

## 1. Estrutura de diretórios

```
assets/                          # carregado pelo jogo (somente arquivos finais)
  sprites/player/player_<grupo>.png
  animations/player.ron          # manifesto (seção 7); local a registrar em DECISIONS.md
art-src/                         # fontes de produção, nunca carregadas pelo jogo
  player/
    blender/    <modelo>_<vNNN>.blend
    scripts/bpy/   geometria, materiais, câmera, luz, render, exportação, prévias
    scripts/post/  contrato, paleta, pixelização, folhas de comparação
    references/ referências com origem e licença registradas
    renders/    saídas geradas (ignoradas pelo Git)
    comparisons/ folhas de comparação para aprovação
```

`assets/sprites/` e `assets/animations/` já estavam previstas em
`assets/README.md`. Pastas só são criadas quando houver o primeiro arquivo real.

**V2.2:** o manifesto pode ter `weapon_in_body: false` (corpo sem arma) e,
então, publica junto `assets/animations/player_sockets.ron` (âncoras por
quadro; formato e unidades em [`VISUAL_EQUIPMENT_V2_2.md`](VISUAL_EQUIPMENT_V2_2.md) §5).
Ausente = `true` (sabre desenhado, como até a V1.5).

## 2. Nomes e lista de animações

- **Regra geral:** ASCII, `snake_case`, sem espaços nem acentos.
- **Atlas:** `player_<grupo>.png`, com os grupos `locomotion` e `combat`.
  O grupo `effects` guarda os VFX desde a V1.5 (`player_effects.png`, densidade 1,
  célula 128×96 u com o pivô do combate; clipes `vfx_<chave>`, ver `PLAYER_ART_POLISH_V1_5.md`).
- **Identificadores de animação:** únicos no manifesto. Os que já aparecem nos
  RON do jogo têm de ser usados exatamente iguais.
- **Variantes:** sufixo, por exemplo `attack_basic_air` e `slash_rising_air`.
- **Quadros exportados:** `<animacao>_<NNN>.png`, de `000` em diante
  (`export_frames.py` valida o identificador).
- **Modelos:** `<nome>_<vNNN>.blend`, com a versão em `scripts/bpy/config.py`.

**Lista de animações** (prioridades e quadros da matriz da Missão B; os
quadros são **estimativas preliminares**):

| Grupo | Identificador (quadros) | Total |
|---|---|---|
| `locomotion` | `idle` 6, `walk` 8, `run` 8, `turn` 3, `crouch` 3, `jump_start` 2, `jump_rise` 2, `jump_apex` 2, `fall` 2, `land` 3, `dodge` 6, `hurt` 4, `guard_start` 2, `guard_loop` 4, `guard_end` 2, `guard_impact` 3, `death` 6, `dead` 1, `respawn` 4 | 71 |
| `combat` | `attack_basic` 6, `attack_basic_air` 6, `slash_crescent`\* 8, `slash_rising`\* 8, `cast_forward`\* 5, `cast_up`\* 8 | 41 |
| | | **112** |

\* Chave já usada no RON (`assets/data/skills/sword_techniques.ron` e
`assets/data/spells/elemental.ron`).

A variante `slash_rising_air` entra na V1.7 e não está na contagem.

## 3. Formato da imagem e filtro

| Item | Regra |
|---|---|
| Arquivo | PNG RGBA, 8 bits por canal. O carregador já está nas features do projeto: `2d` → `2d_api` → `common_api` → `png` |
| Espaço de cor | sRGB (padrão do carregador). No Blender: transformação *Standard*, sem Filmic/AgX |
| Alfa | **Reto** (não pré-multiplicado). A: binário (0 ou 255). B: suave permitido **[COND: acabamento]** |
| Borda no B | Os texels transparentes devem levar a cor da borda ("sangria de cor"); senão o filtro linear escurece o contorno. **Pendente:** `pixelize.py` ainda não faz isso |
| Metadados | Sem perfis ICC extras; o PNG final é só imagem |
| Orientação | Todas as animações desenhadas **olhando para a direita** |
| Filtro **[COND: acabamento]** | A: *nearest*. B: *linear* (padrão do Bevy). Para não afetar UI e fontes, prefira o sampler **por imagem** a `ImagePlugin::default_nearest()`, que muda todas as imagens. O efeito global não foi verificado |
| Mipmaps | Nenhum |

## 4. Resolução, densidade e escala **[COND: acabamento] [COND: câmera]**

**Unidade de mundo:** 1 u = 1 px de tela na câmera 1:1. O corpo de colisão
mede 26×46 u, centrado na posição da entidade (`src/world/physics.rs`). O
personagem desenhado tem ~52 u de altura, porque o cabelo passa do corpo.

| | A: pixel art | B: alta resolução |
|---|---|---|
| Densidade | 1 texel por u | 2 texels por u |
| Célula de movimento | 64×64 | 128×128 |
| Célula de combate (ataques e magias) | 128×96 | 256×192 |

**Por que duas células** (valores do código e dos RON):
- **Golpe básico:** alcança 46 + 8 = 54 u à frente do centro.
- **Crescent Slash:** 70 + 6 = 76 u.
- **Rising Edge:** o topo da hitbox fica 93 u acima da sola. Com o pivô em
  y = 95, sobram só **2 u** de margem na célula de combate.

Uma grade única de 128×96 ocuparia 3× a área nos 71 quadros de movimento
(2× de desperdício).

**A resolução não está congelada.** Ela muda se a altura do personagem ou a
câmera mudarem.

## 5. Pivô, alinhamento e exibição no Bevy

**Definição:**
- O pivô é um ponto **na linha da sola**, em coordenadas contínuas de pixel a
  partir do canto superior esquerdo da célula.
- É declarado por animação no manifesto.
- O corpo deve ocupar a mesma posição em relação ao pivô em todos os quadros.

| Célula | Pivô (A) | Pivô (B) |
|---|---|---|
| Movimento 64×64 | (32, 63) | (64, 126) |
| Combate 128×96 | (40, 95) | (80, 190) |

**Regras:**

1. **A sola fica na linha y do pivô.** No A, a linha 63 fica para o contorno
   de 1 px, que pode encostar 1 px no chão. No B, a borda suave sai cortada
   sob os pés (sobram 2 texels); isso é aceito por enquanto.
2. **O `Anchor` é um componente** exigido pelo `Sprite`, não um campo dele.
   Conversão (y para cima, de −0,5 a 0,5): `anchor = (px / w − 0,5, 0,5 − py / h)`.
   Exemplo: (32, 63) em 64×64 vira (0, −0,484).
3. **Espelhamento:** `flip_x` só inverte as coordenadas de textura; o retângulo
   não se move. Quando o pivô não está centralizado (células de combate),
   **`anchor.x` troca de sinal junto com `flip_x`**, no mesmo sistema que
   atualiza o `Sprite`. Isso precisa ser uma função pura, com teste.
   - *Alternativa:* células centradas no pivô (256 de largura no A), que desperdiçam ~40% da área.
4. **Densidade:** `Sprite.custom_size = célula / densidade`, nunca a escala
   do Transform. O anchor normalizado não muda. `custom_size`, `Anchor` e o
   índice do atlas são atualizados juntos, porque a célula muda entre os grupos.
5. **Entidade visual:** o sprite vive num filho (`PlayerVisual`) com
   `translation.y = −23` (metade da altura do corpo), para a sola encostar no
   chão. Enquanto o sprite estiver ativo, a raiz do jogador fica com escala 1.
   Hoje o visual temporário grava `scale.y` na raiz (`src/player/animation.rs`),
   o que faria o pé flutuar e o sprite achatar; o achatamento precisa ir para o
   próprio visual temporário. Isso fica para a V1.3.
6. **Pixel inteiro (A):** a câmera hoje não arredonda a posição
   (`follow_player` usa `smooth_nudge` em `src/world/camera.rs`). Arredondar no
   filho exige compensar a fração da raiz: `round(raiz) − raiz`.

## 6. Atlases

| Estratégia | Desperdício | Manutenção | Avaliação |
|---|---|---|---|
| Atlas único | Alto: a grade única é ditada pelas células de combate | Um arquivo; qualquer mudança regera tudo | Não recomendado |
| **Atlas por grupo** (`locomotion`, `combat`, depois `effects`) | Baixo dentro do grupo | Mudanças locais; poucos handles | **Recomendado** |
| Um atlas por animação | Mínimo | Muitos arquivos e handles | Só para exceções muito grandes |

**Grade:**
- **Construção:** `TextureAtlasLayout::from_grid(tile: UVec2, columns, rows, padding: Option<UVec2>, offset: Option<UVec2>)`.
- **Padding:** fica **só entre as células**, sem borda externa. O contrato usa
  `padding = Some(UVec2::splat(2))` e `offset = None`.
- **Tamanho da imagem:** `colunas·célula + (colunas−1)·2` × `linhas·célula + (linhas−1)·2`, sem sobra no fim.
- **Ordem dos índices:** por linha (linha por fora, coluna por dentro).

**Organização dos quadros:**
- **Recomendado: sequencial.** O manifesto guarda `first` e `count`, e o
  índice é `first + n`.
  - Movimento: 71 quadros em 8×9 = 72 células, ~1% de sobra.
- *Alternativa:* uma linha por animação. É mais fácil de inspecionar, mas
  desperdiça ~53% no grupo de movimento (19 linhas × 8 colunas para 71 quadros).
- **Índice fora do intervalo:** o Bevy **não dá erro**, apenas prende o índice
  no último quadro. Por isso a validação 3 (seção 7) é obrigatória.

**Tamanhos** (calculados pela fórmula acima, não medidos):

| | Movimento (8×9) | Combate (7×6) |
|---|---|---|
| A | 526×592 | 908×586 |
| B | 1038×1168 | 1804×1162 |

- **Limite de textura:** a meta conservadora é **≤ 2048 px por lado**. O
  limite real do wgpu não foi verificado.
- **Memória:** por padrão o Bevy pode manter também uma cópia da imagem na RAM
  (`RenderAssetUsages`), além da cópia na GPU. A estimativa da direção de arte
  (~3 MiB no A, ~12 MiB no B) conta só uma cópia.

## 7. Manifesto RON

> **V1.3:** integrado para `idle`, `run`, `attack_basic` e `dodge`
> (`assets/animations/player.ron`, gerado por
> `art-src/player/scripts/post/game_sprites_v1_3.py`; carregado por
> `src/player/visual/`). Duas extensões da proposta abaixo:
> - `durations`, a duração de cada quadro (os clipes de ação terminam com um quadro mais curto);
> - `speed_reference`, a velocidade para a qual um loop de locomoção foi feito.
>
> Arquivo ausente ou inválido mantém o visual provisório (D-015).
> **V1.4:** mais 11 clipes (`hurt`, `death`, `jump_rise`, `fall`, `land`, `crouch`,
> `guard_start`, `slash_crescent`, `slash_rising`, `cast_forward`, `cast_up`), com o
> modo `Once(fps)` e sem mudança de contrato (D-016,
> `docs/PLAYER_ANIMATION_EXPANSION_V1_4.md`). Ver
> `docs/PLAYER_VISUAL_ANIMATION_V1_3.md`.

**Arquivo:** `assets/animations/player.ron`. É dado de apresentação, fora de
`assets/data/`, o que exige registrar a decisão em `DECISIONS.md` e atualizar
a seção "Dados" da arquitetura quando for integrado.

```ron
// Proposta V1.2: valores para o acabamento A; não é carregado pelo jogo.
(
    version: 1,
    facing: Right,
    atlases: {
        "locomotion": (texture: "sprites/player/player_locomotion.png",
                       cell: (64, 64), columns: 8, rows: 9, padding: 2, density: 1),
        "combat":     (texture: "sprites/player/player_combat.png",
                       cell: (128, 96), columns: 7, rows: 6, padding: 2, density: 1),
    },
    clips: {
        "idle":           (atlas: "locomotion", first: 0,  count: 6, mode: Loop(fps: 8.0),
                           pivot: (32.0, 63.0), fallback: None),
        "walk":           (atlas: "locomotion", first: 6,  count: 8, mode: Loop(fps: 12.0),
                           pivot: (32.0, 63.0), fallback: Some("idle")),
        "attack_basic":   (atlas: "combat",     first: 0,  count: 6, mode: FitToAction,
                           pivot: (40.0, 95.0), fallback: Some("idle")),
        "slash_crescent": (atlas: "combat",     first: 12, count: 8, mode: FitToAction,
                           pivot: (40.0, 95.0), fallback: Some("attack_basic")),
        "death":          (atlas: "locomotion", first: 60, count: 6, mode: Once(fps: 12.0),
                           pivot: (32.0, 63.0), fallback: Some("dead")),
        // ... demais identificadores da seção 2
    },
)
```

| Campo | Significado |
|---|---|
| `mode` | `Loop(fps)` repete; `FitToAction` estica até a duração da ação do gameplay (sem fps próprio); `Once(fps)` toca uma vez e segura o último quadro |
| `pivot` | Seção 5, na densidade do atlas |
| `fallback` | Animação usada quando esta falta; a cadeia deve terminar em `idle` |
| `padding` | Inteiro no RON, convertido para `Some(UVec2::splat(n))` |

**Onde fica cada parte** (V1.3, conforme o parecer do `game-architect`):

| Parte | Local |
|---|---|
| Tipos puros e validação estrutural, sem Bevy: `AnimationManifest`, `ClipMode`, `ClipKey` (newtype sobre `String`, como os outros ids de conteúdo), `ManifestError`, função anchor/flip | `src/player/animation/manifest.rs` |
| Carga e validação cruzada, com `DataError::Animation` | `GameData::load` / `validate` (`src/core/resources.rs`) |
| Criação dos handles de `Image` e `TextureAtlasLayout` | Sistema de Startup no lado Bevy de `player/animation/` |

**Testes sem janela:** `tests/app_smoke_tests.rs` sobe `MinimalPlugins`, sem
`AssetPlugin`. Por isso:
- os sistemas novos usam `Option<Res<AssetServer>>`;
- ou a camada visual fica num plugin registrado só em `app::build`.

**Arquivo ausente ou inválido — decisão pendente.** Proposta:
- **arquivo ausente:** o jogo usa o visual atual;
- **arquivo presente mas inválido:** erro de dados que encerra o jogo, como os outros RON;
- **em tempo de execução:** a cadeia de `fallback` só cobre animação faltante.

Enquanto a textura não carregar, o jogo mostra o visual atual. Sem isso, o
jogador sumiria por alguns quadros.

**Regras de validação** (futuros testes de dados, sem janela):
1. Toda chave `animation` não vazia dos RON de habilidades e magias existe em `clips`.
2. Os identificadores são únicos e seguem a regra de nome.
3. `first + count ≤ columns × rows` do atlas.
4. O pivô fica dentro da célula.
5. A cadeia de `fallback` termina, sem ciclos.
6. Toda `texture` existe no disco.
7. `fps > 0` em `Loop` e `Once`.
8. `FitToAction` só é usado em animações ligadas a ações com timer.
   **Não dá para verificar só pelo manifesto:** vira um teste do mapeamento
   entre `PlayerState` e animação, no código.

## 8. Pipeline de exportação reproduzível

1. **Modelo:** `blender --background --factory-startup --python art-src/player/scripts/bpy/build_base_model.py`.
   - Gera o `.blend` versionado e três masters de 1024×1024 (64 px × 16),
     `<modelo>_<vNNN>_{side,front,back}_master.png`.
   - Exige EEVEE; `--allow-workbench` aceita um visual diferente.
   - Recusa sobrescrever arquivos da mesma versão.
   - Também renderiza vistas de avaliação, fora do contrato de sprite:
     `<modelo>_<vNNN>_three_quarter.png` e `<modelo>_<vNNN>_detail_<parte>.png`
     (`presentation.py`, mesma luz, fundo transparente).
   - v003 (V1.2-C): `build_swordsman_v003.py`, saídas em `renders/v003/`
     com os mesmos nomes de master (`<modelo>_v003_{side,front,back}_master.png`),
     closes `_close_<parte>`, variantes `_side_master_toon` e
     `_side_master_readable`, e o relatório `build_report_v003.json`.
   - v004 (V1.2-D): `build_swordsman_v004.py`, saídas em `renders/v004/` com os
     mesmos nomes do v003, mais `_close_face_front`, `_close_face_profile`,
     `_close_torso` e `_side_master_readable_toon`.
2. **Animações (V1.4+):** `export_frames.export_animation(id, início, fim)` gera os quadros numerados.
   - V1.2-E: rig experimental `swordsman_rig_v005.blend` (poses e testes de
     movimento, sem spritesheets); célula de combate renderizada com
     `contract.COMBAT_FRAME`/`COMBAT_PIVOT`. Ver `art-src/player/RIG_SPECIFICATION.md`.
   - V1.2-F: rig experimental `swordsman_rig_v006.blend` (correções do v005, ainda sem
     spritesheets); mesmo contrato de células e pivôs. Ver a seção 11 de
     `art-src/player/RIG_SPECIFICATION.md`.
   - V1.2-G: primeiro teste do caminho de exportação (sem integração): `walk` e `dodge`
     na célula de movimento, `attack_basic` na de combate. Quadros `<animacao>_<NNN>.png`,
     spritesheets na grade do contrato (8 ou 7 colunas, padding 2 entre células) com um
     JSON de célula, pivô, densidade, instantes e durações; ver
     `art-src/player/ANIMATION_VALIDATION_V1_2G.md`. Clipes de ação duram o tempo exato do
     gameplay (0,28 s = 6 quadros de 1/24 s + 1 de 0,03 s), sem movimento de raiz.
3. **Acabamento:** `post/pixelize.py` gera os dois:
   - A: cada pixel é encaixado na paleta, depois vale a cor dominante do bloco; alfa binário e contorno;
   - B: redução Lanczos para 2× densidade.
4. **Limpeza manual** (só no A) e conferência visual.
5. **Empacotamento** no atlas do grupo (ferramenta a definir na V1.3; hoje não há empacotador).
6. **Manifesto** atualizado e validado pelos testes da seção 7.

A versão do Blender e o motor usado ficam em `renders/blender_version_<vNNN>.txt`.

## 9. Versionamento

- **Vão para o Git:**
  - em `assets/`: PNG finais e manifesto;
  - em `art-src/`: scripts, READMEs e as folhas de comparação aprovadas.
- **Ignorados por `art-src/.gitignore`:** `renders/`, que é regenerável, e `__pycache__/`.
- **Arquivos `.blend`:** ignorados até a decisão sobre Git LFS.
- **Mudança de modelo:** incrementa `MODEL_VERSION`. O script recusa sobrescrever masters da mesma versão.
- **Publicação (V1.5):** só `publish_guard.py` escreve os sprites e o manifesto
  em `assets/`, a partir do staging de um script de pós-processamento, com
  validação, versão mínima, clipes obrigatórios, backup e rollback. O lock
  `assets/animations/player.publish.json` vai para o Git junto com os PNG.
- **Referências:** registram origem e licença em `references/README.md`.
  Nenhuma imagem de terceiros vira textura final sem licença compatível.

## 10. Dependências

| Dependência | Situação |
|---|---|
| Crates novas | Nenhuma. Sprite, atlas e PNG já estão nas features atuais (seção 3) |
| Blender | 5.2.4 LTS instalado (V1.2-B). Passo 1 validado no Blender real; passo 2 ainda não (sem rig) |
| Pillow (Python) | Instalado (12.1.1); usado pelos scripts `post/` |
| Empacotador de atlas | Ainda não escolhido (V1.3) |
| Decisões | D2 e a decisão definitiva de câmera bloqueiam o congelamento das seções 3 a 5. A decisão "manifesto ausente × inválido" bloqueia a seção 7 |
