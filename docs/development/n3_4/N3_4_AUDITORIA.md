# N3.4 — Auditoria inicial

## Estado

- Branch `art/v1.2-protagonist`, HEAD `8a39ab1a0d4401e97a75fdacb105eea784ef1ed0`, igual a `origin/art/v1.2-protagonist`.
- Árvore limpa, exceto arquivos não rastreados do proprietário em `Md-usario/` (briefings N3.4, N4 e N5 e `Ficha_de_Modelagem_do_Protagonista_Gotico.png`). Nenhum foi alterado nem commitado.
- Memória disponível antes de compilar: 7,7 GiB.

## Arquivos originais

| Arquivo | Rastreado | Como foi analisado |
|---|---|---|
| `image.png` | sim | aberto e visto (243×528) |
| `Ficha_Modular_do_Protagonista_Gotico.png` | sim | aberta e vista (1199×1312); vista lateral direita recortada |
| `white_mesh.glb` | sim | cabeçalho glTF lido em Python, sem abrir no Blender |
| `Ficha_de_Modelagem_do_Protagonista_Gotico.png` | **não** | aberta e vista (1536×1024), só leitura |

A "referência do protagonista equipado em pixel art", citada no briefing, não está no repositório. A ficha de modelagem mostra o personagem equipado, mas como ilustração, não em pixel art. Ela não foi usada como fonte de pixels. Para a base, ela não era indispensável.

## `white_mesh.glb`

glTF 2.0 binário, gerador `trimesh`. Um nó e uma malha (`tmpl5tac3v_.ply`): 94 020 vértices, 188 176 triângulos, só o atributo `POSITION`. Não tem material, textura, esqueleto nem animação. A caixa vai de (−0,40, −1,00, −0,29) a (0,38, 0,99, 0,28): cerca de 2 unidades de altura, eixo Y para cima. É uma escultura branca sem rig e não foi usada nesta etapa.

## Ferramentas

- O Pixelorama existe em `art-src/n3-proof/tools/` (ignorado pelo git). O agente não consegue operar uma interface gráfica, então **não houve pintura no Pixelorama**.
- Aseprite, Krita, GIMP e ImageMagick não estão instalados. Nada foi instalado.
- O Blender (snap) existe e não foi usado.
- O Pillow 12.1.1 do sistema falha em todo `resize(NEAREST)`. Foi usado o contorno `art-src/player/scripts/post/resample.py`, que já existia.
- Nenhum serviço de IA de imagem foi usado e nenhum arquivo foi enviado para fora.

## Integridade (SHA-256 depois da etapa)

```
15ac4ec2028fba9c50f337cbde840ddb5fadec6291a55b4e514de369f1e8d726  image.png
9247670387d245bdd2aee873b364726a3cb55c34bf334ffe9201baa55d501447  Ficha_Modular_do_Protagonista_Gotico.png
076158a98e3225eb06bff2acefda67e609571635f11910ebe14bf7482f939bf7  white_mesh.glb
d26c684541635dad7e5143bdfd91b8c9a69d31ee470d68402b7d64906c476f32  Ficha_de_Modelagem_do_Protagonista_Gotico.png
```

`git diff HEAD -- Md-usario` saiu vazio. `camera.ron`, `src/`, o save, os controles e os manifestos de produção não foram tocados.
