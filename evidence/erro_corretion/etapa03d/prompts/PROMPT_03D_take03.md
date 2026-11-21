# Prompt 03D — Take 03

São duas partes:
- **A:** o que você faz no ChatGPT;
- **B:** o texto que você cola no Claude Code depois que a imagem estiver na inbox.

---

## Parte A — No ChatGPT (feito pelo proprietário)

1. Abra uma **conversa nova** no ChatGPT, para que o take anterior não influencie o resultado.
2. Anexe `evidence/erro_corretion/etapa03d/request_pose_take03/reference.png`. É a mesma
   referência dos takes 01 e 02, idêntica byte a byte.
3. Cole **todo** o texto de `evidence/erro_corretion/etapa03d/request_pose_take03/prompt.txt`.
4. Baixe o PNG sem cortar, redimensionar nem editar.
5. Salve com este nome exato:

       evidence/erro_corretion/etapa03d/inbox/armour_master_03.png

6. **Antes de mandar para o Claude**, confira:
   - **cabelo:** fica por cima da ombreira e das costas;
   - **dourado:** só em poucas faixas grossas (gola, borda das 2 placas da ombreira,
     cinto e borda das escarcelas), sem linhas finas;
   - **quilha:** uma linha vertical visível no peito;
   - **ombreira:** 2 placas grandes e retas, não 3 lâminas finas;
   - **quadradinhos:** os da armadura têm o mesmo tamanho dos do cabelo e das botas;
   - **o resto:** rosto, calça e botas iguais à referência.

   Se a imagem falhar claramente em algum desses pontos, gere outra antes de processar. Se
   salvar várias, use `armour_master_03b.png`, `armour_master_03c.png` e assim por diante.

### O que muda em relação ao pedido do Take 02 (e por quê)

| Defeito visto no Take 01 | Pedido do Take 03 |
|---|---|
| O ChatGPT desenhou a armadura com "pixels" de meia célula e bordas suavizadas | Cada pixel da armadura tem o **mesmo tamanho dos quadrados do cabelo e das botas**: um quadrado inteiro de 9 px, nunca metade |
| Faixas douradas finas, que viraram pontos em 1× | O dourado aparece **só** em 4 lugares, sempre com 2 quadrados ou mais de espessura |
| Ombreira de 3 lâminas, que vira listras em 1× | A ombreira tem **2 placas**, cada uma com 4 quadrados ou mais de altura |
| Muitas listras paralelas e rebites | Proibidos listras, rebites e gravações. No máximo 4 roxos e 2 dourados |
| A ombreira cobriu o cabelo | O cabelo fica por cima da ombreira e das costas, como no Take 02 |
| A imagem veio com 1254×1254 px, e cerca de 40 % das células fora da área pedida foram repintadas | Saída de 1024×1024 com a figura no mesmo lugar, e **nada repintado fora da armadura** |

---

## Parte B — No Claude Code (cole depois de salvar a imagem)

```
Prompt 03D — Take 03: processar a imagem do ChatGPT e avaliar o resultado

Imagem de entrada: evidence/erro_corretion/etapa03d/inbox/armour_master_03.png
Pedido usado no ChatGPT: evidence/erro_corretion/etapa03d/request_pose_take03/

Antes de tudo:
- Confirme que a imagem existe e que o sha256 dela é DIFERENTE do de
  runs/take01/source.png. Se for igual ou se faltar, pare e relate como BLOQUEADO,
  sem processar.

Fluxo oficial (da raiz do projeto, um comando por vez):
  python3 -I -B evidence/erro_corretion/etapa03d/scripts/import_external.py --image evidence/erro_corretion/etapa03d/inbox/armour_master_03.png --run take03 --request evidence/erro_corretion/etapa03d/request_pose_take03
  python3 -I -B evidence/erro_corretion/etapa03d/scripts/take_report.py --run take03
  python3 -I -B evidence/erro_corretion/etapa03d/scripts/restore_hair.py --run take03
  python3 -I -B evidence/erro_corretion/etapa03d/scripts/compare_takes.py --takes take01 take03 --out take03/compare_check

Gere também a folha com 4 colunas (aprovado N3.5 | take01 final | take03 bruto |
take03 + cabelo N4) em 1×, 2×, tronco 8× e prévia sobre fundo escuro, salvando
runs/take03/takes_01_vs_03_comparison.png e runs/take03/takes_dark_preview.png
(pode generalizar scripts/compare_take02_sheet.py; não altere os resultados do take01/take02).

Medir e reportar:
- grade detectada e posição no canvas 80×128, pivô 40,118;
- pixels idênticos: cabeça, rosto, cabelo (camadas N4), pernas (calça e botas);
- pixels alterados no tronco e no total; silhueta ganha e perdida;
- transparência, total de cores e cores novas, contorno fechado com a cor K;
- cabelo antes e depois da restauração N4 (o esperado é 0 depois).

Avaliação visual, com base nas imagens:
- a armadura ficou mais legível que a do Take 01?
- a quilha aparece em 1×?
- o dourado mantém pelo menos 2 px na resolução do jogo?
- a ombreira lê como 2 placas planas?
- houve regressão no cabelo, no rosto, na calça ou nas botas?
- conclusão: aprovável para seguir / melhor que o Take 01, mas insuficiente / rejeitado.

Regras:
- Não redesenhar nem corrigir pixels à mão. Não gerar imagem por código, IA local,
  Blender ou PixelFX. Nenhuma chamada paga.
- Não alterar Rust, Bevy, assets/, os sprites aprovados, a N3.5, N3.6, N3.8, N4, as etapas
  anteriores nem os resultados do take01/take02.
- Não gerar animações. Não iniciar o Prompt 04.

Saídas: evidence/erro_corretion/etapa03d/runs/take03/ (RELATORIO_take03.md,
take03_comparison_sheet.png, take03_torso_source_vs_1x.png, armour_on_canvas_80x128.png,
hair_restore/, compare_check/, as duas folhas acima).

Relatório obrigatório: docs/relatorios/erro-corretion/03D_TAKE03.md, no padrão de
docs/relatorios/PADRAO_RELATORIO.md, com uma linha nova no INDICE.md e o
ULTIMO_RELATORIO.md atualizado. Status técnico e aprovação artística separados.

Git: commit só dos arquivos desta etapa e dos relatórios (nunca Md-usario/), push sem
force para a branch atual, e confirmar a sincronização. Resposta curta no terminal:
etapa, status, resumo, caminho do relatório, imagens principais, branch, hash e push.
```
