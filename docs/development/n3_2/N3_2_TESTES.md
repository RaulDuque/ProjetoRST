# N3.2 — Testes

`tests/n3_2_art_direction_tests.rs` confere o manifesto de um quadro, a altura 100, a arma e a capa ausentes, e que a largura das três poses varia mais de 100 px.

Os testes da N3 e da N3.1 não foram editados.

Nesta sessão, em sequência, com no máximo 2 jobs e cerca de 9 GiB livres:

- `cargo fmt` e `cargo fmt --check`: saída 0.
- `cargo clippy --all-targets --jobs 2 -- -D warnings`: saída 0.
- `cargo test --jobs 2`: saída 0. Os dois testes novos passaram. Os da N3 e da N3.1 continuaram passando.
- `cargo check --jobs 2`: saída 0.
- `cargo run --example n3_visual_proof`: saída 0, na RTX 4050. `bevy_concept_b.png` mostra o quadro reduzido, com o texto de que a arma e a capa não são camadas.
