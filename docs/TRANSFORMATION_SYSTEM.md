# Sistema de transformações

> **V1.9 [Implementado]:** a Forma Espectral (`Spectral`, liberada pela
> Relíquia do Véu) é a primeira forma jogável, com perfis de movimento e
> colisão, véus espectrais, HUD e save seguro. Implementação, valores e
> decisões: [`TRANSFORMATIONS_V1_9.md`](TRANSFORMATIONS_V1_9.md). O resto
> deste documento é a proposta original (V1.6) e continua valendo para as
> formas planejadas; onde ela difere do implementado (forma imune a dano
> físico, comando por sequência, `body` próprio), vale o documento da V1.9.
>
> Histórico da V1.6: **[Planejado]**. Nada de transformação foi implementado na V1.6, e nada do
> movimento, da física ou do combate mudou.
> - **O que existe:** `world::graph::Ability` já tem os identificadores das
>   formas planejadas (`WraithForm`, `RavenForm`, `BeastForm`, `EmberWard`), para
>   que dados e validador possam citá-los.
> - **No protótipo:** só `VeilStep`, uma relíquia que libera grades, sem física própria.

## 1. Formas propostas (originais)

| Forma | Fantasia | Movimento | Combate | Interação com o mundo | Custo |
|---|---|---|---|---|---|
| **Forma Espectral** (`WraithForm`) | O corpo vira um véu de cinza | Flutua devagar, ignora gravidade parcial, atravessa grades e frestas de 16 u | Não ataca; não pode ser ferido por ataques físicos; magia bloqueada | Grades (`requires: Ability(WraithForm)`), correntes de ar | MP por segundo |
| **Forma de Corvo** (`RavenForm`) | Um corvo de penas de ferro | Voo de curta duração (planar + 3 batidas), cabe em vãos de 1 célula de altura | Bicada fraca; pode carregar a lâmina? (não) | Plataformas altas, poços | Medidor de fôlego que recarrega no chão |
| **Forma de Fera Cinzenta** (`BeastForm`) | Lobo de brasa | Corrida rápida, salto longo e baixo | Investida (dano por contato), mordida | Quebra paredes frágeis na corrida; empurra blocos | Sem custo; não usa magias nem itens |
| **Égide de Brasa** (`EmberWard`) | Aura, não muda o corpo | Normal | Normal; resistência a fogo | Atravessa salas de calor | Passiva (relíquia) |
| **Sombra Rasteira** (proposta extra) | Silhueta no chão | Desliza no chão por fendas horizontais baixas | Nenhum | Passagens de 20 u de altura | MP por uso |

Nenhuma reproduz aparência, nome ou progressão de outro jogo. Os nomes são provisórios.

## 2. Dados (proposta)

```ron
// assets/data/transformations/forms.ron
(
    id: "wraith_form",
    ability: WraithForm,                // a chave usada nos requisitos de porta
    unlock: Pickup("wraith_relic"),     // ou Quest, Level
    input: Command("↓ ↓ + GUARD"),      // reconhecedor de comandos existente
    movement: "wraith",                 // perfil em movement_profiles.ron
    allowed_actions: [Move, Interact],
    cost: PerSecond(Mp(3)),
    max_duration: None,
    body: (size: (20.0, 20.0)),         // corpo de colisão próprio
    visual: "form_wraith",              // manifesto de sprites próprio
    hurt: IgnoreKinds([Physical]),
    exit_on: [MpEmpty, GuardPressed],
)
```

## 3. Integração planejada

| Sistema | Mudança necessária | Risco |
|---|---|---|
| **Input** | Comando de transformação no reconhecedor (nenhum código novo de parser) | Baixo |
| **Estado** | Componente `ActiveForm(Option<FormId>)`; a `PlayerStateMachine` ganha a regra "ações permitidas pela forma" | Médio: toca `player/state.rs`, **precisa de autorização** |
| **Movimento/física** | Perfis de movimento em dados (gravidade, velocidade, pulo, voo) substituindo constantes por perfil; o perfil humano usa exatamente as constantes atuais | **Alto**: muda `movement.rs` e `physics.rs`. Exige teste "humano idêntico bit a bit" antes e depois |
| **Corpo de colisão** | `Body` trocado ao entrar na forma, com checagem de espaço ao sair (não destransformar dentro de uma fresta) | Médio |
| **Combate** | Hurtbox por forma; ataques próprios como `SkillDefinition` com `usable_in: [Form(..)]` | Médio |
| **Magia** | `allowed_actions` bloqueia `Casting` | Baixo |
| **Equipamento** | Bônus de equipamento continuam; aparência desligada na forma | Baixo |
| **Animação** | Um `PlayerVisual` por forma (manifesto próprio, mesmo controlador) | Médio (arte) |
| **Save** | `unlocked_forms`, sem a forma ativa (sempre carrega humano) | Baixo |
| **Mundo** | Requisitos de porta já são `Ability(...)`; formas não precisam de nada novo no grafo | Nenhum |

## 4. Por que não na V1.6

Implementar qualquer forma exige mudar movimento e física, e o briefing proíbe.
Pelas condições de parada, uma mudança estrutural de gameplay pede autorização.
**[Decisão pendente]** para a V1.9:
1. Autorizar a refatoração para perfis de movimento.
2. Escolher a primeira forma. Recomendação: **Forma Espectral**, porque reaproveita
   o requisito de grade já testado com `VeilStep` e não precisa de voo.
