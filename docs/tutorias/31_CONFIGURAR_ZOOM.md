# 31 — Configurar o zoom

> Situação: **funciona** (V2.3). **Pendente:** zoom diferente por sala.

## 1. O que vamos fazer?

Mudar o quanto do mundo aparece na tela (o zoom), sem mudar física, salas ou HUD.

## 2. O que você precisa saber antes?

- O zoom é `view_height` em `assets/data/camera.ron`: a **altura do mundo
  visível**, em unidades. A largura segue a proporção da janela.
- `720` = o jogo até a V2.2 (sala de 1 célula inteira na tela).
- `405` = padrão da V2.3 (×1,78). Escolhido comparando 720, 540, 480, 405 e
  360 em três salas: em 405 o personagem fica legível e a sala vertical ainda
  mostra 3–4 plataformas à frente; em 360 o HUD já cobre plataformas.
- Aceito: de `240` a `720`.
- Janela maior ou menor: a altura do mundo visível continua a mesma; o HUD
  mantém o tamanho em pixels.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/data/camera.ron` | Campo `view_height` |
| `examples/world_visual_showcase.rs` | A varredura de zoom (`zoom720` … `zoom360`) |

## 4. Passo a passo

1. Abra `assets/data/camera.ron`.
2. Mude `view_height`.
3. Rode a vitrine e compare `after_*` com `zoom*_*`.
4. Teste salas horizontais **e** verticais antes de decidir.

## 5. Exemplo completo

Um zoom mais aberto (mais mundo, personagem menor):

```ron
// exemplo-verificado: Camera
(view_height: 480.0)
```

## 6. Como testar?

1. `cargo test -j 2 --test world_scenery_tests` (o HUD não muda com o zoom;
   os valores enviados são os padrões documentados).
2. `cargo test -j 2 --test world_camera_tests` (a vista nunca sai da sala).
3. `cargo run -j 2 --example world_visual_showcase`.

## 7. Como editar depois?

Só o número `view_height`. Se mudar o padrão de propósito, atualize também
`CameraSettings::default()` em `src/world/camera_rules.rs` (o teste
`shipped_camera_settings_are_the_documented_defaults` confere os dois).

## 8. Problemas comuns

| Sintoma | Causa | Solução |
|---|---|---|
| `camera setting view_height out of range` | Menor que 240 ou maior que 720 | Use um valor no intervalo |
| O teste `shipped_camera_settings...` falha | O arquivo não bate com o padrão do código | Ajuste os dois juntos |
| HUD cobre plataformas no alto | Zoom fechado demais | Aumente `view_height` |

## 9. Como desfazer com segurança?

Volte `view_height: 405.0`.
