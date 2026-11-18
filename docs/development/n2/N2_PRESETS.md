# N2 — Presets

Os dois presets estão em `src/input/presets.rs`. Os dois passam em `validate`: toda ação existe, ação essencial tem controle, e o mesmo controle não serve a duas ações do mesmo contexto.

## Híbrido

É o perfil quando não existe arquivo.

| Ação | Principal | Alternativo |
|---|---|---|
| Andar à esquerda / direita | A / D | — |
| Pular | W | Espaço |
| Mira para cima | seta para cima | — |
| Mira para baixo / agachar | S | — |
| Correr | Shift esquerdo | — |
| Ataque | mouse esquerdo | X |
| Magia | C | — |
| Guarda | Q | — |
| Esquiva | F | — |
| Interagir | E | — |
| Forma, mapa, status, inventário, levantar, F5, F9, Enter | iguais ao clássico | — |

Q e F são provisórios. A era guarda e passou a ser esquerda. S era esquiva e passou a ser agachar. A seta para cima ficou como "cima" do comando porque W é pulo e não pode ser as duas coisas. Tudo isso é remapeável.

## Clássico

Seta esquerda, seta direita, seta cima, seta baixo, Shift, Z, X, C, A, S, D, T, I, M, Tab, R, F5, F9, Enter. Um controle por ação. É o que `controles.md` descrevia antes da N2.

Carregar um preset copia esses valores para o rascunho da tela. A função do preset não é editada. Se o rascunho ficar igual a um preset, a origem volta a ser Híbrido ou Clássico. Qualquer diferença vira Custom.
