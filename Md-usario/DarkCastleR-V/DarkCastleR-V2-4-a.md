# DARK CASTLE RPG — V2.4 / BLOCO A
## Fundação do Combat Feel, linha do tempo e cancelamento

Autorizo o início da implementação da V2.4 — Combat Feel / Peso do Golpe, exclusivamente nas etapas 0, 1, 2 e 3 do plano técnico.

Não implemente as etapas 4 a 10 nesta execução.

## 1. Documentação obrigatória

Antes de modificar qualquer código, leia integralmente:

- `docs/PLANO_V2_4_COMBAT_FEEL.md`
- `docs/ESTUDO_SOTN_COMBATE.md`
- `docs/ESTUDO_SOTN_ANIMACOES.md`
- `docs/PLANO_MELHORIAS_SOTN.md`
- `docs/DECISIONS.md`
- `docs/ARCHITECTURE.md`

Verifique os caminhos reais dos documentos.

Use o código atual como fonte definitiva para confirmar comportamentos.

## 2. Auditoria inicial

Confirme:

- Branch `art/v1.2-protagonist`.
- HEAD esperado `7bc9b5e`.
- Estado do Git.
- Arquivos protegidos.
- Dependências atuais.
- Sistemas de combate existentes.

Não descarte mudanças locais.

Não altere arquivos não relacionados.

## 3. Etapa 0 — Testes de regressão

Antes de implementar novas mecânicas, crie testes que caracterizem o comportamento atual.

Cubra:

- Ataques normais.
- Ataques aéreos.
- Ataques agachados.
- Reconhecimento de comandos.
- Ativação de habilidades.
- Consumo de MP e Hearts.
- Cooldowns.
- Invulnerabilidade.
- Dano recebido.
- Hitboxes.
- Transições de estados.
- Cancelamento atual.
- Comportamento da Iron Sword.

Os testes devem registrar o funcionamento real, inclusive eventuais limitações.

Não altere a implementação apenas para fazer os testes corresponderem a uma expectativa incorreta.

## 4. Etapa 1 — Linha do tempo do golpe

Implemente a infraestrutura para definir:

- Antecipação.
- Janela ativa.
- Recuperação.
- Duração total.
- Pontos de cancelamento.

Use dados configuráveis.

Preserve os temporizadores atuais como fonte principal de gameplay.

Não faça a física depender dos quadros de animação.

Não modifique os parâmetros atuais da Iron Sword.

Os campos novos devem reproduzir o comportamento anterior quando omitidos.

## 5. Etapa 2 — Ações permitidas

Implemente regras explícitas para identificar quais ações são permitidas em cada fase do golpe.

Considere:

- Movimento.
- Pulo.
- Ataque.
- Magia.
- Guarda.
- Esquiva.
- Habilidades.
- Cancelamento.

Não introduza restrições novas por padrão.

As regras devem ser testáveis de forma isolada.

Evite expandir desnecessariamente o `PlayerStateMachine`.

## 6. Etapa 3 — Cancelamento

Implemente suporte configurável para cancelamento de ataques.

Decisão de projeto:

**As técnicas especiais devem preservar o comportamento atual de cancelamento.**

Não restrinja técnicas existentes sem uma configuração explícita.

Não permita que um cancelamento:

- Cobre MP duas vezes.
- Consuma Hearts duas vezes.
- Ignore cooldowns.
- Execute duas habilidades simultaneamente indevidamente.
- Deixe hitboxes antigas ativas.
- Deixe o personagem preso em um estado.
- Quebre o reconhecimento de comandos.

A prioridade de comandos existente deve ser preservada.

## 7. Compatibilidade

Preserve:

- Save V3.
- Equipamentos.
- Armas visuais da V2.2.
- Magias.
- Transformações.
- Inimigos.
- NPCs.
- Comércio.
- Exploração.
- Câmera e cenários da V2.3.

Não adicione dependências sem aprovação.

Não copie código ou assets de Castlevania.

## 8. Testes

Execute:

- `cargo fmt --all -- --check`
- `cargo check -j 2`
- `cargo clippy --all-targets -j 2 -- -D warnings`
- `cargo test -j 2`

Use apenas um processo Cargo por vez.

Execute também os testes de integração e as rotas de gameplay relevantes.

Se algum teste falhar, investigue a causa antes de prosseguir.

Não remova testes para fazer a suíte passar.

## 9. Documentação

Registre as mudanças em documentação técnica da V2.4.

Documente:

- Novos campos.
- Valores padrão.
- Estados.
- Fases.
- Cancelamentos.
- Compatibilidade.
- Limitações.

Não modifique `Md-usario/`.

## 10. Segurança

Não execute:

- `git reset --hard`
- `git clean`
- `cargo clean`
- `git push`
- `git commit`

Não altere arquivos protegidos.

Não sobrescreva assets publicados.

Não modifique saves reais.

## 11. Entrega

Apresente:

A. Auditoria inicial.

B. Testes de regressão criados.

C. Linha do tempo implementada.

D. Regras de ações permitidas.

E. Cancelamento implementado.

F. Arquivos alterados.

G. Campos e valores padrão.

H. Resultados dos testes.

I. Regressões identificadas e corrigidas.

J. Limitações.

K. Estado final do Git.

L. Próximas etapas recomendadas.

**Não crie commits e não execute push.**

Ao terminar o Bloco A, pare e aguarde minha revisão antes de iniciar o Bloco B.