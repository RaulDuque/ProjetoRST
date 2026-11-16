# Relatório final D7 (A–T)

## A. Estado inicial do Git

Branch `art/v1.2-protagonist` (tracking origin). Trabalho novo só sob
`art-src/visual-prototypes/d7_pixel_programatico/`. Sem commit/push nesta fase.
`Md-usario/DarkCastleR-d-7.md` permanece untracked (não alterado pelo lab).

## B. Auditoria D2/D5/D6

D2 = identidade aprovada. D5/D6 = PoC modular técnico com finish toon/EEVEE
ainda “personagem antigo”. Ver `AUDITORIA_PIPELINE.md`.

## C. Causa principal da aparência antiga

Look final determinado por materiais/luzes Blender, não por desenho 2D D2.

## D. Estratégias

Ver `ESTUDO_ESTRATEGIAS.md` (A primitivas / B máscaras / C híbrido).

## E. D7-A

`scripts/draw_A_primitivas.py` → `estrategias/A_primitivas/`. Silhueta gótica
legível, acabamento geométrico.

## F. D7-B

`scripts/draw_B_sprites_masks.py` → máscaras de `idle_f00` + stack D4.
Fontes documentadas. Melhor fidelidade.

## G. D7-C

`scripts/draw_C_hibrido.py` → máscara D6 + paleta + cabeça D2 + sabre por
socket. Controle negativo: quantize do toon.

## H. Comparação visual

`comparacoes/02_D7_tres_estrategias.png`, `03_…escala_real.png`, `04_…`.

## I. Fidelidade

Tabela em `COMPARACAO_VISUAL.md`. B aprovável; A prova técnica; C incompleto.

## J. Marco A

Três candidatos + boards + avaliação. Seleção: **B**.

## K. Camadas

`personagem/layers_manifest.json`, `equipamentos/*.png`, `ARQUITETURA_CAMADAS.md`.

## L. Loadouts / troca

`loadouts/leve`, `loadouts/pesado`, `comparacoes/06_*.png`, `07_*.png`.
Proxies para luvas/botas (D4 sem peças separadas).

## M. Animações piloto

Idle 6 + run 8 (contagem real D2). GIFs em `previews/`. Extras pesados estáticos
sobre frames D2 (sem deformação de capa ainda).

## N. Causeway

`comparacoes/08_D7_causeway.png`.

## O. Testes

`python3 …/scripts/test_d7.py` — todos OK (técnicos, não artísticos).

## P. Problemas artísticos

- A geométrico.
- C bandas planas.
- Franjas magenta residuais em algumas peças D4.
- Loadout leve: casaco D4 + corpo D2 pode ocluir/estranhar cabeça/hood.
- Animação pesada: armadura não acompanha deformação do passo.

## Q. Limitações

`LIMITACOES.md`.

## R. Arquivos prioritários para avaliação

1. `comparacoes/02_D7_tres_estrategias.png`
2. `comparacoes/03_D7_identidade_escala_real.png`
3. `comparacoes/10_D7_resultado_final.png`
4. `comparacoes/06_D7_loadouts.png`
5. `previews/run_leve.gif` / `run_pesado.gif`
6. `DECISAO.md`

## S. Estado final do Git

Alterações locais untracked em `d7_pixel_programatico/` (+ briefing d-7 em
Md-usario). **Sem commit. Sem push.** Jogo (`src/`, `assets/`) intocado.

## T. Recomendação D8

Atlas 2D D2 limpo + sockets por peça + deformação de capa; manter Blender só
como pose. Ver `DECISAO.md`.
