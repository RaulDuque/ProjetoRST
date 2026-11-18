# N1 — Comparação de ferramentas

Consulta às fontes em 17–18 de novembro de 2026. Nenhuma ferramenta foi instalada. Nenhuma foi declarada compatível com o Bevy só porque o site é promissor.

| Ferramenta | Licença e fonte | Custo | No pipeline | Limite observado |
|---|---|---|---|---|
| Blender | GPL. Binários sob GPL-3 ou posterior. A arte criada é do autor. [blender.org/about/license](https://www.blender.org/about/license/) | grátis | já é a ferramenta de render do protagonista (`art-src/player/scripts/bpy/`) | script publicado que usa `bpy` fica sob licença compatível com a GPL se for distribuído |
| Pixelorama | MIT, Orama Interactive. [LICENSE](https://github.com/Orama-Interactive/Pixelorama/blob/HEAD/LICENSE). Exporta PNG, folha e tem linha de comando, segundo o README do projeto | grátis; a versão da Steam é paga e não é necessária | acabamento dos quadros e paleta | não foi executado aqui; a linha de comando está descrita pelo projeto, não testada nesta máquina |
| LibreSprite | GPL-2, fork do último Aseprite GPL. [LibreSprite](https://github.com/LibreSprite/LibreSprite) | grátis | reserva, se o Pixelorama falhar num quadro | conjunto parado no Aseprite de 2016; não herda as funções novas |
| Aseprite | EULA da Igara Studio. Binário pago, mínimo anunciado US$ 19,99. Código pode ser compilado para uso pessoal e não pode ser redistribuído. [FAQ](https://aseprite.org/faq/) | pago | não comprar sem autorização | a arte feita nele é do autor; o programa não entra no repositório |
| Spine | Editor Essential US$ 69 ou Professional US$ 379 por pessoa, na página de compra. Runtime exige licença. A lista oficial de runtimes não inclui Bevy. [compra](https://esotericsoftware.com/spine-purchase/), [runtimes](https://esotericsoftware.com/spine-runtimes) | pago | fora | sem runtime oficial Bevy, integração seria código nosso em cima de um formato alheio |
| Godot | não entra no produto | — | no máximo um visor auxiliar, se um dia for autorizado | migrar o jogo está fora |
| Mixamo | serviço da Adobe, sem assinatura extra, só conta Adobe. Bípedes. Não redistribuir o arquivo cru. Termos adicionais de 23 de junho de 2021 proíbem uso para treinar modelo. [FAQ](https://helpx.adobe.com/creative-cloud/faq/mixamo-faq.html) | grátis com conta | não nesta etapa | o `white_mesh.glb` não tem esqueleto nem pose neutra comprovada; o auto-rigger pede humanoide limpo, em pose neutra, sem peças soltas |
| Hunyuan3D 2.1 | licença comunitária da Tencent, com exclusão de União Europeia, Reino Unido e Coreia do Sul, e cláusula de usuários ativos. [LICENSE](https://github.com/Tencent-Hunyuan/Hunyuan3D-2.1/blob/main/LICENSE) | o peso é a licença, não o preço | não iniciar | o GLB atual foi escrito pelo `trimesh`, não por essa ferramenta |

## Decisão

O pipeline gratuito recomendado é o que o repositório já pratica, com um acabamento que ainda não está no fluxo: Blender para o manequim e o quadro, Pixelorama para corrigir pixel e exportar a folha, Bevy para compor as camadas. LibreSprite fica de reserva. Aseprite e Spine ficam de fora até o proprietário autorizar uma compra. Mixamo e Hunyuan3D ficam de fora desta fundação.

## Alternativas

Comprar Spine e animar por ossos 2D. A lista de runtimes publicada não cita Bevy, então o custo inclui a licença e um runtime que este repositório não tem.

## Riscos

Tratar o README do Pixelorama como teste. A exportação por linha de comando só vale depois de rodar uma vez, na prova, com autorização para instalar.

## Recomendação

Não instalar nada agora. A primeira instalação, se a N3 for aprovada, é só o Pixelorama, porque o Blender já é a ferramenta dos scripts existentes.
