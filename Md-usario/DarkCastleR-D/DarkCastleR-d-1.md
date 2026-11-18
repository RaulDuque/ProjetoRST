# DARK CASTLE RPG — DIREÇÃO ARTÍSTICA / FASE 1
## Protagonista pixel art, equipamentos e transformação espectral

O laboratório visual foi apresentado ao proprietário do projeto, que selecionou referências de que gostou.

Esta tarefa é transformar essas preferências em protótipos técnicos comparáveis, sem substituir a arte publicada do jogo.

## 1. Referências escolhidas

Use como referência principal os arquivos existentes:

- `art-src/visual-prototypes/07_comparacoes/COMPARE_characters.png`
- `art-src/visual-prototypes/01_personagem/CHAR-A_pixel_32.jpg`
- `art-src/visual-prototypes/01_personagem/CHAR-B_pixel_64.jpg`
- `art-src/visual-prototypes/03_armas_armaduras/ARMOR-on_character.jpg`
- `art-src/visual-prototypes/03_armas_armaduras/WEAPONS_ARMORS_sheet.jpg`
- `art-src/visual-prototypes/05_transformacoes/FORM-spectral_sequence.jpg`
- `art-src/visual-prototypes/05_transformacoes/FORM-future_concepts.jpg`
- `art-src/visual-prototypes/06_inimigos/ENEMY-archetypes_sheet.jpg`

Não trate os conceitos como sprites prontos para publicação.

Não copie diretamente assets protegidos de outros jogos.

## 2. Segurança

Confirme:

- Branch `art/v1.2-protagonist`.
- HEAD esperado `1bf2ed2`.
- Dois commits locais do B2 ainda não publicados.
- Estado do Git e arquivos pessoais.

Não altere o código principal.

Não substitua assets publicados.

Não modifique Save V3.

Não faça commit ou push.

Não altere outros conteúdos de `art-src/`.

Crie os novos protótipos em um subdiretório próprio de `art-src/visual-prototypes/`, preservando os arquivos existentes.

## 3. Protagonista — duas versões finalistas

Produza dois protótipos:

### Versão A — Pixel art compacto

Baseado em CHAR-A, com:

- Casaco gótico.
- Capa.
- Espada.
- Silhueta legível.
- Contraste suficiente com o fundo.
- Proporções consistentes.

### Versão B — Pixel art detalhado

Baseado em CHAR-B, com:

- Maior detalhamento do rosto e cabelo.
- Armadura identificável.
- Capa em camadas.
- Espada e bainha.
- Silhueta aristocrática.
- Paleta compatível com os cenários.

Ambas devem representar o mesmo protagonista, não personagens diferentes.

## 4. Animações de teste

Para cada versão, produza, quando tecnicamente viável:

- Idle.
- Caminhada ou corrida.
- Pulo.
- Queda.
- Ataque básico.
- Reação a dano.

Mantenha pivôs e escala consistentes.

Não use interpolação borrada para simular pixel art.

Se não for possível criar animações coerentes a partir dos conceitos, entregue poses-chave e um plano de produção, identificando claramente que ainda não são spritesheets completos.

## 5. Teste na escala real

Monte comparações usando a câmera atual da V2.3.

Compare:

- Personagem atual.
- Pixel A.
- Pixel B.

Nas mesmas posições, salas e condições de iluminação.

Mostre a imagem em tamanho real de gameplay e ampliada com nearest-neighbor.

Avalie:

- Legibilidade.
- Contraste.
- Silhueta.
- Arma.
- Capa.
- Contato com o chão.
- Relação com plataformas.
- Alinhamento com a hitbox.

Não altere o zoom para favorecer uma versão.

## 6. Equipamentos

Para cada protagonista, produza exemplos de:

- Armadura leve.
- Armadura pesada.
- Espada.
- Escudo.
- Capa alternativa.

Priorize um sistema de camadas reutilizáveis.

Investigue como manter sockets e pontos de ancoragem compatíveis com a arquitetura atual.

Não implemente a troca de equipamentos no jogo principal.

## 7. Transformação espectral

A aparência atual da transformação foi rejeitada como direção final.

Use FORM-spectral_sequence como referência para propor uma forma espectral visualmente distinta.

Crie:

- Silhueta humana.
- Silhueta espectral.
- Transição.
- Idle espectral.
- Movimento espectral.
- Retorno à forma humana.

Preserve a identidade visual do protagonista.

A transformação não deve parecer apenas um filtro azul aplicado ao personagem.

Use os conceitos FORM-golem e FORM-raven apenas como referências futuras, sem implementar novas formas.

## 8. Inimigos

Use ENEMY-archetypes_sheet como referência aprovada para direção artística.

Escolha dois arquétipos para estudos técnicos:

- Um inimigo corpo a corpo.
- Um inimigo mágico ou voador.

Crie exemplos visuais coerentes com a escala do protagonista.

Não implemente IA nem substitua o Bone Sentry nesta fase.

## 9. Entrega

Crie um diretório específico para esta fase, com:

- Protótipos A e B.
- Sprites ou poses-chave.
- Comparações em gameplay.
- Equipamentos.
- Transformação espectral.
- Dois estudos de inimigos.
- Documento técnico.
- Prancha final de comparação.

Crie também um arquivo de decisão no qual eu possa escolher:

- Protagonista A ou B.
- Estilo de armadura.
- Estilo de capa.
- Transformação espectral.
- Direção dos inimigos.

## 10. Critérios de aceite

A tarefa só estará concluída quando:

1. As versões forem visualmente comparáveis.
2. A escala real do jogo estiver demonstrada.
3. Os arquivos gerados existirem.
4. As limitações forem declaradas.
5. Nenhum asset publicado tiver sido alterado.
6. Nenhum comportamento de gameplay tiver sido modificado.
7. O Git estiver preservado.

Não declare uma spritesheet funcional se houver apenas uma imagem conceitual.

## 11. Relatório final

Informe:

A. Auditoria das referências.

B. Ferramentas usadas.

C. Protótipo A.

D. Protótipo B.

E. Comparação na escala real.

F. Equipamentos.

G. Transformação espectral.

H. Estudos de inimigos.

I. Arquivos gerados.

J. Limitações.

K. Estimativa de esforço para integrar cada versão.

L. Recomendação técnica.

M. Estado final do Git.

Não substitua a arte atual.

Não faça commit nem push.

Aguarde a escolha final do proprietário.