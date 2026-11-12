---
paths:
  - "assets/**"
  - "src/**/*.rs"
  - "docs/**"
---

# Regras de game design

- Inspiração mecânica: Metroidvania/Symphony of the Night. **Tudo** é original: nomes, história, personagens, inimigos, mapas, arte e áudio. Nunca copie nomes, sprites, músicas ou textos de jogos existentes.
- Atributos clássicos: STR, CON, INT, LCK, HP, MP, Hearts, EXP, Level; derivados documentados em `docs/CHARACTER_ATTRIBUTES.md`. Toda fórmula nova precisa ser documentada e testada.
- Técnicas e magias são ativadas por **sequências de comandos** definidas em dados (`command: Some("D DF F + ATTACK")`). Não crie um `if` por habilidade.
- Aprender uma habilidade ≠ conhecer o comando (`command_revealed_on_learn`).
- Comandos são relativos à orientação do personagem: `F` é "para frente".
- Arte temporária: formas coloridas geradas em código. A ausência de arte nunca bloqueia sistemas.
- Balanceamento muda em `assets/data/`, não no código.
