# N3.1 — Testes

Os testes da N3 continuam em `tests/n3_visual_proof_tests.rs`. Os da N3.1 estão em `tests/n3_1_visual_proof_tests.rs`:

- o `attack.ron` passa no leitor que o jogo já usa;
- sete quadros, pivô (80, 190) dentro da célula 256×192, arma fora do corpo;
- o atlas automático mede 1792×192;
- os sete PNG brutos medem 1024×768;
- o soquete da mão muda entre o quadro 0 e o quadro 3;
- o espelhamento nega x e o ângulo.

Eles não julgam se o desenho é bom.

Execução desta sessão, em sequência, com no máximo 2 jobs:

- `cargo fmt` e `cargo fmt --check`: saída 0.
- `cargo check --jobs 2`: saída 0.
- `cargo clippy --all-targets --jobs 2 -- -D warnings`: saída 0.
- `cargo test --jobs 2`: saída 0. Os dois testes novos passaram. Os quatro da N3 continuaram passando.
- `cargo run --example n3_visual_proof`: saída 0, na RTX 4050, Vulkan. As capturas estão em `docs/development/n3/evidence/bevy_*.png` e `docs/development/n3_1/evidence/bevy_attack_*.png`.

O quadro 0 do ataque automático mostra a espada geométrica da N3 saindo da mão medida. O quadro 3 abaixa essa espada com a mão. A capa da linha de base, no quadro 1 do pulo, fica atrás do tronco.
