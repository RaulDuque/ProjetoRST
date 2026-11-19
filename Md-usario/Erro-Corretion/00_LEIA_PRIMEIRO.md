# DARK CASTLE RPG — ERRO-CORRETION

## Finalidade
Corrigir a **qualidade artística** dos equipamentos sem desperdiçar a arquitetura funcional do Dark Castle RPG (Rust + Bevy). Não se trata de trocar a engine nem de gerar mais 40 armaduras ruins. O primeiro objetivo é provar, com imagens reais, que **uma armadura 3D modular pode ser renderizada para pixel art 2D** no mesmo estilo do protagonista N3.5/N3.6.

## Diagnóstico confirmado pelo Claude Code (19/11/2026)
- Blender 5.2.4 instalado, com cerca de 10 arquivos `.blend`, rig animado **v007** e histórico da V1.x/V2.2. O rig ainda **não foi validado** contra as poses N3.5/N3.6.
- A N41-B gera pintura de regras no tamanho final de aproximadamente 15×22 px, restrita à máscara da camisa; não cria silhueta 3D convincente. No idle, manga e camisa dividem a camada.
- A arte N3.5/N3.6 aprovada veio de imagem/vídeo em resolução maior reduzidos, com ajustes artísticos. A ideia é **testar** renderização do Blender em resolução maior + redução no mesmo padrão, não presumir que isso basta.
- Python + Pillow são ferramentas **offline** de processamento de PNG; **o jogo continua 100% Rust/Bevy**. Godot 4 não faz parte do jogo.
- N41-B tem valor como **compositor, contratos, cache, validação e integração**. Não descartá-la nem voltar a refinar o pintor procedural antes do teste visual.

## Ordem obrigatória (um prompt por vez)
1. `01_VERIFICAR_RIG_E_REFERENCIAS.md` — encontra rig/arquivos e define limites reais.
2. `02_TESTAR_ENCAIXE_DAS_POSES.md` — prova o alinhamento do rig às 4 poses, antes de modelar armaduras.
3. `03_MODELAR_ARMADURA_PILOTO.md` — apenas **uma** armadura roxa e dourada, com volume real.
4. `04_RENDERIZAR_E_COMPOR_PIXEL_ART.md` — gera 4 quadros e prévias 2D, sem mexer no jogo.
5. `05_GATE_DE_APROVACAO_VISUAL.md` — comparação lado a lado; **o usuário decide** se a arte está boa.
6. `06_INTEGRAR_COM_N41B.md` — SOMENTE após aprovação explícita; reaproveitar compositor Bevy.
7. `07_ESCALAR_EQUIPAMENTOS_MODULARES.md` — SOMENTE após integração aprovada; prova itens diferentes e variações.
8. `08_PLANO_B_SE_RIG_FALHAR.md` — alternativa a ser usada apenas se 01/02 demonstrarem impedimento real.

## Regras globais para TODOS os prompts
- Leia `CLAUDE.md` e a documentação relevante antes de agir. Confirme caminhos em disco; não invente arquivos nem comandos.
- Branch de referência: `art/v1.2-protagonist`. **Não trocar branch, commitar, dar push, merge, rebase ou force-push** sem pedido explícito do proprietário.
- Preservar V2.2, N3.5/N3.6, N3.8, N4, save v3, física, combate, câmera e arte aprovada; não tocar arquivos pessoais fora desta tarefa.
- NÃO instalar ferramentas ou trocar Rust/Bevy por Godot. Blender produz **assets offline**, não substitui a engine.
- Oculsão/oclusores: braços devem poder aparecer à frente de certas peças; ombreiras e peitorais **podem ultrapassar o contorno da camisa**. Não tentar resolver volume real recolorindo a camisa.
- Trabalho limitado a `idle` (80×128, pivô 40,118) e frames C1 da corrida (#0, #2, #4; 112×128, pivô 56,118), **confirmar esses dados nos arquivos reais**.
- Sempre apresentar evidência visual (PNG e, quando couber, GIF) e resultado reproduzível. Teste automatizado não substitui aprovação artística.
- Se uma etapa falhar, **parar e explicar**. Não mascarar o problema com nova fábrica, novas dezenas de itens ou centenas de linhas de código.
- Não afirmar compatibilidade com runtime Bevy se testou apenas imagens isoladas. Armas/capas/ombreiras da N41-B podem estar **apenas isoladas**, não integradas.
- Guardar resultados de testes em área temporária ou `evidence/erro_corretion/`, sem modificar arte aprovada. Registrar caminhos e parâmetros precisos.

## Critério de sucesso final
O proprietário vê um sprite **claramente equipado** com nova silhueta, volume, detalhes e movimento coerente, e aprova visualmente antes que se expanda para novos itens.

## Como começar no Claude Code
> Leia `Md-usario/Erro-Corretion/00_LEIA_PRIMEIRO.md` e execute somente `Md-usario/Erro-Corretion/01_VERIFICAR_RIG_E_REFERENCIAS.md`. Não avance para as próximas etapas até eu aprovar.
