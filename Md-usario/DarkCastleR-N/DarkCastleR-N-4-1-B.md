# DARK CASTLE RPG — MEGA PROMPT N41-B
## FÁBRICA AUTOMATIZADA DE EQUIPAMENTOS — GERAÇÃO ORIENTADA POR DADOS, PIXEL ART DE ALTA FIDELIDADE E ESCALABILIDADE

**Projeto:** Dark Castle RPG  
**Engine:** Bevy 0.20.1  
**Linguagem:** Rust 2024  
**Branch:** `art/v1.2-protagonist`  
**Etapa oficial:** N41-B  
**Objetivo:** construir uma fábrica escalável de equipamentos visuais sem comprometer a qualidade artística do protagonista.

---

# 0. CONTEXTO E DECISÃO DO PROPRIETÁRIO

O projeto já possui uma máquina de sprites capaz de:

- Ler manifestos RON.
- Compor personagens por camadas.
- Utilizar `VisualPlayback` como relógio único.
- Sobrepor equipamentos.
- Ocultar camadas específicas.
- Substituir peças.
- Utilizar sprites próprios para a direção espelhada.
- Reutilizar famílias visuais.
- Gerar equipamentos experimentais a partir de receitas JSON.
- Exportar evidências PNG e GIF.

A prova anterior demonstrou a criação de dois peitorais, incluindo `iron_cuirass` e `blackened_breastplate`.

A arquitetura funcionou, mas os equipamentos ainda apresentaram problemas artísticos:

- Formas excessivamente retangulares.
- Ombros com contornos artificiais.
- Sombreamento insuficiente.
- Aparência de adesivos sobre o personagem.
- Pouca diversidade de silhueta.
- Dependência de código Python para introduzir novos padrões visuais.

O proprietário decidiu priorizar uma **Fábrica Automatizada Orientada por Dados** antes de continuar desenhando manualmente equipamentos ou expandindo as animações.

A fábrica deve permitir criar grandes quantidades de equipamentos por meio de modelos, materiais, ornamentos, regras de composição e perfis de animação reutilizáveis.

## Princípio fundamental

A fábrica precisa automatizar não apenas a criação dos manifestos, mas também a produção de recursos visuais reais.

Entretanto, não confundir:

1. Receita gerada.
2. PNG produzido.
3. Design visualmente distinto.
4. Equipamento artisticamente aceitável.
5. Equipamento integrado ao personagem.
6. Equipamento corretamente animado.

Esses resultados devem ser medidos separadamente.

**A meta desta etapa é qualidade, reutilização e escalabilidade demonstrável — não volume artificial.**

---

# 1. AUDITORIA OBRIGATÓRIA

Antes de modificar qualquer arquivo:

1. Confirmar a branch atual.
2. Verificar HEAD local e remoto.
3. Executar `git status`.
4. Preservar alterações preexistentes.
5. Ler `CLAUDE.md` e todas as regras aplicáveis.
6. Ler a documentação da N3.8.
7. Ler a documentação da N4.
8. Ler os relatórios da N4-1.
9. Ler `docs/development/n4/N41A_RELATORIO.md`.
10. Inspecionar `build_equipment.py`.
11. Inspecionar os contratos RON existentes.
12. Inspecionar `equipment_visual.rs`.
13. Inspecionar `composition.rs` e `composite.rs`.
14. Inspecionar `VisualPlayback`.
15. Inspecionar `n4_sprite_machine`.
16. Identificar quais regiões já possuem máscaras e perfis válidos.
17. Identificar quais soquetes de armas continuam sem validação.
18. Registrar os hashes dos recursos artísticos protegidos.

Não assumir que todos os sistemas descritos em documentos estão implementados.

Confirmar no código.

Se houver alterações locais não relacionadas, preservá-las e informar o proprietário.

Não modificar `Md-usario/`.

---

# 2. MISSÃO ARQUITETURAL

Evoluir a máquina atual para uma fábrica composta por cinco subsistemas.

## A — Biblioteca de templates

Define formas reutilizáveis de equipamentos.

Cada template pode conter:

- Silhueta.
- Máscara.
- Mapa de relevo.
- Regiões de materiais.
- Regiões de ornamentos.
- Pivô.
- Pontos de encaixe.
- Regras de sobreposição.
- Compatibilidade com animações.

## B — Biblioteca de materiais

Define a aparência de:

- Ferro.
- Aço.
- Aço escurecido.
- Bronze.
- Ouro.
- Couro.
- Tecido.
- Madeira.
- Materiais mágicos.

## C — Biblioteca de detalhes

Define componentes reutilizáveis:

- Rebites.
- Fivelas.
- Costuras.
- Emblemas.
- Runas.
- Bordas.
- Placas.
- Gemas.
- Bordados.
- Desgaste.

## D — Gerador visual

Combina os dados anteriores e produz PNGs reais.

## E — Integrador

Transforma os recursos produzidos em equipamentos compatíveis com o compositor Rust/Bevy.

**Não criar um segundo renderizador, segundo inventário ou segundo relógio de animação.**

---

# 3. FORMATOS E CONTRATOS

Adotar inicialmente:

- **RON:** manifestos consumidos pelo jogo.
- **JSON:** receitas, catálogos e relatórios.
- **PNG:** sprites e máscaras finais.
- **SVG:** opcional para autoria, rasterizado offline.
- **Base64:** somente quando necessário para transporte.

Não usar HTML ou JavaScript como dependências do jogo.

Um futuro editor web poderá produzir receitas JSON compatíveis com a fábrica.

## Requisito de extensibilidade

Adicionar um equipamento que utilize operações já suportadas não pode exigir alterações no Rust nem no Python.

Adicionar uma nova paleta, material ou ornamento dentro do contrato existente também deve ser possível por dados.

Se uma nova operação gráfica exigir código, documentar essa extensão separadamente.

---

# 4. CONTRATO DE RECEITA

Criar ou evoluir um contrato que descreva:

- ID.
- Categoria.
- Família.
- Template.
- Material.
- Paleta.
- Detalhes.
- Seed.
- Versão do esquema.
- Perfil de animação.
- Regiões cobertas.
- Regiões ocultadas.
- Máscaras.
- Variantes.
- Regras de espelhamento.
- Regras de fallback.

Exemplo conceitual:

```json
{
  "id": "armor_steel_001",
  "category": "chest_armor",
  "family": "medium_armor",
  "template": "segmented_chestplate",
  "material": "dark_steel",
  "details": [
    {
      "type": "shoulder_trim",
      "material": "silver"
    },
    {
      "type": "center_emblem",
      "variant": "wolf"
    }
  ],
  "generation": {
    "seed": 1042,
    "animation_profile": "humanoid_torso"
  }
}
```

O exemplo não é um contrato definitivo.

Adaptar aos tipos reais do projeto.

Validar todos os campos e referências.

Não aceitar dados inválidos silenciosamente.

---

# 5. GERAÇÃO DETERMINÍSTICA

A mesma receita, com os mesmos recursos e a mesma versão do gerador, deve produzir resultados idênticos.

Implementar:

- Seed explícita.
- Ordenação estável.
- Hash da receita.
- Hash dos recursos de entrada.
- Hash dos arquivos produzidos.
- Versão do gerador.
- Manifesto de proveniência.
- Registro das dependências.

Executar pelo menos duas gerações independentes da mesma amostra e comparar os resultados.

Não depender de aleatoriedade global.

Não depender da ordem arbitrária dos arquivos.

---

# 6. BIBLIOTECA DE TEMPLATES

Criar templates reutilizáveis, priorizando as categorias da prova.

## Armaduras

Preparar quatro templates de torso visualmente distintos.

Não criar quatro retângulos com cores diferentes.

Explorar diferenças reais de:

- Silhueta.
- Gola.
- Segmentação.
- Cintura.
- Ombros.
- Placas.
- Bordas.
- Distribuição do volume.

## Espadas

Preparar quatro modelos de lâmina e três combinações de guarda/cabo.

Permitir materiais e ornamentos diferentes.

Não alterar hitboxes ou lógica de combate.

## Capas e ombreiras

Preparar três modelos, respeitando as diferenças entre:

- Capa flexível.
- Ombreira rígida.
- Acessório de ombro.

Não tratar uma capa longa como se fosse uma placa rígida.

Se uma categoria não tiver dados de movimento suficientes, gerar o recurso isolado e registrar a limitação.

---

# 7. SISTEMA DE MATERIAIS E CONTRATO RÍGIDO DE PIXEL ART

Esta seção é obrigatória e tem prioridade sobre a quantidade de itens gerados.

## 7.1. Mapas de relevo e iluminação

Cada template deverá possuir um mapa de relevo ou iluminação adequado à sua geometria.

O mapa pode utilizar valores de 0 a 255.

O gerador deve utilizá-lo para preservar:

- Volume.
- Curvatura.
- Dobras.
- Planos de superfície.
- Sombras.
- Reflexos.

Não utilizar apenas preenchimento plano.

Um mapa de tons de cinza não representa automaticamente uma normal 3D; documentar se o recurso é um mapa de altura, de iluminação ou de regiões tonais.

Quando o template exigir iluminação direcional real, usar informações adicionais adequadas.

Não inventar volume físico que o mapa não descreve.

## 7.2. Paleta mínima

Cada material deverá possuir pelo menos quatro níveis tonais:

- `outline`
- `shadow`
- `base`
- `highlight`

Permitir níveis adicionais quando necessários.

Os materiais também poderão definir:

- Desgaste.
- Brilho.
- Contraste.
- Rugosidade visual.
- Cor de reflexo.
- Variações de iluminação.

## 7.3. Sel-outline

Utilizar contornos coloridos coerentes com o material.

Exemplos:

- Ferro: azul-aço muito escuro.
- Couro: marrom profundo.
- Tecido vermelho: vinho escuro.
- Ouro: marrom-dourado escuro.

Evitar preto puro `#000000` em contornos internos gerados.

Não alterar contornos pretos existentes na arte original aprovada.

## 7.4. Grade de pixels

Todas as operações devem respeitar a grade de texels da arte original.

Proibido:

- Escala fracionária.
- Anti-aliasing.
- Blur.
- Interpolação linear.
- Subpixels.
- Redimensionamento que produza pixels misturados.

Usar coordenadas inteiras e renderização `nearest`.

## 7.5. Ornamentos

Rebites, runas, fivelas, costuras e bordados devem respeitar:

- Escala 1:1.
- Silhueta.
- Máscara.
- Profundidade.
- Paleta.
- Posição.
- Resolução.

Não permitir detalhes flutuantes ou atravessando bordas.

## 7.6. Qualidade mínima

O equipamento não pode parecer um adesivo plano sobre o personagem.

Exigir avaliação de:

- Volume.
- Integração com o torso.
- Contorno.
- Iluminação.
- Material.
- Silhueta.
- Proporção.
- Profundidade.

Se o gerador não conseguir atingir o padrão, registrar a limitação e produzir evidências.

Não declarar qualidade artística aprovada automaticamente.

---

# 8. SISTEMA DE DETALHES ORIENTADO POR DADOS

Os detalhes devem ser definidos por receitas.

Permitir:

- Posicionamento relativo.
- Máscaras.
- Espelhamento.
- Paletas.
- Repetição.
- Variação de intensidade.
- Seleção por região.
- Compatibilidade com materiais.
- Regras de profundidade.

O gerador não deve possuir condicionais específicas para cada equipamento.

Um detalhe novo que utilize operações existentes deve ser adicionado como recurso e dados.

Não adicionar uma função Python exclusiva para cada emblema.

---

# 9. PERFIS DE ANIMAÇÃO — ESCOPO CONTROLADO

A fábrica deverá preparar perfis reutilizáveis.

Exemplos:

- `humanoid_torso`
- `weapon_main_hand`
- `humanoid_shoulder`
- `cape_back`

Entretanto, nesta etapa a validação deve ser limitada.

## 9.1. Idle primeiro

A primeira integração deve utilizar o idle N3.5.

Validar:

- Canvas.
- Pivô.
- Posicionamento.
- Sobreposição.
- Ocultação.
- Espelhamento.
- Compatibilidade visual.

## 9.2. Tabela de offsets por quadro

Preparar uma tabela orientada por dados com:

- Clipe.
- Índice do quadro.
- Offset X/Y.
- Pivô.
- Ordem de desenho.
- Regiões ocultadas.
- Soquetes disponíveis.
- Confiança da medição.

Não inventar coordenadas para mãos ocultas.

Marcar valores não medidos como pendentes.

## 9.3. Corrida C1

Utilizar somente os três quadros já trabalhados:

- #0.
- #2.
- #4.

Testar apenas:

- Uma família de armadura de torso.
- Uma espada representativa, se o soquete for validado.

Não expandir automaticamente os 40 itens para a corrida.

## 9.4. Oclusão

Mapear onde o braço frontal passa diante do peitoral.

Criar ou reutilizar máscaras de oclusão quando necessário.

Não resolver sobreposição incorreta simplesmente mudando toda a ordem das camadas.

## 9.5. Limitações

Uma única imagem não se transforma automaticamente em uma animação perfeita.

Registrar quando forem necessárias:

- Variantes por pose.
- Transformações.
- Máscaras adicionais.
- Correções artísticas.

Não iniciar os seis quadros completos da corrida.

---

# 10. COMPOSIÇÃO NO BEVY

Reutilizar:

- `VisualPlayback`.
- O compositor atual.
- As camadas existentes.
- As regras OVERLAY, HIDE e REPLACE.

Não criar um segundo sistema de animação.

Não modificar a física.

Não modificar o protagonista aprovado.

Não reconstruir o corpo nu.

O personagem vestido N3.5/N3.6 continua sendo a referência visual.

---

# 11. ARMAS E EQUIPAMENTOS RÍGIDOS

Espadas poderão ser geradas por composição de:

- Lâmina.
- Guarda.
- Cabo.
- Pomo.
- Material.
- Ornamentos.

Gerar recursos visuais isolados inicialmente.

A integração ao personagem depende de soquetes válidos.

Não modificar:

- Dano.
- Hitboxes.
- Ataques.
- Combate.
- Inventário.
- Save.

Não declarar uma espada integrada à corrida apenas porque seu PNG foi produzido.

---

# 12. PROVA DE ESCALA — 40 EQUIPAMENTOS COM ALTA FIDELIDADE

Substituir a meta anterior de 100 equipamentos por uma prova de 40.

Distribuição:

| Categoria | Quantidade |
|---|---:|
| Armaduras de torso | 16 |
| Espadas | 12 |
| Capas / ombreiras | 12 |
| **Total** | **40** |

## 12.1. Armaduras

Gerar:

**4 templates × 4 materiais = 16 combinações iniciais.**

Utilizar detalhes para aumentar a diferenciação.

As armaduras devem possuir silhuetas e materiais perceptivelmente diferentes.

## 12.2. Espadas

Gerar:

**4 lâminas × 3 combinações de guarda/cabo = 12 combinações iniciais.**

Materiais e ornamentos poderão variar.

## 12.3. Capas e ombreiras

Gerar:

**3 modelos × 4 paletas ou bordados = 12 combinações iniciais.**

Se o projeto não possuir suporte confiável para determinada categoria, gerar os recursos isolados e registrar o bloqueio de integração.

## 12.4. Critérios

- 40 receitas válidas.
- 40 recursos gráficos produzidos.
- Seeds determinísticas.
- Manifestos válidos.
- IDs únicos.
- Proveniência.
- Relatório de diversidade.
- Relatório de qualidade.
- Nenhuma duplicata exata não declarada.

**Não exigir que todos os 40 itens estejam integrados à corrida.**

## 12.5. Regra de qualidade

Se a fábrica produzir itens visualmente ruins, interromper a expansão.

Não completar os 40 apenas para satisfazer a contagem.

O relatório deve informar quantos itens foram realmente produzidos e quantos passaram pelos critérios.

---

# 13. TESTE DE DIVERSIDADE VISUAL

Não utilizar somente SHA-256 para determinar diversidade artística.

SHA-256 comprova igualdade ou diferença de arquivos, mas não qualidade visual.

Implementar métricas complementares.

## 13.1. Diferença de pixels

Comparar imagens alinhadas da mesma família.

Calcular:

- Quantidade de pixels diferentes.
- Percentual de pixels visíveis diferentes.
- Diferença de silhueta.
- Diferença de cor.
- Diferença de regiões internas.

Utilizar 8% como referência inicial de triagem.

Quando dois equipamentos apresentarem diferença inferior a 8% dos pixels visíveis, classificá-los inicialmente como:

`VARIACAO_MENOR`

Não classificá-los automaticamente como designs distintos.

## 13.2. Histograma de paleta

Comparar a distribuição de cores.

Detectar:

- Paletas praticamente idênticas.
- Mudanças mínimas de luminosidade.
- Recolorações significativas.
- Materiais diferentes com aparência semelhante.

## 13.3. Hash perceptual

Avaliar a utilização de hash perceptual como métrica auxiliar.

Não confundir distância Hamming de hashes perceptuais com percentual de pixels diferentes.

São medidas distintas.

## 13.4. Normalização

Antes da comparação:

- Alinhar canvas e pivô.
- Preservar transparência.
- Usar a mesma escala.
- Comparar a mesma pose.
- Distinguir diferenças de silhueta e de cor.

Não comparar imagens desalinhadas como se fossem variantes.

## 13.5. Classificação

Utilizar categorias como:

- `DUPLICATA_EXATA`
- `VARIACAO_MENOR`
- `CANDIDATO_DISTINTO`
- `REVISAR_VISUALMENTE`

O limiar de 8% é uma heurística inicial, não uma prova definitiva de originalidade.

A aprovação artística permanece com o proprietário.

---

# 14. TESTE NO PERSONAGEM — ESCOPO RESTRITO

Não tentar equipar todos os 40 itens em todas as animações.

## Idle

Selecionar uma amostra representativa de armaduras.

Testar:

- Sobreposição.
- Cobertura.
- Máscaras.
- Ordem de desenho.
- Espelhamento.
- Silhueta.
- Qualidade artística.

## Corrida C1

Testar somente:

- Uma família de armadura.
- Uma espada representativa, caso o soquete esteja validado.

Para cada quadro:

- Verificar offsets.
- Verificar pivô.
- Verificar oclusão do braço frontal.
- Verificar espelhamento.
- Verificar continuidade visual.

Se a espada não puder ser posicionada corretamente, registrar o bloqueio.

Não inventar soquetes.

## Outras categorias

Capas e ombreiras podem permanecer como recursos isolados se não houver suporte suficiente.

Não declarar integração inexistente.

---

# 15. PROVA DE AUTOMAÇÃO REAL

Demonstrar que um equipamento novo pode ser criado sem alterar código.

Procedimento:

1. Selecionar uma família existente.
2. Criar uma receita JSON.
3. Selecionar template.
4. Selecionar material.
5. Selecionar detalhes.
6. Executar o gerador.
7. Produzir PNG.
8. Produzir manifesto RON.
9. Validar os recursos.
10. Carregar no Bevy quando a categoria possuir suporte.
11. Gerar captura.
12. Confirmar que nenhum código Rust ou Python foi modificado para o novo item.

Repetir com pelo menos dois itens distintos.

Registrar quais operações foram reutilizadas.

---

# 16. PROCESSAMENTO EM LOTE

Criar ou evoluir o comando de geração em lote.

Deve permitir:

- Ler catálogo.
- Validar receitas.
- Gerar recursos.
- Evitar sobrescrita.
- Registrar falhas.
- Gerar hashes.
- Produzir relatório.
- Reutilizar cache.
- Reproduzir resultados.

O cache deve considerar:

- Receita.
- Recursos.
- Versão do gerador.

Não utilizar apenas o nome do arquivo.

---

# 17. CONTROLE AUTOMÁTICO DE QUALIDADE

Validar:

- PNG.
- Dimensões.
- Transparência.
- Máscaras.
- Pivôs.
- Paletas.
- Contornos.
- Grade de pixels.
- Ordem de desenho.
- Regiões descobertas.
- Duplicatas.
- Recursos ausentes.
- Espelhamento.
- Compatibilidade de animação.

Separar:

**ERRO TÉCNICO:** invalida o recurso.

**AVISO ARTÍSTICO:** exige revisão.

Não aprovar automaticamente a qualidade artística com base apenas em testes.

---

# 18. DIREÇÃO ARTÍSTICA

Referência obrigatória:

- Protagonista N3.5.
- Corrida N3.6.
- Identidade gótica/medieval.
- Pixel art detalhada.

Os equipamentos devem apresentar:

- Volume coerente.
- Contornos limpos.
- Sombreamento consistente.
- Materiais reconhecíveis.
- Silhuetas legíveis.
- Ornamentos proporcionais.
- Integração com o personagem.

Evitar:

- Retângulos metálicos colados.
- Preenchimento plano.
- Ombros artificiais.
- Contornos pretos excessivos.
- Detalhes fora da máscara.
- Materiais visualmente indistinguíveis.

Se a qualidade não alcançar a referência, documentar os defeitos e apresentar propostas de melhoria.

---

# 19. SVG, JSON E BASE64

## JSON

Formato principal das receitas e catálogos.

## SVG

Opcional para autoria de:

- Templates.
- Máscaras.
- Ornamentos.
- Formas paramétricas.

Rasterizar offline com controle de resolução.

Não introduzir suavização.

## Base64

Utilizar somente quando houver justificativa concreta.

Não embutir centenas de PNGs nos manifestos principais.

---

# 20. EXPORTAÇÃO

Produzir:

- PNG individual.
- Spritesheets quando apropriado.
- Manifestos RON.
- Receitas JSON.
- Catálogo visual.
- Relatório de diversidade.
- Relatório de validação.
- GIF quando houver animação real.

GIF é formato de prévia.

Não utilizá-lo como substituto do compositor Bevy.

---

# 21. DESEMPENHO

Medir:

- Tempo de geração.
- Tempo de carregamento.
- Uso de memória.
- Tamanho dos recursos.
- Quantidade de arquivos.
- Custo de composição.
- Custo de troca de equipamento.

Não otimizar sem medir.

Não gerar recursos a cada frame do jogo.

---

# 22. TESTES

Executar:

`cargo fmt --check`

`cargo check`

`cargo clippy --all-targets --jobs 2 -- -D warnings`

`cargo test --jobs 2`

Adicionar testes para:

- Parsing.
- Validação.
- Determinismo.
- Seeds.
- Templates.
- Materiais.
- Detalhes.
- Mapas de relevo.
- Paletas.
- Máscaras.
- Contornos.
- Grade 1:1.
- Cobertura.
- Espelhamento.
- Animação.
- Manifestos.
- Duplicatas.
- Cache.
- Compatibilidade anterior.

Executar testes reais de geração em lote.

---

# 23. EVIDÊNCIAS

Criar uma área específica da N41-B em:

`docs/development/n4/evidence/n41b/`

Entregar:

1. Catálogo visual.
2. Comparação dos quatro templates de torso.
3. Comparação de materiais.
4. Comparação de ornamentos.
5. Mapas de relevo.
6. Equipamentos aplicados ao protagonista.
7. Prova de oclusão.
8. Comparação de espelhamento.
9. Corrida C1 com a armadura selecionada.
10. Espada na corrida, se houver soquete validado.
11. Relatório de diversidade.
12. Relatório de determinismo.
13. Relatório de desempenho.
14. Capturas reais do Bevy.
15. GIFs de demonstração quando aplicáveis.

Não apresentar imagens ilustrativas como evidência de implementação.

---

# 24. DOCUMENTAÇÃO

Documentar:

- Arquitetura.
- Contratos.
- Templates.
- Materiais.
- Detalhes.
- Mapas de relevo.
- Geração.
- Determinismo.
- Diversidade.
- Perfis de animação.
- Integração.
- Limitações.
- Testes.
- Evidências.
- Riscos.

Criar tutorial:

**COMO CRIAR UM EQUIPAMENTO NOVO SEM ALTERAR CÓDIGO**

Demonstrar o processo completo.

---

# 25. GATES DE ACEITAÇÃO

## Gate A — Arquitetura

- [ ] Fábrica orientada por dados.
- [ ] Reutilização do compositor.
- [ ] Sem segundo relógio.
- [ ] Sem segundo inventário.
- [ ] Contratos validados.

## Gate B — Pixel art

- [ ] Mapas de relevo.
- [ ] Quatro tons mínimos.
- [ ] Sel-outline.
- [ ] Grade 1:1.
- [ ] Sem blur.
- [ ] Sem escala fracionária.
- [ ] Sem preenchimento plano inadequado.

## Gate C — Geração

- [ ] Templates reutilizáveis.
- [ ] Materiais reutilizáveis.
- [ ] Detalhes reutilizáveis.
- [ ] Seeds determinísticas.
- [ ] Geração em lote.

## Gate D — Escala

- [ ] Até 40 receitas processadas conforme a qualidade permitir.
- [ ] Recursos gráficos reais.
- [ ] Duplicatas identificadas.
- [ ] Diversidade medida.
- [ ] Falhas documentadas.

## Gate E — Integração

- [ ] Idle validado.
- [ ] Corrida C1 limitada.
- [ ] Oclusão validada.
- [ ] Espada integrada somente com soquete confiável.
- [ ] Sem reconstrução anatômica.

## Gate F — Segurança

- [ ] V2.2 preservado.
- [ ] N3.5/N3.6 preservadas.
- [ ] N3.8 preservada.
- [ ] N4 preservada.
- [ ] N4-1 preservada.
- [ ] Save v3 preservado.
- [ ] Física e combate preservados.
- [ ] Testes executados.

## Gate G — Aprovação artística

**Pendente até avaliação explícita do proprietário.**

Não considerar o Gate G aprovado automaticamente.

---

# 26. ESTRATÉGIA DE EXECUÇÃO

Executar em fases internas.

## Fase 1 — Fundação

Contratos, templates, materiais, detalhes e validação.

## Fase 2 — Amostra pequena

Produzir uma amostra representativa antes da geração em lote.

Avaliar:

- Volume.
- Silhueta.
- Contorno.
- Material.
- Integração.

Se a qualidade for insuficiente, corrigir a biblioteca antes de expandir.

## Fase 3 — Integração

Testar o idle e a corrida C1 no escopo autorizado.

## Fase 4 — Escala

Gerar até 40 equipamentos.

Medir diversidade, falhas e desempenho.

## Fase 5 — Evidências

Produzir catálogo, documentação e relatório.

Se uma fase apresentar bloqueio grave, interromper a expansão e informar.

---

# 27. GIT E SEGURANÇA

Trabalhar exclusivamente na branch:

`art/v1.2-protagonist`

Regras obrigatórias:

1. Não fazer force push.
2. Não executar `git push` antes da aprovação visual explícita do proprietário.
3. Não fazer merge ou rebase automático.
4. Não sobrescrever alterações preexistentes.
5. Não modificar `Md-usario/`.
6. Não alterar save v3.
7. Não alterar física.
8. Não alterar combate.
9. Não alterar a câmera de produção.
10. Não modificar a arte aprovada N3.5/N3.6.
11. Não modificar evidências antigas.
12. Não promover o protagonista experimental para produção.

Commits locais organizados são permitidos somente após os testes necessários passarem.

Se houver falhas, registrar o estado e não apresentar o trabalho como concluído.

**A entrega desta etapa deve permanecer local até autorização de publicação.**

---

# 28. RELATÓRIO FINAL

Responder obrigatoriamente:

1. Estado inicial.
2. Arquitetura reutilizada.
3. Arquitetura implementada.
4. Templates criados.
5. Materiais criados.
6. Detalhes criados.
7. Mapas de relevo.
8. Processo de geração.
9. Processo de validação.
10. Determinismo.
11. Quantidade de receitas.
12. Quantidade de PNGs.
13. Quantidade de candidatos distintos.
14. Quantidade de variações menores.
15. Quantidade de duplicatas.
16. Quantidade integrada ao idle.
17. Quantidade integrada à corrida C1.
18. Quantidade de recursos isolados.
19. Problemas artísticos.
20. Trabalho manual necessário.
21. Alterações de código.
22. Testes executados.
23. Evidências.
24. Commits locais.
25. Estado do Git.
26. Bloqueios.
27. Decisões pendentes.
28. Confirmação de que não houve push.

Não declarar sucesso total se apenas parte do escopo estiver funcionando.

---

# 29. PROIBIÇÕES

Não reconstruir o corpo nu.

Não redesenhar o protagonista aprovado.

Não criar spritesheets completas para todas as combinações.

Não gerar dezenas de duplicatas para simular diversidade.

Não programar casos especiais por item.

Não criar segundo renderizador.

Não criar segundo relógio.

Não alterar inventário funcional.

Não alterar save v3.

Não iniciar N4-2.

Não iniciar N5.

Não completar automaticamente os seis quadros da corrida.

Não executar `git push`.

Não declarar aprovação artística sem autorização do proprietário.

---

# 30. CRITÉRIO FINAL DE SUCESSO

A N41-B será considerada tecnicamente bem-sucedida quando demonstrar que:

1. Equipamentos podem ser definidos por dados.
2. Templates podem ser reutilizados.
3. Materiais preservam volume e sombreamento.
4. Ornamentos podem ser combinados sem código específico.
5. A geração é determinística.
6. O catálogo apresenta diversidade mensurável.
7. O compositor utiliza os recursos gerados.
8. O idle funciona corretamente.
9. A corrida C1 funciona dentro do escopo limitado.
10. As limitações artísticas são documentadas.
11. O trabalho manual por item foi reduzido.
12. O projeto permanece seguro.
13. Nenhum push foi realizado antes da aprovação visual.

A fábrica não precisa produzir quarenta obras-primas imediatamente.

Ela precisa demonstrar uma arquitetura capaz de produzir equipamentos variados, tecnicamente válidos e artisticamente evolutivos, com esforço manual controlado.

**Execute somente a N41-B, gere o catálogo e as evidências, mantenha os resultados locais e aguarde minha aprovação visual antes de publicar ou iniciar a próxima etapa.**