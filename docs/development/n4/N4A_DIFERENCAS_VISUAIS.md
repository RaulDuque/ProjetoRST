# N4-A — Diferenças visuais: original × modular

## Medições

| Comparação | Unidade | Total | Diferentes |
|---|---|---|---|
| Grade de paleta recomposta (Rust, ordem do RON) × `candidate_106.txt` | texels | 10 240 (80×128), 1320 opacos | **0** |
| PNG recomposto (Python, `build_idle_layers.py`) × `candidate_106.png` | texels RGBA | 10 240 | **0** |
| Bevy real, fundo escuro: tela do original × tela do modular | pixels | 163 840 (21 120 da figura) | **0** |
| Bevy real, fundo claro | pixels | 163 840 | **0** |
| Bevy real, espelhado | pixels | 163 840 | **0** |

As 21 120 pixels de figura são exatamente 1320 texels × 16 (4×4 px por texel): escala e pivô corretos. Fontes: `evidence/metrics.json` (`recomposition_differences`, `bevy_pixel_check`) e o teste `the_layers_recompose_the_approved_idle_texel_for_texel`.

## Mapa de diferenças

`evidence/n4a_diff_map.png` (6×): verde-escuro = texel opaco igual; vermelho = diferente. **Não há vermelho.** `evidence/n4a_recomposition.png` mostra original, modular e mapa lado a lado.

## Por que a igualdade é exata (e o que ela não prova)

É exata **por construção**: os texels visíveis das camadas são uma partição do original, e cada texel pintado fica sob uma peça que o cobre no original. Por isso a igualdade prova que a máquina de sprites **compõe** certo (ordem, canvas, pivô, espelho e sincronia), inclusive no renderizador do Bevy. Ela **não** prova que a anatomia reconstruída é boa: essa só aparece quando as peças são removidas, e é avaliada visualmente (`N4A_RECONSTRUCAO_ARTISTICA.md` §3). Nenhuma recomposição foi aprovada "por parecer semelhante": a comparação é texel a texel, e a qualidade da anatomia continua pendente do proprietário.
