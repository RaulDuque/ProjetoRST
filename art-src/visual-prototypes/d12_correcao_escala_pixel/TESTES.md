# Testes D12

```bash
python3 art-src/visual-prototypes/d10_finalizacao_modular/scripts/test_d10.py
python3 art-src/visual-prototypes/d12_correcao_escala_pixel/scripts/d12_pipeline.py
python3 art-src/visual-prototypes/d12_correcao_escala_pixel/scripts/test_d12.py
cargo test --lib modular_lab
cargo run --example modular_visual_lab
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo check
```

Cobertura: contrato, dimensões, evidências obrigatórias, 7 unitários (frame, anchor, sombra, after-image, world 64).
