# Erro-Corretion 01 — Verificar rig, referências e reaproveitamento

**Status:** CONCLUÍDO COM RESSALVAS
**Aprovação artística:** não se aplica. Foi uma inspeção técnica.

> **Relatório recuperado em 2026-11-21.** A etapa 01 **não tem relatório próprio**: não há
> `RELATORIO.md` em `evidence/erro_corretion/etapa01/`. Este documento foi montado com:
> - as evidências versionadas;
> - a mensagem do commit `ec8bdac`;
> - o resumo que o proprietário registrou em `Md-usario/Erro-Corretion/00_LEIA_PRIMEIRO.md`.
>
> **Informação incompleta.** O status "com ressalvas" vem do `00_LEIA_PRIMEIRO.md`
> ("reaproveitável **com ajustes**"). Os itens sem fonte estão marcados como "Não registrado".

## 1. Identificação
- Etapa: Erro-Corretion 01, verificação do rig e das referências.
- Branch: `art/v1.2-protagonist`.

## 2. Data e hora
Não registrada. As evidências foram versionadas no commit `ec8bdac` em 2026-11-19 17:51 (-0300).

## 3. Objetivo original
Verificar se o rig `swordsman_rig_v007` do Blender e as referências aprovadas podem ser
reaproveitados como manequim técnico para equipamentos sobre o protagonista vestido.

## 4. Prompt executado
`Md-usario/Erro-Corretion/01_VERIFICAR_RIG_E_REFERENCIAS.md` (arquivo pessoal do proprietário).

## 5. Contexto técnico inicial
O pintor procedural da N41-B foi reprovado visualmente. A base visual aprovada é o
protagonista N3.5/N3.6.

## 6. Ferramentas e versões
Blender 5.2.4, segundo o `00_LEIA_PRIMEIRO.md`. As demais não foram registradas.

## 7. Arquivos consultados
Não registrado. Pelos nomes das evidências: `swordsman_rig_v007` (ações `GAME_*` das
versões 1.3, 1.4 e 1.5), o sprite idle e o `run6`.

## 8. Arquivos criados (commit `ec8bdac`, 10 arquivos)
- `evidence/erro_corretion/etapa01/v007_inspect.txt`: inventário do rig (ossos, câmeras, ações).
- `evidence/erro_corretion/etapa01/joints_swordsman_rig_v007_animations.txt`
- `evidence/erro_corretion/etapa01/joints_swordsman_rig_v007_game_v1_3.txt`
- `evidence/erro_corretion/etapa01/joints_swordsman_rig_v007_game_v1_4.txt`
- `evidence/erro_corretion/etapa01/joints_swordsman_rig_v007_game_v1_5.txt`
- `evidence/erro_corretion/etapa01/match.json`: melhores quadros `GAME_run` por desenho.
- `evidence/erro_corretion/etapa01/rig_vs_sprite.png`
- `evidence/erro_corretion/etapa01/idle_layers.png`
- `evidence/erro_corretion/etapa01/idle_sleeve.png`
- `evidence/erro_corretion/etapa01/run6.png`

## 9–10. Arquivos modificados e removidos
Nenhum, segundo o commit.

## 11. Descrição das alterações
Só inspeção. Foram registradas as articulações projetadas das ações do rig e as primeiras
folhas comparando o rig com o sprite e a manga do idle.

## 12. Decisões
Segundo o `00_LEIA_PRIMEIRO.md`: o rig serve **só como manequim técnico invisível**. Ele não
é sprite final e o corpo dele não é renderizado.

## 13. Comandos
Não registrados.

## 14–15. Testes e resultado
Não registrados como testes formais.

## 16. Métricas
- O rig tem 43 ossos (`00_LEIA_PRIMEIRO.md`).
- `match.json` traz as distâncias de encaixe por desenho da corrida. Não há relatório que as
  interprete.

## 17–19. Problemas
- **Encontrados:** a manga do idle está dentro da camada `shirt`; nos quadros da corrida, o
  cabelo foi classificado como braço. Esses defeitos foram registrados no `00_LEIA_PRIMEIRO.md`
  e tratados na 02B.
- **Corrigidos nesta etapa:** nenhum.
- **Pendentes:** foram levados para as etapas 02 e 02B.

## 20. Limitações
O relatório original está ausente.

## 21–22. Evidências visuais
- `evidence/erro_corretion/etapa01/rig_vs_sprite.png`: rig contra sprite.
- `evidence/erro_corretion/etapa01/idle_layers.png`: camadas do idle.
- `evidence/erro_corretion/etapa01/idle_sleeve.png`: manga do idle.
- `evidence/erro_corretion/etapa01/run6.png`: os 6 desenhos da corrida.

## 23. Links do GitHub
- Pasta: https://github.com/RaulDuque/ProjetoRST/tree/art/v1.2-protagonist/evidence/erro_corretion/etapa01
- Commit: https://github.com/RaulDuque/ProjetoRST/commit/ec8bdac

## 24. Resultado final
Rig reaproveitável **com ajustes**.

## 25. Recomendação seguinte
Etapa 02, a prova de encaixe nas quatro poses. Já foi executada.

## 26. Decisões do proprietário
Já tomada: seguir para a etapa 02.

## 27. Git
Commit `ec8bdac` na branch `art/v1.2-protagonist`, já publicado.
