# DARK CASTLE RPG — MEGA PROMPT D13
## Blender MCP + Modelagem Procedural Assistida + Pipeline 2D Modular

Você é o engenheiro técnico e artista procedural responsável pela fase D13 do Dark Castle RPG.

O projeto utiliza Rust 2024, Bevy 0.20.1 e Blender 5.2.4.

O proprietário deseja investigar uma abordagem na qual scripts Python produzam e modifiquem o modelo 3D, o Claude Code controle o Blender por MCP e os resultados sejam utilizados para criar sprites 2D modulares.

A meta não é produzir um personagem genérico. É desenvolver uma representação 3D reutilizável do protagonista gótico já estabelecido nas fases D2–D12.

## 1. OBJETIVO

Criar um pipeline funcional e reproduzível:

1. Claude Code recebe especificações artísticas e técnicas.
2. Claude Code escreve ou adapta scripts Python `bpy`.
3. Blender MCP executa operações no Blender.
4. Blender produz um modelo modular e renders de referência.
5. Scripts locais convertem as referências em camadas ou máscaras.
6. O acabamento é comparado com D7-B/D12.
7. O Bevy experimental apresenta o resultado, se a qualidade justificar.

Não substitua o pipeline A da D12 automaticamente.

Não trate renderização 3D como equivalente a pixel art final.

## 2. SEGURANÇA E ESTADO DO PROJETO

Antes de instalar ou alterar qualquer coisa:

- Leia `CLAUDE.md`.
- Verifique `git status --short`.
- Confirme branch e HEAD.
- Identifique mudanças não commitadas.
- Preserve D1–D12.
- Preserve o arquivo Blender original.
- Preserve `Md-usario/`.
- Preserve o jogo principal.
- Preserve Save V3.
- Não faça commit ou push.

Crie:

`art-src/visual-prototypes/d13_blender_mcp/`

A instalação de MCP deve ser feita no ambiente de desenvolvimento do usuário, sem modificar arquivos de configuração versionados do projeto desnecessariamente.

Não exponha credenciais nem configure serviços externos de geração 3D.

## 3. INSTALAÇÃO DO BLENDER MCP

Utilize como referência:

https://github.com/ahujasid/mcp-for-blender

Primeiro verifique:

- Sistema operacional.
- Localização do executável Blender.
- Versão do Blender.
- Instalação do `uv` e `uvx`.
- Versão do Claude Code.
- MCPs já configurados.
- Conflitos com outras instâncias do Blender MCP.

Não instale duas versões concorrentes.

Prefira a configuração documentada pelo projeto:

`uvx mcp-for-blender setup`

Se necessário, utilize a configuração manual:

`claude mcp add blender uvx mcp-for-blender`

`uvx mcp-for-blender install-addon`

Confirme os comandos na documentação da versão utilizada antes de executá-los.

Se a instalação exigir privilégios administrativos, alterações globais inesperadas ou execução de scripts remotos não revisados, interrompa e peça autorização.

## 4. TESTE DE CONEXÃO

Após a instalação:

1. Abra o Blender.
2. Confirme o addon.
3. Confirme que o servidor está ativo.
4. Verifique que Claude Code consegue consultar a cena.
5. Liste objetos existentes.
6. Crie um cubo em uma cena temporária.
7. Altere seu material.
8. Renderize uma imagem.
9. Remova o cubo da cena temporária.
10. Confirme que nenhum projeto original foi alterado.

Não considere a instalação concluída apenas porque o comando retornou código zero.

Documente quais ferramentas MCP estão realmente disponíveis.

## 5. AUDITORIA DO MODELO EXISTENTE

Antes de criar novas malhas, inspecione o rig e o modelo do protagonista já existentes.

Identifique:

- Armature.
- Ossos.
- Meshes.
- Materiais.
- Modificadores.
- Weight painting.
- Animações.
- Câmeras.
- Equipamentos.
- Problemas de topologia.
- Objetos duplicados.
- Hierarquias inconsistentes.

Determine o que pode ser reutilizado.

**Não recrie um modelo inteiro se a estrutura atual puder ser refinada.**

Crie `AUDITORIA_MODELO.md`.

## 6. CONTRATO ARTÍSTICO

O protagonista deve preservar:

- Cabelo escuro.
- Rosto relativamente pálido.
- Aparência gótica aristocrática.
- Casaco vinho.
- Capa longa.
- Botas.
- Luvas.
- Espada.
- Bainha.
- Possibilidade de armadura pesada.
- Possibilidade de elmo e escudo.

Não altere a identidade facial ou a silhueta sem apresentar comparação.

Não copie personagens de Castlevania ou de outras franquias.

Utilize D7-B/D12 como referência visual principal.

## 7. ARQUITETURA DE MODELAGEM

Organize o modelo em objetos independentes.

### Corpo

- Head
- Hair
- Torso
- LeftArm
- RightArm
- LeftLeg
- RightLeg
- LeftHand
- RightHand

### Vestuário

- Shirt
- Coat
- Pants
- Gloves
- Boots
- Cape

### Armadura

- Cuirass
- PauldronLeft
- PauldronRight
- GauntletLeft
- GauntletRight
- Greaves
- Helmet

### Equipamentos

- Sword
- Scabbard
- Shield

Cada objeto deve ter:

- Nome estável.
- Transformações documentadas.
- Material independente quando necessário.
- Vinculação adequada ao rig.
- Identificador de slot visual.
- Possibilidade de ocultação.
- Possibilidade de substituição.

Não crie um personagem completo separado para cada loadout.

## 8. SCRIPTS PYTHON REUTILIZÁVEIS

Implemente scripts organizados por responsabilidade.

Sugestão:

- `inspect_scene.py`
- `validate_rig.py`
- `build_materials.py`
- `refine_body.py`
- `refine_coat.py`
- `refine_cape.py`
- `build_armor.py`
- `build_helmet.py`
- `build_sword.py`
- `build_shield.py`
- `configure_camera.py`
- `configure_lighting.py`
- `configure_render.py`
- `export_layers.py`
- `validate_exports.py`

Os scripts devem ser:

- Idempotentes quando possível.
- Determinísticos.
- Comentados.
- Compatíveis com a versão instalada.
- Capazes de produzir logs.
- Seguros para execução repetida.
- Limitados à cópia de laboratório.

Não utilize `bpy.ops` indiscriminadamente quando uma operação pela API de dados for mais segura.

Evite operadores dependentes de contexto quando possível.

Não execute código Python obtido de fontes externas sem revisão.

## 9. PROTÓTIPO DE MODELAGEM — PRIMEIRO MARCO

Não modele o personagem inteiro de uma vez.

Escolha uma pose idle e produza:

1. Corpo-base.
2. Casaco.
3. Capa.
4. Espada.
5. Uma peça de armadura.

Configure câmera ortográfica e iluminação.

Renderize o resultado.

Compare com D7-B/D12.

Se a qualidade for inferior, refine a modelagem e os materiais antes de expandir.

Não esconda problemas com filtros de pós-processamento.

## 10. MATERIAIS E ILUMINAÇÃO

Produza três estudos:

A. Materiais estilizados com iluminação suave.

B. Materiais estilizados com contraste gótico e rim light discreta.

C. Materiais com posterização controlada e realces metálicos.

Investigue a compatibilidade real do Blender 5.2.4.

Não presuma que nós de shader de um motor funcionem no outro.

Não utilize bloom como substituto de boa modelagem.

Não use contorno neon como solução para baixa legibilidade.

A espada precisa apresentar leitura metálica.

O cabelo e o rosto precisam se distinguir do fundo.

A capa deve apresentar volume.

A armadura deve parecer metal e não um conjunto de cubos.

## 11. RIG E ANIMAÇÃO

Depois da aprovação da pose estática:

- Teste idle.
- Teste corrida.
- Teste orientação.
- Teste capa.
- Teste espada.
- Teste armadura.
- Teste escudo.

Verifique deformações.

Evite clipping.

Não modifique as animações originais sem preservar uma cópia.

A animação deve permitir troca de equipamento sem reconstruir o corpo inteiro.

## 12. EXPORTAÇÃO 2D

Teste exportação por camada.

Cada frame deve manter:

- Mesma câmera.
- Mesma projeção.
- Mesma resolução.
- Mesmo pivô.
- Mesma escala.
- Coordenadas compatíveis.
- Fundo transparente.

Exporte separadamente corpo, vestuário e equipamentos.

Inclua máscaras de oclusão quando necessárias.

Não reduza automaticamente os renders para 128 × 160 sem medir a perda de detalhes.

Compare diferentes resoluções de trabalho e métodos de conversão.

## 13. COMPARAÇÃO COM PIXEL ART

Compare:

A. D12 Pipeline A.

B. Render Blender puro.

C. Blender com acabamento pixel art programático.

D. Blender com acabamento híbrido e correções manuais pontuais.

Avalie:

- Silhueta.
- Rosto.
- Cabelo.
- Espada.
- Capa.
- Armadura.
- Contraste.
- Densidade dos pixels.
- Qualidade dos contornos.
- Consistência entre frames.
- Modularidade.

Não declare vitória do Blender apenas porque a renderização tem mais detalhes.

O resultado precisa ser melhor na escala real do jogo.

## 14. TESTES DE MODULARIDADE

Teste:

- Remover capa.
- Trocar casaco.
- Trocar luvas.
- Trocar botas.
- Trocar espada.
- Equipar elmo.
- Equipar armadura.
- Equipar escudo.

Cada peça deve ser independente.

Não utilize imagens completas de loadout para simular modularidade.

Verifique que a animação continua funcionando depois da troca.

## 15. INTEGRAÇÃO BEVY

Não altere o jogo principal.

Se o protótipo atingir qualidade suficiente, utilize o exemplo experimental já existente para demonstrar o resultado.

Preserve:

- Escala de 64 unidades de mundo adotada na D12, salvo nova evidência técnica.
- Configuração de câmera do jogo.
- Filtragem apropriada.
- Pivôs.
- Sockets.
- Estado de animação.
- Visual padrão.

Não integre novas dependências ao jogo.

Não modifique gameplay.

## 16. EVIDÊNCIAS

Produza:

- Captura da conexão MCP funcionando.
- Cena Blender original auditada.
- Hierarquia do modelo.
- Rig.
- Corpo-base.
- Casaco.
- Capa.
- Espada.
- Armadura.
- Materiais A/B/C.
- Iluminação A/B/C.
- Comparação D12 vs Blender.
- Comparação de pixels.
- Testes de equipamentos.
- Idle e corrida, se aprovados.
- Resultado final na escala real.

Inclua arquivos `.blend` de laboratório quando apropriado, sem sobrescrever os originais.

Não gere dezenas de renders redundantes.

## 17. CRITÉRIOS DE ACEITAÇÃO

### MCP

- [ ] Instalado.
- [ ] Conectado.
- [ ] Ferramentas descobertas.
- [ ] Cena inspecionada.
- [ ] Python executado no Blender.
- [ ] Render produzido.
- [ ] Nenhum arquivo original danificado.

### Modelo

- [ ] Corpo-base reutilizável.
- [ ] Equipamentos independentes.
- [ ] Rig funcional.
- [ ] Materiais organizados.
- [ ] Proporções consistentes.
- [ ] Silhueta legível.

### Arte

- [ ] Identidade D7-B preservada.
- [ ] Espada legível.
- [ ] Rosto legível.
- [ ] Capa convincente.
- [ ] Armadura convincente.
- [ ] Pixels limpos.
- [ ] Comparação na escala real.

### Segurança

- [ ] Jogo padrão intacto.
- [ ] Save V3 intacto.
- [ ] Laboratórios anteriores intactos.
- [ ] Sem commit.
- [ ] Sem push.

Use estados concluído, parcial, bloqueado ou não iniciado.

## 18. PONTOS DE PARADA

**Ponto 1 — Instalação:** não prossiga se o MCP não estiver realmente conectado.

**Ponto 2 — Auditoria:** não recrie malhas antes de avaliar o modelo existente.

**Ponto 3 — Pose estática:** não produza todas as animações antes de comparar a qualidade artística.

**Ponto 4 — Modularidade:** não avance se os equipamentos não forem independentes.

**Ponto 5 — Pixel art:** não integre ao Bevy se o render perder a identidade D7-B.

**Ponto 6 — Aprovação:** não substitua o visual padrão sem autorização do proprietário.

## 19. RELATÓRIO FINAL

Informe:

A. Estado inicial do Git.

B. Instalação do Blender MCP.

C. Versões utilizadas.

D. Ferramentas MCP disponíveis.

E. Resultado da conexão.

F. Auditoria do modelo original.

G. Scripts criados.

H. Modelagem realizada.

I. Materiais.

J. Iluminação.

K. Rig.

L. Equipamentos.

M. Exportação 2D.

N. Comparação artística.

O. Testes de modularidade.

P. Evidências.

Q. Limitações.

R. Estado final do Git.

S. Recomendação para D14.

Não faça commit.

Não faça push.

Aguarde aprovação.

## 20. DIRETRIZ FINAL

O objetivo é criar uma ferramenta de produção de arte reutilizável.

Não queremos depender de gerar um personagem inteiro novamente para cada equipamento.

Queremos poder escrever scripts, controlar o Blender, ajustar malhas, produzir equipamentos e gerar referências para sprites consistentes.

**O sucesso da D13 será ter um pipeline de modelagem controlável e um protótipo visual convincente — não simplesmente instalar o MCP.**