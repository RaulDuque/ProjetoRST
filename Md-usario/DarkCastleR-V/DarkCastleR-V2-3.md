# DARK CASTLE RPG — V2.3
## Câmera, Cenários Góticos, Parallax e Atmosfera Visual

### MISSÃO

Iniciar a V2.3 do Dark Castle RPG, desenvolvido em Rust 2024, Bevy 0.20.1 e Blender, com foco em transformar a apresentação visual do mundo.

A V2.2 está concluída e publicada.

O objetivo não é criar um novo jogo, mas melhorar visualmente o mundo já existente, preservando todos os sistemas funcionais.

A gravação de gameplay analisada mostrou três problemas principais:

1. O protagonista aparece pequeno em relação à área visível.
2. Há muito espaço vazio em determinadas regiões da tela.
3. As salas ainda possuem aparência de ambientes de teste, com geometria simples e pouca identidade gótica.

Esta versão deve enfrentar esses problemas de maneira estrutural e reutilizável.

---

# 1. AUDITORIA INICIAL

Confirme:

- Branch `art/v1.2-protagonist`.
- HEAD `6db2667404c7c90632a893c59e56d4787b9c53c5`.
- Remoto sincronizado.
- Staging vazio.
- Saves intactos.
- Assets publicados íntegros.
- Tutoriais sincronizados.

Inspecione:

- Sistema de câmera.
- Dimensões da janela.
- Resolução interna.
- Escala do protagonista.
- Escala dos inimigos.
- Tamanho das salas.
- Física e colisões.
- Renderização do mundo.
- Camadas de cenário.
- Sistema de regiões.
- Transições entre salas.
- Pipeline artístico.
- HUD.
- Mapa.
- Exemplos de validação visual.

Documente os problemas reais encontrados antes de implementar.

Não altere valores apenas porque parecem melhores em teoria. Compare capturas e comportamento em execução.

---

# 2. REGRAS DE SEGURANÇA

Não execute staging, commit ou push.

Não altere:

- `Md-usario/`
- `saves/`
- `.cargo/config.toml`
- `.claude/settings.json`
- `.claude/hooks/guard.py`
- `.claude/hooks/test_guard.py`

Não sobrescreva assets publicados sem autorização específica.

Não modifique o PNG de inspiração.

Não execute `cargo clean`, `git reset --hard` ou `git clean`.

Utilize no máximo dois jobs de compilação e apenas um processo Cargo por vez.

Preserve os backups do pipeline artístico.

Não modifique o Save V3 sem necessidade comprovada.

---

# 3. CÂMERA — PRIORIDADE MÁXIMA

## Objetivo

Fazer o protagonista ficar mais legível na tela, sem prejudicar a navegação e o combate.

## Implementar ou melhorar

- Zoom configurável.
- Acompanhamento suave.
- Dead zone horizontal.
- Dead zone vertical.
- Look-ahead na direção do movimento.
- Enquadramento durante saltos.
- Enquadramento durante quedas.
- Limites por sala.
- Transição suave entre salas.
- Ajuste para diferentes resoluções de janela.

Evite uma câmera que siga cada pequeno movimento vertical do personagem, causando tremores.

Evite atrasos que impeçam o jogador de enxergar obstáculos.

## Requisitos

A câmera deve respeitar:

1. Movimento normal.
2. Corrida.
3. Pulo.
4. Queda.
5. Esquiva.
6. Ataque.
7. Transformação espectral.
8. Salas pequenas.
9. Salas grandes.
10. Salas verticais.
11. Entrada e saída por portas.
12. Morte e renascimento.

Mantenha coordenadas físicas e colisões independentes do zoom visual.

Crie uma configuração declarativa para parâmetros de câmera, com valores padrão seguros.

## Comparação

Gere capturas antes e depois nas mesmas salas, resoluções e posições.

Não escolha o zoom final usando apenas uma sala.

---

# 4. SISTEMA VISUAL DE CENÁRIOS

## Objetivo

Separar a representação artística das salas de sua geometria funcional.

A geometria atual deve continuar responsável por:

- Colisões.
- Plataformas.
- Portas.
- Passagens secretas.
- Alavancas.
- Baús.
- Paredes quebráveis.
- Barreiras espectrais.
- Posições de inimigos.

A nova camada visual deve ser responsável por:

- Paredes decorativas.
- Pisos.
- Arcos.
- Colunas.
- Janelas.
- Correntes.
- Bandeiras.
- Estátuas.
- Ruínas.
- Detalhes arquitetônicos.

Não altere colisões para acomodar decoração sem necessidade.

## Arquitetura

Investigue o sistema existente e implemente a menor extensão reutilizável possível.

Prefira definições por dados, usando o formato já adotado no projeto.

Não crie um segundo sistema de salas.

Não duplique a lógica de carregamento de regiões.

---

# 5. IDENTIDADE GÓTICA DO CASTELO

Defina uma linguagem visual coerente para o Dark Castle RPG.

Elementos desejados:

- Pedra envelhecida.
- Arcos góticos.
- Pilares.
- Janelas altas.
- Grades de ferro.
- Correntes.
- Escadarias.
- Tochas.
- Estátuas.
- Ruínas.
- Portas ornamentadas.
- Névoa.
- Vegetação morta.
- Detalhes de arquitetura medieval sombria.

Use uma paleta consistente com o estilo já aprovado do protagonista.

Evite cenários excessivamente coloridos.

Evite fundos tão escuros que prejudiquem a leitura das plataformas.

O personagem, inimigos, itens e perigos precisam continuar distinguíveis do cenário.

Não copie diretamente assets de Castlevania ou de outros jogos.

---

# 6. PARALLAX

Implemente ou amplie um sistema de profundidade com camadas.

Sugestão de organização:

- Fundo distante.
- Arquitetura distante.
- Arquitetura intermediária.
- Plano jogável.
- Decoração frontal.

O movimento das camadas deve ser coerente com a câmera.

## Requisitos

- Sem costuras visíveis entre segmentos repetidos.
- Sem vazamentos de fundo nas bordas das salas.
- Sem elementos decorativos confundidos com plataformas.
- Sem oscilações durante mudanças de direção.
- Sem saltos visuais nas transições.
- Compatível com salas verticais.

Não use parallax exagerado.

O efeito deve melhorar a percepção de profundidade sem prejudicar a jogabilidade.

---

# 7. ILUMINAÇÃO E ATMOSFERA

Investigue a solução de iluminação compatível com a arquitetura atual do Bevy.

Não adicione dependências pesadas sem justificativa.

Priorize uma implementação incremental.

Elementos possíveis:

- Luz ambiente por região.
- Tochas com variação sutil.
- Brilho de janelas.
- Névoa em camadas.
- Partículas ambientais.
- Destaque de objetos interativos.

## Regras

- Não esconder inimigos.
- Não esconder portas.
- Não esconder plataformas.
- Não esconder perigos.
- Não reduzir excessivamente o contraste.
- Não prejudicar o desempenho.

Permita desligar efeitos decorativos caros durante testes e diagnósticos.

Não introduza pós-processamento complexo apenas para cumprir uma lista de funcionalidades.

---

# 8. DIFERENCIAÇÃO DAS REGIÕES

O jogo já possui múltiplas regiões.

Não aplique a mesma decoração a todas elas.

Defina uma identidade visual para pelo menos três ambientes já implementados.

Sugestão:

**Castelo principal:** arquitetura monumental, pedra fria, colunas e iluminação contida.

**Galerias esquecidas:** ruínas, passagens antigas, correntes e elementos de abandono.

**Vila das Cinzas:** construções habitadas, comércio, iluminação mais quente e detalhes urbanos.

Use os nomes e IDs reais encontrados no projeto.

Não invente novas regiões.

Não altere a progressão existente.

---

# 9. IMPLEMENTAÇÃO PILOTO

Antes de converter o mundo inteiro, escolha três salas existentes:

1. Uma sala horizontal de exploração.
2. Uma sala vertical ou com plataformas.
3. Uma sala da Vila das Cinzas.

Implemente nelas o novo sistema visual.

Valide:

- Câmera.
- Zoom.
- Parallax.
- Decoração.
- Iluminação.
- Colisões.
- Inimigos.
- Interações.
- Transições.

Após validar as três salas, avalie se o sistema pode ser aplicado com segurança às demais.

Não produza dezenas de salas decoradas antes de validar a arquitetura.

---

# 10. HUD E RESOLUÇÃO

A V2.2 corrigiu a sobreposição das notificações.

Preserve essa correção.

Teste a interface com:

- Câmera aproximada.
- Diferentes tamanhos de janela.
- Inventário.
- Loja.
- Mapa.
- Diálogos.
- Notificações.
- Tela de morte.

O zoom da câmera não deve aumentar indevidamente os elementos do HUD.

Se for necessário ajustar a interface, mantenha as mudanças pequenas e testáveis.

---

# 11. COMPATIBILIDADE

A V2.3 não deve quebrar:

- Save V3.
- Migração V1/V2.
- Economia.
- NPCs.
- Lojas.
- Inventário.
- Equipamentos.
- Armas visíveis.
- Âncoras.
- Combate.
- Magias.
- Forma espectral.
- Mapa.
- Segredos.
- Baús.
- Portas.
- Alavancas.
- Progressão.

As novas definições visuais devem ter valores padrão seguros.

Saves antigos devem continuar carregando.

---

# 12. TESTES

Execute:

```bash
export CARGO_BUILD_JOBS=2

cargo fmt --all -- --check
cargo check -j 2
cargo clippy --all-targets -j 2 -- -D warnings
cargo test -j 2
```

Execute também:

- Testes Python do pipeline artístico.
- Testes dos hooks.
- Testes dos tutoriais.
- World Report.
- Autoplay.
- Rotas reais do mundo.
- Rota da vila.
- Rota espectral.
- Rota de equipamentos.
- Visual Check.

Crie testes específicos para:

1. Limites da câmera.
2. Zoom.
3. Dead zones.
4. Look-ahead.
5. Transições.
6. Salas verticais.
7. Parallax.
8. Carregamento de decoração.
9. Dados visuais ausentes.
10. Compatibilidade com salas antigas.
11. Ausência de alteração nas colisões.
12. Integridade do Save V3.
13. Escala do HUD.
14. Ausência de crescimento contínuo de entidades.
15. Descarregamento correto de elementos de cenário.

---

# 13. VALIDAÇÃO VISUAL

Crie uma demonstração de câmera e cenários, reutilizando os exemplos existentes quando possível.

Gere capturas comparativas.

Exija:

- Mesma resolução.
- Mesma sala.
- Mesma posição.
- Mesmo estado do personagem.
- Identificação de antes e depois.

Inspecione as imagens efetivamente.

Não considere uma captura aprovada apenas porque o arquivo foi gerado.

Verifique:

- Legibilidade do protagonista.
- Visibilidade das plataformas.
- Enquadramento.
- Profundidade.
- Contraste.
- Composição.
- HUD.
- Coerência arquitetônica.
- Ausência de artefatos.
- Ausência de elementos flutuando.

---

# 14. DESEMPENHO

Meça:

- Tempo médio por quadro.
- Tempo máximo observado.
- Quantidade de entidades.
- Quantidade de sprites.
- Quantidade de texturas.
- Memória, se houver medição confiável.
- Tempo de transição entre salas.

Compare cenários equivalentes com e sem efeitos decorativos.

Não afirme melhoria de desempenho quando a diferença estiver dentro do ruído experimental.

Evite recriar todas as entidades decorativas a cada frame.

---

# 15. DOCUMENTAÇÃO E TUTORIAIS

Crie:

`docs/WORLD_VISUAL_V2_3.md`

Documente:

- Arquitetura visual.
- Configuração da câmera.
- Parallax.
- Decoração.
- Iluminação.
- Definições de dados.
- Exemplos reais.
- Testes.
- Performance.
- Limitações.
- Próximas melhorias.

Atualize os documentos técnicos afetados.

Crie tutoriais em `docs/tutorias/` para:

- Configurar câmera.
- Configurar zoom.
- Decorar uma sala.
- Criar fundos em parallax.
- Configurar atmosfera de uma região.
- Testar cenários visuais.

Use a numeração posterior ao tutorial 29.

Siga o padrão existente de nove seções.

Não escreva em `Md-usario/`.

Liste todos os tutoriais criados ou alterados para sincronização manual.

---

# 16. CRITÉRIOS DE ACEITAÇÃO

A V2.3 estará concluída quando:

1. A câmera tiver enquadramento melhor e comprovado.
2. O protagonista estiver mais legível.
3. A câmera funcionar em salas horizontais e verticais.
4. O parallax funcionar sem artefatos críticos.
5. Pelo menos três salas tiverem decoração gótica funcional.
6. As três salas representarem situações diferentes.
7. A decoração não alterar indevidamente a física.
8. A iluminação não prejudicar a jogabilidade.
9. O HUD continuar legível.
10. O Save V3 continuar compatível.
11. As rotas reais continuarem passando.
12. Os testes automatizados passarem.
13. O desempenho for medido.
14. A documentação estiver atualizada.
15. Os tutoriais estiverem validados.
16. Os arquivos protegidos permanecerem intactos.

Se alguma funcionalidade depender de substituir assets publicados, gere candidatos e solicite autorização.

Não declare concluída uma funcionalidade ainda não integrada ao jogo normal.

---

# 17. RELATÓRIO FINAL

Apresente:

A. Estado inicial do Git.

B. Diagnóstico da câmera anterior.

C. Configuração final da câmera.

D. Comparações antes/depois.

E. Arquitetura de cenários.

F. Parallax.

G. Iluminação.

H. Regiões contempladas.

I. Salas modificadas.

J. Compatibilidade das colisões.

K. HUD.

L. Testes Rust.

M. Testes Python.

N. Testes visuais.

O. Capturas inspecionadas.

P. Desempenho.

Q. Save V3.

R. Arquivos criados.

S. Arquivos modificados.

T. Tutoriais.

U. Segurança e hashes.

V. Estado final do Git.

W. Limitações.

X. Próximos passos.

Não faça commit nem push.

Não inicie a V2.4.

**Objetivo final: fazer o Dark Castle RPG começar a parecer um Metroidvania gótico de verdade, preservando toda a estrutura funcional construída até a V2.2.**