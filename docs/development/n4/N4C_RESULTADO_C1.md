# N4-1 — Resultado da C1 (três quadros)

Os quadros #0, #2 e #4 da corrida aprovada são, agora, 14 camadas independentes na mesma composição de dados do idle. A recomposição vestida é **idêntica texel a texel** (e pixel a pixel no Bevy), e cada peça sai durante a animação sem abrir buraco. A sincronia vale a 30, 60 e 144 fps e com taxa variável. A **anatomia reconstruída está abaixo da referência N4-B aprovada**, com defeitos artísticos importantes. Pelo briefing, a **C2 não foi iniciada** e depende da avaliação do proprietário.

## 1. Estado inicial

Local `4456341`; o remoto tinha `a5f7f1c` (upload da referência N4-B). Fast-forward, sem merge nem rebase. Não há N4-B no repositório além dessa imagem. `N4C_AUDITORIA.md`.

## 2. Arquitetura reutilizada

O mesmo contrato RON, o mesmo compositor (`sync_composite_layers`), o mesmo `VisualPlayback`. Nenhum código Rust de produção mudou: só dados (`protagonist.ron`, clipe `run_c1`, 6 camadas novas de membros de trás), testes e o exemplo.

## 3. Quadros modularizados

#0 contato (`video_048`), #2 passada aberta (`video_052`) e #4 recuperação (`video_056`), em tiras de 3 colunas (`stage/parts/run_c1/*.png|txt`). `N4C_ESTRATEGIA_SEPARACAO.md`.

## 4. Qualidade da anatomia

Construída por pose a partir da silhueta vestida de cada membro na arte aprovada (pose e movimento corretos), limitada a espessuras anatômicas, com núcleo ao longo dos ossos, sombreamento e contorno por membro, e membros de trás em sombra. **Defeitos:** tronco chapado e riscado pelo contorno do braço, sem músculos modelados, pés em cunha sem dedos, perna de trás fina, roupa de baixo irregular. `N4C_ANATOMIA_POSES.md`.

## 5. Peças removíveis

Cabelo (2 camadas), camisa (3), calça (2), botas (2) e braceletes (2), cada uma como **uma** peça lógica, mais o corpo-base (3 camadas, fixo). Remoção testada parada e durante a animação.

## 6. Diferenças em relação à N3.6

0 texels, 0 de silhueta, mesmo contato com o chão (118, 114, 118), mesmo pivô e 0 pixels no Bevy, inclusive espelhado. `N4C_DIFERENCAS_VISUAIS.md`.

## 7. Testes

fmt, check e clippy `-D warnings` limpos; `cargo test -j 2`: **54 suítes, 660 passaram, 0 falharam**; 5 testes novos. `N4C_TESTES.md`.

## 8. Evidências

`docs/development/n4/evidence/n4c/`: 4 GIFs (corrida modular, corpo-base em movimento, remoções animadas e original × modular), folhas de remoção, camadas, mapas de diferença, esqueletos, cavidades, 23 capturas do Bevy e `metrics.json`. `N4C_EVIDENCIAS.md`.

## 9. Bloqueios

1. A anatomia precisa de uma passada artística (ou de referência na mesma pose e resolução) para chegar à N4-B.
2. Perto e longe das pernas em #0 e #2 é interpretação (contralateral).
3. Soquetes: ficha preparada, com mãos ocultas não marcadas (`N4C_SOQUETES.md`). Faltam ângulo e frente/trás das mãos, e a confirmação de qual é a mão principal.

## 10. Commits e SHA

Push em `origin/art/v1.2-protagonist` (`a5f7f1c..929ede2`), sem force, merge ou rebase:

- `5905d06` feat(visual): add the n4-1 c1 modular run layers
- `75690b3` test(player): validate the n4-1 c1 modular run
- `fea9684` feat(visual): add the run c1 mode to the sprite machine proof
- `c9cee5e` docs(claude): list the n4-1 c1 run tests
- `929ede2dc16f87385b4dd91673acb3c357487f08` docs(n4): record the n4-1 c1 result and evidence (SHA remoto confirmado igual ao local)

O commit que grava esta seção é o registro da publicação; o hash remoto final é o HEAD depois do push dele. `Md-usario/` não foi alterado: a referência N4-B veio do commit `a5f7f1c` do proprietário.

## Gate C1

- [x] Três quadros modularizados
- [ ] Corpo anatômico coerente em cada pose: **coerente na pose e no movimento; qualidade abaixo da N4-B (defeitos listados)**
- [x] Peças removíveis
- [x] Recomposição comparada
- [x] Diferenças medidas (0)
- [x] Sincronização validada
- [x] Evidências produzidas
- [ ] Avaliação artística do proprietário

## Gate C2

**Bloqueado até a aprovação explícita da C1.** Nada da C2 foi feito.

## Decisões do proprietário

1. Avaliar `n4c_removal.png`, `n4c_original_vs_modular.gif`, `n4c_run_base_body.gif` e `N4_INTERACTIVE=1 cargo run --example n4_sprite_machine -- --run-c1`.
2. Decidir o caminho da anatomia: aceitar este nível para seguir à C2, pedir mais uma rodada do agente nos defeitos listados, ou fornecer arte/referência por pose.
3. Confirmar perto e longe das pernas em #0 e #2, e a mão principal (proposta: a direita, do lado da câmera).
4. Autorizar (ou não) a C2.
