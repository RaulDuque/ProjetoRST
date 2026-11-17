# DARK CASTLE RPG — MEGA PROMPT D10
## Finalização da modularidade 2D, fidelidade D7-B e laboratório de efeitos visuais

**Projeto:** Dark Castle RPG  
**Engine:** Rust 2024 / Bevy 0.20.1  
**Base artística:** D2 → D7-B → D8 → D9  
**Objetivo:** resolver os bloqueios reais da D9 e preparar a futura integração visual  
**Escopo:** laboratório de arte e estudos técnicos, sem alterações no jogo principal

---

# 1. MISSÃO

A D9 demonstrou avanços importantes na composição modular do protagonista.

O compositor clássico deixou de utilizar `full_classic_ref` diretamente e passou a montar loadouts a partir de camadas.

Os resultados conhecidos são:

- Casaco: teste de independência aprovado.
- Espada: aprovado.
- Botas: aprovado.
- Bainha: aprovado.
- Capa: parcial.
- Luvas: parcial.
- Idle e corrida: ainda utilizam quadros D2 para preservar fidelidade.
- Clássico modular: próximo ao D7-B, mas não idêntico.
- Armadura pesada: precisa continuar sendo refinada.

A missão da D10 é resolver os bloqueios restantes sem sacrificar a identidade artística.

**Não crie um novo protagonista.**

**Não retorne ao acabamento EEVEE/toon.**

**Não altere a câmera, o cenário ou a arquitetura do jogo principal.**

---

# 2. SEGURANÇA E GIT

Antes de trabalhar:

1. Leia `CLAUDE.md`.
2. Leia as regras e documentos de arquitetura.
3. Confirme branch, HEAD e upstream.
4. Execute `git status --short`.
5. Identifique alterações locais.
6. Preserve integralmente D1–D9.

Trabalhe exclusivamente em:

`art-src/visual-prototypes/d10_finalizacao_modular/`

Não modifique:

- `src/`
- `assets/`
- `saves/`
- `Md-usario/`
- `.cargo/`
- `.claude/`
- Laboratórios anteriores.

Não faça commit nem push.

Não execute comandos destrutivos de Git.

---

# 3. AUDITORIA REAL DA D9

Leia os documentos, scripts e testes da D9.

Priorize:

- `AUDITORIA_D8.md`
- `MAPA_DEPENDENCIAS.md`
- `ARQUITETURA_MODULAR.md`
- `FIDELIDADE_VISUAL.md`
- `LIMITACOES.md`
- `RELATORIO_FINAL.md`
- `DECISAO.md`
- `testes/antifake_report.json`

Inspecione:

- `comparacoes/16_resultado_final.png`
- `comparacoes/03_desmontagem_completa.png`
- `comparacoes/01_D8_vs_D9.png`
- `comparacoes/09_troca_individual.png`
- GIFs de idle e corrida.
- Código de `compose_loadout`.

Não confie apenas nos estados pass/partial.

Reproduza os testes.

Identifique precisamente quais pixels causam os resultados parciais.

Documente se as falhas são:

- Resíduos de chroma.
- Pixels compartilhados.
- Máscaras incompletas.
- Contornos duplicados.
- Problemas de alfa.
- Falta de arte sob peças removidas.
- Ordem de composição.
- Problemas de sockets.

Crie `AUDITORIA_D9.md`.

---

# 4. MARCO A — CORRIGIR CAPA E LUVAS

Essas são as prioridades máximas.

## Capa

Corrija:

- Resíduos de cor.
- Máscaras.
- Pixels compartilhados com o casaco.
- Regiões ocultas.
- Contornos.
- Oclusão.
- Remoção completa.

O protagonista sem capa precisa continuar visualmente correto.

Não apague detalhes do casaco ao remover a capa.

## Luvas

Separe:

- Luva dianteira.
- Luva traseira.
- Mãos.
- Punhos.
- Contornos compartilhados.

Permita substituir uma luva sem alterar a roupa.

Se houver pixels impossíveis de recuperar automaticamente, crie arte de reposição específica e documente-a.

Não utilize um proxy geométrico como resultado final.

---

# 5. MARCO B — FIDELIDADE PIXEL A PIXEL

Crie comparações entre:

- D7-B.
- D8 clássico.
- D9 clássico.
- D10 clássico.

Compare os frames equivalentes.

Calcule:

- Diferença de alfa.
- Diferença de silhueta.
- Diferença de cor.
- Pixels alterados.
- Regiões com perda de detalhe.
- Contornos.
- Posição dos pés.

Gere mapas de diferenças.

Não exija igualdade absoluta quando houver uma melhoria artística deliberada.

Entretanto, toda diferença relevante deve ser explicada.

O objetivo é preservar o visual aprovado, não alcançar um número artificial.

---

# 6. MARCO C — CORPO-BASE VERDADEIRAMENTE REUTILIZÁVEL

A D9 avançou na composição modular, mas idle e corrida ainda dependem dos quadros D2.

Investigue como separar as animações em:

- Corpo-base.
- Cabeça.
- Braços.
- Pernas.
- Casaco.
- Capa.
- Luvas.
- Botas.
- Armas.
- Equipamentos pesados.

Não descarte os quadros D2.

Utilize-os como referência de pose e acabamento.

O objetivo é que a mesma animação corporal aceite equipamentos diferentes.

Não crie uma animação corporal completa por loadout.

Se alguma peça exigir quadros próprios, documente a necessidade.

---

# 7. MARCO D — REFINAR ARMADURA PESADA

Preserve o design geral do cavaleiro pesado da D8/D9.

Corrija:

- Diferença de escala.
- Iluminação inconsistente.
- Contornos.
- Shading metálico.
- Elmo.
- Escudo.
- Ombreiras.
- Manoplas.
- Botas.
- Proporções.
- Encaixes durante a corrida.

O cavaleiro pesado deve parecer o mesmo protagonista utilizando armadura.

Não simplesmente uma segunda imagem de personagem.

Mostre comparações na mesma pose e escala.

---

# 8. MARCO E — ANIMAÇÃO MODULAR COMPLETA

Produza:

- Idle clássico.
- Idle pesado.
- Corrida clássica.
- Corrida pesada.
- Troca dinâmica durante corrida.

A troca dinâmica deve preservar:

- Frame atual.
- Posição dos pés.
- Orientação.
- Pivôs.
- Escala.
- Corpo-base.
- Estado da animação.

Não reinicie a animação quando trocar equipamentos.

Não substitua a composição por um sprite completo diferente.

---

# 9. MARCO F — LABORATÓRIO DE AFTER-IMAGE

Além da finalização modular, crie um estudo visual separado de rastro fantasmagórico.

Esse estudo não deve alterar o jogo principal.

## Objetivo

Investigar um efeito visual para:

- Corrida rápida.
- Esquiva.
- Dash futuro.
- Técnicas especiais futuras.
- Transformações futuras.

Não copie diretamente a identidade visual ou os efeitos de personagens protegidos de outros jogos.

Crie uma linguagem visual própria para Dark Castle RPG.

## Propostas

Produza três variantes:

**A — Rastro espectral frio**

Silhuetas azuladas translúcidas.

**B — Rastro vinho e violeta**

Silhuetas compatíveis com a paleta gótica do protagonista.

**C — Rastro de energia arcana**

Silhuetas com brilho discreto e dissipação.

Avalie:

- Legibilidade.
- Fidelidade ao sprite.
- Transparência.
- Persistência.
- Densidade.
- Custo de renderização.
- Compatibilidade com armaduras e capas.

## Regra fundamental

O rastro deve representar a aparência visual completa do personagem equipado.

Não capture somente o corpo e deixe espada, escudo ou capa de fora.

Se o efeito exigir composição prévia das camadas, documente a arquitetura.

Não implemente componentes ECS nesta fase.

Entregue GIFs simulados no laboratório.

---

# 10. MARCO G — ESTUDO DE INTEGRAÇÃO BEVY

Produza um documento técnico, sem modificar o código do jogo.

Investigue a versão real do Bevy utilizada no projeto.

Verifique:

- Configuração atual de filtragem de texturas.
- Configuração da câmera.
- Escala de sprites.
- Atlas e layout.
- Componentes visuais do jogador.
- Ordem de desenho.
- Sistema de equipamentos.
- Sistema de animação.
- Renderização de múltiplas camadas.
- Orientação e espelhamento.
- Custo de renderização.
- Possibilidade de capturar uma composição para after-image.

Não presuma que exemplos de versões antigas do Bevy sejam compatíveis com 0.20.1.

Não altere a altura atual da câmera para 240 ou 270 unidades.

Não modifique o parallax V2.3.

Não adicione crates.

Não ative bloom.

Não migre mapas para tilemaps.

Não implemente HUD novo.

Esses assuntos poderão ser tratados em fases próprias.

---

# 11. ESTUDO DE PIPELINE BLENDER → SPRITES

O Blender poderá continuar sendo usado para:

- Referências de pose.
- Anatomia.
- Movimentação.
- Renderização de máscaras auxiliares.
- Profundidade.
- Normal maps experimentais.

Mas não poderá substituir automaticamente o acabamento D7-B.

Investigue se renders auxiliares do Blender poderiam ajudar a criar:

- Máscaras de profundidade.
- Referências de articulação.
- Sombras.
- Posições de equipamentos.
- Mapas de normais.

Não altere o pipeline principal sem apresentar comparação visual.

Não presuma que `Film > Filter Size = 0.01px` esteja disponível ou produza o resultado desejado na versão instalada; verifique a API e o comportamento real antes de documentar essa técnica.

---

# 12. TESTES ANTIFAKE

Amplie os testes da D9.

Para cada equipamento:

1. Compor com a peça.
2. Compor sem a peça.
3. Compor com alternativa.
4. Verificar pixels residuais.
5. Verificar integridade do corpo-base.
6. Verificar alterações indevidas.
7. Verificar frames de idle.
8. Verificar frames de corrida.

Não considere uma peça independente apenas porque pode ser escondida.

Ela precisa poder ser substituída sem deixar resíduos e sem exigir a recriação do personagem inteiro.

---

# 13. TESTES AUTOMÁTICOS

Inclua:

- Testes de transparência.
- Testes de máscaras.
- Testes de dimensões.
- Testes de sockets.
- Testes de atlas.
- Testes de pivôs.
- Testes de composição.
- Testes de equipamentos ausentes.
- Testes de IDs inválidos.
- Testes de combinações mistas.
- Testes de animação.
- Testes de continuidade.
- Testes de reprodução determinística.
- Testes de independência visual.
- Testes de integridade dos GIFs.

Os testes devem informar falhas com mensagens descritivas.

Não esconda erros com fallbacks silenciosos.

---

# 14. COMPARAÇÃO NA CAUSEWAY

Utilize a captura real do cenário V2.3.

Compare:

- D7-B.
- D8 clássico.
- D9 clássico.
- D10 clássico.
- D9 pesado.
- D10 pesado.

Mostre:

- Escala real.
- Ampliação nearest-neighbor.
- Fundo claro.
- Fundo escuro.
- Cena gótica.

Não altere a câmera ou o cenário.

---

# 15. ENTREGAS

Crie:

```text
art-src/visual-prototypes/d10_finalizacao_modular/
├── README.md
├── AUDITORIA_D9.md
├── CONTRATO_CAMADAS.md
├── CONTRATO_ANIMACOES.md
├── FIDELIDADE_VISUAL.md
├── ESTUDO_AFTER_IMAGE.md
├── ESTUDO_BLENDER_AUXILIAR.md
├── PLANO_INTEGRACAO_BEVY.md
├── LIMITACOES.md
├── RELATORIO_FINAL.md
├── DECISAO.md
├── fontes/
├── atlas/
├── mascaras/
├── equipamentos/
├── animacoes/
├── efeitos/
│   └── after_image/
├── previews/
├── comparacoes/
├── scripts/
└── testes/
```

Evite arquivos redundantes.

---

# 16. PRANCHAS OBRIGATÓRIAS

Produza, conforme os marcos concluídos:

1. `01_D9_vs_D10.png`
2. `02_mascaras_corrigidas.png`
3. `03_capa_independente.png`
4. `04_luvas_independentes.png`
5. `05_desmontagem_final.png`
6. `06_corpo_base.png`
7. `07_classico_refinado.png`
8. `08_pesado_refinado.png`
9. `09_idle_comparacao.png`
10. `10_run_comparacao.png`
11. `11_troca_dinamica.png`
12. `12_after_image_variantes.png`
13. `13_causeway_escala_real.png`
14. `14_resultado_final.png`

GIFs:

- `idle_classico.gif`
- `idle_pesado.gif`
- `run_classico.gif`
- `run_pesado.gif`
- `troca_dinamica.gif`
- `after_image_A.gif`
- `after_image_B.gif`
- `after_image_C.gif`

Não gere arquivos vazios para preencher a lista.

---

# 17. CRITÉRIOS DE ACEITAÇÃO

## Arte

- [ ] D7-B preservado.
- [ ] Clássico consistente.
- [ ] Pesado consistente.
- [ ] Sem perda grave de detalhes.
- [ ] Escala real respeitada.

## Modularidade

- [ ] Capa independente.
- [ ] Luvas independentes.
- [ ] Casaco independente.
- [ ] Botas independentes.
- [ ] Espada independente.
- [ ] Bainha independente.
- [ ] Armadura substituível.
- [ ] Elmo independente.
- [ ] Escudo independente.
- [ ] Sem resíduos de pixels.
- [ ] Sem buracos no corpo-base.

## Animação

- [ ] Idle modular.
- [ ] Corrida modular.
- [ ] Corpo-base compartilhado.
- [ ] Equipamentos acompanham sockets.
- [ ] Troca dinâmica sem reiniciar o ciclo.
- [ ] Capa animada corretamente.

## Estudos futuros

- [ ] Três variantes de after-image.
- [ ] Avaliação de custo visual.
- [ ] Plano de integração compatível com Bevy 0.20.1.
- [ ] Blender documentado como ferramenta auxiliar.

## Segurança

- [ ] D1–D9 preservados.
- [ ] Jogo principal intocado.
- [ ] Save V3 intocado.
- [ ] Sem commit.
- [ ] Sem push.

Use os estados:

- Concluído.
- Parcial.
- Bloqueado.
- Não iniciado.

Não marque como concluído um item sem evidência.

---

# 18. PONTOS DE PARADA

Execute em ordem:

**Marco A:** corrigir capa e luvas.

**Marco B:** validar fidelidade D7-B.

**Marco C:** consolidar corpo-base e animações.

**Marco D:** refinar armadura pesada.

**Marco E:** comprovar modularidade em movimento.

**Marco F:** criar protótipos after-image.

**Marco G:** documentar integração futura.

Se os marcos A ou B falharem, não avance para efeitos visuais.

Se a qualidade artística piorar, preserve o resultado anterior e explique o bloqueio.

---

# 19. RELATÓRIO FINAL

Informe:

A. Estado inicial do Git.

B. Auditoria D9.

C. Causas das falhas de capa e luvas.

D. Correções realizadas.

E. Resultado dos testes antifake.

F. Fidelidade visual D7-B.

G. Estado do corpo-base.

H. Estado do atlas.

I. Estado dos sockets.

J. Clássico.

K. Pesado.

L. Idle.

M. Corrida.

N. Troca dinâmica.

O. Protótipos after-image.

P. Estudo de integração Bevy.

Q. Limitações.

R. Arquivos prioritários para revisão.

S. Estado final do Git.

T. Recomendação para D11.

Não faça commit.

Não faça push.

Não integre ao jogo.

Aguarde aprovação.

---

# 20. DIRETRIZ FINAL

A D10 não é uma fase para criar mais um protagonista.

É uma fase para finalizar a modularidade do personagem que já foi escolhido.

A prioridade é:

**Fidelidade visual → separação real → animação modular → refinamento da armadura → efeitos experimentais.**

O after-image é um estudo adicional, não uma justificativa para deixar a modularidade incompleta.

O Blender permanece como ferramenta auxiliar.

O Bevy permanece intocado.

O protagonista D7-B/D8 continua sendo a referência artística.

**Não declare sucesso técnico completo enquanto capa, luvas ou outras peças dependerem de composições que impeçam sua substituição real.**