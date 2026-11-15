# 13 — Criar barreiras espectrais (véus)

> Situação: **funciona**.

## 1. O que vamos fazer?

Pôr um **véu espectral** numa porta: humanos e inimigos batem nele como numa
parede; só a Forma Espectral (tecla T, depois da Relíquia do Véu) atravessa.

## 2. O que você precisa saber antes?

- O véu fica na lista `veils` da sala e **preenche a abertura de uma porta
  da própria sala** (`door`).
- `kind: Spectral` (o único tipo). A passagem é definida em
  `assets/world/world.ron` (`passages: { Spectral: Ability(VeilStep) }`).
- `depth`: em porta lateral, quanto o véu entra na sala (de 0 a **120**). Em
  porta no chão, deve ser **0**. Nunca em porta no teto.
- Atravessar gasta MP (a forma consome 8 MP ao ativar + 4 MP/s). O jogo
  recusa véus que o personagem inicial não consiga atravessar.
- Um véu de cada lado é preciso se a outra sala também tiver véu na porta.

## 3. Quais arquivos vamos utilizar?

| Arquivo | Para quê |
|---|---|
| `assets/world/regions/veiled_cloister.ron` | Exemplo real (dois véus laterais) |
| `assets/world/regions/vigil_gates.ron` | Exemplo real (véu no chão da galeria) |

## 4. Passo a passo

1. Escolha uma porta lateral ou de chão da sala.
2. Acrescente em `veils: [ ... ]` o véu com `door` = id da porta.
3. Teste com a Forma Espectral.

## 5. Exemplo completo

Véu real do Poço Velado (porta `to_shrine`, lateral):

```ron
// exemplo-verificado: Veu
(id: "vc_well_west_veil", door: "to_shrine", kind: Spectral, depth: 80.0)
```

Véu real no chão da galeria (profundidade 0):

```ron
// exemplo-verificado: Veu
(id: "vg_gallery_floor_veil", door: "to_veiled_well", kind: Spectral)
```

## 6. Como testar?

1. `cargo run -j 2 --example world_report`: a seção `Veils` mostra o custo
   (`crossing ... MP`).
2. `cargo run -j 2`: humano, você para no véu; pegue a relíquia, T, atravesse.
3. Rotas: `cargo test -j 2 --test world_spectral_route_tests`.

## 7. Como editar depois?

Mude `depth` (0..120). Para remover, apague o véu.

## 8. Problemas comuns

| Mensagem (`BadVeil`) | Causa | Solução |
|---|---|---|
| `UnknownDoor` | `door` não é porta desta sala | Corrija o id |
| `CeilingDoor` | Véu em porta do teto | Não use |
| `Depth` | `depth` fora de 0..120, ou ≠ 0 no chão | Ajuste |
| `SameDoor` | Dois véus na mesma porta | Só um |
| `Impassable` | Passagem não definida em `world.ron` | Não mude `passages` |
| `veil ... costs ... MP to cross; the starting character has ...` | Véu fundo demais para o MP inicial | Diminua `depth` |

## 9. Como desfazer com segurança?

Apague o bloco do véu.
