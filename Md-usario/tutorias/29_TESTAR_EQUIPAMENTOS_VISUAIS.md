# 29 — Testar equipamentos visuais

> Situação: **funciona** (a arte sem sabre foi publicada na V2.2).

## 1. O que vamos fazer?

Conferir, com testes e capturas, que a arma mostrada é a equipada, que o
escudo respeita as duas mãos e que tudo acompanha as animações.

## 2. O que você precisa saber antes?

- **Arte publicada** (`assets/`, V2.2): corpo sem arma + âncoras; camadas
  de arma e escudo **ligadas**. A vitrine lê no manifesto se espera camadas.
- **Arte candidata** (`art-src/player/renders/v2_2_weaponless/stage/`): o
  staging de onde a publicada saiu; útil para conferir uma arte nova antes de
  publicar. É lida por uma fonte de assets separada.
- Arte antiga com o sabre desenhado (manifesto sem `weapon_in_body`): as
  camadas ficam desligadas para nunca duplicar a arma.
- A vitrine é uma **bancada**: ela coloca itens na bolsa. As rotas reais
  (`world_*_route`) não fazem isso.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `examples/equipment_visual_showcase.rs` | Bancada com 25 capturas e verificações |
| `tests/held_visual_tests.rs` | Testes automáticos das camadas |
| `target/equipment_visual_showcase/candidate/` | Capturas geradas |

## 4. Passo a passo

1. `cargo test -j 2 --test held_visual_tests`.
2. `cargo run -j 2 --example equipment_visual_showcase -- --published`
   (arte publicada, capturas em `target/equipment_visual_showcase/published/`).
3. `cargo run -j 2 --example equipment_visual_showcase` (staging candidato).
4. Abra as capturas e compare com a seção 5.

## 5. Exemplo completo

O que cada captura deve mostrar:

| Captura | Esperado |
|---|---|
| `s01_unarmed_idle` | Mão vazia, nenhuma arma |
| `s02`/`s03` | Espada; no golpe, a lâmina acompanha o braço |
| `s04` | Adaga curta |
| `s05` | Machado com cabeça na ponta |
| `s06b`/`s06` | Lança de pé em repouso; para a frente no golpe |
| `s07b`/`s07` | Montante no ombro em repouso; no golpe, acompanha |
| `s08` | Arma de duas mãos recusa escudo (nenhum escudo aparece) |
| `s09`–`s11` | Espada, adaga e machado com escudo no braço |
| `s12` | Lança recusada enquanto há escudo |
| `s13`/`s14` | Capa atrás do corpo; armadura com tom |
| `s15`–`s19` | Corrida, pulo, queda, guarda, esquiva |
| `s20`/`s21` | Virado para a esquerda (espelhado) |
| `s22`/`s23` | F5 salva, troca, F9 volta a aparência salva |
| `s24` | Tudo desequipado: nada na mão |
| `s25` | Fila dos NPCs (só apresentação) |

Uma cor de arma usada nas capturas (Duskwater Saber):

```ron
// exemplo-verificado: Visual
Trail(color: (0.55, 0.85, 1.0))
```

## 6. Como testar?

A vitrine termina com `[equipment_visual_showcase] RESULT: PASS` e sai com
código diferente de 0 se algo falhar. Os testes: `cargo test -j 2`.

## 7. Como editar depois?

Para testar um item novo, acrescente o `id` em `Give(&[...])` e um
`Equip(...)` com `Shot("nome")` no roteiro de `script()`.

## 8. Problemas comuns

| Mensagem | Causa | Solução |
|---|---|---|
| `no staged candidate` | Staging não gerado | Tutorial 25, passos 1–2 |
| `weapon layer visible false, expected true` | Âncoras ausentes ou inválidas | Regere a tabela (tutorial 25) |
| `holds ..., expected ...` | Equipar foi recusado (requisito, duas mãos) | Veja a notificação na captura |

## 9. Como desfazer com segurança?

As capturas e saves da vitrine ficam em `target/equipment_visual_showcase/`
(pasta de build). Nada em `assets/` ou `saves/` é escrito.
