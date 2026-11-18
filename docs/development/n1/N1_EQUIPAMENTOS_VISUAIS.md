# N1 — Equipamentos visuais

Equipar muda o que está no slot e, quando o item tem representação, muda o desenho no mesmo frame. Não há segundo inventário. O item continua sendo `ItemDefinition` no catálogo que o save já guarda por id.

## O que a troca já faz

`WeaponForm` tem cinco famílias: Sword, Dagger, Axe, Spear, Heavy. A forma sai da categoria do item. A cor sai do `Trail`, ou de um padrão. Escudo usa `Tint` ou um marrom padrão. Arma de duas mãos ocupa as duas mãos; o escudo, se um save antigo ainda o tiver, vai para as costas em vez de sumir. A capa usa o visual do item de corpo.

Isso é silhueta gerada em código (`held_art.rs`), não a pixel art definitiva. Serve para provar a troca. Não serve de arte final.

## Decisão para centenas de itens

Um item novo não ganha uma folha. Ganha um registro opcional, com `#[serde(default)]`, para os RON velhos seguirem válidos:

- `visual_id`: qual peça dentro da família. Ausente significa a silhueta padrão da família.
- `animation_family`: uma das famílias já existentes, ou uma nova quando o movimento for outro. Espadas de uma mão compartilham o clipe. Espada de duas mãos, chicote, arco e cajado pedem família própria no dia em que o golpe for diferente. Até lá usam a família mais próxima e ficam marcados como exceção.
- `palette_id` ou a cor que o `Trail` já carrega. Recolorir uma folha é mais barato do que desenhar dez espadas.
- `attachment_point`: nome da âncora (`hand`, `back`, `off_hand`). Já é o modelo das sockets.
- `layer`: arma, escudo, capa, e no futuro elmo ou bota.
- `visible`: acessório sem desenho continua só número. A maioria dos anéis fica assim de propósito.

Dez armaduras, dez armas e dez capas são trinta peças e uma ordem de desenho, não mil folhas. Armadura volumosa que não cabe no pivô é exceção artística: uma variante de corpo, ainda um atlas, não um produto das outras peças.

Famílias planejadas além das cinco: espada de duas mãos (pode cair em Heavy), chicote, arco, cajado, escudo. Não se desenha uma animação por espada.

## Compatibilidade

Elmo some no agachamento se a âncora da cabeça descer com o quadro. Capa e arma de duas mãos não ocupam o mesmo osso das costas: a capa usa `back`, a arma de duas mãos usa as mãos, o escudo deslocado usa um ponto `slung` se um dia for preciso separar. Isso é extensão da tabela de âncoras, não um sistema novo.

## Alternativas

Campo solto de caminho de PNG dentro do item, sem família. Cada contribuidor inventa um layout e o manifesto deixa de validar.

## Riscos

Save v3 guarda o id do item, não a folha. Trocar o visual de um id muda a aparência de todo save que o possui. Isso é o desejado. Trocar o id quebra o save. Não renomear ids.

A silhueta em código e a folha pintada não podem ficar as duas ligadas. `held_layers_enabled` já desliga a camada quando o corpo antigo traz o sabre desenhado. A mesma chave vale para a arte nova.

## Recomendação

Na prova, duas armas e duas armaduras passam por esse registro, mesmo que o campo ainda não exista no RON. Se a troca funcionar com quatro peças, o campo entra na N5. Se precisar de uma folha por par arma-armadura, a arquitetura está errada e a N5 não começa.
