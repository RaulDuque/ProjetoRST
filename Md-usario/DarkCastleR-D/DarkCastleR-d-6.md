# DARK CASTLE RPG — MEGA PROMPT D6
## Fidelidade artística D2, armaduras modulares reais e preparação do pipeline definitivo

**Projeto:** Dark Castle RPG  
**Engine:** Rust 2024 + Bevy 0.20.1  
**Pipeline artístico:** Blender 5.2.4 + acabamento 2D  
**Referência visual do protagonista:** D2 detalhado  
**Referência técnica:** D5 — corrida modular  
**Escopo:** laboratório de arte, sem integração no jogo principal

---

# 1. SUA MISSÃO

Você é o diretor técnico de arte e responsável pelo pipeline visual do Dark Castle RPG.

O proprietário já tomou as seguintes decisões:

1. O protagonista deve manter a identidade visual **D2 detalhado**.
2. O pipeline híbrido Blender + acabamento 2D continua sendo a direção escolhida.
3. O personagem precisa permitir equipamentos intercambiáveis.
4. A Fase D5 demonstrou modularidade durante a corrida, mas o acabamento visual ainda não corresponde ao D2.
5. A armadura pesada da D5 ainda é incompleta.
6. Os equipamentos provisórios precisam ser substituídos por modelos ou camadas coerentes.
7. O cenário gótico V2.3 está aprovado e não deve ser alterado.
8. A imagem de um espadachim de cabelos longos e roupas escuras enviada pelo proprietário é referência **somente para um futuro inimigo**.

Sua missão nesta fase é:

**Demonstrar que o pipeline modular da D5 consegue produzir um personagem visualmente próximo do D2, com armaduras completas, sem perder a capacidade de trocar equipamentos durante a animação.**

Não queremos apenas mais conceitos.

Queremos uma prova de qualidade artística e viabilidade técnica.

---

# 2. REGRAS DE SEGURANÇA

Antes de qualquer trabalho:

- Leia `CLAUDE.md`.
- Leia as regras dos agentes e skills do projeto.
- Execute `git status --short`.
- Confirme branch, HEAD e upstream.
- Identifique alterações locais do proprietário.
- Preserve todas as alterações existentes.

Não presuma que o HEAD ainda seja `0506313`; confirme o estado atual.

Não modifique:

- `src/`
- `assets/`
- `saves/`
- `Md-usario/`
- `.cargo/`
- `.claude/`
- Os arquivos originais de Blender.
- Os protótipos D1–D5.

Não faça commit nem push.

Não execute comandos destrutivos de Git.

Todo trabalho novo deve ficar em:

`art-src/visual-prototypes/d6_fidelidade_d2/`

Se precisar usar arquivos de fases anteriores, leia-os ou copie-os para o laboratório, preservando os originais.

Não substitua nenhuma arte publicada.

---

# 3. AUDITORIA OBRIGATÓRIA DA D5

Leia os documentos reais da D5:

- `README.md`
- `AUDITORIA_D4.md`
- `AUDITORIA_BLENDER.md`
- `DIRECAO_ARTISTICA.md`
- `ARQUITETURA_MODULAR.md`
- `CONTRATO_EXPORTACAO.md`
- `COMPARACAO_ACABAMENTO.md`
- `ESTIMATIVA_PRODUCAO.md`
- `LIMITACOES.md`
- `DECISAO.md`

Inspecione os GIFs e as pranchas produzidos.

Verifique no código e nos arquivos Blender:

1. Se as camadas são realmente independentes.
2. Se os três loadouts compartilham o mesmo ciclo de corrida.
3. Se a troca dinâmica preserva o quadro atual.
4. Se os equipamentos seguem os pontos corretos.
5. Se o corpo-base é realmente compartilhado.
6. Se os resultados podem ser regenerados.
7. Se existem problemas de clipping.
8. Se as camadas possuem pivôs e dimensões compatíveis.
9. Se a armadura pesada foi modelada ou é apenas uma ombreira.
10. Se a qualidade visual realmente está distante do D2.

**Não considere a D5 aprovada artisticamente apenas porque os testes automáticos passaram.**

Crie `AUDITORIA_D5.md` com os resultados.

---

# 4. IDENTIDADE VISUAL — NÃO REDESENHAR O PROTAGONISTA

A referência principal continua sendo:

`art-src/visual-prototypes/d2_protagonista_B/`

Use especialmente:

- A referência-mestre D2.
- O personagem lateral detalhado.
- As poses de corrida.
- As comparações na causeway.
- As referências de equipamentos.
- A paleta de cores.
- As escolhas artísticas registradas.

Preserve:

- Proporções elegantes.
- Cabelo escuro.
- Casaco gótico detalhado.
- Capa vinho.
- Ombreiras e peças metálicas.
- Botas.
- Luvas.
- Espada.
- Bainha.
- Silhueta aristocrática.
- Leitura lateral adequada para um Metroidvania.

Não transformar o protagonista em:

- Um personagem geométrico como os estudos D3 rejeitados.
- Um guerreiro genérico.
- Um cavaleiro permanentemente coberto por armadura.
- Um personagem com aparência de outro jogo.
- O inimigo de cabelos longos enviado pelo proprietário.

**A referência do inimigo é separada e não deve influenciar o redesign do protagonista.**

---

# 5. ETAPA ZERO — COMPARAÇÃO DE QUALIDADE

Antes de modelar novos equipamentos, gere uma prancha comparando:

1. D2 detalhado original.
2. D5 render bruto.
3. D5 acabamento toon.
4. D5 acabamento com contornos.
5. D5 na escala real da V2.3.

Faça a comparação com a mesma pose, escala e orientação sempre que possível.

Avalie:

- Silhueta.
- Proporções.
- Rosto.
- Cabelo.
- Casaco.
- Armadura.
- Capa.
- Espada.
- Paleta.
- Contraste.
- Textura.
- Leitura na câmera real.

Classifique cada aspecto:

- Satisfatório.
- Precisa melhorar.
- Inaceitável.
- Ainda não comparável.

Documente o que está causando a perda de qualidade.

Exemplos:

- Materiais simplificados.
- Modelagem insuficiente.
- Iluminação.
- Contornos excessivos.
- Falta de detalhes.
- Pixelização inadequada.
- Resolução.
- Ausência de acabamento artístico.
- Silhueta pouco definida.

Não tente resolver tudo apenas aumentando a resolução.

---

# 6. CRIAR UM PERSONAGEM-MESTRE D6

Produza uma versão experimental do protagonista D2 utilizando o rig modular da D5.

A prioridade é uma **pose lateral de qualidade**, antes de produzir animações completas.

Crie três alternativas de acabamento sobre a mesma identidade:

### D6-A — Blender refinado

Melhore materiais, iluminação, modelagem e renderização.

Não use acabamento pixel adicional além do necessário para exportar.

### D6-B — Híbrido refinado

Utilize Blender para anatomia, rig e renderização.

Depois aplique acabamento 2D controlado:

- Correção de silhueta.
- Limpeza de bordas.
- Ajustes de paleta.
- Sombras.
- Reflexos metálicos.
- Detalhes de roupa.
- Tratamento de cabelo.
- Separação visual dos materiais.

### D6-C — Híbrido com maior acabamento artístico

Busque maior proximidade com o D2, mantendo a arquitetura modular.

É permitido realizar retoques específicos nas camadas, desde que sejam reproduzíveis ou documentados.

Não apresente filtros automáticos como se fossem trabalho de pixel art manual.

**Não gere dezenas de animações antes de verificar se uma pose realmente alcança a direção D2.**

---

# 7. ARMADURA PESADA VERDADEIRA

A D5 utilizou uma armadura pesada incompleta.

Agora crie um conjunto experimental completo.

Peças:

- Peitoral.
- Ombreira esquerda.
- Ombreira direita.
- Braçadeiras.
- Manoplas.
- Proteção de quadril.
- Proteção de coxas.
- Grevas.
- Botas.
- Elmo.

Cada peça deve ser visualmente identificável.

Não utilize apenas uma mudança de cor.

Não considere uma ombreira isolada como armadura pesada completa.

A armadura deve acompanhar o rig.

Se alguma peça precisar de deformação especial, implemente-a no laboratório.

Registre os problemas de clipping.

---

# 8. CONJUNTO LEVE

Crie um conjunto leve coerente com o protagonista D2.

Peças:

- Camisa.
- Casaco.
- Calças.
- Luvas.
- Botas.
- Capa opcional.

O casaco deve ser independente das calças, botas e luvas.

Evite que uma única textura contenha todas as peças.

Se o casaco incluir gola ou capuz, documente como ele interage com elmos e acessórios de cabeça.

---

# 9. SISTEMA VISUAL DE SLOTS

Projete a arquitetura futura de equipamentos visuais.

Categorias desejadas:

| Categoria | Exemplos |
|---|---|
| Cabeça | Chapéu, elmo, capuz |
| Rosto | Óculos, máscara |
| Tronco | Camisa, casaco, peitoral |
| Braços | Ombreiras, braçadeiras |
| Mãos | Luvas, manoplas |
| Pernas | Calças, proteção de coxas |
| Pés | Botas, sapatos |
| Costas | Capa, manto |
| Mão principal | Espada, adaga, machado |
| Mão secundária | Escudo, arma secundária |
| Acessórios visuais | Amuletos, efeitos, adornos |

Essas categorias são uma proposta de **slots visuais**, não uma autorização para alterar os slots funcionais atuais.

Leia o sistema real de inventário antes de propor qualquer mapeamento.

Diferencie:

- Slot funcional.
- Slot visual.
- Item equipado.
- Camada renderizada.
- Socket.
- Regra de ocultação.
- Prioridade de desenho.

Exemplo:

Um elmo fechado pode ocultar cabelo e óculos.

Um casaco comprido pode ocultar parte das calças.

Uma armadura pesada pode substituir o peitoral e as ombreiras, mas preservar botas e luvas escolhidas separadamente.

Não altere o Save V3.

---

# 10. REGRAS DE SOBREPOSIÇÃO

Crie uma proposta de ordenação de camadas.

Considere:

- Braço atrás do tronco.
- Braço à frente do tronco.
- Espada na mão.
- Escudo no braço.
- Capa atrás do corpo.
- Capa eventualmente passando à frente durante movimento.
- Cabelo atrás da cabeça.
- Franja à frente do rosto.
- Elmo ocultando cabelo.
- Casaco cobrindo parte da calça.

A ordem não deve depender apenas de um número global fixo se a animação exigir mudança de profundidade.

Avalie se é necessário separar algumas peças em partes dianteiras e traseiras.

Documente como isso funcionaria no Bevy, mas não implemente no jogo.

---

# 11. TESTE DE CORRIDA COM ARMADURA COMPLETA

Após validar as poses estáticas, reutilize a animação corporal da D5.

Produza:

### LOADOUT A — Viajante

Roupa leve, espada, botas e capa.

### LOADOUT B — Cavaleiro pesado

Armadura completa, elmo, espada, escudo e capa.

### LOADOUT C — Misto

Casaco leve, manoplas pesadas, botas reforçadas, chapéu ou acessório de cabeça, espada e escudo.

Os três devem compartilhar a mesma animação corporal.

Não crie três animações independentes do corpo.

Permita animações secundárias das peças.

A armadura pesada deve ser realmente diferente da roupa leve.

A troca dinâmica deve preservar a fase da corrida.

---

# 12. ANIMAÇÃO E CONTINUIDADE

Inspecione todos os quadros.

Verifique:

- Estabilidade da cabeça.
- Comprimento dos braços.
- Comprimento das pernas.
- Contato dos pés com o chão.
- Movimento do casaco.
- Movimento da capa.
- Armas seguindo os sockets.
- Escudo acompanhando o braço.
- Elmo seguindo a cabeça.
- Armadura sem atravessar o corpo.
- Ausência de saltos de escala.
- Ausência de tremulação excessiva.

O número de quadros deve respeitar o contrato de animação realmente utilizado no projeto.

Se o ciclo D5 possuir dez quadros, preserve-o inicialmente.

Não mude a temporização de gameplay.

---

# 13. TESTES NO CENÁRIO V2.3

Utilize capturas reais do cenário aprovado.

Priorize:

`vigil_gates_causeway`

Mostre:

- D2 original.
- D6-A.
- D6-B.
- D6-C.
- Loadout leve.
- Loadout pesado.
- Loadout misto.

Produza comparações:

1. Escala real do jogo.
2. Ampliação nearest-neighbor.
3. Fundo claro.
4. Fundo escuro.
5. Cenário gótico V2.3.

Não altere o cenário.

Não modifique a câmera.

Não esconda problemas usando zoom diferente.

---

# 14. TESTES AUTOMÁTICOS

Crie testes para:

- Dimensões dos sprites.
- Transparência.
- Contagem de quadros.
- Pivôs.
- Consistência de escala.
- Presença das camadas.
- Composição dos loadouts.
- Independência das peças.
- Ordem de desenho.
- Ausência de fundo magenta.
- Reprodução determinística.
- Integridade dos arquivos exportados.

Crie também um relatório de inspeção visual.

Não declare qualidade artística aprovada apenas com base em testes automatizados.

---

# 15. TESTE DE INDEPENDÊNCIA

Este teste é obrigatório.

Utilize a mesma animação-base e demonstre:

1. Trocar somente as botas.
2. Trocar somente as luvas.
3. Trocar somente o peitoral.
4. Trocar somente o elmo.
5. Trocar somente a capa.
6. Trocar somente a espada.
7. Trocar somente o escudo.

Cada troca deve preservar os demais equipamentos.

Se alguma peça depender de outra, documente a dependência.

Exemplo:

Um peitoral pesado pode exigir ocultar a camisa por baixo, mas não deve obrigar a recriar todo o personagem.

---

# 16. ACABAMENTO D2 — CRITÉRIO DE QUALIDADE

A aparência final é tão importante quanto a modularidade.

Compare os resultados com a referência D2.

Avalie:

- Fidelidade do rosto.
- Cabelo.
- Proporção.
- Detalhes do casaco.
- Metal das armaduras.
- Tecido da capa.
- Botas.
- Espada.
- Paleta.
- Leitura na câmera real.

Crie uma tabela com notas justificadas e imagens de evidência.

Não declare o resultado aprovado artisticamente.

Essa decisão pertence ao proprietário.

Se o acabamento ainda estiver distante do D2, identifique exatamente o trabalho necessário.

---

# 17. REFERÊNCIA FUTURA DE INIMIGO

O proprietário enviou uma imagem de pixel art mostrando um espadachim de cabelos longos, roupas escuras, espada e capa desgastada.

Essa imagem é referência **somente para um futuro inimigo**.

Não a utilize para modificar o protagonista.

Crie apenas uma nota de planejamento dentro da D6:

`REFERENCIA_INIMIGO_FUTURO.md`

Nome provisório:

**Espadachim das Sombras**

Características visuais:

- Cabelos longos escuros.
- Roupas sombrias.
- Capa desgastada.
- Espada longa.
- Silhueta humana.
- Aparência ameaçadora.

Possíveis mecânicas futuras:

- Ataque de espada.
- Esquiva.
- Contra-ataque.
- Investida.
- Técnica especial sombria.

Essas mecânicas são apenas sugestões.

Não implemente o inimigo.

Não copie a imagem diretamente para assets publicados.

Se a imagem original não estiver disponível no ambiente, não invente um arquivo ou afirme tê-la analisado localmente. Registre a descrição e solicite a referência posteriormente, se necessário.

---

# 18. ESTRUTURA DE ENTREGA

Crie:

```text
art-src/visual-prototypes/d6_fidelidade_d2/
├── README.md
├── AUDITORIA_D5.md
├── IDENTIDADE_D2.md
├── ARQUITETURA_EQUIPAMENTOS.md
├── REGRAS_CAMADAS.md
├── COMPARACAO_VISUAL.md
├── REFERENCIA_INIMIGO_FUTURO.md
├── LIMITACOES.md
├── ESTIMATIVA_PRODUCAO.md
├── DECISAO.md
├── blender/
├── referencias/
├── personagem/
│   ├── D6_A/
│   ├── D6_B/
│   └── D6_C/
├── equipamentos/
│   ├── leves/
│   ├── pesados/
│   ├── cabeca/
│   ├── rosto/
│   ├── maos/
│   ├── pernas/
│   ├── botas/
│   ├── capas/
│   └── armas/
├── loadouts/
│   ├── A_viajante/
│   ├── B_cavaleiro/
│   └── C_misto/
├── animacoes/
├── previews/
├── comparacoes/
├── scripts/
└── testes/
```

Ajuste a estrutura quando necessário.

Evite duplicar arquivos grandes sem necessidade.

---

# 19. PRANCHAS E GIFS OBRIGATÓRIOS

Entregue:

1. `01_D2_vs_D5_vs_D6.png`
2. `02_D6_acabamentos.png`
3. `03_armadura_pesada_pecas.png`
4. `04_roupa_leve_pecas.png`
5. `05_slots_visuais.png`
6. `06_loadouts_A_B_C.png`
7. `07_troca_individual_pecas.png`
8. `08_causeway_escala_real.png`
9. `09_problemas_clipping.png`
10. `10_comparacao_final.png`

GIFs:

- Corrida com roupa leve.
- Corrida com armadura pesada.
- Corrida com equipamento misto.
- Troca dinâmica de equipamentos.

Se algum artefato não puder ser produzido, explique o motivo.

Não substitua uma demonstração real por uma montagem que apenas simule modularidade.

---

# 20. ESTIMATIVA DE PRODUÇÃO

Após os testes, estime:

- Tempo para finalizar o protagonista.
- Tempo para produzir uma armadura nova.
- Tempo para produzir um elmo.
- Tempo para produzir uma capa.
- Tempo para produzir uma espada.
- Tempo para produzir um escudo.
- Tempo para adaptar cada peça às animações.
- Tempo para integrar o sistema ao Bevy.
- Tempo para testes de clipping.
- Tempo para acabamento 2D.

Diferencie trabalho automatizável de trabalho artístico.

Considere também a expansão futura para inimigos humanoides que possam compartilhar partes do pipeline.

Não assuma que todos os inimigos poderão utilizar o mesmo rig.

---

# 21. CRITÉRIOS DE ACEITAÇÃO

A D6 será considerada tecnicamente bem-sucedida somente se:

- [ ] A identidade D2 for preservada.
- [ ] A armadura pesada possuir peças reais.
- [ ] O conjunto leve possuir peças separadas.
- [ ] Os três loadouts compartilharem a animação corporal.
- [ ] A troca individual de peças funcionar.
- [ ] O elmo acompanhar a cabeça.
- [ ] O escudo acompanhar o braço.
- [ ] A espada acompanhar a mão.
- [ ] A capa acompanhar a corrida.
- [ ] A composição das camadas for reproduzível.
- [ ] Os pivôs forem consistentes.
- [ ] Os problemas de clipping forem documentados.
- [ ] A escala real da V2.3 for respeitada.
- [ ] Nenhum arquivo do jogo principal for modificado.

**A qualidade artística precisa ser avaliada separadamente.**

Se a aparência ainda estiver distante do D2, não considere a arte aprovada.

---

# 22. RELATÓRIO FINAL

Apresente:

A. Estado inicial do Git.

B. Auditoria D5.

C. Problemas de fidelidade visual.

D. Estratégia de acabamento escolhida.

E. Personagem-mestre D6.

F. Comparação D6-A/B/C.

G. Armadura pesada completa.

H. Roupa leve modular.

I. Sistema de slots visuais.

J. Regras de sobreposição.

K. Corrida dos três loadouts.

L. Troca individual de equipamentos.

M. Comparação na causeway.

N. Testes automáticos.

O. Inspeção visual.

P. Limitações e problemas.

Q. Estimativa de produção.

R. Referência futura de inimigo.

S. Lista exata dos arquivos para revisão.

T. Estado final do Git.

U. Recomendação para a próxima fase.

Não faça commit.

Não faça push.

Não modifique o jogo.

Aguarde aprovação do proprietário.

---

# 23. DIRETRIZ FINAL

O objetivo não é apenas ter um personagem que troca de armadura.

O objetivo é construir uma base artística que permita, futuramente:

- Dezenas de armas.
- Diversas armaduras.
- Elmos.
- Chapéus.
- Óculos.
- Luvas.
- Calças.
- Botas.
- Capas.
- Escudos.
- Acessórios.
- Efeitos mágicos.
- Transformações.

Tudo isso mantendo um protagonista visualmente consistente e animações reutilizáveis.

**Preserve o D2 detalhado. Comprove a modularidade. Melhore a qualidade artística. Não integre nada antes da aprovação.**