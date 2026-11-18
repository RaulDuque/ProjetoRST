# N3.7 — Auditoria inicial

## Git

- Branch `art/v1.2-protagonist`, HEAD local `d4e5e046c3ae1e10c1cb5fd9f3f2945867e343c4`, igual ao remoto depois de `git fetch` (o HEAD de referência do briefing).
- Árvore limpa, exceto arquivos não rastreados do proprietário em `Md-usario/` (briefings N3.4 a N5 e a `Ficha_de_Modelagem_do_Protagonista_Gotico.png`). Nada disso foi lido para escrita, movido ou commitado.

## O que foi lido antes de mudar qualquer arquivo

- `CLAUDE.md` e `.claude/rules/` (compilação segura, commits atômicos, testes).
- Documentos N3.5 e N3.6 pedidos: relatórios finais, avaliações, câmera, análise da corrida, alinhamento, fundo escuro e Bevy.
- Scripts da N3.6: `extract_video.py` (GStreamer + chroma key) e `build_run.py` (redução, offset global, manifesto, `planted_speed`).
- `examples/n3_visual_proof.rs` (fontes 0 a 7, `View`, `RunBank`, sobreposição por sprites, roteiro de capturas).
- Contrato de animação: `src/player/visual/manifest.rs` (`AnimationManifest`, `ClipDef` com `speed_reference`) e `src/player/visual/playback.rs` (`locomotion_rate`, limitado a 0,5–2,5×).
- `src/core/constants.rs`: `PLAYER_RUN_SPEED = 310`, `PLAYER_WALK_SPEED = 190`, `GRAVITY = -1900`, `PLAYER_JUMP_SPEED = 720`.
- `assets/data/camera.ron` (`view_height: 405.0`, `vertical_offset: 40`, `look_ahead: 70`) e as dimensões das salas em `assets/world/regions/`.

## Achados que mudam a análise

1. **A estimativa de 78 u/s da N3.6 estava baixa.** `planted_speed` só aceitava deslocamentos de até 8 texels por quadro na linha do pivô, e descartava justamente os maiores. A medição nova (`N3_7_DESLIZAMENTO.md`) dá cerca de **157 u/s**.
2. **O vídeo tem uma partida real.** Os quadros 0–2 estão parados, os quadros 3–~10 mostram o personagem saindo para a corrida e, a partir do 13, ele já está no ciclo. A N3.6 não usou esse trecho. Ele resolve parte da transição sem inventar quadros (`N3_7_TRANSICAO.md`).
3. **O contrato atual já tem a ferramenta de sincronização.** `speed_reference` + `locomotion_rate` aceleram um clipe de locomoção proporcionalmente à velocidade. O cenário B é exatamente esse mecanismo (`N3_7_CONTRATO_ANIMACAO.md`).
