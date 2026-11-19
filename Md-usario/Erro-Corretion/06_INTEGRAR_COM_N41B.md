# PROMPT 06 — INTEGRAR PIPELINE APROVADO AO RUST/BEVY

**SOMENTE EXECUTAR APÓS O PROPRIETÁRIO APROVAR EXPLICITAMENTE O GATE VISUAL DO PROMPT 05.** Leia `00_LEIA_PRIMEIRO.md` e o resultado aprovado.

**Objetivo:** ligar a armadura aprovada ao sistema existente N41-B sem jogar fora trabalho válido e sem trazer Blender para o runtime. O jogo permanece Rust + Bevy.

1. Leia a arquitetura real `equipment_visual.rs`, `composition.rs`, `composite.rs`, `VisualPlayback`, manifestos RON e a fábrica N41-B; valide os caminhos reais. Antes de editar, proponha **o menor conjunto de alterações**.
2. Integrar o PNG/atlas da armadura pré-renderizada por pose e slot mantendo sincronização com `VisualPlayback`, espelhamento, pivôs, prioridade de camada e orientação.
3. Permitir silhueta externa à camisa e oclusão correta por pose. Se for indispensável separar manga/camisa no idle, propor mudança **não destrutiva** sem substituir/alterar a arte original aprovada; usar novas máscaras derivadas, com prova visual.
4. Usar RON/JSON apenas para montagem e referências de arquivos: **não obrigar novo código Python ou Rust para cada equipamento**. O código de pintura procedural anterior continua como legado/fallback, não fonte obrigatória do novo estilo.
5. Garantir modo sem equipamento idêntico texel a texel à arte aprovada; preservar gameplay, save v3, física, câmera, áudio, hitboxes e estados.
6. Executar testes existentes e checks relevantes (`cargo fmt --check`, `cargo clippy --all-targets`, `cargo test` e evidência pixel-perfect quando disponíveis). Se dependências de sistema impedirem algum teste, registrar honestamente.
7. Capturar evidência dentro do Bevy (idle e C1), lado a lado com prévia offline e N41-B antiga, e verificar consistência real.

**Critérios de aprovação:** sprite vestido corresponde à prévia aprovada; sem equipamento = original idêntico; comandos funcionam; testes passam; nenhuma regressão; não há segunda engine ou renderização 3D dentro do jogo.

**Proteções:** nenhum push, commit, merge/rebase, instalação ou modificação de `Md-usario/` e artes aprovadas sem pedido explícito. Não implementar inventário completo, multiplayer, capas, espadas ou novas dezenas de armaduras. Entregar relatório e **parar**.
