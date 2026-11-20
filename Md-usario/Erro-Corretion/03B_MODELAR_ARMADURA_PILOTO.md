# DARK CASTLE RPG — ETAPA 03B: REFINAMENTO DA ARMADURA GÓTICA

A Etapa 03 foi aprovada como piloto técnico, mas ainda precisa de refinamento artístico antes do Prompt 04.

Execute uma etapa intermediária chamada **03B — Refinamento Visual e Preparação para Pixel Art**.

Leia os documentos e resultados das etapas 02B e 03 em `Md-usario/Erro-Corretion/`, além dos relatórios e arquivos correspondentes em `evidence/erro_corretion/`.

**Objetivo:** melhorar a armadura gótica roxa e dourada criada no Blender, preservando a arquitetura modular e preparando suas formas para uma redução de alta qualidade em pixel art 2D.

## 1. Corrigir o quadro de corrida #4

O quadro `run_c1_4` apresenta o pior encaixe entre as quatro poses.

- Corrija a inclinação e o posicionamento do tronco do manequim nessa pose.
- Elimine a faixa de camisa que continua aparecendo na frente do peito.
- Melhore o encaixe entre couraça, ombreiras e cintura.
- Preserve a postura e a silhueta do protagonista aprovado.
- Não esconda defeitos simplesmente cobrindo tudo com uma imagem maior.
- Verifique se o ajuste não provoca interseções entre peças ou problemas de oclusão.

**Critério de aprovação:** nenhuma faixa indevida da camisa visível sobre o peitoral.

## 2. Refinar as ombreiras

As ombreiras atuais estão excessivamente arredondadas, parecendo esferas metálicas.

Quero uma aparência mais próxima de uma armadura gótica medieval de fantasia.

- Reduza a aparência esférica.
- Crie planos e arestas mais definidos.
- Valorize as três lâminas segmentadas existentes.
- Acrescente uma silhueta mais marcante, sem exagerar nas dimensões.
- Preserve o movimento dos braços.
- Evite detalhes que desapareçam na redução para aproximadamente 106 pixels de altura do personagem.

As ombreiras devem parecer peças metálicas de proteção, não bolas roxas presas aos ombros.

## 3. Melhorar os detalhes dourados

Os frisos dourados atuais parecem fios metálicos muito finos.

- Transforme os principais frisos em bordas ou faixas metálicas com largura visual adequada.
- Simplifique linhas paralelas excessivamente próximas.
- Destaque a quilha central do peitoral.
- Preserve os ornamentos que ajudam a identificar a armadura.
- Evite dezenas de detalhes microscópicos que desapareceriam na redução.

Priorize formas grandes, intermediárias e detalhes realmente legíveis.

## 4. Corrigir materiais e iluminação

O metal roxo apresenta brilho excessivamente plástico.

- Ajuste materiais e iluminação para representar uma armadura de metal pintado ou esmaltado.
- Reduza reflexos arredondados excessivos.
- Utilize regiões de luz e sombra mais controladas.
- Valorize a geometria por meio de planos e arestas.
- Mantenha o roxo e o dourado como identidade principal.

Não é necessário produzir renderização fotorrealista. O objetivo é criar uma boa imagem-fonte para pixel art.

## 5. Melhorar a gola

A gola alta está pouco visível na vista lateral porque o cabelo cobre grande parte dela.

- Revise sua geometria e altura.
- Preserve a ordem de camadas aprovada.
- Não coloque a gola artificialmente por cima do cabelo.
- Garanta uma leitura perceptível da gola nas vistas em que ela deveria aparecer.

## 6. Preservar a arquitetura aprovada

Regras obrigatórias:

- O protagonista N3.5/N3.6 continua sendo a base visual vestida.
- O rig v007 continua invisível.
- Não criar corpo nu.
- Não modificar os sprites originais.
- Não substituir Rust/Bevy.
- Não retornar às máscaras estreitas da N41-B.
- Preservar o sistema de equipamentos modulares.
- Trabalhar em cópias dos arquivos Blender.
- Manter as quatro poses: idle, C1 #0, C1 #2 e C1 #4.
- Não modificar as etapas anteriormente aprovadas.

## 7. Realizar um teste preliminar de legibilidade

Sem executar o Prompt 04 completo, produza uma **prévia experimental de redução** para verificar a leitura das formas.

Use os métodos de escala e quantização documentados na N3.5 como referência, mas avalie também a necessidade de uma paleta complementar de roxo e dourado, pois a paleta N3.4 original não contempla adequadamente essas cores.

A prévia deve ajudar a responder:

- As ombreiras continuam parecendo armaduras?
- A quilha central permanece visível?
- Os detalhes dourados continuam legíveis?
- A silhueta externa continua marcante?
- O acabamento ainda parece plástico?
- A armadura combina artisticamente com o protagonista?

Essa redução é somente uma prova visual. Não integre o resultado ao jogo e não declare o Prompt 04 concluído.

## 8. Evidências obrigatórias

Gere imagens de comparação entre o modelo anterior e o modelo refinado:

- `model_sheet_before_after.png`
- `fit_summary_refined.png`
- `fit_idle_refined.png`
- `fit_run_c1_0_refined.png`
- `fit_run_c1_2_refined.png`
- `fit_run_c1_4_refined.png`
- `pixel_readability_test.png`

Salve em `evidence/erro_corretion/etapa03b/`.

Mostre também a armadura isolada para que possamos avaliar sua geometria sem interferência do personagem.

**Importante:** informe os caminhos exatos das imagens. Eu as enviarei ao ChatGPT para aprovação visual.

## 9. Commit e push obrigatórios

Ao concluir:

1. Confirme que está na branch `art/v1.2-protagonist`.
2. Confira o `git status`.
3. Execute as verificações pertinentes.
4. Faça commit somente dos arquivos relacionados à Etapa 03B, incluindo os fontes Blender refinados e as evidências necessárias para reproduzir o resultado, desde que não sejam arquivos temporários ou excessivamente grandes.
5. Faça push para a mesma branch remota, sem force-push.
6. Confirme o hash do commit e o estado final do repositório.

Não inclua arquivos pessoais de `Md-usario/`, alterações preexistentes ou arquivos não relacionados. Caso algum arquivo importante seja grande demais para o Git, informe como será preservado; não considere o trabalho reproduzível se o modelo refinado existir apenas em uma pasta temporária.

## 10. Condição para encerrar

Ao terminar, apresente:

- O que foi corrigido.
- O que ainda não está satisfatório.
- Comparação visual antes/depois.
- Resultado do teste preliminar de legibilidade.
- Caminhos das imagens.
- Commit e confirmação do push.

**Não execute o Prompt 04.**

Pare ao concluir a Etapa 03B e aguarde minha aprovação visual.