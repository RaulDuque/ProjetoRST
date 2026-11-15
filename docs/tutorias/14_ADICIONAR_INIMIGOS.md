# 14 — Adicionar inimigos

> Situação: **funciona**. **Pendente:** inimigos ainda são retângulos coloridos
> (sem arte) e todos usam a mesma IA (patrulha, perseguição, ataque).

## 1. O que vamos fazer?

Criar um tipo de inimigo novo (**Crypt Ghoul**) e colocá-lo numa sala.

## 2. O que você precisa saber antes?

- **Tipo** de inimigo: em `assets/data/enemies/*.ron` (uma lista `[ ... ]`).
- **Onde ele aparece:** em cada sala, `enemies: [ (enemy: "id", position: (x, y)) ]`.
- `position.y`: o inimigo cai até o chão; use ~70 para o chão.
- `patrol_half_width`: quanto anda para cada lado (padrão 120).
- `drops`: itens que podem cair, com chance de 0.0 a 1.0 (aumenta com a sorte).
- `respawn_time`: segundos para reaparecer (0 = nunca).

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/data/enemies/crypt_dwellers.ron` | **Novo.** O tipo de inimigo |
| `assets/data/enemies/castle_undead.ron` | Existe. Modelo: Bone Sentry |
| `assets/world/regions/ashen_crypt.ron` | Onde ele aparece |

## 4. Passo a passo

1. Crie `assets/data/enemies/crypt_dwellers.ron` (seção 5).
2. Na sala, acrescente em `enemies` a posição dele.
3. Teste.

## 5. Exemplo completo

```ron
// exemplo-verificado: Inimigos
// Inimigos da região de exemplo Ashen Crypt.
[
    (
        id: "crypt_ghoul",
        name: "Crypt Ghoul",
        max_hp: 40,
        attack: 10,
        defense: 2,
        resistances: (values: [(Light, -0.5), (Dark, 0.3)]),
        move_speed: 70.0,
        size: (30.0, 44.0),
        attack_cooldown: 1.6,
        attack_hitbox: (36.0, 26.0),
        contact_damage_multiplier: 0.4,
        experience: 18,
        drops: [(item: "crypt_draught", chance: 0.2)],
        ai: (aggro_range: 220.0, leash_range: 380.0, attack_range: 40.0, flee_below_hp: 0.0),
        color: (0.55, 0.6, 0.5),
        respawn_time: 8.0,
    ),
]
```

Na sala:

```ron
// exemplo-verificado: SpawnInimigo
(enemy: "crypt_ghoul", position: (1000.0, 70.0), patrol_half_width: 120.0)
```

| Campo | Significado |
|---|---|
| `max_hp`, `attack`, `defense` | Vida, ataque, defesa |
| `resistances` | Negativo = fraqueza (`-0.5` leva 50% a mais) |
| `move_speed` | Velocidade |
| `size` | Tamanho do corpo |
| `attack_cooldown`, `attack_hitbox` | Intervalo e tamanho do golpe |
| `contact_damage_multiplier` | Dano ao encostar (0 = sem dano) |
| `experience` | EXP ao derrotar |
| `ai` | Distâncias para notar, desistir, atacar; fugir com pouca vida |
| `color` | Cor do retângulo |

## 6. Como testar?

1. `cargo test -j 2 --test data_tests`.
2. `cargo run -j 2`: entre na sala e lute.
3. `cargo test -j 2 --test tutorial_examples_tests` (o teste derrota o Ghoul).

## 7. Como editar depois?

Mude os números e teste. Para tirá-lo de uma sala, apague a linha de `enemies`.

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `room ... spawns unknown enemy` | Sala usa um `id` que não existe | Crie o tipo ou corrija o nome |
| `invalid data in .../enemies/...` | Campo obrigatório faltando | Compare com o exemplo |
| Inimigo some no chão / flutua | `position` fora da sala | Ajuste x e y |

## 9. Como desfazer com segurança?

Tire o inimigo de todas as salas e apague o arquivo novo.
