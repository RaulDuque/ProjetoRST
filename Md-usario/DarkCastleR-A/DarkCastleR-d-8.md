# DARK CASTLE RPG — MEGA PROMPT D8
## Atlas 2D definitivo experimental, equipamentos modulares e animações com fidelidade D7-B

**Projeto:** Dark Castle RPG  
**Engine:** Rust 2024 / Bevy 0.20.1  
**Fase:** D8 — Consolidação do Protagonista Modular 2D  
**Pipeline principal:** sprites 2D + máscaras + composição programática  
**Blender:** ferramenta auxiliar de poses e anatomia  
**Referências aprovadas:** D2 e D7-B  
**Escopo:** laboratório artístico, sem integração no jogo principal

---

# 1. DECISÃO DO PROPRIETÁRIO

A Fase D7 foi concluída e o proprietário avaliou os resultados visuais.

As escolhas foram:

- `comparacoes/02_D7_tres_estrategias.png`: gostou do **D7-B**.
- `comparacoes/03_D7_identidade_escala_real.png`: gostou de **D2.GAME e B.GAME**.
- `comparacoes/10_D7_resultado_final.png`: gostou do resultado.
- `comparacoes/06_D7_loadouts.png`: incluída entre as pranchas destacadas para revisão, sem aprovação explícita de cada equipamento.

Portanto, a decisão é:

**Continuar a partir do D7-B, preservando sua identidade e acabamento.**

Não voltar ao D5/D6 toon.

Não criar outro personagem geométrico.

Não trocar o estilo por uma nova interpretação.

Não redesenhar o protagonista do zero.

O objetivo é consolidar a arte escolhida e transformá-la em um sistema de produção consistente.

A aprovação atual é da direção visual, não de todos os sprites, animações e equipamentos.

---

# 2. REGRAS DE SEGURANÇA

Antes de começar:

1. Leia `CLAUDE.md`.
2. Leia as regras do projeto.
3. Confirme branch, HEAD e upstream.
4. Execute `git status --short`.
5. Identifique alterações não commitadas.
6. Leia os documentos e scripts D7.
7. Preserve integralmente D1–D7.

Não modifique:

- `src/`
- `assets/`
- `saves/`
- `Md-usario/`
- `.cargo/`
- `.claude/`
- Laboratórios anteriores.

Não faça commit ou push.

Não utilize comandos destrutivos.

Crie todo o trabalho em:

`art-src/visual-prototypes/d8_atlas_modular_2d/`

---

# 3. AUDITORIA COMPLETA DA D7

Leia:

- `d7_pixel_programatico/README.md`
- `d7_pixel_programatico/RELATORIO_FINAL.md`
- `d7_pixel_programatico/DECISAO.md`
- `d7_pixel_programatico/ARQUITETURA_CAMADAS.md`
- `d7_pixel_programatico/COMPARACAO_VISUAL.md`
- `d7_pixel_programatico/LIMITACOES.md`

Inspecione:

- `comparacoes/02_D7_tres_estrategias.png`
- `comparacoes/03_D7_identidade_escala_real.png`
- `comparacoes/06_D7_loadouts.png`
- `comparacoes/10_D7_resultado_final.png`
- `previews/`
- `scripts/run_d7.py`

Confirme:

1. De onde vêm os sprites D7-B.
2. Quais partes são realmente independentes.
3. Quais partes são reaproveitadas da D2/D4.
4. Quais máscaras possuem fundo ou chroma residual.
5. Como os loadouts são compostos.
6. Como os quadros de idle e run são produzidos.
7. Quais equipamentos são fontes próprias e quais são apenas transformações.
8. Quais elementos ainda dependem de composições completas.
9. Se o pipeline é reproduzível.
10. Quais limitações impedem a produção de novas animações.

Não suponha que todos os equipamentos já são modulares apenas porque aparecem em um loadout.

Crie `AUDITORIA_D7.md`.

---

# 4. REGRA MAIS IMPORTANTE — PRESERVAR D7-B

O protagonista aprovado deve continuar reconhecível.

Preservar:

- Proporções D2.
- Silhueta elegante.
- Cabelo curto escuro.
- Rosto.
- Casaco gótico.
- Capa vinho.
- Detalhes metálicos.
- Botas.
- Luvas.
- Espada curva.
- Bainha.
- Paleta.
- Aparência na escala real do jogo.

Não substituir detalhes artísticos por polígonos simples.

Não simplificar o rosto desnecessariamente.

Não transformar o personagem em uma versão genérica.

Não usar a imagem do futuro inimigo como referência para o protagonista.

**Se houver conflito entre simplicidade técnica e fidelidade visual, preserve a identidade D7-B e documente o custo técnico.**

---

# 5. MARCO A — ATLAS 2D LIMPO

O primeiro objetivo é organizar a arte-fonte.

Crie um atlas experimental composto por partes independentes.

Camadas desejadas:

```text
body
hair_back
hair_front
face
shirt
coat_back
coat_front
pants
gloves
boots
cape_back
cape_front
weapon
scabbard
```

Adicione outras camadas somente quando justificadas.

Cada camada deve ter:

- Fundo transparente.
- Resolução documentada.
- Pivô.
- Posição-base.
- Máscara alfa.
- Identificador.
- Ordem de composição.
- Fonte rastreável.

Não deixe fundo verde, rosa ou magenta.

Não remova detalhes do personagem junto com o chroma.

Quando a separação automática danificar uma borda, use máscara corrigida e documente o ajuste.

Não considere um recorte visualmente incorreto como aprovado apenas porque a transparência é válida.

---

# 6. FONTES ARTÍSTICAS E PROVENIÊNCIA

Use prioritariamente a arte D2/D7-B já existente no projeto.

Registre:

- Arquivo-fonte.
- Tipo de transformação.
- Máscara aplicada.
- Dimensões.
- Paleta.
- Licença ou origem, quando relevante.
- Dependências entre camadas.

Não dependa de arquivos ausentes sem documentar a limitação.

Se uma parte da arte precisar ser redesenhada, mantenha a mesma direção visual.

Evite reconstruir o personagem inteiro quando apenas uma peça precisar de correção.

---

# 7. MARCO B — SISTEMA DE SOCKETS 2D

Crie um sistema declarativo de pontos de ancoragem por quadro.

Sockets mínimos:

- Cabeça.
- Pescoço.
- Ombro traseiro.
- Ombro dianteiro.
- Cotovelo traseiro.
- Cotovelo dianteiro.
- Mão traseira.
- Mão dianteira.
- Quadril.
- Joelho traseiro.
- Joelho dianteiro.
- Pé traseiro.
- Pé dianteiro.
- Costas.
- Bainha.
- Centro do corpo.

Cada socket deve possuir:

- Nome.
- Posição.
- Referencial de coordenadas.
- Frame.
- Transformação opcional.
- Regra de espelhamento.
- Associação com equipamento.

Use um formato declarativo apropriado ao laboratório, como JSON ou RON.

Não altere os contratos atuais do jogo.

Não modifique o Save V3.

---

# 8. EQUIPAMENTOS REALMENTE INDEPENDENTES

Crie um contrato de peças visuais.

Categorias:

- Chapéu.
- Elmo.
- Óculos.
- Máscara.
- Camisa.
- Casaco.
- Armadura.
- Ombreiras.
- Luvas.
- Calças.
- Botas.
- Capa.
- Espada.
- Escudo.
- Bainha.

Cada peça deve poder ser identificada e composta independentemente.

Não gere uma imagem inteira do protagonista para cada combinação.

Se um equipamento precisar ocultar outra camada, use regras declarativas.

Exemplos:

- Elmo fechado oculta parte do cabelo.
- Casaco cobre parcialmente as calças.
- Escudo pode ficar atrás ou à frente do tronco conforme a pose.
- Espada acompanha a mão.
- Bainha acompanha o quadril.
- Capa utiliza sockets nas costas.

Diferencie os slots visuais dos oito slots funcionais do inventário.

---

# 9. MARCO C — DOIS LOADOUTS REFINADOS

Crie dois conjuntos.

## A — Protagonista clássico D7-B

- Cabelo visível.
- Camisa.
- Casaco gótico.
- Calças.
- Luvas.
- Botas.
- Capa vinho.
- Espada.
- Bainha.

## B — Cavaleiro gótico

- Elmo.
- Peitoral.
- Ombreiras.
- Braçadeiras.
- Manoplas.
- Proteção das pernas.
- Botas reforçadas.
- Capa.
- Espada.
- Escudo.

A armadura deve possuir detalhes visuais compatíveis com D7-B.

Não reutilize placas cúbicas D6 como arte final.

O conjunto pesado precisa ser distinguível na escala real.

Não descaracterize o protagonista clássico para acomodar o conjunto pesado.

---

# 10. TESTE DE TROCA INDIVIDUAL

Demonstre a troca de:

1. Elmo.
2. Casaco.
3. Peitoral.
4. Luvas.
5. Botas.
6. Capa.
7. Espada.
8. Escudo.

Cada troca deve preservar os demais equipamentos.

Crie uma prancha de comparação.

Se alguma peça não for realmente independente, classifique-a como limitação.

Não falsifique modularidade criando personagens completos separados.

---

# 11. MARCO D — IDLE REFINADO

A D7 produziu um idle de seis quadros.

Use essa animação como ponto de partida.

Objetivos:

- Preservar proporções.
- Manter o rosto estável.
- Evitar tremulação do cabelo.
- Movimento discreto de respiração.
- Movimento secundário da capa.
- Equipamentos presos aos sockets.
- Pés estáveis.
- Sem deformação excessiva.

Produza o idle nos loadouts A e B.

Não aumente a quantidade de quadros apenas para parecer mais elaborado.

A qualidade por quadro é mais importante que a quantidade.

---

# 12. MARCO E — CORRIDA REFINADA

A D7 produziu uma corrida de oito quadros.

Use essa animação como referência.

Verifique:

- Ciclo completo.
- Contato dos pés.
- Movimento das pernas.
- Balanço dos braços.
- Estabilidade do tronco.
- Cabelo.
- Casaco.
- Capa.
- Espada.
- Bainha.
- Escudo.
- Armadura.

A mesma animação corporal deve servir aos dois loadouts.

Não recrie a corrida completa para cada armadura.

É permitido ter quadros específicos para peças flexíveis.

Se o personagem perder qualidade em movimento, corrija os quadros antes de expandir.

---

# 13. CAPA — DEFORMAÇÃO E MOVIMENTO

A capa precisa ter movimento próprio.

Investigue três abordagens:

### A — Quadros 2D desenhados

Sequência própria de sprites da capa.

### B — Deformação por malha 2D

Pontos de controle e deformação suave.

### C — Simulação simplificada

Movimento secundário baseado na velocidade do personagem.

Compare as três.

Critérios:

- Fidelidade D7-B.
- Consistência da textura.
- Silhueta.
- Ausência de distorções.
- Custo de produção.
- Reprodutibilidade.
- Facilidade de integração futura.

Não implemente física complexa sem necessidade.

Escolha a solução que preserve melhor a aparência.

---

# 14. ORDEM DE DESENHO E OCLUSÃO

Crie regras de profundidade.

Exemplos:

- Capa atrás do corpo.
- Braço traseiro atrás do tronco.
- Braço dianteiro à frente.
- Espada na mão.
- Escudo podendo ocupar planos diferentes.
- Franja à frente do rosto.
- Elmo ocultando cabelo.
- Casaco sobre a calça.

Se necessário, divida peças em camadas dianteiras e traseiras.

Não resolva clipping escondendo permanentemente uma peça que deveria aparecer.

Documente a ordem de composição por animação e pose.

---

# 15. ESCALA REAL NO JOGO

Utilize capturas do cenário V2.3 aprovado.

Priorize a causeway.

Mostre:

- D2 original.
- D7-B.
- D8 clássico.
- D8 armadura pesada.
- D8 idle.
- D8 corrida.

Mantenha:

- Escala real.
- Mesmo ponto do chão.
- Mesmo enquadramento.
- Mesma orientação.
- Mesmo cenário.

Inclua ampliações nearest-neighbor para inspeção.

Não altere a câmera ou o cenário.

---

# 16. TESTES AUTOMÁTICOS

Crie testes para:

- Transparência.
- Dimensões.
- Paleta.
- Máscaras.
- Pivôs.
- Sockets.
- Ordem de desenho.
- Oclusão.
- Independência das peças.
- Quantidade de quadros.
- Alinhamento dos pés.
- Continuidade das animações.
- Composição dos loadouts.
- Reprodutibilidade.
- Integridade dos PNGs e GIFs.

Execute a regeneração em diretório temporário.

Não sobrescreva arte-fonte sem necessidade.

Os testes automáticos não substituem avaliação visual.

---

# 17. TESTE DE ESTRESSE MODULAR

Crie combinações adicionais de equipamentos.

Exemplos:

- Casaco leve + botas pesadas.
- Armadura pesada sem elmo.
- Casaco leve + elmo.
- Armadura pesada + capa.
- Casaco leve + escudo.
- Armadura pesada + espada diferente, caso exista fonte apropriada.

Não é necessário criar novos itens funcionais.

Essas combinações servem para encontrar:

- Clipping.
- Problemas de oclusão.
- Incompatibilidades.
- Pivôs errados.
- Diferenças de escala.
- Perda de identidade visual.

---

# 18. ESTRUTURA DE ENTREGA

Crie:

```text
art-src/visual-prototypes/d8_atlas_modular_2d/
├── README.md
├── AUDITORIA_D7.md
├── DIRECAO_ARTISTICA.md
├── CONTRATO_ATLAS.md
├── CONTRATO_SOCKETS.md
├── ARQUITETURA_MODULAR.md
├── REGRAS_OCLUSAO.md
├── COMPARACAO_VISUAL.md
├── LIMITACOES.md
├── RELATORIO_FINAL.md
├── DECISAO.md
├── fontes/
├── atlas/
├── mascaras/
├── sockets/
├── equipamentos/
├── loadouts/
├── animacoes/
│   ├── idle/
│   └── run/
├── capa/
├── previews/
├── comparacoes/
├── scripts/
└── testes/
```

Evite duplicações desnecessárias.

Priorize artefatos verificáveis.

---

# 19. PRANCHAS OBRIGATÓRIAS

Produza:

1. `01_D7_vs_D8.png`
2. `02_atlas_limpo.png`
3. `03_camadas_independentes.png`
4. `04_sockets.png`
5. `05_loadout_classico.png`
6. `06_loadout_pesado.png`
7. `07_troca_individual.png`
8. `08_idle_comparacao.png`
9. `09_run_comparacao.png`
10. `10_capa_metodos.png`
11. `11_causeway_escala_real.png`
12. `12_resultado_final.png`

GIFs:

- Idle clássico.
- Idle pesado.
- Corrida clássica.
- Corrida pesada.
- Troca dinâmica.
- Comparação do movimento da capa.

Se algum marco não for alcançado, não crie arquivos vazios.

Documente o bloqueio.

---

# 20. CRITÉRIOS DE ACEITAÇÃO

## Qualidade visual

- [ ] Identidade D7-B preservada.
- [ ] Fidelidade D2 mantida.
- [ ] Rosto e cabelo consistentes.
- [ ] Casaco detalhado.
- [ ] Capa coerente.
- [ ] Armadura pesada com aparência própria.
- [ ] Leitura na escala real.

## Modularidade

- [ ] Atlas limpo.
- [ ] Camadas independentes.
- [ ] Sockets documentados.
- [ ] Troca individual.
- [ ] Oclusão correta.
- [ ] Corpo-base compartilhado.
- [ ] Dois loadouts.

## Animação

- [ ] Idle consistente.
- [ ] Corrida consistente.
- [ ] Equipamentos acompanham o corpo.
- [ ] Capa possui movimento adequado.
- [ ] Pés alinhados.
- [ ] Sem clipping grave.
- [ ] Sem tremulação excessiva.

## Segurança

- [ ] D1–D7 preservados.
- [ ] Jogo principal intocado.
- [ ] Save V3 intocado.
- [ ] Sem commit.
- [ ] Sem push.

---

# 21. PONTOS DE PARADA

A D8 deve seguir uma sequência controlada.

**Marco A:** atlas limpo e identidade preservada.

**Marco B:** sockets e equipamentos independentes.

**Marco C:** dois loadouts visualmente convincentes.

**Marco D:** idle refinado.

**Marco E:** corrida refinada e capa.

Não avance para o próximo marco se o anterior apresentar perda grave de qualidade visual.

Se não conseguir separar a arte-fonte sem destruir detalhes, pare e documente a necessidade de redesenho específico.

Não substitua automaticamente a arte aprovada por versões simplificadas.

---

# 22. RELATÓRIO FINAL

Informe:

A. Estado inicial do Git.

B. Auditoria da D7.

C. Arte-fonte selecionada.

D. Estratégia de atlas.

E. Máscaras e transparência.

F. Arquitetura de sockets.

G. Regras de oclusão.

H. Equipamentos independentes.

I. Loadout clássico.

J. Loadout pesado.

K. Trocas individuais.

L. Idle.

M. Corrida.

N. Movimento da capa.

O. Comparação na causeway.

P. Testes.

Q. Limitações.

R. Arquivos prioritários para revisão.

S. Estado final do Git.

T. Recomendação para D9.

Não faça commit nem push.

Não integre ao Bevy.

Aguarde aprovação.

---

# 23. DIRETRIZ FINAL

A D7 demonstrou que a estratégia baseada em arte-fonte 2D e máscaras pode preservar a aparência que o proprietário deseja.

A D8 não deve abandonar essa conquista.

O objetivo é transformar o D7-B em uma base de produção artística modular, capaz de receber futuramente:

- Novas armas.
- Armaduras.
- Chapéus.
- Elmos.
- Óculos.
- Luvas.
- Botas.
- Capas.
- Escudos.
- Animações de combate.
- Técnicas especiais.
- Magias.
- Transformações.

Mas não produza esses sistemas agora.

Primeiro consolide o protagonista clássico, a armadura pesada, o idle e a corrida.

**D7-B é a referência visual. D2 é a identidade. O código controla o pipeline. O Blender é auxiliar. A qualidade artística vem antes da expansão.**