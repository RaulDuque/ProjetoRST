# Índice de relatórios

Ordem cronológica, do mais antigo para o mais novo. Novas linhas entram **no fim**. Datas
marcadas com "(commit)" são a hora do commit, porque a hora da execução não foi registrada.

| # | Data | Identificação | Status | Relatório | Resumo |
|---|---|---|---|---|---|
| 1 | 2026-11-19 17:51 (commit) | Erro-Corretion 01 | CONCLUÍDO COM RESSALVAS | [01_VERIFICAR_RIG](erro-corretion/01_VERIFICAR_RIG.md) | Rig v007 reaproveitável com ajustes. **Recuperado sem relatório original.** |
| 2 | 2026-11-19 17:52 (commit) | Erro-Corretion 02 | CONCLUÍDO COM RESSALVAS | [02_ENCAIXE_POSES](erro-corretion/02_ENCAIXE_POSES.md) | Encaixe nas 4 poses; "PRECISA DE AJUSTE" em ombros e pernas |
| 3 | 2026-11-19 18:20 (commit) | Erro-Corretion 02B | CONCLUÍDO | [02B_MANEQUIM_CAMADAS](erro-corretion/02B_MANEQUIM_CAMADAS.md) | Clavículas corrigidas e camadas vestidas; reconstrução com 0 px divergentes |
| 4 | 2026-11-19 18:46 (commit) | Erro-Corretion 03 | CONCLUÍDO COM RESSALVAS | [03_ARMADURA_PILOTO](erro-corretion/03_ARMADURA_PILOTO.md) | Piloto 3D de armadura gótica; aprovado como piloto técnico |
| 5 | 2026-11-19 22:52 (commit) | Erro-Corretion 03B | AGUARDANDO APROVAÇÃO VISUAL | [03B_REFINAMENTO](erro-corretion/03B_REFINAMENTO.md) | Armadura refinada e prova de redução; ombreira ainda lê como medalhão no #0 e #2 |
| 6 | 2026-11-19 23:14 (commit) | Erro-Corretion 03C | CONCLUÍDO | [03C_NOVAS_SKILLS](erro-corretion/03C_NOVAS_SKILLS.md) | ASF e PixelFX avaliados offline. **Prompt original não versionado.** |
| 7 | 2026-11-20 00:21 (commit) | Erro-Corretion 03D Take 01 | CONCLUÍDO COM RESSALVAS | [03D_TAKE01](erro-corretion/03D_TAKE01.md) | Bloqueio inicial, depois imagem do ChatGPT lida pelo `pose`; dourado fino e quilha ilegível em 1×; arte não aprovada |
| 8 | 2026-11-21 14:41 | Erro-Corretion 03D Take 02 | REJEITADO | [03D_TAKE02](erro-corretion/03D_TAKE02.md) | Usou a mesma imagem do Take 01 (resultado idêntico, 0 px). Não vale como segunda tentativa artística |
| 9 | 2026-11-21 15:10 | Processo: estrutura de relatórios | CONCLUÍDO COM RESSALVAS | [2026-11-21_ESTRUTURA_RELATORIOS](processo/2026-11-21_ESTRUTURA_RELATORIOS.md) | Criação de `docs/relatorios/`, regra permanente e histórico; prompts de `Md-usario/` dependem do proprietário |
| 10 | 2026-11-21 15:30 | Erro-Corretion 03D Take 03, pedido | CONCLUÍDO | [03D_TAKE03_PEDIDO](erro-corretion/03D_TAKE03_PEDIDO.md) | Pedido `ask-pose` do Take 03 (pixels inteiros, ombreira de 2 placas, dourado só em faixas grossas) e prompt para o Claude; aguarda a imagem do ChatGPT |
| 11 | 2026-11-21 16:00 | Erro-Corretion 03D Take 03 | AGUARDANDO APROVAÇÃO VISUAL | [03D_TAKE03](erro-corretion/03D_TAKE03.md) | Imagem nova lida pelo `pose`; mais limpa e legível que o Take 01, sem regressão, mas lê como túnica (ombreira sem volume, dourado parte com 1 px). Avaliação: melhor, mas insuficiente |
| 12 | 2026-11-21 16:05 | Modular Character Engine 01 | AGUARDANDO APROVAÇÃO VISUAL | [01_PROTOTIPO](modular-character-engine/01_PROTOTIPO.md) | Rig 2D em Rust/Bevy: esqueleto marcado nos quadros aprovados, camadas sem perda, espada e ombreira presas a ossos, troca e espelho sem reiniciar; a articulação completa fica bem abaixo da arte, então a proposta é a abordagem híbrida |

## Correções registradas

Nenhuma correção em relatórios desta pasta até agora.

Fora desta pasta, o relatório do Take 01 recebeu em `4feec7f` a seção "Correção (Take 02)",
que explica a contagem "43" do cabelo e a restauração N4.
