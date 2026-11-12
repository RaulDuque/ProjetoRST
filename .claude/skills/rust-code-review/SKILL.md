---
name: rust-code-review
description: Revisa código Rust/Bevy do Dark Castle RPG procurando duplicação, problemas de ownership/borrowing, desempenho, arquitetura ECS, bugs e tratamento de erros, e entrega os problemas classificados por gravidade. Use antes de commits grandes ou quando pedirem revisão.
argument-hint: "[arquivos, módulo ou 'diff' (padrão: alterações não commitadas)]"
---

# /rust-code-review

Alvo: $ARGUMENTS (se vazio: `git diff` + arquivos não rastreados relevantes).

Não altere código durante a revisão: apenas relate. Cada achado precisa de
arquivo:linha e de um cenário concreto (entrada → comportamento errado).

## Checklist

1. **Código duplicado** — lógica repetida entre player/enemies, fórmulas copiadas fora de `StatFormulas`/`calculate_damage`, sistemas que fazem a mesma coisa.
2. **Ownership e borrowing** — `clone()` desnecessário em loops quentes, `String` onde `&str` basta, `Vec` coletado só para iterar, empréstimos que forçam cópias.
3. **Desempenho** — trabalho por frame que poderia ser por evento (`Changed<T>`, mensagens), buscas O(n²) sem necessidade, alocação por frame em sistemas de UI, queries amplas sem filtro.
4. **Arquitetura ECS** — sistema fora de `GameSet`, módulo mutando componentes de outro módulo em vez de emitir mensagem, `use bevy` em arquivo puro, lógica em `main.rs`, ordem de sistemas implícita que causa atraso de 1 frame.
5. **Bugs** — estados incompatíveis (agir morto), recursos negativos, cooldown não iniciado, entidade usada após `despawn`, comando executado duas vezes, divisões por zero, overflow.
6. **Tratamento de erros** — `unwrap`/`expect` em caminho normal, erros engolidos sem log, `Result` ignorado, mensagens de erro sem contexto (caminho do arquivo).
7. **Testes** — regra nova sem teste, asserções fracas, aleatoriedade não injetada.

Ferramentas: `cargo clippy --all-targets`, `grep -rn`, leitura dos arquivos.

## Formato da resposta

```
## Crítico      (quebra jogo, perde save, panic)
- arquivo:linha — problema. Cenário: ... Sugestão: ...
## Alto         (bug de gameplay, violação de arquitetura)
## Médio        (desempenho, duplicação, erro mal tratado)
## Baixo        (estilo, nomes, docs)
## Pontos positivos
```
Se não houver achados numa categoria, escreva "nenhum".
