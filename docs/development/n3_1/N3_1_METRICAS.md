# N3.1 — Métricas

Tempo de máquina e tempo humano ficam separados. Não houve artista. `human_artist_hours` em `art-src/n3-proof/n3_1/metrics.json` é nulo.

- Conversão do ataque (BOX, 16 cores, contorno, 2×): 0,66 s. É o `machine_seconds` do JSON.
- Renders de `jump_rise` e `fall`: a invocação do Blender que os gravou e em seguida falhou na primeira tentativa do walk levou 73 s de relógio, com duas leituras do blend.
- O ataque (7 quadros de 1024×768) foi uma invocação anterior. Cada quadro de combate, depois de o blend já estar aberto, fica na casa dos 8 s neste hardware.
- Walk de 16 quadros: 139 s de relógio na segunda invocação, que terminou com o blend sem gravação. A primeira tentativa falhou em menos de um segundo por um `import` e não renderizou nada.

Tamanho: sete masters de ataque, cerca de 250 KB cada. O atlas automático é uma faixa de 1792×192.

Trocar a espada ou uma segunda armadura neste render não foi medido como atualização de produção: a espada está fora da malha e a segunda armadura não existe. Medir um recolor como se fosse uma armadura nova falsificaria o custo.

Na janela 1280×720 a caixa do corpo, sem a lâmina e sem o texto, mede 223 px na vista 360 e 99 px na vista 405.

`cargo fmt --check`, `cargo check --jobs 2`, `cargo clippy --all-targets --jobs 2 -- -D warnings` e `cargo test --jobs 2` terminaram com saída 0. O exemplo abriu na RTX 4050 e gravou as capturas. Detalhe em `N3_1_TESTES.md`. Memória disponível antes do Cargo: cerca de 10 GiB.
