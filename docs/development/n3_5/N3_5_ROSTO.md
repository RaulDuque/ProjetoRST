# N3.5 — Rosto

Evidências: `face_comparison.png` (em close e na escala real das duas câmeras) e `palette_test.png`.

## Comparação

| Ponto | Ficha (perfil) | `reducao` | N3.4 | N3.5 |
|---|---|---|---|---|
| Testa | inclinada, com a franja por cima | sim | reta, vertical | sim, como na `reducao` |
| Sobrancelha | escura, reta | 2 px escuros | 2 px `e` | 2 px escuros da redução (44–45, 20) |
| Olho | recuado, semicerrado | junto da sobrancelha | 1 px `e` isolado | junto da sobrancelha, como na ficha |
| Nariz | reto, saliente | cinza (halo) | 1 px à frente | pele (46, 21–22), corrigido à mão |
| Boca | linha curta | 1 px escuro | `z` recuado | 1 px `z` da redução (45, 24) |
| Queixo e mandíbula | firmes | sim | quadrados | sim, maxilar em diagonal (40–44, 26–27) |
| Pescoço | sombra sob o maxilar | sim | sim | sim |
| Cabelo e pele | mecha sobre a orelha | sim | faixa vertical dupla | mecha da redução |
| Expressão | séria | séria | neutra e rígida | séria |

## Edições à mão no rosto (todas em `metrics.json`)

- (46, 18), (46, 19) e (46, 20): removidos. Eram cinza do halo na borda da testa.
- (46, 21) → `z` e (46, 22) → `s`: o nariz, que a redução deixara cinza.
- (46, 23) → `z`: sombra sob o nariz, onde a redução pôs cor de couro.
- (47, 17): removido. Era um pixel cinza isolado na franja.

## Na escala real

- **Câmera 360:** a captura do Bevy é idêntica ao sprite. Sobrancelha, nariz e queixo aparecem.
- **Câmera 405:** 12 linhas somem na figura toda. No rosto, o olho e a boca afinam, e o perfil continua lendo como perfil.
- **1×:** o rosto é um perfil de 6 px. A identidade, nessa escala, vem do cabelo e da silhueta, não da expressão.

## Paleta

Com a paleta adaptativa de 32 cores (mediancut), a pele ficou cinza-rosada e o rosto se misturou ao cabelo. Com 24 cores + as rampas de pele e cabelo da N3.4, a pele voltou, mas o cabelo ficou bege. A paleta da N3.4, pele quente e cabelo frio, é a da `reducao` escolhida pelo proprietário e foi mantida (`palette_test.png`).
