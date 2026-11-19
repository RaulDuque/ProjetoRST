# PROMPT 06 — INTEGRAR ARMADURA APROVADA AO BEVY, SEM CORPO NU

**PRÉ-REQUISITO ABSOLUTO:** aprovação visual explícita do proprietário no Prompt 05. Leia `00_LEIA_PRIMEIRO.md` e as evidências aprovadas.

**Modelo visual obrigatório:** jogo usa sempre o protagonista N3.5/N3.6 já vestido como base. O personagem 3D não entra no jogo, não aparece e não substitui o sprite. Blender é apenas ferramenta offline. A armadura aprovada chega como camadas PNG 2D independentes.

1. Auditar `equipment_visual.rs`, `composition.rs`, `composite.rs`, RON, `VisualPlayback` e gerador N41-B. Confirmar caminhos reais; propor **mudanças mínimas** antes de editar.
2. Registrar o equipamento nos contratos existentes, com imagens por pose, pivô, sincronismo e ordem de camada. Reutilizar `VisualPlayback`, sem relógio separado e sem nova engine.
3. Usar **OVERLAY** para a couraça, com oclusão/HIDE seletivo somente nas áreas cobertas. Permitir geometria fora da máscara da camisa. Braço, manga, mãos e cabelo na frente conforme o quadro. Corrigir a separação lógica idle manga/tronco de forma derivada/não destrutiva e reutilizar as máscaras limpas da Etapa 02B, caso imprescindível.
4. Em **modo sem armadura**, reconstrução deve ser idêntica ao PNG aprovado texel a texel. Em **modo com armadura**, não renderizar nudez, corpo alternativo ou roupa visível indevidamente debaixo da placa. Os pixels da camisa que ficam à mostra fora da área da armadura devem permanecer.
5. Manter configurável por dados: nenhuma nova modificação Rust/Python por armadura de mesmo contrato. Preservar sem alterações gameplay, save v3, física, combate, câmera, hitboxes, estados e arte aprovada. Não expandir slots não testados.
6. Rodar verificações relevantes (`cargo fmt --check`, `cargo clippy --all-targets`, `cargo test` e testes pixel-perfect conforme aplicável). Relatar limites do ambiente sem inventar sucesso.
7. Mostrar capturas reais do Bevy das quatro poses, comparadas às prévias offline aprovadas, e teste do equipamento ligado/desligado.

**Entregas:** código/ativos mínimos dentro da área autorizada, testes, imagens do jogo, lista de arquivos modificados e riscos. **Sem commit, push, merge/rebase, instalações ou alteração de `Md-usario/` sem pedido expresso. Parar após evidências.**
