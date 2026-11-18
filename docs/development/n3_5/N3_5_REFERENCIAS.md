# N3.5 — Referências

Evidência: `evidence/reference_sources.png`.

| Referência | Arquivo | Uso nesta etapa |
|---|---|---|
| Identidade (prioridade 1) | `Md-usario/assets-user/prota/base/image.png` | conferência de cores, roupa e proporção; vista frontal, sem pixels copiados |
| `ficha lateral` | recorte x 680–775, y 80–418 da `Ficha_Modular` | **fonte dos pixels** do candidato e das três hipóteses |
| `ficha (perfil)` | recorte x 700–780, y 82–158 da `Ficha_Modular` | alvo do rosto e do cabelo |
| `reducao` | `art-src/n3-4-original-protagonist/source/base_idle_draft.txt` | alvo de acabamento; é o próprio arquivo, não uma reconstrução |
| `Ficha_de_Modelagem` (não rastreada) | `Md-usario/.../Ficha_de_Modelagem_do_Protagonista_Gotico.png` | só lida; diverge da prioridade 1 na cicatriz do rosto |

Todos os recortes são feitos pelos scripts a cada execução, sempre com as mesmas coordenadas, sem copiar os originais para fora de `Md-usario/`. A única cópia é `original_reference.png` da N3.4, que já estava publicada.

## O que a `reducao` é, tecnicamente

Recorte da vista lateral, máscara por preenchimento do fundo (tolerância 22), redução BOX para 106 px e cada pixel mapeado para a cor mais próxima da paleta de 19 cores da N3.4, sem metal (`M`) e sem olho (`e`). Ela tinha um halo claro nas bordas, porque a máscara incluía a orla suavizada da ilustração, onde a figura se mistura ao fundo.
