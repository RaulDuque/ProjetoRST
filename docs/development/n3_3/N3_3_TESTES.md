# N3.3 — Testes

`tests/n3_3_native_pixel_tests.rs` tem dois testes. Os dois passaram: o manifesto é um quadro em pé, a arma está fora do corpo, não existe clipe de ataque, o soquete espelha o sinal de x, e a altura opaca está entre 96 e 112.

Na mesma execução, `cargo fmt --check`, `cargo clippy --all-targets --jobs 2 -- -D warnings`, `cargo test --jobs 2` e `cargo check --jobs 2` terminaram sem erro. O exemplo `n3_visual_proof` gravou `bevy_n33.png` e saiu com código 0. Os testes da N3 (4), da N3.1 e da N3.2 (2) continuaram verdes. As capturas antigas não mudaram no git.

Os testes antigos não foram editados. Passar nos testes não aprova o desenho.
