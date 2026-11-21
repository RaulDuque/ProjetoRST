Prompt 03D — Take 02: processar a imagem do ChatGPT e avaliar o resultado
A imagem de entrada já foi colocada em:

evidence/erro_corretion/etapa03d/inbox/armour_master.png

Ela deve ser tratada como a imagem mestre do Take 02.
Objetivo
Usar o fluxo oficial já preparado na Etapa 03D para:
1. ler a imagem mestre com o pixel_look pose;
2. converter o resultado para o canvas do jogo (80×128, pivô 40,118);
3. medir a compatibilidade com o sprite N3.5 aprovado;
4. restaurar o cabelo usando as camadas aprovadas da N4, sem redesenhar nada;
5. comparar Take 01 vs Take 02;
6. gerar as imagens e o relatório de avaliação;
7. fazer commit e push ao final, se tudo terminar corretamente.
Regras obrigatórias
1. Não redesenhar nada à mão.
2. Não corrigir manualmente pixels da armadura, do rosto, do cabelo, da calça ou das botas.
3. Não gerar nova imagem por código, IA local, Blender, PixelFX ou qualquer substituto.
4. Usar somente a imagem já colocada em inbox/armour_master.png.
5. Não alterar Rust, Bevy, assets/, sprites aprovados, Blender original, N3.5, N3.6, N3.8, N4, Etapa 02B ou etapas anteriores.
6. Não gerar animações novas.
7. Não usar chamadas pagas nem credenciais externas.
8. Se algo falhar, parar e relatar com clareza, sem inventar resultado.
Entrada
Imagem de entrada
- evidence/erro_corretion/etapa03d/inbox/armour_master.png
Referência aprovada
- usar o mesmo protagonista N3.5 aprovado que já vinha sendo usado no Take 01.
Fluxo preparado
Seguir o procedimento já descrito em:
- evidence/erro_corretion/etapa03d/inbox/LEIA.md
O que fazer
1. Rodar a leitura oficial do Take 02
Executar o fluxo oficial do pixel_look pose sobre armour_master.png.

Objetivo:
- detectar a grade automaticamente;
- extrair apenas a alteração do tronco/armadura;
- preservar a posição original do personagem no canvas do jogo.
2. Gerar o sprite final no canvas do jogo
Montar o resultado final no canvas:
- 80×128
- pivô 40,118
Sem deslocar o personagem.
3. Calcular as métricas por região
Medir e reportar, no mínimo:
- cabeça: pixels idênticos ao aprovado;
- rosto: pixels idênticos ao aprovado;
- cabelo: pixels idênticos ao aprovado;
- pernas (calça + botas): pixels idênticos ao aprovado;
- tronco: pixels alterados;
- total de pixels alterados;
- silhueta: pixels ganhos e perdidos;
- transparência;
- total de cores e novas cores introduzidas;
- contorno: se continua fechado e se mantém a cor de contorno correta.
4. Restaurar o cabelo
Aplicar a mesma estratégia aprovada no Take 01:
- restaurar o cabelo somente usando as camadas aprovadas da N4 (hair_back e hair_front);
- sem redesenhar nada;
- manter a ordem de camadas já aprovada.
Depois disso:
- recalcular a diferença do cabelo;
- confirmar se o cabelo volta a 0 pixels diferentes ou relatar o valor real.
5. Comparar Take 01 vs Take 02
Gerar comparação visual e métrica entre:
- resultado final do Take 01;
- resultado final do Take 02;
- versão com restauração de cabelo do Take 02.
Mostrar, no mínimo:
- 1×
- 2×
- 8× no tronco
- prévia sobre fundo escuro
6. Avaliação visual
Responder com base nas imagens geradas:
- a armadura do Take 02 ficou mais legível que a do Take 01?
- a quilha central ficou mais visível?
- as faixas douradas realmente seguram largura mínima útil na resolução do jogo?
- as ombreiras leem como placas planas?
- houve regressão em cabelo, rosto, calça ou botas?
- o resultado está pronto para aprovação visual ou ainda pede novo take?
Arquivos de saída
Salvar tudo em:

evidence/erro_corretion/etapa03d/runs/take02/

Organizar de forma parecida com o Take 01.

Gerar pelo menos estes arquivos:
- RELATORIO_take02.md
- take02_comparison_sheet.png
- take02_torso_source_vs_1x.png
- armour_on_canvas_80x128.png
- takes_01_vs_02_comparison.png
- takes_dark_preview.png
Se fizer sentido, criar subpastas como:
- hair_restore/
- compare_check/
- pose/
Mas sem bagunçar a estrutura.
Conteúdo mínimo do relatório
O relatório deve informar:
1. se a leitura da grade funcionou;
2. se a posição do personagem foi preservada;
3. quantos pixels ficaram idênticos por região;
4. quantos pixels o cabelo precisou restaurar;
5. se o Take 02 melhorou em relação ao Take 01;
6. defeitos visuais observados;
7. conclusão objetiva:
   - aprovável para seguir,
   - melhor que o Take 01, mas ainda insuficiente,
   - ou rejeitado.
Git
Ao final, se tudo tiver sido executado corretamente:
1. conferir git status;
2. criar commit apenas com os arquivos desta etapa;
3. fazer push para a branch atual, sem force;
4. informar:
   - branch;
   - hash do commit;
   - se local e remoto ficaram sincronizados.
Se houver falha séria no processamento, não inventar sucesso. Nesse caso, relatar o problema e só commitar se houver evidências úteis reais dessa tentativa.
Restrições finais
- Não alterar nada fora do escopo desta etapa.
- Não mexer em Md-usario/.
- Não iniciar o Prompt 04.
- Parar depois de entregar o relatório, as imagens e o estado do Git.
