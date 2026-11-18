# N3 — Métricas

Medidas nesta máquina, em 18 de novembro de 2026. Não são horas de artista.

| Medida | Valor |
|---|---|
| Gerador Pillow, todos os clipes e as folhas de evidência | 1,25 s (`generated/metrics.json`) |
| Um quadro idle do manequim no Blender 5.2.4 | cerca de 11 s, código 0 |
| Pixelorama | ausente no PATH; não instalado |
| Corpo idle | 97 texels opacos de altura |
| Vista close na janela | 193 px (`bevy_attack.png`) e 214 px (`bevy_idle.png`, inclui a arma baixa) |
| Vista no enquadramento do jogo | 86 px (`bevy_framing_game.png`) |
| PNG do corpo leve / pesado / capa / espada / lança | 14.528 / 13.957 / 6.538 / 308 / 257 bytes |
| RGBA descomprimido dessas cinco imagens | 3.216.576 bytes, cerca de 3,1 MiB |
| Ouro no corpo / nas armas | 0 / 30 pixels |
| `white_mesh.glb` | 3.387.148 bytes, não reescrito |

O alvo experimental de menos de 32 MiB de textura residente cabe com folga neste elenco. O número é o RGBA descomprimido, não uma medição de VRAM pelo driver.

Não há hora medida para "o primeiro clipe" e "o segundo" nas mãos de uma pessoa. O script gera os quatro de uma vez.
