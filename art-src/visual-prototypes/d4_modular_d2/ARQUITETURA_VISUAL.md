# Arquitetura visual modular (proposta D4)

Objetivo: aparência **D2 detalhada** com peças combináveis, sem redesenhar o
protagonista inteiro a cada item.

## Ordem de desenho (atrás → frente)

1. Capa (atrás) — `01_cape_back`
2. Corpo-base (pele, cabelo, roupa íntima/calça base) — `00_body`
3. Calças / botas (quando forem camadas próprias)
4. Peitoral leve **ou** armadura pesada — `02_light_coat` / `03_heavy_armor`
5. Luvas / braços
6. Cabeça já no body; depois chapéu/elmo — `04_hat` / `05_helm`
7. Óculos/máscara — `06_glasses`
8. Escudo (mão esquerda / offhand) — `08_shield`
9. Arma principal — `07_sword`
10. FX de equipamento

Z sugerido (Bevy, relativo ao body ~0.05): capa 0.03, body 0.05, armor 0.06,
headgear 0.07, held back 0.04 / front 0.08 (já há `weapon.front` nos sockets).

## Pivôs e sockets

| Peça | Âncora | Notas |
|---|---|---|
| Body | pivô sola (contrato atual) | |
| Cape | `back` | V2.2 já usa |
| Weapon | `weapon` + ângulo | V2.2 já usa |
| Shield | `hand_l` / placement | V2.2 já usa |
| Hat/Helm/Glasses | `head` | precisa sprite por quadro |
| Coat/Armor | `chest` ou baked no body | peitoral largo ≠ tint |

## Oclusão

- Capa atrás do torso; forro vinho pode aparecer na frente da perna traseira.
- Escudo cobre o lado de trás da câmera quando `front=false`.
- Elmo deve ocluir cabelo (máscara de cabelo ou body “careca” + hair layer).

## Animação

- **Idle:** overlays quase estáticos aceitáveis se o pivô for estável.
- **Run/Attack:** cada camada precisa seguir o socket **ou** ser re-pixelada
  por quadro; senão desalinha (o teste de movimento D4 mostra o risco).

## Slots visuais vs funcionais

Slots visuais são camadas de render. Slots funcionais continuam os 8 atuais
até autorização explícita. Cosméticos (óculos) podem mapear para Head ou
Accessory sem novo enum — decisão do proprietário.
