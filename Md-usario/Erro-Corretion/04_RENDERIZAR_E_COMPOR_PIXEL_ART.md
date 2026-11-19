# PROMPT 04 — DO BLENDER AO SPRITE 2D, SEM PERDER O ESTILO

Leia `00_LEIA_PRIMEIRO.md`. Use apenas o modelo do Prompt 03 aprovado, sem expandir escopo.

**Meta:** demonstrar o caminho real de produção: **Blender → PNG transparente em alta resolução → redução e paleta coerentes com N3.4/N3.5 → quatro quadros prontos para prévia 2D**. O jogo continua Rust/Bevy.

1. Recupere os parâmetros de redução comprovados no Prompt 01. Não trate como fato a existência de um script específico sem verificar. Se a técnica exata não estiver disponível, documente a aproximação e compare alternativas em amostra pequena.
2. Renderize a **armadura separada** do corpo, incluindo alpha e posicionamento/câmera consistentes. Guarde fontes em alta resolução para não pintar diretamente em 15×22 px.
3. Reduza para a grade dos sprites aprovados: idle 80×128, pivô (40,118), run C1 #0/#2/#4 112×128, pivô (56,118), **após conferir dados reais**.
4. Trate bordas, alpha, contorno, paleta, antialias e ruído. Não invente sombras que a geometria não gerou. Não suavize os pixels finais. Nenhuma armadura deve parecer uma faixa bidimensional.
5. Componha prévias com o personagem original **sem sobrescrever o PNG aprovado**. Oclusão é por frente/atrás dos braços; permita ombreiras e abas externas à máscara da camisa.
6. Separe diagnóstico: falha da geometria? da pose? da renderização? da redução? da oclusão? Não esconda falhas fazendo recolorações.
7. Exporte uma folha comparativa e um GIF/contato simples da corrida C1; mantenha arquivos de geração reproduzíveis.

**Saída:** 4 PNGs finais, previews vestido/isolado, comparativo com N41-B, um GIF curto, parâmetros reais e problemas ainda observados.

**Restrições:** NÃO integrar no runtime Bevy ainda; NÃO alterar o pintor da N41-B nem a arte aprovada; NÃO instalar/commitar/fazer push. **PARE para revisão do Prompt 05.**
