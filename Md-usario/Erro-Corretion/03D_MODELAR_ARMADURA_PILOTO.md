# DARK CASTLE RPG — ETAPA 03D: NOVA ARMADURA COM AGENT SPRITE FORGE

Quero realizar um experimento artístico independente usando as novas skills do Agent Sprite Forge 0.6.0, instaladas na Etapa 03C.

**Objetivo:** utilizar exclusivamente o protagonista original aprovado N3.5/N3.6 como referência de identidade e criar uma NOVA versão dele usando uma armadura gótica medieval de fantasia, desenhada do zero por um modelo de geração de imagens.

Não quero que você utilize como referência as armaduras das etapas N41-B, 03 ou 03B. Este é um experimento independente para descobrir se a geração de imagens produz resultados visualmente superiores ao pipeline Blender atual.

## Referências obrigatórias

Utilize os arquivos originais aprovados:

- `art-src/n3-5-fidelity/generated/candidate_106.png`
- `art-src/n3-6-animation-proof/generated/run6.png`

O protagonista deve manter cabelo prateado comprido, rosto, proporções, direção lateral e identidade artística.

A camisa, a calça e as botas originais devem continuar sendo a base do personagem. A nova armadura precisa parecer equipada por cima dessa aparência, podendo ocultar corretamente as partes cobertas.

## Ferramentas

Use as skills oficiais `generate2dsprite`, `video2dsprite`, `pixel_look` e `pixel_motion` conforme forem aplicáveis.

Antes de gerar, execute os diagnósticos do Agent Sprite Forge e verifique se existe uma rota REAL de geração de imagens.

Na Etapa 03C não havia chave de API nem Codex/Grok CLI disponível. Caso isso continue assim, informe claramente que a geração está bloqueada. Não invente imagens geradas, não substitua o teste por desenhos procedurais em Python e não instale serviços pagos ou configure credenciais por conta própria.

Não faça chamadas pagas sem autorização específica.

## Nova armadura

Crie um design original de armadura roxa e dourada, com estética gótica medieval, peitoral estruturado, ombreiras em placas, proteções de antebraço e elementos de cintura.

Exijo uma aparência que pareça verdadeiramente desenhada para um jogo de qualidade profissional, e não uma recoloração da camisa original.

Evite ombreiras esféricas, frisos excessivamente finos, superfícies de plástico e detalhes que se percam em pixel art.

A primeira entrega deve ser uma única imagem mestre do personagem equipado, de perfil, olhando para a direita.

**Não gere várias animações antes de eu aprovar essa imagem mestre.**

Se a imagem mestre for aprovada, a próxima fase poderá usar `video2dsprite` e `pixel_motion` para testar idle e corrida, tomando as poses N3.6 como referência. Não presuma que a geração de vídeo consegue preservar automaticamente a armadura em todos os quadros.

## Comparação e qualidade

Compare o novo resultado com o protagonista original, sem mudar o original.

Verifique se o rosto, o cabelo e as proporções continuam reconhecíveis, se a armadura possui silhueta própria e se a conversão para a resolução de jogo preserva as formas essenciais.

Não altere Rust, Bevy, os sprites aprovados, as composições N4, os modelos Blender nem a N41-B.

Não execute o Prompt 04.

## Evidências e Git

Salve este experimento em `evidence/erro_corretion/etapa03d/`.

Entregue a imagem mestre original gerada, uma prévia de conversão para pixel art, uma comparação lado a lado com o personagem aprovado e um relatório curto com a ferramenta e a rota efetivamente utilizadas.

Informe os caminhos exatos dos PNGs para que eu possa enviá-los ao ChatGPT e aprovar pessoalmente.

Ao finalizar, faça commit e push somente dos arquivos pertinentes na branch `art/v1.2-protagonist`, sem force-push. Não inclua arquivos pessoais de `Md-usario/`.

**Critério final:** quero comprovar se o Agent Sprite Forge consegue, com uma fonte real de geração de imagens, criar uma armadura original visualmente melhor do que as tentativas anteriores.

Pare após a imagem mestre e sua comparação. Aguarde aprovação antes de gerar animações.