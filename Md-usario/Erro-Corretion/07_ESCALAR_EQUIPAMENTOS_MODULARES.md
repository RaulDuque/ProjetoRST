# PROMPT 07 — ESCALABILIDADE SEM PERDER A BASE VESTIDA

Executar SOMENTE após o proprietário aprovar explicitamente a integração do Prompt 06. Leia `00_LEIA_PRIMEIRO.md`.

**Regra:** protagonista vestido aprovado é a base visual. Rig 3D técnico é invisível. Equipamentos externos cobrem/sobrepõem a roupa com oclusão correta; novas roupas/calças/botas que realmente **substituem** uma peça usam HIDE/REPLACE específico do slot, sem obrigar corpo nu completo.

1. Criar UM segundo modelo de armadura geometricamente diferente do primeiro (não só outra cor), no mesmo rig/poses e com PNGs exportados como camadas.
2. Criar UMA variante de material do primeiro modelo. Comparar modelagem nova versus troca de material, sem vender recoloração como silhueta inédita.
3. Demonstrar os dois modelos equipados no personagem vestido nas quatro poses, mostrando oclusão, volume, estabilidade e retorno perfeito à base original sem equipamento.
4. Medir trabalho de modelagem, ajustes, renderização, conversão, manifestos, armazenamento e correções. Não prometer geração automática perfeita de centenas de peças.
5. Apenas **planejar**, sem implementar, slots de botas/calças/luvas/capacetes, armas e capas. Distinguir peças externas (OVERLAY) de roupa a substituir (REPLACE/HIDE). Para botas e grevas, registrar necessidade de corrigir proporções das pernas do rig; para capa, ancoragem e animação próprias.
6. Verificar o que na N41-B é realmente integrado e o que continua como preview isolado. Reutilizar sistema do Bevy onde válido e evitar combinação pré-renderizada de todas as configurações, se isso causar explosão combinatória.

**Gate:** evidência de duas silhuetas distintas, uma variante, uma composição estável sem alterar o sprite padrão, e relatório curto de custo real. Não gerar catálogo em massa, instalar, alterar gameplay, commitar ou fazer push. Parar e pedir aprovação.
