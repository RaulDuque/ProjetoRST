# N4-A — Riscos

## Artísticos

1. **Corpo neutro abaixo da qualidade aprovada** (alto): é interpretação do agente, mais chapada que a arte N3.5. Se for aceito como base de produção sem uma passada artística, todas as combinações futuras herdam esse nível. Mitigação: tratar como prova técnica e pedir arte ou referência (`N4A_RECONSTRUCAO_ARTISTICA.md` §4).
2. **Bordas das peças** (médio): as chaves de cor se misturam na redução por IA; algumas sombras podem ter ido para a peça vizinha (por exemplo, um texel de sombra do cabelo na camisa). Não afeta o original recomposto; afeta a peça isolada e a troca por outra peça.
3. **Cano das botas estimado** (médio): a linha 90 é palpite visual.
4. **Escala do problema na corrida** (alto): o idle tem 1 quadro. A corrida tem 6, com o cabelo mudando de 177 para 318 texels e braços cruzando o torso. A reconstrução por quadro é 6× o trabalho, e as poses têm mais áreas escondidas (o braço de trás, a perna de trás).
5. **Peças volumosas** (médio): casaco, capa e armaduras largas mudam a silhueta; serão arte por quadro, não recorte.

## Técnicos

1. **Um sprite por slot** (baixo): se duas peças vestidas usarem o mesmo slot, `sync_composite_layers` mostra a última visível. O protagonista não tem conflito. Se a N4 precisar de duas peças no mesmo slot, basta trocar "um filho por slot" por "um filho por peça desenhada", sem mudar o contrato.
2. **Máscaras** (baixo por enquanto): nenhuma foi necessária no idle. A primeira necessidade provável é a bota sobre uma calça larga, ou uma luva sobre a manga. A recomendação continua: variante primeiro, máscara offline depois, sem shader.
3. **Desempenho** (baixo): 8 sprites com a mesma folha de 80×128 por camada. Não foi medido custo por frame, porque a prova tem um personagem só; medir quando entrar no jogo.
4. **`VisualPlayback.frame` e `flip`** (baixo): são preenchidos por `animate_player_visual`. Um sistema que leia esses campos antes dele no mesmo frame vê o valor do frame anterior. O compositor roda depois (ordem registrada no plugin).

## Fora do escopo, mantidos

Save v3, inventário, equipamentos funcionais, câmera, física, combate, `cargo run` padrão e o modo N3.8: inalterados.
