# N4-1 (C1) — Auditoria inicial

## Git

- Branch `art/v1.2-protagonist`. HEAD local `4456341` (registro da N4-A).
- **O remoto tinha avançado:** `origin/art/v1.2-protagonist` em `a5f7f1c65dbc2bae2687dbf631febe352c35d2a9`, um commit "Add files via upload" (upload pelo GitHub) que só acrescenta `Md-usario/assets-user/prota/base/Gemini_Generated_Image_n7ajjyn7ajjyn7aj.jpg`. O local não tinha commits próprios além do remoto, então foi feito um avanço **fast-forward** (`git merge --ff-only`), sem merge nem rebase. Nenhum arquivo local conflitava.
- Árvore: limpa, exceto os pessoais não rastreados de `Md-usario/` (preservados).

## A referência e a "N4-B"

- `Gemini_Generated_Image_n7ajjyn7ajjyn7aj.jpg` (2752×1536, SHA-256 `564ebea6326f4121f12284094722816707684b176fb763f7c22d48ae7aae7148`) existe depois do fast-forward. Foi **só lida**.
- Ela é uma imagem gerada por IA no formato das capturas da N4-A, intitulada "N4-B SPRITE MACHINE PROOF (APPROVED REVISIONS)": original revisado, sem cabelo, sem camisa, sem calça, sem botas, sem braceletes e corpo-base. A anatomia é bem mais definida que a da N4-A (peitoral, abdômen, músculos dos braços e das pernas, pés com dedos).
- **Não existe nenhuma N4-B no repositório:** nem código, nem camadas, nem relatório. A "N4-B" é essa imagem. Ela foi usada **só como guia artístico** da anatomia das poses, como o briefing pede, e não como dado do projeto. O idle N4-A não foi alterado, porque o briefing pede ausência de regressão no idle.

## O compositor existente (reutilizado)

`src/player/visual/composition.rs` (contrato RON, `resolve`), `src/player/visual/composite.rs` (`sync_composite_layers`) e `VisualPlayback.frame` / `.flip`. **Nenhum código Rust de produção mudou na N4-1.** Só os dados (`protagonist.ron`), os testes e o exemplo.

## Os seis quadros da corrida

`art-src/n3-6-animation-proof/source/video_{048,050,052,054,056,058}.txt` são as grades de paleta (112×128) dos quadros #0–#5 de `generated/run6.png`. Conferido texel a texel: 0 diferenças nos 6. C1 usa #0 (`video_048`, contato), #2 (`video_052`, passada aberta) e #4 (`video_056`, recuperação). Os quadros do vídeo em alta resolução (`frames/video_0NN.png`, 496×552) foram usados para ler mangas, bracelete, cano das botas e qual braço passa na frente.

## Hashes protegidos (iguais aos da auditoria N4-A)

| Arquivo | SHA-256 |
|---|---|
| `art-src/n3-6-animation-proof/generated/run6.png` | `e15803703a4d1d3c8e6da5d815f3a6a2d759839bbb966b6c8762de8b36864957` |
| `art-src/n3-5-fidelity/generated/candidate_106.png` | `e8e4bfd3837ea58dbc1437ee5ff9f829f80702c8732dfba4044611d6da971984` |
| `assets/animations/player.ron` | `7491f714364997b6278b7821438f3a101f039fcc3423790523116cc0f40c0c3a` |
| `assets/data/camera.ron` | `e822c722d64b76d2536aff98b82f978d05c68aa62bd1dd0982f357aef113637c` |

## Alterações artísticas aprovadas na N4-B

Só as visíveis na imagem de referência: corpo mais musculoso e definido, pés descalços com dedos, roupa de baixo escura. Não há outra fonte.
