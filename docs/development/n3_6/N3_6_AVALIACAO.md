# N3.6 — Avaliação

Autoavaliação do agente. **Nenhum gate artístico foi aprovado.** Os itens marcados são verificações técnicas com evidência, e os pendentes dependem do proprietário.

## Gate A — Identidade (proprietário)

- [ ] O protagonista mantém o cabelo original: o vídeo reduzido mantém a massa prateada e o comprimento (`run_hair_comparison.png`).
- [ ] O rosto permanece reconhecível: o perfil está presente em todos os quadros, mas é pequeno em 1×.
- [ ] Proporções consistentes: mesma escala em todos os quadros, com o topo da cabeça variando 2 linhas.
- [ ] Roupas no estilo: camisa cinza, calça e botas na paleta N3.4.
- [ ] A animação não troca a identidade: todos os quadros vêm de um único vídeo.

## Gate B — Consistência dos quadros

- [x] Quadros com procedência documentada (`N3_6_QUADROS.md`).
- [x] Sem deformação arbitrária: escala única e nenhum quadro redimensionado ou deslocado sozinho.
- [ ] Cabelo com continuidade: avaliação do proprietário.
- [ ] Pernas alternando de forma coerente: dois passos espelhados no ciclo, mas a avaliação é do proprietário.
- [x] Pivô consistente: um só pivô e um só deslocamento.
- [ ] Ciclo sem saltos visuais graves: 58 → 48 fecha no vídeo (o 60 repete o 48). A avaliação final é visual.

## Gate C — Apresentação

- [x] Resolução documentada (112×128, figura de 106).
- [x] Escala documentada (densidade 2, 53 unidades).
- [x] Transparência correta (alfa binário).
- [x] Fundo escuro testado, com borda de luz só como teste.
- [x] Câmera 405 testada (reduz).
- [x] Câmera 360 testada (1:1 exato).

## Gate D — Integração isolada

- [x] A animação carrega no Bevy.
- [x] Dá para pausar (Espaço).
- [x] Dá para avançar quadros (`.`).
- [x] O jogo principal não mudou.
- [x] As provas anteriores continuam funcionando (capturas idênticas, teste de compatibilidade).

## Modularidade futura: riscos

- **Largura:** a passada ocupa 85 texels. Uma casaca longa ou capa precisa acompanhar as pernas abertas e ficaria maior que o canvas de 112 se esvoaçar. Elas precisarão de canvas e pivô próprios, alinhados a (56, 118).
- **Oclusão do cabelo:** o cabelo cobre as costas. Capa e casaca teriam de passar entre o cabelo e o torso: são três camadas (cabelo, capa, corpo) e hoje o cabelo está assado no corpo.
- **Mãos:** o punho muda de lugar em cada quadro. Uma espada exige um soquete por quadro, que não existe para a corrida.
- **Torso:** a inclinação muda pouco, mas o ombro sobe e desce. Ombreiras e elmo precisariam de soquetes de ombro e cabeça.
- **Sincronização:** o `run10` tem durações desiguais. As camadas de equipamento teriam de seguir o mesmo índice e o mesmo tempo (o `VisualPlayback` atual já faz isso para o corpo).
- **Orientação:** o espelho funciona no exemplo, mas troca o lado do bracelete e do cabelo. É aceitável num corpo simétrico, e menos aceitável com escudo num braço só.
