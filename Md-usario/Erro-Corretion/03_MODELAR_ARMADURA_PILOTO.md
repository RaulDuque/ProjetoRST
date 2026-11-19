# PROMPT 03 — CRIAR UMA ÚNICA ARMADURA 3D MODULAR

Leia `00_LEIA_PRIMEIRO.md`. Só execute se o Prompt 02 foi aprovado pelo proprietário.

**Meta:** criar UMA armadura gótica roxa com detalhes dourados capaz de transformar claramente a silhueta do protagonista 2D aprovado. Não crie 40 itens, não redesenhe o corpo, não comece integração Bevy.

**Design mínimo obrigatório**
- Couraça roxa metálica com placas segmentadas, peça central com volume, gola/gorjal e acabamento dourado.
- Ombreiras pronunciadas que **sobressaiam ao contorno da camisa**, sem se separar dos ombros durante a corrida.
- Segmentos na cintura/abas que criem leitura visual inequívoca de armadura, não de camiseta recolorida.
- Materiais e iluminação coerentes com a orientação de luz da arte N3.5/N3.6, ainda que a correspondência perfeita dependa da conversão.
- Geometria/rigging modular **separados do corpo**: peças parentadas/skinadas ao esqueleto. Não modelar capa nem espada neste piloto, para isolar a principal causa de erro.

**Execução**
1. Reaproveite o rig v007 e as poses registradas no Prompt 02; preserve o original e salve qualquer cópia de teste separadamente.
2. Crie a geometria real do equipamento no Blender. Scripts Blender podem automatizar tarefas, mas não substituir volume tridimensional por pinturas de faixas na camisa.
3. Teste encaixe e deformação nas 4 poses. Evite clipping com braços/mãos; planeje oclusões 2D específicas por pose, se necessário.
4. Gere prévias de alta resolução, vista lateral ortográfica idêntica à dos testes. Mostre, para cada pose, armadura isolada e sobre o protagonista aprovado.
5. Liste as peças reutilizáveis, o esforço real necessário para criar outro modelo e o que não pode ser automatizado com segurança.

**Evidência:** imagens da armadura isolada, em composição com personagem e prévia das quatro poses, além do arquivo `.blend` de teste se viável. Se a prévia não tiver silhueta bem diferente da camisa, **pare e revise somente o modelo**.

**Não fazer:** novos programas, dezenas de modelos, modificar jogos/sprites originais, commits ou push. **Não avançar ao Prompt 04 sem aprovação.**
