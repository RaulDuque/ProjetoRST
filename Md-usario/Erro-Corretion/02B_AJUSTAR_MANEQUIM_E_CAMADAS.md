# PROMPT 02B — CORRIGIR O ENCAIXE E PREPARAR CAMADAS DA BASE VESTIDA

Leia `00_LEIA_PRIMEIRO.md` revisado e `evidence/erro_corretion/etapa02/RELATORIO.md`. As etapas 01 e 02 já aconteceram. **Não repita inspeções anteriores nem crie armadura agora.**

## Objetivo
Eliminar os bloqueios reais para **sobrepor** uma couraça e ombreiras no protagonista já vestido N3.5/N3.6. O rig v007 é guia técnico invisível; o sprite aprovado é a única base visual. **Não gerar nudez, corpo-base nu, personagem substituto ou nova animação de corpo inteiro.**

## Atividades limitadas
1. Abrir apenas uma cópia de trabalho do rig alinhado na Etapa 02 (confirmar caminho real). Nos frames C1 #0 e #2, ajustar somente clavículas/ombros para coincidir com o sprite. Manter referências, câmera, escala, pivô e arte original. Medir erros antes/depois. Não é necessário corrigir pernas para um teste apenas de torso/ombros; registrar limitações.
2. Preparar em **arquivos derivados de teste** a separação lógica entre manga e tronco no idle, usando exclusivamente texels já existentes do sprite vestido. A união das camadas no modo **sem armadura** deve reproduzir exatamente o sprite aprovado, pixel a pixel. Não inventar anatomia oculta para preencher recortes.
3. Corrigir, somente nas máscaras/camadas derivadas do piloto, os pixels de cabelo classificados como braço nos frames #0/#2. Verificar o cabelo preservado e a oclusão de braço/mão sobre futuras peças.
4. Definir zonas e ordem de sobreposição por pose: cabelo, braço/manga próximos, peitoral e ombreiras, braço distante, camisa subjacente. Identificar o que deve ser coberto, o que fica na frente e onde há espaço para aumentar a silhueta além da camisa. Mostrar máscaras translúcidas de exemplo, **sem desenho de armadura**.
5. Criar quatro prévias comparativas (idle, #0/#2/#4) com: sprite original, guias, divisão de camadas e reconstrução vestido-sem-armadura. Mostrar diferença pixel a pixel e medidas de erro das articulações. Confirmar que o personagem preserva camisa, calça, botas e cabelo.

## Entrega e gate
- Evidências e qualquer `.blend`/máscara de teste em `evidence/erro_corretion/etapa02b/` (ou pasta temporária informada), sem apagar etapas anteriores.
- Apresentar, de forma curta: ombros antes/depois, resultado da separação manga/tronco, cabelos corrigidos, oclusão prevista e **contagem de pixels divergentes na reconstrução sem armadura** (meta: zero).
- Se não for possível reconstruir a base vestida intacta, **PARE** e explique exatamente a limitação, sem avançar à Etapa 03.
- Solicitar aprovação explícita do proprietário antes de modelar a couraça.

**Proibições:** não editar arte final, N4 de produção, código Rust/Bevy, save, rig original, nem criar armadura ou corpo nu. Sem instalações, commits, push ou avanço automático.
