# D23 — Playtest humano

**Status: PENDENTE DE PLAYTEST HUMANO**

Nenhuma pessoa jogou esta luta. O probe de janela morreu com o chefe em
13 HP e não conta como vitória humana.

## Como chegar

```bash
cargo run
```

Janela 1280×720. O jogo abre no Gate Approach.

1. Direita: Broken Causeway (há um sentinela de osso) e Watch Shaft.
2. Subir até a Upper Gallery e pegar a Relic of the Veil (lado esquerdo
   da sala).
3. Voltar ao Gate Approach.
4. `T` para a Forma Espectral e atravessar a porta da esquerda (véu).
5. Sealed Hall, porta esquerda: Gate Reliquary.
6. Subir as cinco plataformas com `Z` (pulo parado alcança o próximo
   degrau) e entrar pela porta do teto.
7. Cair de pé na plataforma da Ashen Court. O sentinela está à direita,
   fora da tela nesse ponto. Andar para a direita até ele entrar no quadro.

O probe não faz esse caminho: ele pede a sala do Reliquary e só sobe a
escada com Z.

## Na luta

- `X` ataque, `A` segurado guarda, `S` esquiva, `Z` pulo.
- Cores: laranja slash, amarelo lunge, ciano ash wave, roxo "Phase shift".
  O rótulo da barra repete o padrão. A cor deve continuar visível quando
  o número de dano aparece.
- `I` abre o inventário e pausa a luta. Oito linhas de equipamento, depois
  a bolsa. Descer até Minor Tonic e `Enter` (cura 40, recusa se a vida
  já está cheia). `I` ou `Esc` fecha. Há 3 tônicos e 1 éter.
- Se cair: "Press R to rise". `R` enche HP/MP no spawn da sala (a
  plataforma de chegada). O chefe some. Para lutar de novo, andar para a
  esquerda e cair no vão (entre a plataforma e o chão da esquerda,
  x 540–600), voltar ao Reliquary, subir outra vez e entrar.

## Ficha

Preencher depois de jogar. Enquanto estiver em branco, o status continua
pendente.

| Pergunta | Resposta |
|---|---|
| A escada sobe só com Z, sem ficar presa? | |
| A chegada fica de pé, sem cair de volta? | |
| O sentinela entra no quadro quando você anda para a direita? | |
| Dá para distinguir slash, lunge, ash wave e a troca de fase? | |
| A cor do golpe continua visível quando você acerta? | |
| Guarda e esquiva cabem no aviso? | |
| Usou tônico? Quantos? | |
| Venceu, ou morreu em qual fase e com quanto HP no chefe? | |
| `R`, a descida e a segunda subida funcionam? | |
| `sentinel_brand` entrou na bolsa uma vez só? | |
| O que atrapalhou | |
