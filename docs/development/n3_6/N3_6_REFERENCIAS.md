# N3.6 — Referências

Evidência: `evidence/references_overview.png`, com todas as folhas reduzidas e cada pose numerada.

Todas as imagens e o vídeo foram abertos e vistos. As poses foram separadas por `scripts/segment_sheets.py` (componentes conexos sobre o fundo) e numeradas em ordem de leitura. As caixas estão em `art-src/n3-6-animation-proof/analysis/poses.json`, e os documentos citam as poses como `<apelido>#<número>`.

| Apelido | Arquivo | Tamanho e formato | Poses | Direção | Conteúdo |
|---|---|---|---|---|---|
| `run_a` | `Gemini_Generated_Image_15q6g515q6g515q6.jpg` | 2752×1536, JPEG RGB | 8 (2×4) | direita | corrida lateral |
| `run_jump` | `Gemini_Generated_Image_gm471hgm471hgm47.jpg` | 2752×1536, JPEG | 18 poses + 1 efeito (#14, um respingo) | direita | corrida (1–8), salto e acrobacia (9–17), aterrissagem (18) e de pé (19) |
| `attack` | `Gemini_Generated_Image_p5njkap5njkap5nj.jpg` | 2752×1536, JPEG | 12 (2×6) | direita | guarda, soco direto, impacto e recuperação, em duas tomadas |
| `idle_run_jump` | `Gemini_Generated_Image_xwfavvxwfavvxwfa.jpg` | 2752×1536, JPEG | 19 (6 + 8 + 5) | frente 3/4 e direita | idle de frente (1–6), corrida em 3/4 (7–14), salto (15–19) |
| `fight_left` | `al.jpg` | 2400×1792, JPEG | 31 | **esquerda**, quase todas | golpes, guarda, avanço, rasteira, salto vertical |
| — | `side.png` | 78×280, PNG RGBA (fundo transparente) | 1 | direita | de pé, de perfil |
| — | `trimmed_fd4c76cf-….mp4` | 496×864, H.264, 24 fps, 7,917 s, 189 quadros decodificados | — | direita | fundo verde; parado nos quadros 0–2, transição em 3–5, depois **corrida no lugar com câmera fixa** |

## Observações por arquivo

- **Fundo:** as quatro folhas Gemini têm fundo branco com ruído de JPEG. `al.jpg` tem fundo verde-acinzentado e **marcadores verdes de pivô** (⊥ e cruzes), que funcionam como interface de edição e não fazem parte da arte. O vídeo tem fundo verde de chroma key. `side.png` já vem recortada.
- **Bordas:** as folhas Gemini imitam pixel art (blocos de cerca de 5 px da folha), mas com artefato de JPEG nas bordas. No vídeo, a figura é pintada e suave, e há borrão de interpolação em alguns quadros.
- **Consistência:** é o mesmo personagem em todas, mas as cores variam. `idle_run_jump` tem a camisa esverdeada, a gola com cordão e as botas marrons, além de uma fita escura solta na corrida (#9, #10, #13 e #14). As folhas Gemini têm a camisa quase preta. No vídeo a camisa é cinza, como na ficha.
- **Extração de quadros:** possível em todas. Nas folhas, cada pose é uma imagem gerada à parte, sem ordem temporal garantida. Só o vídeo tem continuidade real entre quadros.
- **Limitações para sprite:** as folhas misturam escalas (a altura de pé varia de folha para folha), e `fight_left` está espelhada e tem marcadores. No vídeo, os quadros 49 e 55 do ciclo usado têm pernas duplicadas, resto de interpolação.

## Integridade (SHA-256)

```
01adf3a99172c9f3a779802cbdaad48320e656f9e925495ac7e0eb08bd891b2e  al.jpg
bf463d7023dec330b224f70995a5d4442346b7f93eda536624ed7681c272549f  Gemini_Generated_Image_15q6g515q6g515q6.jpg
df06f79ac177a3fd31139a20d5d20741efb3b7e6209ec4b9ad7e3d89503db7fc  Gemini_Generated_Image_gm471hgm471hgm47.jpg
cfbb9e9784eb40e7804369655a919a278b4ceb4cbb801e27640d7341d5c1ef6b  Gemini_Generated_Image_p5njkap5njkap5nj.jpg
f14be281a35f8b9b2af3f765716eb760a18554ab0394f5e5ace7d323f64ca089  Gemini_Generated_Image_xwfavvxwfavvxwfa.jpg
431ec45b69ef6a890e246cbde3d28c22d939ea177e705c0e3878e7a29f3b5053  side.png
29cdbcb8594f94f38b7668919d18482af241a0f5fc3f449b7dfc96bec702cc57  trimmed_fd4c76cf-0416-4387-b451-da4cc90a2061.mp4
15ac4ec2028fba9c50f337cbde840ddb5fadec6291a55b4e514de369f1e8d726  image.png
9247670387d245bdd2aee873b364726a3cb55c34bf334ffe9201baa55d501447  Ficha_Modular_do_Protagonista_Gotico.png
076158a98e3225eb06bff2acefda67e609571635f11910ebe14bf7482f939bf7  white_mesh.glb
```

Os originais são referenciados no lugar onde estão. Nenhum foi copiado, convertido por cima, renomeado ou apagado.
