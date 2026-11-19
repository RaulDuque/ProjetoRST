# N3.8 — Avaliação e gates

A aprovação artística é do proprietário. Abaixo, o que o agente verificou e o que fica pendente.

## Evidências (`evidence/`)

| Arquivo | Conteúdo |
|---|---|
| `n38_run_right_15fps.gif` | corrida para a direita no jogo, 405, tempo real (2 cs por quadro de GIF = 1 update de 20 ms) |
| `n38_run_left_15fps.gif` | corrida para a esquerda (espelhada) |
| `n38_idle_to_run.gif`, `n38_run_to_idle.gif` | partida e parada (cortes diretos) |
| `n38_run_turn.gif` | virar correndo |
| `n38_run_to_jump.gif`, `n38_run_to_attack.gif`, `n38_run_to_guard.gif`, `n38_run_to_dodge.gif` | ações a partir da corrida, com o selo de placeholder |
| `n38_run_with_cape.gif` | equipamento existente (capa) durante a corrida |
| `n38_production_vs_n3_run.gif` | produção V2.2 × N3 experimental, mesmo roteiro, mesmos updates |
| `n38_feet_alignment.png` | os 6 quadros no jogo com a linha da sola |
| `n38_camera_405_vs_360.png` | mesma cena nas duas câmeras |
| `n38_vs_n37_run6.png` | `run6.png` aprovado (N3.6/N3.7) × o que o jogo desenha a 360 |
| `n38_game_405_idle.png`, `n38_game_405_run.png`, `n38_game_405_jump_placeholder.png` | telas inteiras, com HUD e selo |
| `measurements.json` | fps medido, permanência por quadro, velocidade e verificação pixel a pixel |

Gerado por `art-src/n3-8-production-run/scripts/make_evidence.py` a partir de `target/n3_8_production_run/` (capturas brutas, não versionadas).

## Leitura do agente (não é aprovação)

- **Movimento:** a 15 fps, a corrida lê como corrida no jogo. Pela medição da N3.7, o pé desliza para a frente cerca de 37 %. No jogo não há régua: o deslizamento só pode ser julgado a olho contra o cenário em `n38_run_right_15fps.gif`, e o agente não o mediu de novo. Foi a escolha do proprietário, e a física não mudou.
- **Identidade:** idle e corrida são o mesmo personagem. As ações paradas na pose N3.5 deixam óbvio que faltam animações. É para isso que existe o selo.
- **Câmera 405:** a redução de 0,889 px por texel come detalhes finos (mechas, braceletes) com *nearest*. A 360 o desenho é exato. A câmera não mudou, e a escolha continua pendente desde a N3.5.
- **Produção a 40 fps:** a comparação mostra que o V2.2 corre a ~39–40 fps (24 fps × 310/190). O contraste de cadência entre os dois é grande.

## Decisões pendentes do proprietário

1. **Amostragem da produção:** o V2.2 sempre foi desenhado com filtro linear no jogo (borrado em posições fracionárias). A N3.8 não mexeu nisso. Passar a produção para *nearest* é uma linha (inserir `NearestPlayerArt` no app padrão), mas muda a aparência do V2.2. Precisa de autorização.
2. **Câmera 360** para o novo protagonista (continua só na prova).
3. **Arma no espelho** (troca de mão ou "mão da frente").
4. **Arquitetura N4** (`N3_8_ARQUITETURA_TERRARIA.md`, `N3_8_PLANO_MODULAR_N4.md`).

## Gates

### Gate A: integração

- [x] Corrida funciona em `cargo run`: **com `DARK_CASTLE_PROTAGONIST=n3`** (modo experimental, decisão do proprietário)
- [x] Seis quadros aprovados (bytes iguais; 24/24 capturas idênticas a 360)
- [x] Reprodução a 15 fps (medido no jogo: 15,0)
- [x] Velocidade de 310 u/s
- [x] Física inalterada (gameplay idêntico com as duas artes)
- [ ] **Promoção ao `cargo run` padrão: PENDENTE**

### Gate B: apresentação

- [x] Pivô correto (por clipe; 40,118 e 56,118)
- [x] Escala correta (densidade 2)
- [x] Espelhamento correto (âncora 0, sem deslocamento)
- [x] Alinhamento do chão (pés na linha da sola; quadro 0 1 texel acima, como aprovado)
- [x] Câmera principal 405 preservada (`camera.ron` intocado)
- [ ] Identidade artística preservada: **o agente não aprova**. Pixels idênticos; a avaliação final é do proprietário

### Gate C: compatibilidade

- [x] Idle funciona (pose N3.5)
- [x] Salto, ataque, guarda, esquiva, dano e morte: **funcionam no gameplay, com representação temporária** (pose N3.5 + selo). Não são animações
- [x] Equipamentos existentes sem regressão na produção; no modo experimental, a arma e o escudo ficam ocultos (sem soquetes) e a capa usa o ombro fixo V2.0

### Gate D: preparação N4

- [x] Arquitetura existente auditada
- [x] Conceito estilo Terraria documentado (proposta)
- [x] Regiões removíveis definidas
- [x] Corpo anatômico neutro especificado (requisitos, não arte)
- [x] Cobertura e sobreposição estudadas
- [x] Soquetes analisados (medição automática insuficiente: marcar à mão)
- [x] Compatibilidade com save estudada (vanity exige v4 autorizado)
- [x] Plano de migração definido

### Gate E: segurança

- [x] Arte original preservada (hash N3.7 + bytes N3.8)
- [x] Arquivos pessoais preservados (`Md-usario/` não tocado nem commitado)
- [x] Física, combate e save preservados
- [x] Testes executados (641/641)
- [x] Evidências reais
- [ ] GitHub publicado (marcado no registro de publicação, `N3_8_RELATORIO_FINAL.md` §Y)
