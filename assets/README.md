# Assets

Somente arquivos com finalidade real entram aqui. Pastas novas são criadas
quando o primeiro asset correspondente existir.

| Pasta | Conteúdo |
|---|---|
| `data/character.ron` | Curva de EXP, crescimento por nível, coeficientes das fórmulas, personagem inicial |
| `data/skills/` | Técnicas físicas (formato `SkillDefinition`) |
| `data/spells/` | Magias (mesmo formato; executadas pelo sistema de magia) |
| `data/items/` | Raridades e itens (`ItemDataFile`) |
| `data/enemies/` | Arquétipos de inimigos (`EnemyDefinition`) |
| `maps/` | Salas (`RoomDefinition`) |

Planejadas (ainda sem arquivos): `sprites/{player,enemies,bosses,effects,environment}`,
`animations/`, `audio/{music,sfx}`, `fonts/`, `ui/`.

Enquanto não houver arte, o jogo usa formas coloridas geradas em código.
Nunca adicione arte ou áudio protegidos por direitos autorais.
