# N3.8 — Arquitetura proposta: composição em camadas (referência Terraria)

**Tudo nesta página é PROPOSTA para a N4. Nada foi implementado na N3.8.** O que existe hoje está em `N3_8_COMPATIBILIDADE.md`.

Do Terraria usamos só os princípios de composição, não código nem arte: um personagem, um estado de animação compartilhado, várias regiões, peças independentes com folhas próprias, regras de sobreposição e ocultação, e aparência separada dos atributos.

## 1. Um controlador, um estado

O estado compartilhado já existe: `VisualPlayback` (clipe, índice, tempo) + `FacingDirection`. A N4 o generaliza em vez de criar outro:

```
AnimationPose { clip: "run", frame: 3, facing: Right, fps: 15 }   // derivado de VisualPlayback
```

- O relógio continua sendo **um só**: `VisualPlayback.elapsed`, avançado uma vez por frame em `animate_player_visual`.
- Cada camada lê `(clip, frame)` e escolhe o **mesmo índice** na própria folha. Uma camada não tem temporizador próprio.
- Peças com movimento secundário (capa, mechas longas) podem ter um **desvio** calculado a partir do estado principal (velocidade, quadro), como a `CapeLayer` já faz com o balanço. Nunca um segundo relógio de quadros.

## 2. Regiões lógicas

Categorias, não entidades. Uma peça pode cobrir várias.

| Grupo | Regiões |
|---|---|
| Corpo-base | cabeça, pescoço, torso, braço de trás, braço da frente, mão de trás, mão da frente, quadril, perna de trás, perna da frente, pé de trás, pé da frente |
| Aparência | cabelo traseiro, cabelo frontal, camisa, calça, botas, luvas, braceletes |
| Equipamento | elmo, capuz, casaco, peitoral, ombreiras, armadura de braços, armadura de pernas, capa traseira, capa frontal, arma principal, escudo, arma secundária, acessórios visíveis |

"Trás" e "frente" são em relação à câmera, não lado esquerdo ou direito. Assim o espelhamento continua sendo só `flip_x` (`N3_8_ALINHAMENTO.md`).

## 3. Camadas de desenho (proposta mínima)

Cada camada é **uma folha com a mesma grade, o mesmo canvas e o mesmo pivô do corpo** (112×128, pivô 56,118 na corrida; 80×128, 40,118 no idle). Assim o alinhamento é por construção e não por soquete. Ordem padrão, de trás para a frente:

| z | Camada | Exemplo |
|---|---|---|
| 0 | capa traseira | capa |
| 1 | cabelo traseiro | mechas atrás da cabeça |
| 2 | braço de trás (corpo + manga + luva + item de trás) | |
| 3 | perna de trás (corpo + calça + bota) | |
| 4 | torso e quadril (corpo + camisa + calça + peitoral) | |
| 5 | perna da frente | |
| 6 | cabeça (rosto) | |
| 7 | cabelo frontal / elmo / capuz | |
| 8 | braço da frente (corpo + manga + luva) | |
| 9 | arma principal / escudo (soquete) | |
| 10 | capa frontal | gola, capuz caído |

Peças "de figurino" (camisa, calça, botas) são folhas inteiras alinhadas, divididas nas mesmas fatias de frente e trás. Armas e escudos continuam como hoje: sprites pequenos posicionados por soquete.

**Ordem por animação ou quadro:** o padrão é a tabela acima. Um clipe pode trazer exceções por quadro (por exemplo, no ataque a arma passa para trás do torso na recuperação), como lista opcional ao lado do manifesto: `draw_order.ron`, `"attack_basic": [(frame: 5, layer: "weapon", z: 3)]`. Isso já estava previsto em `N1_PERSONAGEM_MODULAR.md` (`draw_order`).

## 4. Cobertura e ocultação

Esconder camadas inteiras não basta: a bota cobre só a barra da calça, a luva só o punho. Contrato mínimo, do mais simples para o mais caro:

1. **Ocultação de camada** (`hides: [HairFront, HairBack]` no elmo). Barato; resolve elmo × cabelo, capuz × cabelo frontal, casaco × camisa.
2. **Variante de composição** (`variant: "under_helmet"`): o cabelo traz uma folha alternativa "achatada" para usar com elmo, como o Terraria faz com cabelo sob chapéu. Resolve o volume que sobra.
3. **Máscara por região** (uma folha 1-bit do mesmo tamanho, "onde esta peça cobre"). Só se 1 e 2 não bastarem: bota sobre calça larga, luva sobre manga. Custo: uma textura e um shader de recorte, ou máscaras pré-aplicadas no pipeline offline.

Recomendação: começar com 1 + 2. Fazer as máscaras **no pipeline offline** (Python, como o da N3.7), gerando folhas já recortadas por combinação de **região**, nunca de item, se a prova mostrar necessidade. Sem shader novo na N4.

## 5. Soquetes

A tabela atual (`weapon`, `hand_l`, `head`, `chest`, `back`) cobre arma, escudo, capa e elmo pequeno. A N4 acrescenta, **medidos** (nunca estimados) nos quadros aprovados: `neck`, `shoulder_front`, `shoulder_back`, `waist`, `foot_front`, `foot_back` e, na mão, um `side` (mão da frente ou de trás) para a arma não trocar de lado no espelho, se o proprietário preferir assim. As medições automáticas da N3.8 mostraram que isso precisa ser feito à mão ou na fonte em camadas (`N3_8_COMPATIBILIDADE.md`). A arma segue o soquete **do quadro mostrado**, como já acontece no V2.2.

## 6. Famílias

- Armas: as 5 `WeaponForm` atuais compartilham o golpe por família. Uma espada nova é uma silhueta e uma paleta, não uma animação.
- Roupas e armaduras: famílias por silhueta (justa, folgada, volumosa, saia ou túnica longa). Itens da mesma família compartilham as fatias e só mudam pintura e paleta (recolorir é mais barato que redesenhar).
- **Limite:** uma peça que muda a silhueta (armadura de placas larga, capa até o chão, ombreiras grandes) precisa de arte específica para cada quadro de cada clipe. Isso não é combinatório (é por peça), mas não é grátis.

O que se evita é o produto (N camisas × M calças × K elmos folhas), não o trabalho por peça.

## 7. Aparência cosmética (vanity)

| | Equipamento funcional | Aparência exibida |
|---|---|---|
| Decide | atributos, defesa, dano, efeitos | o que é desenhado |
| Onde vive hoje | `Equipment` (slots) no save v3 | derivado do equipamento (`EquipmentAppearance`) |
| Proposta | sem mudança | `Appearance { slot → Option<item_id ou "none"> }`; vazio significa "mostra o funcional" |

Regras: nunca duplicar o inventário. O slot cosmético referencia um item que o jogador possui (ou uma peça base de aparência, como cabelo ou camisa comum). O cálculo de atributos ignora a aparência.

**Save:** o v3 guarda `equipment` (slots → ids). Uma aparência persistida exige um campo novo, logo um **save v4 com struct congelado e autorização** (`.claude/rules/architecture.md`). Até lá, a N4 pode testar a aparência só em memória. Nada foi alterado no save na N3.8.

## 8. Desempenho

Hoje o jogador tem 1 sprite de corpo + até 3 camadas. A proposta chega a ~10 sprites por personagem, todos com o mesmo atlas e o mesmo índice; o Bevy junta sprites com a mesma textura num lote. Para um único personagem jogável, isso é desprezível. O custo real é **memória de textura** (uma folha por peça e clipe) e **trabalho artístico**. Medir na prova da N4 com o mesmo método de `held_visual_tests.rs` (custo medido por frame).

## 9. O que não fazer

- Esqueleto 2D obrigatório, ou dezenas de ossos rígidos (`N1_ANIMACAO_ESQUELETICA_2D.md`): destruiria as dobras e o cabelo da arte aprovada.
- Uma folha por combinação.
- Um segundo controlador ou temporizador por camada.
- Um segundo inventário.
