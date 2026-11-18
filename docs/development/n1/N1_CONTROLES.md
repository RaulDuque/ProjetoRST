# N1 — Controles

O proprietário escolheu o esquema híbrido. A N1 registra. Não remapeia o jogo.

## Hoje

Uma tecla por ação, em `InputBindings`, sem mouse e sem gravar a escolha. O arquivo `controles.md` na raiz lista o que cada tecla faz agora.

| Ação | Tecla atual | Tecla desejada |
|---|---|---|
| Esquerda / direita | setas | A / D |
| Pular | Z | W e Espaço, os dois |
| Agachar | seta para baixo | S |
| Ataque | X | botão esquerdo do mouse, e uma tecla também |
| Magia, guarda, esquiva, interagir, forma, mapa, status, bolsa, respawn, save, load | C, A, S, D, T, M, Tab, I, R, F5, F9 | continuam, e passam a ser remapeáveis |

O conflito real: **D hoje é usar e conversar.** No esquema novo, D é andar para a direita. A tecla de usar tem de mudar ou o andar come o altar. Isso se decide antes de escrever a N2. Proposta: usar passa para E, D fica com o movimento, e o diálogo continua aceitando Enter.

## Sequências

O buffer já grava frente e trás em relação à face, não a tecla física (`command_buffer`). "Baixo, frente, ataque" funciona com seta ou, no futuro, com S e D, porque o que entra no buffer é o eixo, não o código da tecla. A N2 não reescreve o reconhecedor. Ela só muda de onde o eixo vem.

Guarda limpa o buffer. Isso fica.

## O que a N2 precisa ter

- Várias teclas para a mesma ação (W e Espaço).
- Mouse no ataque, sem impedir uma tecla reserva.
- Tabela editável, conflito avisado (a mesma tecla em duas ações).
- Gravar a tabela no save ou num arquivo ao lado, e um botão que devolve o padrão.
- O padrão novo é o híbrido. O padrão antigo (setas e Z X C) fica como um preset, para não quebrar a mão de quem já joga.
- Gamepad fica na struct, vazio. Não se compra controle nem se escreve o driver agora.

A tela de configuração pausa, como o inventário. Enquanto ela está aberta, o eixo não anda.

## Alternativas

Trocar as constantes agora e deixar o remapeamento para depois. Quebra o `controles.md`, os probes e a memória muscular sem entregar a tela. Recusado.

## Riscos

Mouse e tecla no mesmo frame disparando dois ataques. A ação de ataque continua sendo um pulso por aperto, seja do mouse ou da tecla.

Salvar o binding dentro do Save v3 mistura preferência com progresso. Um arquivo próprio de configuração sobrevive ao apagar do save e não exige bump se for só local. Preferência da N2: arquivo local, não o save do personagem.

## Recomendação

Aprovar o híbrido, com usar em E, e o preset antigo conservado. A implementação espera essa aprovação e não começa nesta etapa.
