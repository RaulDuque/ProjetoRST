# DARK CASTLE RPG — MEGA PROMPT N4-A
## MÁQUINA DE SPRITES ORIENTADA POR DADOS — PROVA ARTÍSTICA E ARQUITETURAL

**Branch:** `art/v1.2-protagonist`  
**Base:** N3.8 experimental publicada  
**Engine:** Bevy 0.20.1  
**Linguagem:** Rust 2024  
**Referência:** modularidade visual estilo Terraria  
**Escopo:** somente N4 — Fase A

---

# 0. MISSÃO

Implementar a primeira prova funcional de um sistema de composição de personagens 2D orientado por dados.

O sistema deverá permitir que o Rust/Bevy leia uma definição de personagem e monte sua aparência utilizando camadas de sprites independentes.

O objetivo final é permitir que cabelo, roupas, armaduras, botas, armas e acessórios sejam removidos ou substituídos sem produzir uma spritesheet completa para cada combinação.

**Nesta fase, o foco é o idle N3.5.**

Não implementar a modularidade completa da corrida.

Não substituir o protagonista de produção.

Não iniciar N5.

# 1. AUDITORIA E SEGURANÇA

Antes de modificar qualquer arquivo:

1. Confirmar branch e HEAD local/remoto.
2. Inspecionar `git status`.
3. Preservar alterações preexistentes.
4. Ler `CLAUDE.md` e regras do projeto.
5. Ler `N3_8_ARQUITETURA_TERRARIA.md`.
6. Ler `N3_8_PLANO_MODULAR_N4.md`.
7. Ler `N3_8_RISCOS_ARTISTICOS.md`.
8. Inspecionar `VisualPlayback`, `FacingDirection`, `HeldLayer` e `CapeLayer`.
9. Identificar a fonte aprovada do idle N3.5.
10. Registrar hashes dos recursos artísticos protegidos.

Não modificar `Md-usario/`, câmera, física, combate, save v3 ou arquivos artísticos originais.

# 2. CONTRATO DA MÁQUINA DE SPRITES

Criar um contrato de composição visual baseado em dados.

Utilizar RON como formato principal, aproveitando Serde e o carregamento de dados já existentes.

O contrato deve suportar:

- Identificador do personagem.
- Corpo-base.
- Partes visuais equipadas.
- Camadas e ordem de desenho.
- Spritesheets por camada.
- Clipe e quadro de animação.
- Canvas e pivô.
- Visibilidade.
- Regras de ocultação.
- Variantes visuais.
- Referências a soquetes quando necessárias.

O esquema deve ser extensível, mas não excessivamente complexo.

JSON poderá ser aceito posteriormente como formato de intercâmbio.

**Não adicionar HTML ou JavaScript como dependências do jogo.**

# 3. UM ÚNICO CONTROLADOR

Reutilizar `VisualPlayback`.

Não criar outro relógio.

Todas as camadas devem utilizar o mesmo estado de animação, incluindo clipe, quadro e direção.

O idle N3.5 deve utilizar:

- Canvas 80×128.
- Pivô `(40,118)`.
- Mesmo posicionamento do original.

Preparar o contrato para a corrida 112×128, sem implementá-la nesta fase.

# 4. REGIÕES MODULARES INICIAIS

Implementar o suporte arquitetural para:

- Corpo anatômico neutro.
- Cabelo traseiro.
- Cabelo frontal.
- Camisa.
- Calça.
- Botas.

Permitir subdivisões de frente e trás quando necessárias.

Não exigir uma entidade ECS para cada detalhe anatômico.

Não implementar dezessete camadas vazias apenas para cumprir uma contagem.

# 5. RECONSTRUÇÃO ARTÍSTICA

O protagonista N3.5 possui roupas e cabelo incorporados à imagem original.

Separar essas peças exige reconstruir as regiões ocultas.

Criar uma prova artística de corpo neutro com:

- Cabeça coerente sem cabelo.
- Torso coerente sem camisa.
- Braços e mãos completos.
- Quadril e pernas completos.
- Pés completos sem botas.

Preservar as proporções, a orientação e a linguagem visual do protagonista aprovado.

Não utilizar manequins geométricos como arte final.

Não afirmar que a anatomia foi recuperada exatamente quando tiver sido reconstruída por interpretação artística.

Se as referências disponíveis não forem suficientes para reconstruir uma região com qualidade, registrar o bloqueio e solicitar referência ao proprietário.

Não comprometer a arte aprovada para preencher lacunas.

# 6. RECOMPOSIÇÃO PIXEL A PIXEL

A prova mais importante é reconstruir a aparência original a partir das camadas.

Produzir:

1. Idle N3.5 original.
2. Idle modular recomposto.
3. Comparação lado a lado.
4. Mapa de diferenças por pixel.
5. Métricas de divergência.

A meta é correspondência exata dos pixels visíveis.

Quando a igualdade exata não for possível, identificar os pixels divergentes e explicar a causa.

**Não aprovar automaticamente uma recomposição apenas porque parece semelhante.**

A decisão artística pertence ao proprietário.

# 7. DEMONSTRAÇÃO DE REMOÇÃO

Criar uma prova interativa ou um conjunto de capturas mostrando:

- Personagem com aparência original.
- Personagem sem cabelo.
- Personagem sem camisa.
- Personagem sem calça.
- Personagem sem botas.
- Corpo-base sem roupas e cabelo.

A remoção de uma peça não pode deixar buracos transparentes onde deveria existir anatomia.

Cada peça deve ser removida independentemente.

Não usar troca por uma imagem completa de personagem para simular modularidade.

# 8. REGRAS DE SOBREPOSIÇÃO

Implementar inicialmente:

- Ordem de desenho explícita.
- Visibilidade por camada.
- Ocultação simples.
- Variantes opcionais.

Não implementar shader de máscara antes de demonstrar sua necessidade.

As peças devem utilizar o mesmo canvas e pivô do corpo, conforme o plano N3.8.

O sistema deve permitir futura ordem de desenho por clipe ou quadro.

# 9. COMPATIBILIDADE

Os manifestos antigos devem continuar funcionando.

O V2.2 deve permanecer intacto.

O modo experimental N3.8 deve continuar disponível.

O save v3 não pode mudar.

Não criar segundo inventário.

Não implementar persistência de aparência cosmética.

Não modificar os equipamentos funcionais existentes.

# 10. PROVA NO BEVY

Criar ou estender um exemplo visual isolado para a N4.

O exemplo deve permitir:

- Mostrar o original.
- Mostrar a composição modular.
- Alternar a visibilidade das peças.
- Exibir pivô e grade.
- Comparar fundo claro e escuro.
- Espelhar o personagem.
- Pausar e inspecionar a composição.

Não modificar o comportamento padrão de `cargo run`.

# 11. EVIDÊNCIAS

Criar:

`docs/development/n4/evidence/`

Entregar evidências reais de:

- Original N3.5.
- Corpo-base.
- Cabelo separado.
- Camisa separada.
- Calça separada.
- Botas separadas.
- Recomposição completa.
- Mapa de diferenças.
- Remoção individual.
- Prova no Bevy.

Não criar imagens fictícias ou declarar capturas que não foram produzidas.

# 12. TESTES

Executar:

`cargo fmt --check`

`cargo check`

`cargo clippy --all-targets --jobs 2 -- -D warnings`

`cargo test --jobs 2`

Adicionar testes para:

- Leitura dos dados RON.
- Validação das camadas.
- Sincronização do estado.
- Canvas.
- Pivô.
- Ordem de desenho.
- Visibilidade.
- Ocultação.
- Compatibilidade com manifestos antigos.
- Integridade da arte original.
- Ausência de alterações no save.

Registrar resultados reais.

# 13. GATES

## Gate A — Arquitetura
- [ ] Compositor orientado por dados.
- [ ] Reutilização de `VisualPlayback`.
- [ ] Camadas independentes.
- [ ] Sem segundo inventário.
- [ ] Sem segundo relógio.

## Gate B — Arte
- [ ] Corpo neutro coerente.
- [ ] Cabelo separável.
- [ ] Camisa separável.
- [ ] Calça separável.
- [ ] Botas separáveis.
- [ ] Proporções preservadas.

## Gate C — Recomposição
- [ ] Original comparado ao modular.
- [ ] Diferenças medidas.
- [ ] Qualidade avaliada.
- [ ] Nenhuma regressão artística escondida.

## Gate D — Segurança
- [ ] V2.2 preservado.
- [ ] N3.5 preservada.
- [ ] N3.6 preservada.
- [ ] N3.8 preservada.
- [ ] Física e combate preservados.
- [ ] Save v3 preservado.
- [ ] Testes executados.

**Nenhum gate artístico será considerado aprovado sem avaliação do proprietário.**

# 14. DOCUMENTAÇÃO E GIT

Criar documentação real em:

`docs/development/n4/`

Registrar:

- Auditoria.
- Arquitetura.
- Contrato de dados.
- Reconstrução artística.
- Composição.
- Diferenças visuais.
- Testes.
- Riscos.
- Relatório da Fase A.

Criar commits organizados na branch atual.

Não fazer force push, merge ou rebase automático.

Confirmar HEAD remoto e local após o push.

Preservar arquivos pessoais e alterações preexistentes.

# 15. ENTREGA FINAL

Apresentar:

1. O que foi implementado.
2. Quais arquivos foram criados.
3. Quais partes são realmente independentes.
4. Como o Rust lê e compõe os dados.
5. Como funciona a sincronização.
6. Como foi reconstruído o corpo neutro.
7. Comparação visual com a N3.5.
8. Quantidade e natureza das diferenças.
9. Demonstração da remoção das peças.
10. Testes realizados.
11. Evidências.
12. Commits e SHA.
13. Bloqueios artísticos.
14. Próximos passos recomendados.

**Não avançar para a modularização da corrida sem aprovação explícita da Fase A.**

# REGRA FINAL

Estamos construindo uma máquina de sprites que poderá sustentar o protagonista, NPCs e outros personagens no futuro.

Mas a primeira obrigação é preservar a qualidade artística já aprovada.

Não substituir trabalho artístico necessário por geometria simplificada.

Não transformar o personagem em um boneco rígido.

Não gerar spritesheets por combinação.

Não misturar V2.2 com N3.

Não alterar a produção.

**Implemente a N4-A como prova isolada, produza evidências verificáveis, publique os resultados e aguarde a avaliação artística do proprietário.**