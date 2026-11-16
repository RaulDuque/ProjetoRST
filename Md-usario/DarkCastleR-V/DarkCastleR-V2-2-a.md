# DARK CASTLE RPG — AUTORIZAÇÃO CONTROLADA DA ARTE V2.2

Aprovo a solução técnica da V2.2 e autorizo a publicação da arte candidata do protagonista, exclusivamente nos arquivos identificados no relatório.

Esta autorização está condicionada às verificações abaixo.

## 1. Verificações antes da publicação

- Confirme a branch `art/v1.2-protagonist` e o HEAD `d33944c`.
- Confirme que o staging está vazio e que não existem alterações inesperadas nos arquivos de destino.
- Confirme que a candidata foi gerada corretamente e passou pelo `publish_guard.py check`.
- Confirme os hashes dos sprites atualmente publicados.
- Confirme que o backup poderá restaurar os arquivos anteriores.
- Confirme que o processo não modificará `Md-usario/`, `saves/`, configurações protegidas ou o PNG de inspiração.

Se alguma condição falhar, interrompa e apresente o problema.

## 2. Publicação autorizada

Publique somente:

- `player_locomotion.png`
- `player_combat.png`
- `player.ron`
- Lock correspondente
- Novo `player_sockets.ron`

Preserve `player_effects.png`.

Utilize o mecanismo oficial de publicação e seu backup automático.

Não altere arquivos fora desse escopo.

## 3. Validação após publicação

Execute:

1. `cargo fmt --all -- --check`
2. `cargo check -j 2`
3. `cargo clippy --all-targets -j 2 -- -D warnings`
4. `cargo test -j 2`
5. Testes Python do pipeline artístico.
6. Testes do hook de proteção.
7. `visual_check`.
8. `equipment_visual_showcase` com `--published`.
9. Rotas reais de exploração, equipamentos e Vila das Cinzas.

Atualize os testes que ainda esperam que a arte publicada não possua camadas de equipamentos. Não remova verificações legítimas para obter PASS.

## 4. Verificação visual obrigatória

Abra e inspecione as capturas da versão publicada.

Confirme:

- Sem arma: nenhuma espada fixa.
- Espada: apenas a espada equipada.
- Adaga: adaga visível.
- Machado: machado visível.
- Lança: lança visível.
- Arma pesada: arma pesada visível.
- Escudo: posição correta.
- Capa: acompanha o personagem.
- Ataques: arma acompanha a animação.
- Mudança de direção: equipamento acompanha o personagem.
- F5/F9: aparência restaurada corretamente.

Se a publicação provocar regressão, utilize o rollback oficial e informe a falha.

## 5. Encerramento

Não faça commit nem push.

Não inicie a V2.3.

Entregue o relatório final com:

- Hashes anteriores e novos.
- Backups criados.
- Arquivos publicados.
- Resultado de todos os testes.
- Capturas verificadas.
- Estado final do Git.
- Arquivos protegidos preservados.
- Tutoriais que preciso sincronizar.
- Limitações visuais ainda existentes.

Somente declare a V2.2 concluída quando a versão normal do jogo utilizar corretamente os novos sprites e todas as validações obrigatórias estiverem aprovadas.