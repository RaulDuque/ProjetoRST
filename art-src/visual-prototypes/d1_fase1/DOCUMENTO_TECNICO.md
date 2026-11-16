# Fase 1 — Documento técnico (protagonista pixel A/B)

**Status:** protótipos técnicos para decisão. **Não aprovado. Não publicado.**
**HEAD de referência:** `1bf2ed2` (B2 local, sem push).
**Pasta:** `art-src/visual-prototypes/d1_fase1/` (não altera outros conteúdos de `art-src/`).

## 1. Referências usadas

| Arquivo | Uso |
|---|---|
| `07_comparacoes/COMPARE_characters.png` | Contexto do laboratório |
| `01_personagem/CHAR-A_pixel_32.jpg` | Direção visual A |
| `01_personagem/CHAR-B_pixel_64.jpg` | Direção visual B |
| `03_armas_armaduras/*` | Equipamentos |
| `05_transformacoes/FORM-spectral_sequence.jpg` | Forma espectral (atual rejeitada como final) |
| `05_transformacoes/FORM-future_concepts.jpg` | Só referência futura (golem/corvo **não** implementados) |
| `06_inimigos/ENEMY-archetypes_sheet.jpg` | Direção de inimigos |
| `target/world_visual_showcase/after_causeway.png` | Escala real / luz V2.3 |
| `assets/sprites/player/player_locomotion.png` (célula idle) | Baseline CURRENT (somente leitura) |

## 2. Identidade comum (A e B)

Mesmo protagonista aristocrático sombrio:

- cabelo curto ardósia com franja à frente;
- casaco escuro com forro vinho;
- meia-capa / capa assimétrica;
- acentos âmbar;
- espada longa fina (B inclui bainha);
- perfil direito, silhueta vertical.

Paleta alinhada a `docs/ART_DIRECTION.md` / `art-src/player/scripts/post/palette.py`.

## 3. Versão A — compacta

| Item | Caminho |
|---|---|
| Conceito isolado | `A_compacto/d1_A_idle_chroma.jpg` |
| Idle técnico | `A_compacto/A_idle_tech.png` (+ `*_nn.png` ampliado NN) |
| Idle ~52 u | `A_compacto/A_idle_52u.png` |
| Poses-chave | `poses/A/` (`idle|run|jump|fall|attack|hurt` concept + tech) |
| Prancha | `poses/A_keyframes_board.png` |

**Meta:** silhueta legível, casaco/capa/espada, contraste com fundo escuro, poucos pixels.

## 4. Versão B — detalhada

| Item | Caminho |
|---|---|
| Conceito isolado | `B_detalhado/d1_B_idle_chroma.jpg` |
| Idle técnico | `B_detalhado/B_idle_tech.png` |
| Idle ~52 u | `B_detalhado/B_idle_52u.png` |
| Poses-chave | `poses/B/` |
| Prancha | `poses/B_keyframes_board.png` |

**Meta:** rosto/cabelo, ombreira, capa em camadas, espada+bainha, ar aristocrático.

## 5. Escala real (câmera V2.3)

- `view_height` 405; janela 720 px de altura → **≈1,778 px/u**.
- Altura visual de referência **52 u** → **≈92 px** na tela.
- Hurtbox atual **26×46 u** desenhada em vermelho nas comparações.
- Zoom **não** foi alterado.

Arquivos:

- `comparacoes/gameplay_scale_causeway.png` — CURRENT × A × B na causeway real;
- `comparacoes/COMPARE_AB_current_3x_floor.png` — mesmos 52 u a 3× nearest-neighbor + hitbox;
- `comparacoes/COMPARE_FINAL_d1.png` — prancha final.

## 6. Equipamentos (somente protótipo)

`equipamentos/d1_equip_variants.jpg`: leve, pesada, escudo, capa alternativa.
`equipamentos/sockets_layers_diagram.png`: mapa conceitual das âncoras V2.2
(`weapon`, `hand_l`, `head`, `chest`, `back`).

**Compatibilidade:** a arquitetura atual (`player_sockets.ron` + `held_visual` +
capa procedural) continua válida se A/B forem publicados no mesmo contrato de
pivô/densidade. **Nenhuma âncora nova foi medida** nesta fase — overlays são
conceituais.

## 7. Transformação espectral

Proposta em `espectral/d1_spectral_sequence.jpg`:

humano → dissolução com partículas → silhueta espectral distinta (material
translucente / traços, **não** filtro azul) → movimento com rastro → retorno.

FORM-golem e FORM-raven: **não** produzidos como protótipos de integração.

## 8. Inimigos (estudos)

Dois arquétipos, sem IA e sem substituir Bone Sentry:

1. Corpo a corpo (sentinela óssea);
2. Mágico / voador (espectro velado).

`inimigos/d1_enemies_two.jpg` + `inimigos/enemies_vs_protagonist_scale.png`.

## 9. O que isto NÃO é

- **Não** é spritesheet publicável nem atlas de `assets/`.
- **Não** há clips com pivôs validados nem contagem de frames de produção.
- As “animações” são **poses-chave** derivadas de folhas de conceito,
  quantizadas por nearest-neighbor. Declaração obrigatória do briefing:
  ainda não são spritesheets completos.
- Geração de conceito + downsample ≠ pixel art final limpa quadro a quadro.

## 10. Plano de produção (após escolha A ou B)

1. Folha modelo lateral + frontal na resolução escolhida (célula alvo alinhada
   ao contrato: locomoción 64×64 u / combate 128×96 u ou variante alta).
2. Pixelar idle + run (prioridade) com rim obrigatório.
3. Medir sockets no Blender **ou** marcar âncoras à mão por quadro.
4. Ataque / hurt / jump / fall / land / dodge.
5. Camadas de capa e arma (ou manter procedurais V2.2 no início).
6. Forma espectral como atlas/VFX dedicado (não só tint).
7. `publish_guard` → `assets/` somente com autorização explícita.

## 11. Estimativa relativa de esforço para integrar

| Versão | Arte (idle+locomoção mínima) | Sockets/equip | Espectral distinta | Risco |
|---|---|---|---|---|
| **A compacta** | Menor (menos pixels/clip) | Médio | Médio | Silhueta ok; pouco espaço para armadura |
| **B detalhada** | Maior (mais limpeza/clip) | Médio–alto | Médio–alto | Melhor leitura de equipamento; custo de animação |
| Manter CURRENT + polish | Baixo–médio | Já existe | Médio (hoje é tint) | Menos ruptura; direção A/B rejeitaria o look atual |

## 12. Recomendação técnica (não estética)

Para a câmera atual (~92 px de altura), **B detalhada** só vale se houver
orçamento para limpeza manual; caso contrário **A compacta** (ou um híbrido
“silhueta A + poucos acentos B”) reduz risco de borrão. Em ambos os casos,
**não** publicar conceitos gerados sem passagem de pixel art humana.

A escolha estética é do proprietário → `DECISAO.md`.
