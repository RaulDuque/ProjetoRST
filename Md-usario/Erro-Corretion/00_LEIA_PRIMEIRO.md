# DARK CASTLE RPG — ERRO-CORRETION (REVISÃO APÓS ETAPA 02)

## Regra visual definitiva, aprovada pelo proprietário
**O protagonista aprovado N3.5/N3.6, já com cabelo, camisa, calça e botas, É A BASE VISUAL PADRÃO.** Não gerar, exibir, reconstruir ou utilizar um corpo nu como base visual da armadura. O corpo nu da antiga máquina de sprites não é requisito para equipamentos externos.

O rig v007 do Blender é SOMENTE um **manequim técnico invisível**: referência de ossos, ângulos, ancoragens e volume para modelar/renderizar peças independentes. **Não renderizar seu corpo/casaca/capa/sabre**, nem trocar o protagonista aprovado por um boneco 3D.

Uma armadura é uma **peça sobreposta ao personagem vestido**: cobre visualmente as áreas que ocupa, pode ocultar os texels da camisa que ficariam por baixo e pode ultrapassar a silhueta antiga (ombros, gola, placas, abas). Braços, mangas, mãos e cabelo que estejam na frente devem continuar visíveis segundo a pose. **Não confinar a armadura à máscara da camisa.** Não acrescentar nudez nem pele inventada nas regiões encobertas. Item não equipado deve conservar o original exato.

Equipamentos que trocam roupas (nova calça, botas, camisa) podem exigir **HIDE/REPLACE da peça visível**, sempre a partir da base aprovada, e não uma reconstrução completa de anatomia. Não confundir isso com armaduras externas OVERLAY.

## Situação atual — etapas 01 e 02 já concluídas
- Etapa 01: Blender 5.2.4 disponível, rig `swordsman_rig_v007` reaproveitável **com ajustes**; ele tem 43 ossos e serve como guia, não como sprite final.
- Etapa 02: quatro poses ajustadas em cópia de trabalho (idle e C1 #0/#2/#4). Resultado **PRECISA DE AJUSTE**, principalmente ombros/clavículas da corrida e articulações das pernas. A precisão para torso/ombreiras é promissora, ainda não aprovada como arte final.
- Defeitos de composição: no idle, manga integrada à camada `shirt`; em C1 #0/#2 há pixels de cabelo confundidos com camadas do braço. A armadura precisa de máscaras de oclusão corretas.
- Evidências e arquivos locais da Etapa 02 constam em `evidence/erro_corretion/etapa02/`; leia os caminhos reais e o relatório. A pasta `evidence/` pode estar não versionada; **não apagar nem sobrescrever**.
- N41-B possui infraestrutura útil (RON, compositor Rust/Bevy, `VisualPlayback`, validação, cache), mas seu pintor procedural diretamente na máscara estreita da camisa está reprovado visualmente. Blender será usado somente na produção offline, se o piloto comprovar qualidade. Python+Pillow servem só para pós-processar PNG; o jogo continua Rust+Bevy.

## Nova ordem de execução (um prompt por vez)
1. `01_VERIFICAR_RIG_E_REFERENCIAS.md` — **CONCLUÍDO, não repetir**.
2. `02_TESTAR_ENCAIXE_DAS_POSES.md` — **CONCLUÍDO, não repetir**.
3. `02B_AJUSTAR_MANEQUIM_E_CAMADAS.md` — **PRÓXIMO**: corrigir ombros, recortes e composição do personagem vestido, sem criar armadura.
4. `03_MODELAR_ARMADURA_PILOTO.md` — UMA armadura de tronco + ombros no Blender, com o sprite vestido como base, não um corpo nu.
5. `04_RENDERIZAR_E_COMPOR_PIXEL_ART.md` — quatro quadros de armadura independente + prévia vestido.
6. `05_GATE_DE_APROVACAO_VISUAL.md` — proprietário aprova/reprova a qualidade real.
7. `06_INTEGRAR_COM_N41B.md` — só após aprovação visual explícita.
8. `07_ESCALAR_EQUIPAMENTOS_MODULARES.md` — provar apenas um segundo modelo e uma variante.
9. `08_PLANO_B_SE_RIG_FALHAR.md` — contingência se 02B/03/04 falharem com evidências, sem voltar ao pintor reprovado.

## Regras de segurança para TODAS as etapas
- Ler `CLAUDE.md`, documentação do projeto e a evidência das etapas já executadas. Verificar existência dos caminhos; não inventar arquivos nem resultados.
- Branch de referência `art/v1.2-protagonist`; **sem commit, push, merge, rebase, force-push ou mudança de branch**, salvo autorização explícita do proprietário.
- Preservar V2.2, arte N3.5/N3.6, N3.8, N4, save v3, física, câmera, combate, animação e todos os sprites aprovados. Não sobrescrever os PNGs originais.
- Não instalar nada, nem trocar engine. Não modificar arquivos pessoais da pasta `Md-usario/`, exceto quando o proprietário pedir explicitamente uma revisão dos prompts. Não executar próximas etapas automaticamente.
- Limitar prova atual a idle (80×128; pivô 40,118) e corrida C1 #0/#2/#4 (112×128; pivô 56,118), confirmando parâmetros no projeto.
- Trabalho em **cópias de teste** e evidências em `evidence/erro_corretion/` ou local temporário. Manter a obra aprovada intacta e comparar hashes quando pertinente.
- Gate visual é indispensável. Métricas, testes e ausência de erros não significam aprovação artística.
- Não ampliar para pernas, botas, espadas, capas, inventário completo ou dezenas de armaduras antes de passar no piloto de tronco/ombros.
- Respeitar diferenças de profundidade: cabelo, braço próximo, manga, peitoral, braço distante e ombreiras exigem recortes/ordem específicos por pose. Não remover cabelo/pele nem criar buracos ao ocultar a camisa.

## Próxima mensagem para o Claude
> Leia `Md-usario/Erro-Corretion/00_LEIA_PRIMEIRO.md` revisado e execute **somente** `Md-usario/Erro-Corretion/02B_AJUSTAR_MANEQUIM_E_CAMADAS.md`. As etapas 01 e 02 já estão concluídas; não as refaça. O protagonista aprovado **vestido** é a base visual, e o rig v007 é apenas um guia invisível. Pare e mostre evidências antes do Prompt 03.
