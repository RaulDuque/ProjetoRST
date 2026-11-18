# DARK CASTLE RPG — MEGA PROMPT
# FASE 3 — PROTAGONISTA B: IDENTIDADE DEFINITIVA, PIXEL ART CONSISTENTE E ANIMAÇÕES PILOTO

## 0. PAPEL E MISSÃO

Você é o responsável técnico e artístico pela próxima fase do Dark Castle RPG, desenvolvido em Rust 2024 com Bevy 0.20.1.

O proprietário aprovou a direção visual do protagonista:

**Opção B detalhada como identidade principal, com a legibilidade das poses e dos movimentos da opção A.**

O jogo possui cenários góticos V2.3 aprovados e sistemas de combate V2.4 em desenvolvimento.

A Fase D2 entregou conceitos, animações piloto e comparações, mas ainda existem problemas de consistência entre os quadros.

**Sua missão nesta fase é criar uma versão pixel art consistente do protagonista B, preparar três animações piloto de qualidade superior e comprovar sua viabilidade técnica sem substituir a arte publicada.**

Não trate esta tarefa como geração indiscriminada de imagens. Trabalhe como uma equipe de direção artística, animação 2D, engenharia de ferramentas e controle de qualidade.

O resultado deve permitir ao proprietário decidir se o personagem B está pronto para avançar à produção completa.

---

## 1. AUDITORIA OBRIGATÓRIA DO PROJETO

Antes de qualquer alteração:

1. Confirme a branch e o HEAD.
2. Confirme a sincronização com o remoto.
3. Execute `git status --short`.
4. Identifique alterações e arquivos não rastreados.
5. Preserve integralmente o trabalho preexistente.
6. Leia as regras em `CLAUDE.md` e a documentação relevante.
7. Verifique as ferramentas disponíveis para geração e processamento de arte.

Referência publicada conhecida:

- Repositório: `RaulDuque/ProjetoRST`
- Branch: `art/v1.2-protagonist`
- Último commit informado: `bb14554`

O HEAD é uma referência esperada, não uma autorização para descartar alterações posteriores.

**Não execute `git reset --hard`, `git clean`, force push ou operações destrutivas.**

Não modifique:

- `src/`
- `assets/`
- `saves/`
- `.cargo/config.toml`
- `.claude/`
- `Md-usario/`
- Os protótipos D1 e D2 originais.

Não altere Save V3, gameplay, combate, física, câmera, HUD, mapas ou dados RON.

Não crie commits nem faça push nesta fase.

Se houver conflito de arquivos ou limitações relevantes, registre-as antes de prosseguir.

---

## 2. ESTUDAR AS REFERÊNCIAS REAIS

Examine as imagens, animações, documentos e contratos existentes.

### 2.1. Referências artísticas

Leia e inspecione:

- `art-src/visual-prototypes/CATALOGO.md`
- `art-src/visual-prototypes/d1_fase1/DOCUMENTO_TECNICO.md`
- `art-src/visual-prototypes/d1_fase1/poses/d1_B_poses_sheet.jpg`
- `art-src/visual-prototypes/d1_fase1/poses/A/idle_concept.jpg`
- `art-src/visual-prototypes/d1_fase1/poses/A/run_concept.jpg`
- `art-src/visual-prototypes/d1_fase1/poses/A/jump_concept.jpg`
- `art-src/visual-prototypes/d1_fase1/poses/A/fall_concept.jpg`
- `art-src/visual-prototypes/d1_fase1/equipamentos/d1_equip_variants.jpg`
- `art-src/visual-prototypes/d1_fase1/espectral/d1_spectral_sequence.jpg`
- `art-src/visual-prototypes/d2_protagonista_B/DOCUMENTO_TECNICO.md`
- `art-src/visual-prototypes/d2_protagonista_B/LIMITACOES.md`
- `art-src/visual-prototypes/d2_protagonista_B/ESTIMATIVA.md`

### 2.2. Animações D2

Inspecione todos os quadros originais, spritesheets e GIFs de:

- Idle
- Run
- Attack

Não avalie apenas a primeira imagem de cada animação.

Compare visualmente o início, meio e fim de cada ciclo.

### 2.3. Contratos técnicos

Examine os arquivos existentes que definem:

- Dimensões de células.
- Quantidade de quadros.
- Pivôs.
- Sockets.
- Escala em unidades do mundo.
- Orientação lateral.
- Tempo das animações.
- Hitbox e hurtbox.
- Regras do pipeline de publicação.

Leia especialmente:

- `assets/animations/player.ron`
- `assets/animations/player_sockets.ron`
- `assets/animations/player.publish.json`
- A documentação do contrato de assets.
- Os scripts de produção e `publish_guard`.

Não altere esses arquivos.

---

## 3. DIAGNÓSTICO DA FASE D2

Antes de gerar novos sprites, produza uma auditoria visual objetiva.

Analise:

1. Variação de altura entre quadros.
2. Mudanças na cabeça e no rosto.
3. Comprimento do cabelo.
4. Comprimento do casaco.
5. Quantidade e formato das peças de armadura.
6. Comprimento da espada.
7. Posição da bainha.
8. Cores que variam sem motivo.
9. Mudanças na silhueta.
10. Deslocamento dos pés.
11. Contornos serrilhados ou borrados.
12. Perda de detalhes ao reduzir imagens.
13. Problemas de transparência.
14. Oscilação da posição da câmera nas prévias.
15. Incompatibilidades com sockets e colisão.

Classifique cada problema como:

- Crítico.
- Importante.
- Cosmético.

Para cada problema, indique o quadro afetado e a solução proposta.

Não confunda deformação intencional de animação com erro de identidade.

---

## 4. CRIAR UMA REFERÊNCIA-MESTRE PIXEL ART

Crie uma nova referência-mestre que servirá como fonte de verdade para as animações.

Identidade obrigatória:

- Aristocrata gótico.
- Cabelo curto e escuro.
- Casaco comprido.
- Armadura detalhada.
- Capa vinho/vermelho escuro.
- Botas resistentes.
- Espada e bainha.
- Silhueta lateral bem definida.
- Proporções estáveis.

A direção deve ser predominantemente B, sem perder a clareza das poses A.

### 4.1. Duas versões técnicas

Produza duas alternativas do mesmo personagem:

**B1 — Detalhe equilibrado**

Menos detalhes internos, silhueta forte e boa legibilidade na câmera atual.

**B2 — Detalhe elevado**

Mais detalhes de armadura, casaco e acessórios, mas sem comprometer a leitura do personagem.

As duas devem preservar a mesma identidade, anatomia e proporções.

Não gere dois personagens completamente diferentes.

### 4.2. Paleta controlada

Defina uma paleta documentada para:

- Pele.
- Cabelo.
- Metal.
- Couro.
- Tecido.
- Capa.
- Sombra.
- Luz.
- Contorno.

Use cores consistentes em todos os quadros.

### 4.3. Fundo e transparência

Os sprites técnicos devem ter transparência verdadeira.

Não mantenha fundo magenta, preto ou branco dentro dos sprites finais.

Use fundos de contraste apenas nas pranchas de apresentação.

---

## 5. ESCALA E LEGIBILIDADE

A escala do jogo é uma restrição técnica, não uma sugestão.

Referências conhecidas:

- Altura visual aproximada do personagem: 52 unidades.
- Hurtbox existente: 26 × 46 unidades.
- Altura de visão da câmera V2.3: 405 unidades.
- O jogo já utiliza sprites de diferentes tamanhos de célula.

Verifique esses números diretamente no projeto antes de aplicá-los.

Produza comparações nas seguintes condições:

1. Personagem isolado em escala 1×.
2. Ampliação nearest-neighbor 3×.
3. Cena real da V2.3.
4. Fundo claro.
5. Fundo escuro.
6. Personagem com espada.
7. Personagem com capa.

A qualidade deve ser julgada principalmente na escala real de gameplay.

**Não escolha automaticamente a versão mais detalhada se ela perder legibilidade.**

---

## 6. PILOTO DE ANIMAÇÃO — IDLE

Produza um idle com a contagem e a temporização exigidas pelo contrato atual, após confirmá-las no projeto.

A documentação D2 informa 12 quadros a 12 fps. Verifique isso.

Movimentos:

- Respiração sutil.
- Pequena variação dos ombros.
- Capa com movimento discreto.
- Peso corporal estável.
- Pés fixos no chão.
- Espada e bainha estáveis.

O ciclo deve retornar naturalmente ao primeiro quadro.

Não use zoom, deslocamento da câmera ou transformação global da imagem para simular respiração.

Cada quadro precisa ser uma pose válida do mesmo personagem.

### Testes

- Loop contínuo.
- Primeiro e último quadros coerentes.
- Pivô estável.
- Identidade constante.
- Sem saltos visuais.
- Sem mudanças aleatórias no equipamento.

---

## 7. PILOTO DE ANIMAÇÃO — CORRIDA

Produza o ciclo de corrida conforme o contrato real.

A documentação D2 informa 10 quadros a 24 fps. Verifique.

A corrida precisa demonstrar:

- Alternância correta das pernas.
- Apoio e impulsão.
- Movimento de braços.
- Movimento da capa.
- Peso corporal.
- Leitura clara da direção.
- Ciclo sem teletransporte dos pés.

Não crie apenas uma animação de caminhada acelerada.

### Testes

- Ciclo completo.
- Pés em contato coerente com o chão.
- Sem deformações involuntárias.
- Arma e bainha acompanhando o corpo.
- Capa sem atravessar excessivamente o corpo.
- Silhueta legível no cenário V2.3.

---

## 8. PILOTO DE ANIMAÇÃO — ATAQUE BÁSICO

Produza sete quadros ou a contagem exigida pelo contrato confirmado.

Respeite os tempos atuais do gameplay.

Referências:

- `attack_time = 0,28 s`
- Janela ativa padrão = 60% do tempo.
- Iron Sword sem alteração de balanceamento.

**Atenção: o ataque atual pode iniciar sua hitbox no primeiro quadro.**

Portanto, não assuma que a antecipação visual de dois quadros representa automaticamente um startup mecânico.

Diferencie explicitamente:

- Fases visuais.
- Fases mecânicas.
- Momento em que a hitbox causa dano.
- Momento de recuperação.

Não modifique gameplay para encaixar a arte.

### Movimento

O ataque deve demonstrar:

- Preparação corporal.
- Transferência de peso.
- Movimento da espada.
- Pose de impacto.
- Recuperação.
- Retorno consistente à postura normal.

Evite espada mudando de comprimento ou posição sem explicação.

### Testes

- Sequência temporal correta.
- Espada conectada à mão.
- Arma não atravessa o próprio personagem de maneira incorreta.
- Orientação lateral correta.
- Pivô consistente.
- Compatibilidade com sockets.
- Comparação com a hitbox atual.

---

## 9. EQUIPAMENTOS E CAMADAS

A referência de equipamentos D1 foi bem recebida pelo proprietário.

Não implemente todos os equipamentos agora.

Produza uma prova de conceito com:

1. Corpo-base.
2. Armadura leve.
3. Armadura pesada.
4. Capa.
5. Espada.
6. Escudo.

Mostre como os equipamentos podem ser trocados sem redesenhar o corpo inteiro.

Avalie tecnicamente:

- Overlays.
- Sockets.
- Camadas.
- Oclusão.
- Recortes de armadura.
- Variações de cor.
- Diferenças de silhueta.

Não afirme que uma armadura pesada pode ser feita apenas com tint se a geometria exigir outra silhueta.

Documente quais equipamentos precisarão de sprites próprios.

---

## 10. COMPATIBILIDADE COM O BLENDER

Não descarte o pipeline Blender sem uma avaliação técnica.

Compare três métodos:

**Método 1 — Pixel art manual:** desenhar e ajustar os quadros.

**Método 2 — Blender + limpeza pixel:** renderizar poses coerentes e finalizar manualmente.

**Método 3 — Híbrido:** Blender como guia de pose e proporção, pixel art como resultado final.

Considere:

- Consistência anatômica.
- Quantidade de animações futuras.
- Equipamentos.
- Armas.
- Sockets.
- Custo de manutenção.
- Facilidade de correção.
- Qualidade em escala real.

Recomende um método com base nos testes, não apenas em preferência estética.

Se a geração automática produzir inconsistências, não esconda o problema.

Não declare pixel art manual realizada se o resultado for apenas downsample de imagens geradas.

---

## 11. TRANSFORMAÇÃO ESPECTRAL

A aparência espectral atual não agradou ao proprietário.

A direção aprovada é a sequência conceitual D1, com transformação visual significativa.

Nesta fase:

- Crie um estudo do personagem B normal.
- Mostre o início da dissolução.
- Mostre a forma espectral completa.
- Mostre a recuperação da forma humana.

A forma espectral deve ter silhueta, material e comportamento próprios.

Não basta aplicar transparência e filtro azul.

Não implemente gameplay ou modifique custos de MP.

Este é um estudo visual, separado dos três clips principais.

---

## 12. INIMIGOS E CENÁRIO

Não altere os inimigos nesta fase.

Utilize os arquétipos aprovados apenas como referência para comparar escala e direção artística.

O protagonista precisa combinar visualmente com:

- Bone Sentry e futuros mortos-vivos.
- Inimigos com armadura.
- Magos.
- Criaturas voadoras.
- Cenários góticos V2.3.

Preserve integralmente os cenários existentes.

---

## 13. CONTROLE DE QUALIDADE AUTOMÁTICO

Crie verificações técnicas para os sprites produzidos, quando possível:

- Dimensões corretas.
- Quantidade de quadros.
- Canal alpha.
- Ausência de fundo magenta residual.
- Paleta consistente.
- Pivô.
- Altura visual.
- Contorno dentro da célula.
- Nomeação de arquivos.
- Ordem dos quadros.
- Duração do GIF.
- Integridade das spritesheets.

Esses testes não substituem a revisão visual humana.

Inclua também uma inspeção visual quadro a quadro.

Não use uma métrica automática como prova de que a animação é artisticamente boa.

---

## 14. ESTRUTURA DE ENTREGA

Crie uma pasta nova:

`art-src/visual-prototypes/d3_protagonista_B_refinado/`

Estrutura sugerida:

```text
d3_protagonista_B_refinado/
├── README.md
├── AUDITORIA_D2.md
├── DIRECAO_ARTISTICA.md
├── COMPARACAO_PIPELINES.md
├── LIMITACOES.md
├── DECISAO.md
├── referencia/
│   ├── B1_equilibrado/
│   └── B2_detalhado/
├── paleta/
├── animacoes/
│   ├── idle/
│   ├── run/
│   └── attack/
├── equipamentos/
├── espectral/
├── comparacoes/
├── previews/
└── testes/
```

Adapte a estrutura se houver justificativa técnica, mantendo os arquivos organizados.

Não sobrescreva D1 ou D2.

---

## 15. PRANCHAS DE COMPARAÇÃO

Entregue pelo menos:

### Comparação 1 — Identidade

CURRENT × D2 × D3-B1 × D3-B2.

### Comparação 2 — Gameplay

Os mesmos personagens no cenário real da causeway V2.3, na escala correta.

### Comparação 3 — Idle

Quadros e prévia animada.

### Comparação 4 — Corrida

Quadros e prévia animada.

### Comparação 5 — Ataque

Quadros e prévia animada, incluindo indicação das fases mecânicas reais.

### Comparação 6 — Equipamentos

Corpo-base, armadura leve, pesada, capa e escudo.

### Comparação 7 — Espectral

Humano → dissolução → espectral → retorno.

Use legendas e arquivos separados para facilitar minha revisão.

---

## 16. REGRAS DE HONESTIDADE TÉCNICA

Não confunda:

- Conceito visual com sprite final.
- Quadros gerados independentemente com animação consistente.
- Downsample com pixel art manual.
- Contagem correta com temporização correta.
- Pivô aproximado com socket validado.
- GIF visualmente interessante com animação pronta para gameplay.
- Compatibilidade teórica com integração testada.

Se não houver ferramentas adequadas para finalizar uma animação consistente, entregue um protótipo parcial de qualidade, explique o bloqueio e recomende a solução.

**É preferível um idle realmente consistente a três animações visualmente instáveis.**

---

## 17. PRIORIDADES E CRITÉRIOS DE PARADA

Trabalhe nesta ordem:

**Prioridade 1:** auditar D2 e definir a identidade visual.

**Prioridade 2:** produzir B1 e B2 em pose lateral consistente.

**Prioridade 3:** criar e validar o idle.

**Prioridade 4:** criar a corrida, somente se o idle tiver consistência aceitável.

**Prioridade 5:** criar o ataque, somente se a identidade continuar estável.

**Prioridade 6:** equipamentos e transformação espectral como estudos complementares.

Se a identidade mudar de um quadro para outro de maneira relevante, interrompa a expansão dos clips e corrija a causa.

Não gere dezenas de arquivos adicionais para mascarar a falta de qualidade.

---

## 18. VALIDAÇÕES FINAIS

Antes de encerrar:

1. Confirme que o Git não sofreu alterações fora da pasta D3.
2. Confirme que D1 e D2 estão intactos.
3. Confirme que `src/`, `assets/` e `saves/` não mudaram.
4. Confirme que nenhum commit ou push foi feito.
5. Execute os testes técnicos pertinentes aos arquivos gerados.
6. Compare os resultados com o personagem atual em escala real.
7. Documente problemas que ainda exigem trabalho manual.
8. Não declare o protótipo pronto para publicação sem cumprir os contratos do jogo.

Como o código não deve ser alterado, não execute uma compilação completa de Cargo sem necessidade técnica demonstrada.

---

## 19. RELATÓRIO FINAL OBRIGATÓRIO

Responda com as seguintes seções:

A. Estado inicial do Git.

B. Auditoria dos problemas da Fase D2.

C. Identidade visual e paleta.

D. Comparação B1 × B2.

E. Idle: quadros, qualidade e problemas.

F. Corrida: quadros, qualidade e problemas.

G. Ataque: quadros, temporização e compatibilidade.

H. Equipamentos e camadas.

I. Transformação espectral.

J. Comparação no cenário real V2.3.

K. Pixel art manual × Blender × híbrido.

L. Testes técnicos executados.

M. Arquivos gerados.

N. Limitações e riscos.

O. Recomendação de direção artística.

P. Estado final do Git.

Q. Arquivos exatos que o proprietário deve abrir para decidir.

---

## 20. AUTORIZAÇÃO E LIMITES

Está autorizado a:

- Estudar os arquivos existentes.
- Criar os protótipos D3.
- Criar scripts e testes auxiliares dentro da pasta D3.
- Gerar comparações, GIFs e documentação.
- Recomendar uma direção técnica.

Não está autorizado a:

- Substituir o protagonista do jogo.
- Modificar código Rust.
- Alterar assets publicados.
- Alterar combate, magias, equipamentos ou transformações funcionais.
- Alterar o roadmap oficial.
- Fazer commits ou push.
- Excluir trabalhos anteriores.

**Objetivo final: apresentar um protagonista B refinado, consistente e tecnicamente viável, para aprovação antes de qualquer integração ao Dark Castle RPG.**