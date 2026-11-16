# Métodos de capa

| Método | Fidelidade | Textura | Silhueta | Custo | Reprodutível |
|--------|------------|---------|----------|-------|-------------|
| A quadros D2 | Alta | Preserva pixels | Boa | Alto (arte) | Sim |
| B malha shear | Média | Pode esticar | Risco | Baixo | Sim |
| C simulação | Baixa–média | Offset só | OK | Baixo | Sim |

**Escolha D8:** método **A** para o piloto de corrida (fidelidade D7-B), com **C** como reforço leve de offset se A falhar em um quadro.
