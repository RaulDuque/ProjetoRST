# Fase 2 — Protagonista B detalhado + legibilidade A

**Status:** protótipo técnico piloto. **Não aprovado. Não publicado.**
**HEAD:** `1bf2ed2`. **Pasta:** `art-src/visual-prototypes/d2_protagonista_B/`
(não sobrescreve `d1_fase1/`; não toca `assets/` nem `src/`).

## Decisão do proprietário

- Direção visual principal: **B detalhado**.
- Movimento / leitura de silhueta: inspirado nas poses **A** (idle, run, jump, fall).
- Híbrido: identidade B + clareza de movimento A.

## Identidade travada

| Elemento | Definição D2 |
|---|---|
| Arquétipo | Aristocrata gótico de fantasia sombria |
| Cabelo | Curto, escuro / ardósia |
| Traje | Casaco longo + armadura de ombro |
| Capa | Vermelho escuro / vinho (forro visível) |
| Arma | Espada curva prata + bainha |
| Vista prioritária | Lateral (jogo 2D) |
| Escala alvo | ~52 u de altura visual; hurtbox 26×46 u intacta |

Referência-mestre: `referencia/d2_master_turnaround.jpg`
Extração lateral técnica: `referencia/side_master_tech.png`
Paleta amostrada: `paleta/palette.md` + `paleta/palette_strip.png`

## Timing de combate (obrigatório — não alterado)

Espada básica atual: `attack_time = 0.28 s`, `HITBOX_ACTIVE_FRACTION = 0.6`
→ janela ativa ≈ **0,168 s**. Clip publicado `attack_basic`: **7 frames**
`FitToAction`.

O piloto D2 de ataque usa **7 quadros** com fases:

| Quadros | Fase visual | Papel |
|---|---|---|
| 0–1 | Antecipação | Espada atrás |
| 2–4 | Ativa | Corte à frente (~60%) |
| 5–6 | Recuperação | Retorno |

Tempos de gameplay **não** foram mudados para caber a arte.

## Animações piloto

| Clip | Quadros | Prévia | Sheet técnico |
|---|---|---|---|
| Idle | 6 | `previews/idle.gif` | `animacoes/idle/frames/idle_sheet.png` |
| Corrida | 8 | `previews/run.gif` | `animacoes/run/frames/run_sheet.png` |
| Ataque | 7 | `previews/attack.gif` | `animacoes/attack/frames/attack_sheet.png` + `attack_phases_board.png` |

Processamento: chroma (sem magenta residual nas PNG finais), pivô
aproximado nos pés (centro-inferior), nearest-neighbor para down/upscale,
GIFs em fundo escuro com marca de pivô.

**Declaração:** estes clips são **pilotos de direção**, não atlases
publicáveis. Há deriva de identidade entre quadros típica de conceitos
gerados; falta pixel art humana quadro a quadro e medição real de sockets.

## Equipamentos (estudo)

Direção aprovada: `d1_equip_variants.jpg` → estudo D2
`equipamentos/d2_equip_layers.jpg` + plano `equipamentos/layers_plan.png`.

Sistema reutilizável alvo (arquitetura V2.2 intacta):

1. Body B (atlas)
2. `CapeLayer` / socket `back` (capa padrão ou alternativa)
3. `HeldLayer::Weapon` / socket `weapon`
4. Escudo = `HeldLayer::Shield` / offhand
5. Armadura leve/pesada = tint ou overlay de torso (sem reanimar tudo)

Nenhuma animação completa por equipamento nesta fase.

## Forma espectral (estudo)

Referência aprovada: `d1_spectral_sequence.jpg` →
`espectral/d2_spectral_study.png`.

Meta: silhueta B reconhecível com material translúcido / partículas /
olhos ocos — **não** filtro azul. Sem implementação no jogo.

## Comparações V2.3

- `comparacoes/gameplay_scale_causeway.png` — CURRENT idle × D2 idle/run/attack
  na causeway real, hitbox 26×46.
- `comparacoes/compare_3x_nn.png` — mesma altura ~52 u a 3× NN.
- `comparacoes/idle_frames_review.png` — revisão de identidade no idle.

Zoom da câmera **não** foi alterado.

## Integração (viabilidade)

Possível no pipeline atual se, após escolha final:

1. Pixelar clips com célula/pivô do contrato (`player.ron` / ASSET_CONTRACT);
2. Remedir `player_sockets.ron` por quadro;
3. Staging + `publish_guard` (único escritor de `assets/`).

Estimativa: ver `ESTIMATIVA.md`. Limitações: ver `LIMITACOES.md`.
