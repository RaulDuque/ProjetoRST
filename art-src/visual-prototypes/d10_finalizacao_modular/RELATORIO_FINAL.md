# Relatório final D10 (A–T)

## A. Git inicial
Branch `art/v1.2-protagonist`; D9 lab + briefs ainda untracked localmente.

## B. Auditoria D9
`AUDITORIA_D9.md` — capa/luvas falhavam por **pixels compartilhados** (110).

## C. Causas
Máscara de capa incluía zona da luva; RGBA gêmeos.

## D. Correções
Máscaras exclusivas; `glove_front`/`glove_back`; `coat_under_cape`; clear_twins.

## E. Antifake
**Todos pass** (cape, gloves, coat, weapon, boots, scabbard) — `testes/antifake_report.json`.

## F. Fidelidade
`01_D9_vs_D10.png` + `diff_D7B_vs_D10.png` + stats.

## G–I. Corpo / atlas / sockets
Corpo-base reutilizável; atlas D10; sockets herdados D8/D9 calibrados.

## J–K. Clássico / pesado
`07` / `08`; causeway `13`.

## L–N. Idle / run / troca dinâmica
GIFs em `previews/`; fase do run preservada.

## O. After-image
A/B/C em `efeitos/after_image/` + GIFs.

## P. Bevy
`PLANO_INTEGRACAO_BEVY.md` (0.20.1, sem editar src).

## Q. Limitações
`LIMITACOES.md`.

## R. Revisar
`14_resultado_final.png`, `03`/`04`, antifake, `12_after_image`, `DECISAO.md`.

## S. Git final
Untracked: `d10_finalizacao_modular/` (+ d9/d10 briefs). **Sem commit/push.**

## T. D11
Integrar atlas aprovado + VFX after-image B; não expandir magias/inimigos antes.

### Critérios
| Item | Estado |
|------|--------|
| Capa/luvas independentes | **Concluído** |
| Antifake sem resíduos | **Concluído** |
| Idle/run 100% só camadas | Parcial (D2 motion guide) |
| After-image lab | **Concluído** |
| Bevy intocado | **Concluído** |
