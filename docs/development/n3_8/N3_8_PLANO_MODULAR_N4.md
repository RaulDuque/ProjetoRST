# N3.8 — Plano de migração para a N4

**Proposta para aprovação. A N4 não começa sem autorização do proprietário.**

## 1. O que falta para substituir o V2.2 de vez

O jogo mostra hoje 15 clipes de personagem (`publish_guard.py`, `REQUIRED_CLIPS`) e 5 de efeito. O novo protagonista tem 2 aprovados.

| Clipe | V2.2 (produção) | Novo protagonista | Origem prevista |
|---|---|---|---|
| `idle` | 12 quadros animados | 1 quadro (N3.5) | aprovado; idle animado só com vídeo (`N3_7_BRIEFING_IDLE.md`) |
| `run` | 10 quadros | **6 quadros a 15 fps (N3.6)** | aprovado |
| partida (`run_start`) | — | 6 quadros extraídos (N3.7), não aprovados | precisa da regra de "próximo clipe" |
| frenagem | — | nenhum | vídeo de frenagem (`N3_7_BRIEFING_IDLE.md`) |
| `jump_rise`, `fall`, `land` | 4 + 4 + 4 | nenhum | vídeo (`N3_7_BRIEFING_SALTO.md`) |
| `crouch` | 3 | nenhum | vídeo novo |
| `guard_start` | 3 | nenhum | vídeo novo |
| `dodge` | 7 (0,28 s, `FitToAction`) | nenhum | vídeo novo |
| `hurt` | 9 (`FitToAction`) | nenhum | vídeo novo |
| `death` | 8 | nenhum | vídeo novo (bloqueante para promoção) |
| `attack_basic`, `attack_basic_air` | 7 + 7 | nenhum | vídeo (`N3_7_BRIEFING_ATAQUE.md`) |
| `slash_crescent`, `slash_rising` | 10 + 11 | nenhum | vídeo novo |
| `cast_forward`, `cast_up` | 8 + 15 | nenhum | vídeo novo |
| efeitos `vfx_*` | atlas próprio | reaproveitáveis | não dependem do corpo |
| soquetes de todos os clipes | medidos no Blender | nenhum | marcação manual por quadro |

Os clipes `FitToAction` precisam somar exatamente a duração da ação (`data_tests.rs` já cobra isso da produção).

## 2. Fases da prova vertical (plano da N4, não entregue na N3.8)

### Fase A: idle modular
Separar experimentalmente, **no quadro N3.5 só**: corpo anatômico, cabelo (traseiro e frontal), camisa, calça e botas, como folhas 80×128 com pivô 40,118. Critério: a recomposição comparada à N3.5 original texel a texel (meta: idêntica, ou diferenças aprovadas pelo proprietário).

### Fase B: troca de peças
No mesmo quadro, mostrar: aparência original; sem cabelo; sem camisa; sem calça; sem botas; corpo-base. Critério: o corpo neutro é arte de verdade, aprovada, e não pixels apagados.

### Fase C: corrida modular
Começar por 3 quadros representativos do `run6`: **#0** (vídeo 48, contato), **#2** (vídeo 52, passada aberta) e **#4** (vídeo 56, recuperação). Validar contato com o chão, cabelo, roupas, pivô e sobreposição. Só depois expandir para os 6 e tocar a 15 fps com o relógio único.

### Fase D: equipamentos
Uma armadura alternativa (cobre a camisa), uma arma alternativa (outra `WeaponForm` no soquete medido) e uma peça cosmética (aparência ≠ funcional, só em memória). Critério: as três são independentes, trocadas sem folha nova de corpo.

## 3. Ordem técnica sugerida

1. Contrato de camadas no manifesto, compatível com o atual: `layers` opcional por atlas, com o mesmo `cell` e `pivot` do corpo; manifestos sem `layers` continuam válidos (V2.2 e N3.8).
2. Uma entidade filha por camada, lendo `VisualPlayback` (relógio único). Testes: todas as camadas no mesmo índice em todos os updates; nenhum temporizador por camada.
3. Ocultação de camada + variantes (`N3_8_ARQUITETURA_TERRARIA.md` §4).
4. Soquetes novos medidos à mão.
5. Aparência cosmética em memória; save v4 só com autorização.
6. Regra de "próximo clipe" para partida e frenagem.

## 4. Gates para iniciar a N4

- [ ] Proprietário aprova esta arquitetura (camadas no mesmo canvas, relógio único, ocultação + variantes, sem esqueleto).
- [ ] Proprietário fornece ou aprova as referências do corpo neutro (§16 de `N3_8_RISCOS_ARTISTICOS.md`).
- [ ] Decide se a arma troca de mão no espelho (como hoje) ou segue "mão da frente".
- [ ] Decide se a produção também passa a amostragem *nearest* (`N3_8_AVALIACAO.md`).

## 5. Gate de promoção ao `cargo run` padrão (PENDENTE)

Só quando, para o novo protagonista: todos os 15 clipes acima estiverem aprovados (ou houver decisão explícita de manter algum como provisório), os soquetes estiverem medidos, a publicação passar por `publish_guard.py` e o proprietário aprovar a troca. Até lá, `DARK_CASTLE_PROTAGONIST=n3` é o único caminho.
