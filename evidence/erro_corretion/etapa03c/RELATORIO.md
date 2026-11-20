# Erro-Corretion — Etapa 03C: avaliação do Agent Sprite Forge e do PixelFX

**Estado:** as duas ferramentas foram instaladas, validadas e testadas.

- Os testes rodaram só localmente, sem nenhuma chamada paga.
- Nada do jogo foi alterado: `src/`, `assets/` e as animações existentes continuam como estavam.
- O pipeline Blender da 03B também não foi tocado.
- O Prompt 04 não foi iniciado.

## O que foi instalado

| Item | Onde | Observação |
|---|---|---|
| Plugin **agent-sprite-forge** 0.6.0 (commit `25203d0`) | `~/.claude/plugins/` (escopo de usuário) | Rota oficial para Claude Code: `claude plugin marketplace add 0x0funky/agent-sprite-forge --sparse .claude-plugin skills`. Traz 5 skills: generate2dsprite, video2dsprite, generate2dmedia, generate2dmap e codeart2d. |
| Plugin **pixelfx** 0.3.1 (commit `5294518`) | `~/.claude/plugins/` (escopo de usuário) | Rota oficial `/plugin marketplace add patch-gg/pixelfx`. O próprio plugin é o checkout usado pelas ferramentas Node. Usa o Node 24, que já estava instalado; nenhum `npm install` foi necessário. |
| Ambiente Python isolado | `~/.local/share/dc-art-tools/asf-venv` (621 MB, criado com `uv`) | Python 3.11 com numpy 2.4, Pillow 13, scipy 1.17 e `static-ffmpeg` (ffmpeg e ffprobe 8.0). A máquina não tinha ffmpeg; nada foi instalado no sistema. |

- **O que não foi instalado:**
  - nenhuma chave de API;
  - nenhum CLI do Codex ou do Grok;
  - nada no `apt`;
  - nada dentro do projeto.
- **Skills:** as do plugin carregam numa **nova sessão** do Claude Code. Nesta sessão os scripts foram executados direto da pasta do plugin.

### Validação
- **Testes do repositório ASF:** 299 de 299 passaram no ambiente isolado (pixel_look, pixel_motion, video2dsprite, generate2dsprite, pacotes de skill, registro, retime, gait_loop e export).
- **`video2dsprite.py doctor`:** ffmpeg 8.0 com VP9 alfa e libx264 funcionando (`out/doctor_video.json`).
- **`forge_doctor.py`:**
  - nenhuma chave configurada;
  - sem Codex ou Grok;
  - imagem, edição e vídeo **sem rota**, ou seja, nenhuma geração é possível e nenhum custo pode ocorrer (`out/doctor_routes.txt`).
- **`pixelfx tools/check.mjs` (melee, arcane, shadow):** 36 de 36 efeitos passaram (`out/pixelfx_check.txt`).

### Rotas pagas: o que existe, limites e custo (verificado, não usado)
O ASF decide a rota assim:

1. **API**, se existir chave em `~/.config/agent-sprite-forge/config.json` ou no ambiente: OpenAI, Gemini, xAI, BytePlus e fal.ai. **A chave conta como consentimento: o ASF não pergunta antes de cada chamada.**
2. **CLI local** logado: Codex `image_gen` ou Grok image-to-video. Gasta a cota da assinatura.
3. **codeart2d**: arte desenhada por código, usada como último recurso.

Hoje não existe nenhuma das duas primeiras rotas. Os itens que dependem de geração (`ask-master`, `cast`, `pose` e `view`, clipes novos de image-to-video e `sprite_set.py run`) **não foram testados com geração**.

**Recomendação:** se um dia uma chave for configurada, fixar a ordem dos provedores e usar `route_media.py resolve` antes de qualquer chamada, porque o ASF não pede confirmação.

## Teste A — Agent Sprite Forge (offline, entradas aprovadas)

| Teste | Funcionalidade | Resultado |
|---|---|---|
| A1 | `video2dsprite` (triage, process) + `gait_loop select` + `finish_frames pixel/hibit`, sobre o **vídeo original** da corrida | Recorte do fundo verde sem franja. Loop de **12 quadros** escolhido sozinho, com confiança alta (mesmo comprimento do ciclo usado na N3.6). Escala 0,2367 contra 0,2366 da N3.6. O finish **pixel** saiu com 22 cores e contorno seletivo; o **hibit** com 17 cores. |
| A2 | `pixel_motion breath` sobre o idle N3.5 | Idle de 2 desenhos gerado do próprio sprite: a metade de cima desce uma célula. É funcional, mas simples (`out/asf_idle_breath.gif`). |
| A3 | `pixel_motion prepare/frames/finish`: os 6 desenhos do run6 viraram um vídeo H.264 (sprite 5× sobre magenta) e foram lidos de volta | Ciclo de 0,5 s encontrado. Mesmas caixas e alinhamento. **91,7 % a 94,8 %** dos pixels idênticos. As trocas ficam nos tons escuros da calça, porque a paleta vem só do desenho 0. |
| A4 | `pixel_look master` sobre o idle ampliado 11×, borrado e salvo em JPEG | Grade reencontrada (célula de 11,67 px): 23×107 contra 23×106. 79 % dos texels ficaram próximos do original. O sprite sai com até **48 cores** (o original tem 17) e o contorno é fechado por regra. |
| — | `generate2dsprite` | Só tem valor com geração de imagem, que não está disponível. Não foi testado. |

**Comparação com o que já existe:**
- A N3.6 tem 6 desenhos com a paleta N3.4 e uma redução própria, aprovada.
- O ASF produz o ciclo completo do vídeo com 12 quadros a 24 fps, de forma automática e reproduzível.
- O finish **pixel** do ASF escurece a calça (quase preta) e acinzenta o cabelo. A paleta aprendida não é a N3.4, e o comando `pixel` não aceita uma paleta pronta; só o `hibit` aceita.
- O **hibit** fica mais próximo da leitura da N3.6, mas a cara e as mãos perdem definição.
- Nenhum dos dois supera a N3.6 em identidade. O ganho está nas **ferramentas de medição e encaixe**: loop por medida, registro, retime e pacote.

## Teste B — PixelFX

| Efeito | Configuração | Resultado |
|---|---|---|
| `melee/sword-slash` | Rampa violeta-prata, raio 64 | 24 quadros, 0,79 s, 30 fps, pivô (44, 92), contém dados de impacto (hit em 0,06 s, freeze de 0,14 s, shake) |
| `arcane/magic-circle-cast` | Rampa roxo → dourado | 118 quadros, 3,9 s, pivô (100, 192), círculo dourado e pilar violeta |

- **O que o PixelFX exporta:** PNG transparente em grade com JSON estilo Aseprite (retângulos, duração de cada quadro em ms, fps, loop, pivô, colunas e tempo de golpe). Está em `out/pixelfx/`.
- **Demonstração sobre o protagonista aprovado:** `out/pixelfx_in_game_demo.gif`, comparando com o `vfx_arc_white` atual.
- **Ponto de atenção:**
  - a camada de luz aditiva gera **milhares de cores** (4.883 e 53.574), com bordas suaves;
  - o pivô corresponde aos pés do boneco de 38 px do PixelFX, então o corte precisa ser **elevado cerca de 40 texels** para o nosso herói de 106 px.

### Como integrar ao Rust/Bevy (sem trocar de engine)
O jogo **já tem tudo o que é preciso**:
- `assets/animations/player.ron` define atlas (`AtlasDef`) e clipes `vfx_<chave>` (`ClipDef` com `Once(fps)`, pivô e durações);
- `src/player/visual/vfx` cria o efeito a partir do campo `vfx` da skill ou magia e ajusta o tempo à duração real da ação;
- os efeitos só observam o combate: não causam dano nem decidem acertos.

Para usar um efeito do PixelFX, basta converter dados, sem código Rust novo:

1. Rodar `scripts/pixelfx_to_manifest.py` (já testado aqui). Ele:
   - reorganiza a folha no formato `AtlasDef` (padding 2, tamanho conferido);
   - reduz de 30 para 15 fps (corte) ou 10 fps (círculo), mantendo o tempo total;
   - **reduz as cores à rampa do efeito com alfa binário:** de 4.883 para 7 cores e de 53.574 para 5;
   - gera o trecho RON (`out/bevy_proposal/manifest_fragment.ron.txt`).
2. Depois da sua aprovação: copiar o PNG para `assets/sprites/player/`, colar o atlas e o clipe no `player.ron` e apontar `vfx: "pixelfx_slash_gothic"` na técnica.

**Não verificado:**
- O trecho RON ainda não foi carregado pelo jogo, e não rodei cargo, porque isso alteraria `assets/`.
- Também falta conferir:
  - se um pivô fora da célula, como o (44, 132) com a elevação, é aceito pelo jogo;
  - o efeito visual da densidade 2 no lugar da 1 usada nos efeitos atuais.
- O `freeze`/hit-stop do PixelFX é só um dado no JSON. Se for usado, deve virar regra em Rust, no módulo de combate.

## Conclusão: o que compensa usar

| Recurso | Vale a pena? | Por quê |
|---|---|---|
| **PixelFX para efeitos** | **Sim, é o maior ganho.** | Gratuito, local e reproduzível. 244 efeitos com timing de golpe, recoloração por rampa e exportação direta para o formato que o jogo já lê. Precisa de redução de cores e de ajuste de escala e pivô, ambos automatizados no conversor. |
| **ASF `gait_loop`, `retime` e `register_clip`** | **Sim, como medição e controle.** | Escolhem o loop por medida, cortam as pausas de golpes e travam os pés. Complementam o pipeline atual sem substituir a arte aprovada. |
| **ASF `pixel_motion frames/finish`** | Útil quando houver vídeo gerado a partir do sprite. | Devolve o vídeo à grade e à paleta do sprite com cerca de 94 % de fidelidade. Hoje não há rota de vídeo. |
| **ASF `pixel_look` e `generate2dsprite`** | **Não agora.** | Dependem de geração paga. A leitura de grade funciona, mas o resultado (48 cores) fica abaixo da redução N3.5 aprovada. |
| **ASF finish pixel/hibit no lugar da N3.6** | **Não.** | Altera as cores da identidade (calça e cabelo) e não aceita a paleta N3.4 no modo pixel. |
| **Pipeline Blender 03B** | Mantido. | Nenhuma das duas ferramentas faz armadura modular encaixada no rig; as duas atuam em outra camada. |

## Arquivos

| Arquivo | Conteúdo |
|---|---|
| `scripts/run_tests.sh` | Reproduz todos os testes em `work/` (não versionado), em cerca de 45 s |
| `scripts/build_evidence.py` | Gera as imagens, GIFs e métricas em `out/` |
| `scripts/pixelfx_to_manifest.py` | Conversor PixelFX → atlas e RON do jogo (proposta) |
| `out/metrics.json` | Números dos testes A e B |
| `out/CREDITS.txt` | Licenças MIT do PixelFX e do ASF |

### Imagens e GIFs para avaliar (`evidence/erro_corretion/etapa03c/out/`)
- `asf_run_comparison.png` e `.gif`: N3.6 contra o ASF pixel e o ASF hibit, mesmo vídeo e mesma escala.
- `asf_gait_loop3x.gif`: o loop escolhido pelo `gait_loop`, em alta resolução, repetido 3×.
- `asf_triage_sheet.png`: triagem do vídeo original.
- `asf_roundtrip.png`: run6 → vídeo → `pixel_motion`, com mapa de diferenças.
- `asf_pixel_look.png`: original, "pintura" degradada e master recuperado.
- `asf_idle_breath.gif`: idle gerado pelo `breath`.
- `pixelfx_in_game_demo.png` e `.gif`: efeito atual contra o corte do PixelFX contra o círculo mágico, sobre o protagonista.
- `pixelfx/sword-slash/anim.gif`, `contact.png`, `sword-slash.png` e `.json`.
- `pixelfx/magic-circle-cast/anim.gif`, `contact.png`, `magic-circle-cast.png` e `.json`.
- `bevy_proposal/effects_pixelfx_*.png` e `manifest_fragment.ron.txt`.
