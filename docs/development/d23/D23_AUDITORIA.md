# D23 — Auditoria

Branch `art/v1.2-protagonist`, HEAD `a9b8f96`. A árvore já tinha D17–D22
por commitar. Nada disso foi revertido. `Md-usario/` não foi editado.
`saves/` segue com mtime 2026-11-11. `white_mesh.glb` segue com mtime
2026-11-16.

O que a D22 já entregava e a D23 reutilizou:

- Escada one-way no Reliquary (cinco plataformas, vão ~110, primeiro degrau
  a ~126 do chão). Um pulo cheio sobe ~136 no centro.
- Porta `Top` do Reliquary ↔ porta `Bottom` da Ashen Court. O validador
  exige lados opostos e células vizinhas; a porta da corte não pode ir
  para a parede.
- Chegada `Bottom` em (640, 72), dentro do vão do chão (x 540–740). Sem
  chão, o corpo caía de volta. Esse era o buraco de acesso.
- Chefe em (1040, 72), patrulha ±80, na laje direita.
- Wind-up do chefe não é apagado por dano (D22). A contagem, porém, era
  pulada enquanto `flash > 0`.
- Cores de preparação já existiam, mas o flash branco vinha depois e
  cobria o corpo. O quadro `d22/evidence/04_phase2.png` estava branco
  durante "Phase shift".
- Câmera: altura visível 405, aspecto 16:9, largura ~720, look-ahead 70.
- Esquiva 0,28 s só a partir da locomoção. Guarda com A, redução 0,7,
  coberta pelo teste da D22 `guard_chips_a_frontal_hit_without_stagger`.
- Morte: R revive no spawn da sala com HP/MP cheios. O chefe é removido
  e só volta se o jogador sair e entrar de novo.
- Cura legítima: 3 `minor_tonic` (+40 HP) e 1 `minor_ether`. O inventário
  (`I`) pausa o jogo (`GameState::Inventory`).

O probe da D22 enche a vida do jogador. A D23 não usa esse resultado como
prova de que uma pessoa vence.
