# Erro-Corretion 03D — Take 03: armadura simplificada lida pelo Agent Sprite Forge

**Status:** AGUARDANDO APROVAÇÃO VISUAL. A execução técnica terminou sem erros.
**Aprovação artística:** pendente. A avaliação do Claude é **"melhor que o Take 01, mas
ainda insuficiente"**.

## 1. Identificação
- Etapa: Erro-Corretion 03D, Take 03.
- Branch: `art/v1.2-protagonist`.

## 2. Data e hora
2026-11-21, por volta das 15:35–16:00 (-0300).

## 3. Objetivo original
Processar a imagem do ChatGPT do Take 03 pelo fluxo oficial, medir, restaurar o cabelo com
as camadas N4, comparar com o Take 01, avaliar e publicar.

## 4. Prompt executado
Parte B de `evidence/erro_corretion/etapa03d/prompts/PROMPT_03D_take03.md`. O pedido enviado
ao ChatGPT está em `evidence/erro_corretion/etapa03d/request_pose_take03/prompt.txt`.

## 5. Contexto técnico inicial
- Take 01: arte não aprovada.
- Take 02: REJEITADO, porque usou a mesma imagem.
- Na conferência, `inbox/armour_master_03.png` **não existia**. A imagem nova (sha256
  `e46890d4…4cb5b66`, diferente do Take 01) estava salva em
  `request_pose_take03/armour_master_03.png`. Ela foi movida sem alteração para `inbox/`, e o
  processamento seguiu.

## 6. Ferramentas
- Agent Sprite Forge 0.6.0 (`pixel_look.py pose`), offline;
- Python 3 com Pillow;
- ChatGPT, usado pelo proprietário.

## 7. Arquivos consultados
- `request_pose_take03/`
- os resultados do Take 01
- o N3.5 aprovado (`art-src/n3-5-fidelity/generated/candidate_106.png`), só leitura
- as camadas N4 de cabelo (`art-src/n4-sprite-machine/stage/parts/idle/hair_back.png` e `hair_front.png`), só leitura

## 8. Arquivos criados
- `evidence/erro_corretion/etapa03d/inbox/armour_master_03.png`, a imagem do proprietário
  depois de movida;
- `evidence/erro_corretion/etapa03d/runs/take03/`, com o relatório técnico
  `RELATORIO_take03.md`;
- `docs/relatorios/erro-corretion/03D_TAKE03.md` (este arquivo).

## 9. Arquivos modificados
- `evidence/erro_corretion/etapa03d/scripts/compare_take02_sheet.py`: ganhou a opção
  `--take`. Sem a opção, gera para o take02 as mesmas saídas de antes, conferido com
  `sha256sum -c`.
- `docs/relatorios/INDICE.md`
- `docs/relatorios/ULTIMO_RELATORIO.md`

## 10. Arquivos removidos
Nenhum. A imagem foi **movida** de `request_pose_take03/` para `inbox/`, e ainda não estava
versionada.

## 11. Descrição das alterações
- Rodado o fluxo oficial completo.
- Gerada a folha de 4 colunas e a prévia sobre fundo escuro.
- Medida a espessura do dourado em 1×.

## 12. Decisões
- A imagem foi movida em vez de bloquear a etapa, porque o conteúdo era novo e válido: só a
  pasta estava errada.
- O script do take02 foi generalizado em vez de copiado, sem mudar as saídas do take02.

## 13. Comandos
Estão em `evidence/erro_corretion/etapa03d/runs/take03/RELATORIO_take03.md`, na seção
"Reprodução".

## 14. Testes realizados
- Conferência de hash da entrada contra o Take 01.
- `sha256sum -c` das saídas do take02 depois da mudança no script.
- Métricas por região.
- Contagem da espessura do dourado.

## 15. Resultado dos testes
- A entrada é nova.
- As saídas do take02 continuam idênticas (OK).
- O fluxo terminou sem erros.

## 16. Métricas

| Medida | Take 01 | Take 03 |
|---|---|---|
| Cabeça idêntica | 175/175 | 175/175 |
| Rosto idêntico | 31/33 | **33/33** |
| Pernas idênticas | 465/465 | 465/465 |
| Cabelo N4 trocado → depois da N4 | 24 → 0 | **3 → 0** |
| Tronco alterado | 465 | 454 |
| Total alterado | 562 | 500 |
| Silhueta | +97 / −2 | +46 / −4 |
| Cores | 29 (+12) | 29 (+12) |
| Contorno `K` | 62/62 | 55/55 |
| Texels dourados | 160 | 53 |
| Trechos de dourado com 2 px / 1 px | 34+ / 52 | 15 / 23 |

- **Grade:** imagem de 1254×1254 px, ou seja, a saída de 1024 pedida não foi respeitada;
  célula de 11,021 px; `pictureEqualShare` 63,6 %.
- **Encaixe:** (26, 13), igual ao aprovado.
- **Take 01 × Take 03:** 563 px diferentes.

## 17. Problemas encontrados
- A imagem estava fora da inbox.
- O ChatGPT entregou a imagem com 1254 px de novo.
- Cerca de 60 % das colunas das faixas douradas têm só 1 px.
- A ombreira não lê como placas.
- O conjunto lê como túnica, não como armadura.
- Em `hair_restore.json`, o campo `take01_report_note` é um rótulo fixo do script com
  dados do take03.

## 18. Problemas corrigidos
- A posição da imagem, que foi movida para `inbox/`.
- O cabelo, restaurado com as camadas N4 (0 texels diferentes).

## 19. Problemas pendentes
- Volume e degrau das ombreiras.
- Quilha central.
- Dourado sempre com 2 px.
- O nome fixo do campo em `restore_hair.py`. Não mudei, para não alterar as saídas dos takes
  anteriores.

## 20. Limitações
- O `pose` não tem máscara de proteção.
- O ChatGPT não segue com precisão tamanho nem grade.
- Não há conserto à mão, por regra.

## 21. Evidências visuais
- **Folha de 4 colunas:** aprovado, take01, take03 bruto e take03 com N4, em 1×, 2× e tronco 8×.
- **Prévia escura.**
- **Tronco:** imagem do ChatGPT contra a leitura em 1×.

## 22. Caminhos
- `evidence/erro_corretion/etapa03d/runs/take03/takes_01_vs_03_comparison.png`
- `evidence/erro_corretion/etapa03d/runs/take03/takes_dark_preview.png`
- `evidence/erro_corretion/etapa03d/runs/take03/take03_torso_source_vs_1x.png`
- `evidence/erro_corretion/etapa03d/runs/take03/take03_comparison_sheet.png`
- `evidence/erro_corretion/etapa03d/runs/take03/armour_on_canvas_80x128.png`
- `evidence/erro_corretion/etapa03d/runs/take03/hair_restore/armour_hair_restored_80x128.png`
- `evidence/erro_corretion/etapa03d/inbox/armour_master_03.png`

## 23. Links do GitHub
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/erro_corretion/etapa03d/runs/take03/RELATORIO_take03.md
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/erro_corretion/etapa03d/runs/take03/takes_01_vs_03_comparison.png
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/erro_corretion/etapa03d/runs/take03/takes_dark_preview.png
- https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/erro_corretion/etapa03d/runs/take03/take03_torso_source_vs_1x.png

## 24. Resultado final
**Melhor que o Take 01, mas ainda insuficiente.**
- **Ganhos:** legibilidade limpa e nenhuma regressão.
- **Perdas:** volume e leitura de metal.

## 25. Recomendações
Em um Take 04 (só se o proprietário quiser):
- manter a regra de pixels inteiros e de poucas cores;
- pedir uma **ombreira mais larga que o braço**, saindo 2 a 3 quadrados para fora da
  silhueta, com um degrau visível entre as placas;
- pedir o **dourado com exatamente 2 quadrados de altura**, sem a linha de sombra embaixo;
- pedir **uma linha de brilho e uma de sombra** em cada placa, para ler como metal;
- pedir a quilha como uma aresta **dentro** do peito, não na borda.

## 26. Decisões do proprietário
- Aprovar visualmente o Take 03, pedir um Take 04 ou voltar à rota Blender da 03B.
- O Prompt 04 continua bloqueado até haver aprovação.

## 27. Estado do Git
- Branch: `art/v1.2-protagonist`.
- Commit: o que adicionou este arquivo. O hash está na resposta final.
- Push: sem force.
- `Md-usario/`, incluindo a pasta `Md-usario/engine/` que apareceu sem versionamento, ficou
  fora do commit.
