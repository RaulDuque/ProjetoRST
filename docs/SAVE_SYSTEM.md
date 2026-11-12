# Salvamento

Código: `src/save/serialization.rs` (formato), `save_manager.rs` (disco, std),
`systems.rs` (F5/F9). Testes: `tests/save_tests.rs`.

## O que é salvo (`SaveData`, versão 1)

- Jogador: sala, posição, atributos base, progressão (nível, EXP, pontos), HP/MP/Hearts,
  habilidades e magias aprendidas (com rank e comando revelado), inventário e equipamentos.
- Mundo (`WorldProgress`): portas destrancadas, baús abertos, pontos de save
  descobertos, chefes derrotados, flags de história, último ponto de save.
- Tempo de jogo.

Arquivo: `saves/slot1.ron` na raiz do projeto (ignorado pelo git).

## Versionamento

O documento começa com `version`. Ao carregar:
1. lê só a versão;
2. versão maior que `CURRENT_SAVE_VERSION` → `TooNew` (o arquivo não é tocado);
3. menor que `MIN_SUPPORTED_SAVE_VERSION` → `TooOld`;
4. senão `migrate(version, texto)` converte passo a passo até a versão atual.

Ao mudar `SaveData` de forma incompatível: incremente `CURRENT_SAVE_VERSION`,
mantenha a struct antiga e adicione um braço em `migrate`. Campos novos opcionais
com `#[serde(default)]` não exigem nova versão.

## Segurança dos dados

- Escrita: `slotN.ron.tmp` → cópia do save anterior para `slotN.ron.bak` → `rename`.
  Uma queda no meio nunca deixa o save corrompido nem apaga o anterior.
- Arquivo corrompido → `Corrupted` com a mensagem do parser; o arquivo e o backup ficam intactos.
- Atualizações do jogo nunca sobrescrevem saves automaticamente.

## Controles

F5 salva (não permitido enquanto caído). F9 carrega o slot 1 e retoma o jogo
(funciona também na tela de status e após a morte).
