# N3.5 — Auditoria inicial

## Estado

- Branch `art/v1.2-protagonist`, HEAD `c1eb46c7e31cdb58926ab6dcdd8f04c13109f868`, igual a `origin/art/v1.2-protagonist`, que é o HEAD esperado pelo briefing.
- Árvore limpa, exceto arquivos não rastreados do proprietário em `Md-usario/`: os briefings N3.4, N3.5, N4 e N5 e `Ficha_de_Modelagem_do_Protagonista_Gotico.png`. Nenhum foi alterado nem commitado.
- Memória disponível antes de compilar: 9,9 GiB.

## Documentos lidos

`CLAUDE.md`, `docs/ARCHITECTURE.md`, `docs/DECISIONS.md` (até D-042), `N1_PIXEL_ART_32_64_BITS.md`, `N1_PIPELINE_3D_PARA_2D.md`, `N1_PERSONAGEM_MODULAR.md` e os relatórios finais da N3 à N3.4, além de `N3_4_RESOLUCAO.md` e `N3_4_AVALIACAO.md`.

Fato relevante da N1: a proposta de pixel art de lá era um buffer de 640×360 com escala 2 e o protagonista entre 80 e 110 texels. A câmera de 360 testada aqui é a mesma relação (720 px de janela, 2 px por unidade).

## Origem das referências "ficha (perfil)", "ficha lateral" e "reducao"

As duas folhas que o proprietário comentou são **evidências publicadas na N3.4**, e os rótulos são delas:

| Rótulo | Folha da N3.4 | Arquivo de origem no repositório |
|---|---|---|
| `ficha (perfil)` | `docs/development/n3_4/evidence/face_study.png` | recorte x 700–780, y 82–158 de `Ficha_Modular_do_Protagonista_Gotico.png` |
| `ficha lateral` | `docs/development/n3_4/evidence/body_comparison.png` | recorte x 680–775, y 80–418 da mesma ficha |
| `reducao` | `docs/development/n3_4/evidence/face_study.png` | `art-src/n3-4-original-protagonist/source/base_idle_draft.txt`, gerado por `reduce_reference.py` |

Portanto, o arquivo original da `reducao` **está disponível** e não é uma reconstrução. A resolução dele foi medida exatamente: figura de **106 px** de altura e 25 de largura, no canvas 80×128, a mesma resolução da N3.4. Na folha, o rosto aparecia ampliado 14 vezes.

## Ferramentas

- O Pixelorama existe em `art-src/n3-proof/tools/`, mas o agente não opera interfaces gráficas. **Não houve pintura em programa gráfico.**
- Nenhuma ferramenta de IA de imagem foi usada e nenhum arquivo foi enviado para fora.
- Usei Python + Pillow para recorte, máscara, redução, paleta, montagem e medição, e o Bevy para as capturas reais.

## Integridade (SHA-256 no fim da etapa)

```
15ac4ec2028fba9c50f337cbde840ddb5fadec6291a55b4e514de369f1e8d726  image.png
9247670387d245bdd2aee873b364726a3cb55c34bf334ffe9201baa55d501447  Ficha_Modular_do_Protagonista_Gotico.png
076158a98e3225eb06bff2acefda67e609571635f11910ebe14bf7482f939bf7  white_mesh.glb
d26c684541635dad7e5143bdfd91b8c9a69d31ee470d68402b7d64906c476f32  Ficha_de_Modelagem_do_Protagonista_Gotico.png
```

Os valores são iguais aos registrados na N3.4.
