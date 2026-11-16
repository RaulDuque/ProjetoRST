# DARK CASTLE RPG — MEGA PROMPT FASE 5
## D5 — Protagonista D2: pipeline híbrido, rig modular e corrida com equipamentos intercambiáveis

**Projeto:** Dark Castle RPG  
**Tecnologia:** Rust 2024, Bevy 0.20.1, Blender e ferramentas de processamento 2D  
**Direção artística:** D2 detalhado  
**Pipeline escolhido:** Blender para rig, poses e animação + acabamento 2D  
**Prioridade:** corrida com troca de armaduras  
**Escopo:** laboratório de protótipos, sem integração no jogo principal

---

# 1. MISSÃO

Você atuará como diretor técnico de arte, animador 2D/3D, especialista em rigging, desenvolvedor de ferramentas e responsável pelo controle de qualidade do Dark Castle RPG.

O proprietário definiu as seguintes decisões:

1. **O protagonista D2 detalhado é a referência artística oficial para esta investigação.**
2. Os estilos geométricos D3-B1 e D3-B2 foram rejeitados como aparência final.
3. O pipeline escolhido para investigação é híbrido: Blender + acabamento 2D.
4. O personagem precisa possuir equipamentos visuais independentes.
5. A prioridade desta fase é uma corrida convincente com diferentes armaduras.
6. O cenário gótico V2.3 está aprovado e deve permanecer intacto.
7. Nenhum código, asset publicado, save ou mecânica do jogo deve ser modificado.

A missão é demonstrar, com evidências visuais e técnicas, que podemos animar **um único personagem-base D2 e trocar seus equipamentos sem recriar a animação completa para cada combinação**.

A Fase D4 conseguiu demonstrar combinações estáticas, mas não comprovou animação modular real. Seus GIFs de movimento utilizavam composições inteiras e apresentavam desalinhamentos.

**A Fase D5 deve atacar exatamente essa limitação.**

Não entregue apenas novos conceitos bonitos. Entregue um protótipo modular reproduzível, tecnicamente verificável e visualmente fiel ao D2.

---

# 2. AUDITORIA INICIAL OBRIGATÓRIA

Antes de criar ou alterar arquivos:

1. Execute `git status --short`.
2. Confirme a branch atual.
3. Confirme o HEAD e o upstream.
4. Verifique divergências entre Git local e remoto.
5. Identifique arquivos modificados e não rastreados.
6. Leia `CLAUDE.md` e as regras do projeto.
7. Preserve todos os arquivos pessoais e trabalhos anteriores.

A branch esperada é:

`art/v1.2-protagonist`

Não presuma um hash específico: confirme o estado real do repositório.

Não execute operações destrutivas.

**Proibido:**

- `git reset --hard`
- `git clean`
- `git push --force`
- `git add .`
- `git add -A`
- Descartar alterações do proprietário.
- Criar commits ou realizar push sem autorização.

Preserve especialmente:

- `Md-usario/`
- `src/`
- `assets/`
- `saves/`
- `.cargo/`
- `.claude/`
- Todos os protótipos D1, D2, D3 e D4.

Se encontrar alterações locais inesperadas, não tente corrigi-las automaticamente.

---

# 3. ESTUDO OBRIGATÓRIO DAS FASES ANTERIORES

Leia os documentos e inspecione as imagens reais.

## 3.1. Fase D1

Diretório:

`art-src/visual-prototypes/d1_fase1/`

Estude:

- Personagem A.
- Personagem B.
- Poses de corrida.
- Proporções.
- Armaduras.
- Capas.
- Comparações em escala real.

O personagem A é referência de clareza de movimento.

O personagem B é referência de aparência.

## 3.2. Fase D2

Diretório:

`art-src/visual-prototypes/d2_protagonista_B/`

Estude:

- Referência-mestre.
- Idle.
- Run.
- Attack.
- Equipamentos.
- Paleta.
- Comparações no cenário.
- Documentação técnica.
- Limitações.

A aparência D2 deve ser preservada.

Não aceite um resultado visual inferior apenas porque é mais fácil de animar.

## 3.3. Fase D3

Diretório:

`art-src/visual-prototypes/d3_protagonista_B_refinado/`

A D3 é útil para estudar:

- Identidade consistente.
- Pivôs.
- Contagem de quadros.
- Composição por camadas.
- Testes automáticos.

Entretanto, seu acabamento geométrico foi rejeitado.

**Não utilize D3-B1 ou D3-B2 como identidade visual final.**

## 3.4. Fase D4

Diretório:

`art-src/visual-prototypes/d4_modular_d2/`

Leia obrigatoriamente:

- `README.md`
- `AUDITORIA_SLOTS.md`
- `ARQUITETURA_VISUAL.md`
- `COMPARACAO_PIPELINES.md`
- `LIMITACOES.md`
- `DECISAO.md`

Inspecione:

- Camadas independentes.
- Combinações de equipamentos.
- GIFs de movimento.
- Comparação na causeway V2.3.

A D4 identificou problemas que precisam ser enfrentados:

1. Corpo-base muito estreito.
2. Casaco contendo partes que deveriam pertencer a outras camadas.
3. Armaduras e capas extraídas como silhuetas quase completas.
4. Chapéus e elmos sem animação por quadro.
5. Corrida usando composições completas em vez de peças animadas independentemente.
6. Ausência de teste real de rig modular no Blender.
7. Problemas de oclusão e alinhamento.

Documente quais desses problemas foram resolvidos, quais continuam e quais dependem de trabalho artístico futuro.

---

# 4. REGRAS ARTÍSTICAS INEGOCIÁVEIS

O protagonista deve continuar sendo o personagem D2.

Identidade:

- Aristocrata gótico.
- Cabelo escuro.
- Proporções elegantes.
- Casaco detalhado.
- Armadura de aparência medieval/fantástica.
- Capa vinho.
- Botas.
- Luvas.
- Espada e bainha.
- Silhueta lateral reconhecível.

A aparência deve preservar a riqueza visual das referências aprovadas.

Evite:

- Personagem excessivamente geométrico.
- Anatomia simplificada sem autorização.
- Armadura que parece um bloco.
- Capa rígida.
- Espada mudando de comprimento.
- Cabeça mudando de formato entre quadros.
- Roupas flutuando.
- Equipamentos que não acompanham o movimento.
- Redução automática de imagens apresentada como pixel art final.

O Blender deve ajudar a manter consistência, não impor o estilo visual do render bruto.

Se o acabamento D2 não puder ser reproduzido com as ferramentas disponíveis, documente isso claramente.

---

# 5. OBJETIVO TÉCNICO CENTRAL

Construir uma prova de conceito baseada nesta arquitetura:

**Um personagem-base + uma animação compartilhada + equipamentos independentes.**

Exemplo:

O personagem executa a animação de corrida.

Durante a mesma corrida, podemos selecionar:

- Roupa leve.
- Armadura pesada.
- Capa.
- Elmo.
- Espada.
- Escudo.

Os equipamentos acompanham a animação.

Não deve ser necessário gerar uma corrida completamente nova para cada combinação.

## 5.1. Diferença importante

Uma peça pode precisar de sua própria geometria, sprite, deformação ou animação.

Isso é aceitável.

O que queremos evitar é reconstruir o personagem inteiro para cada combinação de equipamentos.

Não prometa que qualquer equipamento funcionará com um simples sprite estático.

Armaduras pesadas, capas e casacos longos podem exigir deformações específicas.

---

# 6. AUDITORIA DO BLENDER EXISTENTE

Antes de criar um novo rig, investigue o pipeline Blender já existente no projeto.

Localize:

- Arquivos `.blend`.
- Modelo v007.
- Armature.
- Bones.
- Materiais.
- Collections.
- Objetos de roupas.
- Objetos de armas.
- Scripts de exportação.
- Sockets.
- Convenções de coordenadas.
- Configuração de câmera.
- Iluminação.
- Renderização EEVEE.

Responda:

1. O rig existente pode ser reaproveitado?
2. Quais partes são atualmente uma malha única?
3. Quais partes podem virar objetos separados?
4. Como o equipamento seguirá os ossos?
5. Como evitar clipping?
6. Como exportar o corpo e cada equipamento separadamente?
7. Como manter pivôs idênticos?
8. Como preservar a silhueta D2?
9. Qual é o custo real de adaptar o rig existente?
10. Vale mais adaptar o v007 ou criar um rig experimental separado?

**Não modifique o arquivo Blender original.**

Trabalhe com uma cópia isolada na pasta D5.

Se não for possível acessar ou executar Blender, não simule a existência de um rig funcional. Registre a limitação.

---

# 7. CRIAR UM RIG MODULAR EXPERIMENTAL

Produza um rig de teste com:

- Pelve.
- Tronco.
- Cabeça.
- Braço esquerdo.
- Braço direito.
- Antebraços.
- Mãos.
- Coxas.
- Pernas.
- Pés.
- Âncoras de equipamentos.

Não é necessário criar um rig de produção completo se o existente puder ser adaptado.

O objetivo é validar a arquitetura.

## 7.1. Âncoras

Avalie e reutilize os sockets existentes:

- `weapon`
- `hand_l`
- `head`
- `chest`
- `back`

Caso sejam necessários novos pontos auxiliares, crie-os apenas no laboratório.

Exemplos:

- `cape_upper`
- `cape_lower`
- `helmet`
- `glasses`
- `hip_left`
- `hip_right`
- `boot_left`
- `boot_right`

Não altere os contratos publicados em `assets/`.

Documente a transformação de coordenadas entre Blender, sprites exportados e Bevy.

---

# 8. CAMADAS DE EQUIPAMENTO

Crie a seguinte estrutura visual experimental.

## 8.1. Corpo-base

O corpo-base deve ter anatomia suficiente para receber roupas e armaduras.

Não pode depender da armadura pesada para parecer um personagem completo.

Separar:

- Corpo.
- Cabelo.
- Cabeça.
- Braços.
- Pernas.
- Mãos.
- Pés.

O corpo-base não precisa ser exibido sem roupas no jogo.

## 8.2. Roupa leve

Criar um conjunto com:

- Casaco leve.
- Camisa.
- Calças.
- Botas.
- Luvas.

## 8.3. Armadura pesada

Criar um conjunto com:

- Peitoral.
- Ombreiras.
- Proteção dos braços.
- Proteção das pernas.
- Botas reforçadas.
- Elmo opcional.

A armadura pesada deve alterar a silhueta, não apenas a cor.

## 8.4. Capa

Criar uma capa independente.

A capa precisa acompanhar a corrida.

Avalie se será necessário:

- Rig secundário.
- Bones adicionais.
- Animação própria.
- Deformação.
- Sprites por quadro.

Não utilize simulação física complexa se uma animação controlada resolver o problema.

## 8.5. Armas

Criar demonstrações de:

- Espada.
- Escudo.

A espada e o escudo devem permanecer associados às mãos e aos sockets.

Não modificar as hitboxes do jogo.

---

# 9. TRÊS CONJUNTOS OBRIGATÓRIOS

Crie três loadouts utilizando o mesmo corpo-base e a mesma animação de corrida.

## LOADOUT A — Viajante

- Roupa leve.
- Botas.
- Sem elmo.
- Espada.
- Sem escudo.
- Capa opcional.

## LOADOUT B — Cavaleiro pesado

- Armadura pesada.
- Elmo.
- Botas reforçadas.
- Espada.
- Escudo.
- Capa vinho.

## LOADOUT C — Equipamento misto

- Casaco leve.
- Luvas pesadas.
- Botas reforçadas.
- Chapéu ou acessório de cabeça.
- Espada.
- Escudo.

O loadout C é especialmente importante.

Ele deve provar que o sistema não depende apenas de dois personagens completos pré-renderizados.

Se alguma combinação apresentar clipping, registre o problema e sua causa.

---

# 10. ANIMAÇÃO DE CORRIDA

Esta é a prioridade máxima da Fase 5.

Utilize o contrato real de animação do projeto.

A D3 documentou 10 quadros a aproximadamente 24 fps, mas confirme a temporização nos arquivos atuais antes de produzir.

## 10.1. Movimento

A corrida deve demonstrar:

- Transferência de peso.
- Alternância correta das pernas.
- Contato com o solo.
- Movimento natural dos braços.
- Inclinação coerente do tronco.
- Movimento do cabelo.
- Movimento da capa.
- Movimento da bainha.
- Estabilidade das armas.
- Continuidade do ciclo.

## 10.2. Compartilhamento da animação

A mesma animação-base deve dirigir os três loadouts.

Não crie três animações corporais independentes.

É permitido que equipamentos tenham animações secundárias próprias.

Exemplo:

A capa pesada pode apresentar maior atraso no movimento do que a capa leve.

Entretanto, o corpo-base continua utilizando a mesma animação.

## 10.3. Teste de troca

Produza uma demonstração em que:

1. O personagem começa correndo com o loadout A.
2. O loadout muda para B.
3. O personagem continua correndo.
4. O loadout muda para C.
5. O personagem continua correndo.

A troca deve acontecer sem reiniciar o ciclo da corrida.

Se a troca dinâmica não for viável nesta fase, explique a limitação e apresente a alternativa mais próxima, sem declarar sucesso completo.

---

# 11. EXPORTAÇÃO EM CAMADAS

Implemente uma exportação experimental que produza:

- Corpo-base.
- Casaco.
- Armadura.
- Capa.
- Cabeça e acessórios.
- Espada.
- Escudo.

Cada camada deve:

1. Ter transparência verdadeira.
2. Compartilhar o mesmo referencial espacial.
3. Utilizar o mesmo pivô de origem.
4. Respeitar a ordem dos quadros.
5. Possuir dimensões documentadas.
6. Manter nomenclatura previsível.
7. Ser reproduzível por script.

Evite recortar cada peça automaticamente em limites diferentes sem preservar sua origem.

A exportação precisa permitir reconstruir o personagem pela composição das camadas.

---

# 12. ACABAMENTO 2D

O render do Blender é uma etapa intermediária.

Produza uma comparação entre:

**A. Render bruto do Blender.**

**B. Render com paleta, contornos e tratamento 2D automatizado.**

**C. Amostra com acabamento artístico adicional, quando tecnicamente possível.**

A versão C deve buscar fidelidade ao D2.

Documente exatamente o que foi feito:

- Correção de cores.
- Contornos.
- Limpeza de pixels.
- Ajustes de luz.
- Tratamento de bordas.
- Correções manuais, caso realmente realizadas.

Não apresente filtros ou redução de resolução como equivalente a pixel art manual.

Não invente trabalho humano que não ocorreu.

Se a diferença de qualidade entre o render e o D2 continuar grande, registre isso como bloqueio artístico.

---

# 13. TESTE NA ESCALA REAL DO JOGO

Utilize uma captura real do cenário V2.3.

Referência principal:

`vigil_gates_causeway`

Verifique a câmera e a escala reais.

Compare:

- Personagem D2 original.
- Loadout A.
- Loadout B.
- Loadout C.

Mostre os personagens:

1. Em escala real de gameplay.
2. Ampliados em nearest-neighbor.
3. Sobre fundo escuro.
4. Sobre fundo claro.
5. Sobre o cenário gótico V2.3.

A comparação deve preservar escala, posição do chão e enquadramento.

Não aumente artificialmente o personagem para esconder falta de detalhes.

O cenário aprovado não deve ser redesenhado ou modificado.

---

# 14. TESTES DE MODULARIDADE

Crie verificações automáticas e visuais.

## 14.1. Testes de arquivos

Verificar:

- Dimensões.
- Quantidade de quadros.
- Transparência.
- Pivôs.
- Nomes.
- Ordem das camadas.
- Integridade dos PNGs.
- Ausência de fundo magenta.
- Correspondência temporal.

## 14.2. Testes de composição

Comprovar:

- O corpo-base é o mesmo nos três loadouts.
- A animação-base é a mesma.
- O equipamento pode ser trocado independentemente.
- A espada segue o socket.
- O escudo segue o socket.
- A capa segue o corpo.
- O elmo acompanha a cabeça.
- Não há deslocamento global indevido.

## 14.3. Testes visuais

Inspecionar todos os quadros da corrida.

Registrar:

- Clipping.
- Peças flutuantes.
- Equipamentos atravessando o corpo.
- Mudanças de proporção.
- Pés desalinhados.
- Variação indevida de escala.
- Armas desconectadas.
- Falhas de oclusão.
- Tremulação de pixels.

Os testes automáticos não substituem a inspeção visual.

---

# 15. PROVA DE REPRODUTIBILIDADE

Este é um critério obrigatório.

Depois de criar os protótipos:

1. Registre a configuração dos loadouts.
2. Exporte os quadros.
3. Gere os GIFs.
4. Gere as pranchas.
5. Execute os testes.
6. Repita a geração em um diretório temporário.
7. Compare os resultados.

Arquivos determinísticos devem produzir resultados idênticos.

Caso algum estágio não seja determinístico, documente exatamente qual e por quê.

Não altere a pasta de saída original durante a verificação.

---

# 16. CUSTO DE PRODUÇÃO

Estime o esforço necessário para expandir o pipeline.

Considere:

- Idle.
- Run.
- Jump.
- Fall.
- Attack.
- Hurt.
- Guard.
- Dodge.
- Crouch.
- Transformações.
- Outras animações já existentes.

Não invente quantidades: consulte o catálogo real de animações.

Estime também o esforço de adicionar:

- Uma espada.
- Um escudo.
- Um elmo.
- Uma armadura.
- Uma capa.
- Um conjunto completo.

Diferencie:

- Trabalho de modelagem.
- Rigging.
- Animação.
- Exportação.
- Acabamento 2D.
- QA.
- Integração futura.

Apresente estimativas como intervalos e hipóteses, não como garantias.

---

# 17. ESTRUTURA DE ENTREGA

Crie:

`art-src/visual-prototypes/d5_corrida_modular/`

Estrutura sugerida:

```text
d5_corrida_modular/
├── README.md
├── AUDITORIA_D4.md
├── AUDITORIA_BLENDER.md
├── DIRECAO_ARTISTICA.md
├── ARQUITETURA_MODULAR.md
├── CONTRATO_EXPORTACAO.md
├── COMPARACAO_ACABAMENTO.md
├── ESTIMATIVA_PRODUCAO.md
├── LIMITACOES.md
├── DECISAO.md
├── blender/
├── referencias/
├── rig/
├── camadas/
│   ├── body/
│   ├── light/
│   ├── heavy/
│   ├── cape/
│   ├── head/
│   ├── weapon/
│   └── shield/
├── loadouts/
│   ├── A_viajante/
│   ├── B_cavaleiro/
│   └── C_misto/
├── animacoes/
│   └── run/
├── acabamento/
├── previews/
├── comparacoes/
├── scripts/
└── testes/
```

Ajuste se necessário, preservando a separação entre arquivos-fonte, camadas, composições e apresentações.

Não copie centenas de arquivos redundantes se puder regenerá-los.

---

# 18. ENTREGAS VISUAIS OBRIGATÓRIAS

Produza:

**01 — Identidade:** D2 original versus o protagonista modular.

**02 — Camadas:** corpo-base e equipamentos separados.

**03 — Loadouts:** A, B e C em pose equivalente.

**04 — Corrida A:** GIF com roupa leve.

**05 — Corrida B:** GIF com armadura pesada.

**06 — Corrida C:** GIF com equipamentos mistos.

**07 — Troca dinâmica:** GIF com troca de equipamentos durante a mesma corrida, se viável.

**08 — Escala real:** os três loadouts no cenário causeway V2.3.

**09 — Acabamento:** Blender bruto versus resultado 2D.

**10 — Problemas:** prancha com clipping e falhas encontradas, quando existirem.

Cada imagem deve ter legenda clara.

Não esconda defeitos nas comparações.

---

# 19. CRITÉRIOS DE ACEITAÇÃO

A Fase D5 somente poderá ser considerada uma prova de conceito modular bem-sucedida se:

- [ ] O personagem continuar visualmente reconhecível como D2.
- [ ] Os três loadouts compartilharem o mesmo corpo-base.
- [ ] Os três loadouts compartilharem a mesma animação corporal.
- [ ] As armaduras forem camadas ou peças independentes.
- [ ] A capa acompanhar a corrida.
- [ ] Espada e escudo acompanharem seus sockets.
- [ ] A troca de equipamentos não exigir recriar a animação corporal.
- [ ] A escala do cenário V2.3 for respeitada.
- [ ] Os GIFs não apresentarem desalinhamentos graves.
- [ ] O processo de exportação for reproduzível.
- [ ] As limitações artísticas forem declaradas.
- [ ] Nenhum arquivo do jogo principal for modificado.

**Aprovação técnica não significa aprovação artística.**

Mesmo com os testes passando, o proprietário poderá rejeitar a aparência.

Se o resultado continuar muito distante do D2, classifique a fase como prova técnica parcial, não como arte pronta.

---

# 20. RESTRIÇÕES DE ESCOPO

Você está autorizado a:

- Auditar o projeto.
- Utilizar Blender em arquivos experimentais.
- Criar um rig de laboratório.
- Criar camadas e loadouts.
- Criar scripts auxiliares.
- Exportar sprites experimentais.
- Criar GIFs.
- Criar testes.
- Documentar a arquitetura.

Você não está autorizado a:

- Modificar `src/`.
- Modificar `assets/`.
- Modificar `saves/`.
- Modificar `Md-usario/`.
- Alterar o pipeline publicado.
- Alterar arquivos Blender originais.
- Substituir o protagonista atual.
- Adicionar slots funcionais ao jogo.
- Modificar Save V3.
- Alterar combate, magia, transformação ou física.
- Criar commits.
- Fazer push.

Não implemente novas mecânicas.

Não inicie a produção completa das animações.

---

# 21. RELATÓRIO FINAL OBRIGATÓRIO

Apresente:

A. Estado inicial do Git.

B. Auditoria D4.

C. Auditoria do Blender existente.

D. Rig modular criado ou adaptado.

E. Identidade D2 e fidelidade artística.

F. Camadas independentes.

G. Loadout A.

H. Loadout B.

I. Loadout C.

J. Corrida compartilhada.

K. Troca dinâmica de equipamentos.

L. Exportação e acabamento 2D.

M. Comparação na causeway V2.3.

N. Testes automáticos.

O. Inspeção visual e defeitos encontrados.

P. Reprodutibilidade.

Q. Estimativa de produção completa.

R. Limitações e riscos.

S. Arquivos exatos para revisão do proprietário.

T. Estado final do Git.

U. Recomendação técnica e artística para a próxima fase.

Não esconda falhas.

Não declare sucesso completo se a corrida modular ainda depender de personagens inteiros pré-renderizados.

---

# 22. REGRA FINAL

A prioridade não é produzir muitos sprites.

A prioridade é comprovar que conseguimos manter **a aparência detalhada D2 e um sistema de equipamentos verdadeiramente modular**.

Se houver conflito entre quantidade e qualidade, escolha qualidade.

Se o Blender preservar o movimento, mas perder a aparência D2, não considere o resultado aprovado.

Se o visual ficar bonito, mas os equipamentos não forem realmente independentes, também não considere o resultado aprovado.

**Entregue a prova de conceito, documente os resultados e aguarde minha revisão antes de qualquer integração no jogo.**