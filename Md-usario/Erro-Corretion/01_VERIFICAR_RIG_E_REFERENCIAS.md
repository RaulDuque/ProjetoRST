# PROMPT 01 — VERIFICAR RIG, REFERÊNCIAS E REAPROVEITAMENTO

Você está no projeto **Dark Castle RPG**. Leia `Md-usario/Erro-Corretion/00_LEIA_PRIMEIRO.md` antes de começar.

**Objetivo:** determinar se o Blender rig v007 e os arquivos da V1.x/V2.2 realmente podem servir como manequim técnico para a arte aprovada N3.5/N3.6. NÃO modele nada ainda.

Faça somente:
1. Confirme OS/ambiente real, Blender instalado e versão, caminhos de todos os `.blend` relevantes e scripts existentes de render/rig/armas. Diferencie achado factual, hipótese e arquivo antigo sem uso.
2. Inspecione em modo seguro o rig v007: ossos, hierarquia, controles, animações, transformações, eixos, câmera ortográfica, materiais e âncoras. Se houver dependências externas ausentes, registre; **não instale nada**.
3. Localize os arquivos aprovados N3.5 idle e N3.6 run6, paleta N3.4 e scripts efetivamente usados na redução/recorte. Identifique imagem-fonte, scale, filtro, quantização, posicionamento, pivôs, contorno e retoques — ou registre o que não pode ser recuperado.
4. Localize `run_poses.py`, os dados C1 #0/#2/#4, máscaras, layers do idle e a arquitetura N41-B. Confirme dimensões e pivôs reais.
5. Compare a pose, proporção, direção, articulações, volume e projeção do rig com o sprite aprovado: o rig pode servir como **guia para equipamentos**, mesmo que o corpo dele não seja renderizado? Quais ajustes seriam necessários?
6. Verifique precisamente por que a camisa e manga estão unidas no idle e se existe informação de oclusão/layer que permita colocar braços corretamente à frente/atrás da armadura.

**Entregue no chat:** tabela com arquivos e caminhos, estado do rig (REAPROVEITÁVEL / REAPROVEITÁVEL COM AJUSTES / INADEQUADO), evidência objetiva, riscos e os parâmetros confirmados da arte. Se não conseguir abrir o `.blend`, não invente conclusões.

**Proibições:** nenhum código, asset, arquivo de projeto, commit, push, instalação ou render de nova armadura. Só inspeção. **PARE ao concluir**; peça autorização para o Prompt 02.
