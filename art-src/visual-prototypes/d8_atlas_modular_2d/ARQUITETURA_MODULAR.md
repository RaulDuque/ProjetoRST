# Arquitetura modular D8

```
D2 frames (corpo-base animado)
  + atlas cuts (prova de separação)
  + equipamentos D4 (elmo, armadura, escudo, …)
  + sockets por quadro (deslocamento de peças)
  → loadouts / idle / run
```

- Slots **visuais** ≠ 8 slots funcionais de inventário.
- Clássico: prioriza `full_classic_ref` para não perder D7-B.
- Pesado: `body` D2 animado + armor/helm/shield com delta de socket.
- Capa: método A (quadros D2) escolhido; B/C documentados em `capa/`.
