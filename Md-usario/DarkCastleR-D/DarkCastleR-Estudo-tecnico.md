# DARK CASTLE RPG — ESTUDO TÉCNICO DE CASTLEVANIA SOTN

Quero estudar dois projetos relacionados a Castlevania: Symphony of the Night como referências técnicas para melhorar nosso Dark Castle RPG.

O jogo é um projeto pessoal, feito para minha diversão, mas quero manter nosso código, personagens, gráficos, músicas e identidade próprios.

## Repositórios de referência

1. https://github.com/BlackLabelHQ/SymphonyRecomp
2. https://github.com/xeeynamo/sotn-decomp

## Objetivo

Investigar os dois repositórios e identificar mecânicas, técnicas e decisões arquiteturais que possam ajudar no desenvolvimento do Dark Castle RPG em Rust/Bevy.

Não quero substituir nosso projeto pelo código de Castlevania.

Quero compreender os sistemas e avaliar como implementar soluções equivalentes ou melhores, de maneira independente.

## Etapa 1 — Licenças e estrutura

Para cada repositório:

- Verifique a licença real.
- Identifique dependências e submódulos.
- Diferencie código próprio, código reconstruído e conteúdo original do jogo.
- Identifique restrições de uso.
- Verifique quais arquivos dependem dos dados do jogo original.
- Não presuma que assets públicos sejam livres.
- Não copie conteúdo protegido para nosso repositório.

Registre os links e os caminhos dos arquivos analisados.

## Etapa 2 — Estudo de sistemas

Investigue, quando houver código ou documentação suficiente:

1. Movimento horizontal.
2. Aceleração e desaceleração.
3. Saltos e controle aéreo.
4. Animações.
5. Máquina de estados do personagem.
6. Ataques básicos.
7. Armas e suas diferenças.
8. Hitboxes e hurtboxes.
9. Recuo ao receber dano.
10. Invulnerabilidade temporária.
11. Magias.
12. Comandos especiais.
13. Consumo de recursos.
14. Equipamentos.
15. Atributos e fórmulas de dano.
16. Resistências elementais.
17. IA dos inimigos.
18. Chefes.
19. Organização das salas.
20. Câmera.
21. Transições.
22. Exploração e segredos.
23. Inventário.
24. Progressão.
25. Interface.

Não invente detalhes que não conseguir confirmar no código.

## Etapa 3 — Comparação com nosso jogo

Compare as descobertas com a implementação real do Dark Castle RPG.

Classifique cada sistema como:

- Já implementado adequadamente.
- Implementado, mas precisa de melhorias.
- Parcialmente implementado.
- Ainda não implementado.
- Não recomendado para nosso projeto.

Para cada melhoria, apresente:

- Problema atual.
- Referência técnica encontrada.
- Solução original proposta em Rust/Bevy.
- Arquivos nossos potencialmente envolvidos.
- Riscos.
- Dependências.
- Testes necessários.
- Prioridade.

## Etapa 4 — Plano de evolução

Monte um roadmap de melhorias para as próximas versões.

Priorize qualidade do combate, movimentação, armas, animações, inimigos, chefes e exploração.

Não proponha reconstruir sistemas que já funcionam sem justificativa técnica.

Não altere a arquitetura existente desnecessariamente.

## Etapa 5 — Documentação

Crie um documento técnico em:

`docs/REFERENCIAS_SOTN.md`

Inclua referências verificáveis, links, caminhos e, quando possível, commits específicos dos projetos estudados.

Separe claramente:

- Fatos observados nos repositórios.
- Inferências técnicas.
- Propostas originais para Dark Castle RPG.

Não copie arquivos, sprites, músicas, mapas, personagens, diálogos ou código de Castlevania para nosso projeto.

## Segurança

Não altere código do jogo nesta tarefa.

Não modifique assets.

Não altere `Md-usario/`, saves ou arquivos protegidos.

Não execute commits ou push.

Se precisar baixar os repositórios para estudo, utilize um diretório temporário externo ao nosso projeto, sem incluir os arquivos baixados no Git.

## Entrega

Apresente um relatório técnico detalhado, indicando quais mecânicas vale a pena estudar primeiro e quais mudanças trariam maior benefício ao Dark Castle RPG.

Não inicie a V2.4 antes de minha aprovação.