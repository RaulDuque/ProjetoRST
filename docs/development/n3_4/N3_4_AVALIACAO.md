# N3.4 — Avaliação

**O estilo não está aprovado.** Esta é a autoavaliação do agente, para orientar a decisão do proprietário.

## Gate A — Fidelidade artística (decisão do proprietário)

- [x] `image.png` foi visualmente inspecionado (e também as duas fichas).
- [ ] Identidade preservada: **a avaliar.** Cabelo, roupa e silhueta seguem a ficha. O rosto é esquemático.
- [ ] Cabelo branco-prateado longo: **a avaliar.** Está presente e legível em três fundos. A coroa ainda parece lisa.
- [ ] Rosto reconhecível: **a avaliar.** Em close, há perfil com nariz, boca e queixo. Em 1×, o rosto não se distingue.
- [ ] Corpo proporcional: **a avaliar.** As linhas de proporção batem com a ficha (`reference_analysis.png`).
- [ ] Roupa-base coerente: **a avaliar.** Camisa, cinto, calça, bracelete e botas estão lá. A calça e o cano da bota ainda têm colunas retas.
- [ ] Sprite não parece manequim: **a avaliar.** Ao contrário da N3.3, há anatomia, mas falta acabamento de pintura.
- [ ] Arte legível na escala do jogo: **não, com a câmera atual.** Em 405, o sprite é reduzido a 0,89 px por texel (`N3_4_RESOLUCAO.md`).
- [ ] Comparação visual publicada: as folhas estão em `evidence/` e serão publicadas no push desta etapa.

## Gate B — Preparação técnica

- [x] Canvas documentado (80×128).
- [x] Pivô documentado (`(40, 118)`, apoio real nas colunas 31–47).
- [x] Transparência correta (alfa só 0 ou 255).
- [x] Arquivo-fonte editável (`source/base_idle.txt`). É texto, não `.pxo` nem `.aseprite`.
- [x] Paleta documentada (19 cores).
- [x] Nenhum elemento removível incorporado. A exceção é a calça e as botas, que estão na base por decisão de prioridade 1 (`N3_4_PERSONAGEM_BASE.md`).

## Gate C — Integração isolada

- [x] Sprite carregado no Bevy (`bevy_n34.png`).
- [x] Escala correta no exemplo (2× inteiro na visão fechada).
- [x] Pivô correto (sola no chão do exemplo).
- [x] Sem regressões (607 testes; capturas antigas idênticas).
- [x] Jogo principal intacto.

Aprovação técnica não é aprovação artística.

## Recomendação

**Apresentar para avaliação, com correções conhecidas.** O quadro é a primeira versão que parece o protagonista da ficha. Antes de qualquer animação, o proprietário decide:

1. se a identidade está preservada (Gate A);
2. se a câmera vai para 360 ou se fica em 405 com a perda (`N3_4_RESOLUCAO.md`);
3. se calça e botas fazem parte da base ou serão peças removíveis.

Correções sugeridas para uma próxima passada: fios soltos na coroa, dobras diagonais na calça, volume no cano da bota, sombra no antebraço. De preferência, por um artista no Pixelorama, partindo de `base_idle.png`.
