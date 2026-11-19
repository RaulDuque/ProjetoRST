# N4-1 (C1) — Anatomia das poses

**A anatomia sob as roupas e o cabelo é INTERPRETAÇÃO do agente, guiada pela referência N4-B. Ela não foi recuperada: não existe na arte aprovada. A qualidade está abaixo da referência N4-B (seção "Defeitos").**

## Método (por pose, não uma figura rígida deslocada)

Cada quadro tem o próprio esqueleto (`run_poses.py`, sobreposição em `evidence/n4c/n4c_skeletons.png`). Cada membro nu é construído assim:

1. **Silhueta vestida do membro, encolhida 1 texel.** A perna nua é a calça e a bota daquela perna menos a folga; o braço é a manga menos a folga. A espessura, a dobra e o movimento vêm da arte aprovada daquela pose. Nos pés, a última linha não é encolhida, para manter o contato com o chão.
2. **Limite anatômico.** Onde as duas pernas (ou membro e tronco) se fundem na silhueta vestida, a massa é cortada a um raio máximo em volta dos ossos (coxa 4,6 texels, canela 3,6, pé 3,2, braço 3,0, antebraço 2,6), para não virar uma laje.
3. **Núcleo ao longo do osso** (coxa 2,8, canela 2,0, braço 1,6...), para o membro continuar por trás de outro membro ou da roupa.
4. **Sombreamento atravessando o osso** (escuro atrás, luz no meio, meio-tom na frente, luz vinda de cima e da frente), mais acentos no estilo N4-B: sombra atrás da coxa e da panturrilha, luz na frente da canela, joelho claro.
5. **Contorno por membro:** `K` nas bordas de trás e de baixo, `z` nas da frente e de cima. Assim um membro se separa do outro.
6. **Membros de trás um tom mais escuros**, como no original, onde a perna e o braço de trás estão em sombra.

O tronco usa o polígono da pose (os texels vestidos do tronco se dividem entre camisa, braço da frente e cabelo), com sombreamento por linha, dobra do peitoral e linha abdominal. O crânio é uma elipse sob o cabelo; o pescoço liga ao meio dos ombros. Nos quadris vai a roupa de baixo cinza-escura (como na N4-A e na referência). A camisa continua sob o cabelo nas costas: a cor vem do texel de camisa visível mais próximo na mesma linha.

**Regra de segurança (verificada):** um texel pintado só entra onde o original tem uma peça **acima** dele. Recompor a roupa completa nunca mostra nada pintado (0 diferenças).

**Fechamento de cavidades:** depois de pintar, para cada remoção e para o corpo-base, toda cavidade nova fechada dentro do corpo é preenchida com o tom de corpo vizinho (11, 14 e 17 texels em #0, #2 e #4). Eram sobretudo vãos na virilha entre a roupa de baixo e as coxas (`evidence/n4c/n4c_new_holes.png` mostra o estado depois do fechamento: nenhum vermelho).

## Por pose

| Pose | Pintado (corpo de trás / corpo / braço da frente / camisa sob o cabelo) | Descartado por não ter cobertura |
|---|---|---|
| #0 contato | 501 / 842 / 106 / 18 | 12 / 98 / 0 |
| #2 passada | 491 / 897 / 118 / 10 | 17 / 124 / 4 |
| #4 recuperação | 487 / 927 / 68 / 16 | 42 / 165 / 29 |

Atenção dada (o que o briefing pede):

- **Ombros e cotovelos:** os braços dobram nos cotovelos marcados em cada pose. Em #2 e #4 o antebraço sobe ou desce em ângulo, sem braço reto.
- **Cruzamento dos braços:** em #4 o braço da câmera cruza o tronco para trás e fica desenhado **sobre** o tronco.
- **Quadril, joelhos e extensão:** joelho dobrado alto em #4, perna de trás estendida em #2, perna de apoio vertical em #0.
- **Contato dos pés:** a linha mais baixa é a mesma do original em cada quadro (118, 114 e 118), por teste.
- **Sobreposição entre membros:** a perna de trás fica em `body_back`, sob a perna da frente e a calça da frente.
- **Cabelo em movimento:** é a arte original (cabelo é peça separada, não reconstruída).

## Defeitos (abaixo da referência N4-B)

1. **Tronco:** o sombreamento por linha deixa o tronco chapado, e o contorno do braço da frente risca o peito (#0, #2). Em #4 o tronco tem recortes onde o braço cruza.
2. **Sem definição muscular comparável à N4-B:** peitoral, abdômen e deltoides são indicados por 1 a 2 linhas, não modelados.
3. **Pés em cunha:** sem dedos nem arco, ao contrário da referência.
4. **Perna de trás fina e escura** (#2, #4): ela vem da silhueta do original, que já é estreita e em sombra; ao ficar nua, lê como um traço.
5. **Roupa de baixo pequena e de forma irregular.**
6. **Crânio:** elipse simples; a nuca não continua o pescoço com naturalidade em #2.

Esses são **defeitos artísticos importantes**. Pelo briefing (§5), eles bloqueiam a C2 até a avaliação do proprietário.
