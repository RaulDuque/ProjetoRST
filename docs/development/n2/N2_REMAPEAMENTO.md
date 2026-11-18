# N2 — Remapeamento

A tela é `GameState::Controls`, desenhada com o painel e as cores que as outras telas já usam.

Linhas: uma por ação, depois Aplicar, preset Híbrido, preset Clássico, Restaurar e Fechar. Esquerda e direita escolhem o slot principal ou o alternativo. Enter numa ação começa a captura. Backspace limpa o slot. Enter num rodapé executa aquela linha.

Durante a captura o jogo não anda, não ataca e não abre outra tela, porque a simulação já está parada e este sistema consome o próximo controle em vez de deixar a ação passar. Escape cancela a captura e devolve o slot ao que era. Escape sem captura fecha e joga o rascunho fora. Aplicar grava e passa a usar o rascunho. Restaurar copia de volta o perfil que estava ativo quando a tela abriu.

Política de conflito, uma só: recusar. Se outra ação do mesmo contexto já usa o controle, a mensagem diz qual é e o slot não muda. Não há troca silenciosa. O mesmo controle nos dois slots da mesma ação também é recusado.

Enter é contexto de menu (`Confirm`). Pode coincidir com uma ação de jogo. D não pode andar e interagir ao mesmo tempo.

Ação essencial (andar, mirar, pular, atacar, interagir, confirmar) não fica com os dois slots vazios. A tentativa de limpar o último é recusada.

Escape não é capturável. Tecla fora da lista (por exemplo um caractere que o enum não tem) também não. Botões do mouse aceitos: esquerdo, direito e do meio.

As teclas 1–4 da tela de status não fazem parte do perfil. Continuam alocando atributo.
