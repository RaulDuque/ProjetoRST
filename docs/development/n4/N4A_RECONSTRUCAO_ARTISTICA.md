# N4-A — Separação e reconstrução artística do idle N3.5

**A anatomia escondida foi RECONSTRUÍDA por interpretação artística do agente. Ela NÃO foi recuperada: não existe na arte aprovada.** A qualidade depende da avaliação do proprietário (Gate B pendente).

## 1. Separação (sem mudar nenhum texel)

`art-src/n4-sprite-machine/scripts/partition.py` atribui cada um dos 1320 texels opacos da grade aprovada a **exatamente uma** camada. As regras foram escritas à mão a partir da grade ampliada, porque as chaves de cor se misturam entre as peças:

| Camada | Regra | Texels originais |
|---|---|---|
| `hair_back` | linhas 13–17 da cabeça; nas linhas 18–27, chaves de cabelo (`WwhH`) ou não-pele à esquerda de x = 41, exceto o que vai para `hair_front`; nas linhas 28–47, x ≤ limite da linha (`HAIR_RIGHT`) | 209 |
| `hair_front` | cabelo nas linhas 17–27 com x ≥ 40 (mechas que cruzam o rosto) | 26 |
| `body` | rosto e orelha: linhas 18–27, não-cabelo com x ≥ 41, e pele (`Ssz`) | 43 |
| `arm_front` | antebraço (linhas 52–58, x 36–41) e mão (64–71, x 40–45) | 90 |
| `bracers` | linhas 59–63, x 38–43 | 30 |
| `shirt` | o resto acima da linha 51 | 297 |
| `trousers` | linhas 51–89 (inclui o cinto) | 347 |
| `boots` | linhas ≥ 90 (o cano é uma **estimativa**: a borda da bota não é nítida na redução) | 278 |
| **Total** | | **1320** |

Como as camadas são uma partição, recompor todas devolve o original texel a texel, seja qual for a qualidade das bordas escolhidas. As bordas só decidem quão limpa fica cada peça separada.

## 2. Reconstrução da anatomia escondida

`art-src/n4-sprite-machine/scripts/reconstruct.py` desenha, linha a linha (faixas `{linha: (x_esq, x_dir)}`), só na paleta aprovada:

| Região | Camada | Base usada | O que foi interpretado |
|---|---|---|---|
| Crânio sem cabelo (linhas 15–26) | `body` | contorno da testa e do rosto visíveis; proporção da ficha | forma do crânio, nuca e alto da cabeça |
| Pescoço (27–30) | `body` | lateral do pescoço visível | ligação cabeça-tronco |
| Torso (31–50) | `body` | silhueta da camisa; ficha lateral | peito à frente (x 46), escápula atrás (x 34), cintura, sombra do peitoral, linha abdominal |
| Braço superior (31–51) | `arm_front` | antebraço visível na linha 52 | ombro, bíceps e cotovelo sob a manga |
| Pulso (59–63) | `arm_front` | antebraço e mão visíveis | pulso sob o bracelete |
| Quadril (51–60) | `body` | — | **roupa de baixo neutra** cinza-escura (`q`/`g`, cós `l`), em vez de nudez |
| Pernas (61–112) | `body` | silhueta da calça | coxa, joelho (80–84), panturrilha (91–98), canela, tornozelo, e uma linha de sombra separando as duas pernas |
| Pés (113–118) | `body` | sola da bota | calcanhar e dedos descalços, na linha da sola 118 |

Sombreamento: escuro atrás (`z`), meio-tom (`s`), luz no meio (`S`) e meio-tom na frente, como o antebraço e o rosto aprovados. Contorno: `K` nas bordas das costas e de baixo, `z` nas bordas da frente e de cima (o rosto aprovado termina em `z`, não numa linha preta). Uma primeira versão com contorno `K` fechado parecia um recorte retangular e foi refeita.

### Regras de segurança, verificadas pelo script

- Um texel pintado só entra onde o original tem um texel opaco de uma camada **acima** da pintada. Assim, com a roupa completa, nada pintado aparece. Pintado: 874 texels no `body` e 126 no `arm_front`. Descartados por essa regra: 15 e 7 (ficariam por cima da arte aprovada).
- Nenhum texel original foi repintado.
- Só chaves da paleta N3.4 (teste).

## 3. Resultado

- `evidence/n4a_layers.png`: as 8 camadas isoladas.
- `evidence/n4a_pieces.png`: corpo-base, cabelo, camisa, calça, botas e braceletes.
- `evidence/n4a_removal.png` e `n4a_removal_light.png`: original, sem cabelo, sem camisa, sem calça, sem botas, sem braceletes e corpo-base, em fundo escuro e claro.

Leitura do agente (não é aprovação):

- **Coerente:** a orientação e as proporções seguem a silhueta aprovada; a cabeça sem cabelo tem crânio e nuca; o torso tem peito e cintura; o braço é completo do ombro à mão; as pernas têm joelho e panturrilha; os pés tocam a sola.
- **Mais simples que a arte aprovada:** o sombreamento é mais chapado, a linha das costas do torso é dura, a roupa de baixo é um bloco e o crânio é liso. Um pixel artist faria melhor em todas essas regiões.
- **Sem buraco:** depois de remover cada peça, não sobra texel transparente dentro da extensão do corpo-base em nenhuma linha (métrica no `metrics.json` e no teste). Essa métrica não pega um defeito "opaco": antes da correção, sem o bracelete, aparecia a coxa onde devia estar o pulso. A revisão visual continua obrigatória.

## 4. Bloqueios e pedidos ao proprietário

1. **Corpo neutro de qualidade final:** o corpo-base desta prova é uma interpretação do agente. Para virar referência de produção, precisa de uma passada de pixel art por um artista ou de uma referência do corpo neutro **na mesma pose e resolução** (80×128, perfil). As fichas atuais têm a A-pose em escala pequena.
2. **Cano das botas:** a linha 90 é uma estimativa. Se a bota real começa mais acima ou abaixo, a regra muda numa constante (`BOOTS_FROM_ROW`).
3. **Calça sem botas:** como a calça está enfiada na bota, tirar só as botas deixa a canela nua abaixo da barra. Uma calça "solta até o tornozelo" seria uma **variante** da calça (o contrato já suporta), mas exige arte nova.
4. **Roupa de baixo:** cinza-escuro neutro é uma escolha do agente. A cor, o corte e se haverá roupa de baixo como peça própria são decisões do proprietário.
