# N3.5 — Processo artístico e autoria

## O que foi feito

1. **Diagnóstico.** A `reducao` que o proprietário escolheu é a redução automática da N3.4, com a mesma resolução (106 px) do sprite rejeitado. A diferença era de técnica: o redesenho à mão em grade de texto da N3.4 trocou o modelado da pintura por massas chapadas.
2. **Redução melhorada** (`art-src/n3-5-fidelity/scripts/reduce_fidelity.py`). É o método da `reducao` com uma única mudança: a máscara é erodida em 1 px da ficha antes da redução, o que elimina o halo claro. O mesmo método gerou 106, 132 e 94 px.
3. **Teste de paleta.** Paleta da N3.4 contra mediancut de 32 cores e contra mediancut de 24 cores com as rampas de pele e cabelo. A da N3.4 deu o rosto mais legível e é a da `reducao` (`palette_test.png`).
4. **Correção mínima à mão.** São 10 pixels, todos listados com origem e destino em `generated/metrics.json`: 7 removidos (4 isolados no peito, testa, costas e ponta da bota, mais 3 de halo na testa) e 3 recoloridos (nariz e sombra sob o nariz).
5. **Prova no Bevy**, com câmeras 405 e 360 e comparação pixel a pixel.

## Autoria

- Pixels: redução automática da ilustração do proprietário, mais 10 edições do agente (Claude) na grade de texto.
- Nenhuma pintura em programa gráfico: o agente não opera interfaces gráficas.
- Nenhuma IA de imagem.
- Nenhuma hora de artista humano.

## Por que não redesenhar mais

O briefing aponta a `reducao` como alvo de acabamento. Cada pixel redesenhado à mão afasta o sprite da pintura, e foi isso que reprovou a N3.4. O teste `tests/n3_5_fidelity_tests.rs` limita as edições a 16 pixels, para que o candidato continue sendo a redução.

## Fonte editável

`source/candidate_106.txt`: 80×128, um caractere por pixel, chaves da paleta da N3.4 (`art-src/n3-4-original-protagonist/scripts/n34_palette.py`). `source/reduction_106.txt` é a redução sem edições, para comparar. Para editar:

```bash
python3 art-src/n3-5-fidelity/scripts/reduce_fidelity.py   # regrava só as reduções
python3 art-src/n3-5-fidelity/scripts/build_n3_5.py        # PNG, manifesto, métricas e evidências
```

`reduce_fidelity.py` não toca em `candidate_106.txt`. Se a arte for repintada num programa gráfico, o PNG passa a ser a fonte, e este documento precisa ser atualizado.
