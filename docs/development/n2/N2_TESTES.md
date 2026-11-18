# N2 — Testes

`cargo test` na árvore final (antes do exemplo gráfico, que não entra nos testes) terminou com código 0.

## Unitários novos

Em `src/input/`:

- dois controles no mesmo quadro são um pressionar
- o segundo controle, com o primeiro já segurado, não é outro pressionar
- soltar um dos dois não solta a ação
- soltar e apertar de novo antes da leitura é um pressionar novo
- o preset híbrido não usa W como "cima" e não usa D como interagir
- o clássico mantém Z, X, D, A e S
- os dois presets passam na validação
- arquivo ausente não é criado
- ida e volta em RON conserva o híbrido
- arquivo corrompido é recusado e não é regravado

## Integração (`tests/controls_tests.rs`, 12 testes)

- A/D andam; D não interage; E interage
- W pula e não grava `Motion::Up`; a seta para cima grava
- S agacha e grava `Motion::Down`
- Espaço com W já segurado não é outro pulo
- mouse e X no mesmo quadro são um ataque
- Z e a seta direita continuam no clássico
- a tela de controles para o movimento
- conflito de D com andar é recusado; Enter pode ser pulo e confirmar
- limpar o último controle de uma ação essencial é recusado
- trocar o perfil não altera um arquivo de save
- o save continua na versão 3
- um app novo com arquivo inválido sobe no híbrido e não regrava o arquivo

As rotas antigas (cidade, equipamento, espectral, galerias, mapa) continuam apertando as teclas clássicas porque o harness instala esse preset. Elas passaram na suíte.

O parser não foi editado. O teste de buffer mostra que a seta para cima e o S chegam como `Motion`, e o W não.
