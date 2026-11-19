# DARK CASTLE RPG — MEGA PROMPT N4-1-A
## REVISÃO ARQUITETURAL — MÁQUINA DE SPRITES ORIENTADA POR DADOS E EQUIPAMENTOS REUTILIZÁVEIS

**Projeto:** Dark Castle RPG  
**Branch:** `art/v1.2-protagonist`  
**Engine:** Bevy 0.20.1  
**Linguagem:** Rust 2024  
**Escopo:** revisão da N4-1-A, sem iniciar N4-2 ou N5

---

# 0. DECISÃO DO PROPRIETÁRIO

A estratégia anterior de reconstruir a anatomia completa do personagem em cada quadro da corrida foi considerada excessivamente trabalhosa e artisticamente insatisfatória.

A máquina de sprites funciona, mas precisa evoluir para facilitar a criação de novos equipamentos.

O objetivo não é apenas desmontar um personagem existente.

**O objetivo é permitir que um personagem pronto receba roupas, armaduras, armas e acessórios definidos por dados, com o mínimo possível de trabalho artístico repetitivo.**

A ideia central é criar uma máquina capaz de:

1. Ler a definição de um personagem.
2. Ler a definição de um equipamento.
3. Identificar quais regiões o equipamento cobre.
4. Ocultar ou substituir partes incompatíveis.
5. Desenhar o equipamento sobre o personagem.
6. Sincronizar tudo com a animação existente.
7. Reutilizar regras e recursos entre equipamentos da mesma família.
8. Exportar prévias para PNG, spritesheet ou GIF quando necessário.

Não implementar um sistema que dependa de reconstruir o corpo nu para adicionar cada item.

# 1. AUDITORIA OBRIGATÓRIA

Antes de qualquer modificação:

- Confirmar branch e HEAD local/remoto.
- Verificar alterações preexistentes.
- Ler `CLAUDE.md` e regras do projeto.
- Ler os relatórios da N3.8, N4 e N4-1-A.
- Inspecionar `CharacterComposition`, `Outfit`, `resolve`, `CompositeCharacter`, `sync_composite_layers` e `VisualPlayback`.
- Identificar o estado dos três quadros da corrida N4-1-A.
- Identificar o contrato de equipamentos e inventário já existente.
- Registrar o estado dos arquivos protegidos.

Preservar todos os recursos artísticos aprovados.

Não modificar `Md-usario/`.

Não alterar save v3, câmera, física, combate ou produção.

# 2. PRINCÍPIO CENTRAL: PERSONAGEM VESTIDO COMO BASE

Utilizar o protagonista N3.5/N3.6 aprovado como base visual para a prova de equipamentos.

Não exigir que o personagem seja desenhado sem roupas.

O sistema deve suportar três operações distintas:

**OVERLAY:** desenhar uma peça por cima da aparência existente.

**HIDE:** ocultar regiões ou camadas que ficariam visíveis incorretamente.

**REPLACE:** substituir uma região visual por outra versão compatível.

Exemplos:

- Peitoral de ferro: cobrir ou substituir a região da camisa.
- Capacete: cobrir a cabeça e ocultar parte do cabelo.
- Luvas: cobrir os punhos.
- Botas: substituir visualmente as botas existentes.
- Capa: adicionar camada atrás do personagem.

IMPORTANTE:

Se a peça nova não cobrir totalmente a região original, o sistema deverá identificar a necessidade de máscara, variante ou substituição.

Não afirmar que uma simples sobreposição resolve todos os casos.

# 3. FORMATOS DE DADOS

Avaliar tecnicamente:

- RON.
- JSON.
- PNG.
- SVG.
- Base64.

Recomendação inicial:

**RON:** formato principal dos manifestos de personagens e equipamentos.

**JSON:** intercâmbio com ferramentas externas e possíveis editores.

**PNG:** sprites e texturas finais.

**SVG:** recurso opcional de autoria, importação ou geração de máscaras, rasterizado quando apropriado.

**Base64:** somente para transporte ou encapsulamento quando houver justificativa concreta.

Não adicionar dependências desnecessárias.

Não utilizar HTML ou JavaScript como requisito de execução do jogo.

Se um editor web for útil futuramente, ele deverá produzir dados compatíveis com o compositor Rust.

# 4. CONTRATO DE EQUIPAMENTO VISUAL

Projetar um contrato extensível que descreva:

- ID do item.
- Família visual.
- Região anatômica ou visual.
- Recurso gráfico.
- Ordem de desenho.
- Modo de composição.
- Regiões cobertas.
- Regiões ocultadas.
- Máscaras opcionais.
- Variantes.
- Paleta.
- Pivôs e pontos de encaixe.
- Compatibilidade com animações.
- Exceções por quadro.
- Comportamento ao espelhar.
- Recursos ausentes e fallback.

O contrato deve ser validado antes da renderização.

Dados inválidos devem gerar erros descritivos.

Não criar um segundo inventário.

Não alterar os atributos dos equipamentos existentes.

# 5. EXEMPLO CONCEITUAL DE ITEM

O formato abaixo é ilustrativo. Adaptar ao contrato RON real e documentar qualquer diferença.

```json
{
  "id": "iron_chestplate",
  "visual_family": "medium_armor",
  "asset": "equipment/iron_chestplate.png",
  "regions": ["torso", "upper_arms"],
  "composition": {
    "mode": "overlay",
    "hide": ["shirt_torso"],
    "draw_order": "armor_front"
  },
  "animation": {
    "profile": "humanoid_standard",
    "frame_mapping": "shared"
  },
  "palette": "dark_steel"
}
```

Não copiar esse esquema cegamente.

Primeiro avaliar o que o compositor atual já oferece.

Estender apenas o necessário.

# 6. REUTILIZAÇÃO ENTRE ITENS

O sistema deverá suportar famílias visuais.

Exemplos:

- Armaduras leves.
- Armaduras médias.
- Armaduras pesadas.
- Capacetes.
- Luvas.
- Botas.
- Capas.

Itens da mesma família poderão compartilhar:

- Regras de posicionamento.
- Máscaras.
- Ordem de desenho.
- Perfil de animação.
- Pontos de encaixe.
- Silhuetas-base, quando compatíveis.

Diferentes pinturas ou paletas poderão reutilizar a mesma geometria visual.

Não criar uma spritesheet completa para cada combinação de equipamentos.

Reconhecer que peças com silhuetas diferentes podem exigir arte específica.

# 7. PROVA DE CONCEITO OBRIGATÓRIA

Antes de expandir a corrida para seis quadros, criar uma prova de equipamentos reutilizáveis.

Utilizar:

- Idle N3.5.
- Três quadros da corrida N3.6 já selecionados na N4-1-A.

Criar dois equipamentos visuais experimentais para o torso:

**Equipamento A:** peitoral metálico simples.

**Equipamento B:** variante visual distinta do peitoral, preferencialmente reutilizando a mesma família de composição.

Ambos devem utilizar o mesmo sistema de dados.

O segundo equipamento não pode exigir alteração do código Rust nem reconstrução do corpo-base.

Não alterar os sprites originais do protagonista.

# 8. TESTE DE SOBREPOSIÇÃO

Para cada equipamento:

1. Mostrar personagem original.
2. Mostrar equipamento sobreposto.
3. Mostrar as regiões ocultadas.
4. Mostrar o resultado final.
5. Mostrar os pontos onde a sobreposição não funciona.
6. Registrar eventuais variantes necessárias.

Não aceitar camisa atravessando a armadura.

Não aceitar buracos transparentes.

Não aceitar equipamentos flutuando.

Não aceitar uma silhueta incompatível sem documentar o problema.

# 9. TESTE DE REUTILIZAÇÃO

Medir o trabalho necessário para criar o equipamento B depois do equipamento A.

Registrar:

- Quantos arquivos foram criados.
- Quantos quadros receberam arte específica.
- Quantas regras foram reutilizadas.
- Se foi necessário modificar código Rust.
- Se foi necessário alterar o corpo-base.
- Se foi necessário criar máscaras.
- Quais partes foram reaproveitadas.
- Quais partes precisaram ser desenhadas novamente.

O objetivo é demonstrar ganho real de produtividade.

Não declarar que o sistema elimina todo trabalho artístico.

# 10. ANIMAÇÃO

Reutilizar exclusivamente `VisualPlayback`.

Não criar relógios por equipamento.

Todas as camadas devem acompanhar o mesmo quadro.

Manter os parâmetros aprovados:

- Corrida de 15 FPS.
- Velocidade de 310 u/s.
- Seis quadros originais preservados.
- Canvas 112×128.
- Pivô `(56,118)`.

A prova poderá utilizar somente os três quadros C1.

Não iniciar a C2.

# 11. VISUALIZAÇÃO E EXPORTAÇÃO

Estender o exemplo experimental existente.

Permitir visualizar:

- Personagem sem equipamento novo.
- Equipamento A.
- Equipamento B.
- Regiões cobertas.
- Máscaras e ocultações.
- Grade e pivô.
- Espelhamento.
- Idle.
- Corrida C1.

Gerar evidências em PNG.

Gerar GIF quando houver animação real.

Se for simples e seguro, oferecer exportação offline de uma spritesheet composta.

Não salvar GIF automaticamente a cada mudança de equipamento durante o jogo.

Não criar cache persistente sem necessidade demonstrada.

# 12. REGRAS ARTÍSTICAS

O protagonista N3.5/N3.6 continua sendo a referência de qualidade.

A nova armadura deve parecer pertencer ao mesmo universo visual.

Utilizar:

- Volume coerente.
- Iluminação consistente.
- Sombreamento detalhado.
- Contornos compatíveis.
- Proporções preservadas.
- Movimento coerente.

Não substituir a arte aprovada por um manequim.

Não usar a anatomia reconstruída da C1 como requisito para criar equipamentos.

Não inventar animações novas para o protagonista.

# 13. TESTES

Executar:

`cargo fmt --check`

`cargo check`

`cargo clippy --all-targets --jobs 2 -- -D warnings`

`cargo test --jobs 2`

Adicionar testes para:

- Leitura de manifestos.
- Compatibilidade com dados antigos.
- Famílias visuais.
- Ordem de desenho.
- Ocultação.
- Substituição.
- Espelhamento.
- Sincronização.
- Fallback.
- Integridade da arte.
- Preservação do save.

Testar os equipamentos A e B sem alterações no código de produção entre eles.

# 14. EVIDÊNCIAS E DOCUMENTAÇÃO

Criar uma área específica da N4-1-A para esta revisão.

Documentar:

- Arquitetura proposta.
- Arquitetura implementada.
- Contrato RON.
- Equipamentos A e B.
- Regras de composição.
- Limitações encontradas.
- Métricas de reutilização.
- Comparações visuais.
- Testes.
- Riscos.
- Próximos passos.

Produzir capturas reais do Bevy.

Não apresentar imagens ilustrativas como provas de implementação.

# 15. GATES

## Gate A — Dados

- [ ] Item definido em dados.
- [ ] Validação implementada.
- [ ] Sem código Rust específico por item.

## Gate B — Composição

- [ ] Sobreposição funcional.
- [ ] Ocultação funcional.
- [ ] Substituição funcional quando necessária.
- [ ] Compatibilidade com o personagem original.

## Gate C — Reutilização

- [ ] Dois equipamentos distintos.
- [ ] Mesma família reutilizada.
- [ ] Sem reconstrução anatômica.
- [ ] Sem nova lógica Rust para o segundo item.

## Gate D — Arte

- [ ] Aparência coerente com N3.5/N3.6.
- [ ] Sem defeitos graves de sobreposição.
- [ ] Evidências para avaliação do proprietário.

## Gate E — Segurança

- [ ] V2.2 preservado.
- [ ] N3.8 preservada.
- [ ] N4 preservada.
- [ ] N4-1-A C1 preservada.
- [ ] Save v3 preservado.
- [ ] Física e combate preservados.
- [ ] Testes executados.

# 16. GIT

Trabalhar na branch atual.

Preservar alterações preexistentes.

Criar commits organizados.

Não fazer force push, merge ou rebase automático.

Não alterar `Md-usario/`.

Confirmar HEAD local/remoto após publicar.

# 17. ENTREGA FINAL

Responder:

1. O que foi reaproveitado da máquina existente.
2. O que precisou ser implementado.
3. Qual formato de dados foi escolhido e por quê.
4. Como os equipamentos são definidos.
5. Como funcionam sobreposição, ocultação e substituição.
6. Como funciona a sincronização.
7. Como criar um novo equipamento.
8. Quanto trabalho artístico foi necessário.
9. Se o segundo equipamento reutilizou o primeiro.
10. Quais limitações permanecem.
11. Testes executados.
12. Evidências produzidas.
13. Commits e SHA.
14. Decisões pendentes.

**Não iniciar C2, N4-2 ou N5 automaticamente.**

---

# OBJETIVO FINAL

A máquina de sprites precisa transformar a criação de equipamentos em um processo orientado por dados.

O ideal é que, depois de criada uma família visual, adicionar outro equipamento compatível exija principalmente:

- Criar ou selecionar sua arte.
- Definir seu manifesto RON.
- Escolher sua família.
- Configurar suas regras de composição.
- Validar sua aparência.

**Não deve exigir reconstruir o protagonista inteiro.**

Implemente a prova de reutilização, apresente evidências reais e aguarde aprovação artística antes de avançar.