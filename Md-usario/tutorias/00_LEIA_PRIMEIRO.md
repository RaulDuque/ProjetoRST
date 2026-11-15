# 00 — LEIA PRIMEIRO

Bem-vindo! Estes tutoriais ensinam a criar conteúdo do **Dark Castle RPG**
(salas, regiões, itens, armas, baús, segredos, inimigos, NPCs, lojas e
cidades) **sem precisar saber programar**. Você vai editar arquivos de texto
e rodar alguns comandos no terminal.

> **Onde estão estes arquivos?** O lugar definitivo é `Md-usario/tutorias/`.
> Enquanto a permissão de escrita nessa pasta não for liberada, eles ficam em
> `docs/tutorias/` (mesmo conteúdo, mesmos nomes).

## 1. O que vamos fazer?

Entender como o projeto está organizado, como abrir o jogo, como testar o
que você mudou e em que ordem ler os tutoriais.

## 2. O que você precisa saber antes?

| Palavra | O que é, em uma frase |
|---|---|
| **Rust** | A linguagem em que o jogo é programado (arquivos `.rs` em `src/`). Você **não** precisa mexer neles. |
| **Bevy** | O "motor" do jogo, escrito em Rust: desenha a tela, lê o teclado, roda a física. |
| **RON** | O formato dos arquivos de **dados** (`.ron`): salas, itens, inimigos, NPCs, lojas. É neles que você vai trabalhar. |
| **Cargo** | O programa que compila e roda o jogo (`cargo run`, `cargo test`). |
| **ID** | O nome interno de algo (`"iron_sword"`). Sem espaços, em minúsculas, com `_`. Nunca repita um ID. |

Como ler um arquivo RON:

```text
(                         <- abre um "registro"
    id: "iron_sword",     <- campo: valor, e uma vírgula no fim
    name: "Iron Sword",   <- textos sempre entre aspas
    modifiers: (attack: 8),
    tags: ["a", "b"],     <- listas usam colchetes
)                         <- fecha o registro
```

- Linhas que começam com `//` são **comentários**: o jogo ignora.
- Números com ponto (`3.0`) são decimais; sem ponto (`3`) são inteiros. Use
  o mesmo tipo dos exemplos.
- Palavras sem aspas (`Sword`, `Left`, `Common`) são **opções fixas**: escreva
  exatamente como no exemplo, com a mesma letra maiúscula.

## 3. Quais arquivos vamos utilizar?

| Pasta | O que tem |
|---|---|
| `assets/world/world.ron` | Onde o jogo começa, a sala final e as passagens espectrais |
| `assets/world/regions/` | Uma região por arquivo, com as salas dela |
| `assets/data/items/` | Itens, armas, armaduras, raridades, classes de arma |
| `assets/data/enemies/` | Tipos de inimigo |
| `assets/data/npcs/` | Personagens não jogáveis (NPCs) |
| `assets/data/dialogues/` | Falas dos NPCs |
| `assets/data/shops/` | Lojas e mercadorias |
| `assets/data/character.ron` | Personagem inicial (atributos, itens, moedas) |
| `assets/sprites/`, `assets/animations/` | **Arte publicada. Não edite à mão.** |
| `saves/` | Seus jogos salvos. **Não edite nem apague.** |
| `src/` | Código Rust. Os tutoriais não pedem para mexer aqui. |
| `docs/` | Documentação técnica (mais detalhada). |
| `target/` | Arquivos gerados pela compilação e capturas de tela. |

## 4. Passo a passo

1. **Abra um terminal na pasta do projeto** (a que tem o arquivo `CLAUDE.md`).
2. Se o comando `cargo` não for encontrado, rode antes: `source "$HOME/.cargo/env"`.
3. **Inicie o jogo:** `cargo run -j 2`
   - `-j 2` limita a compilação a 2 tarefas por vez. **Use sempre**: este
     computador já travou por falta de memória com mais tarefas.
   - Rode **um comando `cargo` por vez**.
4. **Teclas do jogo:** setas (mover), Z (pular), X (atacar), C (magia),
   A (defender), S (esquivar), D (usar / conversar), T (Forma Espectral),
   I (equipamento), M (mostra/esconde o mapa), Tab (status), F5 (salvar),
   F9 (carregar).
5. **Depois de mudar um arquivo**, teste (tutorial 16):
   - `cargo run -j 2 --example world_report` → confere o mundo sem abrir o jogo;
   - `cargo test -j 2 --test data_tests` → confere os dados;
   - `cargo run -j 2` → joga e olha.
6. **Ordem recomendada:** 01 → 02 → 03 → 04 → 08 → 05 → 06 → 07 → 09 → 10 → 11
   → 12 → 13 → 14 → 15 → 16 → 17, depois 18 → 19 → 20 → 21 → 22 → 23.

## 5. Exemplo completo

Todos os tutoriais usam o mesmo exemplo, a região **Ashen Crypt** (Cripta
Cinzenta), ligada à casa da Old Wenna na Vila das Cinzas:

```
  y=1  ... [av_smithy] - [av_cottage] -> [ac_crypt_stair] -> [ac_bone_ossuary] ~parede~> [ac_secret_niche]
                                           alavanca, NPC,      relíquia, baú,             baús secretos
                                           inimigo             parede quebrável
```

O teste `tests/tutorial_examples_tests.rs` lê os blocos marcados com
`// exemplo-verificado:` destes tutoriais, monta essa região **numa cópia do
mundo** (sem tocar em `assets/`), roda o validador e percorre as salas
apertando teclas de verdade. Se um exemplo ficar errado, o teste falha.

## 6. Como testar?

```bash
cargo test -j 2 --test tutorial_examples_tests
```

Resultado esperado: `test result: ok.`

## 7. Como editar depois?

Cada tutorial tem esta seção. Regra geral: mude o valor no arquivo `.ron`,
salve e rode os testes do passo 5.

## 8. Problemas comuns

Veja o tutorial **17 — Erros comuns**.

## 9. Como desfazer com segurança?

- **Antes de editar um arquivo que já existe, faça uma cópia:**
  `cp assets/world/regions/vigil_gates.ron ~/copia_vigil_gates.ron`
  Para desfazer, copie de volta:
  `cp ~/copia_vigil_gates.ron assets/world/regions/vigil_gates.ron`
- **Arquivo novo que você criou:** basta apagá-lo.
- **Não use** `git restore`, `git checkout -- .`, `git reset --hard` nem
  `git clean`: muitos arquivos do projeto têm trabalho **ainda não salvo no
  Git**, e esses comandos o apagariam para sempre.
- Para ver o que você mudou num arquivo: `git diff caminho/do/arquivo.ron`.

## Lista de tutoriais

| Nº | Tutorial | Situação |
|---|---|---|
| 01 | [Criar salas](01_CRIAR_SALAS.md) | Funciona |
| 02 | [Editar salas](02_EDITAR_SALAS.md) | Funciona |
| 03 | [Criar regiões e fases](03_CRIAR_REGIOES_E_FASES.md) | Funciona |
| 04 | [Conectar salas e portas](04_CONECTAR_SALAS_E_PORTAS.md) | Funciona |
| 05 | [Criar itens](05_CRIAR_ITENS.md) | Funciona (usar poções: **pendente**) |
| 06 | [Criar armas](06_CRIAR_ARMAS.md) | Funciona |
| 07 | [Criar armaduras e acessórios](07_CRIAR_ARMADURAS_E_ACESSORIOS.md) | Funciona |
| 08 | [Criar baús](08_CRIAR_BAUS.md) | Funciona |
| 09 | [Criar salas secretas](09_CRIAR_SALAS_SECRETAS.md) | Funciona |
| 10 | [Criar paredes quebráveis](10_CRIAR_PAREDES_QUEBRAVEIS.md) | Funciona |
| 11 | [Criar alavancas e mecanismos](11_CRIAR_ALAVANCAS_E_MECANISMOS.md) | Funciona |
| 12 | [Criar relíquias](12_CRIAR_RELIQUIAS.md) | Funciona (habilidades novas: **pendente**) |
| 13 | [Criar barreiras espectrais](13_CRIAR_BARREIRAS_ESPECTRAIS.md) | Funciona |
| 14 | [Adicionar inimigos](14_ADICIONAR_INIMIGOS.md) | Funciona (arte de inimigo: **pendente**) |
| 15 | [Editar mapa e exploração](15_EDITAR_MAPA_E_EXPLORACAO.md) | Funciona |
| 16 | [Testar conteúdo](16_TESTAR_CONTEUDO.md) | Funciona |
| 17 | [Erros comuns](17_ERROS_COMUNS.md) | Referência |
| 18 | [Criar NPCs](18_CRIAR_NPCS.md) | Funciona (arte de NPC: **pendente**) |
| 19 | [Criar diálogos](19_CRIAR_DIALOGOS.md) | Funciona |
| 20 | [Criar lojas](20_CRIAR_LOJAS.md) | Funciona (dinheiro e estoque são salvos) |
| 21 | [Cadastrar mercadorias](21_CADASTRAR_MERCADORIAS.md) | Funciona |
| 22 | [Configurar preços](22_CONFIGURAR_PRECOS.md) | Funciona |
| 23 | [Criar cidade](23_CRIAR_CIDADE.md) | Funciona |
