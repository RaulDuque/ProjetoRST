# DARK CASTLE RPG — MEGA PROMPT D11
## Integração Experimental do Protagonista Modular 2D no Bevy 0.20.1

**Projeto:** Dark Castle RPG  
**Engine:** Rust 2024 / Bevy 0.20.1  
**Fase:** D11 — Integração Experimental, Atlas Modular e After-Image  
**Arte aprovada como direção:** D7-B / D8 / D9 / D10  
**Efeito candidato:** After-image B, vinho  
**Prioridade:** integração segura, compatibilidade e testes  
**Objetivo:** executar o protagonista modular dentro do jogo sem comprometer o sistema existente

---

# 1. MISSÃO

As fases D1–D10 construíram e refinaram o protagonista do Dark Castle RPG.

A D10 resolveu o problema de máscaras compartilhadas entre capa e luvas.

O relatório informa:

- Antifake aprovado para capa.
- Antifake aprovado para luvas.
- Antifake aprovado para casaco.
- Antifake aprovado para espada.
- Antifake aprovado para botas.
- Antifake aprovado para bainha.
- Zero pixels residuais nos testes.
- Troca dinâmica preservando a fase da corrida.
- After-image B recomendado.
- Idle e corrida ainda utilizam frames D2 como guia de movimento.

**Esses resultados são evidências do laboratório, não prova de integração runtime.**

Sua missão é validar e integrar experimentalmente o sistema visual no Bevy 0.20.1.

O objetivo não é substituir imediatamente a renderização atual.

O objetivo é criar uma integração reversível e verificável.

---

# 2. REGRAS INICIAIS

Antes de qualquer alteração:

1. Leia `CLAUDE.md`.
2. Leia as regras e skills do projeto.
3. Confirme branch, HEAD e upstream.
4. Execute `git status --short`.
5. Identifique alterações locais.
6. Leia os documentos de arquitetura do jogo.
7. Inspecione o renderer atual do jogador.
8. Inspecione o sistema de animação.
9. Inspecione o sistema de equipamentos.
10. Inspecione a integração atual de sprites e atlas.

Não presuma que o código esteja igual ao documentado em fases anteriores.

Não faça commit nem push.

Não use comandos destrutivos de Git.

Preserve `Md-usario/`, `saves/`, os laboratórios anteriores e os arquivos de arte originais.

**Diferentemente da D10, esta fase poderá alterar `src/` e `assets/`, mas somente após cumprir os pontos de controle e exclusivamente para a integração experimental autorizada.**

Não altere a lógica de gameplay, combate, dano, progressão ou salvamento.

---

# 3. PONTO DE CONTROLE ZERO — DISPONIBILIDADE E APROVAÇÃO VISUAL

Antes de integrar:

1. Verifique se D10 está disponível integralmente.
2. Execute `run_d10.py` em ambiente controlado.
3. Reproduza os testes antifake.
4. Confirme o relatório de zero resíduos.
5. Inspecione os PNGs e GIFs.
6. Confirme os arquivos-fonte e manifests.
7. Leia `PLANO_INTEGRACAO_BEVY.md`.
8. Identifique quais peças ainda dependem de frames completos.

O proprietário aprovou a direção D7-B e demonstrou interesse nos resultados posteriores, mas ainda não aprovou explicitamente cada sprite final da D10.

Portanto:

**Não trate a D10 inteira como arte final aprovada.**

Se a inspeção encontrar perda grave de qualidade, divergência importante ou arquivos ausentes, pare antes de alterar o jogo.

Produza um relatório de bloqueio.

Se a arte for tecnicamente utilizável e preservar a direção escolhida, prossiga com a integração experimental, mantendo o visual antigo como padrão.

---

# 4. AUDITORIA DO RENDERER EXISTENTE

Investigue o código real.

Localize:

- Entidade do jogador.
- Componentes visuais.
- Sistema de animação.
- Estados do jogador.
- Orientação esquerda/direita.
- Escala visual.
- Hurtbox.
- Hitbox.
- Sockets.
- Equipamentos.
- Ordem de desenho.
- Câmera.
- Filtragem de texturas.
- Renderização de efeitos.

Documente os sistemas encontrados.

Não invente APIs.

Não altere a arquitetura apenas porque uma implementação diferente seria mais fácil.

A integração deve respeitar a organização modular por plugins e os GameSets existentes.

---

# 5. DECISÃO DE ARQUITETURA — RUNTIME OU BAKE

Compare duas estratégias.

## Estratégia A — Composição runtime

O Bevy renderiza múltiplas camadas visuais.

Exemplos:

- Corpo.
- Casaco.
- Capa.
- Luvas.
- Botas.
- Arma.
- Escudo.
- Armadura.

Vantagens potenciais:

- Troca de equipamento imediata.
- Reutilização de animações.
- Menor número de combinações pré-geradas.

Riscos:

- Mais entidades ou draw calls.
- Complexidade de oclusão.
- Sincronização.
- Problemas de espelhamento.

## Estratégia B — Bake por frame/loadout

Um compositor gera previamente frames completos para combinações específicas.

Vantagens potenciais:

- Renderização mais simples.
- Ordem de desenho resolvida antecipadamente.
- Menor complexidade runtime.

Riscos:

- Explosão combinatória.
- Maior uso de memória.
- Trocas mais complexas.
- Necessidade de regenerar sprites.

## Estratégia C — Híbrida

Utilizar bake para camadas corporais estáveis e runtime para equipamentos que precisam ser trocados individualmente.

Avalie essa alternativa também.

**Não escolha a arquitetura apenas pela facilidade inicial.**

Considere o futuro do jogo, com muitas armas, armaduras, acessórios e animações.

Crie `DECISAO_ARQUITETURA.md` com uma recomendação fundamentada.

---

# 6. PROTÓTIPO ISOLADO

Antes de alterar a renderização normal do jogo, crie uma demonstração experimental.

Pode ser:

- Exemplo Rust separado.
- Cena de teste isolada.
- Modo de desenvolvimento.
- Feature experimental.

Escolha a abordagem menos invasiva compatível com o projeto.

A demonstração deve permitir:

1. Exibir o protagonista clássico.
2. Exibir o protagonista pesado.
3. Alternar loadouts.
4. Executar idle.
5. Executar corrida.
6. Trocar equipamentos individuais.
7. Espelhar orientação.
8. Verificar sockets.
9. Exibir uma grade de referência.
10. Comparar o resultado com o PNG do laboratório.

Não altere os controles principais do jogo.

---

# 7. CONTRATO DE ATLAS

Defina um contrato entre o compositor Python e o Bevy.

O contrato deve conter:

- Identificador do atlas.
- Dimensões.
- Layout.
- Frames.
- Camadas.
- Pivôs.
- Sockets.
- Escala.
- Transparência.
- Ordem de desenho.
- Regras de visibilidade.
- Identificadores de equipamentos.
- Referências a arquivos.
- Versão do formato.

Não duplique dados desnecessariamente.

Não deixe caminhos absolutos locais nos manifests.

Valide todos os arquivos antes do carregamento.

Se o atlas estiver incompleto, o sistema deve emitir um erro descritivo.

---

# 8. INTEGRAÇÃO COM EQUIPAMENTOS

Leia o sistema funcional existente.

Os oito slots conhecidos são:

- Weapon.
- Offhand.
- Head.
- Body.
- Gloves.
- Boots.
- Acc1.
- Acc2.

Confirme esses nomes no código.

Não crie novos slots funcionais nesta fase.

Crie um mapeamento de apresentação entre item equipado e aparência visual.

O sistema deve permitir:

- Equipar espada.
- Remover espada.
- Equipar escudo.
- Equipar elmo.
- Trocar armadura.
- Trocar luvas.
- Trocar botas.
- Alternar capa, quando representada por equipamento ou configuração visual.

Não altere o Save V3.

Não altere a serialização de inventário.

Não crie novos itens de gameplay sem autorização.

---

# 9. SINCRONIZAÇÃO DE ANIMAÇÕES

O visual modular deve acompanhar o estado real do jogador.

Estados relevantes:

- Idle.
- Corrida.
- Caminhada, se disponível.
- Pulo.
- Queda.
- Guarda.
- Esquiva.
- Ataque.
- Conjuração.
- Dano.
- Morte.

**A D11 não precisa produzir sprites novos para todos esses estados.**

Para estados sem arte modular:

- Preserve a apresentação anterior.
- Documente o fallback.
- Evite desaparecer com o jogador.
- Não misture escalas incompatíveis.
- Não apresente o fallback como animação modular concluída.

Não altere o funcionamento dos estados.

A sincronização visual não deve afetar o gameplay.

---

# 10. TESTE DE TROCA DURANTE A CORRIDA

Este teste é obrigatório.

Durante uma corrida contínua:

1. Começar com loadout clássico.
2. Trocar botas.
3. Trocar luvas.
4. Equipar armadura.
5. Equipar elmo.
6. Equipar escudo.
7. Trocar espada.
8. Voltar ao clássico.

Verifique:

- Fase do ciclo preservada.
- Posição do personagem preservada.
- Orientação preservada.
- Escala preservada.
- Sockets corretos.
- Ausência de teletransporte visual.
- Ausência de reinício do ciclo.
- Ausência de alteração de colisão.

Não reinicie a animação quando um equipamento mudar.

---

# 11. AFTER-IMAGE B — VINHO

A D10 recomendou a variante B.

Crie um protótipo experimental de rastro visual.

O efeito deve:

- Capturar a aparência completa do protagonista.
- Incluir os equipamentos visíveis.
- Preservar a pose do instante capturado.
- Preservar a orientação.
- Utilizar tons vinho/violeta.
- Desaparecer gradualmente.
- Possuir duração configurável.
- Possuir frequência configurável.
- Ter limite máximo de instâncias.
- Ser desativável.
- Não interferir em colisão ou dano.

Não copie diretamente o efeito visual de um personagem de outra franquia.

Use uma identidade própria do Dark Castle RPG.

## Política de ativação

Inicialmente, permita ativar o efeito somente no laboratório.

Não ligue automaticamente em todos os movimentos.

Teste separadamente:

- Corrida.
- Esquiva.
- Dash futuro, se houver suporte.
- Técnicas especiais futuras.

Não implemente mecânicas novas apenas para demonstrar o efeito.

---

# 12. COMPATIBILIDADE COM BEVY 0.20.1

Verifique as APIs reais.

Não copie exemplos antigos sem adaptação.

Investigue:

- Sprite.
- TextureAtlas.
- TextureAtlasLayout.
- Image.
- Filtragem nearest.
- Camera2d.
- Transform.
- Hierarquia de entidades.
- Temporizadores.
- Estados.
- Sistemas de atualização.
- Despawn.
- Renderização de camadas.

Confirme os nomes e comportamentos na versão instalada.

Não adicione crates sem necessidade.

Não altere o sistema de câmera.

Não altere o parallax.

Não altere o cenário V2.3.

Não ative bloom global.

---

# 13. ISOLAMENTO E REVERSIBILIDADE

A integração deve possuir uma forma explícita de ativação experimental.

Exemplo conceitual:

`visual_modular_experimental = true`

O mecanismo real pode ser:

- Feature Cargo.
- Configuração de desenvolvimento.
- Recurso de runtime.
- Alternância em uma cena de testes.

Escolha a solução que melhor respeite a arquitetura existente.

**O visual atual deve continuar sendo o padrão.**

Quando a integração experimental estiver desativada:

- O jogo deve funcionar como antes.
- A renderização antiga deve permanecer.
- Não devem existir entidades duplicadas visíveis.
- Não deve haver custo relevante de sistemas experimentais.
- Não deve haver alteração de gameplay.

Não crie dois jogadores funcionais.

Não duplique componentes de colisão.

---

# 14. TESTES DE REGRESSÃO

Execute os comandos pertinentes:

```bash
cargo fmt --check
cargo clippy --all-targets
cargo test
cargo check
```

Utilize `CARGO_BUILD_JOBS=2` quando necessário.

Não execute múltiplos processos Cargo concorrentes.

Se bibliotecas de sistema impedirem compilação, registre o bloqueio e não declare os testes aprovados.

Preserve os testes existentes.

Não reduza cobertura.

Não modifique testes apenas para esconder falhas.

---

# 15. TESTES VISUAIS

Produza capturas de:

- Protagonista antigo.
- Protagonista modular clássico.
- Protagonista modular pesado.
- Troca de equipamentos.
- Idle.
- Corrida.
- Orientação esquerda.
- Orientação direita.
- After-image B.
- Cenário causeway.

Compare:

- Altura visual.
- Posição dos pés.
- Orientação.
- Proporções.
- Qualidade dos pixels.
- Ordem de desenho.
- Movimento.
- Estabilidade.

Não use apenas screenshots de uma cena Python como prova de integração Bevy.

**As evidências runtime devem ser capturadas do executável Bevy.**

---

# 16. TESTES DE PERFORMANCE

Meça, quando o ambiente permitir:

- Quantidade de entidades visuais.
- Quantidade de sprites.
- Quantidade de texturas.
- Memória estimada dos atlas.
- Tempo de composição ou atualização.
- Quantidade de after-images simultâneas.
- Comportamento com múltiplos personagens.

Não invente métricas.

Se não conseguir medir FPS de forma confiável, informe a limitação.

Não apresente estimativas como medições.

---

# 17. CRITÉRIOS DE ACEITAÇÃO

## Arquitetura

- [ ] Contrato de atlas documentado.
- [ ] Estratégia runtime/bake justificada.
- [ ] Sistema experimental isolado.
- [ ] Visual antigo preservado.
- [ ] Sem alteração de gameplay.

## Renderização

- [ ] Clássico renderizado no Bevy.
- [ ] Pesado renderizado no Bevy.
- [ ] Idle sincronizado.
- [ ] Corrida sincronizada.
- [ ] Orientação correta.
- [ ] Sockets corretos.
- [ ] Escala consistente.

## Equipamentos

- [ ] Troca individual.
- [ ] Corpo-base compartilhado.
- [ ] Sem reinício de animação.
- [ ] Sem alterações de colisão.
- [ ] Sem duplicação do jogador.

## After-image

- [ ] Variante B implementada experimentalmente.
- [ ] Aparência completa capturada.
- [ ] Duração configurável.
- [ ] Frequência configurável.
- [ ] Limite de instâncias.
- [ ] Desativável.
- [ ] Sem efeitos sobre gameplay.

## Segurança

- [ ] Save V3 preservado.
- [ ] Câmera preservada.
- [ ] Parallax preservado.
- [ ] Cenário V2.3 preservado.
- [ ] Testes existentes preservados.
- [ ] Sem commit.
- [ ] Sem push.

Use estados:

- Concluído.
- Parcial.
- Bloqueado.
- Não iniciado.

---

# 18. ENTREGAS

Crie a documentação da fase em:

`art-src/visual-prototypes/d11_integracao_experimental/`

Estrutura sugerida:

```text
d11_integracao_experimental/
├── README.md
├── AUDITORIA_RENDERER.md
├── AUDITORIA_D10.md
├── DECISAO_ARQUITETURA.md
├── CONTRATO_ATLAS_RUNTIME.md
├── MAPEAMENTO_EQUIPAMENTOS.md
├── CONTRATO_ANIMACOES.md
├── AFTER_IMAGE_DESIGN.md
├── TESTES_REGRESSAO.md
├── TESTES_PERFORMANCE.md
├── LIMITACOES.md
├── RELATORIO_FINAL.md
├── DECISAO.md
├── previews/
├── comparacoes/
├── evidencias_bevy/
├── scripts/
└── testes/
```

O código experimental poderá ficar nos módulos apropriados de `src/`, conforme a arquitetura existente.

Os assets experimentais poderão ficar em `assets/`, organizados e identificados claramente.

Não altere arquivos além dos necessários.

---

# 19. PONTOS DE CONTROLE

## Marco A — Auditoria

Inspecionar D10, renderer e contratos.

## Marco B — Arquitetura

Decidir runtime, bake ou híbrido.

## Marco C — Protótipo isolado

Renderizar o clássico no Bevy.

## Marco D — Equipamentos

Renderizar o pesado e permitir trocas.

## Marco E — Animação

Sincronizar idle e corrida.

## Marco F — After-image

Implementar variante B experimental.

## Marco G — Regressão

Executar testes e verificar reversibilidade.

Se o Marco C não funcionar corretamente, não avance para efeitos.

Se a arte perder qualidade, não substitua o visual antigo.

Se houver alteração inesperada no gameplay, interrompa a integração e investigue.

---

# 20. RELATÓRIO FINAL

Apresente:

A. Estado inicial do Git.

B. Auditoria D10.

C. Auditoria do renderer.

D. Arquitetura escolhida.

E. Arquivos de código alterados.

F. Assets experimentais adicionados.

G. Contrato de atlas.

H. Integração do clássico.

I. Integração do pesado.

J. Troca de equipamentos.

K. Idle e corrida.

L. After-image B.

M. Testes de regressão.

N. Testes visuais.

O. Performance.

P. Limitações.

Q. Arquivos prioritários para revisão.

R. Estado final do Git.

S. Recomendação para D12.

Não faça commit.

Não faça push.

Não substitua o visual padrão.

Aguarde aprovação.

---

# 21. DIRETRIZ FINAL

O objetivo da D11 não é provar novamente que conseguimos gerar PNGs.

Isso já foi demonstrado no laboratório.

O objetivo agora é comprovar que o Bevy 0.20.1 consegue utilizar o protagonista modular preservando:

- Aparência D7-B/D10.
- Animações existentes.
- Equipamentos independentes.
- Escala visual.
- Orientação.
- Performance.
- Gameplay.
- Save V3.

O sistema novo deve ser experimental, desativável e reversível.

**Se a integração funcionar, teremos uma base concreta para produzir as futuras animações de combate, magias e transformações.**

**Se não funcionar, preserve o jogo e documente exatamente o bloqueio.**