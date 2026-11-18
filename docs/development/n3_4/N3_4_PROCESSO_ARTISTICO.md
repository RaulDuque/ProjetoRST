# N3.4 — Processo artístico e autoria

## Resumo

O sprite **não foi pintado num programa gráfico**. O processo teve duas etapas:

1. **Redução automática** da vista lateral da ficha (`scripts/reduce_reference.py`). Ela separa a figura do fundo, reduz por média (BOX) para 106 px e mapeia cada pixel para a paleta. Resultado: `source/base_idle_draft.txt`. É um rascunho com halo nas bordas, rosto borrado e cinzas do cabelo confundidos com metal.
2. **Correção pixel a pixel pelo agente (Claude)**, digitada como texto. Usando o rascunho como decalque, cada linha do sprite foi escrita à mão como sequências de caracteres da paleta. Depois de cada passada, o resultado foi visto ampliado e comparado com a ficha. Resultado: `source/base_idle.txt`, a fonte editável.

`scripts/build_n3_4.py` só converte a grade em PNG e monta as evidências. Ele não desenha nada.

## Números

- Pixels diferentes entre rascunho e versão final: **1 248** (em `generated/metrics.json`).
- Passadas de revisão vistas em close: 4 na cabeça e 1 no corpo e nas botas.
- Horas de artista humano: **nenhuma**.
- Pintura no Pixelorama, Aseprite, Krita ou GIMP: **nenhuma**.

## O que foi decidido à mão

- O rosto inteiro (linhas 17–28), a mecha frontal e o agrupamento das mechas.
- O contorno azulado `H` do cabelo, no lugar do halo cinza da redução.
- As dobras da manga, o cordão do peito, o cinto e a fivela, o bracelete com rebite e a mão.
- O volume do glúteo e da panturrilha, a dobra da calça sobre o cano, a aba e as fivelas da bota, o bico arredondado e o arco entre salto e sola.
- `M` (metal) e `e` (olho) são reservados para colocação manual. A redução não pode usá-los.

## O que veio da redução

A posição geral das massas e as proporções, porque o decalque foi seguido. Isso é intencional: a fidelidade à ficha vem daí.

## Limites do método

Escrever pixels como texto é lento e não permite pintar com o pulso de um artista. As massas tendem a ficar em colunas verticais (calça e cano da bota), que é o defeito que restou mais visível. Um artista no Pixelorama, partindo de `base_idle.png`, resolveria isso mais depressa. Os arquivos estão prontos para isso: PNG 80×128 com transparência binária e paleta fixa.

## Como editar

Edite `source/base_idle.txt` (um caractere por pixel, `.` transparente, cores em `scripts/n34_palette.py`) e rode:

```bash
python3 art-src/n3-4-original-protagonist/scripts/build_n3_4.py
```

Se a arte for repintada num programa gráfico, o PNG passa a ser a fonte, e este documento precisa ser atualizado.
