# PROMPT MESTRE — ETAPA 3: ESTABILIZAÇÃO E CORREÇÃO DA FUNDAÇÃO
## Projeto: Dark Castle RPG

Você está trabalhando no projeto Dark Castle RPG, um RPG de ação 2D / Metroidvania desenvolvido em Rust e Bevy.

Sua responsabilidade é atuar como engenheiro de software sênior, responsável pela estabilização da arquitetura, correção de problemas técnicos e preservação da integridade do projeto.

Esta etapa sucede a auditoria técnica registrada em:

`docs/AUDITORIA_E2.md`

O resumo geral do projeto está em:

`docs/RESUMO_PROJETO.md`

O briefing original da auditoria está em:

`Md-usario/E2-Auditoria-Tecnica.md`

O briefing original pode estar incompleto. Utilize o relatório da auditoria como referência dos problemas encontrados, mas confira cada problema diretamente no código antes de tomar qualquer decisão.

---

# 1. CONTEXTO TÉCNICO

Informações conhecidas do projeto:

- Linguagem: Rust 2024.
- Engine: Bevy 0.20.1.
- Pacote: `dark_castle_rpg`.
- Branch anteriormente utilizada: `master`.
- Arquitetura modular baseada em plugins.
- Regras de domínio separadas dos sistemas ECS.
- Dados configuráveis em RON.
- Testes unitários, integração e ECS headless.
- Claude Code utilizado como ferramenta de desenvolvimento.
- Commits organizados com Conventional Commits.

O projeto possui sistemas de:

- Jogador e movimentação.
- Física e colisões.
- Combate.
- Reconhecimento de comandos.
- Técnicas e habilidades.
- Magia.
- Atributos e progressão.
- Inventário e equipamentos.
- Inimigos e inteligência artificial.
- Mundo e salas.
- Interface.
- Salvamento e carregamento.

A fundação já possui funcionalidades implementadas e testadas. Portanto, não se deve reconstruir o projeto nem substituir sua arquitetura sem necessidade.

# 2. OBJETIVO GERAL

Executar um processo controlado de estabilização da fundação do jogo.

As prioridades são:

1. Proteger o ambiente de desenvolvimento contra consumo excessivo de memória e travamentos.
2. Melhorar a segurança das operações realizadas pelo Claude Code.
3. Impedir que testes sobrescrevam os saves reais do jogador.
4. Corrigir problemas de integração do combate.
5. Fortalecer a validação dos dados RON.
6. Corrigir a integridade do sistema de salvamento e carregamento.
7. Melhorar a confiabilidade dos testes.
8. Manter documentação e código sincronizados.

O objetivo não é adicionar conteúdo ao jogo nesta etapa.

O objetivo é tornar a fundação confiável para receber novas funcionalidades.

# 3. REGRA FUNDAMENTAL: TRABALHO POR FASES

O trabalho será dividido nas seguintes fases:

| Fase | Objetivo | Situação inicial |
|---|---|---|
| 0 | Inspeção e preparação | Executar |
| 1 | Proteção do ambiente de desenvolvimento | Executar agora |
| 2 | Segurança do Claude Code | Planejar |
| 3 | Isolamento e confiabilidade do autoplay | Planejar |
| 4 | Correções de integração do combate | Planejar |
| 5 | Validação dos dados RON | Planejar |
| 6 | Integridade do save/load | Planejar |
| 7 | Revisão final e encerramento | Planejar |

**IMPORTANTE: nesta execução, somente as Fases 0 e 1 estão autorizadas.**

As demais fases devem ser analisadas e planejadas, mas não implementadas.

Não interprete a existência de um plano como autorização para executar todas as correções.

Ao concluir a Fase 1, pare e apresente o relatório para aprovação.

# 4. REGRAS INEGOCIÁVEIS

Estas regras valem para todas as fases.

## 4.1. Preservação do projeto

- Não modificar arquivos dentro de `Md-usario/`.
- Não apagar arquivos existentes.
- Não remover funcionalidades.
- Não substituir sistemas existentes por implementações simplificadas.
- Não alterar versões das dependências.
- Não atualizar o Bevy.
- Não alterar mecânicas de jogo sem autorização.
- Não modificar código de gameplay durante a Fase 1.
- Não executar comandos destrutivos.
- Não executar `git reset --hard`.
- Não executar `git clean`.
- Não utilizar force push.
- Não descartar alterações do usuário.
- Não sobrescrever saves reais.
- Não criar uma segunda cópia integral do projeto para compilar.
- Não iniciar múltiplas compilações pesadas simultaneamente.

## 4.2. Antes de alterar qualquer arquivo

1. Verifique o estado do Git.
2. Identifique a branch atual.
3. Verifique se existem alterações não commitadas.
4. Identifique os arquivos que pretende modificar.
5. Leia o conteúdo atual desses arquivos.
6. Preserve alterações preexistentes.
7. Faça somente as alterações autorizadas.

Se encontrar alterações não commitadas que não foram produzidas por você, não as sobrescreva.

Se não for possível garantir a preservação, pare e informe o problema.

## 4.3. Qualidade das alterações

- Faça alterações pequenas e objetivas.
- Evite duplicação de configurações.
- Não crie abstrações desnecessárias.
- Não adicione dependências sem justificativa.
- Não esconda erros com supressões amplas.
- Não use `unwrap()` ou `expect()` em caminhos normais de execução sem justificativa.
- Não declare uma correção concluída sem testar seu comportamento.
- Não altere arquivos sem necessidade.

## 4.4. Commits

Nesta execução, não faça commits.

Apresente as alterações e aguarde autorização.

Quando os commits forem autorizados futuramente, utilize mensagens Conventional Commits e commits pequenos, independentes e validados.

# 5. FASE 0 — INSPEÇÃO E PREPARAÇÃO

Antes de qualquer modificação:

### 5.1. Verifique o Git

Execute as verificações necessárias para identificar:

- Branch atual.
- Hash do HEAD.
- Estado da árvore de trabalho.
- Alterações staged.
- Alterações unstaged.
- Arquivos não rastreados.
- Existência de worktrees.
- Últimos commits.

Não assuma que o último hash continua sendo o mesmo informado anteriormente.

O último commit conhecido era:

`9dcdfcc5759860b834c3bc45343dfde2e776ec76`

Confirme o estado atual diretamente.

### 5.2. Inspecione as instruções do projeto

Leia:

- `CLAUDE.md`
- `.claude/rules/`
- `.claude/settings.json`
- `docs/AUDITORIA_E2.md`
- `docs/RESUMO_PROJETO.md`

Confira as instruções relacionadas a:

- Compilação.
- Uso de memória.
- Diretórios temporários.
- Testes.
- Commits.
- Preservação de arquivos.
- Segurança dos comandos.

### 5.3. Inspecione o ambiente

Identifique:

- Sistema operacional.
- RAM disponível.
- Espaço em disco.
- Espaço disponível no filesystem utilizado por `/tmp`.
- Diretório real do projeto.
- Diretório `target/`.
- Variáveis `CARGO_TARGET_DIR` e `CARGO_BUILD_JOBS`, se definidas.
- Configurações locais do Cargo.
- Configurações de compilação existentes.

Não faça limpeza nesta fase.

Não remova arquivos temporários ainda.

Não execute uma compilação completa apenas para verificar o ambiente.

### 5.4. Resultado da Fase 0

Apresente um diagnóstico curto:

- Situação do Git.
- Situação do ambiente.
- Riscos encontrados.
- Arquivos que precisam ser modificados na Fase 1.
- Possíveis conflitos com configurações existentes.

Se houver alterações do usuário que possam ser afetadas, pare antes de modificar qualquer coisa.

# 6. FASE 1 — PROTEÇÃO DO AMBIENTE DE DESENVOLVIMENTO

## 6.1. Problema identificado

A auditoria encontrou um risco crítico relacionado à execução de compilações pesadas.

O ambiente utiliza `/tmp` como tmpfs, ou seja, parte de seus arquivos temporários utiliza memória RAM.

Compilações grandes do Bevy, especialmente quando executadas em diretórios temporários ou com paralelismo elevado, podem provocar consumo excessivo de memória e instabilidade do computador.

Já ocorreram travamentos durante sessões anteriores de compilação.

O problema está registrado na auditoria como:

`C1 — proteção do ambiente de desenvolvimento`.

## 6.2. Objetivo

Criar proteções permanentes para evitar que o Claude Code execute operações de compilação de maneira perigosa.

As proteções precisam estar documentadas nas instruções efetivamente utilizadas pelo Claude Code.

Não basta escrever uma recomendação no relatório.

## 6.3. Configuração de paralelismo

Analise a configuração atual do Cargo.

Se não houver uma configuração equivalente, configure o limite padrão de compilação para quatro jobs.

A configuração deve evitar que comandos comuns iniciem compilações excessivamente paralelas.

Considere utilizar:

`.cargo/config.toml`

Com configuração equivalente a:

```toml
[build]
jobs = 4
```

Antes de criar ou alterar esse arquivo:

- Verifique se já existe.
- Preserve configurações existentes.
- Não duplique se a configuração já estiver definida.
- Não substitua configurações personalizadas sem necessidade.
- Verifique se variáveis de ambiente podem sobrescrever o comportamento.

O limite de quatro jobs deve ser o padrão do projeto, não uma promessa de que toda operação consumirá pouca memória.

Não aumente o paralelismo automaticamente.

## 6.4. Diretório de compilação

O projeto deve utilizar o diretório `target/` localizado na própria raiz do projeto.

Não configure `CARGO_TARGET_DIR` para:

- `/tmp`.
- Diretórios temporários.
- Worktrees descartáveis.
- Pastas externas que sejam removidas após a compilação.

Não crie um segundo diretório de compilação para executar testes ou validar commits.

Reutilize os artefatos já existentes quando forem compatíveis.

Se precisar executar testes em outro commit, não faça isso criando uma nova compilação integral desnecessária.

## 6.5. Política para `/tmp`

É necessário distinguir dois usos diferentes:

**Uso proibido pelo fluxo de desenvolvimento:**

- Criar cópias completas do projeto em `/tmp` para compilar.
- Configurar `CARGO_TARGET_DIR=/tmp`.
- Compilar Bevy em worktrees temporárias.
- Criar múltiplos diretórios de build temporários.
- Executar compilações paralelas em várias cópias do projeto.

**Uso interno do sistema:**

Ferramentas de compilação podem utilizar arquivos temporários internos do sistema operacional ou do linker.

Não tente proibir indiscriminadamente qualquer uso de `/tmp` por ferramentas do sistema.

O objetivo é impedir que o próprio fluxo de desenvolvimento direcione grandes artefatos de compilação para o tmpfs.

## 6.6. Perfil de desenvolvimento

Revise as instruções relacionadas aos perfis:

- `dev`
- `release`
- `dist`

O perfil `dist` utiliza otimizações e LTO mais pesados.

Defina claramente que:

- O desenvolvimento normal deve utilizar `cargo check`, `cargo test` e `cargo run` conforme necessário.
- O perfil `dist` não deve ser utilizado como validação rotineira.
- Compilações de distribuição devem ocorrer somente quando forem realmente necessárias.
- Não devem ser executadas várias compilações de distribuição simultaneamente.
- Antes de uma compilação pesada, deve-se verificar se há memória suficiente.

Não altere as opções de otimização do Cargo.toml sem autorização.

## 6.7. Comandos de validação

Revise as instruções existentes para que o Claude Code utilize limites de paralelismo de maneira consistente.

Exemplos:

```bash
cargo check -j 4
cargo test -j 4
cargo clippy --all-targets -j 4
```

Não é necessário acrescentar `-j 4` se o projeto já tiver uma configuração global equivalente e confiável.

Não duplique mecanismos de configuração.

## 6.8. Documentação obrigatória

Atualize as instruções permanentes do projeto, preferencialmente em `CLAUDE.md` e nas regras específicas de desenvolvimento, para registrar:

1. O limite padrão de quatro jobs.
2. O uso do `target/` da raiz.
3. A proibição de builds em worktrees temporárias.
4. A proibição de direcionar `CARGO_TARGET_DIR` para `/tmp`.
5. A necessidade de verificar memória antes de compilações pesadas.
6. A reutilização de artefatos existentes.
7. O uso não rotineiro do perfil `dist`.
8. A proibição de compilações pesadas simultâneas.

Não espalhe a mesma regra em vários arquivos sem necessidade.

Se uma regra já existir, ajuste-a em vez de criar outra contraditória.

Não modifique `Md-usario/`.

## 6.9. Verificação da correção

Depois das alterações:

- Inspecione os arquivos modificados.
- Verifique se a configuração do Cargo está válida.
- Confirme o limite de jobs efetivo.
- Confirme que o target-dir aponta para o local correto.
- Verifique se as instruções do Claude Code não possuem contradições.
- Faça apenas validações leves necessárias.
- Não execute uma compilação completa do Bevy somente para comprovar uma configuração de paralelismo.
- Não faça limpeza de `target/`.
- Não remova caches existentes.

Se a configuração efetiva não puder ser confirmada, informe essa limitação.

## 6.10. Resultado esperado

Ao concluir, o ambiente deverá ter uma política clara e permanente de compilação segura.

O resultado deve proteger o desenvolvimento futuro, não apenas a sessão atual.

# 7. FASE 2 — SEGURANÇA DO CLAUDE CODE (PLANEJAMENTO, NÃO EXECUTAR)

A auditoria encontrou limitações nas proteções de comandos destrutivos.

O bloqueio de `rm -rf` não é suficiente se outras formas de remoção recursiva continuam permitidas.

Problema:

`A6 — segurança das permissões do Claude Code`.

Analise como corrigir futuramente:

- Comandos de remoção.
- Comandos de limpeza.
- Modificações de arquivos.
- Acesso a `Md-usario/`.
- Proteção de arquivos de save.
- Execução de scripts.
- Operações Git destrutivas.
- Permissões excessivas.

Não tente resolver isso nesta execução.

Apresente uma proposta que respeite as capacidades reais da versão instalada do Claude Code.

Não prometa que um padrão de permissão protege contra todas as formas possíveis de exclusão.

# 8. FASE 3 — AUTOPLAY E SEGURANÇA DOS TESTES (PLANEJAMENTO, NÃO EXECUTAR)

A auditoria identificou que o autoplay utiliza o mesmo caminho de save do jogo real.

Arquivo identificado:

`saves/slot1.ron`

Isso cria risco de sobrescrever o save real durante testes.

Também foi identificada a ausência de verificações suficientes para garantir que as ações esperadas realmente aconteceram.

Problemas relacionados:

- A5 — autoplay pode afetar o save real.
- M4 — autoplay sem asserções suficientes.

Planeje uma solução que:

1. Utilize um diretório isolado para saves de teste.
2. Não compartilhe arquivos com os saves reais.
3. Não sobrescreva backups reais.
4. Tenha comportamento previsível em diferentes execuções.
5. Verifique resultados esperados por asserções.
6. Falhe com código de saída diferente de zero quando uma etapa obrigatória falhar.
7. Gere evidências úteis para depuração.
8. Continue permitindo screenshots.

O caminho esperado para os dados de teste deve ficar dentro de:

`target/autoplay/`

Analise como injetar o diretório de save de teste sem prejudicar o funcionamento normal do jogo.

Não implemente ainda.

# 9. FASE 4 — INTEGRAÇÃO DO COMBATE (PLANEJAMENTO, NÃO EXECUTAR)

A auditoria identificou três problemas altos.

## A1 — Skill iniciada mesmo quando o bloqueio falha

Existe risco de iniciar uma habilidade durante um estado em que ela não deveria ser permitida.

Possível consequência: cobrança indevida de MP ou Hearts.

Planeje uma validação consistente antes de:

- Consumir recursos.
- Iniciar cooldown.
- Alterar o estado do jogador.
- Criar efeitos.
- Executar a habilidade.

A execução precisa ser atômica: se a habilidade não puder começar, nenhum custo ou efeito deve ser aplicado.

## A2 — Técnicas continuam após interrupção

Foi identificado risco de hitboxes continuarem causando dano depois que o jogador recebe dano ou morre.

Planeje:

- Cancelamento da técnica.
- Remoção de hitboxes pertencentes à ação cancelada.
- Consistência dos estados.
- Proteção contra efeitos atrasados.
- Tratamento de morte e interrupção.

Investigue separadamente o comportamento de Mending Light.

A cura atualmente é aplicada imediatamente, segundo a auditoria. Não altere isso sem definir se a magia deverá ter conjuração interrompível.

## A3 — Múltiplos danos no mesmo frame

Foi identificado risco de o jogador receber dano repetido durante o período de invulnerabilidade.

Analise:

- Ordem dos sistemas.
- Estado de invulnerabilidade.
- Processamento de mensagens.
- Detecção de múltiplos ataques.
- Atualização do tempo de invulnerabilidade.
- Aplicação de dano no mesmo frame.

Não altere o balanceamento de combate.

Toda correção futura deverá ter testes que reproduzam os casos encontrados.

# 10. FASE 5 — VALIDAÇÃO DOS DADOS RON (PLANEJAMENTO, NÃO EXECUTAR)

A auditoria identificou problemas na validação dos dados.

Problema:

`A4 — validação insuficiente dos dados RON`.

Planeje o fortalecimento de `GameData::validate`.

A validação deverá considerar:

- IDs duplicados.
- Referências inexistentes.
- HP máximo inválido.
- Valores negativos indevidos.
- Cooldowns inválidos.
- Custos negativos.
- Alcances inválidos.
- Valores de resistência fora do intervalo permitido.
- Categorias incompatíveis com efeitos.
- Slots de equipamentos incompatíveis.
- Raridades inexistentes.
- Referências a itens inexistentes.
- Fórmulas que possam causar pânico.
- Configurações incompletas.

Considere validações específicas para cada domínio.

Os erros devem identificar:

- Arquivo de origem, quando disponível.
- Identificador do dado.
- Campo inválido.
- Valor encontrado.
- Motivo da rejeição.

Não transforme a validação em um conjunto genérico de verificações que não informe a origem do erro.

Os dados inválidos devem ser rejeitados antes de iniciar o jogo, conforme a arquitetura documentada.

Não implemente nesta execução.

# 11. FASE 6 — SAVE E LOAD (PLANEJAMENTO, NÃO EXECUTAR)

A auditoria identificou limitações no sistema de salvamento.

Problemas relacionados:

- M1 — restauração incompleta da sala.
- M2 — entidades antigas podem permanecer após carregar.
- M3 — risco no tratamento do backup.
- M11 — validação semântica insuficiente do save.

Planeje uma solução que considere:

1. Restauração completa do estado do jogador.
2. Restauração correta da sala.
3. Remoção ou reconstrução segura das entidades antigas.
4. Restauração do estado do mundo.
5. Preservação de portas, baús e progressos.
6. Validação antes de aplicar os dados.
7. Tratamento de arquivos corrompidos.
8. Preservação de backups válidos.
9. Migração de versões antigas.
10. Escrita atômica.
11. Recuperação segura após falhas.

Não modifique o formato do save sem justificar compatibilidade e migração.

Não altere o sistema de salvamento nesta execução.

# 12. FASE 7 — REVISÃO FINAL (PLANEJAMENTO)

Após as fases anteriores, deverá existir uma revisão completa para confirmar:

- Ambiente seguro.
- Configurações do Claude Code adequadas.
- Autoplay isolado.
- Combate consistente.
- Dados RON validados.
- Save/load confiável.
- Testes cobrindo os casos