# PROMPT 08 — ALTERNATIVA CONTROLADA SE O RIG EXISTENTE NÃO SERVIR

**Executar somente se o Prompt 01 ou 02 comprovar que o rig v007 é inadequado ou que seu conserto custaria mais que uma alternativa.** Leia `00_LEIA_PRIMEIRO.md`.

**Objetivo:** evitar mais uma sequência de tentativas improdutivas. Não migrar de engine, não retornar automaticamente ao pintor de 15×22 pixels e não criar sistema novo antes de prova visual.

1. Explique com evidências **por que** o rig falhou (proporção, câmera, poses, deformação, materiais ou acesso ao arquivo). Estime correções limitadas possíveis.
2. Compare três saídas concretas: (A) ajustar/retargetear o rig existente; (B) criar um manequim 3D técnico mínimo só para o equipamento, sem corpo renderizado; (C) criar UMA arte 2D em alta resolução com armadura e reduzi-la conforme a N3.5, conservando a exigência de repetibilidade nas quatro poses.
3. Para cada alternativa, dizer o que reutiliza, o que precisa ser criado, os riscos de consistência entre quadros e o custo por item. Não inventar ferramentas instaladas, qualidade alcançada ou resultados.
4. Recomendar **um único experimento visual barato e reversível**. Propor condições explícitas de parada caso ele também falhe.

**Entregue apenas diagnóstico, referência visual se já existir e plano do menor teste.** Não implementar alternativas em paralelo, não instalar nada, não modificar código, não commitar nem fazer push sem aprovação do proprietário.
