# DARK CASTLE RPG — FASE 4
## Preservar o visual D2 e projetar equipamentos visuais modulares

O proprietário revisou a comparação `cmp1_identity_dark.png` e tomou uma decisão artística explícita:

**O estilo visual D2 é o escolhido. Os estilos geométricos D3-B1 e D3-B2 foram rejeitados como aparência final.**

Não simplifique o D2 para fazê-lo caber no pipeline D3. Preserve sua identidade visual, riqueza de detalhes, proporções, roupas, cabelo, armadura e capa.

O proprietário também definiu que os equipamentos devem modificar a aparência do protagonista. Precisamos de uma arquitetura visual capaz de trocar chapéu, elmo, óculos, máscara, camisa, peitoral, casaco, calças, botas, luvas, capa, arma principal, arma secundária e outros itens.

## Objetivo

Realizar uma auditoria técnica e produzir protótipos comparativos de um **personagem D2 visualmente modular**, sem alterar o jogo principal.

### 1. Auditoria

- Verifique Git, branch, HEAD e arquivos pessoais.
- Leia os contratos de animação e equipamentos atuais.
- Identifique quais slots já existem e quais seriam novos.
- Inspecione os sockets existentes e suas limitações.
- Identifique as animações que exigiriam adaptações.
- Avalie impactos futuros no inventário, equipamento e Save V3, mas não os implemente.
- Preserve D1, D2, D3 e todos os arquivos publicados.

### 2. Estilo obrigatório

Utilize como referência principal o protagonista D2 e as imagens de equipamentos aprovadas.

A meta é preservar a aparência detalhada do conceito, e não reproduzir o resultado geométrico D3.

### 3. Arquitetura modular

Projete um sistema visual com camadas para:

- Corpo-base.
- Cabelo.
- Cabeça e rosto.
- Peitoral e casaco.
- Braços e luvas.
- Calças.
- Botas.
- Capa.
- Arma principal.
- Arma secundária ou escudo.
- Efeitos de equipamentos.

Documente ordem de desenho, pivôs, sockets, oclusão e animação.

Diferencie claramente slots visuais de slots funcionais. Não introduza slots de gameplay sem autorização.

### 4. Comparar pipelines

Faça estudos técnicos de:

A. Blender com peças intercambiáveis e exportação para sprites 2D.

B. Sprite 2D detalhado com overlays de equipamentos por quadro.

C. Pipeline híbrido, com Blender para movimento e pixel art para acabamento.

Compare fidelidade ao D2, consistência, facilidade de animação, custo de novas peças e compatibilidade com Bevy.

Não declare um método vencedor sem demonstração.

### 5. Protótipo obrigatório

Crie pelo menos uma pose idle lateral baseada no D2, com:

- Corpo-base sem equipamento.
- Roupa leve.
- Armadura pesada.
- Chapéu ou elmo.
- Óculos ou máscara.
- Capa removível.
- Espada.
- Escudo.

Demonstre combinações independentes dessas peças sobre o mesmo corpo.

O personagem deve continuar reconhecível como D2 em todas as combinações.

Faça também um teste curto de movimento com pelo menos duas combinações de equipamento, para revelar problemas de alinhamento e sobreposição.

Se não conseguir produzir um resultado fiel, documente o bloqueio e não apresente uma versão simplificada como equivalente.

### 6. Comparação no jogo

Monte pranchas comparando:

- D2 original.
- Protótipo modular sem equipamento.
- Protótipo com roupa leve.
- Protótipo com armadura pesada.
- Protótipo com capa e espada.
- Protótipo com escudo.

Utilize a escala real da câmera V2.3 e uma captura do cenário aprovado.

### 7. Entrega

Crie uma pasta independente:

`art-src/visual-prototypes/d4_modular_d2/`

Inclua:

- `README.md`
- `AUDITORIA_SLOTS.md`
- `ARQUITETURA_VISUAL.md`
- `COMPARACAO_PIPELINES.md`
- `LIMITACOES.md`
- `DECISAO.md`
- Referências e protótipos.
- Camadas exportadas individualmente.
- Pranchas de comparação.
- GIFs curtos, se tecnicamente viáveis.
- Scripts e testes auxiliares.

### 8. Restrições

Não altere `src/`, `assets/`, `saves/`, `Md-usario/` ou os protótipos anteriores.

Não altere Save V3, combate, magias, física, câmera ou dados RON.

Não faça commit nem push.

Não publique sprites no jogo.

Não utilize assets protegidos de Castlevania ou de outros jogos.

### 9. Critérios de aprovação

A fase só será considerada tecnicamente promissora se:

1. O visual preservar a identidade detalhada do D2.
2. As peças puderem ser combinadas independentemente.
3. Os equipamentos acompanharem o personagem em movimento.
4. A silhueta continuar legível na escala real do jogo.
5. O pipeline demonstrar possibilidade de produzir novas armas e armaduras sem redesenhar todo o protagonista.
6. As limitações forem documentadas com clareza.

Entregue relatório final com arquivos gerados, resultados, problemas, comparação de pipelines, recomendação e estado do Git.

**Não avance para a produção de todas as animações. Aguarde a avaliação do proprietário sobre o protótipo modular.**