# CONTINUAÇÃO DO SETUP — DARK CASTLE RPG

Você já realizou a análise inicial do ambiente e criou a base Cargo do projeto.

Agora vamos continuar a execução do promptMaster.md.

## 1. Verificar as instalações

Primeiro, verifique:

- rustc --version
- cargo --version
- rustup --version
- git --version
- claude --version

Verifique também se as bibliotecas necessárias para Bevy estão disponíveis através de pkg-config.

Se alguma biblioteca ainda estiver faltando, informe exatamente qual.

Não reinstale ferramentas que já estão funcionando.

## 2. Confirmar a raiz do projeto

Verifique a localização atual do Cargo.toml.

Confirme a localização da pasta Md-usario e do arquivo promptMaster.md.

Não renomeie, mova ou exclua arquivos existentes.

A pasta que contém o Cargo.toml será a raiz oficial do projeto.

## 3. Revisar as dependências

Analise o Cargo.toml e confirme:

- Bevy 0.20.1.
- Serde.
- RON.
- Thiserror.
- Tracing.

Confirme a compatibilidade entre as versões.

Avalie cuidadosamente as features do Bevy para o nosso jogo 2D.

Não adicione dependências desnecessárias.

Configure os perfis de desenvolvimento e distribuição.

## 4. Configurar o Git

Verifique o estado do repositório.

Preserve todos os arquivos existentes.

Configure .gitignore.

Não faça commits antes de verificar o conteúdo e a organização dos arquivos.

## 5. Criar a infraestrutura do Claude Code

Agora implemente:

- CLAUDE.md.
- .claude/rules/.
- .claude/skills/.
- .claude/agents/.
- .claude/settings.json.

Crie as cinco skills solicitadas no promptMaster.md.

Crie os três agentes inicialmente solicitados.

Os agentes deverão ter responsabilidades distintas, instruções próprias e permissões adequadas.

Não crie ainda a skill /multi-agent-development. Ela será planejada em uma etapa posterior.

## 6. Preparar a documentação

Crie os documentos técnicos solicitados em docs/.

Crie o ROADMAP.md com etapas e tarefas verificáveis.

Não marque funcionalidades como concluídas se ainda não foram implementadas.

## 7. Implementação da primeira versão

Antes de implementar o protótipo completo, estabeleça a arquitetura e separe as regras de negócio dos sistemas específicos da engine.

Implemente primeiro os módulos puros e testáveis:

- Atributos.
- Progressão.
- Cálculo de dano.
- Parser de comandos.
- Definições de habilidades.
- Estrutura de salvamento.

Depois integre esses módulos ao Bevy.

Não tente implementar todo o jogo simultaneamente.

## 8. Validação

Execute:

cargo fmt --check
cargo clippy
cargo test
cargo check

Depois tente executar o jogo com cargo run, caso o ambiente tenha suporte gráfico disponível.

Se o ambiente não tiver uma sessão gráfica, diferencie claramente um problema de ambiente de um erro de compilação.

## 9. Commits

Utilize commits atômicos.

Cada commit deve representar uma alteração lógica independente.

Não faça commits automáticos sem revisar git diff.

Não misture funcionalidades diferentes.

## 10. Relatório

Ao concluir, apresente:

1. Arquitetura criada.
2. Dependências e versões.
3. Skills criadas.
4. Agentes criados.
5. Regras configuradas.
6. Documentação criada.
7. Funcionalidades realmente implementadas.
8. Testes executados e resultados.
9. Problemas pendentes.
10. Próxima etapa recomendada.

IMPORTANTE:

Não considere uma tarefa concluída apenas porque os arquivos foram criados.

Verifique o código, compile e teste.

Trabalhe diretamente na raiz existente do projeto.

Não destrua alterações anteriores.

Continue a execução do planejamento original de forma incremental.