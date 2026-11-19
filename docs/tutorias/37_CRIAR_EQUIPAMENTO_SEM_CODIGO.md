# 37 — Como criar um equipamento novo sem alterar código

> Situação: **experimental** (N41-B). É a fábrica de equipamentos da prova isolada da máquina de sprites (`n4_sprite_machine`). O jogo normal (`cargo run`) não usa estes itens.

## 1. O que vamos fazer?

Criar um peitoral novo para o protagonista escrevendo **só um arquivo de texto** (uma "receita" em JSON). Uma ferramenta lê a receita, desenha o peitoral em pixel art, cria o arquivo que o jogo entende (RON) e confere se está tudo certo. Depois você vê o peitoral no Bevy.

Também dá para criar, do mesmo jeito, uma **cor de material** nova (por exemplo, cobre) ou um **ornamento** novo (por exemplo, um brasão pequeno).

## 2. O que você precisa saber antes?

- Uma **receita** diz: qual modelo (template), qual material, quais ornamentos e qual número de sorte (seed).
- **Template** é a forma da peça. Para o tronco existem 4: `segmented_cuirass` (couraça com lâminas), `ridged_breastplate` (peitoral curto com quilha), `brigandine_coat` (brigantina com saia) e `scale_hauberk` (cota de escamas).
- **Material** é o conjunto de cores (contorno, sombra, base, luz, brilho). Existem 17, em `art-src/n4-sprite-machine/factory/materials/`.
- **Detalhe** é um ornamento: rebite, fivela, emblema, friso, costura, desgaste. Existem 16, em `factory/details/`.
- A **seed** só muda o que é sorteado (o desgaste). A mesma receita gera sempre o mesmo desenho.
- Só peças de **tronco** entram no personagem. Espadas, capas e ombreiras são geradas como imagens soltas (ainda não há ponto de encaixe confirmado).
- No idle, o tronco visível é estreito: ornamentos grandes podem não caber e são pulados (a ferramenta avisa).

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `art-src/n4-sprite-machine/factory/recipes/<id>.json` | **a receita nova** (você cria) |
| `art-src/n4-sprite-machine/factory/materials/<id>.json` | material novo (opcional) |
| `art-src/n4-sprite-machine/factory/details/<id>.json` | ornamento novo (opcional) |
| `art-src/n4-sprite-machine/stage/factory/` | o que a ferramenta gera (não edite à mão) |
| `docs/development/n4/evidence/n41b/` | catálogo, relatórios e capturas |

## 4. Passo a passo

1. Copie uma receita parecida. Exemplo: copie `factory/recipes/ridged_breastplate_bronze.json` para `factory/recipes/ridged_breastplate_copper.json`.
2. Abra a cópia e troque o `"id"` para o **mesmo nome do arquivo**, sem `.json`: `"ridged_breastplate_copper"`.
3. Troque o `"name"`, o material em `"materials": {"main": ...}` e a `"seed"` (qualquer número inteiro).
4. (Opcional) Se o material não existir, crie `factory/materials/copper.json` copiando outro material e trocando as cores (veja o exemplo completo).
5. No terminal, na pasta do projeto, rode a fábrica:
   `python3 -B art-src/n4-sprite-machine/scripts/factory.py`
6. Leia o resultado. Deve aparecer `"generated": 1` e `"failures": []`. Os outros itens aparecem como `cached` (não foram refeitos).
7. Gere os relatórios e o catálogo:
   `python3 -B art-src/n4-sprite-machine/scripts/factory_report.py`
8. Veja o peitoral no Bevy (o item novo entra sozinho no roteiro de capturas):
   `cargo run --example n4_sprite_machine -- --factory`
9. Abra `docs/development/n4/evidence/n41b/bevy_n41b_idle_ridged_breastplate_copper.png`.

## 5. Exemplo completo

Material novo (`factory/materials/copper.json`):

```json
{
  "schema": 1,
  "id": "copper",
  "kind": "metal",
  "description": "Cobre avermelhado.",
  "tones": {
    "outline": [40, 20, 14],
    "deep": [70, 34, 20],
    "shadow": [104, 52, 30],
    "base": [146, 80, 46],
    "light": [186, 116, 70],
    "highlight": [222, 160, 112]
  },
  "gloss": 0.5,
  "roughness": 0.2,
  "contrast": 1.0,
  "wear": 0.0
}
```

Receita (`factory/recipes/ridged_breastplate_copper.json`):

```json
{
  "schema": 1,
  "id": "ridged_breastplate_copper",
  "category": "chest_armor",
  "family": "torso_plate",
  "name": "Peitoral de quilha de cobre",
  "template": "ridged_breastplate",
  "materials": {"main": "copper"},
  "details": [
    {"detail": "double_trim", "material": "gold", "place": {"edge": "bottom"}},
    {"detail": "emblem_tower", "material": "silver", "place": {"at": [0.35, 0.4]}}
  ],
  "generation": {"seed": 3001, "animation_profile": "humanoid_torso"},
  "covers": ["Chest"],
  "hides": "auto",
  "mirroring": "flip",
  "fallback": {"missing_clip": "hide"}
}
```

Os dois itens da prova real estão em `factory/recipes/ridged_breastplate_gilded_steel.json` e `factory/recipes/scale_hauberk_gilded_steel.json` (com o material `gilded_steel` e o ornamento `emblem_tower`, também criados só com dados).

## 6. Como testar?

- A fábrica sai com erro e explica o motivo se algo estiver errado (por exemplo: `unknown material 'coper' for slot main`).
- `factory_report.py` mostra `technical_errors` (deve ser 0) e grava `docs/development/n4/evidence/n41b/qa.json`. Procure o seu item lá: `TECHNICALLY_VALID` ou `REVIEW` (tem aviso artístico para olhar).
- `cargo test --test n41b_factory_tests -j 2` confere que o item carrega no jogo, não abre buraco e não flutua.
- O exemplo do Bevy termina com `RESULT: PASS`.
- **Aprovação artística é sua**: olhe o catálogo `n41b_catalog.png` e a captura.

## 7. Como editar depois?

- Mude a receita e rode `factory.py --rebuild` (sem `--rebuild` a ferramenta **recusa** sobrescrever, para não perder nada sem querer).
- Para trocar só o desgaste, mude a `seed`.
- Para mudar onde fica um ornamento, mude `"at": [u, v]`: `u` vai de 0 (costas) a 1 (frente), `v` de 0 (alto) a 1 (baixo).
- Rebites ao longo das lâminas: `"place": {"along": "seams", "u": [0.25]}`. Frisos: `"place": {"edge": "top"}` ou `"bottom"`.

## 8. Problemas comuns

| Mensagem | O que fazer |
|---|---|
| `id must equal the file name` | o `"id"` precisa ser igual ao nome do arquivo |
| `materials must fill exactly the slots ['main']` | o tronco tem um só material: `"main"` |
| `unknown detail` / `unknown material` | confira o nome em `factory/details/` ou `factory/materials/` |
| `pure black outline is not allowed` | use um contorno colorido escuro, não `[0, 0, 0]` |
| `refusing to overwrite (use --rebuild)` | você mudou algo já gerado; rode com `--rebuild` |
| aviso `stamp(s) skipped (do not fit inside the mask)` | o ornamento não cabe naquele quadro (comum no idle); diminua ou mova |
| `animation_profile must be 'humanoid_torso'` | o perfil é o do template; copie o da receita original |

## 9. Como desfazer com segurança?

1. Apague o arquivo de receita que você criou (e o material/ornamento novos, se criou).
2. Apague os arquivos gerados do item: `stage/factory/items/<id>.ron`, a pasta `stage/factory/art/<id>/`, `stage/factory/preview/<id>.png` e `.txt` e `stage/factory/provenance/<id>.json`.
3. Rode `python3 -B art-src/n4-sprite-machine/scripts/factory.py` de novo: deve mostrar só `cached`.
4. Não use `git reset --hard` nem `git clean`: há trabalho seu não commitado no projeto.
