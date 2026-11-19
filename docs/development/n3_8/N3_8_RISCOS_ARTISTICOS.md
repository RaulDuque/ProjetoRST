# N3.8 — Riscos artísticos e corpo anatômico neutro

O proprietário decidiu que **toda** parte visual será removível: cabelo, camisa, calça, botas, luvas, braceletes e todo equipamento. Isso exige um corpo anatômico neutro **que hoje não existe em nenhuma imagem**. Apagar pixels de roupa não o produz.

## O que as referências têm (medido nos grids de paleta, `analysis/regions.json`)

| Região | Visível hoje (N3.5 / N3.6) | Oculta | Informação existente | Exige arte nova? | Risco |
|---|---|---|---|---|---|
| Rosto, perfil | sim | — | rosto completo em perfil | não | baixo |
| Cabeça atrás do cabelo (crânio, orelha, nuca) | não | coberta pelo cabelo (207 texels no idle, 231–318 na corrida) | contorno da testa e da franja | **sim** | **alto**: o volume do cabelo define a silhueta da cabeça |
| Pescoço | parcial | gola | lateral do pescoço | sim (base) | médio |
| Antebraços e mãos | sim (mangas arregaçadas) | — | pele, punhos fechados | recuperável | baixo a médio (braceletes cortam o antebraço) |
| Braços e ombros | não | manga da camisa | contorno da manga | **sim** | médio |
| Torso | não | camisa | dobras, volume do peito | **sim** | **alto**: as dobras da camisa são o modelado do torso |
| Quadril | não | cinto e calça | contorno | sim | médio |
| Pernas | não | calça | silhueta da calça (folgada) | **sim** | **alto**: a perna nua é mais fina que a calça, e a silhueta da corrida muda |
| Pés | não | botas altas até a canela | contorno da bota | **sim** | médio: pé descalço é menor; o contato com o chão muda de lugar |
| Pulsos | não | braceletes | — | sim (pequeno) | baixo |

**Recuperáveis com segurança:** rosto, antebraços, punhos e mãos (a pele já está desenhada). **Arte nova obrigatória:** cabeça atrás do cabelo, ombros e braços, torso, quadril, pernas e pés. **Maior risco de perder qualidade:** torso e pernas (o modelado vem das roupas), cabeça (o cabelo é a silhueta) e o movimento da corrida (a calça e as botas carregam o balanço das pernas).

## Respostas pedidas (§18 do briefing)

1. **Separar o cabelo sem destruir as mechas?** Sim, desde que o cabelo seja **redesenhado como camada** sobre a fonte, e não recortado. Recortar perde as mechas que passam atrás do tronco e do braço. As mechas que cruzam o corpo exigem duas fatias (cabelo traseiro e frontal).
2. **Cabeça atrás do cabelo:** desenhar um crânio neutro por quadro (6 + idle) a partir do contorno da testa e da proporção do rosto, validando com o cabelo recomposto por cima. Referência artística nova é necessária (perfil sem cabelo).
3. **Torso atrás da camisa:** pintura nova do torso nu nas 7 poses, mantendo a silhueta dos ombros. A camisa passa a ser camada redesenhada sobre o torso.
4. **Pernas atrás da calça:** idem. A calça folgada vira camada; a perna nua é desenhada mais fina, sem mudar o pivô nem o contato do pé.
5. **Pés atrás das botas:** pé descalço desenhado por quadro. Os pés de apoio precisam continuar na linha da sola (118).
6. **Braços e mangas:** braço nu completo por quadro, manga como camada de roupa. Os antebraços já existem e servem de base.
7. **Alinhamento dos seis quadros:** todas as camadas no mesmo canvas 112×128, desenhadas sobre o quadro aprovado como "papel vegetal". Nenhum deslocamento por camada.
8. **Mesmo pivô:** consequência de 7. O teste da N4 compara o pivô de todas as folhas.
9. **Capa atrás do cabelo:** ordem capa traseira (z 0) < cabelo traseiro (z 1). Mechas longas podem passar por cima da gola; se a capa tiver gola alta, ela entra como capa frontal (z 10).
10. **Espada presa à mão:** soquete `weapon` medido em cada quadro, com o quadro mostrado (já é assim no V2.2).
11. **Roupas volumosas:** peça com silhueta própria = arte específica por quadro; não cabe em "fatias padrão". Limite registrado em `N3_8_ARQUITETURA_TERRARIA.md` §6.
12. **Ocultação de regiões:** ocultação de camada + variantes; máscaras só se necessário, aplicadas offline.
13. **Evitar folhas por combinação:** uma folha por peça e clipe, compostas em tempo real.
14. **Desempenho:** ~10 sprites com o mesmo índice; custo de GPU desprezível para um personagem; medir na prova.
15. **Aparência cosmética:** slots de aparência separados do equipamento, persistidos só num save v4 autorizado.
16. **Referências artísticas novas necessárias:** perfil da cabeça sem cabelo; torso, braços e pernas sem roupa (modelo anatômico neutro, sem nudez explícita: roupa de baixo neutra como corpo-base); pé descalço; e cada peça de roupa isolada nas 7 poses. A `Ficha_de_Modelagem_do_Protagonista_Gotico.png` do proprietário (em `Md-usario/`, não lida nem alterada pela N3.8) pode ser a base.

## Regras de preservação

- Nenhuma composição modular substitui a N3.5/N3.6 sem uma **comparação lado a lado** com a original, quadro a quadro, aprovada pelo proprietário.
- Nada de manequim geométrico (N3.3) nem de blocos coloridos como resultado final.
- A corrida aprovada continua sendo a referência. Se a recomposição perder mechas, dobras ou balanço, prevalece a original.
