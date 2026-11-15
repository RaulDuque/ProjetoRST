# Mapa e exploração (V1.6)

> - **[Protótipo]**: estados de descoberta, porcentagem e painel do mapa
>   (`world/exploration.rs`, `ui/world_map.rs`).
> - **[Planejado]**: mapa em tela cheia, marcadores, ícones de save e de teleporte, estado `Completed`.

## 1. Estados de descoberta

| Estado | Quando | No mapa |
|---|---|---|
| `Unknown` | Nunca vista | Não aparece |
| `Discovered` | Está atrás de uma porta conhecida de uma sala visitada | Contorno |
| `Visited` | O jogador entrou | Preenchida com a cor da região |
| `Completed` **[Planejado]** | Todas as coletas e segredos da sala encontrados | Marca dourada |

**O que não revela salas:**
- portas escondidas (`hidden_by`) antes de o segredo ser encontrado;
- portas só de chegada (`entry_only`): o atalho aparece quando é usado pelo
  outro lado. Isso foi decidido no teste, para não entregar a poterna antes da hora.

**Visitar não é completar:** uma sala pode estar visitada e ter um segredo não
encontrado. A V1.8 traz o estado `Completed`.

## 2. Porcentagem de exploração

```text
exploração (%) = 100 × salas visitadas / total de salas do mundo carregado
```

- **Salas secretas:** entram no total, então 100 % exige encontrar as salas
  secretas, o que é a recompensa da exploração cuidadosa.
- **Indicadores separados:**
  - salas conhecidas (`known` = descobertas + visitadas);
  - segredos encontrados / total (`secrets_found / secrets_total`).
- **Consistência:**
  - **Calculada, nunca armazenada:** o save guarda ids; a porcentagem é recalculada
    a partir do mundo carregado.
  - **Conteúdo novo:** uma região nova baixa a porcentagem em vez de deixá-la
    inconsistente.
  - **Salas removidas:** ids de salas que não existem mais são ignorados (testado).
- **Por região [Planejado]:** a mesma fórmula restrita às salas da região,
  exibida no mapa em tela cheia.

## 3. Painel (no jogo desde a V1.7)

- **Posição:** canto superior direito, em UI (não usa a câmera do mundo).
- **Escala:** cada célula vira 26 × 15 px.
- **Desenho:**
  - salas visitadas: preenchidas;
  - salas descobertas: só o contorno;
  - sala atual: borda clara.
- **Legenda:** `Map 44% rooms 4/9 secrets 1/1`.
- **Redesenho:** só quando a exploração, a sala atual ou o painel mudam.
- **V1.7:**
  - a tecla `M` abre e fecha o painel;
  - portas conhecidas aparecem como marcas (azuis se trancadas);
  - um segredo encontrado aparece como marca dourada;
  - o painel fica oculto sem mundo ativo;
  - salas secretas e portas `hidden_by` só aparecem depois da descoberta.

**V1.8:**
- baús das salas visitadas (laranja se fechados, apagados se abertos);
- atalho aberto em verde;
- marca de segredo também para paredes quebráveis e interruptores;
- linha com a porcentagem da região atual (`region_stats`).

A fórmula global não mudou, e uma região nova entra no denominador. Ver
[`WORLD_EXPLORATION_V1_8.md`](WORLD_EXPLORATION_V1_8.md) §12.

**V1.9:**
- portas com véu espectral em azul-claro;
- a região Claustro Velado (2 salas) entra no total, que passa a 22 salas.

Ver [`TRANSFORMATIONS_V1_9.md`](TRANSFORMATIONS_V1_9.md).

## 4. Mapa labiríntico: o que o modelo suporta

| Recurso | Como |
|---|---|
| Corredores longos | Salas `w` × 1 (o protótipo tem 2×1) |
| Poços, torres | Salas 1 × `h` (o protótipo tem 1×3) com portas em células diferentes |
| Salas sobrepostas visualmente | Permitidas: o validador exige só que portas pareadas encostem |
| Conexões ocultas | `hidden_by` |
| Passagens falsas | **[Planejado]** `SecretKind::FalseDoor`, uma porta desenhada que é parede |
| Elevadores | **[Planejado]** sala com plataforma móvel e duas portas `Top`/`Bottom` |
| Quedas sem volta | `entry_only` no lado de chegada |
| Caminhos que voltam | Ciclos no grafo (validados) |

## 5. Planejado (V1.8)

- **Mapa em tela cheia (a tecla M hoje abre o painel):**
  - pan;
  - regiões nomeadas;
  - ícones de save, teleporte, loja e chefe derrotado;
  - marcadores do jogador (3 a 5).
- **Portas no mapa:** pequenos vãos nas bordas das salas; cadeados nas portas `Locked` conhecidas.
- **Itens de mapa:** "Carta da região" revela as salas `Discovered` sem visitá-las (não revela segredos).
