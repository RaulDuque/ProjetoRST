# N1 — Pipeline 3D para sprites

O repositório já faz uma versão desta abordagem. `build_game_sprites_v2_2.py` abre um `.blend` em leitura, esconde as peças do sabre, renderiza o corpo e o pós-script monta atlas, manifesto e âncoras. O jogo lê `assets/animations/player.ron`. Isso é a abordagem D em miniatura: manequim 3D, quadro ortográfico, folha 2D, composição no Bevy.

A N1 não troca esse fluxo e não escolhe a abordagem D como definitiva. A qualidade de pixel ainda não foi julgada pelo proprietário.

## O GLB existente

`Md-usario/assets-user/prota/base/white_mesh.glb` foi só lido. 3.387.148 bytes, mtime 2026-11-16 23:42. glTF 2.0, gerador `trimesh`.

- 1 malha, nome `tmpl5tac3v_.ply`, 1 primitiva, modo triângulo.
- 94.020 vértices, 564.528 índices.
- Atributo único: `POSITION`. Sem normal, UV, material, pele, osso ou animação.
- Caixa: X −0,40 a 0,38, Y −1,00 a 0,99, Z −0,29 a 0,28. Altura perto de 2, largura perto de 0,79.

Não está pronto para animar. Não há esqueleto para o Mixamo receber, e a densidade é de escultura, não de manequim de produção. Qualquer teste futuro usa uma cópia fora de `Md-usario/`. O original não se move.

## Quatro abordagens

| | O que é | Contra o que já existe |
|---|---|---|
| A — 3D para folha | render ortográfico de um rig, depois pixel | é o caminho do script V2.2; o GLB novo ainda não entra nele |
| B — rig 2D | partes com osso em runtime | o Bevy deste jogo não tem runtime de osso 2D; ver o doc de animação esquelética |
| C — quadro a quadro | cada pose desenhada | controle máximo, e cada arma vira outra folha |
| D — híbrido | A para o corpo e as famílias, Pixelorama no acabamento, camada no Bevy, quadro desenhado só na exceção | hipótese de trabalho, porque junta o que já roda com o acabamento que falta |

## Fluxo proposto para a prova, não para agora

1. Copiar a malha para um diretório de trabalho, nunca o GLB original.
2. Decimar ou refazer a topologia até um corpo que o rig aguente.
3. Criar esqueleto e peso. Isso não existe no arquivo.
4. Quatro clipes: idle, walk, jump, attack. Reusar osso, não reusar a malha crua como sprite.
5. Duas armas e duas armaduras como malhas separadas, no mesmo esqueleto.
6. Câmera ortográfica, mesma densidade da folha (`density` 2 hoje).
7. Pixelorama: contorno, paleta, apagar o cinza de render.
8. Exportar PNG no contrato que `AnimationManifest` já valida.
9. Bevy compõe com as camadas que `held_visual` e a capa já usam.

## Alternativas

Animar o GLB como está. Sem pele, não há o que animar. Ou desenhar os quatro clipes só no Pixelorama, sem 3D. É a abordagem C e serve de controle visual da prova, se o render sair com cara de plástico.

## Riscos

Pixels irregulares na borda do render. Volume de quadros se cada equipamento virar folha própria. O script atual esconde um sabre que estava modelado no blend; um equipamento novo tem de nascer como malha separada, não como peça assada no corpo. A V2.2 já corrigiu esse erro uma vez (`weapon_in_body: false`).

## Recomendação

Manter D como hipótese. A prova compara um clipe renderizado e corrigido com um clipe desenhado à mão do mesmo gesto. Sem esse par, a escolha fica no gosto.
