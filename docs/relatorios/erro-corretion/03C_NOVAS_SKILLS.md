# Erro-Corretion 03C — Avaliação do Agent Sprite Forge e do PixelFX

**Status:** CONCLUÍDO
**Aprovação artística:** não se aplica, porque foi uma avaliação de ferramentas. Copiar
efeitos para `assets/` depende de aprovação.

> **Relatório recuperado em 2026-11-21.** Ele resume
> [`evidence/erro_corretion/etapa03c/RELATORIO.md`](../../../evidence/erro_corretion/etapa03c/RELATORIO.md).
> **Informação incompleta:** não existe arquivo de prompt da 03C em
> `Md-usario/Erro-Corretion/`, e o pedido original não está versionado. A data e hora da
> execução também não foram registradas.

## 1. Identificação
Erro-Corretion 03C, branch `art/v1.2-protagonist`.

## 2. Data e hora
Não registradas. A etapa foi versionada no commit `2470f72` em 2026-11-19 23:14 (-0300).

## 3. Objetivo
Instalar, validar e testar localmente o Agent Sprite Forge e o PixelFX, sem chamadas pagas
e sem alterar o jogo.

## 4. Prompt
Não versionado. Não há arquivo de prompt da 03C.

## 5. Contexto inicial
A 03B estava aguardando avaliação. Nenhuma rota de geração de imagem estava configurada.

## 6. Ferramentas
- Agent Sprite Forge 0.6.0 (`25203d0`);
- PixelFX 0.3.1 (`5294518`);
- Node 24;
- Python 3.11 em um ambiente isolado com numpy 2.4, Pillow 13, scipy 1.17 e ffmpeg 8.0 (`static-ffmpeg`).

## 7. Consultados
- O vídeo original da corrida;
- o idle N3.5, o `run6` e a N3.6;
- `assets/animations/player.ron` e `src/player/visual/vfx`, só leitura.

## 8. Criados (commit `2470f72`, 31 arquivos)
- `scripts/run_tests.sh`, `scripts/build_evidence.py` e `scripts/pixelfx_to_manifest.py`;
- `out/metrics.json` e `out/CREDITS.txt`;
- imagens e GIFs em `out/`.

Os plugins e o ambiente isolado foram instalados **fora** do projeto, em `~/.claude/plugins/`
e `~/.local/share/dc-art-tools/`.

## 9–10. Modificados e removidos
Nenhum no projeto.

## 11. Alterações
Só instalação e testes. Segundo o relatório: "Nada do jogo foi alterado".

## 12. Decisões
- O PixelFX é útil para efeitos.
- O ASF serve para medição e controle de loop.
- O finish do ASF não substitui a N3.6.

## 13. Comandos
Estão em `evidence/erro_corretion/etapa03c/scripts/run_tests.sh` e no relatório original.

## 14–15. Testes e resultado
- Testes do ASF: 299 de 299 passaram.
- `check` do PixelFX: 36 de 36 passaram.
- `doctor` do vídeo: OK.
- `forge_doctor`: **nenhuma rota de geração**.

## 16. Métricas
- **A1:** loop de 12 quadros, escala 0,2367 contra 0,2366.
- **A3:** ida e volta com 91,7 % a 94,8 % dos pixels idênticos.
- **A4:** grade reencontrada, mas com até 48 cores.
- **PixelFX:** de 4.883 cores para 7 e de 53.574 para 5 depois da conversão.

## 17–19. Problemas
- **Pendentes:**
  - o trecho RON não foi carregado no jogo;
  - não foi conferido se o jogo aceita um pivô fora da célula;
  - o hit-stop do PixelFX ainda não é uma regra em Rust.

## 20. Limitações
Sem rota paga, as funções de geração (`ask-master`, `pose` com geração, image-to-video) não
foram testadas.

## 21–22. Evidências
- `evidence/erro_corretion/etapa03c/out/asf_run_comparison.png` e `.gif`
- `evidence/erro_corretion/etapa03c/out/asf_gait_loop3x.gif`
- `evidence/erro_corretion/etapa03c/out/asf_roundtrip.png`
- `evidence/erro_corretion/etapa03c/out/asf_pixel_look.png`
- `evidence/erro_corretion/etapa03c/out/asf_idle_breath.gif`
- `evidence/erro_corretion/etapa03c/out/pixelfx_in_game_demo.png` e `.gif`
- `evidence/erro_corretion/etapa03c/out/pixelfx/`
- `evidence/erro_corretion/etapa03c/out/bevy_proposal/`

## 23. Links
- Relatório original: https://github.com/RaulDuque/ProjetoRST/blob/art/v1.2-protagonist/evidence/erro_corretion/etapa03c/RELATORIO.md
- Commit: https://github.com/RaulDuque/ProjetoRST/commit/2470f72

## 24. Resultado
- O PixelFX é recomendado para efeitos.
- O ASF serve como ferramenta de medição.
- Nenhum dos dois substitui a arte aprovada.

## 25. Recomendação
Usar o ASF com uma fonte real de imagem para testar uma armadura nova. Isso virou a 03D.

## 26. Decisões do proprietário
Integrar ou não os efeitos do PixelFX em `assets/`.

## 27. Git
Commit `2470f72`, já publicado.
