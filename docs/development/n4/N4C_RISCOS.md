# N4-1 (C1) — Riscos

## Artísticos

1. **Anatomia abaixo da referência N4-B** (alto). Tronco chapado, sem músculos modelados, pés sem dedos, perna de trás fina. Se a C2 seguir o mesmo método sem uma passada artística, os seis quadros herdam o nível. Mitigação: avaliação do proprietário antes da C2, e uma passada de pixel art por um artista sobre as camadas `body`, `body_back` e `arm_front` (o formato aceita trocar só essas folhas sem mexer no resto).
2. **Perto e longe das pernas** (médio). A oclusão no quadril é ambígua na redução, e a escolha seguiu a marcha contralateral. Se o proprietário ler o vídeo de outro jeito, as camadas das pernas trocam em #0 e #2 (dados, não código).
3. **Braceletes pequenos na corrida** (baixo). Só aparecem onde o vídeo os mostra (sobretudo #4). Em #0 e #2 o da frente fica escondido.
4. **Cano das botas** (médio). Estimado por quadro (`boot_t`).
5. **Coerência entre quadros** (médio). Cada pose foi reconstruída à parte. Na C2, a corrida a 15 fps vai mostrar se tronco, crânio e pés "tremem" entre os quadros. O GIF da C1 (7,5 fps) já mostra parte disso.

## Técnicos

1. **Esqueletos à mão** (médio). Seis quadros vão pedir mais seis esqueletos. Um erro de articulação aparece como membro cortado ou fragmentado (aconteceu na canela de #4 e foi corrigido olhando a grade). A métrica de cavidades e a revisão visual pegam isso.
2. **Métricas de buraco** (médio). A primeira métrica, herdada do idle, não servia para poses abertas, e a segunda era trivial. A atual (cavidades fechadas) é correta, mas não pega um membro faltando que deixe o ar ligado à borda. A revisão visual continua obrigatória.
3. **Clipe `run_c1` separado** (baixo). É um clipe de prova com 3 quadros. A C2 cria o `run` de 6 quadros; o `run_c1` pode sair depois.
4. **Desempenho** (baixo). 14 sprites por personagem, uma folha por camada. Não foi medido; medir quando entrar no jogo.

## Mantidos

Produção V2.2, modo N3.8, save v3, física, combate, câmera e o idle N4-A: inalterados.
