# PROMPT 07 — PROVAR ESCALABILIDADE SEM REPETIR A N41-B

**Só executar após integração do Prompt 06 comprovada e aprovada pelo proprietário.** Leia `00_LEIA_PRIMEIRO.md`.

**Objetivo:** validar que o pipeline aceita equipamentos verdadeiramente diferentes, e não apenas recolorações da mesma camisa. Não gerar um catálogo grande.

1. Criar **um segundo modelo de couraça geometricamente distinto** do primeiro (por exemplo, casaco de brigandina ou armadura pesada), usando o mesmo manequim, poses, câmera e exportador, sem alterar Rust especificamente para esse item.
2. Criar **uma variante de material** do primeiro modelo (ex.: aço escuro), para demonstrar diferença entre modificar **geometria** e apenas **paleta**.
3. Comparar os dois modelos nas quatro poses quanto a silhueta, proporção, oclusão, estabilidade e qualidade após redução.
4. Medir esforço real: criação de malha, rigging/pesos, materiais, exportação, correções, tamanho dos arquivos, tempo de geração. Não prometer custo zero por item.
5. Propor, **sem implementar ainda**, arquitetura escalável para slots futuros de calças, botas, luvas, capacete, ombreiras, espadas e capas. Diferenciar: (a) peças rígidas fáceis de anexar; (b) peças deformáveis; (c) espadas com soquete de mão; (d) capas com animação/simulação; (e) oclusões entre peças.
6. Inspecionar o status verdadeiro das espadas/capas/ombreiras N41-B: prévias isoladas **não são prova de equipagem integrada**. Reutilizar contratos apenas onde forem válidos.
7. Entregar recomendação sobre gerar sprites offline por combinação, camadas por slot no Bevy, ou solução híbrida. Considerar crescimento combinatório das combinações, armazenamento e número de poses, sem inventar benchmarks.

**Critérios:** ao menos dois modelos com silhuetas claramente diferentes, um variante de material, integração data-driven, preservação da arte base, evidência visual aprovada.

**Não executar em massa, não criar 100+ armaduras, não alterar gameplay, não instalar, commitar ou fazer push.** Parar e apresentar custos/limites antes de expandir.
