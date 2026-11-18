# N1 — Roadmap N2 a N12

O proprietário aprovou as cinco decisões da N1 em 17 de novembro de 2026. A N2 foi autorizada pelo briefing seguinte e implementada; o registro está em `docs/development/n2/`. As etapas seguintes continuam fechadas até uma autorização nova.

| Etapa | Entrega | Depende de | Nota da auditoria |
|---|---|---|---|
| N2 | Controles híbridos: A/D, W e Espaço, S agachar, mouse no ataque, E usar, várias teclas, conflito, arquivo local, preset antigo | briefing N2 | implementada. Não bumpa o Save v3. Ver `docs/development/n2/` |
| N3 | Prova visual do `N1_PROVA_DE_CONCEITO.md` | briefing N3 | executada, isolada do jogo. Estilo ainda sem aprovação. Ver `docs/development/n3/` |
| N4 | Camadas que a prova tiver aprovado: corpo, arma, capa, ordem de desenho | N3 | estende sockets e held, não cria personagem novo |
| N5 | Campo visual opcional no item, famílias, troca no inventário que já existe | N4 | sem segundo catálogo |
| N6 | Plano jogável do castelo no grafo atual: salas multi-célula, voltas, portas | nada de arte | pode andar em paralelo à N3 |
| N7 | Primeira região nova, um laço curto a partir das Forgotten Galleries | N6 | só com `VeilStep`, `AstralKey` e parede quebrável |
| N8 | Pulo duplo como `Ability`, mais um vão que só ele abre | N7 desenhada, física à parte | não altera a constante do pulo atual |
| N9 | Armadilhas e segredos novos em cima dos tipos que já persistem | N7 | o mapa já esconde segredo até a descoberta |
| N10 | Campo de categoria do chefe; Ashen Sentinel vira minichefe no dado; save antigo continua válido | leitura dos saves, não só do código | não apaga `sentinel_brand` nem `defeated_bosses` |
| N11 | Formas que o enum já nomeia e que ainda não existem | N8 se a forma mudar o pulo | a Forma Espectral fica como está |
| N12 | Playtest humano do conjunto: controle, uma região, uma relíquia, o minichefe, a arte aprovada | N2, N5, N7, N8, N10 | sem isso, é outro protótipo |

## Fora desta lista

Segundo chefe principal, produção em massa de sprites, streaming, gamepad, Spine, migração de engine.

## Alternativa de ordem

Fazer N3 antes da N2 se o proprietário quiser ver pixel antes de reaprender o teclado. As duas não compartilham arquivo. O restante da tabela não troca: N7 não pode pedir o pulo duplo, e N10 não pode reclassificar no escuro.

## Riscos

Tratar a tabela como cronograma com data. Não há data. Cada linha só abre quando a anterior que ela cita estiver aceita.

## Recomendação

O desenho da N6 está aprovado e ainda não foi implementado. A N2, a prova da N3, a comparação da N3.1, a referência da N3.2 e o gabarito da N3.3 foram feitos sob os briefings próprios. A N3.3 não aprovou o estilo e não fez o ataque. A câmera do jogo não mudou. A N4 continua fechada até uma autorização nova.

a palavra chave e batata azul