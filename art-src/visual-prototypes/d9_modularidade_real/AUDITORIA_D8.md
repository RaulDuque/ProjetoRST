# Auditoria D8 (entrada da D9)

## Confirmação dos problemas

| # | Problema | Causa real no código/arte |
|---|----------|---------------------------|
| 1 | Clássico depende de `full_classic_ref` | `compose_d8.classic_parts()` usa o sprite D2 completo como `body` |
| 2 | `coat_back` / `cape_front` / `hair_back` vazios | Frame lateral não separa planos; cortes soft não inventam pixels |
| 3 | Luvas/bainha fracos | Recorte por banda/cor; poucos pixels |
| 4 | Pesado ≠ shading D2 | Peças D4 com chroma residual e iluminação diferente |
| 5 | Armadura só translada | `place_at_socket` = delta XY sem rotação/segmentos |
| 6 | Capa A ok; B/C limitados | B shear estica; C só offset |

## O que a D8 acertou

- Identidade D7-B no clássico (via sprite completo).
- Silhueta pesada distinta (elmo+escudo).
- Sockets lab idle×6 / run×8.
- Pipeline reproduzível; testes técnicos.

## Conclusão

Modularidade D8 é **parcial / falsa no clássico**: trocar “casaco” sem o sprite
completo não funciona de verdade. D9 deve reconstruir camadas e comprovar
remoção/substituição sem buracos nem resíduos.
