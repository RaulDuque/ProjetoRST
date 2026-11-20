# Erro-Corretion — Etapa 03D: nova armadura com o Agent Sprite Forge

**Estado: BLOQUEADO.** Não existe rota real de geração de imagens nesta máquina.

- **Imagem mestre:** não foi gerada.
- **Prévia em pixel art e comparação:** não existem, porque dependem da imagem mestre.
- **Sem substitutos:** nada foi inventado ou desenhado por código.
- **Pagamento:** nenhuma chamada paga foi feita e nenhuma credencial foi configurada.

## Diagnóstico (Agent Sprite Forge 0.6.0, data 2026-11-19)

| Verificação | Resultado | Arquivo |
|---|---|---|
| `forge_doctor.py --host-tools none` | Chaves openai, gemini, xai, byteplus e fal: **nenhuma**. Imagem, edição e vídeo: **none**. Ffmpeg pronto. | `out/doctor_routes.txt`, `out/doctor_routes.json` |
| `route_media.py resolve --kind image` e `--kind video` | `"status":"no-route"`, com a única alternativa sendo o `codeart2d` (arte desenhada por código, **recusada pelo pedido**) | `out/route_resolve.txt` |
| CLIs locais (`codex`, `grok`, `gemini`) | Nenhum instalado | `out/cli_check.txt` |
| `~/.config/agent-sprite-forge/config.json` e variáveis de ambiente | Não existem | — |

O Claude Code desta sessão também não tem uma ferramenta própria de geração de imagens (`image_gen`). Por isso `--host-tools none` é o valor correto.

## O que ficou pronto (offline, sem custo)

O pedido já está preparado pela rota recomendada para preservar a identidade: `pixel_look.py ask-pose`.

- **Por que essa rota:** o sprite N3.5 já é um master limpo em 1×, então não precisa de `ask-master`. O modelo recebe o personagem aprovado sobre uma grade exata, com **uma única mudança** descrita: a armadura.
- **O que acontece depois:** a etapa `pose` aproveita só as células que mudaram. Rosto, cabelo, calça e botas continuam sendo os pixels do original.

| Arquivo | Conteúdo |
|---|---|
| `request_pose/reference.png` | `candidate_106.png` sobre verde (#00FF00), 9 px por pixel, 4 px de folga nas laterais (para as ombreiras) e 8 px em cima |
| `request_pose/prompt.txt` | O prompt completo em inglês: peitoral roxo esmaltado com quilha, faixas douradas largas e planas, ombreiras angulares em placas (não esféricas), braçais com bordas douradas, cintura com escarcelas. Proíbe frisos finos e brilho de plástico. Mantém o resto pixel por pixel. |
| `request_pose/request.json`, `request_pose/master.png` | Registro do pedido e o master usado |

Não usei nenhuma armadura das etapas N41-B, 03 ou 03B como referência.

## Para desbloquear (decisão sua)

Escolha **uma** rota e autorize de forma explícita:

1. **API com chave**, por exemplo OpenAI (GPT Image) ou Gemini, que fazem edição com referência.
   - Você coloca a chave em `~/.config/agent-sprite-forge/config.json` ou no ambiente. Eu não configuro credenciais.
   - **Atenção:** com a chave presente, o ASF chama a API sem perguntar a cada vez.
   - Para controlar o gasto, eu rodaria primeiro `route_media.py ... --dry-run`, que mostra provedor, modelo e o `estimateUsd` sem enviar nada. Depois rodaria com `--max-calls 1` (e `--budget-usd`, se houver preço verificado).
2. **CLI local logado:** Codex (`image_gen`) ou Grok. Usa a cota da sua assinatura, não uma chave de API.

Com a rota pronta, os passos são:
1. Gerar a imagem com `route_media.py image --prompt-file request_pose/prompt.txt --reference request_pose/reference.png --size 1024x1024` (**uma** imagem).
2. Ler o resultado com `pixel_look.py pose`.
3. Montar a prévia 1× e a comparação lado a lado com o original.
4. Parar e esperar a sua aprovação.

Nenhuma animação será gerada antes disso.

## Atualização: rota externa (imagem do ChatGPT), sem API

O proprietário vai gerar a imagem fora do projeto, no ChatGPT, e enviá-la para cá. O
Agent Sprite Forge **não gera nada** nesta rota: ele só lê a imagem recebida.

- **Onde colocar a imagem:** `evidence/erro_corretion/etapa03d/inbox/armour_master.png`
  (instruções em `inbox/LEIA.md`).
- **O que mandar ao ChatGPT:** `request_pose/reference.png` mais o texto de `request_pose/prompt.txt`.
- **Processamento:**

  ```bash
  python3 -I -B evidence/erro_corretion/etapa03d/scripts/import_external.py \
    --image evidence/erro_corretion/etapa03d/inbox/armour_master.png --run take01
  ```

  - **1. Leitura da imagem:** `pixel_look.py pose` (generate2dsprite) lê a imagem
    sobre a grade do próprio sprite N3.5 (célula de 9 px, master na célula
    (45, 5)). Só as células que mudaram são aproveitadas; o resto continua sendo o
    pixel aprovado.
  - **2. Posicionamento:** o resultado é colocado no canvas 80×128 (pivô 40, 118).
  - **3. Medição:** o script mede os texels alterados por material da grade N3.4:
    cabelo, rosto, calça, couro e botas (que devem ficar intactos), além de pele,
    camisa e contorno. Também conta a silhueta nova e as cores novas.
  - **4. Saídas, em `runs/<take>/`:**
    - `comparison.png`;
    - `preview_1x_2x.png`;
    - `armour_on_canvas_80x128.png`;
    - `metrics.json`;
    - uma cópia da imagem-fonte e o log.
- **Autoteste da montagem, que não é arte:**
  - **Imagem sem nenhuma mudança:** `runs/selftest_unchanged_reference/` usa a
    própria `reference.png`, sem alterações. O `pose` recusa corretamente com
    "the picture shows no change".
  - **Posicionamento:** recolocar o master no canvas reproduz o sprite aprovado
    sem nenhuma diferença.
- **Limite conhecido:** se a imagem do ChatGPT não mantiver a grade, o `pose` pode
  recusá-la ou aproveitar pouco. Nesse caso o recomendado é pedir uma nova versão.
  Não haverá conserto à mão nem desenho por código.
