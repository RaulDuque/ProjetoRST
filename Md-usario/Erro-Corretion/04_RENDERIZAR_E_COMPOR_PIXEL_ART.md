# PROMPT 04 — RENDERIZAR EQUIPAMENTO, PRESERVAR PROTAGONISTA VESTIDO

**Pré-requisito:** aprovação explícita do Prompt 03. Leia `00_LEIA_PRIMEIRO.md` revisado.

**Objetivo:** produzir PNGs da **armadura isolada** em quatro poses e composições com o protagonista **já vestido**. O rig/corpo Blender fica invisível. O jogo continua exclusivamente Rust/Bevy.

1. Renderizar em alta resolução somente peitoral, gola, ombreiras e abas. Fundo transparente; câmera lateral ortográfica, pivôs e poses correspondentes a idle e C1 #0/#2/#4. Não renderizar o corpo nu nem roupas do rig.
2. Reduzir as peças renderizadas pelo processo documentado de N3.5/N3.6 (BOX, alfa pré-multiplicado, quantização e tratamento de contorno), registrando os parâmetros efetivos. A paleta N3.4 original pode precisar de rampas adicionais roxa/dourada: testar isso controladamente, mantendo a arte aprovada intacta.
3. Exportar equipamento em grade correta (idle 80×128, pivô 40,118; C1 112×128, pivô 56,118, salvo confirmação distinta no código). Manter versão de alta resolução para auditoria.
4. **Compor por OVERLAY sobre o sprite vestido.** Onde uma peça da armadura cobre uma camisa, aplicar HIDE por cobertura real, não por máscara estreita herdada; deixar manga/antebraço/mão/cabelo na frente quando apropriado. Não ocultar a camisa inteira se isso abrir buracos, nem usar nudez como preenchimento. A armadura deve poder se estender além da roupa original.
5. Validar pose e recorte: evitar flutuação, deslocamento, transparência irregular, cabelo perdido, armadura cortando braços ou falsa espessura. Comparar com as máscaras e ajustes da Etapa 02B.
6. Gerar quatro PNGs isolados, quatro PNGs vestidos, folha comparativa com personagem original e peitoral N41-B, e GIF curto da corrida C1. Separar erros de modelagem, alinhamento, oclusão e quantização.
7. Verificar a montagem **sem armadura**: original N3.5/N3.6 idêntico pixel a pixel. O equipamento não deve alterar o sprite padrão nem criar novas versões do corpo.

**Entregar imagens reais e resumo curto. Não integrar no Bevy nem modificar assets originais. Não instalar, commitar ou fazer push. Pare e peça autorização para o Prompt 05.**
