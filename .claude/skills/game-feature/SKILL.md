---
name: game-feature
description: Implementa uma funcionalidade de jogo do Dark Castle RPG de ponta a ponta (análise, plano, código puro + sistemas Bevy, testes, validação, docs). Use quando o usuário pedir uma nova mecânica, sistema, conteúdo ou tela.
argument-hint: "<descrição da funcionalidade>"
---

# /game-feature

Funcionalidade pedida: $ARGUMENTS

Siga **todas** as etapas, na ordem. Não pule validação.

## 1. Analisar o requisito
- Reescreva o requisito em 2–4 frases objetivas e liste critérios de aceitação verificáveis.
- Se algo for ambíguo **e** mudar o design, pergunte. Caso contrário, escolha o padrão mais simples e registre a escolha.
- Confira `docs/ROADMAP.md`: a funcionalidade pertence a qual etapa? Há dependências pendentes?

## 2. Verificar a arquitetura
- Leia `docs/ARCHITECTURE.md` e `.claude/rules/architecture.md`.
- Procure sistemas existentes que já resolvem parte do problema (`grep -rn "<conceito>" src/`). **Não duplique.**

## 3. Identificar módulos afetados
Liste os arquivos a criar/alterar, separando:
- regras puras (sem Bevy) → onde ficam e quais testes recebem;
- componentes/sistemas Bevy → em qual `GameSet` entram;
- dados (`assets/data/*.ron`) → novos campos com `#[serde(default)]` para não quebrar arquivos existentes;
- mensagens novas em `src/core/events.rs` (registrar no `CorePlugin`).

## 4. Planejar
Apresente um plano curto (passos numerados). Para funcionalidades grandes, divida em incrementos que compilem sozinhos (cada um vira um commit).

## 5. Implementar
- Primeiro a regra pura + testes; depois a integração Bevy.
- Sem `unwrap`/`expect` em caminhos normais, sem valores mágicos, sem funções vazias.
- Alterações pequenas e localizadas; não reescreva módulos inteiros.

## 6. Criar testes
- Unitários/integração para toda regra nova (casos normais, limites e erros).
- Se houver fluxo ECS novo relevante, acrescente um caso em `tests/app_smoke_tests.rs`.
- Conteúdo de dados novo é coberto por `tests/data_tests.rs` (adicione asserções se fizer sentido).

## 7. Executar validações
Rode a skill `/test-and-validate` (fmt, clippy, test, check e, se houver tela, autoplay). Corrija tudo antes de seguir.

## 8. Atualizar documentação
- Doc do sistema em `docs/` (fórmulas, formatos de dados, controles).
- `docs/ROADMAP.md`: marque `[x]` **somente** o que foi implementado e validado.
- `docs/DECISIONS.md` se houve decisão técnica relevante; `CHANGELOG.md` em `[Unreleased]`.

## 9. Apresentar resumo
- O que foi feito (arquivos), como testar manualmente (teclas), resultados das validações, pendências honestas.
- Sugira os commits atômicos (ou use `/atomic-commit` se o usuário pediu commit).
