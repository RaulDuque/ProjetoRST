# D20.1 — Defeitos

Nenhum **P0** ou **P1** reproduzido que bloqueie o jogo ou os fluxos D17–D20.

| ID | Sistema | Local | Descrição | Repro | Esperado | Observado | Evidência | Sev. | Status |
|---|---|---|---|---|---|---|---|---|---|
| D201-01 | UI inventário | `InventoryCursor` | Cursor persiste entre aberturas; reabrir + N Downs pode selecionar ether em vez do tonic | Abrir I, ir ao tonic, fechar, abrir, 8×Down, Enter | Cursor no topo ou no último consumível útil | Cursor residual; Enter no ether (NoEffect) se MP cheio | probe_run2.log | P3 | Documentado — workaround na rota (Up×16); correção de produto opcional |
| D201-02 | Combate plataforma | Watch Shaft ledge | Lutar com Bone Bolt na ledge derruba o jogador ao chão e quebra a sequência de climb | Fight no bolt em y≈409 | Combate estável na plataforma | Queda para y=63 | probe_run3.log | P3 | Aceito p/ D20.1 (probe só observa o bolt); relevante p/ D21 arenas |
| D201-03 | Arte placeholder | Inimigos | Bolt/Warden/Sentry são blocos coloridos; contraste baixo em screenshots | Qualquer captura D20 | Identidade visual clara | Blocos cyan/laranja/bege | `12_bone_bolt.png`, `13_ash_warden.png` | P3 | Arte definitiva = FUTURO |

## Não-defeitos confirmados

- Viagem rápida **não** altera `last_save_point` (continua `vg_causeway_save` após ir ao Approach).  
- Saves reais intocados.  
- Autoplay e probe encerram com PASS controlado.
