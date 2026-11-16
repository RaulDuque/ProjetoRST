# Limitações do protótipo D4

## O que NÃO está pronto

- **Não é arte final de produção.** Camadas vêm de geração + chroma + registro
  por pés; há franjas de borda e overlap entre peitoral e braços.
- **Não há um spritesheet modular completo** para os 22 clips do jogador.
- **Registro entre camadas no run é frágil:** as GIFs de movimento usam um
  composite idle + um composite run completo (não overlay peça-a-peça no run).
- Chapéu/elmo/óculos **não** têm variantes por quadro de animação.
- Luvas e botas **não** têm sprites dedicados (só peitoral/capa/held/headgear).
- **Nada foi plugado no Bevy.** `src/`, `assets/`, saves intactos.
- Não há Save V3, novos `EquipmentSlot`, nem mudanças de inventário.
- Pipeline Blender A não foi re-exportado nesta pasta (só referenciado).

## Problemas observados no protótipo

1. **Matte rosa/magenta nos JPG:** o fundo gerado não é `#FF00FF` puro
   (ex.: `#E5007E`). A primeira passagem de chroma falhou e deixou retângulos
   opacos. Corrigido em `scripts/compose_d4.py` (matte hot-pink + flood).
2. **Corpo-base estreito:** após o chroma limpo, o bare body fica ~19 px de
   largura máxima (silhueta AI muito fina). Casaco/armadura restauram largura
   legível (~50–60 px). Em escala de jogo (~52 u), o bare sozinho é frágil.
3. **Casaco leve pode trazer capuz:** a camada `02_light_coat` às vezes oclui
   a cabeça do body (peça “cheia” em vez de só peitoral aberto).
4. **Armadura/capa geradas como silhueta quase completa:** demonstram troca de
   look, mas não são máscaras cirúrgicas parentadas a `chest`/`back`.

## Riscos se for para produção sem híbrido

1. Overlays 2D AI-isolados desalinhados em attack/run.
2. Cabelo sob elmo sem máscara → clipping.
3. Peitoral pesado sem deformação de manga → “armadura flutuante”.
4. Multiplicar peças × clips sem sockets → custo proibitivo.
5. Regenerar peças por IA sem pivô fixo → re-registro manual a cada item.

## Honestidade visual

O look **tenta** D2 (detalhe, capa, casaco, metal). Não é pixel-perfect com o
idle D2 publicado. Comparações `cmp_*` mostram escala/legibilidade, não
identidade 1:1. O protótipo **prova combinação de camadas**, não entrega arte
pronta para `assets/`.
