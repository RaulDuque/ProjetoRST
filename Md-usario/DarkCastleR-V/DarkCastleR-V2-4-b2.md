# DARK CASTLE RPG — V2.4 / BLOCO B2
## Etapas 6 e 7 — Dano simultâneo e recuo por intensidade

Você continuará a implementação do Dark Castle RPG em Rust/Bevy.

As etapas 0 a 5 da V2.4 foram concluídas e registradas em dois commits locais:

- `9aa312b` — implementação do Combat Feel.
- `887de84` — documentação dos Blocos A e B1.

A última validação relatada apresentou 484 testes Rust aprovados e autoplay 23/23.

**Objetivo desta execução: publicar com segurança o checkpoint anterior e implementar exclusivamente as etapas 6 e 7.**

Não implemente as etapas 8, 9 ou 10.

---

## 1. Auditoria inicial

Antes de modificar qualquer código:

1. Confirme a branch `art/v1.2-protagonist`.
2. Confirme o HEAD `887de84`.
3. Execute `git status --short`.
4. Execute `git fetch origin`.
5. Verifique a divergência local/remota.
6. Confirme que os dois commits locais são exatamente `9aa312b` e `887de84`.
7. Confirme que nenhum arquivo protegido está incluído.
8. Preserve as alterações pessoais em `Md-usario/` e `art-src/`.

Se houver commits remotos desconhecidos, divergência inesperada ou alterações suspeitas, interrompa.

Não faça merge ou rebase automaticamente.

## 2. Publicação do checkpoint

Se todas as verificações anteriores passarem, está autorizado exclusivamente um push normal:

`git push origin art/v1.2-protagonist`

Não utilize force push.

Confirme depois:

- Hash remoto.
- Hash local.
- Upstream.
- Divergência 0/0.

Não inclua novas alterações nesse push.

Se a publicação falhar, interrompa e explique o problema.

---

## 3. Documentação obrigatória

Leia integralmente:

- `docs/PLANO_V2_4_COMBAT_FEEL.md`
- `docs/COMBAT_FEEL_V2_4.md`
- `docs/ESTUDO_SOTN_COMBATE.md`
- `docs/ESTUDO_SOTN_ATRIBUTOS.md`
- `docs/COMBAT_SYSTEM.md`
- `docs/ARCHITECTURE.md`
- `docs/DECISIONS.md`

Localize os documentos pelos caminhos reais.

Examine também o código de:

- `src/combat/`
- `src/player/`
- `src/enemies/`
- `src/core/`
- `src/inventory/`
- `src/skills/`
- `src/magic/`

Não presuma que a arquitetura descrita na documentação corresponde integralmente ao código: confirme.

---

## 4. Etapa 0 do B2 — Caracterização do dano atual

Antes de implementar qualquer mudança, crie testes para registrar:

- Dano físico.
- Dano mágico.
- Defesa.
- Resistências.
- Críticos.
- Invulnerabilidade.
- Recuo.
- Dano por contato.
- Dano por hitbox.
- Dano de projéteis.
- Dano recebido pelo jogador.
- Dano recebido por inimigos.
- Dois ataques no mesmo quadro.
- Dois inimigos atacando simultaneamente.
- Ataques com intensidades diferentes.
- Morte causada por dano.

Registre as diferenças reais entre jogador e inimigos.

Não modifique fórmulas existentes para fazer os testes corresponderem a suposições.

---

## 5. Etapa 6 — Política de dano simultâneo

Implemente uma política configurável com duas modalidades:

### Legacy

Mantém integralmente o comportamento atual.

Esta é a modalidade padrão.

### HighestDamagePerFrame

Quando múltiplos acertos elegíveis atingirem o mesmo alvo no mesmo quadro, considera apenas o acerto de maior dano efetivo.

Requisitos:

1. Agrupar candidatos por alvo e quadro lógico.
2. Não confundir alvos distintos.
3. Não confundir quadros distintos.
4. Não introduzir danos duplicados.
5. Não permitir que a invulnerabilidade do primeiro acerto impeça indevidamente a comparação com outros acertos do mesmo quadro.
6. Preservar a ordem e a semântica dos GameSets.
7. Preservar os efeitos de defesa, resistência e crítico.
8. Definir claramente se a comparação utiliza dano bruto ou efetivo; preferir dano efetivo, desde que compatível com a arquitetura.
9. Definir uma regra determinística de desempate.
10. Preservar o comportamento de morte, recuo e invulnerabilidade.
11. Não alterar o Save V3.
12. Não alterar o comportamento atual dos inimigos por padrão.

### Atenção à arquitetura

Primeiro investigue em que ponto os eventos de dano são produzidos, transformados e consumidos.

Não implemente agregação apenas no consumidor se parte dos candidatos já for descartada antes dele.

Se for necessário alterar a geração dos eventos, proponha a menor mudança compatível.

Evite duplicar a fórmula de dano.

Prefira regras puras testáveis e integração ECS mínima.

### Testes específicos

Cubra:

- Dois acertos no jogador no mesmo quadro.
- Dois acertos em um inimigo no mesmo quadro.
- Acertos em alvos diferentes.
- Empate de dano.
- Dano físico e elemental simultâneo.
- Um golpe crítico e outro normal.
- Invulnerabilidade preexistente.
- Invulnerabilidade iniciada no próprio quadro.
- Ataque que mata o alvo.
- Modalidade Legacy.
- Modalidade HighestDamagePerFrame.

Não ative HighestDamagePerFrame no jogo atual.

---

## 6. Etapa 7 — Recuo por intensidade

Implemente suporte para três intensidades:

- Leve.
- Médio.
- Forte.

Inclua também um limiar configurável de atordoamento/recuo (`stagger_threshold`), conforme o plano técnico.

### Requisitos

1. Preservar o recuo atual como padrão.
2. Não alterar os valores existentes das armas.
3. Não alterar a duração atual de invulnerabilidade.
4. Não modificar o Save V3.
5. Não modificar a física global.
6. Evitar empurrar entidades através de paredes.
7. Respeitar estados de morte.
8. Respeitar estados de transformação.
9. Preservar o comportamento de projéteis.
10. Evitar transições inválidas da máquina de estados.

### Modelagem

Antes de alterar `DamageDealt`, compare:

- Campo opcional no evento.
- Componente auxiliar.
- Recurso ou contexto de resolução.
- Outra solução compatível com a arquitetura existente.

Escolha a solução de menor complexidade total.

Se adicionar um campo a `DamageDealt`, atualize mecanicamente todos os construtores necessários, mantendo o valor legado como padrão.

Não altere a fórmula de dano para determinar a intensidade sem justificativa.

Documente como o limiar e a intensidade são calculados.

### Testes específicos

Cubra:

- Golpe abaixo do limiar.
- Golpe exatamente no limiar.
- Golpe acima do limiar.
- Recuo leve.
- Recuo médio.
- Recuo forte.
- Golpe fatal.
- Golpe durante invulnerabilidade.
- Golpe durante ataque.
- Golpe durante magia.
- Golpe durante transformação.
- Colisão com parede.
- Comportamento legado.

---

## 7. Interação entre as etapas 6 e 7

Teste a combinação:

- Dois golpes simultâneos com danos diferentes.
- Golpe mais forte selecionado.
- Intensidade de recuo correspondente ao golpe selecionado.
- Nenhum efeito duplicado.
- Nenhum estado inválido.
- Invulnerabilidade aplicada uma única vez.

Evite combinar o dano de um ataque com o recuo de outro, salvo se houver uma regra explícita e documentada.

---

## 8. Preservação do jogo

Não altere:

- Iron Sword.
- Valores das demais armas.
- Dados RON de balanceamento.
- Política padrão `StrikeCancelPolicy::Preserve`.
- Custos de MP e Hearts.
- Cooldowns.
- Comandos especiais.
- Save V3.
- Equipamentos visuais.
- Cenários e câmera V2.3.
- NPCs e comércio.

Todos os novos comportamentos devem ser opcionais e desativados por padrão.

---

## 9. Validação

Execute sequencialmente:

`cargo fmt --all -- --check`

`cargo check -j 2`

`cargo clippy --all-targets -j 2 -- -D warnings`

`cargo test -j 2`

`cargo run -j 2 --example autoplay`

Utilize um único processo Cargo por vez.

Compare os resultados com a referência de 484 testes e autoplay 23/23.

Verifique também a integridade dos saves reais, sem sobrescrevê-los.

Não remova testes para fazer a suíte passar.

---

## 10. Documentação

Atualize `docs/COMBAT_FEEL_V2_4.md` com:

- Política de dano simultâneo.
- Comportamento legado.
- Regras de agregação.
- Intensidades de recuo.
- Limiar de stagger.
- Novos campos e padrões.
- Integração com GameSets.
- Casos de borda.
- Limitações.
- Resultados dos testes.

Não modifique `Md-usario/`.

---

## 11. Segurança e Git

Não execute:

- `git reset --hard`
- `git clean`
- `cargo clean`
- `git rebase`
- Force push.

Não altere arquivos protegidos.

Não crie commits das etapas 6 e 7.

Não publique o Bloco B2.

A autorização de push desta tarefa se limita aos dois commits anteriores, `9aa312b` e `887de84`.

---

## 12. Relatório final

Entregue:

A. Auditoria inicial.

B. Confirmação da publicação do checkpoint anterior.

C. Testes de caracterização criados.

D. Implementação da etapa 6.

E. Implementação da etapa 7.

F. Integração entre dano e recuo.

G. Arquivos modificados.

H. Novos campos e valores padrão.

I. Testes executados.

J. Comparação com os 484 testes anteriores.

K. Compatibilidade com Iron Sword.

L. Compatibilidade com Save V3.

M. Riscos e limitações.

N. Proposta de commits para o B2.

O. Estado final do Git.

P. Preparação da etapa 8 — atordoamento de inimigos.

**Não implemente a etapa 8.**

Ao terminar, aguarde minha revisão antes de criar commits ou continuar a V2.4.