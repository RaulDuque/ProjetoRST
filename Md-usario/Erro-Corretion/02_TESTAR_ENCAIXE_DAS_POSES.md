# PROMPT 02 — PROVA DE ENCAIXE DO RIG NAS QUATRO POSES

Leia `00_LEIA_PRIMEIRO.md` e o resultado aprovado do Prompt 01. Execute apenas se o rig for plausivelmente reaproveitável.

**Problema a resolver:** o Blender foi reprovado como personagem inteiro na N3.1; aqui ele deve funcionar como **manequim técnico invisível**, enquanto o protagonista 2D aprovado permanece intacto. Não confunda essas propostas.

1. Em cópia de trabalho isolada e reversível, configure a câmera lateral ortográfica e escala/pivô equivalentes à arte N3.5/N3.6.
2. Posicione o rig nos quadros reais: idle e run #0, #2, #4 (C1). Use dados de poses existentes, não coordenadas inventadas.
3. Renderize **somente guias transparentes** ou volumes simplificados temporários com pontos articulares para verificar encaixe. O objetivo é alinhamento, não estética.
4. Sobreponha cada guia ao sprite aprovado e entregue 4 imagens com grid, pivô e landmarks identificados: cabeça, pescoço, ombro, cotovelo, mão, cintura, quadril, joelho e tornozelo, onde forem discerníveis.
5. Meça desvios em pixels após conversão final. Distinga diferenças reais de diferenças causadas por câmera/escala. Verifique oclusão no idle e no run, principalmente braços cruzando o peito.
6. Investigue a viabilidade de criar peças que extrapolem a máscara da camisa, com máscaras de oclusão de braço por pose. Não modifique a camada original neste teste.

**Resultado obrigatório:** `PASSA`, `PRECISA DE AJUSTE` ou `FALHA`, com justificativa visível, imagens lado a lado e lista de ajustes para cada pose. Declare explicitamente se o pipeline consegue preservar a silhueta original do corpo e adicionar volume externo à armadura.

**Gate de parada:** se rig/câmera/poses não encaixarem de forma convincente ou exigirem redesenhar o personagem inteiro, **pare antes de construir armaduras**. Informe o custo de correção ou indique o Prompt 08.

**Proibições:** não alterar código Rust, Python de produção, rig original, sprites aprovados, nem criar commits/push. **Não avance automaticamente ao Prompt 03.**
