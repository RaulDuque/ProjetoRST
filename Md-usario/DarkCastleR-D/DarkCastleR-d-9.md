# DARK CASTLE RPG — MEGA PROMPT D9
## Modularidade Real, Reconstrução de Camadas e Refinamento Artístico do Protagonista

**Projeto:** Dark Castle RPG  
**Engine:** Rust 2024 + Bevy 0.20.1  
**Fase:** D9 — Modularidade Real e Refinamento Artístico  
**Referência visual principal:** D7-B / D8 clássico  
**Referência de armadura:** D8 pesado  
**Pipeline:** arte-fonte 2D + máscaras + atlas + sockets + composição programática  
**Blender:** somente ferramenta auxiliar  
**Escopo:** laboratório artístico isolado, sem integração ao jogo principal

---

# 1. CONTEXTO E DECISÃO DO PROPRIETÁRIO

Estamos desenvolvendo um RPG 2D Metroidvania de fantasia gótica inspirado em aspectos de jogos como Castlevania: Symphony of the Night.

O protagonista terá equipamentos visuais intercambiáveis, diversas animações, armas, magias e transformações.

As fases D1–D8 investigaram diferentes pipelines artísticos.

Resultados importantes:

- D2 estabeleceu a identidade artística detalhada do protagonista.
- D3 foi rejeitada por aparência geométrica.
- D4 iniciou a separação de equipamentos.
- D5 demonstrou corrida modular, mas manteve aparência antiga do Blender.
- D6 melhorou o protótipo técnico de armaduras, mas não alcançou o acabamento D2.
- D7 demonstrou que sprites 2D e máscaras preservam melhor a aparência desejada.
- D7-B foi escolhido pelo proprietário.
- D8 consolidou atlas experimental, sockets, loadouts e animações.
- O proprietário gostou da apresentação final da D8, especialmente do protagonista clássico e da direção visual do cavaleiro pesado.

**A decisão atual é preservar a aparência D7-B/D8 e melhorar sua modularidade real.**

Não queremos criar uma nova identidade visual.

Não queremos voltar ao acabamento EEVEE/toon.

Não queremos repetir a produção de centenas de arquivos que apenas demonstram o mesmo problema.

A D9 deve resolver os problemas técnicos e artísticos que impedem a utilização do pipeline em produção.

---

# 2. MISSÃO PRINCIPAL

Sua missão é transformar o protótipo D8 em um sistema de personagem 2D modular, mantendo a qualidade visual do protagonista aprovado.

Os objetivos são:

1. Separar verdadeiramente as peças do protagonista.
2. Reduzir a dependência do sprite completo `full_classic_ref`.
3. Preservar a identidade visual D7-B.
4. Corrigir máscaras e transparências.
5. Refinar o atlas.
6. Calibrar os sockets por quadro.
7. Criar regras consistentes de sobreposição.
8. Melhorar o encaixe dos equipamentos.
9. Aproximar a armadura pesada do acabamento D2.
10. Melhorar a animação das peças durante idle e corrida.
11. Demonstrar trocas individuais sem reconstruir o personagem completo.
12. Preparar uma arquitetura reutilizável para futuros equipamentos.

**Não integre o resultado ao Bevy nesta fase.**

A integração só poderá acontecer após aprovação visual e técnica.

---

# 3. REGRAS DE SEGURANÇA

Antes de qualquer trabalho:

- Leia `CLAUDE.md`.
- Leia as regras do projeto.
- Confirme branch, HEAD e upstream.
- Execute `git status --short`.
- Identifique arquivos modificados e não rastreados.
- Verifique se D7 e D8 estão versionadas.
- Preserve todos os arquivos existentes.

A branch esperada é:

`art/v1.2-protagonist`

Não presuma um hash específico.

Não modifique:

- `src/`
- `assets/`
- `saves/`
- `Md-usario/`
- `.cargo/`
- `.claude/`
- Laboratórios D1–D8.
- Arquivos Blender originais.

Não execute comandos destrutivos de Git.

Não faça commit.

Não faça push.

Não utilize `git add .` nem `git add -A`.

Todo trabalho novo deve ficar em:

`art-src/visual-prototypes/d9_modularidade_real/`

Se precisar utilizar fontes de D2, D4, D7 ou D8, faça referências ou cópias controladas.

Não sobrescreva os originais.

---

# 4. AUDITORIA OBRIGATÓRIA DA D8

Antes de implementar qualquer solução, estude os arquivos reais.

Leia:

- `d8_atlas_modular_2d/README.md`
- `d8_atlas_modular_2d/AUDITORIA_D7.md`
- `d8_atlas_modular_2d/CONTRATO_ATLAS.md`
- `d8_atlas_modular_2d/CONTRATO_SOCKETS.md`
- `d8_atlas_modular_2d/ARQUITETURA_MODULAR.md`
- `d8_atlas_modular_2d/REGRAS_OCLUSAO.md`
- `d8_atlas_modular_2d/COMPARACAO_VISUAL.md`
- `d8_atlas_modular_2d/LIMITACOES.md`
- `d8_atlas_modular_2d/RELATORIO_FINAL.md`
- `d8_atlas_modular_2d/DECISAO.md`

Inspecione:

- `comparacoes/12_resultado_final.png`
- `comparacoes/01_D7_vs_D8.png`
- `comparacoes/05_loadout_classico.png`
- `comparacoes/06_loadout_pesado.png`
- `comparacoes/07_troca_individual.png`
- `comparacoes/11_causeway_escala_real.png`
- `previews/run_classico.gif`
- `previews/run_pesado.gif`
- `previews/idle_classico.gif`
- `previews/idle_pesado.gif`

Examine também os scripts de geração.

Não se limite aos relatórios.

Verifique o comportamento real do código.

---

# 5. PROBLEMAS CONHECIDOS QUE DEVEM SER INVESTIGADOS

A D8 registrou os seguintes problemas:

## 5.1 Dependência do sprite completo

O protagonista clássico utiliza `full_classic_ref` para preservar sua aparência.

Isso é aceitável como referência artística, mas impede a modularidade completa quando peças precisam ser removidas ou substituídas.

## 5.2 Camadas incompletas

Algumas camadas estão vazias ou não foram separadas adequadamente:

- `coat_back`
- `cape_front`
- `hair_back`

Uma camada vazia pode ser legítima em determinada pose. Não a preencha artificialmente se a peça realmente não for visível.

## 5.3 Luvas e bainha

Os recortes ainda são fracos e algumas trocas utilizam proxies.

## 5.4 Armadura pesada

O acabamento das peças D4 não corresponde totalmente ao protagonista clássico D2.

## 5.5 Movimento da armadura

Na corrida, algumas peças seguem sockets por translação, sem deformação adequada.

## 5.6 Movimento da capa

O método A foi escolhido por preservar melhor a arte D2.

Os métodos B e C apresentaram limitações de deformação ou movimento.

Sua primeira tarefa é confirmar esses problemas e identificar suas causas.

Não implemente soluções antes de compreender a estrutura atual.

Crie `AUDITORIA_D8.md`.

---

# 6. PRINCÍPIO FUNDAMENTAL DA D9

**Modularidade não pode significar perda de qualidade artística.**

Se o sprite completo D2 apresenta um casaco detalhado, a versão modular deve preservar esse nível de detalhe.

Não é aceitável substituir:

- Casaco detalhado por um polígono simples.
- Cabelo por um bloco escuro.
- Luvas por manchas de cor.
- Botas por retângulos.
- Armadura por placas cúbicas.
- Capa por um triângulo sem acabamento.

O objetivo é reconstruir ou separar as peças com fidelidade.

Quando a separação automática não funcionar, utilize uma solução artística específica.

Documente claramente quais partes foram separadas automaticamente e quais exigiram reconstrução.

---

# 7. MARCO A — MAPA DE DEPENDÊNCIAS VISUAIS

Antes de criar novas imagens, identifique a composição real do protagonista.

Para cada peça, registre:

- Nome.
- Arquivo-fonte.
- Frames em que aparece.
- Pixels ocupados.
- Relação com outras peças.
- Máscaras.
- Pivôs.
- Sockets.
- Ordem de desenho.
- Possibilidade de remoção.
- Possibilidade de substituição.
- Problemas conhecidos.

Crie uma matriz de dependências.

Exemplo:

| Peça | Independência desejada | Problema |
|---|---|---|
| Cabelo | Trocar ou ocultar | Sobreposição com rosto |
| Casaco | Remover e substituir | Pixels misturados ao tronco |
| Capa | Remover e substituir | Mistura com silhueta |
| Luvas | Trocar | Recortes incompletos |
| Botas | Trocar | Pixels compartilhados com pernas |
| Espada | Trocar | Sobreposição com mão |
| Bainha | Trocar | Contornos misturados ao casaco |
| Armadura | Equipar | Shading diferente |

Não declare uma peça independente se ela não puder ser removida sem deixar resíduos.

---

# 8. MARCO B — RECONSTRUÇÃO DAS CAMADAS

Crie um atlas de trabalho a partir das fontes D2/D7-B/D8.

O objetivo é separar:

```text
body_base
head
face
hair_back
hair_front

torso_base
shirt
coat_back
coat_front

arm_back
arm_front
glove_back
glove_front

leg_back
leg_front
boot_back
boot_front

cape_back
cape_front

weapon
scabbard

armor_chest
armor_shoulder_back
armor_shoulder_front
armor_forearm_back
armor_forearm_front
armor_legs
armor_boots
helmet
shield
```

A estrutura poderá ser adaptada à anatomia real da arte.

Não crie arquivos vazios apenas para preencher nomes.

Cada camada precisa ter uma função visual.

## 8.1 Separação por máscaras

Utilize:

- Máscaras alfa.
- Seleção por regiões.
- Seleção por conectividade.
- Recortes semânticos.
- Correções de contorno.
- Remoção controlada de chroma.
- Reconstrução de pixels ocultos.

Não dependa exclusivamente de seleção por cor.

Peças diferentes podem compartilhar a mesma paleta.

## 8.2 Reconstrução de regiões ocultas

Quando remover o casaco, o corpo por baixo não pode ficar com buracos.

Crie arte-fonte adequada para o corpo-base.

Não tente reconstruir automaticamente áreas ocultas e apresentar o resultado como artisticamente correto sem inspeção.

## 8.3 Separação da espada

A espada deve ser independente da mão.

A mão deve continuar visível quando a espada for removida.

## 8.4 Separação da capa

A capa deve ser removível sem apagar o corpo.

Não deixe pixels vinho residuais quando a capa estiver desativada.

---

# 9. TESTE OBRIGATÓRIO — DESMONTAGEM COMPLETA

Crie uma prancha mostrando o protagonista sendo desmontado por camadas.

Sequência:

1. Personagem completo.
2. Sem espada.
3. Sem bainha.
4. Sem capa.
5. Sem casaco.
6. Sem luvas.
7. Sem botas.
8. Corpo-base com roupas mínimas.
9. Todas as peças separadas.

O corpo-base deve permanecer coerente.

Não é necessário criar nudez ou anatomia detalhada; utilize uma roupa-base simples e apropriada.

**Se a desmontagem produzir buracos ou resíduos, a peça ainda não está modularizada corretamente.**

Crie testes para detectar pixels residuais quando possível.

A inspeção visual continua obrigatória.

---

# 10. MARCO C — ATLAS E MANIFESTO

Consolide um atlas experimental com:

- Dimensões consistentes.
- Transparência RGBA.
- Pivô de sola.
- Pivôs por peça.
- Metadados por frame.
- IDs estáveis.
- Máscaras.
- Regras de visibilidade.
- Ordenação.
- Fonte de cada sprite.

A D8 utilizou como referência:

- Célula: 128 × 160 pixels.
- Pivô de sola: (64, 156).
- Altura visual de laboratório: aproximadamente 110 pixels.
- Altura visual no jogo: aproximadamente 52 pixels.

Confirme esses valores nos arquivos reais.

Não altere a escala arbitrariamente.

Se for necessário mudar o contrato, apresente justificativa e comparação.

Crie `CONTRATO_ATLAS_D9.md`.

---

# 11. MARCO D — SOCKETS PRECISOS POR QUADRO

A D8 criou sockets para:

- Idle: 6 quadros.
- Corrida: 8 quadros.

Agora refine os pontos de ancoragem.

Sockets importantes:

- Cabeça.
- Pescoço.
- Ombros.
- Cotovelos.
- Punhos.
- Mãos.
- Quadril.
- Joelhos.
- Tornozelos.
- Pés.
- Costas.
- Bainha.
- Arma principal.
- Escudo.

Cada socket deve possuir:

- Posição por quadro.
- Espaço de coordenadas.
- Referencial.
- Pivô.
- Orientação.
- Espelhamento.
- Regras de profundidade.

Não derive todos os sockets apenas da bounding box global do personagem.

Calibre-os de acordo com a anatomia real de cada quadro.

Crie overlays de depuração.

---

# 12. MARCO E — ARMADURA PESADA COM ACABAMENTO COMPATÍVEL

A armadura pesada D8 possui uma silhueta interessante.

**Preserve seu design geral.**

Não redesenhe o cavaleiro pesado como outro personagem.

Refine:

- Elmo.
- Peitoral.
- Ombreiras.
- Braçadeiras.
- Manoplas.
- Proteções das pernas.
- Botas.
- Escudo.
- Detalhes metálicos.
- Sombras.
- Reflexos.
- Contornos.
- Paleta.

A armadura precisa parecer parte do mesmo universo visual do protagonista clássico.

Evite que o equipamento pesado pareça simplesmente uma imagem colada por cima.

## 12.1 Consistência de proporções

Compare:

- Altura dos personagens.
- Posição dos pés.
- Comprimento das pernas.
- Comprimento dos braços.
- Largura do tronco.
- Posição da cabeça.
- Centro de massa.
- Escala das armas.

A armadura pode aumentar a silhueta externa, mas não deve alterar arbitrariamente o tamanho do corpo-base.

## 12.2 Consistência de iluminação

Padronize:

- Direção da luz.
- Contraste.
- Tons metálicos.
- Reflexos.
- Sombras.
- Saturação.
- Contornos.

Não aplique apenas um filtro global.

Corrija as peças que realmente precisam de tratamento.

---

# 13. MARCO F — TROCA REAL DE EQUIPAMENTOS

Crie um sistema de composição que receba uma configuração de equipamentos.

Exemplo conceitual:

```python
loadout = {
    "head": "none",
    "body": "gothic_coat",
    "hands": "leather_gloves",
    "feet": "traveler_boots",
    "back": "wine_cape",
    "weapon": "curved_sword",
    "offhand": "none",
}
```

O exemplo não é um contrato obrigatório.

Adapte ao manifesto real.

O compositor deve montar o personagem a partir das camadas.

Não deve selecionar uma imagem completa pré-renderizada para cada loadout.

## Testes obrigatórios

Trocar individualmente:

1. Casaco.
2. Armadura.
3. Elmo.
4. Luvas.
5. Botas.
6. Capa.
7. Espada.
8. Escudo.
9. Bainha.

Cada troca deve preservar os demais equipamentos.

---

# 14. TESTE DE COMBINAÇÕES MISTAS

Além dos dois loadouts principais, teste:

- Clássico sem capa.
- Clássico sem espada.
- Clássico com elmo.
- Clássico com escudo.
- Clássico com botas pesadas.
- Armadura pesada sem elmo.
- Armadura pesada sem capa.
- Armadura pesada com luvas leves.
- Armadura pesada com espada clássica.

Não é necessário criar itens funcionais no jogo.

O objetivo é verificar a independência visual.

Documente incompatibilidades reais.

---

# 15. MARCO G — ANIMAÇÃO IDLE

Utilize o idle de seis quadros como referência.

A animação deve preservar:

- Anatomia.
- Rosto.
- Cabelo.
- Casaco.
- Capa.
- Armas.
- Escudo.
- Armadura.
- Pivô de sola.

Não permita que equipamentos flutuem ou atravessem o corpo.

A respiração deve ser discreta.

O personagem clássico deve manter a aparência D7-B.

O pesado deve manter a aparência da D8 refinada.

---

# 16. MARCO H — ANIMAÇÃO DE CORRIDA

Utilize os oito quadros existentes como referência.

A mesma animação corporal deve funcionar com diferentes equipamentos.

Verifique:

- Movimento dos braços.
- Movimento das pernas.
- Posição dos pés.
- Balanço do tronco.
- Capa.
- Casaco.
- Espada.
- Escudo.
- Bainha.
- Elmo.
- Armadura.

Não gere uma animação corporal diferente para cada loadout.

Permita animações específicas para peças flexíveis.

---

# 17. DEFORMAÇÃO DE EQUIPAMENTOS

A D8 movimenta algumas peças pesadas por translação dos sockets.

Isso pode não ser suficiente.

Investigue:

- Translação.
- Rotação.
- Articulação por segmentos.
- Quadros próprios por peça.
- Deformação por malha 2D.
- Máscaras animadas.

Escolha a técnica adequada para cada equipamento.

Exemplos:

- Elmo: posição e rotação.
- Espada: socket e rotação.
- Escudo: socket, rotação e profundidade.
- Casaco: quadros próprios ou deformação controlada.
- Capa: quadros próprios.
- Armadura rígida: segmentos articulados.

Não deforme peças metálicas rígidas como tecido.

Não estique indiscriminadamente texturas de pixel art.

---

# 18. CAPA — MOVIMENTO REFINADO

A D8 escolheu o método A, baseado em quadros 2D.

Preserve essa decisão como referência inicial.

Melhore:

- Continuidade.
- Silhueta.
- Alinhamento com os ombros.
- Movimento durante corrida.
- Movimento durante idle.
- Oclusão.
- Contornos.
- Transparência.

É permitido combinar quadros desenhados com deslocamentos secundários discretos.

Não utilize deformações que destruam o acabamento dos pixels.

---

# 19. TROCA DE EQUIPAMENTO DURANTE ANIMAÇÃO

Crie um GIF de demonstração.

O personagem deve estar correndo.

Durante a animação:

1. Começa com equipamento clássico.
2. Troca as botas.
3. Troca as luvas.
4. Equipa o peitoral.
5. Equipa o elmo.
6. Equipa o escudo.
7. Troca a espada.
8. Retorna ao equipamento clássico.

**A fase da corrida deve continuar sem reiniciar.**

A troca não pode provocar:

- Teletransporte visual.
- Mudança de escala.
- Deslocamento dos pés.
- Mudança de pose.
- Reinício da animação.
- Alteração do corpo-base.

Se ocorrer clipping durante a troca, documente o quadro exato.

---

# 20. TESTE DE FIDELIDADE VISUAL

Compare a D9 com a D8.

Para o clássico:

- A identidade deve permanecer reconhecível.
- A silhueta deve continuar consistente.
- A paleta deve permanecer próxima.
- O rosto deve manter legibilidade.
- O casaco não pode perder detalhes.

Para o pesado:

- A silhueta deve permanecer distinta.
- O elmo deve encaixar corretamente.
- O escudo deve acompanhar o braço.
- A armadura deve ter acabamento compatível.
- As proporções devem ser coerentes.

Crie uma comparação com imagens alinhadas.

Não use escalas diferentes para favorecer a D9.

---

# 21. TESTE NA CAUSEWAY V2.3

Utilize o cenário aprovado.

Mostre:

- D7-B.
- D8 clássico.
- D9 clássico.
- D8 pesado.
- D9 pesado.

Produza comparações:

- Escala real.
- Zoom nearest-neighbor.
- Fundo escuro.
- Fundo claro.
- Cena V2.3.

Não altere o cenário.

Não altere a câmera.

Não altere o tamanho funcional do personagem.

---

# 22. TESTES AUTOMÁTICOS

Crie testes para:

- Dimensões.
- Transparência.
- Paleta.
- Integridade das máscaras.
- Presença de camadas.
- Dependências do atlas.
- Pivôs.
- Sockets.
- Ordem de composição.
- Oclusão.
- Troca individual.
- Combinações mistas.
- Idle.
- Corrida.
- Continuidade da animação.
- Alinhamento dos pés.
- Regeneração determinística.
- Integridade dos arquivos.

Inclua testes negativos.

Exemplos:

- Equipamento inexistente.
- Camada ausente.
- Socket inválido.
- Frame fora do intervalo.
- Máscara com dimensões erradas.
- Dependência circular.
- Equipamento incompatível.

O compositor deve emitir erros descritivos.

Não mascarar falhas com fallback silencioso.

---

# 23. TESTE CONTRA FALSA MODULARIDADE

Este teste é obrigatório.

Para cada peça declarada independente:

1. Renderize o personagem com a peça.
2. Renderize sem a peça.
3. Renderize com uma peça alternativa.
4. Compare os pixels fora da região legitimamente afetada.
5. Verifique se o corpo-base permanece.
6. Verifique se não surgem buracos.
7. Verifique se não há resíduos da peça removida.

Não aceite como modular uma implementação que apenas troca a imagem completa do personagem.

Não declare independência quando houver dependência oculta.

Se a separação de uma peça ainda for inviável, documente a limitação e mantenha a arte-fonte original preservada.

---

# 24. PONTOS DE CONTROLE

A D9 deve ser executada por marcos.

## Marco A — Auditoria e mapa de dependências

Entregue diagnóstico real da D8.

## Marco B — Camadas reconstruídas

Entregue atlas e desmontagem.

## Marco C — Sockets calibrados

Entregue overlays e validação.

## Marco D — Equipamentos refinados

Entregue clássico e pesado.

## Marco E — Trocas individuais

Entregue combinações e testes.

## Marco F — Idle

Entregue GIFs e inspeção.

## Marco G — Corrida

Entregue GIFs e inspeção.

## Marco H — Troca dinâmica

Entregue GIF e relatório.

Não avance automaticamente se um marco apresentar falha artística grave.

**Se a separação da arte D2 exigir redesenho artístico indisponível, pare no marco correspondente e explique o bloqueio.**

Não substitua a arte aprovada por uma versão inferior apenas para concluir a tarefa.

---

# 25. ESTRUTURA DE ENTREGA

Crie:

```text
art-src/visual-prototypes/d9_modularidade_real/
├── README.md
├── AUDITORIA_D8.md
├── MAPA_DEPENDENCIAS.md
├── CONTRATO_ATLAS_D9.md
├── CONTRATO_SOCKETS_D9.md
├── ARQUITETURA_MODULAR.md
├── REGRAS_OCLUSAO.md
├── FIDELIDADE_VISUAL.md
├── LIMITACOES.md
├── RELATORIO_FINAL.md
├── DECISAO.md
├── fontes/
├── atlas/
├── mascaras/
├── corpo/
├── equipamentos/
│   ├── roupas/
│   ├── armaduras/
│   ├── cabeca/
│   ├── luvas/
│   ├── botas/
│   ├── capas/
│   ├── armas/
│   └── escudos/
├── sockets/
├── loadouts/
├── animacoes/
│   ├── idle/
│   └── run/
├── previews/
├── comparacoes/
├── scripts/
└── testes/
```

Adapte a estrutura quando necessário.

Não gere centenas de arquivos redundantes.

---

# 26. PRANCHAS OBRIGATÓRIAS

Produza, conforme os marcos efetivamente alcançados:

1. `01_D8_vs_D9.png`
2. `02_mapa_dependencias.png`
3. `03_desmontagem_completa.png`
4. `04_atlas_limpo.png`
5. `05_sockets_idle.png`
6. `06_sockets_run.png`
7. `07_classico_refinado.png`
8. `08_pesado_refinado.png`
9. `09_troca_individual.png`
10. `10_combinacoes_mistas.png`
11. `11_idle_comparacao.png`
12. `12_run_comparacao.png`
13. `13_capa_refinada.png`
14. `14_troca_dinamica.png`
15. `15_causeway_escala_real.png`
16. `16_resultado_final.png`

GIFs:

- `idle_classico.gif`
- `idle_pesado.gif`
- `run_classico.gif`
- `run_pesado.gif`
- `troca_dinamica.gif`
- `capa_refinada.gif`

Não crie arquivos vazios para simular entregas.

---

# 27. CRITÉRIOS DE ACEITAÇÃO

## Identidade

- [ ] D7-B preservado.
- [ ] D8 clássico reconhecível.
- [ ] Rosto consistente.
- [ ] Cabelo consistente.
- [ ] Casaco detalhado.
- [ ] Capa coerente.
- [ ] Espada e bainha legíveis.

## Modularidade

- [ ] Corpo-base independente.
- [ ] Casaco removível.
- [ ] Capa removível.
- [ ] Luvas removíveis.
- [ ] Botas removíveis.
- [ ] Espada removível.
- [ ] Bainha removível.
- [ ] Armadura substituível.
- [ ] Elmo independente.
- [ ] Escudo independente.
- [ ] Sem resíduos visuais graves.
- [ ] Sem buracos no corpo-base.

## Equipamentos

- [ ] Clássico preservado.
- [ ] Pesado refinado.
- [ ] Proporções consistentes.
- [ ] Iluminação compatível.
- [ ] Ordem de desenho correta.
- [ ] Sockets calibrados.

## Animações

- [ ] Idle consistente.
- [ ] Corrida consistente.
- [ ] Capa acompanha o movimento.
- [ ] Armadura acompanha o corpo.
- [ ] Armas acompanham as mãos.
- [ ] Troca dinâmica sem reiniciar animação.
- [ ] Sem clipping grave.
- [ ] Sem tremulação excessiva.

## Segurança

- [ ] D1–D8 preservados.
- [ ] Jogo principal intocado.
- [ ] Save V3 intocado.
- [ ] Sem commit.
- [ ] Sem push.

Não marque itens como concluídos sem evidência.

Use os estados:

- Concluído.
- Parcial.
- Bloqueado.
- Não iniciado.

---

# 28. RELATÓRIO FINAL

Apresente:

A. Estado inicial do Git.

B. Auditoria D8.

C. Mapa de dependências.

D. Camadas reconstruídas.

E. Problemas de separação.

F. Atlas.

G. Máscaras.

H. Sockets.

I. Regras de oclusão.

J. Clássico refinado.

K. Pesado refinado.

L. Trocas individuais.

M. Combinações mistas.

N. Idle.

O. Corrida.

P. Movimento da capa.

Q. Troca dinâmica.

R. Comparação na causeway.

S. Testes executados.

T. Limitações restantes.

U. Arquivos prioritários para revisão.

V. Estado final do Git.

W. Recomendação para D10.

Informe claramente quais objetivos foram alcançados e quais continuam pendentes.

Não declare a fase totalmente concluída se a modularidade continuar dependendo de sprites completos.

Não faça commit.

Não faça push.

Não integre ao Bevy.

Aguarde aprovação.

---

# 29. DIRETRIZ FINAL

O Dark Castle RPG precisa de um protagonista visualmente detalhado e capaz de utilizar diferentes equipamentos.

A D7 demonstrou a direção visual correta.

A D8 demonstrou que conseguimos preservar essa aparência e acrescentar equipamentos.

A D9 deve demonstrar que a modularidade é verdadeira.

O sucesso não será medido pela quantidade de arquivos, mas pela capacidade de:

**Manter o mesmo personagem, com a mesma qualidade visual, enquanto suas roupas, armaduras, armas e acessórios são substituídos individualmente.**

Se isso ainda não for possível, identifique exatamente o bloqueio.

Não esconda limitações.

Não sacrifique a arte aprovada.

Não expanda o escopo para magias, inimigos ou integração com Bevy.

**Preserve D7-B. Refine D8. Reconstrua as camadas. Comprove a modularidade.**