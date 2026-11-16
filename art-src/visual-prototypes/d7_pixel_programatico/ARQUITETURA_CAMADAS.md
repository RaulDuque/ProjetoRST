# Arquitetura de camadas (D7)

Slots **funcionais** do jogo (8) não são alterados. Aqui: slots **visuais**.

## Ordem de desenho (fundo → frente)

1. `cape_back`
2. `body` (corpo-base / animação compartilhada)
3. `coat_or_armor` (casaco leve **ou** peitoral)
4. `helm_or_hat` / óculos
5. `sword`
6. `shield`

Manifesto gerado: `personagem/layers_manifest.json`.

Cada entrada registra: id, arquivo, size, bbox, pivô sola, paleta, visibilidade.

## Corpo-base

Marco B usa frames D2 (`idle` 6 / `run` 8) como animação do corpo. Equipamentos
D4 são compostos por cima (piloto estático por frame — sem deformação de capa
ainda).

## Oclusão

- Elmo fechado pode cobrir cabelo (helm sobre head).
- Casaco/capa ocultam parte das pernas.
- Escudo à frente pode tapar tronco inferior.
