# N3.7 — Proveniência

Cada execução do pipeline grava `pipeline/<nome>/generated/provenance.json`.

## Campos

| Campo | Exemplo (`run_start`, quadro 13) |
|---|---|
| `pipeline_version` | `n3.7-pipeline/1` |
| `config` | `art-src/n3-7-run-sync/configs/run_start.json` |
| `source.file` | `Md-usario/assets-user/prota/base/trimmed_fd4c76cf-0416-4387-b451-da4cc90a2061.mp4` |
| `source.sha256` | `29cdbcb8594f94f38b7668919d18482af241a0f5fc3f449b7dfc96bec702cc57` |
| `source.width/height/fps/duration_s/codec` | 496, 864, 24, 7,917, H.264 |
| `source.decoded_frames` | 189 |
| `source.authorship` | vídeo gerado pelo proprietário com uma ferramenta de IA; não gerado pelo agente |
| `extraction` | decodificação GStreamer, PNG sem perda |
| `background_removal` | chave verde (margem 30), erosão de 1 px, limite de excesso de verde |
| `crop`, `scale`, `scale_rule` | (0, 150, 496, 702); 0,236607; "frame 0 opaque height 448 px → 106 texels" |
| `reduction` | BOX, alfa ≥ 128, paleta N3.4 sem M/e |
| `offset`, `offset_rule` | (−3, −5), herdado da corrida N3.6 |
| `canvas`, `pivot`, `density` | 112×128, (56, 118), 2 |
| `frames.<n>.video_index` / `timestamp_s` | 13 / 0,54167 |
| `frames.<n>.keyed_intermediate` | recorte em resolução cheia no cache (ignorado) |
| `frames.<n>.grid` / `opaque_box` | `pipeline/run_start/source/video_013.txt` / caixa opaca |
| `frames.<n>.excluded` | motivo, se o quadro foi descartado (`ghost`, `still`) |
| `frames.<n>.adjustments` | `[]`: nenhum ajuste por quadro |
| `frames.<n>.manual_edit` | `false` |
| `frames.<n>.agent_generated` | `false`: o agente extraiu, não gerou |
| `frames.<n>.source_ai_generated` | `true`: a fonte é um vídeo gerado por IA (pelo proprietário) |
| `clips` | quadros, durações, total e loop de cada clipe |
| `approval` | `NOT APPROVED (experimental)` |

## Rastreabilidade

- Todo quadro de um clipe leva ao índice e ao tempo no vídeo, e o vídeo ao hash. Rodar a mesma configuração refaz as mesmas grades, como mostra a reconstrução da N3.6.
- `tests/n3_7_sync_tests.rs::every_pipeline_frame_records_its_provenance` exige, para cada quadro extraído:
  - índice e tempo;
  - `manual_edit: false`, `agent_generated: false` e `source_ai_generated: true`;
  - a versão do pipeline, o hash da fonte e `NOT APPROVED`.
- Nenhuma fonte foi misturada. As duas configurações usam o mesmo vídeo. O N3.5 entra na transição como outra fonte, identificada pelo nome em todo lugar (HUD, GIF, documentos).

## Autoria

- O agente não pintou nem editou nenhum texel. As variantes B e C do contraste (`run6_rim.png`, `run6_boots.png`) são transformações automáticas por regra, descritas em `N3_7_CONTRASTE.md`, e não pintura.
- Nenhuma IA foi usada pelo agente para gerar ou interpolar quadros. Nenhum arquivo foi enviado a serviços externos.
