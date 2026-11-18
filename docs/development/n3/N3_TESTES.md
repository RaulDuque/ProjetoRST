# N3 — Testes

`cargo test --jobs 2` terminou com código 0. A suíte inteira passou, incluindo `tests/n3_visual_proof_tests.rs` (4 testes). Os testes antigos não foram editados.

Os quatro testes novos conferem:

- o manifesto abre, tem idle, walk, jump e attack, um só atlas, `weapon_in_body: false`, pivô dentro da célula e a duração de cada quadro;
- o PNG do corpo tem 576×512, que é 96×128 vezes o grid 6×4, e as duas armaduras têm o mesmo tamanho e bytes diferentes;
- os sockets abrem contra o manifesto, o golpe afasta a mão do preparo em mais de 5 unidades, o preparo desenha a arma atrás e o golpe na frente, e o espelho nega o x e o ângulo;
- o ouro da arma está nas armas e não no corpo, via Pillow;
- `white_mesh.glb` continua com 3.387.148 bytes.

O exemplo gráfico não entra no `cargo test`. Ele foi executado à parte, com `DISPLAY=:0` e a RTX 4050 em Vulkan. As capturas estão em `docs/development/n3/evidence/bevy_*.png`.

```text
cargo fmt --check
cargo check --examples --tests --jobs 2
cargo clippy --all-targets --jobs 2 -- -D warnings
cargo test --jobs 2
cargo run --example n3_visual_proof --jobs 2
```

`fmt`, `clippy -D warnings` e o exemplo foram repetidos depois do ajuste da última captura. O `cargo test` completo não foi repetido: os testes não mudaram depois da corrida que passou.
