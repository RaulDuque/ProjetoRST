# N4-1-A — Avaliação de formatos

| Formato | Uso escolhido | Por quê | Dependência nova? |
|---|---|---|---|
| **RON** | manifestos do jogo: composição, família e item | já é o formato de todo dado do projeto (serde + `ron`); aceita comentários e enums (`Region`, `MissingClip`) com tipagem forte | não |
| **JSON** | receitas de autoria (`equipment/families/*.json`, `equipment/recipes/*.json`) e relatórios (`coverage.json`, `metrics.json`) | intercâmbio com ferramentas: o gerador Python lê com a biblioteca padrão, e um editor externo (web ou outro) pode produzir as receitas | não (o jogo não lê JSON) |
| **PNG** | folhas finais dos itens, máscaras das famílias, evidências | é o que o Bevy carrega; sem perda; o mesmo canvas e pivô dos quadros aprovados | não |
| **SVG** | **não usado** | as máscaras saem das camadas vestidas, texel a texel; um SVG rasterizado introduziria borda suavizada (anti-aliasing) em pixel art. Fica como opção de autoria de silhuetas grandes, rasterizada offline com *nearest* antes de virar PNG | — |
| **Base64** | **não usado** | não há transporte que exija; os PNG são arquivos do projeto | — |

Os `.txt` ao lado de cada PNG (grades de caracteres) existem para os testes Rust compararem texel a texel sem um decodificador de PNG.

Não há HTML nem JavaScript no jogo. Um editor web futuro precisaria só gerar o JSON da receita (ou o RON do item) e as folhas PNG no mesmo canvas. O compositor Rust não muda.
