# Comparação de pipelines (com demonstração D4)

| Critério | A. Blender peças 3D → sprites | B. Overlays 2D por quadro | C. Híbrido (Blender pose + pixel D2) |
|---|---|---|---|
| Fidelidade ao D2 | Média–alta se materiais/luz baterem; EEVEE cru não é D2 | Alta se o pixel for desenhado no estilo D2 | Mais alta: pose estável + acabamento D2 |
| Consistência entre frames | Alta (rig) | Frágil se cada overlay for gerado isolado | Alta na pose; polish controla o look |
| Novas armas/armaduras | Trocar mesh / collection | Novo overlay por clip | Mesh guia + pixel da peça |
| Sockets Bevy | Já medidos no v007 | Exigem marcação 2D por quadro | Reusa medição Blender |
| Custo | Setup 3D + export | Explode com nº de clips × peças | Intermediário |
| Demo nesta fase | Conceitual (rig já existe) | **Protótipo idle + motion** em `combinacoes/` / `movimento/` | Recomendado para produção |

## O que o protótipo D4 demonstrou

- **B (overlays):** camadas independentes empilháveis no idle (`camadas/`,
  `combinacoes/`). Estilo mira D2, não o geométrico D3.
- **Motion:** duas loadouts (leve vs pesada) em idle↔run (`previews/motion_*.gif`).
  Revela desalinhamento típico de overlays não parentados a sockets.
- **A:** não houve export novo do Blender nesta fase; o pipeline V2.2 já prova
  que peças (sabre) podem sair do body e voltar via socket.

## Recomendação (após demo)

Para **preservar D2** e escalar equipamentos: **C — híbrido**.

1. Body D2 pixelado (ou render limpo) sem equipamentos baked.
2. Peças de silhueta (capa, peitoral pesado, elmo) como collections Blender
   **ou** overlays pixel com âncora `head`/`chest`/`back`.
3. Armas/escudo continuam no caminho V2.2 (`held_visual`) com arte melhorada.

Não declarar A ou B como vencedor absoluto: A sozinho não entrega look D2;
B sozinho não escala em 22 clips sem disciplina de sockets.
