# N4-1-A — Relatório da revisão arquitetural

A máquina de sprites agora veste o **personagem vestido aprovado** com equipamentos definidos em RON, sem corpo nu. Família e item são dados, validados antes de desenhar. OVERLAY, HIDE (por camada e clipe) e REPLACE funcionam. Dois peitorais da mesma família (A e B) foram criados e o segundo não exigiu código, corpo ou máscara novos. No Bevy, o resultado é idêntico pixel a pixel ao que os dados descrevem, no idle e nos três quadros C1. **A qualidade da pintura dos peitorais é de prova, não de arte final** (Gate D com o proprietário). C2, N4-2 e N5 não foram iniciadas.

## 1. O que foi reaproveitado

`CharacterComposition`, `Outfit`, `resolve`, `CompositeCharacter`, `sync_composite_layers`, `VisualPlayback`, a partição vestida das camadas (idle e C1), os polígonos do tronco das poses C1 e o exemplo do Bevy com a checagem pixel a pixel.

## 2. O que foi implementado

HIDE por camada e clipe, REPLACE, folha espelhada e ordem estável (`composition.rs`); família, item e `equip` (`equipment_visual.rs`, novo, puro); `resolve_facing` no compositor; o slot `armor_torso`; a ferramenta de autoria `build_equipment.py`; o modo `--equipment` do exemplo; 15 testes. `N41A_ARQUITETURA.md`.

## 3. Formato

RON para os manifestos (padrão do projeto, tipado, sem dependência nova); JSON para receitas e relatórios de ferramentas; PNG para a arte. SVG e Base64 avaliados e não usados (anti-aliasing em pixel art; sem caso de transporte). `N41A_FORMATOS.md`.

## 4. Como os equipamentos são definidos

Família (`families/<id>.ron`): slot, clipes, HIDE/REPLACE padrão, `covers`, máscaras, exceções de ordem e regra de clipe ausente. Item (`items/<id>.ron`): folhas por clipe, folhas espelhadas, variantes, sobrescritas, paleta e exceções de ordem. `N41A_CONTRATO_RON.md`.

## 5. Sobreposição, ocultação e substituição

OVERLAY = o item desenhado no slot da família, no canvas, pivô e quadro do corpo. HIDE = camadas da base fora enquanto o item está vestido, **só onde o item cobre** (verificador e teste). REPLACE = uma peça inteira da base fora. `N41A_EQUIPAMENTOS.md`.

## 6. Sincronização

O item vira uma peça comum: o mesmo `VisualPlayback` (quadro e espelho), o mesmo `sync_composite_layers`. Nenhum relógio por equipamento. A corrida aprovada continua com 6 quadros a 15 fps e 310 u/s (N3.8); a prova usa os 3 quadros C1, 112×128, pivô (56,118).

## 7. Como criar um equipamento novo

Receita JSON (ou arte à mão sobre as máscaras), manifesto RON, `build_equipment.py`, teste e olhada no exemplo (tecla E). Nenhum código. `N41A_EQUIPAMENTOS.md` §"Como criar".

## 8. Trabalho artístico

A: família (máscaras geradas da arte vestida) + receita. B: **2 arquivos à mão** (receita e RON), **0 quadros desenhados à mão**, 8 regras reutilizadas e 1 sobrescrita, 0 código Rust, 0 corpo-base, 0 máscara nova. A pintura é procedural; a arte final pede um artista sobre as máscaras. `N41A_REUTILIZACAO.md`.

## 9. O segundo reutilizou o primeiro?

Reutilizou a **família** que A estabeleceu: silhueta por quadro, decote, sombreamento, ordem e regras. B é um recorte mais curto da mesma geometria (teste: B ⊂ A), com outra paleta, aresta central e friso. A arte de A em si não é copiada.

## 10. Limitações

Formas quadradas e uma linha de contorno no ombro; sem HIDE no idle (a camisa do idle é uma camada só); máscaras só offline; polígonos do tronco marcados à mão; novos padrões de pintura são código de autoria. `N41A_LIMITACOES.md`.

## 11. Testes

fmt, check e clippy `-D warnings` limpos; `cargo test -j 2`: **55 suítes, 675 passaram, 0 falharam**; 15 testes novos. `N41A_TESTES.md`.

## 12. Evidências

`evidence/n41a/`: folhas de sobreposição (idle e corrida), verificações de "e se ocultasse", spritesheets compostas, GIF da corrida C1 com os itens, 16 capturas do Bevy e `coverage.json` (cobertura + checagem pixel a pixel, 14/14 com 0 diferença). `N41A_COMPARACOES.md`.

## 13. Commits e SHA

Push em `origin/art/v1.2-protagonist` (`bf12119..55a884c`), sem force, merge ou rebase:

- `d3aef79` feat(player): add data-driven visual equipment to the sprite composition
- `acf3f29` feat(visual): add the torso_plate family and equipment items a and b
- `3dae0e6` test(player): validate data-driven visual equipment
- `22254eb` feat(visual): add the equipment mode to the sprite machine proof
- `8de1a8b` docs(architecture): document data-driven visual equipment
- `1e85e93` docs(claude): list the n4-1-a equipment tests
- `55a884c595d2d384d472ee25c0d7dcf88b888138` docs(n4): record the n4-1-a equipment review and evidence (SHA remoto confirmado igual ao local)

O commit que grava esta seção é o registro da publicação; o hash remoto final é o HEAD depois do push dele. `Md-usario/` não foi alterado nem commitado.

## 14. Decisões pendentes

1. Avaliação artística dos peitorais (`n41a_run_c1_overlay.png`, `n41a_run_c1_items.gif`, modo interativo).
2. Arte final: refinar o pintor procedural ou entregar as máscaras a um artista.
3. Próxima família para provar REPLACE e HIDE de região com dados reais (botas ou capacete).
4. Autorização da C2 (agora sem exigir anatomia: partição dos 6 quadros + polígonos do tronco).

## Gates

### Gate A: dados
- [x] Item definido em dados
- [x] Validação implementada
- [x] Sem código Rust específico por item

### Gate B: composição
- [x] Sobreposição funcional
- [x] Ocultação funcional
- [x] Substituição funcional (testada com dados de teste; não necessária nos peitorais)
- [x] Compatibilidade com o personagem original (sem item = aprovado, texel a texel)

### Gate C: reutilização
- [x] Dois equipamentos distintos
- [x] Mesma família reutilizada
- [x] Sem reconstrução anatômica
- [x] Sem nova lógica Rust para o segundo item

### Gate D: arte
- [ ] Aparência coerente com N3.5/N3.6: **paleta, contorno e luz coerentes; formas e acabamento abaixo da arte aprovada**
- [x] Sem defeitos graves de sobreposição (sem camisa atravessando, sem buraco, sem item flutuando)
- [x] Evidências para avaliação do proprietário

### Gate E: segurança
- [x] V2.2 preservado
- [x] N3.8 preservada
- [x] N4 preservada (idle N4-A igual; só um slot vazio novo)
- [x] N4-1-A C1 preservada (camadas C1 sem mudança)
- [x] Save v3 preservado
- [x] Física e combate preservados
- [x] Testes executados
