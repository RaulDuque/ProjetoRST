# N3.7 — Avaliação

Autoavaliação do agente. **A aprovação artística é do proprietário.** Os itens marcados são verificações técnicas com evidência. Os itens pendentes dependem de olhar.

## Gate A — Preservação artística

- [x] Identidade N3.5 preservada: candidato byte a byte igual (teste FNV).
- [x] Corrida N3.6 preservada: os seis quadros, o manifesto e as grades estão byte a byte iguais. O pipeline os reproduz texel a texel.
- [ ] Rosto reconhecível nas variantes: as variantes B e C só mexem do quadril para baixo. Falta o olhar do proprietário.
- [ ] Cabelo consistente na partida (`run_start`): vem do mesmo vídeo, com a mesma escala. Falta o olhar do proprietário.
- [x] Roupas consistentes: mesma paleta, sem cor nova.
- [x] Nenhuma simplificação: nenhuma redução de paleta além da N3.4, nenhum bloco, nenhum redesenho.

## Gate B — Sincronização

- [x] Deslizamento dos pés medido (`N3_7_DESLIZAMENTO.md`): 157 u/s de referência, e a estimativa de 78 da N3.6 foi corrigida.
- [x] Velocidade do jogo comparada: 310 / 157 = 1,97.
- [x] Alternativas testadas: A, B, C e D em GIF, folha e Bevy. E com custo registrado.
- [x] Impacto visual documentado (cadência, deslizamento, ciclo).
- [x] Recomendação fundamentada: B com `speed_reference` (sem mudar a física) ou, se a cadência incomodar, E (vídeo novo).
- [ ] Escolha do cenário: proprietário.

## Gate C — Contraste

- [x] Fundo claro testado.
- [x] Fundo escuro testado (cinza, quase preto e cena real do castelo).
- [x] Contorno experimental testado: B (borda de luz) e também C e D.
- [x] Arte original preservada (variantes em arquivos novos).
- [x] Comparação publicada (`dark_comparison.png`, `.gif`, métricas).
- [ ] Escolha (B, D ou nenhuma): proprietário.

## Gate D — Transição

- [x] Parado e corrida alinhados pelo pivô, sem ajuste por quadro.
- [x] Troca de estado testada (3 sequências em GIF, loop no Bevy).
- [x] Diferenças de proporção documentadas (cabeça, quadril, peito, cabelo, chão).
- [x] Problemas de continuidade identificados: a parada não tem material, o cabelo dá um salto na troca direta e o quadro 0 fica 1 texel acima do chão.
- [ ] A partida real convence? Proprietário.

## Gate E — Pipeline

- [x] Extração reproduzível (a reconstrução da N3.6 sai idêntica).
- [x] Proveniência registrada por quadro.
- [x] Escala consistente (uma escala por vídeo, a mesma da corrida).
- [x] Pivô consistente (deslocamento global único).
- [x] Manifestos válidos (`AnimationManifest::from_ron`).
- [x] Testes automatizados (7 novos).

## Gate F — Segurança

- [x] Jogo principal inalterado (`git diff` vazio em `src/` e `assets/`).
- [x] Câmera principal inalterada.
- [x] N3.5 preservada.
- [x] N3.6 preservada.
- [x] Arquivos pessoais preservados (SHA-256 dos 11 arquivos de `Md-usario/assets-user/prota/base/` iguais aos da N3.6; nada de `Md-usario/` commitado).
- [x] Testes aprovados (623/623).
- [x] GitHub publicado (`N3_7_RELATORIO_FINAL.md`, seção X).

## Modularidade futura (N4): requisitos e riscos

- **Cabelo:** está assado no corpo, e na corrida ele ocupa até 318 texels atrás do tronco. Uma capa teria de passar entre cabelo e costas. Separar o cabelo exige uma camada própria no vídeo (por exemplo, um segundo vídeo sem cabelo), e não um recorte por regra, que danificaria a arte aprovada.
- **Capa:** a passada abre 85 texels, e o canvas de 112 não tem folga para uma capa esvoaçante. A capa precisaria de um canvas próprio, alinhado ao pivô (56, 118).
- **Mãos e soquetes de arma:** o punho muda de lugar em todo quadro. Um soquete por quadro poderia ser estimado no mesmo estilo da análise dos pés (região do punho em resolução cheia), mas seria uma estimativa experimental e não foi criado.
- **Ombros e cabeça:** o topo da cabeça varia de 12 a 16 entre parado e corrida, e o peito avança até 4,5 texels. Elmo e ombreiras precisariam de soquetes por quadro.
- **Orientação:** o espelho troca o lado do bracelete e o lado do cabelo. Funciona num corpo simétrico, não com escudo num braço só.
- **Sincronização das camadas:** com B (`speed_reference`), todas as camadas teriam de usar o mesmo índice e o mesmo tempo escalado. O `VisualPlayback` já faz isso para o corpo.
- **Largura variável:** parado ocupa 22 texels de largura, a partida até 83 e a corrida até 85. Equipamentos largos devem caber no pior quadro.
