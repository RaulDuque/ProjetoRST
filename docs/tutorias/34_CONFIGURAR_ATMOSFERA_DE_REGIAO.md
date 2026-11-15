# 34 — Configurar a atmosfera de uma região

> Situação: **funciona** (V2.3) em Vigil Gates, Forgotten Galleries e Village
> of Ashes. Veiled Cloister continua sem `visual` (visual da V2.2).
> **Pendente:** iluminação dinâmica de verdade (hoje são brilhos suaves).

## 1. O que vamos fazer?

Dar a uma região sua identidade: fundo, cor da pedra, cor da luz, céu e névoa.

## 2. O que você precisa saber antes?

- Fica em `visual: Some((...))` dentro de `region: (...)`, no arquivo da região.
- Cores de 0.0 a 1.0.
- `stone`: pedra da arquitetura (a decoração usa 80% dela; sempre mais escura que as plataformas).
- `light`: cor de tochas, janelas e lanternas.
- `sky`: céu/breu do fundo.
- `fog`: névoa baixa, opcional; `density` de 0 a 0.35 (o chão continua legível).
- As três regiões atuais:

| Região | `backdrop` | Clima |
|---|---|---|
| `vigil_gates` | `Castle` | Pedra fria, luz de tocha contida |
| `forgotten_galleries` | `Ruins` | Breu esverdeado, luz espectral, névoa densa |
| `ash_village` | `Village` | Entardecer quente, janelas acesas |

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/world/regions/<região>.ron` | O `visual` |
| `src/world/scenery.rs` | Só leitura: formato e limites |

## 4. Passo a passo

1. Abra o arquivo da região.
2. Em `region: (...)`, depois de `color`, acrescente `visual: Some((...))`.
3. Teste com `world_report` e a vitrine.

## 5. Exemplo completo

O visual da Vigil Gates:

```ron
// exemplo-verificado: VisualRegiao
(backdrop: Castle, stone: (0.34, 0.33, 0.4), light: (1.0, 0.68, 0.36),
    sky: (0.07, 0.07, 0.13), fog: Some((color: (0.45, 0.45, 0.55), density: 0.12)))
```

## 6. Como testar?

1. `cargo run -j 2 --example world_report` (0 erros).
2. `cargo test -j 2 --test world_scenery_tests`.
3. `cargo run -j 2 --example world_visual_showcase` (captura `after_approach`:
   sala sem decoração, só a atmosfera da região).

## 7. Como editar depois?

Mude cores e névoa. Mantenha `stone` escura (luminância abaixo da das
plataformas: o teste `decor_stone_stays_darker_than_the_platforms` ajuda).

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `BadScenery(Colour ...)` | Canal fora de 0–1 | Corrija a cor |
| `BadScenery(Fog ...)` | `density` acima de 0.35 | Diminua |
| `unknown field` | Nome de campo errado | Só `backdrop`, `stone`, `light`, `sky`, `fog` |

## 9. Como desfazer com segurança?

Apague `visual: Some((...))`: a região volta ao visual da V2.2.
