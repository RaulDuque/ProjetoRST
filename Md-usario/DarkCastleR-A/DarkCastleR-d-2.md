# DARK CASTLE RPG — DIREÇÃO ARTÍSTICA / FASE 2
## Protagonista B detalhado com legibilidade da opção A

O proprietário analisou as comparações da Fase 1 e escolheu a opção B como direção visual principal.

Também gostou das poses de idle, corrida, pulo e queda da opção A.

**Decisão: produzir um protagonista híbrido, com identidade visual B e legibilidade de movimento A.**

Esta tarefa é um protótipo técnico. Não substitua o personagem publicado.

## 1. Referências obrigatórias

Examine diretamente:

- `art-src/visual-prototypes/d1_fase1/poses/A/idle_concept.jpg`
- `art-src/visual-prototypes/d1_fase1/poses/A/fall_concept.jpg`
- `art-src/visual-prototypes/d1_fase1/poses/A/jump_concept.jpg`
- `art-src/visual-prototypes/d1_fase1/poses/A/run_concept.jpg`
- `art-src/visual-prototypes/d1_fase1/poses/d1_B_poses_sheet.jpg`
- `art-src/visual-prototypes/d1_fase1/comparacoes/COMPARE_FINAL_d1.png`
- `art-src/visual-prototypes/d1_fase1/equipamentos/d1_equip_variants.jpg`
- `art-src/visual-prototypes/d1_fase1/espectral/d1_spectral_sequence.jpg`

Leia também os documentos técnicos da arte atual e da Fase 1.

Não presuma que imagens conceituais já são spritesheets utilizáveis.

## 2. Segurança

Confirme o Git antes de começar.

HEAD esperado: `1bf2ed2`.

Preserve os commits locais do B2.

Não altere `assets/`, `src/`, `saves/` ou `Md-usario/`.

Não sobrescreva os protótipos existentes.

Não execute commit ou push.

Crie os resultados exclusivamente em:

`art-src/visual-prototypes/d2_protagonista_B/`

## 3. Definir a identidade do personagem

Base visual: B detalhado.

Características:

- Protagonista gótico de fantasia sombria.
- Casaco longo e armadura.
- Capa vermelha escura.
- Cabelo escuro.
- Espada equipada.
- Silhueta legível.
- Detalhes reconhecíveis na escala real.
- Proporções constantes em todas as poses.

Use a opção A como referência para movimentos e leitura da silhueta.

Não transforme cada animação em um personagem visualmente diferente.

## 4. Criar uma folha de referência consistente

Antes de animar, produza uma referência-mestre com:

- Vista lateral.
- Vista frontal.
- Vista traseira.
- Paleta de cores.
- Proporções.
- Posição das mãos.
- Pontos de ancoragem.
- Espada.
- Capa.
- Armadura.

A vista lateral é prioritária, pois o jogo é 2D.

Se não conseguir produzir vistas coerentes, identifique a limitação antes de prosseguir.

## 5. Três animações piloto

Produza:

### Idle

Respiração discreta, movimento suave da capa e postura consistente.

### Corrida

Ciclo completo de movimento, contato correto dos pés com o chão e deslocamento convincente da capa.

### Ataque básico

Antecipação, janela ativa e recuperação visualmente identificáveis.

A animação deve ser compatível com a linha do tempo de combate já implementada na V2.4.

Não altere os tempos atuais do ataque para acomodar a arte.

## 6. Qualidade técnica

Não considere uma sequência de imagens independentes uma animação pronta.

Verifique:

- Consistência da identidade do personagem.
- Tamanho e proporções.
- Pivô dos pés.
- Alinhamento da espada.
- Transparência verdadeira.
- Ausência de fundo rosa nas sprites finais.
- Ausência de borrões de interpolação.
- Continuidade entre quadros.
- Compatibilidade com nearest-neighbor.
- Legibilidade na escala real.
- Compatibilidade com a hurtbox existente.

Não declare spritesheets prontas se os quadros não forem coerentes.

## 7. Equipamentos

Use `d1_equip_variants.jpg` como direção aprovada.

Prepare estudos de como o protagonista B poderá usar:

- Armadura leve.
- Armadura pesada.
- Escudo.
- Capa alternativa.

Não produza todas as animações de cada equipamento agora.

Priorize um sistema reutilizável de camadas e sockets.

## 8. Transformação espectral

A proposta `d1_spectral_sequence.jpg` está aprovada como referência visual.

Prepare um estudo de como a identidade do protagonista B pode se transformar em uma silhueta espectral distinta.

Não implemente a transformação nesta fase.

Não use apenas um filtro azul.

## 9. Comparação no jogo

Crie capturas comparativas com:

- Personagem atual.
- Protótipo B em idle.
- Protótipo B em corrida.
- Protótipo B em ataque.

Use o cenário e a câmera reais da V2.3.

Se for possível criar um visualizador isolado sem modificar o jogo principal, faça isso.

Não altere o jogo principal para demonstrar o protótipo.

## 10. Entregas

Na pasta `d2_protagonista_B/`, inclua:

- Referência-mestre.
- Paleta.
- Poses.
- Quadros de animação.
- Spritesheets, se tecnicamente válidas.
- Prévia animada.
- Comparação em gameplay.
- Documento técnico.
- Relatório de limitações.
- Estimativa de trabalho restante.

## 11. Critérios de aceite

O protótipo deve demonstrar:

1. Identidade visual consistente.
2. Idle convincente.
3. Corrida legível.
4. Ataque compatível com o gameplay.
5. Transparência e pivôs corretos.
6. Boa leitura na câmera V2.3.
7. Nenhuma alteração no jogo publicado.

Se não conseguir produzir animações consistentes, não declare a tarefa concluída como animação funcional. Entregue o que foi realmente produzido e explique o que falta.

## 12. Relatório final

Informe:

A. Estado inicial do Git.

B. Referências utilizadas.

C. Identidade visual definida.

D. Referência-mestre.

E. Idle.

F. Corrida.

G. Ataque.

H. Equipamentos.

I. Transformação espectral.

J. Comparações na escala real.

K. Arquivos gerados.

L. Limitações.

M. Viabilidade de integração.

N. Próxima fase recomendada.

O. Estado final do Git.

**Não publique sprites, não faça commits, não execute push e não altere gameplay.**

Aguarde minha revisão antes de integrar qualquer arte ao jogo.