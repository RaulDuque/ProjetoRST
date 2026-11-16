# Auditoria objetiva — Fase D2

Fontes: todos os frames/sheets/GIFs em `d2_protagonista_B/animacoes/` e
prévias. Não é julgamento estético do conceito B; é consistência entre quadros.

## Problemas

| # | Problema | Gravidade | Onde | Solução D3 |
|---|---|---|---|---|
| 1 | Identidade muda entre clips (idle vs attack parecem personagens diferentes) | **Crítico** | idle_f00 vs attack_f00/f03 | Modelo em camadas único; mesma anatomia em todos os clips |
| 2 | Comprimento/ângulo da espada instável; às vezes some ou vira efeito | **Crítico** | attack sheet | Lâmina com comprimento fixo (18 px) e modos rest/windup/active/recover |
| 3 | Cabelo/volume da cabeça muda (franja, “capacete”, comprimento) | **Importante** | idle f00 vs f03; run | Cabelo desenhado pelos mesmos pixels + offsets mínimos |
| 4 | Capa muda de formato/cor e ganha glow magenta | **Importante** | attack_f00 | Capa = retângulos de paleta fixa; sem glow procedural |
| 5 | Armadura/ombreira aparece/desaparece | **Importante** | run vs idle | Pauldron sempre desenhado; B2 adiciona painéis fixos |
| 6 | Bainha some em vários frames | **Importante** | attack | Bainha no quadril em todos os modos |
| 7 | Silhueta e postura “resetam” em vez de interpolar pose | **Importante** | run_f00 vs run_f04 | Run = mesmas peças + `leg_phase` |
| 8 | Downsample de JPG gera serrilhado e deriva de cor | **Importante** | todos | Sem downsample de conceito; putpixel na paleta |
| 9 | Pés/pivô flutuam visualmente nas GIFs D2 | **Importante** | previews | Célula 64×80, pivô (32,78) em todos |
| 10 | Matte rosa residual (corrigido parcialmente no fim do D2) | Cosmético/crítico residual | frames antigos | Alpha limpo por construção |
| 11 | Contagem idle 6 / run 8 ≠ contrato 12 / 10 | **Importante** | D2 docs | D3: idle 12 @ ~12 fps, run 10 @ ~24 fps |
| 12 | Oscilação de enquadramento nas prévias geradas | Cosmético | GIFs D2 | Canvas fixo + marca de pivô |

Deformação intencional (capa ao correr, espada no golpe) **não** é erro; o
erro é trocar o personagem.

## Veredito

D2 serve como **direção visual** (B detalhado + poses A), não como base de
produção. D3 reconstrói a identidade em pixel art construída.
