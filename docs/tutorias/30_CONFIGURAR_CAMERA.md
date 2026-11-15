# 30 — Configurar a câmera

> Situação: **funciona** (V2.3). Os valores vêm de `assets/data/camera.ron`.
> **Pendente:** ajuste por sala ou por região (hoje a câmera é uma só).

## 1. O que vamos fazer?

Entender e ajustar como a câmera segue o personagem: zona morta, olhar à
frente, enquadramento de pulo e de queda.

## 2. O que você precisa saber antes?

- A câmera **nunca** sai da sala: ela para nas bordas (salas pequenas ficam
  centralizadas).
- **Zona morta:** pequenos movimentos dentro dela não movem a tela.
  - `dead_zone_x`: metade da largura, para os lados.
  - `dead_zone_up` / `dead_zone_down`: no ar, a tela só sobe/desce quando o
    personagem passa desses limites. No chão, a tela recentraliza.
- **Olhar à frente** (`look_ahead`): a tela mostra mais do lado para onde o
  personagem olha; `look_ahead_rate` é a rapidez com que ela vira.
- **Queda:** caindo mais rápido que `fall_speed`, a tela olha para baixo
  `fall_look_down` unidades, para ver onde vai cair.
- `follow_rate_x` / `follow_rate_y`: rapidez do acompanhamento (maior = mais firme).
- Teleporte (renascer, carregar, trocar de sala): a câmera vai direto, sem deslizar.
- Física e colisões não mudam com a câmera.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/data/camera.ron` | Os valores da câmera |
| `src/world/camera_rules.rs` | Só leitura: regras e limites aceitos |

## 4. Passo a passo

1. Abra `assets/data/camera.ron`.
2. Mude **um** valor por vez.
3. Rode a vitrine (seção 6) e compare as capturas `motion_*`.
4. Rode os testes.

## 5. Exemplo completo

Uma câmera mais "solta" para salas verticais (todos os campos são opcionais;
o que faltar usa o padrão):

```ron
// exemplo-verificado: Camera
(
    view_height: 405.0,
    follow_rate_x: 7.0,
    follow_rate_y: 4.0,
    dead_zone_x: 24.0,
    dead_zone_up: 110.0,
    dead_zone_down: 40.0,
    look_ahead: 70.0,
    look_ahead_rate: 2.5,
    vertical_offset: 40.0,
    fall_look_down: 90.0,
    fall_speed: 420.0,
)
```

| Campo | Efeito |
|---|---|
| `dead_zone_up: 110.0` | Pulos normais não movem a tela |
| `fall_look_down: 90.0` | Mostra mais do chão numa queda longa |
| `follow_rate_y: 4.0` | A subida/descida da tela é mais suave |

## 6. Como testar?

1. `cargo test -j 2 --lib camera_rules` e `cargo test -j 2 --test world_camera_tests`.
2. `cargo run -j 2 --example world_visual_showcase` e veja
   `target/world_visual_showcase/motion_run.png`, `motion_jump_apex.png`,
   `motion_fall.png` e `motion_landed.png` (tutorial 35).
3. `cargo run -j 2` e jogue nas salas `vigil_gates_causeway` e `fg_stair`.

## 7. Como editar depois?

Volte o valor anterior ou apague a linha (vale o padrão). Os padrões estão no
próprio arquivo e em `docs/WORLD_VISUAL_V2_3.md` §2.

## 8. Problemas comuns

| Mensagem ou sintoma | Causa | Solução |
|---|---|---|
| `invalid camera settings; defaults used` (log) | Valor fora do limite ou nome errado | Corrija; o jogo usa os padrões enquanto isso |
| `camera setting ... out of range` (teste) | Ex.: zona morta maior que meia tela | Diminua o valor |
| A tela treme ao pular | `dead_zone_up` pequena | Aumente `dead_zone_up` |
| Não vejo obstáculos à frente | `look_ahead` pequeno ou `follow_rate_x` baixo | Aumente um deles |

## 9. Como desfazer com segurança?

Restaure `assets/data/camera.ron` (os valores do exemplo acima com
`dead_zone_up: 90.0`, `follow_rate_y: 5.0` e `fall_look_down: 70.0` são os
padrões). Sem o arquivo, o jogo usa os padrões.
