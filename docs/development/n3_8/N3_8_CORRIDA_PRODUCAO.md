# N3.8 — Corrida no jogo principal (modo experimental)

**Status: integração experimental com fallback seguro. A promoção ao `cargo run` padrão está PENDENTE (decisão do proprietário). A substituição do protagonista NÃO está concluída.**

## Como jogar

```bash
cargo run                                  # protagonista de produção V2.2 (padrão, inalterado)
DARK_CASTLE_PROTAGONIST=n3 cargo run       # novo protagonista experimental (N3.5 + N3.6)
DARK_CASTLE_PROTAGONIST=production cargo run   # igual ao padrão
```

Um valor desconhecido (por exemplo `n4`) é avisado no terminal e cai no protagonista de produção. Se a pasta de staging não existir, o jogo também volta à produção. Nunca carrega meio protagonista.

No modo experimental, um selo no canto inferior direito diz `EXPERIMENTAL PROTAGONIST (N3.8): idle N3.5 + run N3.6 15 fps`. Quando o estado não tem clipe aprovado, aparece uma segunda linha, por exemplo `PLACEHOLDER: N3.5 pose for "jump_rise" (no approved clip)`.

## O que foi implementado (existente agora)

| Peça | Arquivo | Tipo |
|---|---|---|
| Escolha da arte (`ProtagonistArt`, `PROTAGONIST_ENV`) | `src/player/visual/art_profile.rs` | regra pura, testada |
| Montagem do app com a arte escolhida (`build_for_protagonist`, `protagonist_from_env`) | `src/app/mod.rs` | app |
| Marcadores `ExperimentalProtagonist` e `NearestPlayerArt` | `src/player/visual/mod.rs` | recursos Bevy |
| `VisualPlayback::placeholder_for` (qual clipe faltou) | `src/player/visual/{mod,systems}.rs` | apresentação |
| Amostragem *nearest* só quando `NearestPlayerArt` existe | `src/player/visual/systems.rs` | apresentação |
| Selo experimental | `src/ui/art_badge.rs` (+ registro em `src/ui/mod.rs`) | UI |
| Staging do novo protagonista | `art-src/n3-8-production-run/stage/` | dados de arte |

## Reutilização (sem sistema paralelo)

O modo experimental usa **o mesmo** carregador (`load_player_sprites`), o mesmo formato de manifesto (`AnimationManifest`), a mesma seleção de clipe (`selection::select`), o mesmo relógio (`VisualPlayback.elapsed` + `playback::loop_frame`), o mesmo espelhamento (`anchor_for`) e o mesmo caminho de staging que a V2.2 criou para candidatas (`PlayerArtSource` + `app::build_with_candidate_art`). A única diferença é a pasta de onde a arte é lida.

## Staging

`art-src/n3-8-production-run/stage/` tem o mesmo layout de `assets/`:

- `sprites/player/n35_idle.png`: cópia **byte a byte** de `art-src/n3-5-fidelity/generated/candidate_106.png` (80×128).
- `sprites/player/n36_run6.png`: cópia **byte a byte** de `art-src/n3-6-animation-proof/generated/run6.png` (6×112×128, vídeo 48, 50, 52, 54, 56, 58).
- `animations/player.ron`: dois clipes, `idle` (1 quadro, pivô 40,118) e `run` (6 quadros de 1/15 s, pivô 56,118, `speed_reference: None`).
- Sem `player_sockets.ron` (de propósito; ver `N3_8_COMPATIBILIDADE.md`).

Um teste compara os bytes das cópias com os originais. Os originais não mudaram, e o teste de hash da N3.7 (`tests/n3_7_sync_tests.rs`) continua passando.

## O que não mudou

- `assets/` inteiro: `player.ron`, `player_sockets.ron`, atlas e a trava `player.publish.json`. O teste Python do guard de publicação passa (21/21).
- `assets/data/camera.ron` (405).
- Física, `PLAYER_RUN_SPEED` (310), entrada, combate, magia, colisões, progressão e save v3.
- Animações V2.2 e caminho padrão de `cargo run` (`build_for_protagonist(.., Production)` chama o mesmo `build_with_world` de antes).

## Por que não foi direto para `assets/`

1. O proprietário não autorizou misturar identidades no personagem padrão.
2. A tabela de soquetes seria invalidada, e a arma e o escudo sumiriam em todos os estados da produção.
3. `player.ron` só pode ser escrito por `publish_guard.py`, e o contrato de célula e pivô do teste de dados é o da V2.2.

A promoção futura exige: clipes aprovados para os outros estados (`N3_8_PLANO_MODULAR_N4.md`), soquetes medidos e publicação pelo guard.
