# Validação das animações técnicas (V1.2-G)

> Experimental. Correção das cascas do modelo (v007) e três animações de teste
> (`walk`, `attack_basic`, `dodge`) para validar o pipeline de sprites. Não é
> animação final e nada foi integrado ao jogo (D8). Blender 5.2.4 LTS (EEVEE),
> processos separados `--background --factory-startup`. Dados brutos
> (regeneráveis, fora do Git): `renders/v007/rig_manifest_v007.json`,
> `renders/v1_2g_animation_tests/animation_manifest.json`, `renders/v007/reopen_check_*.json`.
> Folhas e relatório automático: `comparisons/v1_2g_animation_tests/`.

## 1. Estado inicial

- Branch `art/v1.2-protagonist`, HEAD `204f965`, sem commits novos.
- Preexistentes e preservados:
  - staged: `inspiration/protagonist_concept_v001.png`;
  - alterações não commitadas das V1.2-D a V1.2-F;
  - `Md-usario/DarkCastleR-V1-2-{e,f,g}.md`, não rastreados.
- Hashes registrados antes de começar: 831 arquivos fora de `target/` e `.git/`, e o
  estado de `target/verify` e `target/f4-stage`.
- Tempos do jogo, conferidos só lendo `src/core/constants.rs` e `src/combat/attacks.rs`:
  - `BASIC_ATTACK_TIME` 0,28 s; `PLAYER_DODGE_TIME` 0,28 s;
  - `HITBOX_ACTIVE_FRACTION` 0,6; a hitbox nasce no início da ação;
  - `PLAYER_WALK_SPEED` 190 px/s; `PLAYER_DODGE_SPEED` 480 px/s.
  - O build das animações relê esses valores a cada execução e para se algum mudar.

## 2. Blender e MCP

- Blender 5.2.4 LTS, com o EEVEE funcionando.
- **MCP offline:** o Blender do usuário estava aberto, mas a porta 127.0.0.1:9876
  não estava escutando e `get_addon_status` respondeu "Could not connect". A
  sessão interativa não foi tocada.
- Pillow 12.1.1, Python 3.14.4: `resize(NEAREST)` continua quebrado, e o desvio
  `post/resample.py` continua em uso, sem mudança.
- Não instalei nada e não usei nenhum serviço externo.

## 3. Ombreira: causa e correção

### Reprodução controlada

O teste usado aqui empurra o objeto do rig por movimentos rígidos minúsculos
(1 µm em cada eixo, 1 µrad em cada eixo) e mede quanto a casca final se afasta
do movimento rígido esperado. Uma casca estável fica em ~0,002 mm (ruído de
ponto flutuante); uma instável salta milímetros. O teste é determinístico e
não depende de outros objetos. O gatilho da V1.2-F (ligar o DQS de uma sola)
já não reproduz no estado salvo do v006.

- **v006:** a ombreira salta **3,5 mm em 20 das 24 poses e testes**, ou seja,
  em toda pose que move o braço direito. Só P01, T06, T07 e T08 ficaram estáveis.
- **v007:** ≤ 0,0022 mm em todas.

### Causa

Investiguei os dez pontos do briefing.
- **Modificadores e malha:** a ordem é Armature, Solidify, Subdivision (correta),
  sem transformações não aplicadas.
- **Pesos:** constantes, sem mudança de topologia.
- **O que estava errado:** cada placa (`pauldron`, `pauldron_lame`) é uma grade em
  anéis cujo **último anel tinha 37 vértices colapsados no mesmo ponto**, em vez
  de fundidos. Isso gera 36 arestas de comprimento zero e 36 faces de área zero.
- **Por que aparece só depois do Solidify:**
  - A saída do Armature é contínua (idêntica a 2,4×10⁻⁷ m).
  - O Solidify (espessura uniforme, offset −1) precisa da normal de cada face,
    e uma face de área zero não tem normal: a direção vem do ruído numérico.
  - Por isso 217 vértices da casca de 4 mm trocam de lado, e a Subdivisão
    espalha o salto pela área do polo.
- **Onde aparece:** na geometria avaliada, não na iluminação nem no grafo de dependências.
- **Varredura das 132 malhas:** só as duas placas combinam faces degeneradas
  com Solidify. As escleras dos olhos têm arestas de comprimento zero, mas
  nenhuma face degenerada e nenhum Solidify, e são rígidas na cabeça.

### Correção

- Fundi os vértices coincidentes num único polo (`remove_doubles` a 1 µm, que só
  afeta esse grupo): 361 → 325 vértices e 360 → 324 faces por placa.
- As faces do polo viram um leque de triângulos reais.
- **O que não mudou:** pesos (os mesmos 55 % `upper_arm.R` e 45 % `clavicle.R`),
  modificadores, materiais e rig.
- **Repouso:** a casca final fica a no máximo 1,5 mm da antiga (subdivisão do polo).
- **Interseções:** as contagens da ombreira caíram, porque parte dos números do
  v006 vinha da casca invertida. Por exemplo, ombreira × manga na P05 foi de 268 para 107.
- Visualmente, nas poses estáticas a ombreira nova e a antiga são quase iguais
  (`shoulder_before_after.png`). O defeito era um salto entre estados, não uma forma errada.

### Segundo achado, pelo mesmo teste: casaca e meia-capa

- **Sintoma:** a casca da casaca saltava 4,2 mm na P09 e 0,6 mm no T10. No teste de
  esquiva, saltava **83 mm** num quadro (2,4 px na escala do jogo), e a capa 6,3 mm noutro.
- **Causa:** nenhuma face degenerada. A opção de espessura uniforme
  (`use_even_offset`) amplifica a dobra forte do tecido na cintura da frente.
- **Alternativas medidas e descartadas:**
  - limitar o fator (`thickness_clamp`) não estabilizou e mudou o repouso em até 1,2 cm;
  - normais de qualidade pioraram (19 mm).
- **Correção:** espessura simples (`use_even_offset = False`) só na casaca e na
  meia-capa. O resíduo cai para ~0,002 mm em todas as poses e quadros.
- **Repouso:** a casca muda no máximo 6,6 mm na casaca (99 % dos pontos ≤ 1,6 mm,
  ou seja, ≤ 0,05 px) e 1,2 mm na capa.
- **Masters neutros v006 × v007:** diferença média ≤ 0,0009/255 e no máximo 15 px
  acima de 8/255.
- **Efeito colateral bom:** o Médio que a esquiva tinha no quadro 1 (capa × casaca,
  46) vinha da casca da capa invertendo, e desapareceu.

## 4. Modelo

| Arquivo | Conteúdo | Situação |
|---|---|---|
| `swordsman_rig_v006.blend` | Fonte, aberta só para leitura | **Intacto** (hash igual) |
| `swordsman_rig_v007.blend` | v006 + polo das ombreiras fundido + espessura simples na casaca e na capa; mesmo rig (43 ossos, 36 de deformação, restrições e pesos), as 13 ações `POSE_` | Novo |
| `swordsman_rig_v007_animations.blend` | v007 + `ANIM_walk`, `ANIM_attack_basic`, `ANIM_dodge` | Novo; as animações não tocam o arquivo do modelo |

**Por que v007:** a correção muda geometria e modificadores, ou seja, é
persistente. O rig e a hierarquia não mudaram: a comparação automática dos
pivôs dá zero diferenças, e todas as outras malhas têm a saída do rig idêntica
à do v006 nas 24 poses.

## 5. Animações

### Regras comuns

- Cada quadro é calculado como uma pose (o sistema de poses do v005, sem
  alteração) e recebe uma chave própria, com interpolação constante. O que é
  renderizado é exatamente o que foi validado; não há interpolação do Blender
  entre chaves.
- **Sem movimento de raiz:** a raiz só se move na vertical (pés no chão). O
  deslocamento real fica com o Bevy, e os sprites ficam no lugar. Na exportação
  não há nada a remover.
- **Pernas:** IK de dois ossos no plano sagital (lei dos cossenos), calculado no
  script. O pé de apoio fica plano no chão. O rolamento na ponta ou no calcanhar
  ergue o tornozelo, em vez de afundar o pé no chão.
- **Taxa de produção:** 24 fps. O quadro i começa em t = i/24.

### Walk

- 16 quadros, 0,667 s, em loop. O quadro 16 seria igual ao 0 (fechamento medido:
  0 m) e **não é exportado**.
- **Pé de apoio:** em contato (z = 0) enquanto se apoia. Ele recua 5,4 a 5,7 cm por
  quadro, de forma uniforme; no último quadro do apoio, 6,6 cm.
- **O resto do corpo:**
  - quadril oscila 1,5 cm, duas vezes por ciclo;
  - pelve e peito giram em oposição;
  - o braço esquerdo balança contra a perna;
  - o braço da espada balança pouco;
  - casaca e capa seguem com atraso de fase.
- **Pior severidade: Baixo** (capa × manga esquerda ≤ 190 pares, em 7 quadros).
- O passo de fechamento do loop (15 → 0) mede 0,058 m, dentro da faixa dos outros
  passos. Não há salto.
- **Atenção, incompatibilidade com o jogo:**
  - O ciclo anda a 1,25 m/s, ou seja, **36 px/s**.
  - O jogo anda a **190 px/s** (6,7 m/s nesta escala, velocidade de corrida).
  - Tocado assim, o pé desliza 5,3× mais rápido que o chão.
  - Para não deslizar a 190 px/s, seria preciso um ciclo de ~0,2 s com passos
    de 0,8 m, uma corrida desenfreada.
  - É uma decisão de projeto, não do rig (ver §15).

### Basic Attack

- 7 quadros cobrindo **exatamente 0,28 s**: os quadros 0 a 5 duram 1/24 s e o
  quadro 6 dura 0,03 s.
- A hitbox fica ativa de 0 a 0,168 s (quadros 0 a 4).
- **O quadro 0 já é o golpe:** a lâmina está no arco, à frente e acima, e cruza a
  frente na horizontal no quadro 1.
  - Seguimento nos quadros 2 a 4, com o tronco ainda girando e capa e casaca abrindo.
  - Recuperação nos quadros 5 e 6, para uma guarda próxima do idle.
- **Pés:** os dois ficam no chão em todos os quadros (no máximo 5 mm; antes do IK o
  pé da frente flutuava 7 cm).
- **Interrupção:** todo quadro é uma pose válida para sair do clipe.
- **Pior severidade: Baixo** (ombreira × manga ≤ 114; manga × casaca 103).

### Dodge

- 7 quadros, exatamente 0,28 s.
- Começa já baixo e inclinado, porque a esquiva começa com a ação. Chega à
  silhueta mais longa nos quadros 2 e 3 e recupera nos quadros 5 e 6.
- A capa abre antes da casaca.
- **No jogo:** a esquiva percorre 134 px (480 px/s × 0,28 s). O sprite fica no lugar.
- **Pior severidade: Baixo** (manga × casaca ≤ 378; capa × manga ≤ 136).
- **Visual:** nos quadros 2 a 4 a capa sobe bastante atrás dos ombros. Fica
  dramático, sem interseção medida.

### Interrupção e pausa

- Nenhum quadro tem escala alterada, peça solta ou espada fora da mão (deriva ≤ 2×10⁻⁷ m).
- Nenhuma casca instável: ≤ 0,0041 mm.
- Nenhum "salto" entre quadros: casca final contra saída do rig, com tolerância de 1 cm.

## 6. FPS: 24 de produção, 12 derivado

| Clipe | 24 fps | 12 fps derivado (um quadro sim, um não) |
|---|---|---|
| Walk | 16 quadros de 41,7 ms | 8 quadros de 83,3 ms (é a estimativa do contrato) |
| Basic Attack | 7 quadros (6 × 41,7 ms + 30 ms) | Quadros 0, 2, 4 e 6 (3 × 83,3 ms + 30 ms) |
| Dodge | 7 quadros | Quadros 0, 2, 4 e 6 |

- **Duração:** a mesma do clipe nas duas taxas (0,28 s), sem esticar para um
  número inteiro de quadros. Os instantes de cada quadro estão nos JSON das spritesheets.
- **O que se perde a 12 fps:**
  - No ataque, perde-se o **quadro 1, o da lâmina horizontal**, que é a leitura mais
    clara do golpe; o arco salta do alto direto para baixo.
  - Na esquiva, perde-se o quadro 3, que é parecido com o 2.
  - Na caminhada, a leitura continua boa e o pé ainda fica plantado.
- **Tamanho:** 12 fps tem metade dos quadros (atlas e limpeza manual do A pela metade).
- **Sugestão** (não decidida):
  - 24 fps nas ações curtas de combate;
  - 12 fps em locomoção e magia longa, ou mesmo derivado da produção a 24.
- **Prévias GIF:** os atrasos são em centésimos de segundo, então 41,7 ms vira
  40/50 ms, com o erro compensado. A soma dá 670 ms contra 666,7 na caminhada, e
  280 ms exatos no ataque e na esquiva.

## 7. Acabamentos A, B e C

`finish_abc_animation_comparison.png`, spritesheets e GIFs dos três acabamentos.

- **A (pixel art):**
  - Alfa binário confirmado; contorno de 1 px; sem cortes.
  - A lâmina vira uma faixa pontilhada nos quadros de arco do ataque. O ataque em
    A precisaria de limpeza manual ou de um arco desenhado (VFX).
- **B (alta resolução):**
  - Melhor leitura do movimento e da lâmina.
  - A borda suave encosta na última linha da célula sob os pés: a linha opaca
    mais baixa é 127, para o pivô em 126. É coerente com a regra 1 do contrato
    (borda cortada sob os pés).
  - Em 1:1 o personagem fica levemente borrado.
- **C (cores chapadas):** o maior contraste da capa vinho e da lâmina contra a
  casaca e o fundo; a silhueta lê melhor na esquiva.
- **Lâmina ×1,7** (variante B do ataque, em `renders/.../sprites/B_wide_blade/`):
  ajuda a ler o arco em 1:1.

**Nenhum acabamento foi escolhido.**

## 8. Câmera

`game_scale_animation_comparison.png`, quadros-chave em 1:1, ×1,5 e ×2.

- **1:1:**
  - A esquiva e o ataque leem pela silhueta e pela capa vinho; a lâmina, só em B e C.
  - A caminhada lê, mas pernas e braços viram poucos pixels.
  - É o maior espaço visível ao redor, o que é melhor para antecipar ameaças e plataformas.
- **×1,5:** movimento claro em B e C; o A fica irregular por causa da escala fracionária.
- **×2:** tudo legível. A área visível cai para ~1/4 da 1:1, como medido na V1.1.
  Isso pode comprometer a antecipação de ameaças vindas de longe e a visão de
  plataformas acima e abaixo.

**D2 e D3 continuam pendentes.**

## 9. Casaca, capa e tornozelo

- **Casaca:** sem atravessamento Médio ou pior nas três animações. A rigidez
  vista no T10 não aparece em nenhuma delas, então fica registrada como
  **limitação aceita provisoriamente**.
- **Meia-capa:** funciona na caminhada e no ataque. Na esquiva, abre de forma ampla.
- **Tornozelo:**
  - Volume do pé ≥ 0,986 da referência em todos os quadros; ≥ 1,0 na caminhada,
    onde o *dual quaternion* infla um pouco.
  - A P13 continua com 0,938, como no v006.
  - Sem colapso visível; sola, salto e biqueira estão preservados.
  - Não mexi na bota.

## 10. Exportação de sprites (teste, sem integração)

- **Nomes:** `<clipe>_<NNN>.png` a partir de 000, em `renders/v1_2g_animation_tests/sprites/<A|B|C>/<clipe>/`.
- **Formato:** PNG RGBA sRGB sem perfil ICC, alfa reto.
- **Células e pivôs**, conforme o contrato:
  - `walk` e `dodge` na célula de movimento 64×64, pivô (32, 63). O contrato
    classifica `dodge` em `locomotion`, e foi seguido.
  - `attack_basic` na célula de combate 128×96, pivô (40, 95).
  - B e C com densidade 2: células de 128×128 e 256×192.
- **Verificações:**
  - nenhum master toca as bordas esquerda, direita ou superior, ou seja, nenhum
    corte em espada, casaca ou capa;
  - a linha da sola cai no pivô;
  - nenhum quadro duplicado (nem o último igual ao primeiro na caminhada).
- **Spritesheets** (`comparisons/v1_2g_animation_tests/spritesheets/`):
  - grade do contrato, com 8 colunas na locomoção e 7 no combate, por linhas;
  - 2 px de padding só entre células;
  - todas ≤ 2048 px de lado; a caminhada B tem 1038×258;
  - cada uma com um JSON: célula, densidade, pivô, colunas, linhas, `first`/`count`,
    instantes e durações a 24 fps, seleção de 12 fps, `mode` (`Loop`/`FitToAction`)
    e movimento de raiz.
- **Prévias:** `previews/<clipe>_<A|B|C>_<12|24>fps.gif`, geradas só a partir dos renders locais.
- **Não fiz:** o atlas por grupo (`player_locomotion.png`) e o manifesto RON. Ficam
  para a V1.3, e nada foi escrito em `assets/`.

## 11. Validações

### Automáticas

Todas executadas e passando:
- **Fonte e modelo:**
  - v006 conferido (43/36 ossos, pesos, hash da geometria do v004);
  - v007 conferido pelos hashes do próprio build antes de animar;
  - pesos normalizados, ≤ 4 influências;
  - IK OK; pivôs inalterados.
- **Cascas:** estáveis em todas as 24 poses e em todos os quadros.
- **Movimento:** espada presa; sem pulos entre quadros; sem escala alterada.
- **Clipes:**
  - loop fechado;
  - 0,28 s exatos no ataque e na esquiva, nas duas taxas;
  - pés no chão em todos os quadros (z mais baixo = 0).
- **Sprites:** sem cortes; dimensões, pivôs, alfa e nomes corretos; spritesheets ≤ 2048 px.
- **Reabertura em outro processo**, 0 falhas nos dois arquivos:
  - **modelo:** polos fundidos, espessura simples, ações `POSE_` estáveis;
  - **animações:** chaves em todo quadro, interpolação constante, intervalos de
    quadros, hash das chaves igual ao do build, todos os quadros repetidos.
- **Preservação:** arquivos protegidos com hashes iguais; nenhum Rust, RON ou save
  tocado (§14).

### Visuais

Conferi as folhas de contato (lateral e 3/4 de costas), a de acabamentos, a de
escala, as de 12 × 24 e a da ombreira. Os quadros intermediários são todos
exportados e inspecionados, porque cada quadro é uma pose própria.

## 12. Reprodutibilidade

| Etapa | Execuções | Resultado |
|---|---|---|
| Build v006 → v007 | 2 | Manifesto idêntico (estrutura, pesos, hashes de geometria e pesos, resíduos, interseções das 24 poses); reabertura OK nas duas |
| Build das animações | 2 (a segunda em pasta temporária, sem gravar `.blend`) | Hash das chaves idêntico nos 3 clipes; validação quadro a quadro idêntica; 74 masters com pixels idênticos (diferença 0), mas arquivos PNG não idênticos byte a byte (metadados do render) |
| Pós-processamento | 2 sobre os mesmos masters, mais 1 sobre os masters da segunda execução | 133 PNG/GIF idênticos byte a byte; os 90 sprites da segunda execução idênticos aos da primeira; relatório JSON idêntico (sem caminhos absolutos) |

O EEVEE não variou entre as execuções nesta máquina. Ainda assim, a identidade
afirmada é a dos pixels, não a dos arquivos dos masters.

## 13. Problemas restantes e riscos para a V1.3

1. **Velocidade da caminhada do jogo (190 px/s) × ciclo natural (36 px/s):** os pés
   deslizam. É decisão de projeto: ajustar o ciclo, tratar "walk" como trote ou
   corrida, ou aceitar o deslizamento.
2. **Lâmina do ataque no acabamento A:** fica pontilhada no arco; precisa de
   limpeza ou de um arco de VFX.
3. **12 fps no ataque:** perde a pose horizontal do golpe.
4. **Capa na esquiva:** sobe bastante atrás dos ombros. É aceitável, mas vale
   revisão artística.
5. **Espessura simples na casaca e na capa:** nas bordas muito curvas a casca fica
   até 6,6 mm mais fina que antes. Invisível na escala do jogo.
6. **Escleras dos olhos com arestas de comprimento zero:** sem Solidify, e rígidas;
   inofensivo hoje, mas um risco se algum dia receberem modificadores.
7. **Fora deste teste:** atlas por grupo, manifesto RON, `flip_x` com anchor,
   controlador de animação e `FitToAction` com `attack_speed`.

## 14. Arquivos

**Criados:**
- Scripts:
  - `scripts/bpy/build_swordsman_rig_v007.py`;
  - `scripts/bpy/build_animations_v007.py`;
  - `scripts/bpy/validate_rig_v007.py`;
  - o pacote `scripts/bpy/swordsman_rig_v007/` (`rig_config`, `shell_fix`, `views_v007`, `animations`);
  - `scripts/post/animation_sheets_v1_2g.py`.
- Este arquivo.
- `comparisons/v1_2g_animation_tests/`: folhas, `spritesheets/`, `previews/` e
  `animation_validation_report.json`.
- Fora do Git (ignorados):
  - `blender/swordsman_rig_v007.blend` e `swordsman_rig_v007_animations.blend`;
  - `blender/swordsman_rig_v007.blend1`, backup automático da primeira gravação do v007;
  - `renders/v007/` e `renders/v1_2g_animation_tests/`.

**Modificados:** só seções da documentação (`art-src/README.md`, `RIG_SPECIFICATION.md`
seção 12, `docs/ART_DIRECTION.md`, `docs/ASSET_CONTRACT.md`).

**Preservados:**
- os `.blend` do v001 ao v006 e os seus backups;
- os scripts e pacotes do v003 ao v006;
- as folhas das versões anteriores;
- gameplay, RON, saves, `Md-usario/`, `.cargo/`, `target/verify`, `target/f4-stage`.

## 15. Decisões pendentes

D2 (acabamento), D3 (zoom da câmera), a variante da espada, a taxa de quadros,
a velocidade de caminhada contra o ciclo, o versionamento dos `.blend` e os commits.
