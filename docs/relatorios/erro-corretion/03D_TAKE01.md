# Erro-Corretion 03D — Take 01: armadura do ChatGPT lida pelo Agent Sprite Forge

**Status:** CONCLUÍDO COM RESSALVAS. A execução técnica funcionou, mas há defeitos registrados.
**Aprovação artística:** não aprovada. O proprietário pediu um novo take (Take 02) com um
pedido mais rígido.

> **Relatório recuperado em 2026-11-21.** Ele resume:
> - [`evidence/erro_corretion/etapa03d/RELATORIO.md`](../../../evidence/erro_corretion/etapa03d/RELATORIO.md),
>   com o bloqueio inicial e a rota externa;
> - [`evidence/erro_corretion/etapa03d/runs/take01/RELATORIO_take01.md`](../../../evidence/erro_corretion/etapa03d/runs/take01/RELATORIO_take01.md),
>   com o resultado.
>
> A hora exata da execução não foi registrada; abaixo estão os horários dos commits.

## 1. Identificação
Erro-Corretion 03D, Take 01. Branch `art/v1.2-protagonist`.

## 2. Data e hora (commits, -0300)

| Commit | Data e hora | Conteúdo |
|---|---|---|
| `b0bff8f` | 2026-11-19 23:38 | Relatório de **BLOQUEIO**: não havia rota de geração de imagem |
| `5cec023` | 2026-11-19 23:54 | Rota externa: a imagem é gerada no ChatGPT pelo proprietário |
| `1c2cabf` | 2026-11-20 00:15 | Commit do proprietário ("armor") com a imagem `Cavaleiro Gótico em Armadura Roxa.png` |
| `cea0443` | 2026-11-20 00:21 | Leitura do Take 01; a imagem foi renomeada para `inbox/armour_master.png` |
| `4feec7f` | 2026-11-20 00:43 | Correção registrada no relatório do Take 01 (restauração do cabelo N4) e preparação do Take 02 |

## 3. Objetivo
Testar se o Agent Sprite Forge, com uma fonte real de imagem, produz uma armadura original
melhor do que as tentativas anteriores. Entregar a imagem mestre, a prévia em pixel art e a
comparação, e parar antes de qualquer animação.

## 4. Prompt
- Prompt da etapa: `Md-usario/Erro-Corretion/03D_MODELAR_ARMADURA_PILOTO.md`.
- Texto enviado ao ChatGPT: `evidence/erro_corretion/etapa03d/request_pose/prompt.txt`.

## 5. Contexto inicial
- O ASF estava instalado (03C), mas sem rota de geração.
- A primeira execução foi **BLOQUEADA**.
- O proprietário escolheu a rota externa: o ChatGPT gera, o ASF só lê.

## 6. Ferramentas
- Agent Sprite Forge 0.6.0, `generate2dsprite/scripts/pixel_look.py pose`, offline;
- Python com Pillow;
- ChatGPT, usado pelo proprietário, fora do projeto.

## 7. Consultados
- `art-src/n3-5-fidelity/generated/candidate_106.png` e `art-src/n3-5-fidelity/source/candidate_106.txt`, só leitura;
- camadas N4 `art-src/n4-sprite-machine/stage/parts/idle/hair_back.png` e `hair_front.png`, só leitura.

## 8. Criados
- `evidence/erro_corretion/etapa03d/request_pose/`: referência, prompt, request e master;
- `evidence/erro_corretion/etapa03d/inbox/armour_master.png` e `inbox/LEIA.md`;
- `evidence/erro_corretion/etapa03d/scripts/`: `import_external.py`, `take_report.py`,
  `restore_hair.py` e `compare_takes.py`;
- `evidence/erro_corretion/etapa03d/runs/take01/`: pose, variantes, `hair_restore/`,
  `compare_check/`, folhas e métricas;
- `evidence/erro_corretion/etapa03d/runs/selftest_unchanged_reference/`.

## 9. Modificados
- `evidence/erro_corretion/etapa03d/RELATORIO.md`
- `evidence/erro_corretion/etapa03d/inbox/LEIA.md`
- `evidence/erro_corretion/etapa03d/scripts/import_external.py`
- `evidence/erro_corretion/etapa03d/runs/take01/RELATORIO_take01.md`: correção do cabelo,
  acrescentada como seção nova.

## 10. Removidos
Nenhum. A imagem do proprietário foi **renomeada**: `inbox/Cavaleiro Gótico em Armadura Roxa.png`
virou `inbox/armour_master.png`.

## 11. Alterações
O `pose` leu a imagem sobre a grade do N3.5 e aproveitou só as células que mudaram. O
resultado foi montado no canvas de 80×128, com pivô 40,118.

## 12. Decisões
- Não houve conserto à mão.
- As opções documentadas do `pose` foram testadas em `variants/`.
- O cabelo foi restaurado com as camadas N4, sem redesenho.

## 13. Comandos

```bash
python3 -I -B evidence/erro_corretion/etapa03d/scripts/import_external.py --image evidence/erro_corretion/etapa03d/inbox/armour_master.png --run take01
python3 -I -B evidence/erro_corretion/etapa03d/scripts/take_report.py --run take01
python3 -I -B evidence/erro_corretion/etapa03d/scripts/restore_hair.py --run take01
```

## 14–15. Testes
- **Autoteste com a referência sem mudança:** o `pose` recusa corretamente.
- **Recolocar o master:** reproduz o sprite aprovado com 0 de diferença.

## 16. Métricas
- **Grade:** imagem de 1254×1254 px, célula de 11,021 px; `pictureEqualShare` 60,5 %.
- **Encaixe:** deslocamento (26, 13), igual ao sprite aprovado.

| Região | Idênticos |
|---|---|
| Cabeça | 175 de 175 |
| Pernas | 465 de 465 |
| Tronco | 465 de 680 alterados |

- **Total:** 562 texels alterados; silhueta +97 e −2.
- **Cores:** de 17 para 29 (+12).
- **Contorno:** fechado, com a cor `K`.
- **Cabelo:** 24 texels de cabelo N4 trocados. Depois da restauração, 0.

## 17–19. Problemas
- **Encontrados:**
  - faixas douradas finas e anti-serrilhadas, que viram pontos em 1×;
  - quilha e gola quase somem;
  - a ombreira cobriu o cabelo;
  - cerca de 40 % das células fora da área pedida foram repintadas pelo ChatGPT e
    descartadas pelo `pose`.
- **Corrigidos:** o cabelo, pela restauração N4 (0 texels diferentes).
- **Pendentes:** a legibilidade da armadura em 1×.

## 20. Limitações
- O `pose` não tem máscara de proteção.
- A regra documentada pede uma imagem nova, não conserto.

## 21–22. Evidências
- `evidence/erro_corretion/etapa03d/runs/take01/take01_comparison_sheet.png`
- `evidence/erro_corretion/etapa03d/runs/take01/take01_torso_source_vs_1x.png`
- `evidence/erro_corretion/etapa03d/runs/take01/armour_on_canvas_80x128.png`
- `evidence/erro_corretion/etapa03d/runs/take01/hair_restore/hair_restore_comparison.png`
- `evidence/erro_corretion/etapa03d/runs/take01/compare_check/takes_comparison.png`
- `evidence/erro_corretion/etapa03d/runs/take01/compare_check/takes_dark_preview.png`
- `evidence/erro_corretion/etapa03d/runs/take01/source.png`: a imagem do ChatGPT.

## 23. Links
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/erro_corretion/etapa03d/runs/take01/RELATORIO_take01.md
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/erro_corretion/etapa03d/runs/take01/take01_comparison_sheet.png
- https://github.com/RaulDuque/ProjetoRST/commit/cea0443

## 24. Resultado
- **Técnico:** a leitura e o encaixe funcionaram, com cabeça e pernas preservadas.
- **Artístico:** a armadura é pouco legível em 1×.

## 25. Recomendação
Fazer um novo take com o pedido mais rígido de `request_pose/take02_prompt.txt`: quilha
explícita, dourado com 2 pixels ou mais, e cabelo por cima.

## 26. Decisões do proprietário
Gerar a imagem do Take 02 no ChatGPT.

## 27. Git
Commits `b0bff8f`, `5cec023`, `1c2cabf`, `cea0443` e `4feec7f`, todos publicados.
