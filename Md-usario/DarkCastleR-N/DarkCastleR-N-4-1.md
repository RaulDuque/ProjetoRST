# DARK CASTLE RPG — MEGA PROMPT N4-1
## MODULARIZAÇÃO DA CORRIDA E SINCRONIZAÇÃO DAS CAMADAS

**Projeto:** Dark Castle RPG  
**Branch:** `art/v1.2-protagonist`  
**Engine:** Bevy 0.20.1  
**Linguagem:** Rust 2024  
**Estado:** N4-A implementada; revisões artísticas N4-B aprovadas pelo proprietário  
**Escopo:** N4-1 — corrida modular experimental

---

# 0. AUTORIZAÇÃO

O proprietário avaliou a imagem da N4-B, contendo o protagonista vestido, sem cabelo, sem camisa, sem calça, sem botas, sem braceletes e apenas com o corpo-base.

**A revisão artística foi aprovada para continuidade do desenvolvimento.**

Está autorizado o início da N4-1.

Essa aprovação não significa autorização para substituir o protagonista de produção, modificar animações aprovadas ou iniciar N5.

# 1. MISSÃO

Transformar os seis quadros da corrida N3.6 em uma animação modular, utilizando a máquina de sprites implementada na N4-A e aprimorada na N4-B.

O personagem deverá correr com suas partes visuais independentes:

- Corpo anatômico.
- Cabelo traseiro.
- Cabelo frontal.
- Camisa.
- Calça.
- Botas.
- Braceletes.

Cada peça deverá acompanhar corretamente a animação, preservando a identidade visual e os detalhes da corrida original.

**Não criar uma nova animação de corrida. Modularizar a animação aprovada.**

# 2. AUDITORIA INICIAL

Antes de modificar arquivos:

1. Confirmar branch, HEAD local e remoto.
2. Inspecionar `git status`.
3. Ler `CLAUDE.md` e as regras do projeto.
4. Ler o relatório e as evidências finais da N4-B.
5. Ler os documentos da N4-A e da N3.8.
6. Inspecionar o compositor existente.
7. Identificar os seis quadros originais da corrida N3.6.
8. Confirmar os hashes da arte protegida.
9. Identificar todas as alterações artísticas aprovadas na N4-B.
10. Registrar o estado inicial.

Não sobrescrever alterações preexistentes.

# 3. REFERÊNCIA ARTÍSTICA

A referência fornecida pelo proprietário está em:

`Md-usario/assets-user/prota/base/Gemini_Generated_Image_n7ajjyn7ajjyn7aj.jpg`

Esse arquivo deve ser tratado como referência artística protegida.

Não modificar, mover, renomear ou sobrescrever.

Se o arquivo não estiver disponível, registrar a ausência e solicitar orientação antes de substituir a referência por outra.

A aparência da N4-B aprovada deve orientar a reconstrução anatômica das poses da corrida.

# 4. PRESERVAÇÃO DA CORRIDA

Manter exatamente:

- Seis quadros.
- 15 FPS.
- Ciclo de 0,4 segundo.
- Velocidade de deslocamento de 310 unidades/s.
- Canvas de 112×128.
- Pivô `(56,118)`.
- Identidade do personagem.
- Silhueta original vestido.
- Posição dos pés.
- Direção e espelhamento.

Os seis quadros aprovados não podem ser repintados.

O resultado vestido deve reproduzir os quadros originais.

# 5. IMPLEMENTAÇÃO EM DUAS ETAPAS

## Etapa C1 — Três quadros representativos

Começar pelos quadros:

- #0 — contato.
- #2 — passada aberta.
- #4 — recuperação.

Separar as peças e reconstruir a anatomia oculta em cada pose.

Criar comparações entre:

1. Original aprovado.
2. Modular vestido.
3. Modular sem cabelo.
4. Modular sem camisa.
5. Modular sem calça.
6. Modular sem botas.
7. Corpo-base.

**Não avançar automaticamente para os outros três quadros caso a prova C1 apresente defeitos artísticos importantes.**

## Etapa C2 — Seis quadros

Após a validação técnica e artística da C1 pelo proprietário, completar os seis quadros.

Não interpretar a aprovação da N4-B como aprovação automática da C1.

# 6. ANATOMIA DINÂMICA

A reconstrução anatômica deverá respeitar o movimento real.

Especial atenção a:

- Rotação dos ombros.
- Flexão dos cotovelos.
- Cruzamento dos braços.
- Movimento do quadril.
- Flexão dos joelhos.
- Extensão das pernas.
- Contato dos pés com o chão.
- Sobreposição entre membros.
- Cabelo em movimento.

Não reutilizar o corpo neutro do idle como uma figura rígida deslocada.

Cada pose precisa de anatomia coerente.

# 7. COMPOSIÇÃO POR DADOS

Reutilizar o contrato RON existente.

Não criar outro compositor.

Não criar outro controlador de animação.

Cada camada deverá possuir os quadros correspondentes à corrida.

O `VisualPlayback` continuará sendo a única fonte do quadro atual.

Todas as peças devem mudar de quadro simultaneamente.

# 8. ORDEM DE DESENHO

Verificar a ordem das camadas em cada quadro.

Braços e pernas podem passar à frente ou atrás do torso durante a corrida.

Usar as regras de ordem por quadro já previstas na arquitetura.

Não resolver cruzamentos complexos simplesmente escondendo membros.

Quando necessário, subdividir uma peça em camadas de frente e trás, mantendo-a como um único equipamento lógico.

# 9. RECOMPOSIÇÃO EXATA

Comparar cada quadro modular vestido com o quadro N3.6 original.

Gerar:

- Mapa de diferenças.
- Contagem de pixels divergentes.
- Comparação de silhueta.
- Comparação de pivô.
- Comparação de contato com o chão.

A meta é **zero diferenças nos pixels visíveis**.

Diferenças devem ser documentadas, nunca ocultadas.

# 10. REMOÇÃO DURANTE A CORRIDA

A máquina deverá demonstrar a remoção independente de cada peça enquanto a animação está em execução.

Testar:

- Cabelo removido.
- Camisa removida.
- Calça removida.
- Botas removidas.
- Braceletes removidos.
- Corpo-base.

Não permitir que apareçam buracos transparentes ou membros anatomicamente incoerentes.

# 11. TESTES DE SINCRONIZAÇÃO

Validar a animação a:

- 30 FPS de renderização.
- 60 FPS.
- 144 FPS.
- Variação de frame time.

A animação deve continuar a 15 FPS, independentemente da taxa de renderização.

Nenhuma camada pode adiantar ou atrasar em relação às outras.

# 12. EQUIPAMENTOS E SOQUETES

Preparar a marcação manual dos soquetes para:

- Mão principal.
- Mão secundária.
- Cabeça.
- Ombros.
- Costas.
- Cintura.

Não inventar coordenadas automáticas para mãos parcialmente ocultas.

Não implementar novas animações de ataque nesta fase.

Armas e escudos continuam fora da prova até que os soquetes sejam validados.

# 13. EVIDÊNCIAS

Criar:

`docs/development/n4/evidence/n4c/`

Entregar:

- Comparação original × modular.
- Quadros isolados.
- Mapas de diferenças.
- Personagem sem cabelo.
- Personagem sem camisa.
- Personagem sem calça.
- Personagem sem botas.
- Corpo-base em movimento.
- GIF da corrida modular.
- Comparação entre corrida original e modular.

Todos os GIFs devem ser gerados a partir dos quadros reais do projeto.

# 14. TESTES E REGRESSÕES

Executar:

`cargo fmt --check`

`cargo check`

`cargo clippy --all-targets --jobs 2 -- -D warnings`

`cargo test --jobs 2`

Verificar:

- Integridade dos arquivos originais.
- Compatibilidade dos manifestos.
- Sincronização de todas as camadas.
- Espelhamento.
- Pivôs.
- Ordenação.
- Ocultação.
- Ausência de regressões no idle.
- Preservação do V2.2.
- Preservação do modo N3.8.
- Preservação do save v3.
- Preservação da física e do combate.

# 15. DOCUMENTAÇÃO

Documentar em:

`docs/development/n4/`

Criar relatórios separados para:

- Auditoria inicial.
- Estratégia de separação.
- Anatomia das poses.
- Sincronização.
- Ordem das camadas.
- Diferenças visuais.
- Testes.
- Evidências.
- Riscos.
- Resultado da C1.

# 16. GATES

## Gate C1 — Prova de três quadros

- [ ] Três quadros modularizados.
- [ ] Corpo anatômico coerente em cada pose.
- [ ] Peças removíveis.
- [ ] Recomposição comparada.
- [ ] Diferenças medidas.
- [ ] Sincronização validada.
- [ ] Evidências produzidas.
- [ ] Avaliação artística do proprietário.

## Gate C2 — Corrida completa

**Bloqueado até aprovação explícita da C1.**

- [ ] Seis quadros modularizados.
- [ ] Corrida a 15 FPS.
- [ ] Peças sincronizadas.
- [ ] Recomposição validada.
- [ ] Qualidade artística aprovada.

# 17. GIT E PUBLICAÇÃO

Trabalhar na branch atual.

Criar commits pequenos e organizados.

Não fazer force push, merge ou rebase automático.

Preservar `Md-usario/` e todos os arquivos protegidos.

Publicar a C1 após os testes, sem iniciar a C2.

Confirmar HEAD local, remoto e estado do Git.

# 18. ENTREGA FINAL

Informar:

1. Estado inicial do repositório.
2. Arquitetura reutilizada.
3. Quadros modularizados.
4. Qualidade da anatomia reconstruída.
5. Peças removíveis.
6. Diferenças em relação à N3.6.
7. Testes executados.
8. Evidências produzidas.
9. Bloqueios encontrados.
10. Commits e SHA.
11. Decisões necessárias do proprietário.

**Não iniciar C2, N5 ou promoção do protagonista automaticamente.**

---

# REGRA FINAL

A máquina de sprites deve permitir que o personagem troque roupas e equipamentos sem perder a qualidade da pixel art.

A N4-1 deve provar que isso funciona não apenas em uma pose parada, mas também durante o movimento.

Preservar integralmente o trabalho artístico aprovado.

**Execute somente a N4-1, entregue as três poses modulares e aguarde aprovação antes de continuar.**