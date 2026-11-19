# N4-1-A — Limitações, riscos e próximos passos

## Limitações encontradas

1. **Qualidade artística dos peitorais.** A pintura procedural dá volume, luz, contorno, lâminas, aresta e friso coerentes com a paleta e o contorno do protagonista, mas as formas são quadradas: não seguem a curva do peito, e no ombro aparece uma linha reta de contorno. **Não está no nível da arte N3.5/N3.6.** Quem decide é o proprietário (Gate D).
2. **Máscaras da corrida dependem dos polígonos do tronco marcados à mão** (`run_poses.py`). Na C2, os quadros #1, #3 e #5 vão precisar de polígonos também.
3. **HIDE no idle:** impossível sem separar a manga da camisa do idle (dados N4-A).
4. **Máscaras são offline.** O jogo não recorta em tempo real (não há shader). Recortar é papel da ferramenta de autoria, antes de gravar o PNG. Se algum dia uma peça precisar de recorte dinâmico (por exemplo, uma capa recortada pelo cabelo que muda), aí sim o shader passa a fazer sentido.
5. **Um sprite por slot.** Duas peças vestidas no mesmo slot: só a última visível aparece. A família de peitorais usa um slot só; famílias diferentes precisam de slots próprios.
6. **Pontos de encaixe:** os peitorais não precisam, porque usam o canvas do corpo. Armas e escudos continuam esperando os soquetes manuais (`N4C_SOQUETES.md`).
7. **O pintor de autoria cresce com padrões novos.** Um padrão que ainda não existe é código Python; um item que usa padrões existentes é só dado.

## Riscos

| Risco | Nível | Mitigação |
|---|---|---|
| Aceitar a pintura procedural como arte final | alto | tratar como prova de fluxo; para produção, o artista pinta sobre as máscaras e o verificador confere |
| Famílias com silhueta própria (ombreiras, casaco, capa longa) | médio | família nova com máscara própria; não esticar a `torso_plate` |
| HIDE inseguro em itens futuros | baixo | o verificador recusa; o teste exige "todo texel oculto sob o item" |
| Crescimento de slots | baixo | um slot por família, declarado na composição |
| Desempenho | baixo | +1 sprite por família vestida; não medido (prova isolada) |

## Próximos passos recomendados (não iniciados)

1. Avaliação do proprietário das evidências e do modo interativo (`N4_INTERACTIVE=1 cargo run --example n4_sprite_machine -- --equipment --run-c1`, tecla E).
2. Decidir quem pinta a arte final dos itens: o agente refina o pintor, ou um artista pinta sobre as máscaras.
3. Se aprovado: uma segunda família (botas por REPLACE, ou capacete por HIDE do cabelo) para provar REPLACE e HIDE de região nos dados reais.
4. C2 (6 quadros da corrida) **só com autorização**. Com a base vestida, a C2 precisa da partição dos 6 quadros e dos polígonos do tronco; a reconstrução anatômica deixa de ser requisito.
