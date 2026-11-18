# DARK CASTLE RPG — MEGA PROMPT D12
## Correção Definitiva de Escala, Pixel Art, Contraste, Silhueta e Equipamentos — Blender + Bevy 0.20.1

**Projeto:** Dark Castle RPG  
**Engine:** Rust 2024 / Bevy 0.20.1  
**Fase:** D12 — Recuperação da Qualidade Visual  
**Referências internas:** D2, D7-B, D8, D9, D10 e D11  
**Identidade artística:** protagonista gótico aristocrático, cabelo escuro, casaco vinho, capa, espada e equipamentos modulares  
**Objetivo:** fazer o protagonista parecer uma obra de pixel art gótica detalhada, legível e consistente dentro do jogo  
**Estado:** laboratório experimental; nenhuma aprovação para substituir o visual padrão

---

# 1. MISSÃO PRINCIPAL

A D11 comprovou que o Bevy consegue renderizar um protagonista modular.

Entretanto, o proprietário rejeitou a qualidade visual da integração atual.

Os problemas reportados incluem:

1. Equipamentos aparentemente desproporcionais.
2. Escala visual inconsistente.
3. Pixels irregulares.
4. Contornos com possíveis resíduos de alfa.
5. Personagem escuro demais sobre fundos escuros.
6. Espada pouco distinguível da roupa.
7. Falta de separação entre cabelo, rosto e casaco.
8. Armaduras com iluminação pouco convincente.
9. Falta de sombra de contato.
10. Dificuldade para avaliar a arte na escala real.
11. Possível perda de detalhes na conversão para atlas.
12. Qualidade artística inferior às referências aprovadas D2/D7-B.

A missão da D12 é diagnosticar e corrigir esses problemas.

**Não basta fazer o sistema compilar.**

**Não basta produzir PNGs tecnicamente válidos.**

**Não basta aplicar um filtro ou aumentar o zoom.**

O resultado precisa melhorar visualmente de maneira demonstrável.

---

# 2. REGRAS ABSOLUTAS

Antes de trabalhar:

1. Leia `CLAUDE.md`.
2. Leia as regras e documentos relevantes.
3. Confirme branch, HEAD e upstream.
4. Execute `git status --short`.
5. Preserve todas as alterações existentes.
6. Inspecione os laboratórios D2, D7, D8, D9, D10 e D11.
7. Reproduza os testes da D10 e da D11.
8. Confirme a versão real do Blender e do Bevy.

Crie a documentação e as novas fontes em:

`art-src/visual-prototypes/d12_correcao_escala_pixel/`

São permitidas alterações estritamente necessárias no exemplo experimental Bevy, no módulo visual experimental e em assets experimentais.

Não altere:

- Gameplay.
- Combate.
- Fórmulas de dano.
- Física.
- Colisões.
- Progressão.
- Inventário funcional.
- Save V3.
- Câmera principal.
- Parallax V2.3.
- Cenários aprovados.
- Laboratórios anteriores.
- Arquivos de `Md-usario/`.

Não faça commit nem push.

Não use comandos destrutivos.

Não substitua o protagonista padrão.

---

# 3. MARCO A — AUDITORIA COMPLETA DA D11

Inspecione os arquivos reais:

- `examples/modular_visual_lab.rs`
- Módulo `modular_lab.rs`, onde estiver localizado.
- `assets/experimental/d11/`
- `d11_integracao_experimental/RELATORIO.md`
- Contratos de atlas.
- Scripts de exportação.
- Imagens do D10.
- Screenshots do Bevy.
- Manifests.
- Sockets.
- Pivôs.

Investigue a cadeia completa:

**Arte-fonte → máscaras → composição → atlas → textura → Sprite → Transform → câmera → pixels exibidos.**

Registre:

- Resolução original.
- Bounding box alfa.
- Dimensões de cada peça.
- Dimensões da célula do atlas.
- Pivôs.
- Sockets.
- Escala do corpo.
- Escala dos equipamentos.
- Escala das entidades-pai.
- Escala da câmera.
- Resolução da janela.
- Pixels por unidade de mundo.

Verifique especialmente o contrato informado pela D11:

- Atlas de 128 × 160.
- Pivô `(64, 156)`.
- Altura visual de aproximadamente 52 unidades.
- Filtragem nearest.

Não considere esses valores corretos sem verificar sua utilização real.

Crie `AUDITORIA_D11.md`.

---

# 4. MARCO B — ESCALA E PROPORÇÕES

Corrija primeiro a escala.

O protagonista e seus equipamentos devem compartilhar um sistema de coordenadas coerente.

Verifique:

- Transformações herdadas.
- Escala duplicada.
- Coordenadas locais.
- Pivôs.
- Sockets.
- Dimensões do atlas.
- Bounding boxes.
- Ancoragem dos pés.
- Origem das animações.
- Diferenças entre render clássico e pesado.

Cada equipamento deve respeitar a anatomia do personagem.

O elmo deve acompanhar a cabeça.

As luvas devem acompanhar as mãos.

As botas devem acompanhar os pés.

A espada deve acompanhar a mão dominante.

O escudo deve acompanhar o braço correspondente.

A bainha deve acompanhar o quadril.

A armadura deve acompanhar o tronco.

A capa deve acompanhar os ombros e as costas.

Não corrija um erro de socket apenas aplicando escala arbitrária.

Crie `CONTRATO_COORDENADAS.md`.

---

# 5. MARCO C — LEGIBILIDADE DA SILHUETA

O protagonista precisa ser reconhecível mesmo em movimento e sobre cenários escuros.

Preserve sua identidade:

- Cabelo escuro.
- Rosto relativamente pálido.
- Casaco vinho.
- Capa escura.
- Elementos metálicos.
- Aparência gótica aristocrática.

Não transforme o personagem em Alucard.

Não substitua o cabelo escuro por cabelo branco.

Não copie designs protegidos.

Melhore a separação visual entre:

- Cabeça e fundo.
- Cabelo e rosto.
- Ombros e capa.
- Tronco e braços.
- Casaco e pernas.
- Espada e roupa.
- Botas e chão.
- Escudo e armadura.

## Teste de silhueta

Produza versões:

A. Silhueta preta sobre fundo branco.

B. Silhueta branca sobre fundo preto.

C. Personagem colorido sobre fundo quase preto.

D. Personagem colorido sobre fundo cinza.

E. Personagem colorido sobre o cenário V2.3.

O protagonista precisa continuar identificável nas cinco situações.

---

# 6. MARCO D — CONTRASTE E PALETA

Crie um estudo de contraste sem descaracterizar a arte.

Investigue:

- Luminância do cabelo.
- Luminância do rosto.
- Contraste entre casaco e capa.
- Brilho da espada.
- Brilho das bordas metálicas.
- Saturação do vinho.
- Separação dos materiais.
- Detalhes dourados e bronze.

Proponha três variantes:

**A — D7-B preservado**

Alterações mínimas de contraste.

**B — Gótico legível**

Melhoria moderada de luz e separação entre materiais.

**C — Gótico dramático**

Contraste mais forte, realces metálicos e luz de recorte controlada.

A variante C não pode virar um personagem com contorno neon.

Não altere permanentemente a paleta sem comparação.

O proprietário deverá escolher a variante final.

---

# 7. MARCO E — ESPADA E MÃOS

A espada precisa ser imediatamente reconhecível.

Na pose idle atual, investigue se a lâmina se confunde com a perna ou a capa.

Teste:

- Separação maior entre braço e tronco.
- Pequena alteração na inclinação da espada.
- Maior contraste da lâmina.
- Reflexo metálico localizado.
- Guarda da espada mais legível.
- Posição alternativa da mão.
- Espaço negativo entre arma e roupa.

Não aumente a espada indiscriminadamente.

Se uma alteração de espessura for necessária, apresente versões comparativas.

Não altere hitboxes de combate.

Não altere alcance de ataques.

A mudança é visual.

---

# 8. MARCO F — PIXEL ART E LIMPEZA DE ALFA

Investigue os problemas de pixels na origem.

Diferencie:

1. Artefatos de redimensionamento.
2. Filtragem inadequada.
3. Escala fracionária.
4. Posicionamento subpixel.
5. Resíduos de chroma.
6. Pixels semitransparentes.
7. Contaminação RGB em áreas transparentes.
8. Pixel bleeding.
9. Bordas duplicadas.
10. Compressão de vídeo nas evidências.

**Não confunda artefatos do vídeo comprimido com defeitos comprovados no PNG original.**

Compare diretamente os PNGs sem perdas.

Crie diagnósticos de:

- Distribuição de alfa.
- Contornos.
- Pixels isolados.
- Cores próximas das bordas.
- Diferenças entre frames.
- Integridade de máscaras.

Teste alfa binário como uma variante, não como correção universal.

O corte rígido de alfa pode eliminar detalhes delicados, especialmente cabelo, tecido e efeitos translúcidos.

Se usar um threshold, documente o valor e compare o resultado com o original.

Preserve transparência gradual onde ela for artisticamente necessária.

---

# 9. MARCO G — BLENDER COMO FERRAMENTA DE ACABAMENTO

O usuário quer investigar um acabamento pré-renderizado mais sofisticado, com inspiração na estética de jogos clássicos de sprites detalhados.

Entretanto, as fases D3–D6 já demonstraram que renders EEVEE/toon não recuperam automaticamente a qualidade D2.

Portanto, Blender deve ser tratado como uma ferramenta auxiliar experimental.

## 9.1 Auditoria do modelo

Identifique:

- Arquivo `.blend` original.
- Versão do Blender.
- Motor de render.
- Câmera ortográfica.
- Malhas modulares.
- Rig.
- Materiais.
- Texturas.
- Luzes.
- Configuração de transparência.
- Pipeline de exportação.

Não altere o arquivo original.

Crie uma cópia de laboratório, se necessário.

## 9.2 Silhueta e proporções

Experimente ajustes moderados em:

- Espessura da espada.
- Ombreiras.
- Gola.
- Botas.
- Luvas.
- Escudo.
- Capa.

Não aplique automaticamente aumentos de 20% a 30%.

Esse intervalo é apenas uma hipótese artística a testar.

Produza comparações na resolução final.

## 9.3 Luz de recorte

Teste uma luz fria ou prateada posicionada atrás/acima do personagem.

O objetivo é destacar:

- Cabelo.
- Ombros.
- Capa.
- Espada.
- Armadura.

Produza três variantes:

A. Sem rim light.

B. Rim light discreta.

C. Rim light dramática.

O efeito precisa continuar legível após rasterização em baixa resolução.

Não crie contorno brilhante uniforme ao redor do personagem.

## 9.4 Materiais

Experimente:

- Texturas de baixa resolução com amostragem nearest/closest.
- Materiais estilizados.
- Realces metálicos.
- Contraste de tecido.
- Reflexos controlados.
- Sombras em poucos níveis.
- Quantização de cores.

Não presuma que PBR ou toon sejam superiores ao acabamento D7-B.

Compare sempre.

## 9.5 Compatibilidade técnica

Verifique as APIs reais do Blender instalado.

Não presuma que `Film > Filter Size = 0.01px` esteja disponível ou seja a configuração ideal.

Não presuma que `Shader to RGB` funcione igualmente em todos os motores de render.

Não altere o motor de render sem documentar diferenças.

---

# 10. MARCO H — TRÊS PIPELINES DE ACABAMENTO

Produza três candidatos utilizando a mesma pose e escala.

## Pipeline A — Pixel art D10 refinada

- Fonte D10.
- Correções manuais ou programáticas.
- Máscaras limpas.
- Contraste aprimorado.
- Contornos consistentes.

## Pipeline B — Blender pré-renderizado

- Modelo modular.
- Câmera ortográfica.
- Iluminação estilizada.
- Materiais ajustados.
- Renderização de referência.
- Conversão controlada para pixel art.

## Pipeline C — Híbrido

- Pose e profundidade vindas do Blender.
- Máscaras e composição D10.
- Acabamento pixel art refinado.
- Ajustes localizados de luz e contraste.

Compare:

- Identidade D7-B.
- Silhueta.
- Materiais.
- Espada.
- Rosto.
- Armadura.
- Contornos.
- Consistência entre frames.
- Modularidade.
- Tempo de produção.
- Complexidade de manutenção.

**Não escolha o Blender automaticamente.**

A estratégia vencedora será aquela que apresentar a melhor qualidade visual e preservar a modularidade.

---

# 11. MARCO I — SOMBRA DE CONTATO

Crie uma sombra de contato experimental.

Características:

- Elipse discreta.
- Cor escura semitransparente.
- Posicionamento abaixo dos pés.
- Ordem de desenho atrás do personagem.
- Largura proporcional ao corpo.
- Comportamento configurável.

Durante o salto:

- A sombra permanece projetada no chão.
- Pode diminuir de tamanho.
- Pode perder opacidade.
- Não acompanha verticalmente o personagem como se estivesse colada às botas.

Durante a corrida:

- A sombra acompanha a posição horizontal.
- Não deve piscar.
- Não deve alterar a colisão.

A sombra deve ser um efeito puramente visual.

Não adicione uma entidade física.

Não altere o sistema de gravidade.

---

# 12. MARCO J — AFTER-IMAGE B

A D10 selecionou a variante vinho como candidata.

A D11 implementou um estudo experimental.

Nesta fase, avalie somente a legibilidade visual do efeito.

Verifique:

- Contraste com fundo escuro.
- Transparência.
- Saturação.
- Contorno.
- Silhueta.
- Equipamentos incluídos.
- Persistência.
- Quantidade de rastros.

Compare:

A. Vinho original.

B. Vinho com bordas ligeiramente mais claras.

C. Vinho com realce magenta discreto.

Não utilize composição aditiva automaticamente.

Se testar blend aditivo, faça-o como variante isolada e verifique compatibilidade com o renderer real.

Não transforme o personagem em uma mancha luminosa.

Não implemente novas mecânicas.

---

# 13. MARCO K — ANIMAÇÕES

Depois de aprovar uma pose estática, valide:

- Idle clássico.
- Idle pesado.
- Corrida clássica.
- Corrida pesada.
- Troca dinâmica.

Verifique:

- Posição dos pés.
- Proporções.
- Escala.
- Espada.
- Capa.
- Elmo.
- Escudo.
- Armadura.
- Luvas.
- Contornos.
- Consistência da iluminação.

A troca de equipamento deve preservar a fase do ciclo.

Não produza uma animação inteira com um acabamento diferente do aprovado para a pose estática.

Não esconda dependências de quadros completos D2.

---

# 14. MARCO L — COMPARAÇÃO NA ESCALA REAL

Utilize a configuração de câmera existente no jogo.

Não altere a escala principal do projeto.

Compare:

- D7-B.
- D10.
- D11.
- D12 Pipeline A.
- D12 Pipeline B.
- D12 Pipeline C.

Produza capturas em:

- Escala real.
- Zoom 2×.
- Zoom 4×.
- Fundo escuro.
- Fundo claro.
- Cenário V2.3.

O zoom deve ser usado para inspeção.

Não use zoom para esconder problemas na escala real.

---

# 15. MARCO M — TESTES AUTOMÁTICOS

Inclua testes de:

- Atlas.
- Máscaras.
- Alfa.
- Pivôs.
- Sockets.
- Escala.
- Dimensões.
- Coordenadas.
- Equipamentos.
- Continuidade dos frames.
- Integridade de transparência.
- Ausência de resíduos.
- Troca dinâmica.
- Reprodutibilidade.
- Integridade dos arquivos exportados.

Teste também:

- Equipamento ausente.
- Socket inválido.
- Escala incompatível.
- Pivô incorreto.
- Máscara vazia.
- Dimensão inesperada.
- Arquivo ausente.
- Frame ausente.

Não silencie erros.

---

# 16. EVIDÊNCIAS OBRIGATÓRIAS

Produza:

1. `01_D11_diagnostico.png`
2. `02_escala_e_pivos.png`
3. `03_silhueta.png`
4. `04_contraste_A_B_C.png`
5. `05_espada_e_maos.png`
6. `06_alfa_e_contornos.png`
7. `07_blender_luzes.png`
8. `08_blender_materiais.png`
9. `09_pipeline_A_B_C.png`
10. `10_classico_refinado.png`
11. `11_pesado_refinado.png`
12. `12_sombra_contato.png`
13. `13_after_image_B.png`
14. `14_idle_run.png`
15. `15_causeway_escala_real.png`
16. `16_resultado_final.png`

GIFs:

- `idle_classico.gif`
- `idle_pesado.gif`
- `run_classico.gif`
- `run_pesado.gif`
- `troca_dinamica.gif`
- `after_image_vinho.gif`

Separe claramente evidências de Blender, Python e Bevy.

Não apresente renders Blender como capturas do Bevy.

Não gere arquivos artificiais apenas para completar a lista.

---

# 17. CRITÉRIOS DE ACEITAÇÃO

## Escala e equipamentos

- [ ] Proporções corretas.
- [ ] Pivôs consistentes.
- [ ] Sockets consistentes.
- [ ] Sem escala duplicada.
- [ ] Equipamentos encaixados.
- [ ] Sem mudanças de tamanho entre frames.

## Arte

- [ ] Identidade D7-B preservada.
- [ ] Silhueta legível.
- [ ] Espada distinguível.
- [ ] Rosto legível.
- [ ] Cabelo separado do fundo.
- [ ] Capa distinguível do casaco.
- [ ] Metais com contraste adequado.
- [ ] Sem perda grave de detalhes.
- [ ] Pixels consistentes.
- [ ] Contornos limpos.

## Pipeline

- [ ] Pipeline A avaliado.
- [ ] Pipeline B avaliado.
- [ ] Pipeline C avaliado.
- [ ] Diferenças documentadas.
- [ ] Recomendação fundamentada.
- [ ] Sem retorno automático ao toon antigo.

## Runtime

- [ ] Clássico renderizado no Bevy.
- [ ] Pesado renderizado no Bevy.
- [ ] Sombra de contato experimental.
- [ ] After-image avaliado.
- [ ] Idle consistente.
- [ ] Corrida consistente.
- [ ] Troca dinâmica preservada.

## Segurança

- [ ] Visual padrão intacto.
- [ ] Save V3 intacto.
- [ ] Gameplay intacto.
- [ ] Câmera principal intacta.
- [ ] Parallax intacto.
- [ ] Sem commit.
- [ ] Sem push.

---

# 18. PONTOS DE CONTROLE

Execute na seguinte ordem:

**A — Auditoria técnica:** identificar causas reais dos problemas.

**B — Escala:** corrigir proporções e coordenadas.

**C — Silhueta:** melhorar a leitura do protagonista.

**D — Contraste:** apresentar três variantes.

**E — Espada:** corrigir legibilidade.

**F — Alfa:** limpar pixels e contornos.

**G — Blender:** testar iluminação e materiais.

**H — Pipelines:** comparar A, B e C.

**I — Sombra:** testar contato com o chão.

**J — After-image:** avaliar vinho e realces.

**K — Animação:** validar idle e corrida.

**L — Cenário:** comparar na escala real.

**M — Testes:** validar regressões.

Se a escala estiver errada, não avance para acabamento.

Se a silhueta estiver ruim, não avance para animações completas.

Se o Blender piorar a qualidade, descarte o candidato Blender e preserve a arte D10.

Se a arte não estiver pronta, não declare aprovação visual.

---

# 19. DOCUMENTAÇÃO

Crie:

```text
art-src/visual-prototypes/d12_correcao_escala_pixel/
├── README.md
├── AUDITORIA_D11.md
├── CONTRATO_COORDENADAS.md
├── ANALISE_SILHUETA.md
├── ANALISE_CONTRASTE.md
├── ANALISE_PIXEL_ART.md
├── ESTUDO_BLENDER.md
├── COMPARACAO_PIPELINES.md
├── SOMBRA_CONTATO.md
├── AFTER_IMAGE.md
├── TESTES.md
├── LIMITACOES.md
├── RELATORIO_FINAL.md
├── DECISAO.md
├── fontes/
├── blender/
├── mascaras/
├── atlas/
├── equipamentos/
├── animacoes/
├── previews/
├── comparacoes/
├── evidencias_bevy/
├── scripts/
└── testes/
```

Evite duplicar arquivos sem necessidade.

Não gere documentos vazios.

---

# 20. RELATÓRIO FINAL

Apresente:

A. Estado inicial do Git.

B. Problemas encontrados na D11.

C. Causa-raiz dos erros de escala.

D. Causa-raiz dos erros de pixels.

E. Correções de proporção.

F. Correções de sockets.

G. Correções de silhueta.

H. Correções de contraste.

I. Correções da espada.

J. Limpeza de alfa.

K. Resultados Blender.

L. Comparação dos pipelines A/B/C.

M. Sombra de contato.

N. After-image.

O. Idle e corrida.

P. Testes.

Q. Evidências Bevy.

R. Limitações.

S. Estado final do Git.

T. Recomendação para D13.

Não faça commit.

Não faça push.

Não altere o visual padrão.

Aguarde aprovação.

---

# 21. DIRETRIZ FINAL

O Dark Castle RPG não precisa simplesmente de um personagem 3D convertido em PNG.

Ele precisa de um protagonista com identidade artística própria.

A D12 deve combinar:

- Silhueta forte.
- Proporções corretas.
- Equipamentos modulares.
- Contraste gótico.
- Materiais legíveis.
- Pixels limpos.
- Iluminação controlada.
- Animações consistentes.
- Renderização correta no Bevy.

**O Blender é uma ferramenta, não a identidade visual.**

**A fidelidade D7-B/D10 é o ponto de partida, não algo a descartar.**

**O resultado final precisa ser avaliado visualmente pelo proprietário antes de qualquer integração definitiva.**