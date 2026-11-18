# N1 — Referências e licenças

Duas fontes foram citadas pelo proprietário para estudar comportamento, não para copiar conteúdo.

## SymphonyRecomp

Repositório público: [BlackLabelHQ/SymphonyRecomp](https://github.com/BlackLabelHQ/SymphonyRecomp). É uma recompilação de Castlevania: Symphony of the Night, jogo comercial. O README do projeto, lido em 18 de novembro de 2026, diz que o build pede uma cópia própria do disco norte-americano de PlayStation e que o código foi escrito por pessoas. A página do GitHub consultada não mostra uma licença SPDX. Um índice externo marca a licença como ausente. Sem licença, o padrão é que o direito fica com os autores. Isso não autoriza cópia.

Não entra neste jogo, vindo de lá ou do jogo original:

- código recompilado ou descompilado;
- sprite, animação, mapa, música, efeito, personagem ou qualquer arquivo extraído.

O que a N1 aproveita é de categoria: castelo com volta, habilidade que reabre sala antiga, minichefe fora da sala do chefe principal, segredo que o mapa não entrega. O desenho concreto está em `N1_CASTELO_LABIRINTICO.md` e usa as regiões deste projeto.

O outro endereço, `vgmuseum.com/mrp/2/extras-sotn.htm`, é material de um jogo alheio. Não foi baixado nenhum arquivo de lá. Serve, no máximo, para lembrar quantas famílias de movimento um jogo desse gênero mostra. Nenhum quadro foi copiado.

## Ferramentas

A tabela com licença, preço e o que foi de fato lido está em `N1_COMPARACAO_FERRAMENTAS.md`. Resumo do que pode ir para um jogo comercial, segundo essas fontes:

- Arte feita no Blender é do autor.
- Arte feita no Pixelorama ou no LibreSprite é do autor. O programa é que tem licença, não o PNG.
- Aseprite e Spine não foram comprados e não entram.
- Mixamo não cede o arquivo cru para redistribuir. Não foi usado.
- Hunyuan3D tem licença territorial e não foi usado. O GLB do protagonista foi gerado pelo `trimesh`, não por essa ferramenta.

## Decisão

Nenhum recurso externo entra em `assets/` sem uma linha neste arquivo: origem, licença, o que é, e se pode ir num binário vendido. Até essa linha existir, o asset não é commitado.

## Alternativas

Estudar o recomp por dentro para "ver como a sala é feita". O risco de copiar estrutura protegida é maior do que o proveito. A auditoria deste código já mostra sala, porta, requisito e câmera. Não faz falta abrir o outro projeto.

## Riscos

Um modelo baixado de uma biblioteca grátis com licença que proíbe jogo comercial. A regra da linha de origem pega isso antes do atlas.

## Recomendação

Não clonar o SymphonyRecomp. Não baixar ROM, sprite ou música. A próxima referência visual, se o proprietário quiser, é um quadro nosso lado a lado com a prova, não um asset de terceiro.
