# Arquitetura modular D9

```
loadout dict → find_piece(id) → DRAW_ORDER → pose
```

- Compose **nunca** carrega `full_classic_ref`.
- Erros descritivos (`ComposeError`) se peça inexistente.
- Oclusão: elmo oculta cabelo/rosto; armadura substitui casaco.
- Animação clássica: frames D2 para fidelidade de movimento.
- Animação pesada: peças seguem sockets (translação + leve rotação de arma).
