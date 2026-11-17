# Relatório final D11 — Integração experimental Bevy 0.20.1

**Sem commit. Sem push. Visual padrão não substituído.**

## A. Estado inicial do Git

- Branch: `art/v1.2-protagonist` (tracking `origin/art/v1.2-protagonist`)
- HEAD de partida da sessão D11: `1115f0d` — `docs(art): add D8 modular 2D atlas prototypes`
- Já existiam untracked D9/D10 labs e briefs (não tocados neste relatório além do lab D11)

## B. Auditoria D10

Ver `AUDITORIA_D10.md`. Antifake capa/luvas residual 0; after-image A/B/C no lab; plano Bevy presente. Lacuna: frames clássicos ainda usam D2 como guia de pose.

## C. Auditoria do renderer

Ver `AUDITORIA_RENDERER.md`. Default continua `PlayerVisualPlugin` + `player.ron`.

## D. Arquitetura escolhida

**Estratégia C (híbrida)** — bake D10 dos frames + runtime de troca/clip/facing/after-image no exemplo. Detalhe: `DECISAO_ARQUITETURA.md`.

## E. Arquivos de código alterados / adicionados

| Arquivo | Papel |
|---|---|
| `src/player/visual/modular_lab.rs` | Regras puras + testes |
| `src/player/visual/mod.rs` | `pub mod modular_lab` |
| `examples/modular_visual_lab.rs` | App Bevy isolado |

## F. Assets experimentais

`assets/experimental/d11/` — frames idle/run classic+heavy, poses, `contract.json`, cópia do antifake D10. Publicador: `scripts/publish_d11_assets.py`.

## G. Contrato de atlas

128×160, pivot (64,156), world height 52, nearest por imagem. `CONTRATO_ATLAS.md` + `contract.json`.

## H–K. Clássico / pesado / troca / idle+run

Exemplo carrega ambos; script troca classic→heavy **sem zerar** `elapsed`; idle 6 @10fps e run 8 @12fps; `flip_x` para facing.

## L. After-image B

Trail vinho (`AFTERIMAGE_TINT`), spawn por intervalo, fade linear — regras em `modular_lab` + fantasmas no exemplo.

## M. Regressão

- `cargo fmt --check` OK  
- `cargo clippy --all-targets -- -D warnings` OK  
- `cargo check` OK  
- `cargo test --lib modular_lab` 5/5 OK  
- Manifesto publicado e plugin default intocados  

## N. Testes visuais

`evidencias_bevy/01`…`06` gerados por Bevy (idle classic, run+trail, heavy mantendo clock, facing left, idle heavy, classic right).

## O. Performance

Lab mínimo (sem mundo/combate). Sem medição de FPS do jogo completo — fora do escopo do exemplo isolado.

## P. Limitações

1. Não é visual final aprovado; animação clássica ainda herda guia D2.  
2. Só idle/run; sem combate/magia/hurt.  
3. After-image ainda experimental (não no VFX de produção).  
4. Exemplo separado — ainda não há feature flag no app principal (proposital, menos invasivo).

## Q. Prioridade de revisão

1. `evidencias_bevy/*.png`  
2. `examples/modular_visual_lab.rs`  
3. `src/player/visual/modular_lab.rs`  
4. `assets/experimental/d11/contract.json`  
5. `DECISAO_ARQUITETURA.md`

## R. Estado final do Git

Working tree suja com D9–D11 untracked + `modular_lab` / exemplo / `assets/experimental/`. **Nenhum commit feito.** HEAD permanece `1115f0d` até o usuário pedir commit.

## S. Recomendação para D12

1. Aprovação visual do proprietário nas evidências Bevy.  
2. Feature flag opcional no jogo (`modular_visual_experimental`) se quiser playtest in-world — ainda desligada por default.  
3. Bake modular puro das poses de animação (sair do guia D2).  
4. Estender clips (attack/guard/hurt) só após A/B acima.  
5. Portar after-image B para `player::visual::vfx` atrás da mesma flag.
