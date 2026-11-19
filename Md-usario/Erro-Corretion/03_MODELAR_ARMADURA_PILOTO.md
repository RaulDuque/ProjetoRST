# PROMPT 03 — PILOTO DE ARMADURA SOBRE O PERSONAGEM JÁ VESTIDO

**Pré-requisito:** o proprietário aprovou explicitamente o resultado do Prompt 02B. Leia `00_LEIA_PRIMEIRO.md` e evidências da Etapa 02B antes de iniciar.

## Regra principal
**O sprite N3.5/N3.6 já vestido (cabelo, camisa, calça e botas) permanece como base visível.** Modelar **somente o equipamento** no Blender. O rig v007 é manequim invisível para posição, proporção, orientação e ancoragem. Não gerar corpo nu, reconstruir anatomia ou renderizar o figurino/corpo do rig. A couraça é sobreposta ao personagem existente; pode cobrir parte da camisa e expandir a silhueta externa.

## Piloto pequeno
Criar **UMA armadura gótica roxa e dourada**, composta de couraça com volume, gola, placas segmentadas, abas junto ao cinto e ombreiras salientes. É importante que pareça uma armadura de verdade, não pintura sobre camiseta. Não gerar espada, capa, grevas, luvas, calças ou lote de itens.

## Execução
1. Partir da cópia do rig alinhado do Prompt 02B. Ocultar de todo render o corpo, a casaca, a capa, o sabre e qualquer vestuário 3D original. O protagonista vestido aprovado é exibido como **referência de composição 2D**.
2. Criar peças **3D independentes** da armadura (couraça, ombreiras, gola, abas). Vincular suas partes ao rig conforme necessário e controlar a espessura, profundidade, encaixe e deformações. Não limitar geometria à máscara 2D da camisa.
3. Conferir o encaixe em idle e C1 #0/#2/#4. Respeitar a posição dos braços e cabelos. Não encostar peças em pontos sem evidência de ancoragem; reportar incertezas.
4. Criar prévias lado a lado da armadura isolada e **sobre o personagem vestido aprovado**, com marcações dos limites e da zona que encobrirá a camisa. Mostrar a nova silhueta, inclusive o volume de ombros.
5. Registrar arquivo `.blend` de trabalho, geometrias, materiais, poses/câmera, parâmetros e limites. Não sobrescrever o rig original nem sprites da N3.5/N3.6.

## Gate
A armadura precisa mostrar volume, leitura clara de peças e encaixe plausível em quatro poses. **Se parecer uma faixa colorida, PARE** e informe o motivo antes de avançar à redução pixel art. Não se autoaprovar.

**Não integrar ao Bevy, não implementar novo sistema de personagem, não instalar, não commitar, não fazer push. Aguardar aprovação para Prompt 04.**
