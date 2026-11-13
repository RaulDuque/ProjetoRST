# Direção artística V1.1: protagonista

> **Status: proposta aguardando aprovação.** Nada aqui está aprovado como
> definitivo. Formatos técnicos (quadro, pivô, atlas, manifesto) ficam em
> [`ASSET_CONTRACT.md`](ASSET_CONTRACT.md), a fonte única para esses dados.

Decisões do usuário:
- **D1** (aprovada): conceito B, espadachim aristocrático sombrio.
- **D2** (em aberto): acabamento; comparar pixel art e alta resolução.
- **D3** (provisória): câmera 1:1 mantida; zoom ×2 apenas estudado. A decisão
  definitiva de zoom continua pendente (seção 7).

Até a V1.1 nenhuma imagem de referência foi fornecida. Na V1.2-B chegou
`art-src/player/inspiration/protagonist_concept_v001.png`; o conceito B tem
prioridade e os conflitos estão em `art-src/player/references/README.md`.

## 1. Identidade

Espadachim de fantasia sombria, aristocrático, misterioso e combativo.
Silhueta vertical e esguia que comunica velocidade, coerente com a esquiva de
0,28 s e o avanço do Crescent Slash.

**Originalidade.** O arquétipo "aristocrata sombrio de casaca e capa" é comum
em jogos góticos de exploração. Para não lembrar personagens conhecidos, o
design **evita** a combinação cabelo longo prateado + capa longa até o chão +
preto e dourado. Marcas próprias:

- cabelo **curto**, cinza-ardósia, com franja projetada para a frente;
- **meia-capa assimétrica**, só sobre o ombro de trás, na altura do quadril;
- casaca aberta com **forro vinho** visível na borda;
- acentos em **âmbar** (olho, fivela, guarda e pomo) em vez de ouro;
- espada longa e fina com **curvatura leve**, sem cruz ornamentada.

## 2. Proporções (7 cabeças)

Alturas medidas da sola, em cabeças (1 cabeça ≈ 0,26 m num modelo de 1,82 m).

| Marco | Altura | Observação |
|---|---|---|
| Topo do cabelo | 7,0 | |
| Queixo | 6,0 | Rosto ocupa a parte da frente da cabeça |
| Ombros | 5,6 | Ombreira pequena de prata só no ombro da espada |
| Cintura (cinto) | 4,3 | Fivela âmbar: um dos 4 pontos de cor que sobrevivem à escala |
| Virilha | 3,5 | |
| Topo das botas | 2,2 | Botas altas logo abaixo do joelho, com faixa de cano |
| Barra da casaca | 1,4 | Meia-canela; barra com friso prata |
| Sola | 0,0 | Linha do pivô |

**Silhueta lateral** (vista principal do jogo): cabeça pequena, gola alta atrás
da nuca, tronco estreito, casaca abrindo em trapézio para trás, meia-capa
quebrando a linha das costas, pernas finas e espada em diagonal à frente.
**Silhueta frontal**: estreita, ombros pouco mais largos que o quadril, casaca
aberta formando um "V" invertido que mostra pernas e botas.

## 3. Especificação por peça

| Peça | Forma | Material | Cor (paleta) |
|---|---|---|---|
| Cabeça e rosto | Oval estreito; nariz e queixo marcados de perfil | Pele | `skin`, `skin_shadow` |
| Olho | Um traço âmbar, sem pupila visível | Brilho | `amber` |
| Cabelo | Curto, volume na nuca, franja para a frente | Fosco | `hair`, `hair_light` |
| Gola | Alta, rígida, aberta na frente, forro vinho | Tecido grosso | `coat_light`, `lining_wine` |
| Casaca | Longa até a meia-canela, aberta, cauda mais longa atrás | Lã escura | `coat`, `coat_shadow`, `coat_light` |
| Forro | Visível na borda da abertura | Cetim | `lining_wine` |
| Meia-capa | Curta, sobre o ombro de trás | Lã | `cape_wine`, `cape_shadow` |
| Luvas | Justas, de couro | Couro | `glove` |
| Punhos e ombreira | Faixas e placa pequenas | Metal frio | `silver_dark` |
| Calças | Justas, escuras | Tecido | `trousers` |
| Botas | Altas, salto baixo, faixa de cano | Couro | `boots`, `boot_cuff` |
| Cinto e fivela | Faixa fina, fivela retangular | Couro e metal | `boots`, `amber` |
| Espada | Lâmina longa (~3,2 cabeças), fina, curva suave; guarda curta | Aço e latão | `silver`, `silver_light`, `amber` |

## 4. Paleta preliminar

Definida em `art-src/player/scripts/post/palette.py`, a fonte única usada pelos
scripts PIL e Blender.

| Nome | Hex | Uso |
|---|---|---|
| `outline_dark` | `#0d0b12` | Contorno escuro (testado; some contra o fundo) |
| `rim` | `#4a5570` | Contorno de recorte (rim) |
| `coat` / `coat_shadow` / `coat_light` | `#1d2434` / `#141a26` / `#2c3650` | Casaca, sombra e peito |
| `lining_wine` / `cape_wine` / `cape_shadow` | `#6e1f30` / `#571827` / `#3b101b` | Forro, capa e sombra da capa |
| `trousers` | `#262a36` | Calças |
| `boots` / `boot_cuff` | `#17161d` / `#3a3446` | Botas e cano |
| `silver` / `silver_light` / `silver_dark` | `#b9c3cf` / `#e6edf3` / `#7d8794` | Lâmina, fio e frisos |
| `skin` / `skin_shadow` | `#dcc6b4` / `#b39a8a` | Pele |
| `hair` / `hair_light` | `#3a3e4b` / `#5d6476` | Cabelo |
| `glove` | `#2c2329` | Luvas e empunhadura |
| `amber` / `amber_dark` | `#d18f2f` / `#8f5a1c` | Olho, fivela, guarda e pomo |

## 5. Distribuição dos detalhes e legibilidade

Na câmera 1:1 o personagem tem **~52 px de altura** na tela (o corpo de colisão
tem 46). Nessa escala, uma cabeça tem ~7 px.

**Regra de detalhe:** só entra no sprite o que ocupa **≥ 2 px** na resolução
final. O que sobrevive na comparação: silhueta, meia-capa vinho, faixa clara da
lâmina, fivela e guarda âmbar, rosto claro. O que some: olho, botões, frisos
finos e costuras, que ficam como textura do modelo, não como informação de jogo.

**Achados das comparações** (`art-src/player/comparisons/`):

1. **Casaca escura contra o fundo escuro some.** O fundo da sala (`#120f1a`) é
   quase da cor da casaca, e um contorno escuro não ajuda. Um **contorno de
   recorte** (rim) azul-acinzentado devolve a silhueta nos dois acabamentos.
   Recomendação: rim obrigatório (contorno de 1 px no A, luz de recorte no
   Blender e borda suave no B).
2. **A conversão automática para pixel art não basta.** A lâmina vira uma
   linha pontilhada e o olho desaparece. O acabamento A exige limpeza manual
   quadro a quadro.
3. **Alta resolução não ganha nada na câmera 1:1.** Reduzida para 52 px, fica
   mais borrada que a pixel art. Só mostra vantagem com zoom ×2.

## 6. Comparação de acabamentos (D2)

Mesmo personagem, pose e enquadramento: `finish_A_pixel_vs_B_hd.png`.
**Não há vencedor declarado:** a escolha depende da sua decisão de câmera.

| Critério | A: pixel art detalhada | B: alta resolução (2× densidade) |
|---|---|---|
| 1. Qualidade visual | Nítida e coesa; exige limpeza manual para manter a qualidade | Curvas suaves (lâmina, capa); em 1:1 fica borrada |
| 2. Leitura em movimento | Boa quando os quadros são limpos; detalhes de 1 px "piscam" entre quadros | Estável; bordas suaves escondem tremores, mas reduzem o contraste |
| 3. Cenário gótico | Combina com cenário também em pixel art | Pede cenário com o mesmo nível de acabamento |
| 4. Complexidade de produção | Render + limpeza manual por quadro | Render + retoque leve; menos trabalho por quadro |
| 5. Novas animações | Cada animação nova custa limpeza | Mais barato depois do rig pronto |
| 6. Consistência | Depende do artista na limpeza | Alta (vem do rig e da câmera fixos) |
| 7. Memória (estimativa, não medida) | ~3,0 MiB RGBA8 para os 112 quadros (lista na seção 2 do contrato) | ~12 MiB (4× texels) |
| 8. Manutenção | Mudar a roupa exige limpar tudo de novo | Mudar a roupa = renderizar de novo + retoque |
| 9. Trocar armas e roupas | Possível com camadas, mas cada combinação pede limpeza | Mais simples (camadas renderizadas) |
| 10. Bevy | Filtro nearest (sampler por imagem), posição em pixel inteiro, `Msaa::Off`, só zoom inteiro | Filtro linear (padrão atual), aceita zoom fracionário; 2 px de padding e sangria de cor na borda |

Detalhes técnicos de filtro, padding e pivô: [`ASSET_CONTRACT.md`](ASSET_CONTRACT.md).

**Cálculo da memória** (aritmética, sem medição):
- **Quadros:** 112, dos quais 41 de ataque e magia em células de 128×96 e 71 de
  movimento em 64×64.
- **A, RGBA8 sem mipmaps:** 41 × 48 KiB + 71 × 16 KiB ≈ 3,0 MiB.
- **B:** densidade 2, logo 4× os texels, ≈ 12 MiB.
- **Fora da conta:** as sobras de atlas e a cópia na RAM que o Bevy pode manter
  (`RenderAssetUsages`).

**Leitura técnica, condicionada à câmera:**
- **Com câmera 1:1:** A dá melhor leitura e gasta menos memória.
- **Com zoom ×2:** os dois são viáveis; B reduz o custo por animação.
- **Com zoom fracionário (×1,5):** só B, porque pixel art exige escala inteira.

## 7. Câmera: 1:1 × zoom ×2 (D3)

Composição experimental feita a partir de `entry_hall.ron`, com a mesma regra
de `camera.rs`. A câmera do jogo **não foi alterada**. Imagens:
`camera_{spawn,combat}_{1x_pixel,2x_pixel,2x_hd}.png`.

| Critério | 1:1 (atual) | Zoom ×2 (estudo) |
|---|---|---|
| Legibilidade do personagem | ~52 px: só a silhueta e as manchas de cor | ~104 px: rosto, capa e espada legíveis |
| Área visível | 1280×720 u, 43% da sala de 2400×900 | 640×360 u, 11% da sala |
| Visão à frente / atrás (olhando à direita, com o deslocamento de 60 u) | ~700 u / ~580 u | ~380 u / ~260 u |
| Inimigos | `aggro_range` 260 u bem dentro da tela | Um inimigo que o detecta por trás está **na borda** da tela (260 u) |
| Antecipação de ataques | Preparo do inimigo pequeno, mas visto de longe | Preparo grande e claro, visto mais tarde |
| Espaço para saltos | Altura do salto ~136 u (720²/2·1900): sobra | Cabe; da altura do chão, a plataforma de y=360 (topo 376) fica **fora** da vista |
| Plataformas | Todas da área vistas de uma vez | Plataformas altas só aparecem quando a câmera sobe |
| Efeitos | Ember Bolt (alcance 600 u) visível o caminho todo | O projétil sai da tela depois de ~380 u |
| Exploração e combate | Favorece planejamento e leitura do espaço | Favorece leitura do personagem; reduz a consciência do espaço |

O HUD ocupa a mesma fração da tela nas duas escalas (362×150 px, ~5,9%). Em
unidades de mundo, no ×2 ele cobre 181×75 u, um quarto da área coberta em 1:1.

**Opção a estudar, se você quiser:** zoom ×1,5 (vista de 853×480 u, personagem
de ~78 px). Só é compatível com o acabamento B.

## 8. Folha de referência (planejada)

Uma imagem por versão do modelo, gerada pelo Blender (`previews.py`):

| Item | Conteúdo |
|---|---|
| Vistas | Frontal, lateral e traseira, câmera ortográfica, mesma escala |
| Poses | Neutra (braços soltos) e com espada (a pose atual da prévia) |
| Paleta | Faixa com os nomes da seção 4 |
| Detalhes | Recortes ampliados da gola, da fivela, da meia-capa e da espada (guarda, curvatura) |
| Escala | Silhueta em 52 px e 104 px ao lado da vista lateral |

## 9. Pendências artísticas

- Aprovar ou ajustar o design, a paleta e as marcas de originalidade.
- Escolher o acabamento (D2) depois da decisão de câmera.
- Decidir a câmera (D3): manter 1:1, ×2 ou estudar ×1,5.
- Avaliar o modelo-base real `v002` (V1.2-B, Blender 5.2.4):
  `art-src/player/comparisons/v1_2b_swordsman_base_v002/`. As folhas
  anteriores, da figura vetorial, continuam em `comparisons/`.
- Confirmar origem e licença da referência recebida na V1.2-B.
- Avaliar o modelo v003 (V1.2-C, malhas procedurais):
  `art-src/player/comparisons/v1_2c_swordsman_v003/`, incluindo o teste C
  (cores chapadas) e a variante de lâmina mais larga. Nada aprovado.
- Rig: avaliado em `art-src/README.md`, adiado para V1.3/V1.4.
